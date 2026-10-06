//! 💾️ Binary representation codec surface for `s.stdio.semio.kit.diff` — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::kit::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_link_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_link_list};
use crate::standards::v1::subsets::object::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::object::io::text::snapshot::{enc_child_opt};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_child_list};

impl protocol::DiffBinary for SemioKitDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=types, bit1=designs,
/// bit2=objects, bit3=models, bit4=properties, bit5=representations), then each present
/// field's own real encoding in bit order.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    use crate::standards::v1::subsets::kit::schema::snapshot::{write_child_list, write_child_opt, write_design_list, write_link_list, write_type_list};
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence: u8 = 0;
    if self.types.is_some() {
        presence |= 0b0000_0001;
    }
    if self.designs.is_some() {
        presence |= 0b0000_0010;
    }
    if self.objects.is_some() {
        presence |= 0b0000_0100;
    }
    if self.models.is_some() {
        presence |= 0b0000_1000;
    }
    if self.properties.is_some() {
        presence |= 0b0001_0000;
    }
    if self.representations.is_some() {
        presence |= 0b0010_0000;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(t) = &self.types {
        write_type_list(&mut out, &t.values);
    }
    if let Some(d) = &self.designs {
        write_design_list(&mut out, &d.values);
    }
    if let Some(o) = &self.objects {
        write_child_list(&mut out, &o.values);
    }
    if let Some(m) = &self.models {
        write_child_list(&mut out, &m.values);
    }
    if let Some(p) = &self.properties {
        write_child_opt(&mut out, p);
    }
    if let Some(r) = &self.representations {
        write_link_list(&mut out, &r.values);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    use crate::standards::v1::subsets::kit::schema::snapshot::{read_child_list, read_child_opt, read_design_list, read_link_list, read_type_list};
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let map_err = |e: String| protocol::ProtocolError::Malformed { what: "kit diff field", offset: 2, detail: e };
    let types = if presence & 0b0000_0001 != 0 { Some(SemioKitTypeList { values: read_type_list(&mut reader).map_err(map_err)? }) } else { None };
    let designs = if presence & 0b0000_0010 != 0 { Some(SemioKitDesignList { values: read_design_list(&mut reader).map_err(map_err)? }) } else { None };
    let objects = if presence & 0b0000_0100 != 0 { Some(SemioKitObjectChildList { values: read_child_list(&mut reader).map_err(map_err)? }) } else { None };
    let models = if presence & 0b0000_1000 != 0 { Some(SemioKitModelChildList { values: read_child_list(&mut reader).map_err(map_err)? }) } else { None };
    let properties = if presence & 0b0001_0000 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
    let representations = if presence & 0b0010_0000 != 0 { Some(SemioKitLinkList { values: read_link_list(&mut reader).map_err(map_err)? }) } else { None };
    Ok(SemioKitDiff { types, designs, objects, models, properties, representations })
}
}
}
pub use diff_codec::*;
