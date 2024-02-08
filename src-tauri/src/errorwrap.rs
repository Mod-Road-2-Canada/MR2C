// create the error type that represents all errors possible in our program
#[derive(thiserror::Error, Debug)]
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
	ModScriptError(String),
	#[error("Spritesheet Error: {0}")]
	ModImgError(#[from] ModImgError),
	#[error("ERR: {0}")]
	Other(String)
}

#[derive(thiserror::Error, Debug)]
pub enum ModImgError {
	#[error("WARNING: TOO MANY SPRITES {0} > 15200 (GAME MIGHT BREAK)")]
	TooManySprites(u32),

	#[error("Spritesheet not found: {0}")]
	SpriteMapNotFound(String),

	// #[error("Range parse error: {0}")]
	// ExtractRange(String)
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