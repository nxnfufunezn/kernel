#![no_std]

static mut CAPS: bool = false;
static mut SHIFT: bool = false;

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
