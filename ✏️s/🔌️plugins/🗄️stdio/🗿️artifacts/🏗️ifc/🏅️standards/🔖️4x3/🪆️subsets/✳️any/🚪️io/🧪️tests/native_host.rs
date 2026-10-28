use semio_s_artifact_stdio_ifc_4x3::{io::IFC4X3_DIALECT,native_exchange::GLTF_DIALECT};

#[test]
fn normal_ifc_assembly_declares_and_registers_native_geometry_exchange() {
    let declaration=crate::declaration(crate::definition().unwrap()).unwrap();
    assert!(declaration.composers.iter().any(|entry|entry.writes==GLTF_DIALECT&&entry.reads==[IFC4X3_DIALECT]));
    assert!(declaration.composers.iter().any(|entry|entry.writes==IFC4X3_DIALECT&&entry.reads==[GLTF_DIALECT]));
    crate::assembly().unwrap();
    let entries=semio_framework_os_kernel::io::io_mechanism::io_entries();
    for (from,into) in [(IFC4X3_DIALECT,GLTF_DIALECT),(GLTF_DIALECT,IFC4X3_DIALECT)] {
        assert!(entries.iter().any(|entry|entry.from==from.into()&&entry.into==into.into()&&entry.fidelity==semio_framework_os_kernel::io::io_schema::IoFidelity::Lossy));
    }
    assert!(entries.iter().any(|entry|entry.from==IFC4X3_DIALECT.into()&&entry.into==semio_framework_os_kernel::io::io_schema::CARRIER_BINARY.into()));
    for target in [semio_s_artifact_stdio_ifc_4x3::native_exchange::domains::BIM_DIALECT,semio_s_artifact_stdio_ifc_4x3::native_exchange::domains::FEM_DIALECT] {
        assert!(declaration.composers.iter().any(|entry|entry.writes==target&&entry.reads==[IFC4X3_DIALECT]));
        assert!(declaration.composers.iter().any(|entry|entry.writes==IFC4X3_DIALECT&&entry.reads==[target]));
        for (from,into) in [(IFC4X3_DIALECT,target),(target,IFC4X3_DIALECT)] {assert!(entries.iter().any(|entry|entry.from==from.into()&&entry.into==into.into()));}
    }
}
