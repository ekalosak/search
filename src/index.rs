use std::fs;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;

use futures::future::select_all;
use openai::embeddings::Embedding;

// Index all the files in the input dir using text embedding vectors stored in the output csv.
pub async fn index_all_files(dir: &PathBuf, csv: &PathBuf) -> Result<(), Error> {
    println!("Indexing files in dir: {:?}", *dir);
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
            (Ok(emb), _index, remaining) => {
                write_embedding_to_csv(emb, csv);
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

fn write_embedding_to_csv(emb: Embedding, csv: &PathBuf) {
    println!("write_embedding_to_csv {:?}", csv);
    for x in &emb.vec {
        println!("\t{:?}", x);
    }
}

async fn get_embedding(file: PathBuf) -> Result<Embedding, Error> {
    let fs = file.to_str().ok_or(Error::new(ErrorKind::InvalidInput, "Invalid file path"))?;
    let fc = read_file(fs)?;
    let embd = Embedding::create(
        "text-embedding-ada-002",
        &fc,
        "",
    )
    .await
    .unwrap();
    Ok(embd)
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
