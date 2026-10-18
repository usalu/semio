//! 🔣️ Csi, mouse and paste decoding: the csi feeding and finishing, the tilde and x10 sequences, mouse decoding and the bracketed paste. See [`super`].

use super::*;

//#region 🔤️Parse
impl AnsiParser {
    pub(super) fn feed_csi(&mut self, b: u8, out: &mut Vec<Event>) {
        match b {
            b'0'..=b'9' => {
                if self.intermediate {
                    self.invalid = true;
                } else if self.sub_index == 0 {
                    self.current = self.current.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                    self.has_current = true;
                } else if self.sub_index <= 2 {
                    let slot = &mut self.current_subs[self.sub_index - 1];
                    *slot = slot.saturating_mul(10).saturating_add(u16::from(b - b'0'));
                    self.has_current = true;
                }
            }
            b';' => {
                if self.intermediate {
                    self.invalid = true;
                }
                self.end_param();
            }
            b':' => {
                if self.intermediate {
                    self.invalid = true;
                }
                self.sub_index += 1;
                self.has_current = true;
            }
            b'<'..=b'?' => {
                if self.private.is_none() && self.count == 0 && !self.has_current && !self.intermediate {
                    self.private = Some(b);
                } else {
                    self.invalid = true;
                }
            }
            b'[' if self.count == 0 && !self.has_current && self.private.is_none() && !self.intermediate && !self.linux_function => self.linux_function = true,
            0x20..=0x2f => self.intermediate = true,
            0x40..=0x7e => {
                if self.has_current || self.count > 0 {
                    self.end_param();
                }
                self.state = ParserState::Ground;
                if !self.invalid {
                    self.finish_csi(b, out);
                }
            }
            0x1b => self.state = ParserState::Escape,
            0x18 | 0x1a => self.state = ParserState::Ground,
            0x80..=0xff => {
                self.state = ParserState::Ground;
                self.feed_ground(b, out);
            }
            _ => {}
        }
    }

    fn finish_csi(&mut self, final_byte: u8, out: &mut Vec<Event>) {
        if self.private == Some(b'<') {
            if matches!(final_byte, b'M' | b'm') {
                let (code, col, row) = (self.param(0, 0), self.param(1, 1), self.param(2, 1));
                self.mouse(code, col, row, final_byte == b'm', false, out);
            }
            return;
        }
        if self.private.is_some() || self.intermediate {
            return;
        }
        if self.linux_function {
            if let b'A'..=b'E' = final_byte {
                self.key(Key::F(final_byte - b'A' + 1), 0, out);
            }
            return;
        }
        let with_mods = self.count >= 2;
        match final_byte {
            b'A' | b'B' | b'C' | b'D' | b'H' | b'F' => {
                if self.is_release(1) {
                    return;
                }
                let key = match final_byte {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    b'H' => Key::Home,
                    _ => Key::End,
                };
                self.key(key, self.mods_param(1), out);
            }
            b'P' | b'Q' | b'S' if with_mods => {
                let number = match final_byte {
                    b'P' => 1,
                    b'Q' => 2,
                    _ => 4,
                };
                self.key(Key::F(number), self.mods_param(1), out);
            }
            b'R' if with_mods && self.param(0, 0) == 1 && self.param(1, 0) >= 2 => self.key(Key::F(3), self.mods_param(1), out),
            b'Z' => self.key(Key::BackTab, self.mods_param(1) & !mods::SHIFT, out),
            b'I' if self.count == 0 => out.push(Event::FocusGained),
            b'O' if self.count == 0 => out.push(Event::FocusLost),
            b'M' if self.count == 0 => {
                self.x10_len = 0;
                self.state = ParserState::X10Mouse;
            }
            b'M' if self.count == 3 => {
                let (code, col, row) = (self.param(0, 32).saturating_sub(32), self.param(1, 1), self.param(2, 1));
                self.mouse(code, col, row, false, true, out);
            }
            b'u' => self.finish_codepoint(self.param(0, 0), self.subs[0][0], 1, out),
            b'~' => self.finish_tilde(out),
            _ => {}
        }
    }

