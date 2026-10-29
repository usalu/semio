//! 🔤️ Lexer and parser of the Part-21 codec: the tokenizer over the exchange structure and the document parse entry. See [`super`].

use super::*;

#[path = "alphabets/🦀️.rs"]
mod alphabets;
pub use alphabets::part21_alphabet_character;

//#region 🔖️Lexer
struct Lexer<'a,'c> {
    text: &'a str,
    pos: usize,
    unterminated_comment: Option<usize>,
    comment_observer: Option<&'c mut dyn FnMut(usize,usize)>,
}

impl<'a,'c> Lexer<'a,'c> {
    pub(super) fn new(text: &'a str) -> Self {
        let mut lexer = Self { text, pos: 0, unterminated_comment: None, comment_observer: None };
        lexer.advance(0);
        lexer
    }
    fn peek(&self) -> Option<char> {
        self.text[self.pos..].chars().next()
    }
    fn peek_at(&self, offset: usize) -> Option<char> {
        self.text[self.pos..].chars().filter(|value| !value.is_ascii_control()).nth(offset)
    }
    fn advance(&mut self, count: usize) {
        let mut remaining = count;
        while let Some(value) = self.peek() {
            if value.is_ascii_control() { self.pos += value.len_utf8(); }
            else if remaining == 0 { break; }
            else { self.pos += value.len_utf8(); remaining -= 1; }
        }
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.advance(1);
        }
        c
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') => self.advance(1),
                Some('\\') if self.peek_at(2) == Some('\\') && matches!(self.peek_at(1), Some('N' | 'F')) => self.advance(3),
                Some('/') if self.peek_at(1) == Some('*') => {
                    let comment_start=self.pos;
                    let mut closed=false;
                    let mut comment_end=comment_start;
                    self.advance(2);
                    while let Some(c) = self.peek() {
                        if c == '*' && self.peek_at(1) == Some('/') {
                            comment_end=self.pos+2;
                            self.advance(2);
                            closed=true;
                            break;
                        }
                        self.advance(1);
                    }
                    if !closed {self.unterminated_comment=Some(comment_start);}
                    else if let Some(observer)=self.comment_observer.as_deref_mut(){observer(comment_start,comment_end);}
                }
                _ => break,
            }
        }
    }

    fn expect_literal(&mut self, lit: &'static str) -> Result<(), Part21Error> {
        self.skip_ws_and_comments();
        for c in lit.chars() {
            if self.peek() == Some(c) {
                self.advance(1);
            } else {
                return Err(Part21Error::MissingLiteral { at: self.pos, literal: lit });
            }
        }
        Ok(())
    }

    fn try_literal(&mut self, lit: &str) -> bool {
        let save = self.pos;
        self.skip_ws_and_comments();
        for c in lit.chars() {
            if self.peek() == Some(c) {
                self.advance(1);
            } else {
                self.pos = save;
                return false;
            }
        }
        true
    }

    fn read_keyword(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' {
                s.push(c);
                self.advance(1);
            } else {
                break;
            }
        }
        if s.is_empty() {
            return Err(Part21Error::UnexpectedChar { at: start, found: self.peek().unwrap_or('\0'), expected: "keyword" });
        }
        Ok(s)
    }

    fn read_string(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('\'') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "'" });
        }
        self.advance(1);
        let mut out = String::new();
        let mut alphabet = 'A';
        loop {
            let start = self.pos;
            match self.bump() {
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing '" }),
                Some('\'') => {
                    if self.peek() == Some('\'') {
                        self.advance(1);
                        out.push('\'');
                    } else {
                        break;
                    }
                }
                Some('\\') => self.read_escape(start, &mut out, &mut alphabet)?,
                Some(c) => out.push(c),
            }
        }
        Ok(out)
    }

    /// 🔤 Decode clause 6.4.3 control directives and clause 13 print directives after their opener.
    fn read_escape(&mut self, start: usize, out: &mut String, alphabet: &mut char) -> Result<(), Part21Error> {
        match self.bump() {
            Some('\\') => {
                out.push('\\');
                Ok(())
            }
            Some('N' | 'F') => {
                if self.bump() != Some('\\') { return Err(Part21Error::UnsupportedEscape { at: start, detail: "unterminated print directive".into() }); }
                Ok(())
            }
            Some('P') => {
                let selected = match self.bump() {
                    Some(c @ 'A'..='I') => c,
                    other => return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\P alphabet {other:?}") }),
                };
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\P".into() });
                }
                *alphabet = selected;
                Ok(())
            }
            Some('S') => {
                if self.bump() != Some('\\') {
                    return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\S".into() });
                }
                match self.bump() {
                    Some(c) => {
                        out.push(part21_alphabet_character(*alphabet, c).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: format!("undefined ISO 8859 page {alphabet} character") })?);
                        Ok(())
                    }
                    other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("bad \\S\\ character {other:?}") }),
                }
            }
            Some('X') => match self.peek() {
                Some(width @ ('2' | '4')) => {
                    let group = if width == '2' { 4 } else { 8 };
                    self.advance(1);
                    if self.bump() != Some('\\') { return Err(Part21Error::UnsupportedEscape { at: start, detail: format!("expected \\ after \\X{width}") }); }
                    let mut high: Option<u32> = None;
                    let mut count = 0usize;
                    loop {
                        self.skip_print_directives();
                        if self.peek() == Some('\\') && self.peek_at(1) == Some('X') && self.peek_at(2) == Some('0') && self.peek_at(3) == Some('\\') {
                            if count == 0 || high.is_some() { return Err(Part21Error::UnsupportedEscape { at: start, detail: "empty Unicode run or unpaired UTF-16 surrogate".into() }); }
                            self.advance(4);
                            break;
                        }
                        let code = self.read_hex(group, start)?;
                        count += 1;
                        if width == '2' {
                            if let Some(previous) = high.take() {
                                if !(0xDC00..=0xDFFF).contains(&code) { return Err(Part21Error::UnsupportedEscape { at: start, detail: "unpaired UTF-16 surrogate".into() }); }
                                out.push(char::from_u32(0x10000 + ((previous - 0xD800) << 10) + code - 0xDC00).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "invalid surrogate pair".into() })?);
                            } else if (0xD800..=0xDBFF).contains(&code) { high = Some(code); }
                            else { out.push(char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "unpaired UTF-16 surrogate".into() })?); }
                        } else { out.push(char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "invalid Unicode scalar".into() })?); }
                    }
                    Ok(())
                }
                _ => {
                    if self.bump() != Some('\\') { return Err(Part21Error::UnsupportedEscape { at: start, detail: "expected \\ after \\X".into() }); }
                    let code = self.read_hex(2, start)?;
                    out.push(char::from_u32(code).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "invalid arbitrary byte".into() })?);
                    Ok(())
                }
            },
            other => Err(Part21Error::UnsupportedEscape { at: start, detail: format!("unsupported escape start {other:?}") }),
        }
    }

    fn skip_print_directives(&mut self) {
        while self.peek() == Some('\\') && matches!(self.peek_at(1), Some('N' | 'F')) && self.peek_at(2) == Some('\\') { self.advance(3); }
    }

    fn read_hex(&mut self, width: usize, start: usize) -> Result<u32, Part21Error> {
        let mut code = 0u32;
        for _ in 0..width {
            self.skip_print_directives();
            let digit = self.bump().and_then(|value| value.is_ascii_hexdigit().then(|| value.to_digit(16)).flatten()).ok_or_else(|| Part21Error::UnsupportedEscape { at: start, detail: "invalid hexadecimal escape".into() })?;
            code = code * 16 + digit;
        }
        Ok(code)
    }

    fn read_number(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        let start = self.pos;
        let mut s = String::new();
        if matches!(self.peek(), Some('-') | Some('+')) {
            s.push(self.bump().unwrap());
        }
        if !self.peek().is_some_and(|value| value.is_ascii_digit()) { return Err(Part21Error::InvalidNumber { at: start, text: s }); }
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance(1);
            } else {
                break;
            }
        }
        let mut is_real = false;
        if self.peek() == Some('.') {
            is_real = true;
            s.push('.');
            self.advance(1);
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.advance(1);
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek(), Some('E') | Some('e')) {
            if !is_real { return Err(Part21Error::InvalidNumber { at: start, text: s }); }
            s.push('E');
            self.advance(1);
            if matches!(self.peek(), Some('+') | Some('-')) {
                s.push(self.bump().unwrap());
            }
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    self.advance(1);
                } else {
                    break;
                }
            }
        }
        if is_real {
            Part21Decimal::parse(&s).map(Part21Value::Real).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        } else {
            s.parse::<i64>().map(Part21Value::Int).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
        }
    }

    fn read_enum(&mut self) -> Result<String, Part21Error> {
        self.skip_ws_and_comments();
        if self.peek() != Some('.') {
            return Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "." });
        }
        self.advance(1);
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' {
                s.push(c);
                self.advance(1);
            } else {
                break;
            }
        }
        if self.bump() != Some('.') {
            return Err(Part21Error::MissingLiteral { at: self.pos, literal: "." });
        }
        Ok(s)
    }

    fn read_binary(&mut self) -> Result<Part21Value, Part21Error> {
        let start = self.pos;
        self.advance(1);
        let mut content = String::new();
        loop {
            self.skip_print_directives();
            match self.bump() {
                Some('"') => break,
                Some(value) if value.is_ascii_hexdigit() => content.push(value.to_ascii_uppercase()),
                Some(found) => return Err(Part21Error::UnexpectedChar { at: self.pos - 1, found, expected: "STEP binary hexadecimal digit or closing quote" }),
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing binary quote" }),
            }
        }
        if !is_part21_binary(&content) { return Err(Part21Error::InvalidNumber { at: start, text: content }); }
        Ok(Part21Value::Binary(content))
    }

    fn read_value(&mut self) -> Result<Part21Value, Part21Error> {
        self.skip_ws_and_comments();
        match self.peek() {
            Some('$') => {
                self.advance(1);
                Ok(Part21Value::Unset)
            }
            Some('*') => {
                self.advance(1);
                Ok(Part21Value::Derived)
            }
            Some('#') => {
                self.advance(1);
                let start = self.pos;
                let mut s = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        s.push(c);
                        self.advance(1);
                    } else {
                        break;
                    }
                }
                s.parse::<u64>().map(Part21Value::Ref).map_err(|_| Part21Error::InvalidNumber { at: start, text: s })
            }
            Some('\'') => self.read_string().map(Part21Value::Str),
            Some('.') => self.read_enum().map(Part21Value::Enum),
            Some('"') => self.read_binary(),
            Some('(') => {
                self.advance(1);
                let items = self.read_value_list()?;
                self.expect_literal(")")?;
                Ok(Part21Value::List(items))
            }
            Some(c) if c.is_ascii_digit() || c == '-' || c == '+' => self.read_number(),
            Some(c) if c.is_ascii_uppercase() => {
                let kw = self.read_keyword()?;
                self.skip_ws_and_comments();
                if self.peek() == Some('(') {
                    self.advance(1);
                    let items = self.read_value_list()?;
                    self.expect_literal(")")?;
                    Ok(Part21Value::Typed { name: kw, items })
                } else {
                    Err(Part21Error::UnexpectedChar { at: self.pos, found: self.peek().unwrap_or('\0'), expected: "( after typed value keyword" })
                }
            }
            Some(c) => Err(Part21Error::UnexpectedChar { at: self.pos, found: c, expected: "value" }),
            None => Err(Part21Error::UnexpectedEof { at: self.pos, expected: "value" }),
        }
    }

    fn read_value_list(&mut self) -> Result<Vec<Part21Value>, Part21Error> {
        self.skip_ws_and_comments();
        let mut out = Vec::new();
        if self.peek() == Some(')') {
            return Ok(out);
        }
        loop {
            out.push(self.read_value()?);
            self.skip_ws_and_comments();
            if self.peek() == Some(',') {
                self.advance(1);
                continue;
            }
            break;
        }
        Ok(out)
    }

    fn read_record(&mut self) -> Result<(String, Vec<Part21Value>), Part21Error> {
        let name = self.read_keyword()?;
        self.expect_literal("(")?;
        let args = self.read_value_list()?;
        self.expect_literal(")")?;
        Ok((name, args))
    }

    fn read_instance(&mut self) -> Result<Part21Instance, Part21Error> {
        self.expect_literal("#")?;
        let start = self.pos;
        let mut id_s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                id_s.push(c);
                self.advance(1);
            } else {
                break;
            }
        }
        let id = id_s.parse::<u64>().map_err(|_| Part21Error::InvalidNumber { at: start, text: id_s })?;
        self.expect_literal("=")?;
        self.skip_ws_and_comments();
        let mut entities = Vec::new();
        if self.peek() == Some('(') {
            self.advance(1);
            loop {
                self.skip_ws_and_comments();
                if self.peek() == Some(')') {
                    break;
                }
                entities.push(self.read_record()?);
                self.skip_ws_and_comments();
            }
            self.expect_literal(")")?;
        } else {
            entities.push(self.read_record()?);
        }
        self.expect_literal(";")?;
        Ok(Part21Instance { id, entities })
    }
}
//#endregion 🔖️Lexer

