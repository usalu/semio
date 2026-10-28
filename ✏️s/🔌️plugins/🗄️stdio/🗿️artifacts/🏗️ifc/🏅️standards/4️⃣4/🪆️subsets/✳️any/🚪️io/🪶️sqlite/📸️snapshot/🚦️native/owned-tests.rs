//! 🧪️ Actual original Pack receiving preserves literal IFC ownership independently of STEP schema restrictions.
use super::*;
use semio_framework_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner};
use semio_framework_value::RetainedCloneGrant;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 2_000_000, maximum_copy_bytes: 64 * 1024 * 1024, maximum_capacity_bytes: 64 * 1024 * 1024, maximum_release_bytes: 64 * 1024 * 1024, maximum_depth: 512 }
}
fn drain(native: &mut NativeDecodeControl<'_>, mut progress: semio_framework_value::RetainedCloneProgress) {
    for _ in 0..2_000_000 {
        if !native.has_retirement_owner() {
            assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
            return;
        }
        let step = native.with_retirement(grant(), |original| original.close_retirement_recipient(grant())).unwrap();
        progress = progress.checked_add(step.progress()).unwrap();
    }
    panic!("original IFC Pack fixture failed funded terminal drain");
}
fn snapshot() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader {
            file_description: vec![IfcValue::Aggregate(vec![IfcValue::String("ViewDefinition [ReferenceView]".into())]), IfcValue::String("2;1".into())],
            file_name: vec![IfcValue::String("original.ifc".into())],
            file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])],
        },
        entities: vec![IfcEntity {
            id: u64::MAX,
            name: String::new(),
            args: Vec::new(),
            complex: vec![IfcComplexType {
                name: "A".into(),
                args: vec![IfcValue::Binary("17".into()), IfcValue::TypedValue(IfcTypedValue { name: "B".into(), items: vec![IfcValue::Real(f64::from_bits(0x7ff8_0000_0000_0042)), IfcValue::Integer(i64::MIN), IfcValue::Reference(u64::MAX)] })],
            }],
        }],
    }
}
#[test]
fn original_pack_receiving_preserves_owned_literal_values() {
    let source = snapshot();
    let wire = encode_pack(&source, &Default::default()).unwrap();
    let mut observe = |_| true;
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(64 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let received = <IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire, &mut owner).unwrap();
    assert!(crate::standards::v4::subsets::any::schema::snapshot::ifc_arguments_identical(&source.entities[0].complex[0].args, &received.entities[0].complex[0].args));
    assert_eq!(source.header, received.header);
    assert_eq!(source.entities[0].id, received.entities[0].id);
    assert!(owner.progress().copied_items > 0);
    drop(owner);
    close(received);
    close(source);
}
#[test]
fn original_pack_receiving_does_not_replace_the_callers_cancelled_decoder() {
    let source = snapshot();
    let wire = encode_pack(&source, &Default::default()).unwrap();
    let mut observe = |_| false;
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(64 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let error = <IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire, &mut owner).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    drop(owner);
    close(source);
}

#[test]
fn original_ifc_pack_publication_matches_the_independent_reference_codec() {
    let source = snapshot();
    let expected = encode_pack(&source, &Default::default()).unwrap();
    let mut observer = |_| true;
    let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(64 * 1024 * 1024, &mut observer);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant());
    let wire = source.encode_exchange_pack(&mut owner).unwrap();
    assert_eq!(wire, expected);
    assert!(owner.progress().copied_items > 0);
    drop(owner);
    close(source);
}

#[test]
fn original_ifc_pack_cancelled_prefixes_drain_under_the_same_original_recipient() {
    use semio_framework_os_kernel::{OriginalArtifactPackReceiving, OriginalArtifactSnapshotReceiving};
    for prefix in [0, 1, 8, 64, 512, 4096] {
        let source = snapshot();
        let wire = encode_pack(&source, &Default::default()).unwrap();
        let live = std::cell::Cell::new(true);
        let mut observer = |_| live.get();
        let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(64 * 1024 * 1024, &mut observer);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = owner.receive_nested::<super::owned::IfcOriginalPackReceiving, IfcSnapshot>(|slot, original| {
            let g = original.remaining_grant();
            let (rx, p) = IfcSnapshot::admit_original_pack(&wire, Default::default(), original.native(), g)?.unwrap();
            *slot = Some(rx);
            original.record_progress(p)?;
            for _ in 0..prefix {
                let rx = slot.as_mut().unwrap();
                if rx.is_ready() {
                    break;
                }
                let g = original.remaining_grant();
                let before = rx.demands(&wire, 4096)?;
                assert_eq!(rx.step(&wire, original.native(), 4096, RetainedCloneGrant { maximum_items: 0, ..g })?, Default::default());
                assert_eq!(rx.demands(&wire, 4096)?, before);
                let p = rx.step(&wire, original.native(), 4096, g)?;
                original.record_progress(p)?;
            }
            live.set(false);
            original.native().checkpoint()?;
            unreachable!()
        });
        assert_eq!(result.unwrap_err().kind, ValueRefusalKind::Canceled);
        let p = owner.progress();
        drop(owner);
        drain(&mut native, p);
        assert!(!native.has_retirement_owner());
        close(source);
    }
}

