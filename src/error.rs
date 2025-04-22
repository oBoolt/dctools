use std::{
    error,
    fmt::{self, Display},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    #[allow(dead_code)]
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn error::Error + Send + Sync>>,
    exit: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Message,
    MessagePayload,
    SpamCommand,
    Config,
    Io,
    Reqwest,
    Toml,
    Warn,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Display) -> Self {
        Self {
            kind,
            message: message.to_string(),
            source: None,
            exit: true,
        }
    }
    pub fn warn(message: impl Display) -> Self {
        Self {
            kind: ErrorKind::Warn,
            message: message.to_string(),
            source: None,
            exit: false,
        }
    }
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn set_message(mut self, message: impl Display) -> Self {
        self.message = message.to_string();
        self
    }
    pub fn exit(&self) -> bool {
        self.exit
    }
    pub fn set_exit(mut self, exit: bool) -> Self {
        self.exit = exit;
        self
    }
    fn set_source(mut self, source: Box<dyn error::Error + Send + Sync>) -> Self {
        self.source = Some(source);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(ErrorKind::Io, &e).set_source(e.into())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::new(ErrorKind::Reqwest, &e).set_source(e.into())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::new(ErrorKind::Toml, &e).set_source(e.into())
    }
}
