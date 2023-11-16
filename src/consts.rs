pub const MAJOR_OFFSET: u8 = 5;

pub const MAJOR_POSINT: u8 = 0;
pub const MAJOR_NEGINT: u8 = 1;
pub const MAJOR_BYTES: u8 = 2;
pub const MAJOR_STR: u8 = 3;
pub const MAJOR_ARRAY: u8 = 4;
pub const MAJOR_MAP: u8 = 5;
pub const MAJOR_SIMPLE: u8 = 7;
pub const MAJOR_FLOAT: u8 = 7;

pub const SIMPLE_FALSE: u8 = 20;
pub const SIMPLE_TRUE: u8 = 21;
pub const SIMPLE_NULL: u8 = 22;
// pub const SIMPLE_UNDEFINED: u8 = 23;

pub const VALUE_FALSE: u8 = (MAJOR_SIMPLE << MAJOR_OFFSET) | SIMPLE_FALSE;
pub const VALUE_TRUE: u8 = (MAJOR_SIMPLE << MAJOR_OFFSET) | SIMPLE_TRUE;
pub const VALUE_NULL: u8 = (MAJOR_SIMPLE << MAJOR_OFFSET) | SIMPLE_NULL;
// pub const VALUE_UNDEFINED: u8 = (MAJOR_SIMPLE << MAJOR_LEN) | SIMPLE_UNDEFINED;
