use super::*;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{
    FemMaterialValue, FemNodeValue,
    snapshot::Fem3dSnapshotValue,
    structure::{FemElement, FemSection},
};
fn wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1024 * 1024, maximum_capacity_bytes: 256 * 1024 * 1024, maximum_release_bytes: 256 * 1024 * 1024, maximum_depth: 128 }
}
fn input() -> Fem3dSnapshotValue {
    let mut v = Fem3dSnapshotValue::default();
    v.nodes = vec![FemNodeValue { id: "a".into(), x: 0., y: 0., z: 0. }, FemNodeValue { id: "b".into(), x: 1., y: 0., z: 0. }, FemNodeValue { id: "c".into(), x: 2., y: 0., z: 0. }];
    v.materials = vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 2e11, g: 8e10, nu: 0.25, rho: 7850. }];
    v.sections = vec![FemSection { id: "s".into(), name: "S".into(), area: 0.01, iy: 2e-5, iz: 3e-5, j: 4e-5 }];
    v.elements = vec![
        FemElement::Bar { id: "ab".into(), start: "a".into(), end: "b".into(), material_id: "steel".into(), section_id: "s".into() },
        FemElement::Bar { id: "bc".into(), start: "b".into(), end: "c".into(), material_id: "steel".into(), section_id: "s".into() },
    ];
    v
}
fn ready<T>(future: impl std::future::Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(v) => v,
        std::task::Poll::Pending => panic!("native exchange route unexpectedly yielded without an asynchronous dependency"),
    }
}
fn retire<T: semio_framework_value::RetireOwned>(value: T) {
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    for _ in 0..1_000_000 {
        if close.terminal_is_empty() {
            return;
        }
        close.step(wallet()).unwrap();
    }
    panic!("fixture failed to close");
}
#[test]
fn actual_registered_fem_host_pack_roundtrip_keeps_shared_definitions() {
    let source = domains::receive_fem_uncontrolled(&input()).unwrap();
    let mut observe = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let graph = fem_build::receive(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet())).unwrap();
    let mut receive = |_| true;
    let mut publish = |_| true;
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(256 * 1024 * 1024, &mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new(256 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, wallet());
    let native_input = IoPayload::Binary(graph.encode_pack());
    let entry = domains::io_entries().into_iter().find(|e| e.from == crate::io::IFC4X3_DIALECT && e.into == domains::FEM_DIALECT).unwrap();
    let result = (entry.run)(&native_input, &mut control).unwrap();
    let payload = result.value;
    assert!(!result.diagnostics.is_empty());
    assert_eq!(control.direction(), semio_framework_os_kernel::io::io_mechanism::IoNativeDirection::Decode);
    assert!(control.progress().retained_capacity_bytes > 0);
    drop(control);
    assert!(encode.owned_bytes() > 0);
    assert!(!decode.has_retirement_owner());
    assert!(!encode.has_retirement_owner());
    let mut control = IoRunControl::new(&mut decode, &mut encode, wallet());
    let entry = domains::io_entries().into_iter().find(|e| e.from == domains::FEM_DIALECT && e.into == crate::io::IFC4X3_DIALECT).unwrap();
    let output = (entry.run)(&payload, &mut control).unwrap().value;
    let IoPayload::Binary(bytes) = output else { unreachable!() };
    let restored_graph = IfcSnapshot::decode_pack(&bytes).unwrap();
    drop(control);
    assert!(!decode.has_retirement_owner());
    assert!(!encode.has_retirement_owner());
    let restored = fem_owned::receive(&restored_graph, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet())).unwrap();
    assert_eq!(restored.nodes.len(), 3);
    assert_eq!(restored.elements.len(), 2);
    assert_eq!(restored.materials.len(), 1);
    assert_eq!(restored.sections.len(), 1);
    assert_eq!(restored.sections[0].j, 4e-5);
    let (a, b) = match (&restored.elements[0], &restored.elements[1]) {
        (FemElement::Bar { material_id: a, section_id: x, .. }, FemElement::Bar { material_id: b, section_id: y, .. }) => {
            assert_eq!(x, y);
            (a, b)
        }
        _ => unreachable!(),
    };
    assert_eq!(a, b);
    if let Ok(path) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(std::path::Path::new(&path).join("native-fem-owned.ifc"), crate::io::write_ifc4x3_step(&restored_graph).unwrap()).unwrap();
    }
    retire((source, restored));
    retire((graph,restored_graph));
    retire((native_input,payload,bytes));
}
#[test]
fn native_owned_mapping_preserves_original_terminal_cancellation() {
    let source = domains::receive_fem_uncontrolled(&input()).unwrap();
    let mut callback = |_| false;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let error = fem_build::receive(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet())).unwrap_err();
    assert_eq!(error.kind, ValueRefusalKind::Canceled);
    retire(source);
}

