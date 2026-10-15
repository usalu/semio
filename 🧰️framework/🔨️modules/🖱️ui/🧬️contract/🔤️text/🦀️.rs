//! 🔤️ Bounded semantic UI text, codecs, original copying and byte retirement.
use serde::{Deserialize,Deserializer,Serialize,Serializer};
use std::fmt;
use semio_framework_ui_contract_budget::UI_TEXT_MAX_BYTES;
/// ✂️ Marks a display-only value clipped at a UTF8 boundary.
pub const UI_TEXT_CLIP_MARK:&str="…";
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiText {
    bytes: [u8; UI_TEXT_MAX_BYTES],
    len: u16,
}

impl Default for UiText {
    fn default() -> Self {
        Self { bytes: [0; UI_TEXT_MAX_BYTES], len: 0 }
    }
}

impl fmt::Debug for UiText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

/// 🫴️ A fixed receiving cell copies the original UTF8 bytes before donating its whole typed header.
pub struct UiTextOriginalCopy{target:Option<UiText>,source:(usize,usize),offset:usize}
impl UiTextOriginalCopy{
 pub fn new(source:&UiText)->Self{Self{target:Some(UiText::default()),source:(source.bytes.as_ptr()as usize,source.len()),offset:0}}
 pub fn step(&mut self,source:&UiText,grant:semio_framework_value::RetainedCloneGrant)->Result<(Option<UiText>,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{
  use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
  if self.source!=(source.bytes.as_ptr()as usize,source.len()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original UI text copy source changed"))}
  if self.target.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,Default::default()))}
  if self.offset<source.len(){
   let copied=(source.len()-self.offset).min(grant.maximum_copy_bytes);if copied==0{return Ok((None,Default::default()))}
   let start=self.offset;let end=start+copied;self.target.as_mut().unwrap().bytes[start..end].copy_from_slice(&source.bytes[start..end]);self.offset=end;
   return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}))
  }
  self.target.as_mut().unwrap().len=source.len;Ok((self.target.take(),RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}

impl UiText {
    pub fn try_from_string(value: String) -> Result<Self, String> {
        if value.len() > UI_TEXT_MAX_BYTES {
            return Err(value);
        }
        let mut text = Self::default();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len() as u16;
        Ok(text)
    }

    pub fn try_from_str(value: &str) -> Option<Self> {
        if value.len() > UI_TEXT_MAX_BYTES {
            return None;
        }
        let mut text = Self::default();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len() as u16;
        Some(text)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).unwrap_or("")
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub const fn capacity(&self) -> usize {
        UI_TEXT_MAX_BYTES
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn try_format(arguments: fmt::Arguments<'_>) -> Option<Self> {
        use fmt::Write;
        let mut text = Self::default();
        text.write_fmt(arguments).ok()?;
        Some(text)
    }

    /// ✂️ Display copy that must never fail admission: the whole value when it fits, otherwise the
    /// longest char-boundary prefix leaving room for [`UI_TEXT_CLIP_MARK`], which is appended so a
    /// reader sees the text was clipped. Use it only for descriptions and summaries whose full text
    /// stays reachable elsewhere (e.g. the history view), never for identifiers or values.
    pub fn clipped(value: &str) -> Self {
        if let Some(text) = Self::try_from_str(value) {
            return text;
        }
        let budget = UI_TEXT_MAX_BYTES - UI_TEXT_CLIP_MARK.len();
        let mut end = budget;
        while !value.is_char_boundary(end) {
            end -= 1;
        }
        let mut text = Self::default();
        text.bytes[..end].copy_from_slice(&value.as_bytes()[..end]);
        text.bytes[end..end + UI_TEXT_CLIP_MARK.len()].copy_from_slice(UI_TEXT_CLIP_MARK.as_bytes());
        text.len = (end + UI_TEXT_CLIP_MARK.len()) as u16;
        text
    }
}

impl fmt::Write for UiText {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let start = self.len();
        let end = start.checked_add(value.len()).filter(|end| *end <= UI_TEXT_MAX_BYTES).ok_or(fmt::Error)?;
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.len = u16::try_from(end).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

impl fmt::Display for UiText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for UiText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for UiText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl TryFrom<String> for UiText {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}

impl<'a> TryFrom<&'a str> for UiText {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from_str(value).ok_or(value)
    }
}

impl Serialize for UiText {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for UiText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_from_string(value).map_err(|value| serde::de::Error::custom(format!("UiText exceeds {UI_TEXT_MAX_BYTES} bytes with {}", value.len())))
    }
}

/// 🌱️ Hand-written, not derived — same reason as `Serialize`/`Deserialize` above: `bytes` is a
/// fixed-size `[u8; UI_TEXT_MAX_BYTES]` with no field-level shape to annotate. Mirrors the two impls
/// exactly (a bare wire string). Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
impl ::semio_framework_value::ToValue for UiText {
    fn to_value(&self) -> ::semio_framework_value::DslValue {
        ::semio_framework_value::DslValue::String(self.as_str().to_string())
    }
}

impl ::semio_framework_value::FromValue for UiText {
    fn from_value(value: ::semio_framework_value::DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        let text = match value {
            ::semio_framework_value::DslValue::String(s) => s,
            other => return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for UiText, found {other:?}"))),
        };
        Self::try_from_string(text).map_err(|value| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("UiText exceeds {UI_TEXT_MAX_BYTES} bytes with {}", value.len())))
    }
}

impl UiText {
 /// 🔎️ Borrows the admitted bytes for bounded native comparisons.
 pub fn comparison_bytes(&self)->&[u8]{&self.bytes[..self.len()]}
 /// 🫴️ Copies one exact source prefix under the caller's byte grant.
 pub fn copy_prefix_from(&mut self,source:&Self,maximum_bytes:usize)->Result<(bool,usize),&'static str>{let start=self.len();if start>source.len(){return Err("original UI text copy prefix changed")}let copied=(source.len()-start).min(maximum_bytes);self.bytes[start..start+copied].copy_from_slice(&source.bytes[start..start+copied]);self.len+=copied as u16;Ok((self.len==source.len,copied))}
 /// ♻️ Retires at most the caller's admitted number of inline bytes.
 pub fn retire_bytes(&mut self,maximum_bytes:usize)->usize{let bytes=self.len().min(maximum_bytes);self.len-=bytes as u16;bytes}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
