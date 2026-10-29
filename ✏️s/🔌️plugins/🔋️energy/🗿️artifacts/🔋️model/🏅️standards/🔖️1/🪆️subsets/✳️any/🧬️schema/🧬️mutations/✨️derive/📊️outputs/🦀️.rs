//! 📊️ Authored reporting recipes compile the actual composite collection and complete inverse event.
use super::*;
pub(super) fn compile(input:&DeriveInput,schema:&Value,r:&Value,c:&Path,snapshot:&Path,diff:&Path)->syn::Result<Tokens>{
 let insert=match text(r,"intent")?{"insert"=>true,"remove"=>false,_=>return Err(error("output registration intent must insert or remove"))};
 for(key,value)in[("row","StateOutputVariableSpec"),("branchType","ModelOutputState"),("branch","outputs"),("collection","output_variables")]{if text(r,key)?!=value{return Err(error("reporting recipe differs from its actual original root collection"))}}
 if r["identity"]!=serde_json::json!(["name","key"]){return Err(error("reporting identity is exactly its authored name and key"))}
 let syn::Data::Struct(data)=&input.data else{return Err(error("reporting input must be an authored record"))};let syn::Fields::Named(fields)=&data.fields else{return Err(error("reporting input must name its original fields"))};
 let expected=if insert{vec!["name","key","reporting_frequency","index"]}else{vec!["name","key"]};if fields.named.iter().map(|f|f.ident.as_ref().unwrap().to_string()).collect::<Vec<_>>()!=expected{return Err(error("reporting payload differs from its complete authored event"))}
 if schema["properties"]["name"]["type"]!="string"||schema["properties"]["key"]["type"]!="string"{return Err(error("reporting composite identity requires genuine string sources"))}
 let name=&input.ident;let plan=quote::format_ident!("__{}NativeOutputPath",name);let frequency=if insert{quote!(Some(p.reporting_frequency))}else{quote!({let _=p;None})};let index=if insert{quote!(p.index.map(|i|i as usize))}else{quote!({let _=p;None})};
 let inverse=if insert{quote!(crate::RemoveOutputVariable{name:v.name,key:v.key}.into())}else{quote!(crate::AddOutputVariable{name:v.name,key:v.key,reporting_frequency:v.reporting_frequency,index:Some(v.index)}.into())};
 let en=text(&r["labels"],"en")?;let de=text(&r["labels"],"de")?;let verb=text(&r["semantics"],"verb")?;let entity=text(&r["semantics"],"entity")?;let kind=text(&r["semantics"],"kind")?;let record=text(&r["semantics"],"record")?;
 let invalid=diagnostic(&r["diagnostics"],"invalid",c)?;let duplicate=diagnostic(&r["diagnostics"],"duplicate",c)?;let missing=diagnostic(&r["diagnostics"],"missing",c)?;let unchanged=diagnostic(&r["diagnostics"],"unchanged",c)?;
 Ok(quote!{
 #[doc(hidden)]pub struct #plan;
 impl ::semio_s_artifact_energy_model_native_mutations::row::RowBranchPath for #plan{
 type Row=#snapshot::state::StateOutputVariableSpec;type Branch=#snapshot::state::ModelOutputState;type Output=::semio_s_artifact_energy_model_native_mutations::row::DirectBranchOutput<Self::Branch>;
 fn rows(o: ::semio_framework_value::RetainedCloneRef<'_,#snapshot::original::EnergyModelSnapshot>)->::semio_framework_value::RetainedCloneRef<'_,::semio_framework_value_persistent::list::persistent::PersistentList<Self::Row>>{o.project(1,|s|&s.model.outputs.get().output_variables)}
 fn branch(o: ::semio_framework_value::RetainedCloneRef<'_,#snapshot::original::EnergyModelSnapshot>)->::semio_framework_value::RetainedCloneRef<'_,Self::Branch>{o.project(0,|s|s.model.outputs.get())}
 fn rows_mut(b:&mut Self::Branch)->&mut ::semio_framework_value_persistent::list::persistent::PersistentList<Self::Row>{&mut b.output_variables}
 fn output()->Self::Output{Self::Output::new(|value|{let mut d=#diff::original::EnergyModelDiff::default();d.model.outputs=Some(value);d})}
 }
 impl ::semio_s_artifact_energy_model_native_mutations_singletons::outputs::OutputVariablePath for #plan{type Payload=#name;const INSERT:bool=#insert;fn text(p: ::semio_framework_value::RetainedCloneRef<'_,#name>,field:usize)->::semio_framework_value::RetainedCloneRef<'_,String>{p.project(field,move|p|if field==0{&p.name}else{&p.key})}fn frequency(p:&#name)->Option<crate::model::OutputReportFrequency>{#frequency}fn index(p:&#name)->Option<usize>{#index}}
 impl<M:From<crate::AddOutputVariable>+From<crate::RemoveOutputVariable>> ::semio_s_artifact_energy_model_native_mutations::leaf::RowLeafSpec<#name,M,2> for #plan{
 type Input=#name;type Inverse=::semio_s_artifact_energy_model_native_mutations_singletons::outputs::OutputVariableInverse;type Core=::semio_s_artifact_energy_model_native_mutations_singletons::outputs::OutputVariablePreparation<Self>;
 const INVALID:#c::RetainedDiagnosticSpec=#invalid;const DUPLICATE:#c::RetainedDiagnosticSpec=#duplicate;const MISSING:#c::RetainedDiagnosticSpec=#missing;const UNCHANGED:#c::RetainedDiagnosticSpec=#unchanged;
 fn input(p: ::semio_framework_value::RetainedCloneRef<'_,#name>)->::semio_framework_value::RetainedCloneRef<'_,#name>{p}fn key(_: ::semio_framework_value::RetainedCloneRef<'_,#name>)->Option<::semio_framework_value::RetainedCloneRef<'_,u32>>{None}
 fn inverse(_: ::semio_framework_value::RetainedCloneRef<'_,#name>,v:Self::Inverse)->M{#inverse}
 fn diagnostic_targets(p: ::semio_framework_value::RetainedCloneRef<'_,#name>,_: ::semio_s_artifact_energy_model_native_mutations::row::RowOutcome)->Result<[#c::RetainedDiagnosticTarget<'_>;2],::semio_framework_value::ValueError>{Ok([#c::RetainedDiagnosticTarget::Original(p.project(0,|p|&p.name)),#c::RetainedDiagnosticTarget::Original(p.project(1,|p|&p.key))])}
 }
 impl<M> #c::RetainedMutationLeaf<#snapshot::original::EnergyModelSnapshot,M> for #name where M:#c::Mutation<#snapshot::original::EnergyModelSnapshot,Diff=#diff::original::EnergyModelDiff>+From<crate::AddOutputVariable>+From<crate::RemoveOutputVariable>+Send+'static{type Preparation=::semio_s_artifact_energy_model_native_mutations::leaf::RowLeafPreparation<Self,M,#plan,2>;}
 impl<M> #c::MutationKind<#snapshot::original::EnergyModelSnapshot,M> for #name where M:#c::Mutation<#snapshot::original::EnergyModelSnapshot,Diff=#diff::original::EnergyModelDiff>+From<crate::AddOutputVariable>+From<crate::RemoveOutputVariable>{const SEMANTICS:#c::SemanticDescriptor=#c::SemanticDescriptor{verb:#verb,entity:#entity,kind:#kind,record:#record};fn label(&self)->#c::LocalizedLabel{#c::LocalizedLabel::native(#en,#de)}}
 impl ::semio_s_artifact_energy_model_native_mutations::target::ArtifactOperationTargets for #name{fn len(&self)->usize{2}fn segment_len(&self,index:usize)->Result<usize,::semio_framework_value::ValueError>{match index{0=>Ok(self.name.len()),1=>Ok(self.key.len()),_=>Err(::semio_s_artifact_energy_model_native_mutations::scalar::invalid())}}fn read_segment(&self,index:usize,offset:usize,output:&mut[u8])->Result<usize,::semio_framework_value::ValueError>{let text=match index{0=>&self.name,1=>&self.key,_=>return Err(::semio_s_artifact_energy_model_native_mutations::scalar::invalid())};let source=text.as_bytes().get(offset..).ok_or_else(::semio_s_artifact_energy_model_native_mutations::scalar::invalid)?;let count=source.len().min(output.len());output[..count].copy_from_slice(&source[..count]);Ok(count)}}
 })
}
