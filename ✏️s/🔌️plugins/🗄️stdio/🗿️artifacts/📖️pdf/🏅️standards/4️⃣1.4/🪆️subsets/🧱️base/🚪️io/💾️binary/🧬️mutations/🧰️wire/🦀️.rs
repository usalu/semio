//! 🧰️ Index, text and cursor primitives of the PDF 1.4 binary mutation payloads.

//#region 🔖️Primitives
pub fn put_index(value: usize, out: &mut Vec<u8>) -> Result<(), String> {
    out.extend_from_slice(&u64::try_from(value).map_err(|error| error.to_string())?.to_le_bytes());
    Ok(())
}

pub fn put_text(value: &str, out: &mut Vec<u8>) -> Result<(), String> {
    put_index(value.len(), out)?;
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

pub struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.position.checked_add(count).ok_or("Payload length overflow")?;
        let result = self.bytes.get(self.position..end).ok_or("Truncated mutation payload")?;
        self.position = end;
        Ok(result)
    }
    pub fn index(&mut self) -> Result<usize, String> {
        usize::try_from(u64::from_le_bytes(self.take(8)?.try_into().unwrap())).map_err(|error| error.to_string())
    }
    pub fn number(&mut self) -> Result<f64, String> {
        let value = f64::from_le_bytes(self.take(8)?.try_into().unwrap());
        if !value.is_finite() {
            return Err("Non-finite geometry".into());
        }
        Ok(value)
    }
    pub fn text(&mut self) -> Result<String, String> {
        let length = self.index()?;
        String::from_utf8(self.take(length)?.to_vec()).map_err(|error| error.to_string())
    }
    pub fn finish(self) -> Result<(), String> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err("Trailing mutation payload bytes".into())
        }
    }
}
//#endregion 🔖️Primitives
