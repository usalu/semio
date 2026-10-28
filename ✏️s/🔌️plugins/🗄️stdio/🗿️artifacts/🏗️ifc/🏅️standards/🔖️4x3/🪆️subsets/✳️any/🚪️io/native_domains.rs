//! 🏢 Actual native domain snapshot packs compose through explicit ADD2 partial projections.
use super::*;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot,Fem3dSnapshotValue};

pub const BIM_DIALECT:Dialect=Dialect {artifact_kind:"s.bim.model",standard:StandardId("1"),subset:SubsetId("*")};
pub const FEM_DIALECT:Dialect=Dialect {artifact_kind:"s.fem.fem3d",standard:StandardId("1"),subset:SubsetId("*")};

#[cfg(test)]
pub(super) fn receive_fem_uncontrolled(value:&Fem3dSnapshotValue)->Result<Fem3dSnapshot,ExchangeError> {
    let mut input=Some(value.clone());let mut callback=|_|true;
    let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut native=semio_framework_value::NativeDecodeControl::new(128*1024*1024,&mut callback);native.install_retirement_recipient(&mut recipient).map_err(|e|error("test.receiving",e.to_string()))?;
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:128*1024*1024,maximum_capacity_bytes:128*1024*1024,maximum_release_bytes:128*1024*1024,maximum_depth:128};
    let mut owner=semio_framework_os_kernel::NativeSnapshotDecodeOwner::new(&mut native,grant);
    Fem3dSnapshot::receive_authoring(&mut input,&mut owner).map_err(|e|error("test.receiving",e.to_string()))
}
#[cfg(test)]
pub(super) fn retire_fem_fixture(value:Fem3dSnapshot){
    let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:128*1024*1024,maximum_capacity_bytes:128*1024*1024,maximum_release_bytes:128*1024*1024,maximum_depth:128};
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    for _ in 0..1_000_000{if owner.terminal_is_empty(){return;}owner.step(grant).unwrap();}panic!("native FEM fixture failed funded retirement");
}
fn original_mapping<T>(control:&mut IoRunControl<'_,'_>,operation:impl FnOnce(&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<T>,ExchangeError>)->Result<Composition<T>,IoError> {
    let mut terminal=None;
    let result=operation(&mut |_|{if terminal.is_some(){return false;}match control.checkpoint(){Ok(())=>true,Err(error)=>{terminal=Some(error);false}}});
    if let Some(error)=terminal{return Err(IoError::from_value_error(error));}
    result.map_err(io_error)
}

fn source<'a>(sources:&'a [ComposeSource<'a>],dialect:Dialect)->Result<&'a [u8],ComposeError> {
    let [source]=sources else{return Err(composition_error("Exactly one native artifact source required"));};
    if source.dialect!=dialect {return Err(composition_error("Wrong source artifact dialect"));}
    let AnalyzeSource::Binary(bytes)=source.payload else{return Err(composition_error("Native artifact snapshot pack required"));};
    if bytes.len()>64*1024*1024 {return Err(composition_error("Native artifact source exceeds byte bound"));}
    Ok(bytes)
}

pub struct IfcToBim;
impl ArtifactComposer for IfcToBim {
    type Snapshot=ModelSnapshot;
    const WRITES:Dialect=BIM_DIALECT;
    fn reads()->&'static[Dialect]{&[crate::io::IFC4X3_DIALECT]}
    fn compose(sources:&[ComposeSource<'_>])->Result<Composition<Self::Snapshot>,ComposeError>{
        let input=IfcSnapshot::decode_pack(source(sources,crate::io::IFC4X3_DIALECT)?).map_err(composition_error)?;
        super::bim::ifc_to_bim_controlled(&input,&mut |_|true).map_err(composition_error)
    }
}
pub struct BimToIfc;
impl ArtifactComposer for BimToIfc {
    type Snapshot=IfcSnapshot;
    const WRITES:Dialect=crate::io::IFC4X3_DIALECT;
    fn reads()->&'static[Dialect]{&[BIM_DIALECT]}
    fn compose(sources:&[ComposeSource<'_>])->Result<Composition<Self::Snapshot>,ComposeError>{
        let input=ModelSnapshot::decode_pack(source(sources,BIM_DIALECT)?).map_err(composition_error)?;
        super::bim::bim_to_ifc_controlled(&input,&mut |_|true).map_err(composition_error)
    }
}

pub struct IfcToFem;
impl ArtifactComposer for IfcToFem {
    type Snapshot=Fem3dSnapshot;
    const WRITES:Dialect=FEM_DIALECT;
    fn reads()->&'static[Dialect]{&[crate::io::IFC4X3_DIALECT]}
    fn compose(_sources:&[ComposeSource<'_>])->Result<Composition<Self::Snapshot>,ComposeError>{
        Err(composition_error("Persistent FEM composition requires the controlled host route and original receiving wallet"))
    }
}
pub struct FemToIfc;
impl ArtifactComposer for FemToIfc {
    type Snapshot=IfcSnapshot;
    const WRITES:Dialect=crate::io::IFC4X3_DIALECT;
    fn reads()->&'static[Dialect]{&[FEM_DIALECT]}
    fn compose(_sources:&[ComposeSource<'_>])->Result<Composition<Self::Snapshot>,ComposeError>{
        Err(composition_error("Persistent FEM composition requires the controlled host route and original receiving wallet"))
    }
}

pub struct IfcIntoBim;
impl Serializer<IfcSnapshot> for IfcIntoBim {
    const INTO:Dialect=BIM_DIALECT;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn serialize(input:&IfcSnapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{
        let result=original_mapping(control,|observer|super::bim::ifc_to_bim_controlled(input,observer))?;
        let value=result.snapshot.encode_native_snapshot(NativeSnapshotEncoding::Binary,control.encode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?;
        Ok(IoOutcome {value,diagnostics:result.diagnostics})
    }
}
pub struct BimIntoIfc;
impl Deserializer<IfcSnapshot> for BimIntoIfc {
    const FROM:Dialect=BIM_DIALECT;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn deserialize(input:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<IfcSnapshot>{
        let IoPayload::Binary(bytes)=input else{return Err(io_error(error("bim.source","Native BIM snapshot pack required")));};
        let source=ModelSnapshot::decode_native_snapshot(NativeSnapshotInput::Binary(bytes),control.decode().map_err(IoError::from_value_error)?).map_err(IoError::from_value_error)?;
        let result=original_mapping(control,|observer|super::bim::bim_to_ifc_controlled(&source,observer))?;
        Ok(IoOutcome {value:result.snapshot,diagnostics:result.diagnostics})
    }
}

pub struct IfcIntoFem;
impl Serializer<IfcSnapshot> for IfcIntoFem {
    const INTO:Dialect=FEM_DIALECT;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn serialize(input:&IfcSnapshot,_:&ArchiveChildren,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{
        let value=control.receive_nested::<Fem3dSnapshot,IoPayload>(|slot,original|{
            *slot=Some(super::fem_owned::receive(input,&mut original.snapshot_decode()?)?);
            slot.as_ref().unwrap().encode_exchange_pack(&mut original.snapshot_encode()?)
        }).map_err(IoError::from_value_error)?;
        Ok(IoOutcome{value,diagnostics:vec![loss("Native analytical projection retains nodes, bars and zero roll frames, shared canonical materials and sections, boolean supports and supported direct global nodal load cases with explicit component presence. Distributed or area loads, nonunit factors, nonzero self-weight, combinations, solids, releases, local placements, non-SI units and nonzero roll refuse atomically. Canonical case and component identities derive from actual IFC IDs; other source identities, unreferenced definitions and solver/display settings are omitted.")]})
    }
}
pub struct FemIntoIfc;
impl Deserializer<IfcSnapshot> for FemIntoIfc {
    const FROM:Dialect=FEM_DIALECT;
    const FIDELITY:IoFidelity=IoFidelity::Lossy;
    async fn deserialize(input:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<IfcSnapshot>{
        let IoPayload::Binary(bytes)=input else{return Err(IoError::from_value_error(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"FEM native snapshot pack required")));};
        let value=control.receive_nested::<Fem3dSnapshot,IfcSnapshot>(|slot,original|{
            *slot=Some(<Fem3dSnapshot as semio_framework_os_kernel::ArtifactPackReceiving>::receive_pack(bytes,&mut original.snapshot_decode()?)?);
            super::fem_build::receive(slot.as_ref().unwrap(),&mut original.snapshot_decode()?)
        }).map_err(IoError::from_value_error)?;
        Ok(IoOutcome{value,diagnostics:vec![loss("Native analytical export retains nodes, bars and zero roll frames, canonical material and section engineering values, boolean supports and explicit global nodal load components including signed zero. Self-weight, distributed or area loads, combinations, solids, isolated nodes and nonzero frame roll refuse atomically. New IFC identities and reference axes are authored; solver/display settings are omitted.")]})
    }
}

pub fn entries()->&'static[ComposerEntry] {
    static ENTRIES:OnceLock<Vec<ComposerEntry>>=OnceLock::new();
    ENTRIES.get_or_init(||vec![semio_framework_plugin::composer_entry_of::<IfcToBim>(),semio_framework_plugin::composer_entry_of::<BimToIfc>(),semio_framework_plugin::composer_entry_of::<IfcToFem>(),semio_framework_plugin::composer_entry_of::<FemToIfc>()]).as_slice()
}
pub(super) fn io_entries()->Vec<IoEntry>{vec![original_pack::serializer::<IfcIntoBim>(),original_pack::deserializer::<BimIntoIfc>(),original_pack::serializer::<IfcIntoFem>(),original_pack::deserializer::<FemIntoIfc>()]}

#[cfg(test)]
mod controlled_tests {
 use super::*;
 #[test]
 fn original_terminal_refusal_survives_mapping_adapter(){
  let mut receive=|_|false;let mut decode=semio_framework_value::NativeDecodeControl::new(0,&mut receive);
  let mut control=IoRunControl::decoding(&mut decode,semio_framework_value::RetainedCloneGrant::default());
  let result=original_mapping::<IfcSnapshot>(&mut control,|observer|{checkpoint(observer,"test",0,1)?;unreachable!()});
  assert_eq!(result.err().unwrap().cause.kind,semio_framework_value::ValueRefusalKind::Canceled);
 }
}
