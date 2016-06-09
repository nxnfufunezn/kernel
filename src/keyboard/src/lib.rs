#![no_std]

#[macro_use]
extern crate vga;

static mut CAPS: bool = false;
static mut SHIFT: bool = false;
static mut BUFFER: [u8; 256] = [0; 256];
static mut BUFFER_WRITE_IDX: usize = 0;
static mut BUFFER_READ_IDX: usize = 0;

pub fn write_to_buffer(c: u8) {
    unsafe {
        BUFFER[BUFFER_WRITE_IDX] = c;
        BUFFER_WRITE_IDX += 1;
    }
}

fn read_from_buffer() -> &'static [u8] {
    unsafe {
        let start = BUFFER_READ_IDX;
        BUFFER_READ_IDX = BUFFER_WRITE_IDX;

        &BUFFER[start..BUFFER_WRITE_IDX]
    }
}


/// gets fills a buff with everything up till a newline, and returns a slice of buff, not including
/// the newline.
pub fn gets(buf: &mut [u8]) -> &[u8] {
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

        if buf[last_index - 1]  == b'\n' {
            break;
        }
    }

    &buf[..last_index - 1]
}

pub fn getchar(scancode: u8) -> Option<u8> {
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

static MAPPING: [u8; 58] = [
  b'0',  b'0', b'1',  b'2',  b'3',  b'4',  b'5',  b'6', b'7',  b'8',  b'9',  b'0',  b'-',  b'=',  b'0',
  b'\t', b'q',  b'w',  b'e',  b'r',  b't',  b'y',  b'u',  b'i', b'o',  b'p',  b'[',  b']',  b'\n', b'0', 
  b'a',  b's', b'd',  b'f',  b'g',  b'h',  b'j',  b'k',  b'l',  b';', b'\'', b'`',  b'0', b'\\',
  b'z',  b'x',  b'c',  b'v', b'b',  b'n',  b'm',  b',',  b'.',  b'/',
  b'0', b'0', b'0', b' ',
];

static SHIFT_MAPPING: [u8; 58] = [
  b'0',  b'0', b'!',  b'@',  b'#',  b'$',  b'%',  b'^', b'&',  b'*',  b'(',  b')',  b'_',  b'+', b'0',
  b'\t', b'Q',  b'W',  b'E',  b'R',  b'T',  b'Y',  b'U',  b'I', b'O',  b'P',  b'{',  b'}',  b'\n', b'0',
  b'A',  b'S', b'D',  b'F',  b'G',  b'H',  b'J',  b'K',  b'L',  b':', b'"', b'`',  b'0',   b'|',
  b'Z',  b'X',  b'C',  b'V', b'B',  b'N',  b'M',  b'<',  b'>',  b'?',
  b'0', b'0', b'0', b' ',
];
