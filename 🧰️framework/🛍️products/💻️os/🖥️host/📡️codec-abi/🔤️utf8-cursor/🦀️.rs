//! 🔤️ UTF-8 cursor of the codec ABI: the incremental UTF-8 validity cursor the structural cursors share. See [`super`].

//#region ⏳️RetainedOperation
pub(super) struct Utf8Cursor {
    remaining: u8,
    next_minimum: u8,
    next_maximum: u8,
    pub(super) invalid: bool,
}

impl Utf8Cursor {
    pub(super) fn new() -> Self {
        Self { remaining: 0, next_minimum: 0x80, next_maximum: 0xbf, invalid: false }
    }

    pub(super) fn feed(&mut self, byte: u8) {
        if self.invalid {
            return;
        }
        if self.remaining != 0 {
            if byte < self.next_minimum || byte > self.next_maximum {
                self.invalid = true;
                return;
            }
            self.remaining -= 1;
            self.next_minimum = 0x80;
            self.next_maximum = 0xbf;
            return;
        }
        match byte {
            0x00..=0x7f => {}
            0xc2..=0xdf => self.remaining = 1,
            0xe0 => {
                self.remaining = 2;
                self.next_minimum = 0xa0;
            }
            0xe1..=0xec | 0xee..=0xef => self.remaining = 2,
            0xed => {
                self.remaining = 2;
                self.next_maximum = 0x9f;
            }
            0xf0 => {
                self.remaining = 3;
                self.next_minimum = 0x90;
            }
            0xf1..=0xf3 => self.remaining = 3,
            0xf4 => {
                self.remaining = 3;
                self.next_maximum = 0x8f;
            }
            _ => self.invalid = true,
        }
    }

    pub(super) fn complete(&self) -> bool {
        !self.invalid && self.remaining == 0
    }
}
//#endregion ⏳️RetainedOperation
