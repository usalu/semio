//! 🧪️ Actual source-owned STEP receiving uses the original wallet and original source offsets.
use super::*;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 16_000_000, maximum_copy_bytes: 1024 * 1024 * 1024, maximum_capacity_bytes: 128 * 1024 * 1024, maximum_release_bytes: 128 * 1024 * 1024, maximum_depth: 512 }
}
fn input(body: &str) -> String {
    format!("ISO-10303-21;\r\nHEADER;FILE_DESCRIPTION(('x'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;{body}ENDSEC;END-ISO-10303-21;")
}
fn retire<T: semio_framework_value::RetireOwned>(value: T) {
    let mut r = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !r.terminal_is_empty() {
        r.step(grant()).unwrap();
    }
}
#[test]
fn actual_original_step_receiver_preserves_complete_literal_graph() {
    let text = input("#18446744073709551615=(A(\"17\",-9223372036854775808,-0.,'\\PA\\\\S\\!')B(T((#18446744073709551615,$,*))));");
    let source = TxtSnapshot::from_body(&text);
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant());
    let graph = read(&source, &mut owner).unwrap();
    assert_eq!(graph.entities[0].id, u64::MAX);
    assert_eq!(graph.entities[0].complex.len(), 1);
    assert_eq!(graph.entities[0].args[0], IfcValue::Binary("17".into()));
    assert_eq!(graph.entities[0].args[1], IfcValue::Integer(i64::MIN));
    let IfcValue::Real(real) = graph.entities[0].args[2] else { panic!() };
    assert_eq!(real.to_bits(), (-0.0f64).to_bits());
    assert_eq!(graph.entities[0].args[3], IfcValue::String("¡".into()));
    drop(owner);
    assert!(!native.has_retirement_owner());
    retire((source, graph));
}
#[test]
fn actual_original_step_refusal_keeps_original_utf8_offset() {
    let text = input("#1=A('界');#2=A(#0);");
    let expected = text.find("#0").unwrap();
    let source = TxtSnapshot::from_body(&text);
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant());
    let error = read(&source, &mut owner).unwrap_err();
    assert_eq!(error.offset, expected);
    assert_eq!(error.cause.kind, ValueRefusalKind::InvalidValue);
    drop(owner);
    while native.has_retirement_owner() {
        native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
    }
    retire(error);
    retire(source);
}
#[test]
fn original_step_denied_axes_do_not_publish_or_replace_authority() {
    for axis in 0..5 {
        let source = TxtSnapshot::from_body(&input("#1=A('text');"));
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let identity = (native.ledger_identity(), native.retirement_recipient_identity());
        let mut g = grant();
        match axis {
            0 => g.maximum_items = 0,
            1 => g.maximum_copy_bytes = 0,
            2 => g.maximum_capacity_bytes = 0,
            3 => g.maximum_release_bytes = 0,
            _ => g.maximum_depth = 0,
        };
        let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, g);
        assert!(read(&source, &mut owner).is_err());
        drop(owner);
        assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
        while native.has_retirement_owner() {
            native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
        }
        retire(source);
    }
}

#[test]
fn original_step_reader_consumes_all_owned_lexical_witnesses() {
    let pages: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/cases.json")).unwrap();
    for row in pages["cases"].as_array().unwrap() {
        let source = TxtSnapshot::from_body(&input(&format!("#1=IFCLABEL('{}');", row["source"].as_str().unwrap())));
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = read(&source, &mut owner);
        drop(owner);
        if let Some(expected) = row["expected"].as_str() {
            let graph = result.unwrap_or_else(|e| panic!("{}: {e}", row["name"]));
            assert_eq!(graph.entities[0].args[0], IfcValue::String(expected.into()), "{}", row["name"]);
            retire(graph);
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.cause.kind, ValueRefusalKind::InvalidValue);
            retire(error);
            while native.has_retirement_owner() {
                native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap();
            }
        }
        retire(source);
    }
    let cases: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/control-cases.json")).unwrap();
    for row in cases["lexical"].as_array().unwrap() {
        let source = TxtSnapshot::from_body(&input(row["data"].as_str().unwrap()));
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let graph = read(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant())).unwrap();
        assert_eq!(graph.entities[0].name, row["nameAfter"].as_str().unwrap());
        let expected = if let Some(binary) = row["binary"].as_str() { IfcValue::Binary(binary.into()) } else { IfcValue::String(row["value"].as_str().unwrap().into()) };
        assert_eq!(graph.entities[0].args[0], expected);
        retire((source, graph));
    }
}

