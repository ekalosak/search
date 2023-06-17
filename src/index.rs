use std::fs;
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;

pub fn index_all_files(dir: &PathBuf) -> Result<(), Error> {
    println!("Indexing files in dir: {:?}", *dir);
    for ent in fs::read_dir(dir)? {
        let ent = ent?;
        let path = ent.path();
        let metadata = fs::metadata(&path)?;
        println!("{:?}", path);
        if metadata.is_file() {
            let vec = get_vector(path)?;
            println!("Got vec: {:?}", vec);
            println!("");
        } else {
            index_all_files(&path)?;
        }
    }
    Ok(())
}

fn get_vector(file: PathBuf) -> Result<(), Error> {
    let fs = file.to_str().ok_or(Error::new(ErrorKind::InvalidInput, "Invalid file path"))?;
    let fc = read_file(fs)?;
    println!("{:?}", fc);
    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
