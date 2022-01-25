use std::fmt;

bitflags! {
    pub struct Attributes: u32 {
	const READONLY = 0x01;
	const HIDDEN = 0x02;
	const SYSTEM = 0x04;
	const DIRECTORY = 0x10;
	const ARCHIVE = 0x20;
	const REPARSE_POINT = 0x400;
    }
}

impl fmt::Display for Attributes {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.write_flags())
	}
}
impl Attributes {
	fn get_char(&self, other: Attributes, ch: char) -> char {
		match self.contains(other) {
			true => ch,
			false => '-',
		}
	}

	fn write_flags(&self) -> String {
		let mut result = String::with_capacity(6);
		result.push(self.get_char(Attributes::DIRECTORY, 'd'));
		result.push(self.get_char(Attributes::ARCHIVE, 'a'));
		result.push(self.get_char(Attributes::READONLY, 'r'));
		result.push(self.get_char(Attributes::HIDDEN, 'h'));
		result.push(self.get_char(Attributes::SYSTEM, 's'));
		result.push(self.get_char(Attributes::SYSTEM, 'l'));
		result
	}
}
