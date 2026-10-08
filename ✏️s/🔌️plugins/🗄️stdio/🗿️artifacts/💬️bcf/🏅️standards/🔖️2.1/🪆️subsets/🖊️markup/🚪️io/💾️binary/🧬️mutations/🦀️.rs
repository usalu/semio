//! bcf rep for stdio.bcf 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2_1::subsets::any::schema::mutations::*;
use crate::standards::v2_1::subsets::any::io::binary::diff::{dec_components_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{enc_components_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{dec_camera_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{enc_camera_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{dec_viewpoint_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{enc_viewpoint_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{dec_comment_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{enc_comment_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{dec_topic_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{enc_topic_bin};
use crate::standards::v2_1::subsets::any::io::binary::diff::{read_bytes_lp};
use crate::standards::v2_1::subsets::any::io::binary::diff::{write_bytes_lp};
use crate::standards::v2_1::subsets::any::io::binary::diff::{read_str_lp};
use crate::standards::v2_1::subsets::any::io::binary::diff::{write_str_lp};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_components};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_components};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_camera};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_camera};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_viewpoint};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_viewpoint};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_comment};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_comment};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_list};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_list};
use crate::standards::v2_1::subsets::any::io::text::diff::{decode_option};
use crate::standards::v2_1::subsets::any::io::text::diff::{encode_option};
use crate::standards::v2_1::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v2_1::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_bytes};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_bytes};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_part};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_part};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_topic};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_topic};
use crate::standards::v2_1::subsets::any::io::text::diff::{dec_str};
use crate::standards::v2_1::subsets::any::io::text::diff::{enc_str};
use crate::schema::diff::{wrap_comment_diff, wrap_topic_diff, wrap_viewpoint_diff, BcfCommentDiff, BcfCommentsDiff, BcfDiff, BcfTopicDiff, BcfTopicsDiff, BcfViewpointDiff, BcfViewpointsDiff};
use crate::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfTopic, BcfViewpoint};
use crate::BcfSnapshot;
use protocol::Mutation;

/// 🧪️ FG-wave: real recursive BINARY primitives for `BcfMutation`'s own variant-specific fields
/// (`Option<String>`/`Option<Option<String>>` tri-states, `Option<BcfCamera>`/
/// `Option<BcfComponents>`/`Option<Vec<u8>>`) -- everything ELSE (whole `BcfSnapshot`/`BcfTopic`/
/// `BcfComment`/`BcfViewpoint`/`BcfCamera`/`BcfComponents`) reuses `../🔺️diff/🦀️.rs`'s
/// own `pub(crate)` binary primitives directly (imported above), same intra-artifact reuse pattern
/// this file's text-form `OpText` impl already established for the string-grammar codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_opt_str_bin(out: &mut Vec<u8>, opt: &Option<String>) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        write_str_lp(out, v);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    Ok(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_list_bin(out: &mut Vec<u8>, items: &[String]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for s in items {
        write_str_lp(out, s);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<String>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(read_str_lp(reader)?);
    }
    Ok(out)
}

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `BcfMutation` variant ordinal; tag 0 (formerly `NoMutation`) is retired rather than reused, so a
/// stray zero tag on the wire fails `decode_op` instead of silently resurrecting a dropped variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_opt_index_bin(out: &mut Vec<u8>, index: &Option<usize>) {
    out.push(if index.is_some() { 1 } else { 0 });
    if let Some(index) = index {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_opt_index_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<usize>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        _ => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
    }
}

