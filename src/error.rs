#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    AlsaError { message: String, sys_error: String },
    ReadError(audrey::read::ReadError),
    FormatError(audrey::read::FormatError),
    ManyChannelsError,
    NoChannelsError,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}

impl From<audrey::read::ReadError> for Error {
    fn from(error: audrey::read::ReadError) -> Error {
        Error::ReadError(error)
    }
}

impl From<audrey::read::FormatError> for Error {
    fn from(error: audrey::read::FormatError) -> Error {
        Error::FormatError(error)
    }
}
