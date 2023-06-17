// TODO refactor loops over embeddings to use openai::Embeddings
use csv;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind, Read};
use std::path::PathBuf;

use futures::future::select_all;
use openai::embeddings::Embedding;

// Index all the files in the input dir using text embedding vectors stored in the output csv.
pub async fn index_all_files(dir: &PathBuf, csv: &PathBuf) -> Result<(), Error> {
    println!("Indexing files in dir: {:?}", *dir);
    openai::set_key(env::var("OPENAI_API_KEY").unwrap());
    println!("Foud OpenAI API key");
    let all_files = list_all_files(dir)?;
    let mut raw_futs = Vec::new();
    for file in all_files {
        let rfut = get_embedding(file);
        raw_futs.push(rfut);
    }
    let unpin_futs: Vec<_> = raw_futs.into_iter().map(Box::pin).collect();
    let mut futs = unpin_futs;
    while !futs.is_empty() {
        match select_all(futs).await {
            (Ok((emb, fp)), _index, remaining) => {
                write_embedding_to_csv(emb, fp, csv)?;
                futs = remaining;
            }
            (Err(e), _index, remaining) => {
                println!("Error: {:?}", e);
                futs = remaining;
            }
        }
    }
    Ok(())
}

// Traverse the directory tree under dir and collect all file paths in a Vec.
fn list_all_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut fps = Vec::new();
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let path = ent.path();
        let metadata = fs::metadata(&path)?;
        println!("{:?}", path);
        if metadata.is_file() {
            fps.push(path);
        } else {
            let more_fps = list_all_files(&path)?;
            fps.extend(more_fps);
        }
    }
    Ok(fps)
}

fn write_embedding_to_csv(emb: Embedding, source: PathBuf, csv: &PathBuf) -> Result<(), Error> {
    let file = File::create(csv)?;
    let buf = BufWriter::new(file);
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(buf);
    let path_str: String = (source.clone().into_os_string().into_string().unwrap()).to_string();
    let mut row_str: Vec<String> = emb.vec.iter().map(|value| value.to_string()).collect();
    row_str.insert(0, path_str);
    writer.write_record(row_str)?;
    writer.flush()?;
    println!("wrote embedding to {:?}", csv);
    Ok(())
}

async fn get_embedding(file: PathBuf) -> Result<(Embedding, PathBuf), Error> {
    let fs = file.to_str().ok_or(Error::new(ErrorKind::InvalidInput, "Invalid file path"))?;
    let fc = read_file(fs)?;
    let embd = Embedding::create(
        "text-embedding-ada-002",
        &fc,
        "",
    )
    .await
    .unwrap();
    Ok((embd, file))
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