fn node(children: Vec<usize>) -> Node {
    Node { kind: if children.is_empty() { "unset" } else { "aggregate" }.into(), integer: None, real: None, text: None, reference: None, name: None, children }
}
#[test]
fn invalid_flat_forests_refuse_without_publishing_a_graph() {
    for case in 0..4 {
        let values = match case {
            0 => vec![node(vec![0])],
            1 => vec![node(vec![1, 1]), node(Vec::new())],
            2 => vec![node(Vec::new()), node(Vec::new())],
            _ => (0..67).map(|i| node(if i == 66 { Vec::new() } else { vec![i + 1] })).collect(),
        };
        let frame = Frame { schema: "stdio.ifc".into(), file_description: vec![0], file_name: Vec::new(), file_schema: Vec::new(), entities: Vec::new(), values };
        let body = store::pack_rt::encode_document(&Frame::__dsl_spec(), &frame.__dsl_to_record(), &Default::default()).unwrap();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(STDIO_IFC_DOCUMENT_SCHEMA, store::semio_format::Component::Pack, 1).unwrap();
        let wire = store::semio_format::wrap_binary(&envelope, &body);
        let mut observer = |_| true;
        let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(64 * 1024 * 1024, &mut observer);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant());
        let error = <IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire, &mut owner).unwrap_err();
        assert_eq!(error.kind, if case == 3 { ValueRefusalKind::DepthLimit } else { ValueRefusalKind::InvalidValue });
        let p = owner.progress();
        drop(owner);
        drain(&mut native, p);
        assert!(!native.has_retirement_owner());
        close(frame);
    }
}

#[test]
fn graph_completed_threshold_cancels_after_an_accepted_binary_turn_and_drains_original_custody() {
    use semio_framework_os_kernel::{OriginalArtifactPackReceiving, OriginalArtifactSnapshotReceiving};
    use semio_framework_value::{RetainedCloneProgress, ValueError};
    use std::cell::Cell;
    let mut source = snapshot();
    source.entities[0].complex[0].args[0] = IfcValue::Binary(format!("0{}", "F".repeat(8192)));
    let wire = encode_pack(&source, &Default::default()).unwrap();
    let threshold = Cell::new(None::<usize>);
    let completed = Cell::new(0usize);
    let calls = Cell::new(0usize);
    let was_binary = Cell::new(false);
    let returned = Cell::new(RetainedCloneProgress::default());
    let mut observer = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| {
        calls.set(calls.get() + 1);
        completed.set(p.completed);
        threshold.get().is_none_or(|at| p.completed < at)
    };
    let mut recipient = semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(64 * 1024 * 1024, &mut observer);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let identity = (native.ledger_identity(), native.retirement_recipient_identity());
    let mut original = NativeSnapshotDecodeOwner::new(&mut native, grant());
    let result = original.receive_nested::<super::owned::IfcOriginalPackReceiving, IfcSnapshot>(|slot, original| {
        let g = original.remaining_grant();
        let (rx, p) = IfcSnapshot::admit_original_pack(&wire, Default::default(), original.native(), g)?.unwrap();
        *slot = Some(rx);
        original.record_progress(p)?;
        for _ in 0..1_000_000 {
            if slot.as_ref().unwrap().graph_phase().is_some() {
                break;
            }
            let g = original.remaining_grant();
            let p = slot.as_mut().unwrap().step(&wire, original.native(), 4096, g)?;
            original.record_progress(p)?;
        }
        assert_eq!(slot.as_ref().unwrap().graph_phase(), Some(false));
        let g = original.remaining_grant();
        let p = slot.as_mut().unwrap().step(&wire, original.native(), 1, g)?;
        original.record_progress(p)?;
        let observed = calls.get();
        assert_eq!(slot.as_mut().unwrap().step(&wire, original.native(), 1, RetainedCloneGrant { maximum_items: 0, ..g })?, Default::default());
        assert_eq!(calls.get(), observed);
        threshold.set(Some(256));
        original.drive_cursor(slot, |rx, native, g| {
            if rx.is_ready() {
                return (Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "graph finished without its completed-work cancellation")), Default::default());
            }
            let binary = rx.graph_phase() == Some(true);
            match rx.step(&wire, native, 1, g) {
                Ok(p) => (Ok(None), p),
                Err(e) => {
                    let p = e.retained_progress();
                    was_binary.set(binary);
                    returned.set(p);
                    (Err(e), p)
                }
            }
        })
    });
    assert_eq!(result.unwrap_err().kind, ValueRefusalKind::Canceled);
    assert_eq!(completed.get(), 256);
    assert!(was_binary.get());
    assert_eq!(returned.get().copied_items, 1);
    assert!(returned.get().copied_bytes > 0);
    let progress = original.progress();
    drop(original);
    assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
    assert!(native.has_retirement_owner());
    drain(&mut native, progress);
    assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
    assert!(!native.has_retirement_owner());
    close(source);
}
