//! 🧵️ Physical coordinate and URI text extensions for owned artifact identities.

use crate::{ArtifactRef,ArtifactDialect};
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🔖️ Native text projection of an owned dialect.
pub trait DialectCoordinateText:Sized {fn to_coordinate(&self)->String;fn parse_coordinate(text:&str)->Result<Self,String>;}

/// 🪢️ Native URI projection and bounded text admission for owned references.
pub trait ArtifactReferenceText:Sized {
 fn to_uri(&self)->String;
 fn parse_uri(text:&str)->Result<Self,String>;
 fn parse_uri_controlled(text:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,String>;
}

impl DialectCoordinateText for ArtifactDialect {
    /// 🧵️ Canonical single-string coordinate form: `"s.stdio.gif@87a/*"`. The one format that
    /// crosses every boundary in the system — the only dialect-coordinate codec in the repo.
    fn to_coordinate(&self) -> String {
        format!("{}@{}/{}", self.artifact_kind, self.standard, self.subset)
    }

    /// 🧵️ Inverse of `to_coordinate`. `@` separates artifact_kind from standard/subset; the LAST
    /// `/` separates standard from subset.
    fn parse_coordinate(s: &str) -> Result<Self, String> {
        let (kind, rest) = s.split_once('@').ok_or_else(|| format!("dialect coordinate {s:?} missing '@'"))?;
        let (standard, subset) = rest.rsplit_once('/').ok_or_else(|| format!("dialect coordinate {s:?} missing '/'"))?;
        if kind.is_empty() || standard.is_empty() || subset.is_empty() {
            return Err(format!("dialect coordinate {s:?} has an empty component"));
        }
        Ok(ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() })
    }
}
impl ArtifactReferenceText for ArtifactRef {
    /// 🧵️ Canonical wire form: `"<artifact_id>!<kind>@<standard>/<subset>"`.
    fn to_uri(&self) -> String {
        format!("{}!{}", self.artifact_id, self.dialect.to_coordinate())
    }

    /// 🚦️ Parses identity handles with bounded borrowed scanning and admitted ownership of their four strings.
    fn parse_uri_controlled(text:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,String>{
        control.scoped_stage(|control|{
            control.begin_stage(text.len())?;let mut bang=None;let mut at=None;let mut slash=None;let mut position=0;
            for chunk in text.as_bytes().chunks(256){for(byte_offset,byte)in chunk.iter().enumerate(){let offset=position+byte_offset;if bang.is_none(){if *byte==b'!'{bang=Some(offset);}}else if at.is_none(){if *byte==b'@'{at=Some(offset);}}else if *byte==b'/'{slash=Some(offset);}}position+=chunk.len();control.advance(chunk.len())?;}
            let bang=bang.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact reference requires '!'"))?;let at=at.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact dialect requires '@'"))?;let slash=slash.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact dialect requires '/'"))?;
            if bang==0||at==bang+1||slash==at+1||slash+1==text.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"artifact reference has an empty identity component"));}
            Ok(Self{artifact_id:control.copy_text(&text[..bang])?,dialect:ArtifactDialect{artifact_kind:control.copy_text(&text[bang+1..at])?,standard:control.copy_text(&text[at+1..slash])?,subset:control.copy_text(&text[slash+1..])?}})
        }).map_err(ValueError::into_message)
    }

    /// 🧵️ Inverse of `to_uri`. Splits on the FIRST `!`.
    fn parse_uri(s: &str) -> Result<Self, String> {
        let (artifact_id, coordinate) = s.split_once('!').ok_or_else(|| format!("artifact ref uri {s:?} missing '!'"))?;
        if artifact_id.is_empty() {
            return Err(format!("artifact ref uri {s:?} has an empty artifact id"));
        }
        let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
        Ok(ArtifactRef { artifact_id: artifact_id.to_string(), dialect })
    }
}