impl protocol::OpBinary for BcfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            BcfMutation::SetVersion(_) => TAG_SET_VERSION,
            BcfMutation::InsertTopic(_) => TAG_INSERT_TOPIC,
            BcfMutation::RemoveTopic(_) => TAG_REMOVE_TOPIC,
            BcfMutation::SetTopicMarkup(_) => TAG_SET_TOPIC_MARKUP,
            BcfMutation::InsertComment(_) => TAG_INSERT_COMMENT,
            BcfMutation::RemoveComment(_) => TAG_REMOVE_COMMENT,
            BcfMutation::SetComment(_) => TAG_SET_COMMENT,
            BcfMutation::InsertViewpoint(_) => TAG_INSERT_VIEWPOINT,
            BcfMutation::RemoveViewpoint(_) => TAG_REMOVE_VIEWPOINT,
            BcfMutation::SetViewpointCamera(_) => TAG_SET_VIEWPOINT_CAMERA,
            BcfMutation::SetViewpointComponents(_) => TAG_SET_VIEWPOINT_COMPONENTS,
            BcfMutation::SetViewpointSnapshot(_) => TAG_SET_VIEWPOINT_SNAPSHOT,
            BcfMutation::SetParts(_) => TAG_SET_PARTS,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            BcfMutation::SetVersion(set_version::SetVersion { version }) => write_str_lp(&mut out, version),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic, index }) => {
                enc_topic_bin(topic, &mut out);
                write_opt_index_bin(&mut out, index);
            }
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => write_str_lp(&mut out, guid),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => {
                write_str_lp(&mut out, guid);
                write_opt_str_bin(&mut out, title);
                write_opt_str_bin(&mut out, description);
                write_opt_str_bin(&mut out, status);
                write_opt_str_bin(&mut out, priority);
                out.push(if labels.is_some() { 1 } else { 0 });
                if let Some(v) = labels {
                    write_str_list_bin(&mut out, v);
                }
                write_opt_str_bin(&mut out, creation_date);
                write_opt_str_bin(&mut out, creation_author);
            }
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment, index }) => {
                write_str_lp(&mut out, topic_guid);
                enc_comment_bin(comment, &mut out);
                write_opt_index_bin(&mut out, index);
            }
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
            }
            BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                write_opt_str_bin(&mut out, date);
                write_opt_str_bin(&mut out, author);
                write_opt_str_bin(&mut out, text);
                out.push(if viewpoint_ref.is_some() { 1 } else { 0 });
                if let Some(inner) = viewpoint_ref {
                    write_opt_str_bin(&mut out, inner);
                }
            }
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint, index }) => {
                write_str_lp(&mut out, topic_guid);
                enc_viewpoint_bin(viewpoint, &mut out);
                write_opt_index_bin(&mut out, index);
            }
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
            }
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if camera.is_some() { 1 } else { 0 });
                if let Some(c) = camera {
                    enc_camera_bin(c, &mut out);
                }
            }
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if components.is_some() { 1 } else { 0 });
                if let Some(c) = components {
                    enc_components_bin(c, &mut out);
                }
            }
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if snapshot.is_some() { 1 } else { 0 });
                if let Some(b) = snapshot {
                    write_bytes_lp(&mut out, b);
                }
            }
            BcfMutation::SetParts(set_parts::SetParts { parts }) => {
                store::pack_rt::write_varint_u64(&mut out, parts.len() as u64);
                for part in parts {
                    write_str_lp(&mut out, &part.name);
                    write_bytes_lp(&mut out, &part.data);
                }
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_SET_VERSION => Ok(BcfMutation::SetVersion(set_version::SetVersion { version: read_str_lp(&mut reader).map_err(|e| malformed("op version", reader.position(), e))? })),
            TAG_INSERT_TOPIC => {
                let topic = dec_topic_bin(&mut reader).map_err(|e| malformed("op topic", reader.position(), e))?;
                let index = read_opt_index_bin(&mut reader).map_err(|e| malformed("op index", reader.position(), e))?;
                Ok(BcfMutation::InsertTopic(insert_topic::InsertTopic { topic, index }))
            }
            TAG_REMOVE_TOPIC => Ok(BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))? })),
            TAG_SET_TOPIC_MARKUP => {
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let title = read_opt_str_bin(&mut reader).map_err(|e| malformed("op title", reader.position(), e))?;
                let description = read_opt_str_bin(&mut reader).map_err(|e| malformed("op description", reader.position(), e))?;
                let status = read_opt_str_bin(&mut reader).map_err(|e| malformed("op status", reader.position(), e))?;
                let priority = read_opt_str_bin(&mut reader).map_err(|e| malformed("op priority", reader.position(), e))?;
                let labels = if reader.read_u8().map_err(|e| malformed("op labels presence", reader.position(), e.to_string()))? != 0 { Some(read_str_list_bin(&mut reader).map_err(|e| malformed("op labels", reader.position(), e))?) } else { None };
                let creation_date = read_opt_str_bin(&mut reader).map_err(|e| malformed("op creation_date", reader.position(), e))?;
                let creation_author = read_opt_str_bin(&mut reader).map_err(|e| malformed("op creation_author", reader.position(), e))?;
                Ok(BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }))
            }
            TAG_INSERT_COMMENT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let comment = dec_comment_bin(&mut reader).map_err(|e| malformed("op comment", reader.position(), e))?;
                let index = read_opt_index_bin(&mut reader).map_err(|e| malformed("op index", reader.position(), e))?;
                Ok(BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment, index }))
            }
            TAG_REMOVE_COMMENT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                Ok(BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }))
            }
            TAG_SET_COMMENT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let date = read_opt_str_bin(&mut reader).map_err(|e| malformed("op date", reader.position(), e))?;
                let author = read_opt_str_bin(&mut reader).map_err(|e| malformed("op author", reader.position(), e))?;
                let text = read_opt_str_bin(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                let viewpoint_ref = if reader.read_u8().map_err(|e| malformed("op viewpoint_ref presence", reader.position(), e.to_string()))? != 0 {
                    Some(read_opt_str_bin(&mut reader).map_err(|e| malformed("op viewpoint_ref", reader.position(), e))?)
                } else {
                    None
                };
                Ok(BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }))
            }
            TAG_INSERT_VIEWPOINT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let viewpoint = dec_viewpoint_bin(&mut reader).map_err(|e| malformed("op viewpoint", reader.position(), e))?;
                let index = read_opt_index_bin(&mut reader).map_err(|e| malformed("op index", reader.position(), e))?;
                Ok(BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint, index }))
            }
            TAG_REMOVE_VIEWPOINT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                Ok(BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }))
            }
            TAG_SET_VIEWPOINT_CAMERA => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let camera = if reader.read_u8().map_err(|e| malformed("op camera presence", reader.position(), e.to_string()))? != 0 { Some(dec_camera_bin(&mut reader).map_err(|e| malformed("op camera", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }))
            }
            TAG_SET_VIEWPOINT_COMPONENTS => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let components =
                    if reader.read_u8().map_err(|e| malformed("op components presence", reader.position(), e.to_string()))? != 0 { Some(dec_components_bin(&mut reader).map_err(|e| malformed("op components", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }))
            }
            TAG_SET_VIEWPOINT_SNAPSHOT => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let snapshot = if reader.read_u8().map_err(|e| malformed("op snapshot presence", reader.position(), e.to_string()))? != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }))
            }
            TAG_SET_PARTS => {
                let count = reader.read_varint_u64().map_err(|e| malformed("op parts count", reader.position(), e.to_string()))?;
                let mut parts = Vec::new();
                for _ in 0..count {
                    let name = read_str_lp(&mut reader).map_err(|e| malformed("op part name", reader.position(), e))?;
                    let data = read_bytes_lp(&mut reader).map_err(|e| malformed("op part data", reader.position(), e))?;
                    parts.push(crate::schema::snapshot::BcfRawPart { name, data });
                }
                Ok(BcfMutation::SetParts(set_parts::SetParts { parts }))
            }
            other => Err(malformed("op tag", 1, format!("unknown BcfMutation tag {other}"))),
        }
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `BcfMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_VERSION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-version");
const TAG_INSERT_TOPIC: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-topic");
const TAG_REMOVE_TOPIC: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-topic");
const TAG_SET_TOPIC_MARKUP: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-topic-markup");
const TAG_INSERT_COMMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-comment");
const TAG_REMOVE_COMMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-comment");
const TAG_SET_COMMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-comment");
const TAG_INSERT_VIEWPOINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-viewpoint");
const TAG_REMOVE_VIEWPOINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-viewpoint");
const TAG_SET_VIEWPOINT_CAMERA: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-viewpoint-camera");
const TAG_SET_VIEWPOINT_COMPONENTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-viewpoint-components");
const TAG_SET_PARTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-parts");
const TAG_SET_VIEWPOINT_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-viewpoint-snapshot");
//#endregion 🏷️WireTags
