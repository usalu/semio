
use super::*;
use crate::{ArtifactVersion, DictBuilder, DictReader, HybridLogicalTimestamp, MutationId, RecordHasher};

//#region 🔖️Errors
#[test]
fn pack_error_converts_into_protocol_error_via_from() {
    let pack_err = crate::codec::PackError::Truncated(7);
    let protocol_err: ProtocolError = pack_err.clone().into();
    assert_eq!(protocol_err, ProtocolError::Pack(pack_err));
}
//#endregion 🔖️Errors

//#region 🔖️Limits
#[test]
fn protocol_limits_default_matches_contract() {
    let limits = ProtocolLimits::default();
    assert_eq!(limits.max_file_len, 64 * 1024 * 1024 * 1024);
    assert_eq!(limits.max_frame_len, 2 * 1024 * 1024 * 1024);
    assert_eq!(limits.max_record_count, 256_000_000);
    assert_eq!(limits.max_dict_entries, 1_000_000);
    assert_eq!(limits.max_op_count_per_edit, 100_000);
    assert_eq!(limits.max_total_alloc, 4 * 1024 * 1024 * 1024);
}
//#endregion 🔖️Limits

//#region 🔖️RecordKinds
#[test]
fn record_kind_constants_match_contract() {
    assert_eq!(REC_END, 0x00);
    assert_eq!(REC_DOC, 0x01);
    assert_eq!(REC_ACTOR_DICT, 0x02);
    assert_eq!(REC_STR_DICT, 0x03);
    assert_eq!(REC_EDIT, 0x04);
    assert_eq!(REC_CHANGE, 0x05);
    assert_eq!(REC_CHECKPOINT, 0x06);
    assert_eq!(REC_ALTERNATIVE, 0x07);
    assert_eq!(REC_ACTIVE, 0x08);
    assert_eq!(REC_FRONTIER, 0x09);
    assert_eq!(REC_PROJECTION, 0x0A);
    assert_eq!(REC_INDEX, 0x0B);
    assert_eq!(REC_COMMIT, 0x0C);
    assert_eq!(REC_SIGNATURE, 0x0D);
    assert_eq!(REC_REDACTION, 0x0E);
    assert_eq!(REC_UPCAST, 0x0F);
    assert_eq!(REC_EPHEMERAL, 0x10);
    assert_eq!(REC_SEALED, 0x11);
    assert_eq!(REC_COMPACTION, 0x12);
    assert_eq!(REC_PADDING, 0x7F);
}

#[test]
fn is_critical_kind_matches_contract_set() {
    for kind in [REC_DOC, REC_EDIT, REC_CHANGE, REC_CHECKPOINT, REC_ALTERNATIVE, REC_ACTIVE, REC_COMMIT, REC_ACTOR_DICT, REC_STR_DICT] {
        assert!(is_critical_kind(kind), "{kind:#x} should be critical");
    }
    for kind in [REC_END, REC_FRONTIER, REC_PROJECTION, REC_INDEX, REC_SIGNATURE, REC_REDACTION, REC_UPCAST, REC_EPHEMERAL, REC_SEALED, REC_COMPACTION, REC_PADDING, 0x50] {
        assert!(!is_critical_kind(kind), "{kind:#x} should not be critical");
    }
}
//#endregion 🔖️RecordKinds

//#region 🔖️Flags
#[test]
fn frame_flags_round_trips_codec_id() {
    for codec in 0u8..=7 {
        for compressed in [false, true] {
            for critical in [false, true] {
                let flags = frame_flags(compressed, critical, codec);
                assert_eq!(flags & FRAME_FLAG_COMPRESSED != 0, compressed);
                assert_eq!(flags & FRAME_FLAG_CRITICAL != 0, critical);
                assert_eq!(frame_codec_id(flags), codec);
            }
        }
    }
}

#[test]
fn frame_codec_id_masks_to_three_bits() {
    assert_eq!(frame_codec_id(0b1111_1100), 0b111);
}
//#endregion 🔖️Flags

//#region 🔖️Scalars
mod scalars {
    use crate::codec::{ByteReader, ByteWriter};
    use crate::scalar::{read_id, read_timestamp, write_id, write_timestamp};

