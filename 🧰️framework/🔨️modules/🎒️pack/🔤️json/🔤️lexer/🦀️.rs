//! 📖️ Lexer: the token enum, the token-at-a-time lexer and the shared number scan. See [`super`].

use super::*;

//#region 🔖️Lexer
/// 🪙️ One structural token — the streaming layer everything else is built on. A future
/// chunked-`Read` streaming API would produce these incrementally across buffer refills; today's
/// `Lexer` already tokenizes without materializing the whole document as a DOM first, it just still
/// requires the whole input as one contiguous `&str` (every framework consumer today hands over a
/// short handcrafted schema leaf, never something worth reading in chunks).
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    True,
    False,
    Number(Number),
    String(String),
    ArrayStart,
    ArrayEnd,
    ObjectStart,
    ObjectEnd,
    Comma,
    Colon,
}

/// 🔤️ Token-at-a-time reader over a `&str` — zero-copy for structural bytes, allocating
/// only for string/number token payloads.
pub struct Lexer<'a> {
    pub(super) input: &'a str,
    pub(super) pos: usize,
}

#[derive(Clone,Copy)]
pub(super) struct NumberScan {
    pub start:usize,pub position:usize,pub state:u8,pub(super) negative:bool,floating:bool,unsigned:Option<u64>,
    integer_digits:i64,leading:i64,significant:usize,significant_start:usize,significant_end:usize,sticky:bool,
    pub(super) exponent:i64,exponent_negative:bool,
}

impl NumberScan {
    pub(super) fn new(start:usize)->Self {Self {start,position:start,state:0,negative:false,floating:false,unsigned:Some(0),integer_digits:0,leading:0,significant:0,significant_start:start,significant_end:start,sticky:false,exponent:0,exponent_negative:false}}
    pub(super) fn digit(&mut self,byte:u8,integer:bool) {
        if integer {self.integer_digits+=1;self.unsigned=self.unsigned.and_then(|value|value.checked_mul(10)?.checked_add(u64::from(byte-b'0')));}
        if self.significant==0 && byte==b'0' {self.leading+=1;}
        else {if self.significant==0 {self.significant_start=self.position;}self.significant+=1;if self.significant<=1152 {self.significant_end=self.position+1;}else {self.sticky|=byte!=b'0';}}
        self.position+=1;
    }
    pub(super) fn finish<S:JsonReadSource+?Sized>(&self,input:&S)->Result<Number,JsonError> {
        if !self.floating {if let Some(value)=self.unsigned {if !self.negative {return Ok(Number::UInt(value));}if value<=i64::MAX as u64 {return Ok(Number::Int(-(value as i64)));}if value==i64::MAX as u64+1 {return Ok(Number::Int(i64::MIN));}}}
        let length=self.position-self.start;
        let value=if length<=1200{
            let mut text=borrowed_read_source::NumberText::new();
            for position in self.start..self.position{text.push(input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?)?;}
            text.text()?.parse::<f64>()
        }else if self.significant==0{Ok(if self.negative{-0.0}else{0.0})}
        else{
            let mut normalized=borrowed_read_source::NumberText::new();if self.negative{normalized.push(b'-')?;}
            let mut first=true;
            for position in self.significant_start..self.significant_end{
                let digit=input.byte_at(position).ok_or(JsonError::InvalidNumber(self.start))?;if digit==b'.'{continue;}
                normalized.push(digit)?;if first{normalized.push(b'.')?;first=false;}
            }
            if self.sticky{normalized.push(b'1')?;}normalized.push(b'e')?;
            let exponent=if self.exponent_negative{-self.exponent}else{self.exponent};
            fmt::Write::write_fmt(&mut normalized,format_args!("{}",self.integer_digits.saturating_sub(self.leading).saturating_sub(1).saturating_add(exponent))).map_err(|_|JsonError::InvalidNumber(self.start))?;
            normalized.text()?.parse::<f64>()
        }.map_err(|_|JsonError::InvalidNumber(self.start))?;
        if !value.is_finite() {return Err(JsonError::InvalidNumber(self.start));}Ok(Number::Float(value))
    }
    pub(super) fn step<S:JsonReadSource+?Sized>(&mut self,input:&S)->Result<Option<Number>,JsonError> {
        let byte=input.byte_at(self.position);
        match self.state {
            0=>{self.state=1;if byte==Some(b'-') {self.negative=true;self.position+=1;return Ok(None);}},
            1=>{},
            2=>{if let Some(byte @ b'0'..=b'9')=byte {self.digit(byte,true);return Ok(None);}},
            3=>{},
            4=>{let Some(byte @ b'0'..=b'9')=byte else {return Err(JsonError::InvalidNumber(self.start))};self.digit(byte,false);self.state=5;return Ok(None);},
            5=>{if let Some(byte @ b'0'..=b'9')=byte {self.digit(byte,false);return Ok(None);}},
            6=>{self.state=7;if matches!(byte,Some(b'+'|b'-')) {self.exponent_negative=byte==Some(b'-');self.position+=1;return Ok(None);}if !matches!(byte,Some(b'0'..=b'9')) {return Err(JsonError::InvalidNumber(self.start));}self.state=8;},
            7=>{if !matches!(byte,Some(b'0'..=b'9')) {return Err(JsonError::InvalidNumber(self.start));}self.state=8;},
            8=>{},
            _=>unreachable!(),
        }
        if self.state==1 {let Some(byte @ b'0'..=b'9')=byte else {return Err(JsonError::InvalidNumber(self.start))};self.state=if byte==b'0' {3}else {2};self.digit(byte,true);return Ok(None);}
        if self.state==8 {if let Some(byte @ b'0'..=b'9')=byte {self.exponent=(self.exponent.saturating_mul(10)+i64::from(byte-b'0')).min(i64::try_from(input.byte_len()).unwrap_or(i64::MAX-400).saturating_add(400));self.position+=1;return Ok(None);}return self.finish(input).map(Some);}
        if matches!(self.state,2|3) && byte==Some(b'.') {self.floating=true;self.state=4;self.position+=1;return Ok(None);}
        if matches!(self.state,2|3|5) && matches!(byte,Some(b'e'|b'E')) {self.floating=true;self.state=6;self.position+=1;return Ok(None);}
        self.finish(input).map(Some)
    }
}

