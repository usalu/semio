//! 🦀️ JFIF 1.01 🧱️baseline conformance-class mutation case — Rust adapter.
//!
//! The oracle role reads the real scan's baseline axes with the registered libjpeg-turbo CLIs
//! (`libjpeg-jpg-jfif-1-01-baseline-marker-cli`: `djpeg -v -v` for the SOFn code, the component sampling
//! factors, the DHT tables and a DAC segment, `rdjpgcom -verbose` for the sample precision), applies
//! each kind to those axes as ITU-T T.81 defines the field it names, and reads the class off the
//! specification's own tables (`../../🔮️oracles/🦀️.rs`); it never consults this repository's decoder,
//! snapshot or checker. The subject applies the same row to the decoded snapshot through
//! `JpgBaselineMutation`. Both answer in the same conformance projection, compared field for field.
//!
//! The comparison is on axes, not bytes: `encode_jpg` writes a conforming baseline file and nothing
//! else, so every axis is normalized away on re-serialization. The decode/re-encode law is its own
//! case, `../🔁️round-trip-jpg-jfif-1-01-baseline`.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::baseline::{apply, project, read_axes, Axes};

//#region 🔖️Kinds

/// 🖼️ The real 2275x2560 architectural scan, shared with the `🧾️document` case rather than copied: two
/// DQT, an SOF0 with three components, four DHT and SOS.
const SCAN: &str = "shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";
//#endregion 🔖️Kinds

//#region 🔖️Oracle
/// 📖️ The scan's axes as libjpeg-turbo reads them from a copy in the work directory.
fn scan_axes(ctx: &Context) -> Result<Axes, String> {
    read_axes(&ctx.copy_input(SCAN, Some("scan.jpg"))?)
}

/// 🎯️ The reference answer for one row: the axes after the kind, which must have moved.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    let next = apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?;
    let projection = project(&next);
    if projection == project(&base) {
        return Err(format!("mutate-{kind}: the reference reading of the scan did not move"));
    }
    Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
}