    #[test]
    fn timestamp_round_trips_canonical_utc_no_fraction() {
        let raw = "2024-01-15T10:30:00Z";
        let mut out = ByteWriter::new();
        let epoch = write_timestamp(&mut out, raw, None);
        assert!(epoch.is_some());
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
        assert_eq!(decoded, raw);
        assert_eq!(epoch_back, epoch);
    }

    #[test]
    fn timestamp_round_trips_canonical_utc_with_fraction() {
        let raw = "2024-01-15T10:30:00.123Z";
        let mut out = ByteWriter::new();
        write_timestamp(&mut out, raw, None);
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn timestamp_falls_back_to_raw_for_non_canonical_text() {
        let raw = "not-a-timestamp";
        let mut out = ByteWriter::new();
        let epoch = write_timestamp(&mut out, raw, None);
        assert_eq!(epoch, None);
        let bytes = out.into_bytes();
        assert_eq!(bytes[0], 0, "tag byte must be 0 (raw)");
        let mut reader = ByteReader::new(&bytes);
        let (decoded, epoch_back) = read_timestamp(&mut reader, None).unwrap();
        assert_eq!(decoded, raw);
        assert_eq!(epoch_back, None);
    }

    #[test]
    fn timestamp_falls_back_to_raw_for_non_utc_offset() {
        let raw = "2024-01-15T10:30:00+02:00";
        let mut out = ByteWriter::new();
        let epoch = write_timestamp(&mut out, raw, None);
        assert_eq!(epoch, None);
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn timestamp_chain_uses_delta_tag_after_first_absolute() {
        let mut out = ByteWriter::new();
        let e1 = write_timestamp(&mut out, "2024-01-15T10:30:00Z", None).unwrap();
        let e2 = write_timestamp(&mut out, "2024-01-15T10:30:05Z", Some(e1)).unwrap();
        assert_eq!(e2 - e1, 5_000);
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let (d1, p1) = read_timestamp(&mut reader, None).unwrap();
        let (d2, p2) = read_timestamp(&mut reader, p1).unwrap();
        assert_eq!(d1, "2024-01-15T10:30:00Z");
        assert_eq!(d2, "2024-01-15T10:30:05Z");
        assert_eq!(p2, Some(e2));
    }

    #[test]
    fn timestamp_epoch_zero_round_trips() {
        let raw = "1970-01-01T00:00:00Z";
        let mut out = ByteWriter::new();
        let epoch = write_timestamp(&mut out, raw, None);
        assert_eq!(epoch, Some(0));
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let (decoded, _) = read_timestamp(&mut reader, None).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn id_round_trips_via_edit_ordinal_tag() {
        let mut out = ByteWriter::new();
        write_id(&mut out, "edit-7", |_| unreachable!("must not intern"), |id| (id == "edit-7").then_some(7)).unwrap();
        let bytes = out.into_bytes();
        assert_eq!(bytes[0], 3, "tag byte must be 3 (edit-ordinal)");
        let mut reader = ByteReader::new(&bytes);
        let decoded = read_id(&mut reader, |_| unreachable!("must not resolve"), |ordinal| if ordinal == 7 { Ok("edit-7") } else { Err(crate::codec::PackError::Truncated(0)) }).unwrap();
        assert_eq!(decoded, "edit-7");
    }

    #[test]
    fn id_round_trips_via_prefix_uuid_tag() {
        let id = "actor-3fa85f64-5717-4562-b3fc-2c963f66afa6";
        let mut out = ByteWriter::new();
        write_id(
            &mut out,
            id,
            |s| {
                assert_eq!(s, "actor");
                0
            },
            |_| None,
        )
        .unwrap();
        let bytes = out.into_bytes();
        assert_eq!(bytes[0], 2, "tag byte must be 2 (prefix+uuid)");
        let mut reader = ByteReader::new(&bytes);
        let decoded = read_id(&mut reader, |idx| if idx == 0 { Ok("actor") } else { Err(crate::codec::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
        assert_eq!(decoded, id);
    }

    #[test]
    fn id_falls_back_to_dictref_tag_for_plain_strings() {
        let mut out = ByteWriter::new();
        write_id(
            &mut out,
            "hello-world",
            |s| {
                assert_eq!(s, "hello-world");
                42
            },
            |_| None,
        )
        .unwrap();
        let bytes = out.into_bytes();
        assert_eq!(bytes[0], 1, "tag byte must be 1 (dictref)");
        let mut reader = ByteReader::new(&bytes);
        let decoded = read_id(&mut reader, |idx| if idx == 42 { Ok("hello-world") } else { Err(crate::codec::PackError::Truncated(0)) }, |_| unreachable!("must not resolve ordinal")).unwrap();
        assert_eq!(decoded, "hello-world");
    }

    #[test]
    fn id_raw_tag_is_readable_even_though_writer_never_emits_it() {
        let mut out = ByteWriter::new();
        out.write_u8(0);
        out.write_varint_u64(5);
        out.write_bytes(b"hello");
        let bytes = out.into_bytes();
        let mut reader = ByteReader::new(&bytes);
        let decoded = read_id(&mut reader, |_| unreachable!(), |_| unreachable!()).unwrap();
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn id_dictref_dedupes_repeated_ids_through_intern_closure() {
        let mut dict: Vec<String> = Vec::new();
        let mut out = ByteWriter::new();
        {
            let mut intern = |s: &str| {
                if let Some(pos) = dict.iter().position(|e| e == s) {
                    pos as u32
                } else {
                    dict.push(s.to_string());
                    (dict.len() - 1) as u32
                }
            };
            write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
            write_id(&mut out, "same-id", &mut intern, |_| None).unwrap();
        }
        assert_eq!(dict.len(), 1, "second write must reuse the same dictionary slot");
    }
}
//#endregion 🔖️Scalars

//#region 🔖️Dictionary
#[test]
fn dict_builder_interns_deterministically_and_dedupes() {
    let mut builder = DictBuilder::new();
    assert!(builder.is_empty());
    assert_eq!(builder.intern("a"), 0);
    assert_eq!(builder.intern("b"), 1);
    assert_eq!(builder.intern("a"), 0);
    assert_eq!(builder.len(), 2);
    assert_eq!(builder.entries_since(0), &["a".to_string(), "b".to_string()]);
    assert_eq!(builder.entries_since(1), &["b".to_string()]);
}

#[test]
fn dict_reader_round_trips_builder_deltas_in_order() {
    let mut builder = DictBuilder::new();
    builder.intern("x");
    builder.intern("y");
    let mut reader = DictReader::new();
    reader.extend(0, builder.entries_since(0).to_vec()).unwrap();
    assert_eq!(reader.resolve(0).unwrap(), "x");
    assert_eq!(reader.resolve(1).unwrap(), "y");
    assert_eq!(reader.len(), 2);

    builder.intern("z");
    reader.extend(2, builder.entries_since(2).to_vec()).unwrap();
    assert_eq!(reader.resolve(2).unwrap(), "z");
}

#[test]
fn dict_reader_rejects_out_of_order_deltas() {
    let mut reader = DictReader::new();
    let err = reader.extend(5, vec!["late".to_string()]).unwrap_err();
    assert_eq!(err, ProtocolError::DictOutOfOrder { expected: 0, actual: 5 });
}

#[test]
fn dict_reader_reports_miss_past_the_end() {
    let reader = DictReader::new();
    assert_eq!(reader.resolve(0).unwrap_err(), ProtocolError::DictMiss(0));
}
//#endregion 🔖️Dictionary

//#region 🔖️Crypto
struct FixedHasher;
impl RecordHasher for FixedHasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[0] = bytes.len() as u8;
        out
    }
}

#[test]
fn record_hasher_trait_is_object_usable() {
    let hasher = FixedHasher;
    assert_eq!(hasher.hash(b"abc")[0], 3);
}
//#endregion 🔖️Crypto

//#region 🔖️HybridLogicalTimestamp
#[test]
fn hlc_tick_advances_on_newer_physical_time() {
    let mut hlc = HybridLogicalTimestamp::new(1, 100);
    hlc.tick(200);
    assert_eq!(hlc.physical_ms, 200);
    assert_eq!(hlc.logical, 0);
}

#[test]
fn hlc_tick_bumps_logical_on_equal_or_older_physical_time() {
    let mut hlc = HybridLogicalTimestamp::new(1, 100);
    hlc.tick(100);
    assert_eq!(hlc.logical, 1);
    hlc.tick(50);
    assert_eq!(hlc.logical, 2);
}

#[test]
fn hlc_merge_adopts_the_greater_remote_tick_then_bumps() {
    let mut local = HybridLogicalTimestamp::new(1, 100);
    let remote = HybridLogicalTimestamp { actor: 2, physical_ms: 150, logical: 3 };
    local.merge(&remote);
    assert_eq!(local.physical_ms, 150);
    assert_eq!(local.logical, 4);
}

#[test]
fn hlc_ordering_uses_actor_as_final_tiebreak() {
    let a = HybridLogicalTimestamp { actor: 1, physical_ms: 100, logical: 5 };
    let b = HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 5 };
    assert!(a < b, "equal physical_ms/logical must tiebreak by actor, not compare Equal");
    assert_ne!(a.cmp_key(), b.cmp_key());
}

#[test]
fn hlc_ordering_prioritizes_physical_then_logical_then_actor() {
    let older = HybridLogicalTimestamp { actor: 9, physical_ms: 100, logical: 0 };
    let newer_physical = HybridLogicalTimestamp { actor: 0, physical_ms: 101, logical: 0 };
    let newer_logical = HybridLogicalTimestamp { actor: 0, physical_ms: 100, logical: 1 };
    assert!(older < newer_physical);
    assert!(older < newer_logical);
    assert!(newer_logical < newer_physical);
}
//#endregion 🔖️HybridLogicalTimestamp

//#region 🔖️Identifiers
/// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): asserts the same transparent shape directly on `DslValue`.
#[test]
fn identifier_newtypes_to_value_round_trip_transparently() {
    let op = MutationId("op-1".to_string());
    let value = crate::value::ToValue::to_value(&op);
    assert_eq!(value, crate::value::DslValue::String("op-1".to_string()));
    assert_eq!(<MutationId as crate::value::FromValue>::from_value(value).unwrap(), op);

    let version = ArtifactVersion(42);
    let value = crate::value::ToValue::to_value(&version);
    assert_eq!(value, crate::value::DslValue::uint(42));
    assert_eq!(<ArtifactVersion as crate::value::FromValue>::from_value(value).unwrap(), version);
}

#[test]
fn document_version_orders_numerically() {
    assert!(ArtifactVersion(1) < ArtifactVersion(2));
}
//#endregion 🔖️Identifiers

//#region 🔖️Policies
#[test]
fn merge_policy_default_is_normal() {
    assert_eq!(MergePolicy::default(), MergePolicy::Normal);
}

#[test]
fn merge_policy_rejects_matches_the_frozen_matrix() {
    use crate::diagnostic::Severity;
    assert!(!MergePolicy::LaissezFaire.rejects(Severity::Info));
    assert!(!MergePolicy::LaissezFaire.rejects(Severity::Warning));
    assert!(!MergePolicy::LaissezFaire.rejects(Severity::Error));
    assert!(MergePolicy::LaissezFaire.rejects(Severity::Fatal));

    assert!(!MergePolicy::Normal.rejects(Severity::Info));
    assert!(!MergePolicy::Normal.rejects(Severity::Warning));
    assert!(MergePolicy::Normal.rejects(Severity::Error));
    assert!(MergePolicy::Normal.rejects(Severity::Fatal));

    assert!(!MergePolicy::Vigilant.rejects(Severity::Info));
    assert!(MergePolicy::Vigilant.rejects(Severity::Warning));
    assert!(MergePolicy::Vigilant.rejects(Severity::Error));
    assert!(MergePolicy::Vigilant.rejects(Severity::Fatal));
}

#[test]
fn merge_policy_as_u8_from_u8_round_trips() {
    for policy in [MergePolicy::LaissezFaire, MergePolicy::Normal, MergePolicy::Vigilant] {
        assert_eq!(MergePolicy::from_u8(policy.as_u8()), Some(policy));
    }
    assert_eq!(MergePolicy::from_u8(3), None);
}

/// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): round-trips through `ToValue`/`FromValue` instead.
#[test]
fn undo_policy_and_state_class_to_value_round_trip() {
    for policy in [UndoPolicy::ExactBaseOnly, UndoPolicy::TransformAgainstConcurrent, UndoPolicy::SemanticUndo, UndoPolicy::CompensatingAction] {
        let value = crate::value::ToValue::to_value(&policy);
        assert_eq!(<UndoPolicy as crate::value::FromValue>::from_value(value).unwrap(), policy);
    }
    let lanes = [StateClass::Artifact, StateClass::Config, StateClass::Presence, StateClass::Transient];
    for class in lanes {
        let value = crate::value::ToValue::to_value(&class);
        assert_eq!(<StateClass as crate::value::FromValue>::from_value(value).unwrap(), class);
    }
    assert_eq!(lanes.len(), 4, "the state square admits exactly four lanes");
}
//#endregion 🔖️Policies

//#region 🔖️ArtifactInferenceCatalog
fn empty_kernel_facet_leaves() -> KernelFacetLeaves {
    KernelFacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" }
}

#[test]
fn kernel_artifact_inference_catalog_registers_independently_of_the_four_facet_descriptor() {
    let before = kernel_artifact_inference_catalog_len();
    register_kernel_artifact_inference_descriptor(KernelArtifactInferenceDescriptor { id: "s.wave3.synthetic.inference", inference: empty_kernel_facet_leaves() });
    assert!(kernel_artifact_inference_descriptor_registered("s.wave3.synthetic.inference"));
    assert_eq!(kernel_artifact_inference_catalog_len(), before.max(1));
    let mut found = false;
    with_kernel_artifact_inference_catalog(|entries| {
        found = entries.iter().any(|entry| entry.id == "s.wave3.synthetic.inference");
    });
    assert!(found, "registered inference descriptor must be visible via with_kernel_artifact_inference_catalog");
}
//#endregion 🔖️ArtifactInferenceCatalog

//#region 🔖️WireCodec
#[test]
fn wire_str_round_trips_including_multibyte_utf8() {
    let mut out = Vec::new();
    write_str(&mut out, "héllo wörld 🎞️");
    let mut pos = 0;
    assert_eq!(read_str(&out, &mut pos).unwrap(), "héllo wörld 🎞️");
    assert_eq!(pos, out.len());
}

#[test]
fn wire_str_empty_round_trips() {
    let mut out = Vec::new();
    write_str(&mut out, "");
    let mut pos = 0;
    assert_eq!(read_str(&out, &mut pos).unwrap(), "");
    assert_eq!(pos, out.len());
}

#[test]
fn wire_bytes_round_trips_and_consumes_exact_length() {
    let mut out = Vec::new();
    write_bytes(&mut out, &[1, 2, 3, 4, 5]);
    write_bytes(&mut out, &[9]);
    let mut pos = 0;
    assert_eq!(read_bytes(&out, &mut pos).unwrap(), vec![1, 2, 3, 4, 5]);
    assert_eq!(read_bytes(&out, &mut pos).unwrap(), vec![9]);
    assert_eq!(pos, out.len());
}

#[test]
fn wire_hash32_round_trips_fixed_width_no_length_prefix() {
    let hash = [7u8; 32];
    let mut out = Vec::new();
    write_hash32(&mut out, &hash);
    assert_eq!(out.len(), 32, "hash32 must be fixed-width with no length prefix");
    let mut pos = 0;
    assert_eq!(read_hash32(&out, &mut pos).unwrap(), hash);
    assert_eq!(pos, 32);
}

#[test]
fn wire_bool_round_trips_as_a_single_byte() {
    let mut out = Vec::new();
    write_bool(&mut out, true);
    write_bool(&mut out, false);
    assert_eq!(out, vec![1, 0]);
    let mut pos = 0;
    assert!(read_bool(&out, &mut pos).unwrap());
    assert!(!read_bool(&out, &mut pos).unwrap());
}

#[test]
fn wire_varint_u64_round_trips_via_pack_core() {
    let mut out = Vec::new();
    write_varint_u64(&mut out, 300);
    let mut pos = 0;
    assert_eq!(read_varint_u64(&out, &mut pos).unwrap(), 300);
}

#[test]
fn wire_str_rejects_truncated_input() {
    let mut out = Vec::new();
    write_str(&mut out, "hello");
    out.truncate(out.len() - 1);
    let mut pos = 0;
    assert!(matches!(read_str(&out, &mut pos), Err(ProtocolError::Malformed { .. })));
}

#[test]
fn wire_bytes_rejects_truncated_input() {
    let mut out = Vec::new();
    write_bytes(&mut out, &[1, 2, 3]);
    out.truncate(out.len() - 1);
    let mut pos = 0;
    assert!(matches!(read_bytes(&out, &mut pos), Err(ProtocolError::Malformed { .. })));
}

#[test]
fn wire_hash32_rejects_truncated_input() {
    let bytes = [0u8; 10];
    let mut pos = 0;
    assert!(matches!(read_hash32(&bytes, &mut pos), Err(ProtocolError::Malformed { .. })));
}
//#endregion 🔖️WireCodec