    fn finish_codepoint(&mut self, code: u16, shifted: u16, mods_index: usize, out: &mut Vec<Event>) {
        if self.is_release(mods_index) {
            return;
        }
        let m = self.mods_param(mods_index);
        let Some(key) = key_from_codepoint(code) else { return };
        match key {
            Key::Tab if m & mods::SHIFT != 0 => self.key(Key::BackTab, m & !mods::SHIFT, out),
            Key::Char(_) if shifted != 0 && m & mods::SHIFT != 0 => match key_from_codepoint(shifted) {
                Some(shifted_key) => self.key(shifted_key, m & !mods::SHIFT, out),
                None => self.key(key, m, out),
            },
            _ => self.key(key, m, out),
        }
    }

    fn finish_tilde(&mut self, out: &mut Vec<Event>) {
        let code = self.param(0, 0);
        match code {
            200 => {
                self.paste.clear();
                self.paste_matched = 0;
                self.paste_truncated = false;
                self.state = ParserState::Paste;
            }
            201 => {}
            27 => self.finish_codepoint(self.param(2, 0), 0, 1, out),
            _ => {
                if self.is_release(1) {
                    return;
                }
                let key = match code {
                    1 | 7 => Some(Key::Home),
                    2 => Some(Key::Insert),
                    3 => Some(Key::Delete),
                    4 | 8 => Some(Key::End),
                    5 => Some(Key::PageUp),
                    6 => Some(Key::PageDown),
                    _ => function_key_from_tilde(code),
                };
                if let Some(key) = key {
                    self.key(key, self.mods_param(1), out);
                }
            }
        }
    }

    pub(super) fn feed_x10(&mut self, b: u8, out: &mut Vec<Event>) {
        self.x10[self.x10_len] = b;
        self.x10_len += 1;
        if self.x10_len < 3 {
            return;
        }
        self.state = ParserState::Ground;
        let [code, col, row] = self.x10.map(|byte| u16::from(byte).saturating_sub(32));
        self.mouse(code, col, row, false, true, out);
    }

    pub(super) fn mouse(&mut self, code: u16, col: u16, row: u16, sgr_release: bool, legacy: bool, out: &mut Vec<Event>) {
        if code & 0x80 != 0 {
            return;
        }
        let pos = Pos { x: col.saturating_sub(1), y: row.saturating_sub(1) };
        let mods = modifier_bits(((code >> 2) & 0x7) + 1);
        let low = code & 0x3;
        let named = match low {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => self.last_button,
        };
        let kind = if code & 0x40 != 0 {
            match low {
                0 => MouseKind::Scroll { dx: 0, dy: -1 },
                1 => MouseKind::Scroll { dx: 0, dy: 1 },
                2 => MouseKind::Scroll { dx: -1, dy: 0 },
                _ => MouseKind::Scroll { dx: 1, dy: 0 },
            }
        } else if code & 0x20 != 0 {
            if low == 3 {
                MouseKind::Move
            } else {
                self.last_button = named;
                MouseKind::Drag(named)
            }
        } else if sgr_release || (legacy && low == 3) {
            MouseKind::Up(named)
        } else {
            self.last_button = named;
            MouseKind::Down(named)
        };
        out.push(Event::Mouse(MouseEvent { kind, pos, mods, clicks: 1 }));
    }

    fn push_paste(&mut self, bytes: &[u8]) {
        if self.paste.len() + bytes.len() > PASTE_LIMIT {
            self.paste_truncated = true;
        } else {
            self.paste.extend_from_slice(bytes);
        }
    }

    pub(super) fn feed_paste(&mut self, b: u8, out: &mut Vec<Event>) {
        if b == PASTE_END[self.paste_matched] {
            self.paste_matched += 1;
            if self.paste_matched == PASTE_END.len() {
                self.paste_matched = 0;
                self.finish_paste(out);
                self.state = ParserState::Ground;
            }
            return;
        }
        if self.paste_matched > 0 {
            let matched = self.paste_matched;
            self.paste_matched = 0;
            self.push_paste(&PASTE_END[..matched]);
            if b == PASTE_END[0] {
                self.paste_matched = 1;
                return;
            }
        }
        self.push_paste(&[b]);
    }

    pub(super) fn finish_paste(&mut self, out: &mut Vec<Event>) {
        let bytes = std::mem::take(&mut self.paste);
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(error) if self.paste_truncated && error.utf8_error().error_len().is_none() => {
                let valid = error.utf8_error().valid_up_to();
                String::from_utf8_lossy(&error.as_bytes()[..valid]).into_owned()
            }
            Err(error) => String::from_utf8_lossy(error.as_bytes()).into_owned(),
        };
        self.paste_truncated = false;
        self.paste_matched = 0;
        out.push(Event::Paste(text));
    }
}
//#endregion 🔤️Parse
