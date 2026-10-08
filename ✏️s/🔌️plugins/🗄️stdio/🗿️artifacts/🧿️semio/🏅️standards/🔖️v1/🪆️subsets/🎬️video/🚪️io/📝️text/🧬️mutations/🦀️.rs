//! 📝️ Text representation grammar surface for `stdio.semio.video` (mutations): the `<keyword>
//! arg=value ...` op grammar — actual print/parse lives on `SemioVideoMutation`'s
//! `protocol::OpText` impl in the facet root `🦀️.rs`; this leaf carries the normative
//! grammar description.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::video::schema::mutations::*;
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::video::schema::diff::{diff_insert_sample, diff_insert_stream, diff_remove_sample, diff_remove_stream, diff_set_sample_data, diff_set_sample_flags, diff_set_stream_meta, SemioVideoDiff};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_stream};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_stream};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_sample};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_sample};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_rational};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_rational};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_kind};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_kind};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_bool};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_bool};
use crate::standards::v1::subsets::document::io::text::mutations::{parse_usize};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::video::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::video::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::video::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::video::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// 📥️ Decodes this subset's internally tagged (`{"mutation": "<camelCaseVariant>", ...}`) wire value — the shape
/// `🎥️mutate-semio-video`'s committed specification vectors and doc strings carry — into a real [`SemioVideoMutation`]. A thin
/// `pack::from_json_str` wrapper over `ToValue`/`FromValue`, so the test adapter reads the committed wire value instead of
/// re-declaring it field by field beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_video_mutation_json(text: &str) -> Result<SemioVideoMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🎙️ Hand-rolled `OpText`/`OpBinary` for `SemioVideoMutation` — reuses the diff module's
/// `pub(crate)` grammar primitives (`hex_encode`/`enc_stream`/`enc_sample`/`split_top_level`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same shape docx's own hand-rolled op codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_video_snapshot(s: &SemioVideoSnapshot) -> String {
    format!("[{},{}]", enc_str(&s.schema), enc_list(&s.streams, enc_stream))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_video_snapshot(s: &str) -> Result<SemioVideoSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, streams] = parts.as_slice() else { return Err(format!("snapshot: expected 2 fields, got {}", parts.len())) };
    Ok(SemioVideoSnapshot { schema: dec_str(schema)?, streams: dec_list(streams, dec_stream)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_video_mutation(m: &SemioVideoMutation) -> String {
    match m {
        SemioVideoMutation::InsertStream(insert_stream::InsertStream { index, stream }) => format!("insert-stream index={} stream={}", index, enc_stream(stream)),
        SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index }) => format!("remove-stream index={index}"),
        SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index, kind, codec, width, height, rate }) => {
            format!("set-stream-meta index={} kind={} codec={} width={} height={} rate={}", index, enc_kind(kind), enc_str(codec), width, height, enc_rational(rate))
        }
        SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index, index, sample }) => format!("insert-sample stream-index={} index={} sample={}", stream_index, index, enc_sample(sample)),
        SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index, index }) => format!("remove-sample stream-index={stream_index} index={index}"),
        SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index, index, data }) => format!("set-sample-data stream-index={} index={} data={}", stream_index, index, hex_encode(data)),
        SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index, index, pts, key }) => format!("set-sample-flags stream-index={} index={} pts={} key={}", stream_index, index, pts, enc_bool(key)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_video_mutation(line: &str) -> Result<SemioVideoMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio video mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio video mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_usize(arg(k)?) };
    match keyword {
        "insert-stream" => Ok(SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: usize_arg("index")?, stream: dec_stream(arg("stream")?)? })),
        "remove-stream" => Ok(SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: usize_arg("index")? })),
        "set-stream-meta" => Ok(SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta {
            index: usize_arg("index")?,
            kind: dec_kind(arg("kind")?)?,
            codec: dec_str(arg("codec")?)?,
            width: arg("width")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            height: arg("height")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            rate: dec_rational(arg("rate")?)?,
        })),
        "insert-sample" => Ok(SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, sample: dec_sample(arg("sample")?)? })),
        "remove-sample" => Ok(SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: usize_arg("stream-index")?, index: usize_arg("index")? })),
        "set-sample-data" => Ok(SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: usize_arg("stream-index")?, index: usize_arg("index")?, data: hex_decode(arg("data")?)? })),
        "set-sample-flags" => Ok(SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags {
            stream_index: usize_arg("stream-index")?,
            index: usize_arg("index")?,
            pts: arg("pts")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            key: dec_bool(arg("key")?)?,
        })),
        other => Err(format!("semio video mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioVideoMutation {
    fn print_op(&self) -> String {
        print_semio_video_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_semio_video_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
}
pub use mutations_codec::*;
