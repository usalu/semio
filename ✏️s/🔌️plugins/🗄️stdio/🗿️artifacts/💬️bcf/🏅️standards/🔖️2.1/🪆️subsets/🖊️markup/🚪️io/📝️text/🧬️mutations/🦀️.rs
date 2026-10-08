//! 📝️ Text representation codec surface for `stdio.bcf` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

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

/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `BcfMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above via a real `cargo check` error) — reuses the diff module's `pub(crate)` grammar
/// primitives (`enc_str`/`enc_camera`/`enc_topic`/`encode_option`/...) rather than duplicating them
/// a second time in this file, same pattern `SvgMutation` established. Grammar: `keyword arg=value
/// ...` (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_index(s: &str) -> Result<usize, String> {
    s.parse::<usize>().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_bcf_mutation(m: &BcfMutation) -> String {
    match m {
        BcfMutation::SetVersion(set_version::SetVersion { version }) => format!("set-version version={}", enc_str(version)),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic, index }) => format!("insert-topic topic={} index={}", enc_topic(topic), encode_option(index, |i: &usize| i.to_string())),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => format!("remove-topic guid={}", enc_str(guid)),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => format!(
            "set-topic-markup guid={} title={} description={} status={} priority={} labels={} creation-date={} creation-author={}",
            enc_str(guid),
            encode_option(title, |v: &String| enc_str(v)),
            encode_option(description, |v: &String| enc_str(v)),
            encode_option(status, |v: &String| enc_str(v)),
            encode_option(priority, |v: &String| enc_str(v)),
            encode_option(labels, |v: &Vec<String>| enc_list(v, |s| enc_str(s))),
            encode_option(creation_date, |v: &String| enc_str(v)),
            encode_option(creation_author, |v: &String| enc_str(v)),
        ),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment, index }) => format!("insert-comment topic-guid={} comment={} index={}", enc_str(topic_guid), enc_comment(comment), encode_option(index, |i: &usize| i.to_string())),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => format!("remove-comment topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => format!(
            "set-comment topic-guid={} guid={} date={} author={} text={} viewpoint-ref={}",
            enc_str(topic_guid),
            enc_str(guid),
            encode_option(date, |v: &String| enc_str(v)),
            encode_option(author, |v: &String| enc_str(v)),
            encode_option(text, |v: &String| enc_str(v)),
            encode_option(viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        ),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint, index }) => format!("insert-viewpoint topic-guid={} viewpoint={} index={}", enc_str(topic_guid), enc_viewpoint(viewpoint), encode_option(index, |i: &usize| i.to_string())),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => format!("remove-viewpoint topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => format!("set-viewpoint-camera topic-guid={} guid={} camera={}", enc_str(topic_guid), enc_str(guid), encode_option(camera, enc_camera)),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => {
            format!("set-viewpoint-components topic-guid={} guid={} components={}", enc_str(topic_guid), enc_str(guid), encode_option(components, enc_components))
        }
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => {
            format!("set-viewpoint-snapshot topic-guid={} guid={} snapshot={}", enc_str(topic_guid), enc_str(guid), encode_option(snapshot, |b: &Vec<u8>| enc_bytes(b)))
        }
        BcfMutation::SetParts(set_parts::SetParts { parts }) => format!("set-parts parts={}", enc_list(parts, enc_part)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bcf_mutation(line: &str) -> Result<BcfMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("bcf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("bcf mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-version" => Ok(BcfMutation::SetVersion(set_version::SetVersion { version: dec_str(arg("version")?)? })),
        "insert-topic" => Ok(BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: dec_topic(arg("topic")?)?, index: decode_option(arg("index")?, dec_index)? })),
        "remove-topic" => Ok(BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: dec_str(arg("guid")?)? })),
        "set-topic-markup" => Ok(BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
            guid: dec_str(arg("guid")?)?,
            title: decode_option(arg("title")?, dec_str)?,
            description: decode_option(arg("description")?, dec_str)?,
            status: decode_option(arg("status")?, dec_str)?,
            priority: decode_option(arg("priority")?, dec_str)?,
            labels: decode_option(arg("labels")?, |s| dec_list(s, dec_str))?,
            creation_date: decode_option(arg("creation-date")?, dec_str)?,
            creation_author: decode_option(arg("creation-author")?, dec_str)?,
        })),
        "insert-comment" => Ok(BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: dec_str(arg("topic-guid")?)?, comment: dec_comment(arg("comment")?)?, index: decode_option(arg("index")?, dec_index)? })),
        "remove-comment" => Ok(BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? })),
        "set-comment" => Ok(BcfMutation::SetComment(set_comment::SetComment {
            topic_guid: dec_str(arg("topic-guid")?)?,
            guid: dec_str(arg("guid")?)?,
            date: decode_option(arg("date")?, dec_str)?,
            author: decode_option(arg("author")?, dec_str)?,
            text: decode_option(arg("text")?, dec_str)?,
            viewpoint_ref: decode_option(arg("viewpoint-ref")?, |s| decode_option(s, dec_str))?,
        })),
        "insert-viewpoint" => Ok(BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, viewpoint: dec_viewpoint(arg("viewpoint")?)?, index: decode_option(arg("index")?, dec_index)? })),
        "remove-viewpoint" => Ok(BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? })),
        "set-viewpoint-camera" => Ok(BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, camera: decode_option(arg("camera")?, dec_camera)? })),
        "set-viewpoint-components" => {
            Ok(BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, components: decode_option(arg("components")?, dec_components)? }))
        }
        "set-viewpoint-snapshot" => Ok(BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, snapshot: decode_option(arg("snapshot")?, dec_bytes)? })),
        "set-parts" => Ok(BcfMutation::SetParts(set_parts::SetParts { parts: dec_list(arg("parts")?, dec_part)? })),
        other => Err(format!("bcf mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for BcfMutation {
    fn print_op(&self) -> String {
        print_bcf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_bcf_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
