#[macro_use(concat_string)]
extern crate concat_string;

#[macro_use]
pub mod errorwrap;

pub mod core {
	pub mod func;
	pub mod image_merging;
	pub mod image_constant;

}
