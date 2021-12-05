use cache::*;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

wit_bindgen_rust::export!("../cache.wit");

struct Cache {}

impl cache::Cache for Cache {
    fn set(key: String, value: cache::Payload, _: Option<u32>) -> Result<(), Error> {
        let mut file = File::create(path(&key)?)?;
        file.write_all(&value)?;
        Ok(())
    }

    fn get(key: String) -> Result<cache::Payload, Error> {
        let mut file = File::open(path(&key)?)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn delete(key: String) -> Result<(), Error> {
        std::fs::remove_file(path(&key)?)?;
        Ok(())
    }
}

/// Return the absolute path for the file corresponding to the given key.
fn path(name: &str) -> Result<PathBuf, anyhow::Error> {
    Ok(PathBuf::from("cache").join(name))
}

// TODO
// Error handling is currently not implemented.
impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::RuntimeError
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::RuntimeError
    }
}