#[test]
fn original_step_reader_checks_the_46_pinned_official_sources() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️fixtures/official");
    let cases: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join("cases.json")).unwrap()).unwrap();
    assert_eq!(cases["cases"].as_array().unwrap().len(), 46);
    for row in cases["cases"].as_array().unwrap() {
        let text = std::fs::read_to_string(root.join(row["file"].as_str().unwrap())).unwrap();
        let source = TxtSnapshot::from_body(&text);
        let mut callback = |_| true;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let graph = read(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant())).unwrap_or_else(|e| panic!("{}: {e}", row["file"]));
        let expected = super::super::parse_ifc4x3_step(&text).unwrap();
        assert_eq!(graph.entities.len(), row["entities"].as_u64().unwrap() as usize, "independent IfcOpenShell entity count: {}", row["file"]);
        assert_eq!(graph, expected, "{}", row["file"]);
        retire((source, graph, expected));
    }
}

#[test]
fn original_step_heap_receipts_and_completed_cancellation_prefixes_share_one_recipient() {
    use semio_framework_value::RetainedCloneProgress;
    use std::cell::Cell;
    let source = TxtSnapshot::from_body(&input(&format!("#1=A('{}',((1.,2.,3.)),#1);", "x".repeat(8192))));
    let peak = Cell::new(0usize);
    let mut callback = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| {
        peak.set(peak.get().max(p.completed));
        true
    };
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let ((result, progress), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant());
        let result = read(&source, &mut owner);
        (result, owner.progress())
    });
    assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
    retire(result.unwrap());
    drop(native);
    for cutoff in [1, peak.get() / 4, peak.get() / 2, peak.get().saturating_sub(1)] {
        let mut callback = |p: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| p.completed < cutoff;
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut native = NativeDecodeControl::new(128 * 1024 * 1024, &mut callback);
        native.install_retirement_recipient(&mut recipient).unwrap();
        let identity = (native.ledger_identity(), native.retirement_recipient_identity());
        let ((result, mut progress), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let mut owner = semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, grant());
            let result = read(&source, &mut owner);
            (result, owner.progress())
        });
        assert_eq!(result.as_ref().unwrap_err().cause.kind, ValueRefusalKind::Canceled);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        retire(result.unwrap_err());
        for _ in 0..2_000_000 {
            if !native.has_retirement_owner() {
                break;
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| native.with_retirement(grant(), |loan| loan.close_retirement_recipient(grant())).unwrap());
            let receipt = step.progress();
            assert_ne!(receipt, RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            progress = progress.checked_add(receipt).unwrap();
        }
        assert!(!native.has_retirement_owner());
        assert_eq!((native.ledger_identity(), native.retirement_recipient_identity()), identity);
        assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
    }
    retire(source);
}

#[test]
fn actual_registered_txt_ifc_route_returns_an_original_native_graph_pack() {
    use pack::snapshot::ArtifactPack;
    use semio_framework_os_kernel::io::{io_mechanism::IoRunControl, io_schema::IoPayload};
    let source = TxtSnapshot::from_body(include_str!("🧪️tests/triangulated.ifc"));
    let input = IoPayload::Binary(source.encode_pack());
    let mut receive = |_| true;
    let mut publish = |_| true;
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(128 * 1024 * 1024, &mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant());
    let entry = crate::native_exchange::io_entries().iter().find(|e| e.from == super::super::TXT_DIALECT && e.into == super::super::IFC4X3_DIALECT).unwrap();
    let output = (entry.run)(&input, &mut control).unwrap();
    let IoPayload::Binary(bytes) = &output.value else { panic!() };
    let graph = IfcSnapshot::decode_pack(bytes).unwrap();
    assert_eq!(graph.entities.len(), 35);
    drop(control);
    assert!(!decode.has_retirement_owner() && !encode.has_retirement_owner());
    if let Ok(path) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(std::path::Path::new(&path).join("native-step-import.ifc"), super::super::write_ifc4x3_step(&graph).unwrap()).unwrap();
    }
    retire((source, input, output, graph));
}