//#region 🔖️Parse
/// 📥️ Parses a full ISO 10303-21 physical file into the generic graph. Real tokenizer,
/// not a scraper — every header record and every data instance/argument round-trips.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_part21(text: &str) -> Result<Part21Document, Part21Error> {parse_part21_inner(text,None)}

/// 💬️ Observe actual comment ranges in the original UTF-8 source while tokenizing the same strict exchange.
pub fn parse_part21_with_comments(text:&str,observer:&mut dyn FnMut(usize,usize))->Result<Part21Document,Part21Error>{parse_part21_inner(text,Some(observer))}

fn parse_part21_inner(text:&str,observer:Option<&mut dyn FnMut(usize,usize)>)->Result<Part21Document,Part21Error>{
    let mut lex = Lexer::new(text);
    lex.comment_observer=observer;
    lex.expect_literal("ISO-10303-21")?;lex.expect_literal(";")?;
    lex.expect_literal("HEADER")?;lex.expect_literal(";")?;
    let mut mandatory=|expected|{let(name,args)=lex.read_record()?;if name!=expected{return Err(Part21Error::MissingLiteral{at:lex.pos,literal:expected});}lex.expect_literal(";")?;Ok(args)};
    let header=Part21Header{file_description:mandatory("FILE_DESCRIPTION")?,file_name:mandatory("FILE_NAME")?,file_schema:mandatory("FILE_SCHEMA")?};
    lex.expect_literal("ENDSEC")?;lex.expect_literal(";")?;
    lex.expect_literal("DATA")?;lex.expect_literal(";")?;
    let mut instances=Vec::new();
    loop {lex.skip_ws_and_comments();if lex.try_literal("ENDSEC"){lex.expect_literal(";")?;break;}instances.push(lex.read_instance()?);}
    lex.expect_literal("END-ISO-10303-21")?;lex.expect_literal(";")?;
    lex.skip_ws_and_comments();
    if let Some(at)=lex.unterminated_comment{return Err(Part21Error::UnexpectedEof{at,expected:"closing */"});}
    if let Some(found)=lex.peek(){return Err(Part21Error::UnexpectedChar{at:lex.pos,found,expected:"end of exchange"});}
    Ok(Part21Document{header,instances})
}
//#endregion 🔖️Parse
