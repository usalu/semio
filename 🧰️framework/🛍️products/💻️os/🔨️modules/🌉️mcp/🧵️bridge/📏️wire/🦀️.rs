//! 📏️ Minimal length-prefixed little-endian primitives shared by both frame enums' hand-rolled
//! `encode`/`decode`, private to the bridge codec; the TS twin (`🟦️.ts`) mirrors the exact same
//! byte layout independently (that agreement is what the fixtures prove).

//#region 🔖️Wire
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

pub fn write_u8(buf: &mut Vec<u8>, value: u8) {
    buf.push(value);
}
pub fn write_u16(buf: &mut Vec<u8>, value: u16) {
    buf.extend_from_slice(&value.to_le_bytes());
}
pub fn write_u32(buf: &mut Vec<u8>, value: u32) {
    buf.extend_from_slice(&value.to_le_bytes());
}
pub fn write_u64(buf: &mut Vec<u8>, value: u64) {
    buf.extend_from_slice(&value.to_le_bytes());
}
pub fn write_bool(buf: &mut Vec<u8>, value: bool) {
    buf.push(value as u8);
}
pub fn write_bytes(buf: &mut Vec<u8>, value: &[u8]) {
    write_u32(buf, value.len() as u32);
    buf.extend_from_slice(value);
}
pub fn write_string(buf: &mut Vec<u8>, value: &str) {
    write_bytes(buf, value.as_bytes());
}
pub fn write_option_string(buf: &mut Vec<u8>, value: &Option<String>) {
    match value {
        Some(inner) => {
            write_bool(buf, true);
            write_string(buf, inner);
        }
        None => write_bool(buf, false),
    }
}
pub fn write_string_vec(buf: &mut Vec<u8>, value: &[String]) {
    write_u32(buf, value.len() as u32);
    for item in value {
        write_string(buf, item);
    }
}
pub fn write_bytes_vec(buf: &mut Vec<u8>, value: &[Vec<u8>]) {
    write_u32(buf, value.len() as u32);
    for item in value {
        write_bytes(buf, item);
    }
}

/// 📖️ A forward-only cursor over a decode buffer — every read is bounds-checked, `finish()`
/// rejects trailing bytes so a truncated OR over-long frame is always a decode error, never a
/// silent partial parse.
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn need(&self, count: usize) -> Result<(), semio_framework_os_mcp_foundation::errors::GatewayError> {
        if self.pos + count > self.data.len() {
            Err(semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, "bridge frame: unexpected end of buffer"))
        } else {
            Ok(())
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, semio_framework_os_mcp_foundation::errors::GatewayError> {
        self.need(1)?;
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }
    pub fn read_u16(&mut self) -> Result<u16, semio_framework_os_mcp_foundation::errors::GatewayError> {
        self.need(2)?;
        let value = u16::from_le_bytes(self.data[self.pos..self.pos + 2].try_into().expect("checked length"));
        self.pos += 2;
        Ok(value)
    }
    pub fn read_u32(&mut self) -> Result<u32, semio_framework_os_mcp_foundation::errors::GatewayError> {
        self.need(4)?;
        let value = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().expect("checked length"));
        self.pos += 4;
        Ok(value)
    }
    pub fn read_u64(&mut self) -> Result<u64, semio_framework_os_mcp_foundation::errors::GatewayError> {
        self.need(8)?;
        let value = u64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().expect("checked length"));
        self.pos += 8;
        Ok(value)
    }
    pub fn read_bool(&mut self) -> Result<bool, semio_framework_os_mcp_foundation::errors::GatewayError> {
        Ok(self.read_u8()? != 0)
    }
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, semio_framework_os_mcp_foundation::errors::GatewayError> {
        let len = self.read_u32()? as usize;
        self.need(len)?;
        let value = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(value)
    }
    pub fn read_string(&mut self) -> Result<String, semio_framework_os_mcp_foundation::errors::GatewayError> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes).map_err(|error| semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, format!("bridge frame: invalid utf8: {error}")))
    }
    pub fn read_option_string(&mut self) -> Result<Option<String>, semio_framework_os_mcp_foundation::errors::GatewayError> {
        if self.read_bool()? {
            Ok(Some(self.read_string()?))
        } else {
            Ok(None)
        }
    }
    pub fn read_string_vec(&mut self) -> Result<Vec<String>, semio_framework_os_mcp_foundation::errors::GatewayError> {
        let len = self.read_u32()? as usize;
        (0..len).map(|_| self.read_string()).collect()
    }
    pub fn read_bytes_vec(&mut self) -> Result<Vec<Vec<u8>>, semio_framework_os_mcp_foundation::errors::GatewayError> {
        let len = self.read_u32()? as usize;
        (0..len).map(|_| self.read_bytes()).collect()
    }
    pub fn finish(self) -> Result<(), semio_framework_os_mcp_foundation::errors::GatewayError> {
        if self.pos == self.data.len() {
            Ok(())
        } else {
            Err(semio_framework_os_mcp_foundation::errors::GatewayError::new(semio_framework_os_mcp_foundation::errors::GatewayErrorCode::InputInvalid, format!("bridge frame: {} trailing byte(s) after decode", self.data.len() - self.pos)))
        }
    }
}
//#endregion 🔖️Wire
