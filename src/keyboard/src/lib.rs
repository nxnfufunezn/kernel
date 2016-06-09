#![no_std]

#[macro_use]
extern crate vga;

static mut CAPS: bool = false;
static mut SHIFT: bool = false;
static mut BUFFER: [char; 256] = [' '; 256];
static mut BUFFER_WRITE_IDX: usize = 0;
static mut BUFFER_READ_IDX: usize = 0;

pub fn write_to_buffer(c: char) {
    unsafe {
        BUFFER[BUFFER_WRITE_IDX] = c;
        BUFFER_WRITE_IDX += 1;
    }
}

pub fn read_from_buffer() -> &'static [char] {
    unsafe {
        let start = BUFFER_READ_IDX;
        BUFFER_READ_IDX = BUFFER_WRITE_IDX;

        &BUFFER[start..BUFFER_WRITE_IDX]
    }
}

pub fn gets(buf: &mut [char]) {
    let mut last_index = 0;

    loop {
        let chars = read_from_buffer();
        let chars_len = chars.len();

        if chars_len == 0 {
            continue;
        }

        for i in 0..chars_len {
            buf[last_index + i] = chars[i];
        }

        last_index += chars_len;

        if buf[last_index - 1]  == '\n' {
            break;
        }
    }
}

pub fn getchar(scancode: u8) -> Option<char> {
    unsafe {
        // caps lock
        if scancode == 58 {
            CAPS = !CAPS;
            return None;
        }

        // shift down
        if (scancode == 42) || (scancode == 54) {
            SHIFT = true;
            return None;
        }

        // shift up
        if (scancode == 170) || (scancode == 182) {
            SHIFT = false;
            return None;
        }

        if CAPS || SHIFT {
            SHIFT_MAPPING.get(scancode as usize).cloned()
        } else {
            MAPPING.get(scancode as usize).cloned()
        }
    }
}

static MAPPING: [char; 58] = [
  '0',  '0', '1',  '2',  '3',  '4',  '5',  '6', '7',  '8',  '9',  '0',  '-',  '=',  '0',
  '\t', 'q',  'w',  'e',  'r',  't',  'y',  'u',  'i', 'o',  'p',  '[',  ']',  '\n', '0', 
  'a',  's', 'd',  'f',  'g',  'h',  'j',  'k',  'l',  ';', '\'', '`',  '0', '\\',
  'z',  'x',  'c',  'v', 'b',  'n',  'm',  ',',  '.',  '/',
  '0', '0', '0', ' ',
];

static SHIFT_MAPPING: [char; 58] = [
  '0',  '0', '!',  '@',  '#',  '$',  '%',  '^', '&',  '*',  '(',  ')',  '_',  '+',  '0',
  '\t', 'Q',  'W',  'E',  'R',  'T',  'Y',  'U',  'I', 'O',  'P',  '{',  '}',  '\n', '0',
  'A',  'S', 'D',  'F',  'G',  'H',  'J',  'K',  'L',  ':', '"', '`',  '0',   '|',
  'Z',  'X',  'C',  'V', 'B',  'N',  'M',  '<',  '>',  '?',
  '0', '0', '0', ' ',
];
