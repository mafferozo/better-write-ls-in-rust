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

const FORMAT_PAIRS: [(Attributes, char); 6] = [
    (Attributes::DIRECTORY, 'd'),
    (Attributes::ARCHIVE, 'a'),
    (Attributes::READONLY, 'r'),
    (Attributes::HIDDEN, 'h'),
    (Attributes::SYSTEM, 's'),
    (Attributes::REPARSE_POINT, 'l'),
];

impl Attributes {
    fn if_contains(&self, other: Attributes, ch: char) -> char {
        match self.contains(other) {
            true => ch,
            false => '-',
        }
    }

    fn write_flags(&self) -> String {
        let mut result = String::with_capacity(6);

        for (a, ch) in FORMAT_PAIRS {
            result.push(self.if_contains(a, ch))
        }

        result
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.write_flags())
    }
}
