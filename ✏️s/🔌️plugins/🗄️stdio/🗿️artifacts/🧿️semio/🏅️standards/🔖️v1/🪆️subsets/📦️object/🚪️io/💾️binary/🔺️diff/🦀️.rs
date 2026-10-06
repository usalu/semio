//! 💾️ Binary representation codec surface for `s.stdio.semio.object.diff` — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::object::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::object::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::object::io::text::snapshot::{enc_child_opt};
use crate::model::io::text::diff::{dec_transform};
use crate::model::io::text::diff::{enc_transform};

impl protocol::DiffBinary for SemioObjectDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=transform, bit1=brep,
/// bit2=mesh, bit3=properties), then each present field's own real encoding in bit order.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    use crate::standards::v1::subsets::object::schema::snapshot::{write_child_opt, write_transform};
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence: u8 = 0;
    if self.transform.is_some() {
        presence |= 0b0001;
    }
    if self.brep.is_some() {
        presence |= 0b0010;
    }
    if self.mesh.is_some() {
        presence |= 0b0100;
    }
    if self.properties.is_some() {
        presence |= 0b1000;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(t) = &self.transform {
        write_transform(&mut out, t);
    }
    if let Some(b) = &self.brep {
        write_child_opt(&mut out, b);
    }
    if let Some(m) = &self.mesh {
        write_child_opt(&mut out, m);
    }
    if let Some(p) = &self.properties {
        write_child_opt(&mut out, p);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    use crate::standards::v1::subsets::object::schema::snapshot::{read_child_opt, read_transform};
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let map_err = |e: String| protocol::ProtocolError::Malformed { what: "object diff field", offset: 2, detail: e };
    let transform = if presence & 0b0001 != 0 { Some(read_transform(&mut reader).map_err(map_err)?) } else { None };
    let brep = if presence & 0b0010 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
    let mesh = if presence & 0b0100 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
    let properties = if presence & 0b1000 != 0 { Some(read_child_opt(&mut reader).map_err(map_err)?) } else { None };
    let diff = SemioObjectDiff { transform, brep, mesh, properties };
    diff.validate().map_err(map_err)?;
    Ok(diff)
}
}
}
pub use diff_codec::*;