#[test]
fn original_registered_fem_context_refuses_one_sided_identity_directions(){
    for present in [1,2]{
        let source=domains::receive_fem_uncontrolled(&input()).unwrap();let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(256*1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();
        let mut graph=fem_build::receive(&source,&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native,wallet())).unwrap();
        let next=graph.entities.len()as u64+1;graph.entities.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity{id:next,name:"IFCDIRECTION".into(),args:vec![graph::a(if present==1{vec![graph::r(0.),graph::r(0.),graph::r(1.)]}else{vec![graph::r(1.),graph::r(0.),graph::r(0.)]})],complex:Vec::new()});
        let axis=graph.entities.iter_mut().find(|entity|entity.name=="IFCAXIS2PLACEMENT3D").unwrap();assert_eq!(&axis.args[1..],&[IfcValue::Unset,IfcValue::Unset]);axis.args[present]=IfcValue::Reference(next);
        let mut receive=|_|true;let mut publish=|_|true;let mut dr=NativeDecodeRetirementRecipient::new();let mut er=NativeEncodeRetirementRecipient::new();let mut decode=NativeDecodeControl::new(256*1024*1024,&mut receive);let mut encode=semio_framework_value::NativeEncodeControl::new(256*1024*1024,&mut publish);decode.install_retirement_recipient(&mut dr).unwrap();encode.install_retirement_recipient(&mut er).unwrap();
        let mut control=IoRunControl::new(&mut decode,&mut encode,wallet());let result=ready(<domains::IfcIntoFem as Serializer<IfcSnapshot>>::serialize(&graph,&ArchiveChildren::empty(),&mut control));assert_eq!(result.unwrap_err().cause.kind,ValueRefusalKind::InvalidValue);drop(control);
        for _ in 0..1_000_000{if !decode.has_retirement_owner()&&!encode.has_retirement_owner(){break;}decode.close_retirement_recipient(wallet()).unwrap();encode.close_retirement_recipient(wallet()).unwrap();}
        assert!(!decode.has_retirement_owner()&&!encode.has_retirement_owner());retire(source);retire(graph);
    }
}
#[test]
fn registered_fem_target_cancellation_keeps_the_original_terminal_kind() {
    let source = domains::receive_fem_uncontrolled(&input()).unwrap();
    let mut observe = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1024 * 1024, &mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let graph = fem_build::receive(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet())).unwrap();
    let mut receive = |_| true;
    let running = std::cell::Cell::new(false);
    let mut publish = |_| running.get();
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(256 * 1024 * 1024, &mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new(256 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, wallet());
    let result = ready(<domains::IfcIntoFem as Serializer<IfcSnapshot>>::serialize(&graph, &ArchiveChildren::empty(), &mut control));
    assert_eq!(result.unwrap_err().cause.kind, ValueRefusalKind::Canceled);
    drop(control);
    running.set(true);
    for _ in 0..1_000_000 {
        if !decode.has_retirement_owner() && !encode.has_retirement_owner() {
            break;
        }
        decode.close_retirement_recipient(wallet()).unwrap();
        encode.close_retirement_recipient(wallet()).unwrap();
    }
    assert!(!decode.has_retirement_owner());
    assert!(!encode.has_retirement_owner());
    retire(source);
}
#[test]
fn native_owned_dimension_and_membership_refusals_publish_no_fem_snapshot() {
    let source = domains::receive_fem_uncontrolled(&input()).unwrap();
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(256 * 1024 * 1024, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let graph = fem_build::receive(&source, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet())).unwrap();
    for mode in 0..2 {
        let mut graph = graph.clone();
        if mode == 0 {
            graph.entities.iter_mut().find(|v| v.name == "IFCDERIVEDUNITELEMENT" && v.args.get(1) == Some(&IfcValue::Integer(4))).unwrap().args[1] = IfcValue::Integer(1);
        } else {
            graph.entities.retain(|v| v.name != "IFCRELASSIGNSTOGROUP");
        }
        let result = fem_owned::receive(&graph, &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native, wallet()));
        assert!(result.is_err());
        assert!(!native.has_retirement_owner());
    }
    retire(source);
}