/// 🔣️ Retains the original allocation-free RFC8259 numeric scanner under caller-authored turns.
#[derive(Clone,Copy)]
pub struct JsonNumberCursor(NumberScan);

impl JsonNumberCursor{
    /// 🌱️ Begins one exact numeric TEXT field without scanning, copying, or allocating.
    pub fn new()->Self{Self(NumberScan::new(0))}
    /// 📍️ Reports the actual original numeric source position.
    pub fn position(&self)->usize{self.0.position}
    /// 🧮️ Performs one original numeric grammar transition into the retained scalar owner.
    pub fn step<S:JsonReadSource+?Sized>(&mut self,input:&S)->Result<Option<Number>,JsonError>{self.0.step(input)}
}

impl Default for JsonNumberCursor{fn default()->Self{Self::new()}}

semio_framework_value::artifact_retire_leaf!(JsonNumberCursor);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// 🔎️ Current byte offset into the input.
    pub fn position(&self) -> usize {
        self.pos
    }

    pub(super) fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn read_literal(&mut self, literal: &'static str, token: Token) -> Result<Option<Token>, JsonError> {
        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            Ok(Some(token))
        } else {
            Err(JsonError::UnexpectedByte { found: self.peek_byte().unwrap_or(0), offset: self.pos })
        }
    }

    /// 🔢️ Reads one JSON number literal per RFC 8259 §6: `-? int frac? exp?`, no leading zeros.
    /// Falls back to `Number::Float` whenever a fractional part, exponent, or `u64`/`i64` overflow
    /// is present — identical to `serde_json` with its `float_roundtrip` oracle configuration and
    /// without `arbitrary_precision` (the only configuration this repo ever builds with).
    pub(super) fn read_number(&mut self) -> Result<Token, JsonError> {
        let mut number = NumberScan::new(self.pos);
        loop { if let Some(value) = number.step(self.input)? { self.pos = number.position; return Ok(Token::Number(value)); } }
    }

    /// 🧵️ Reads the 4 hex digits of one `\uXXXX` escape (already past the `u`).
    fn read_hex4(&mut self, escape_start: usize) -> Result<u32, JsonError> {
        let bytes = self.input.as_bytes();
        if self.pos + 4 > bytes.len() {
            return Err(JsonError::InvalidUnicodeEscape(escape_start));
        }
        let mut value: u32 = 0;
        for &byte in &bytes[self.pos..self.pos + 4] {
            let digit = match byte {
                b'0'..=b'9' => u32::from(byte - b'0'),
                b'a'..=b'f' => u32::from(byte - b'a') + 10,
                b'A'..=b'F' => u32::from(byte - b'A') + 10,
                _ => return Err(JsonError::InvalidUnicodeEscape(escape_start)),
            };
            value = value * 16 + digit;
        }
        self.pos += 4;
        Ok(value)
    }

    /// 💯️ Reads a full JSON string literal (opening quote must be at `self.pos`). Handles every
    /// short escape, `\uXXXX`, and UTF-16 surrogate pairs for supplementary-plane characters —
    /// rejects a lone (unpaired) surrogate rather than silently producing an invalid `char`.
    fn read_string(&mut self) -> Result<String, JsonError> {
        self.pos += 1; let mut output = String::new();
        while let Some(character) = json_character(self)? { output.push(character); }
        Ok(output)
    }

    /// 📤️ Reads the next structural or scalar token, or `None` at end of input.
    pub fn next_token(&mut self) -> Result<Option<Token>, JsonError> {
        self.skip_ws();
        let Some(byte) = self.peek_byte() else { return Ok(None) };
        match byte {
            b'{' => {
                self.pos += 1;
                Ok(Some(Token::ObjectStart))
            }
            b'}' => {
                self.pos += 1;
                Ok(Some(Token::ObjectEnd))
            }
            b'[' => {
                self.pos += 1;
                Ok(Some(Token::ArrayStart))
            }
            b']' => {
                self.pos += 1;
                Ok(Some(Token::ArrayEnd))
            }
            b',' => {
                self.pos += 1;
                Ok(Some(Token::Comma))
            }
            b':' => {
                self.pos += 1;
                Ok(Some(Token::Colon))
            }
            b'"' => self.read_string().map(|s| Some(Token::String(s))),
            b't' => self.read_literal("true", Token::True),
            b'f' => self.read_literal("false", Token::False),
            b'n' => self.read_literal("null", Token::Null),
            b'-' | b'0'..=b'9' => self.read_number().map(Some),
            _ => Err(JsonError::UnexpectedByte { found: byte, offset: self.pos }),
        }
    }
}
//#endregion 🔖️Lexer
