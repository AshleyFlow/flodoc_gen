use flodoc_gen::FloError;
use std::{fmt::Display, io, path::Path};

pub enum ExpectedPath {
    Dir,
    File,
}

impl Display for ExpectedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedPath::Dir => write!(f, "directory"),
            ExpectedPath::File => write!(f, "file"),
        }
    }
}

impl ExpectedPath {
    pub fn check_path(self, path: &Path) -> Result<(), FloError> {
        let exists = path.exists();
        let matches_expected = match self {
            ExpectedPath::Dir => path.is_dir(),
            ExpectedPath::File => path.is_file(),
        };

        if !exists {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Provided path does not exist",
            )
            .into());
        } else if !matches_expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Provided path exists, but does not lead to a {self}"),
            )
            .into());
        }

        Ok(())
    }
}
