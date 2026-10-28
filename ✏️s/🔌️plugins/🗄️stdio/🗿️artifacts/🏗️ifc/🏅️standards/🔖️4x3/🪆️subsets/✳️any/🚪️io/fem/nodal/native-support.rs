//! 🧪️ Actual native nodal test support uses original caller-owned Pack routes.
use super::*;
use semio_framework_os_kernel::{ArtifactPackReceiving, NativeSnapshotDecodeOwner};
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, RetainedCloneGrant, RetireOwned, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{
    FemMaterialValue, FemNodeValue,
    snapshot::{Fem3dSnapshot, Fem3dSnapshotValue},
    structure::{FemDof, FemElement, FemLoad, FemLoadCase, FemSection},
};
use std::cell::Cell;
pub(super) fn wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 20_000_000, maximum_copy_bytes: 256 * 1024 * 1024, maximum_capacity_bytes: 256 * 1024 * 1024, maximum_release_bytes: 256 * 1024 * 1024, maximum_depth: 128 }
}
pub(super) fn cases() -> serde_json::Value {
    serde_json::from_str(include_str!("cases.json")).unwrap()
}
pub(super) fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("native-cases.json")).unwrap()
}
pub(super) fn dof(name: &str) -> FemDof {
    match name {
        "Tx" => FemDof::Tx,
        "Ty" => FemDof::Ty,
        "Tz" => FemDof::Tz,
        "Rx" => FemDof::Rx,
        "Ry" => FemDof::Ry,
        "Rz" => FemDof::Rz,
        _ => panic!("neutral DOF unknown"),
    }
}
pub(super) fn rank(value: FemDof) -> usize {
    match value {
        FemDof::Tx => 0,
        FemDof::Ty => 1,
        FemDof::Tz => 2,
        FemDof::Rx => 3,
        FemDof::Ry => 4,
        FemDof::Rz => 5,
    }
}
pub(super) fn input(name: &str, bits: u64) -> Fem3dSnapshotValue {
    let mut source = Fem3dSnapshotValue::default();
    source.nodes = vec![FemNodeValue { id: "a".into(), x: 0., y: 0., z: 0. }, FemNodeValue { id: "b".into(), x: 2., y: 0., z: 0. }];
    source.materials = vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2e11, g: 8e10, nu: 0.25, rho: 7850. }];
    source.sections = vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 1e-5, iz: 2e-5, j: 3e-5 }];
    source.elements = vec![FemElement::Bar { id: "member".into(), start: "a".into(), end: "b".into(), material_id: "steel".into(), section_id: "section".into() }];
    source.load_cases = vec![FemLoadCase { id: "case".into(), name: "Case".into(), loads: vec![FemLoad::Nodal { id: "native-zero".into(), node_id: "b".into(), dof: dof(name), value: f64::from_bits(bits) }], self_weight: false }];
    source
}
pub(super) fn retire<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    for _ in 0..1_000_000 {
        if owner.terminal_is_empty() {
            return;
        }
        owner.step(wallet()).unwrap();
    }
    panic!("native fixture close unfinished");
}
pub(super) fn close(decode: &mut NativeDecodeControl<'_>, encode: &mut NativeEncodeControl<'_>) {
    for _ in 0..1_000_000 {
        if !decode.has_retirement_owner() && !encode.has_retirement_owner() {
            return;
        }
        decode.close_retirement_recipient(wallet()).unwrap();
        encode.close_retirement_recipient(wallet()).unwrap();
    }
    panic!("original loaded route recipient close unfinished");
}
pub(super) fn payload(value: &Fem3dSnapshotValue) -> IoPayload {
    let source = domains::receive_fem_uncontrolled(value).unwrap();
    let mut observe = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(256 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let result = {
        let mut original = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, wallet());
        source.encode_exchange_pack(&mut original)
    };
    for _ in 0..1_000_000 {
        if !native.has_retirement_owner() {
            break;
        }
        native.close_retirement_recipient(wallet()).unwrap();
    }
    assert!(!native.has_retirement_owner());
    retire(source);
    result.unwrap()
}
pub(super) fn export(value: &Fem3dSnapshotValue, grant: RetainedCloneGrant, stop: usize) -> (Result<IfcSnapshot, ValueRefusalKind>, usize) {
    let input = payload(value);
    let events = Cell::new(0usize);
    let observe = |events: &Cell<usize>| {
        events.set(events.get() + 1);
        events.get() != stop
    };
    let mut receive = |_| observe(&events);
    let mut publish = |_| observe(&events);
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(256 * 1024 * 1024, &mut receive);
    let mut encode = NativeEncodeControl::new(256 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut original = IoRunControl::new(&mut decode, &mut encode, grant);
    let entry = domains::io_entries().into_iter().find(|entry| entry.from == domains::FEM_DIALECT && entry.into == crate::io::IFC4X3_DIALECT).unwrap();
    let result = (entry.run)(&input, &mut original);
    drop(original);
    close(&mut decode, &mut encode);
    let result = match result {
        Ok(outcome) => {
            let graph = match &outcome.value {
                IoPayload::Binary(bytes) => IfcSnapshot::decode_pack(bytes).unwrap(),
                _ => panic!("native IFC Pack required"),
            };
            retire(outcome.value);
            Ok(graph)
        }
        Err(error) => Err(error.cause.kind),
    };
    retire(input);
    (result, events.get())
}
pub(super) fn graph_payload(graph:&IfcSnapshot)->IoPayload{
 let mut observe=|_|true;let mut recipient=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(256*1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let result={let mut original=semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native,wallet());graph.encode_exchange_pack(&mut original)};for _ in 0..1_000_000{if !native.has_retirement_owner(){break;}native.close_retirement_recipient(wallet()).unwrap();}assert!(!native.has_retirement_owner());IoPayload::Binary(result.unwrap())
}
pub(super) fn import(graph: &IfcSnapshot, grant: RetainedCloneGrant, stop: usize) -> (Result<Fem3dSnapshot, ValueRefusalKind>, usize) {
    let input=graph_payload(graph);
    let events = Cell::new(0usize);
    let observe = |events: &Cell<usize>| {
        events.set(events.get() + 1);
        events.get() != stop
    };
    let mut receive = |_| observe(&events);
    let mut publish = |_| observe(&events);
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(256 * 1024 * 1024, &mut receive);
    let mut encode = NativeEncodeControl::new(256 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut original = IoRunControl::new(&mut decode, &mut encode, grant);
    let entry = domains::io_entries().into_iter().find(|entry| entry.from == crate::io::IFC4X3_DIALECT && entry.into == domains::FEM_DIALECT).unwrap();
    let result = (entry.run)(&input, &mut original);
    let result = match result {
        Ok(outcome) => {
            let received = match &outcome.value {
                IoPayload::Binary(bytes) => Fem3dSnapshot::receive_pack(bytes, &mut original.snapshot_decode().unwrap()).map_err(|error| error.kind),
                _ => panic!("native FEM Pack required"),
            };
            retire(outcome.value);
            received
        }
        Err(error) => Err(error.cause.kind),
    };
    drop(original);
    close(&mut decode, &mut encode);
    retire(input);
    (result, events.get())
}
pub(super) fn records(source: &Fem3dSnapshot) -> Vec<(usize, u64)> {
    source.load_cases[0]
        .loads
        .iter()
        .map(|load| match load {
            FemLoad::Nodal { dof, value, .. } => (rank(*dof), value.to_bits()),
            _ => panic!("unexpected load"),
        })
        .collect()
}
pub(super) fn force(graph: &IfcSnapshot) -> usize {
    graph.entities.iter().position(|row| row.name == "IFCSTRUCTURALLOADSINGLEFORCE").unwrap()
}
