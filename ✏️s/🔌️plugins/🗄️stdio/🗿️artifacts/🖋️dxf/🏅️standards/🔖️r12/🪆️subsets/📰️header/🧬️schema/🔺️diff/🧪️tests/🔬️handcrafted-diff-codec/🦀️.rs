#[allow(unused_imports)]
use std::{collections::BTreeMap, collections::BTreeSet, collections::HashSet};
#[allow(unused_imports)]
use crate::{schema::snapshot::DxfBlock, schema::snapshot::DxfEntity, schema::snapshot::DxfArc, schema::snapshot::DxfCircle, schema::snapshot::DxfInsert, schema::snapshot::DxfLine, schema::snapshot::DxfOther, schema::snapshot::DxfPolyline, schema::snapshot::DxfSolid, schema::snapshot::DxfText, schema::snapshot::DxfHeaderVar, schema::snapshot::DxfLayer, schema::snapshot::DxfLinetype, schema::snapshot::DxfOtherTable, schema::snapshot::DxfStyle, schema::snapshot::DxfTables, schema::snapshot::DxfValue, schema::snapshot::DxfVertex};
#[allow(unused_imports)]
use crate::DxfSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use protocol::command::DiffAlgebra;
#[allow(unused_imports)]
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
#[allow(unused_imports)]
use collection_algebra_core::*;
use crate::standards::v_r12::subsets::any::schema::diff::algebra as collection_algebra_core;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use crate::standards::v_r12::subsets::any::schema::diff::algebra::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

/// 🧪️ `DiffCodec` text/binary round-trip laws over every `demo_diff_cases()` fixture (`#region
/// 🔖️DemoCases` above) — the empty diff, a single-collection sparse diff, and the rich case
/// exercising every collection triple (name-keyed AND index-keyed) simultaneously, plus the
/// `Replace` (kind-change) branch of `DxfEntityDiff` and a NON-`Replace` kind-specific patch,
/// plus a nested block-level `entities` sub-diff (the SAME `DxfEntitiesDiff` machinery reused
/// at two tree depths) — shared with `⚙️engine/🦀️.rs`'s own conformance laws.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must never contain a newline, for {d:?}");
        let parsed = DxfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e:?}, for {d:?}"));
        assert_eq!(parsed, d, "parse_diff(print_diff(d)) == d");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e:?}, for {d:?}"));
        let decoded = DxfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e:?}, for {d:?}"));
        assert_eq!(decoded, d, "decode_diff(encode_diff(d)) == d");

        let printed2 = d.print_diff();
        assert_eq!(printed, printed2, "print_diff must be deterministic, for {d:?}");
    }

    assert!(DxfDiff::default().print_diff().is_empty());
    assert_eq!(DxfDiff::parse_diff("").expect("parse empty"), DxfDiff::default());
}