/// ↩️ The reference inverse: every axis the kind touched goes back to the value libjpeg-turbo read.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let row = ctx.doc_json()?;
    let kind = row.str("kind");
    let base = scan_axes(ctx)?;
    if project(&apply(&base, &kind, &row.get("params").cloned().unwrap_or(Json::Object(Vec::new())))?) == project(&base) {
        return Err(format!("inverse-{kind}: the forward kind left the reference reading untouched, so restoring it proves nothing"));
    }
    let restored = project(&base);
    Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
 use semio_repo_test_host::{Context,Json,Outcome,law};
 use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::{document::io::{inspect_jpg_native_header,binary::snapshot::observations::{JpgNativeObservations,JpgHuffmanClass,JpgHuffmanTable,JpgFrameComponent}},baseline::schema::conformance::check_baseline_facts};
 fn number(value:&Json,key:&str)->Result<u8,String>{match value.get(key){Some(Json::Number(found)) if found.fract()==0.0 && (0.0..=255.0).contains(found)=>Ok(*found as u8),_=>Err(format!("native profile fixture requires byte {key}"))}}
 fn profile(base:&JpgNativeObservations,row:&Json)->Result<JpgNativeObservations,String>{
  let mut next=base.clone();
  let params=row.get("params").ok_or("native profile fixture has no params")?;
  match row.str("kind").as_str(){
   "set-sof-marker"=>next.sof_marker=number(params,"marker")?,
   "set-sample-precision"=>next.frame.precision=number(params,"precision")?,
   "set-arithmetic"=>next.arithmetic=matches!(params.get("arithmetic"),Some(Json::Bool(true))),
   "set-component-sampling"=>{let id=number(params,"id")?;for component in next.frame.components.iter_mut().filter(|item|item.id==id){component.h_sampling=number(params,"hSampling")?;component.v_sampling=number(params,"vSampling")?;}},
   "remove-frame-component"=>{let id=number(params,"id")?;next.frame.components.retain(|item|item.id!=id);},
   "insert-frame-component"=>{let item=params.get("component").ok_or("native component fixture missing")?;let id=number(item,"id")?;if !next.frame.components.iter().any(|item|item.id==id){let at=usize::from(number(params,"index")?).min(next.frame.components.len());next.frame.components.insert(at,JpgFrameComponent{id,h_sampling:number(item,"hSampling")?,v_sampling:number(item,"vSampling")?,quant_table_id:number(item,"quantTableId")?});}},
   "insert-huffman-table"=>{let item=params.get("table").ok_or("native Huffman fixture missing")?;let id=number(item,"id")?;let class=match item.str("class").as_str(){"dc"=>JpgHuffmanClass::Dc,"ac"=>JpgHuffmanClass::Ac,_=>return Err("native Huffman class is invalid".into())};if !next.huffman_tables.iter().any(|item|item.id==id&&item.class==class){let at=usize::from(number(params,"index")?).min(next.huffman_tables.len());next.huffman_tables.insert(at,JpgHuffmanTable{id,class,bits:[0;16],values:Vec::new()});}},
   "remove-huffman-table"=>{let item=params.get("key").ok_or("native Huffman key missing")?;let id=number(item,"id")?;let class=match item.str("class").as_str(){"dc"=>JpgHuffmanClass::Dc,"ac"=>JpgHuffmanClass::Ac,_=>return Err("native Huffman class is invalid".into())};next.huffman_tables.retain(|item|item.id!=id||item.class!=class);},
   other=>return Err(format!("unknown native profile fixture {other}"))
  }
  Ok(next)
 }
 fn projection(observations:&JpgNativeObservations)->Json{
  let strings=|values:Vec<String>|Json::Array(values.into_iter().map(Json::String).collect());
  Json::Object(vec![
   ("format".into(),Json::String("jpg-baseline".into())),
   ("sofMarker".into(),Json::String(format!("{:02x}",observations.sof_marker))),
   ("precision".into(),Json::Number(f64::from(observations.frame.precision))),
   ("arithmetic".into(),Json::Bool(observations.arithmetic)),
   ("componentCount".into(),Json::Number(observations.frame.components.len() as f64)),
   ("huffmanTables".into(),strings(observations.huffman_tables.iter().map(|item|format!("{}:{}",if item.class==JpgHuffmanClass::Dc{"dc"}else{"ac"},item.id)).collect())),
   ("components".into(),strings(observations.frame.components.iter().map(|item|format!("{}:{}x{}",item.id,item.h_sampling,item.v_sampling)).collect())),
   ("conformance".into(),strings(check_baseline_facts(&observations.baseline_facts()).iter().map(|diagnostic|diagnostic.code.to_string()).collect()))
  ])
 }
 fn decoded(ctx:&Context)->Result<JpgNativeObservations,String>{
  inspect_jpg_native_header(&ctx.input_bytes(super::SCAN)?).map_err(|error|format!("{error:?}"))
 }
 pub fn mutate(ctx:&Context)->Result<Outcome,String>{
  let row=ctx.doc_json()?;
  let base=decoded(ctx)?;
  let next=profile(&base,&row)?;
  let now=projection(&next);
  law::mutation_is_observable(&row.str("kind"),&now,&projection(&base),&[])?;
  let expected=row.str("code");
  let codes=check_baseline_facts(&next.baseline_facts()).iter().map(|diagnostic|diagnostic.code.to_string()).collect::<Vec<_>>();
  if (expected.is_empty()&&!codes.is_empty())||(!expected.is_empty()&&!codes.contains(&expected)){return Err(format!("native profile verdict {codes:?} disagrees with {expected:?}"));}
  eprintln!("[DEBUG] JPEG owned native profile={} codes={codes:?}",row.str("kind"));
  Ok(Outcome::with_raw(now.to_string().into_bytes(),now))
 }
 pub fn inverse(ctx:&Context)->Result<Outcome,String>{
  let row=ctx.doc_json()?;
  let base=decoded(ctx)?;
  if projection(&profile(&base,&row)?)==projection(&base){return Err("native profile fixture did not move its observation".into());}
  let restored=projection(&base);
  Ok(Outcome::with_raw(restored.to_string().into_bytes(),restored))
 }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
