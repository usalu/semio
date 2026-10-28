//! 🔲️ A boundary facet binds the original discriminator and optional partner to the one native sealed union.
use super::*;
pub(super) fn compile(input:&DeriveInput,schema:&Value,r:&Value,c:&Path,s:&Path,d:&Path)->syn::Result<Tokens>{
 let name=&input.ident;let plan=format_ident!("__{}NativeFacetPath",name);let storage=format_ident!("__{}NativeBoundaryStorage",name);let key=ident(r,"key")?;let value=ident(r,"input")?;let partner=ident(r,"partner")?;
 let syn::Data::Struct(data)=&input.data else{return Err(error("native boundary requires its original record"));};let syn::Fields::Named(fields)=&data.fields else{return Err(error("native boundary requires named original fields"));};if fields.named.len()!=3||text(r,"row")?!="StateSurface"||text(r,"branch")?!="geometry"||text(r,"collection")?!="surfaces"||text(r,"field")?!="outside_boundary_condition"||r["references"]!=serde_json::json!(["Surfaces"])||text(r,"referenceTarget")?!="key"{return Err(error("native boundary differs from its actual surface union source"));}
 if schema["properties"][camel(&value.to_string())]["enum"]!=serde_json::json!(["OutdoorAir","Ground","OtherSideTemperature","Adiabatic","Interzone"])||schema["properties"][camel(&partner.to_string())]["type"]!=serde_json::json!(["integer","null"]){return Err(error("native boundary schema differs from its original discriminator and partner"));}
 let invalid=diagnostic(&r["diagnostics"],"invalid",c)?;let reference=diagnostic(&r["diagnostics"],"reference",c)?;let missing=diagnostic(&r["diagnostics"],"missing",c)?;let unchanged=diagnostic(&r["diagnostics"],"unchanged",c)?;let en=text(&r["labels"],"en")?;let de=text(&r["labels"],"de")?;let verb=text(&r["semantics"],"verb")?;let entity=text(&r["semantics"],"entity")?;let kind=text(&r["semantics"],"kind")?;let record=text(&r["semantics"],"record")?;
 Ok(quote!{
 #[doc(hidden)]pub struct #storage;
 impl ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage for #storage{
 type Value=(crate::model::OutsideBoundaryKind,Option<crate::model::EntityId>);type Stored=::semio_framework_value::shared_issuance::SealedShared<crate::model::OutsideBoundary>;
 fn value(v:&Self::Stored)->Self::Value{(v.get().kind(),v.get().interzone_partner())}
 fn birth_demand()->::semio_framework_value::RetirementDemand{< ::semio_s_artifact_energy_model_native_mutations::scalar::SharedScalar<crate::model::OutsideBoundary> as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::birth_demand()}
 fn admit(v:Self::Value,g: ::semio_framework_value::RetainedCloneGrant)->Result<(Self::Stored,::semio_framework_value::RetainedCloneProgress),::semio_framework_value::ValueError>{let v=crate::model::OutsideBoundary::from_parts(v.0,v.1).ok_or_else(||::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"an admitted original surface boundary lost its partner"))?;< ::semio_s_artifact_energy_model_native_mutations::scalar::SharedScalar<crate::model::OutsideBoundary> as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::admit(v,g)}
 }
 #[doc(hidden)]pub struct #plan;
 impl ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarRowPath for #plan{
 type Scalar=(crate::model::OutsideBoundaryKind,Option<crate::model::EntityId>);type Storage=#storage;type Row=#s::state::StateSurface;type Branch=#s::state::ModelGeometryState;type Output=::semio_s_artifact_energy_model_native_mutations::row::DirectBranchOutput<Self::Branch>;
 fn branch(o: ::semio_framework_value::RetainedCloneRef<'_,#s::original::EnergyModelSnapshot>)->::semio_framework_value::RetainedCloneRef<'_,Self::Branch>{o.project(0,|s|s.model.geometry.get())}
 fn rows(o: ::semio_framework_value::RetainedCloneRef<'_,#s::original::EnergyModelSnapshot>)->::semio_framework_value::RetainedCloneRef<'_,::semio_framework_value_persistent::list::persistent::PersistentList<Self::Row>>{o.project(1,|s|&s.model.geometry.get().surfaces)}
 fn rows_mut(b:&mut Self::Branch)->&mut ::semio_framework_value_persistent::list::persistent::PersistentList<Self::Row>{&mut b.surfaces}
 fn read(r:&Self::Row)->Self::Scalar{(r.outside_boundary_condition.get().kind(),r.outside_boundary_condition.get().interzone_partner())}
 fn replace(r:&mut Self::Row,v:<Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored)-><Self::Storage as ::semio_s_artifact_energy_model_native_mutations::scalar::ScalarStorage>::Stored{::std::mem::replace(&mut r.outside_boundary_condition,v)}
 fn valid(v:Self::Scalar)->bool{crate::model::OutsideBoundary::from_parts(v.0,v.1).is_some()}
 fn valid_reference(v:Self::Scalar,key:u32)->bool{v.1.is_none_or(|v|v.0!=key)}
 fn has_reference(v:Self::Scalar)->bool{v.1.is_some()}
 fn reference_collections()->&'static[::semio_s_artifact_energy_model_native_mutations::reference::ReferenceCollection]{&[::semio_s_artifact_energy_model_native_mutations::reference::ReferenceCollection::Surfaces]}
 fn reference_key(v: ::semio_framework_value::RetainedCloneRef<'_,Self::Scalar>)->Result<::semio_framework_value::RetainedCloneRef<'_,u32>,::semio_framework_value::ValueError>{if v.get().1.is_none(){return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"an original interzone reference has no partner"));}Ok(v.project(0,|v|&v.1.as_ref().expect("admitted original interzone partner").0))}
 fn output()->Self::Output{::semio_s_artifact_energy_model_native_mutations::row::DirectBranchOutput::new(|b|{let mut d=#d::original::EnergyModelDiff::default();d.model.geometry=Some(b);d})}
 }
 impl ::semio_s_artifact_energy_model_native_mutations::scalar::OriginalScalarTransform for #plan{type Payload=#name;fn value(p:&#name)->Self::Scalar{(p.#value,p.#partner)}fn valid_payload(_:&#name)->bool{true}}
 impl<M:From<#name>> ::semio_s_artifact_energy_model_native_mutations::leaf::RowLeafSpec<#name,M> for #plan{
 type Input=#name;type Inverse=(crate::model::OutsideBoundaryKind,Option<crate::model::EntityId>);type Core=::semio_s_artifact_energy_model_native_mutations::scalar::TransformedScalarPreparation<Self>;
 const INVALID:#c::mutation::retained::RetainedDiagnosticSpec=#invalid;const INVALID_REFERENCE:#c::mutation::retained::RetainedDiagnosticSpec=#reference;const MISSING:#c::mutation::retained::RetainedDiagnosticSpec=#missing;const UNCHANGED:#c::mutation::retained::RetainedDiagnosticSpec=#unchanged;
 fn input(p: ::semio_framework_value::RetainedCloneRef<'_,#name>)->::semio_framework_value::RetainedCloneRef<'_,#name>{p}
 fn key(p: ::semio_framework_value::RetainedCloneRef<'_,#name>)->Option<::semio_framework_value::RetainedCloneRef<'_,u32>>{Some(p.project(0,|p|&p.#key.0))}
 fn diagnostic_targets(p: ::semio_framework_value::RetainedCloneRef<'_,#name>,_: ::semio_s_artifact_energy_model_native_mutations::row::RowOutcome)->Result<[#c::mutation::retained::RetainedDiagnosticTarget<'_>;1],::semio_framework_value::ValueError>{Ok([#c::mutation::retained::RetainedDiagnosticTarget::Unsigned(p.project(0,|p|&p.#key.0))])}
 fn inverse(p: ::semio_framework_value::RetainedCloneRef<'_,#name>,v:Self::Inverse)->M{#name{#key:p.get().#key,#value:v.0,#partner:v.1}.into()}
 }
 impl<M> #c::mutation::retained::RetainedMutationLeaf<#s::original::EnergyModelSnapshot,M> for #name where M:#c::Mutation<#s::original::EnergyModelSnapshot,Diff=#d::original::EnergyModelDiff>+From<#name>+Send+'static{type Preparation=::semio_s_artifact_energy_model_native_mutations::leaf::RowLeafPreparation<Self,M,#plan>;}
 impl<M> #c::MutationKind<#s::original::EnergyModelSnapshot,M> for #name where M:#c::Mutation<#s::original::EnergyModelSnapshot,Diff=#d::original::EnergyModelDiff>+From<#name>{const SEMANTICS:#c::SemanticDescriptor=#c::SemanticDescriptor{verb:#verb,entity:#entity,kind:#kind,record:#record};fn label(&self)->#c::LocalizedLabel{#c::LocalizedLabel::native(#en,#de)}fn target(&self)->Vec<String>{vec![self.#key.0.to_string()]}}
 impl ::semio_s_artifact_energy_model_native_mutations::target::ArtifactOperationTargets for #name{fn len(&self)->usize{1}fn segment_len(&self,i:usize)->Result<usize,::semio_framework_value::ValueError>{::semio_s_artifact_energy_model_native_mutations::target::unsigned_len(self.#key.0,i)}fn read_segment(&self,i:usize,o:usize,b:&mut[u8])->Result<usize,::semio_framework_value::ValueError>{::semio_s_artifact_energy_model_native_mutations::target::read_unsigned(self.#key.0,i,o,b)}}
 })
}
