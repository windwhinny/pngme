use std::str::Utf8Error;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error {
	HeaderLenMisMatch{ actual: usize },
	HeaderMisMatch{ bytes: Vec<u8> },
	HeaderReadError(IOError),
	ChunkTypeLenMismatch{ bytes: Vec<u8> },
	ChunkTypeNotUtf8{ bytes: [u8;4], err: Utf8Error },
	ChunkTypeReadError(IOError),
	ChunkDataReadError(IOError),
	ChunkCrcReadError(IOError),
	ChunkSizeReadError(IOError),
	ChunkTypeNotAplhaBetic{ string: String },
	ChunkDataNotUtf8{ bytes: Vec<u8>, err: Utf8Error },
	ChunkCrcMismatch{ expect: u32, actual: u32 },
	ReadErr(IOError),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)	
	}
}

impl std::error::Error for Error {
		
}