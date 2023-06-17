use std::fs;
use std::fs::File;
use std::future::Future;
use std::pin::Pin;
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;

use openai::embeddings::Embedding;

pub fn index_all_files(dir: &PathBuf, csv: &PathBuf) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
    Box::pin(async move {
        println!("Indexing files in dir: {:?}", *dir);
        for ent in fs::read_dir(dir)? {
            let ent = ent?;
            let path = ent.path();
            let metadata = fs::metadata(&path)?;
            println!("{:?}", path);
            if metadata.is_file() {
                let vec = get_embedding(path).await?;
                write_embedding_to_csv(vec, csv);
            } else {
                index_all_files(&path, csv).await?;
            }
        }
        Ok(())
    })
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
