use std::{fmt::Display, io, path::PathBuf, string::FromUtf8Error};

pub enum Error {
    IO(io::Error),
    FromUtf8(FromUtf8Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(io_error) => write!(f, "{io_error}"),
            Error::FromUtf8(from_utf8_error) => write!(f, "{from_utf8_error}"),
        }
    }
}

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
    pub fn check_path(self, path: &PathBuf) -> Result<(), Error> {
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
