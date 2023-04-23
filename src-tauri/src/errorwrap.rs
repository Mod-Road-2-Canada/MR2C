// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Walkdir(#[from] walkdir::Error),
	#[error(transparent)]
	Anyhow(#[from] anyhow::Error),
	#[error(transparent)]
	StripError(#[from] std::path::StripPrefixError),
	#[error(transparent)]
	ImgError(#[from] image::ImageError),

	#[error("Script Error: {0}")]
	Script(String),
	#[error("ERR: {0}")]
	Other(String)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
	S: serde::ser::Serializer,
	{
	serializer.serialize_str(self.to_string().as_ref())
	}
}

#[macro_export]
macro_rules! custombail {
	($($arg:tt)*) => {{
		return Err(Error::Other(format!($($arg)*)));
	}};
}