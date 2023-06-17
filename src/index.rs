use std::fs;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;

use openai::embeddings::Embedding;

pub async fn index_all_files(dir: &PathBuf, csv: &PathBuf) -> Result<(), Error> {
    println!("Indexing files in dir: {:?}", *dir);
    let all_files = list_all_files(dir)?;
    for file in all_files {
        let emb = get_embedding(file).await?;
        write_embedding_to_csv(emb, csv);
    }
    Ok(())
}

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
