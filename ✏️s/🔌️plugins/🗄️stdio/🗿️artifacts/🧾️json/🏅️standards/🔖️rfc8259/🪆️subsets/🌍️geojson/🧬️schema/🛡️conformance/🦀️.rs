//! 🌍️ Cancellable borrowed RFC7946 conformance without constructing a wire GeoJSON model.
use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
enum Failure{Domain(GeoJsonError),Control(ValueError)}
impl From<GeoJsonError> for Failure{fn from(value:GeoJsonError)->Self{Self::Domain(value)}}
type Result<T>=std::result::Result<T,Failure>;
fn refuse<T>(path:&str,message:impl Into<String>)->Result<T>{Err(Failure::Domain(GeoJsonError{path:path.into(),message:message.into()}))}
struct Gate<'a,C:GeoJsonConformanceControl>{control:&'a mut C,steps:usize,rings:usize,crs:GeoJsonSourceCrs}
impl<C:GeoJsonConformanceControl> Gate<'_,C>{
 fn tick(&mut self)->Result<()>{self.steps+=1;if self.steps%256==0{self.control.checkpoint(self.steps,0).map_err(Failure::Control)?;}Ok(())}
 fn member<'v>(&mut self,value:&'v JsonValue,key:&str)->Result<Option<&'v JsonValue>>{if let JsonValue::Object{members}=value{for member in members.iter().rev(){self.tick()?;if member.key==key{return Ok(Some(&member.value))}}}Ok(None)}
 fn source_crs(&mut self,value:&JsonValue)->Result<GeoJsonSourceCrs>{let Some(crs)=self.member(value,"crs")?else{return Ok(GeoJsonSourceCrs::Rfc7946)};if text(self.member(crs,"type")?)!=Some("name"){return refuse("/crs","only a GJ2008 named crs can be honoured; linked or untyped CRS objects are refused (RFC 7946 §4 is WGS 84)")}let name=if let Some(properties)=self.member(crs,"properties")?{text(self.member(properties,"name")?)}else{None};let Some(name)=name else{return refuse("/crs/properties/name","a named crs carries its name as a string")};if name.len()>65536{self.control.checkpoint(self.steps,0).map_err(Failure::Control)?;}let code=name.trim().to_ascii_uppercase().replace("URN:OGC:DEF:CRS:","").replace("::",":").replace(":1.3:",":");match code.as_str(){"OGC:CRS84"|"CRS84"|"EPSG:4326"=>Ok(GeoJsonSourceCrs::DeclaredCrs84),"EPSG:3857"|"EPSG:900913"|"EPSG:102100"|"EPSG:102113"=>Ok(GeoJsonSourceCrs::WebMercator),_=>refuse("/crs/properties/name",format!("coordinate reference system `{name}` is not WGS 84 and has no exact reprojection here; reproject to WGS 84 (RFC 7946 §4) first"))}}
 fn position(&mut self,value:&JsonValue,path:&str)->Result<[f64;2]>{let Some(coordinates)=items(Some(value))else{return refuse(path,"a position is an array of numbers")};if coordinates.len()<2{return refuse(path,"a position has at least longitude and latitude")}
  for(index,coordinate)in coordinates.iter().enumerate(){self.tick()?;if number(coordinate).is_none(){return refuse(&format!("{path}/{index}"),"a coordinate is a finite number")}}
  let mut xy=[number(&coordinates[0]).unwrap(),number(&coordinates[1]).unwrap()];if self.crs==GeoJsonSourceCrs::WebMercator{xy[0]=(xy[0]/WEB_MERCATOR_RADIUS).to_degrees();xy[1]=(2.0*(xy[1]/WEB_MERCATOR_RADIUS).exp().atan()-std::f64::consts::FRAC_PI_2).to_degrees();}wgs84_range(&xy,path)?;Ok(xy)
 }
 fn positions(&mut self,value:&JsonValue,path:&str,minimum:usize,what:&str)->Result<()>{let Some(list)=items(Some(value))else{return refuse(path,format!("{what} coordinates are an array of positions"))};if list.len()<minimum{return refuse(path,format!("{what} needs at least {minimum} positions"))}for(index,value)in list.iter().enumerate(){self.position(value,&format!("{path}/{index}"))?;}Ok(())}
 fn polygon(&mut self,value:&JsonValue,path:&str)->Result<()>{let Some(rings)=items(Some(value))else{return refuse(path,"Polygon coordinates are an array of linear rings")};if rings.is_empty(){return refuse(path,"a Polygon has an exterior ring")}
  for(index,value)in rings.iter().enumerate(){let path=format!("{path}/{index}");let Some(ring)=items(Some(value))else{return refuse(&path,"a linear ring coordinates are an array of positions")};if ring.len()<4{return refuse(&path,"a linear ring needs at least 4 positions")}
   let(mut first,mut previous,mut area)=(None,None,0.0);for(index,value)in ring.iter().enumerate(){let xy=self.position(value,&format!("{path}/{index}"))?;if first.is_none(){first=Some(xy)}if let Some(previous)=previous{let previous:[f64;2]=previous;area+=previous[0]*xy[1]-xy[0]*previous[1];}previous=Some(xy);}
   let begin=items(ring.first()).unwrap();let end=items(ring.last()).unwrap();let mut closed=begin.len()==end.len()&&first==previous;for(left,right)in begin.iter().skip(2).zip(end.iter().skip(2)){self.tick()?;closed&=number(left)==number(right);}if !closed{return refuse(&path,"a linear ring is closed: its first and last positions are identical (RFC 7946 §3.1.6)")}if(area>0.0)!=(index==0){self.rings+=1;}
  }Ok(())
 }
 fn geometry(&mut self,value:&JsonValue,path:&str,depth:usize)->Result<()>{self.tick()?;if depth>MAXIMUM_GEOMETRY_DEPTH{return refuse(path,format!("GeometryCollections nest deeper than {MAXIMUM_GEOMETRY_DEPTH}"))}let Some(kind)=text(self.member(value,"type")?)else{return refuse(path,"a geometry has a string `type`")};
  if kind=="GeometryCollection"{let Some(children)=items(self.member(value,"geometries")?)else{return refuse(&format!("{path}/geometries"),"a GeometryCollection has a `geometries` array")};for(index,value)in children.iter().enumerate(){self.geometry(value,&format!("{path}/geometries/{index}"),depth+1)?;}return Ok(())}
  let at=format!("{path}/coordinates");let Some(value)=self.member(value,"coordinates")?else{return refuse(&at,format!("a {kind} has `coordinates`"))};match kind{
   "Point"=>{self.position(value,&at)?;},"MultiPoint"=>self.positions(value,&at,0,"a MultiPoint")?,"LineString"=>self.positions(value,&at,2,"a LineString")?,"Polygon"=>self.polygon(value,&at)?,"MultiLineString"|"MultiPolygon"=>{let Some(parts)=items(Some(value))else{return refuse(&at,format!("a {kind} coordinates are an array"))};for(index,value)in parts.iter().enumerate(){if kind=="MultiPolygon"{self.polygon(value,&format!("{at}/{index}"))?;}else{self.positions(value,&format!("{at}/{index}"),2,"a LineString")?;}}},other=>return refuse(&format!("{path}/type"),format!("`{other}` is not an RFC 7946 geometry type"))}Ok(())
 }
 fn feature(&mut self,value:&JsonValue,path:&str)->Result<()>{if text(self.member(value,"type")?)!=Some("Feature"){return refuse(&format!("{path}/type"),"a feature has `\"type\": \"Feature\"`")}
  match self.member(value,"id")?{None|Some(JsonValue::String{..})=>{},Some(value@JsonValue::Number{..})=>if number(value).is_none(){return refuse(&format!("{path}/id"),"a numeric feature id is a finite number")},Some(_)=>return refuse(&format!("{path}/id"),"a feature id is a string or a number (RFC 7946 §3.2)")}
  match self.member(value,"geometry")?{None=>return refuse(&format!("{path}/geometry"),"a feature has a `geometry` member (an object or null)"),Some(JsonValue::Null)=>{},Some(value)=>self.geometry(value,&format!("{path}/geometry"),0)?}
  match self.member(value,"properties")?{None=>return refuse(&format!("{path}/properties"),"a feature has a `properties` member (an object or null)"),Some(JsonValue::Null|JsonValue::Object{..})=>{},Some(_)=>return refuse(&format!("{path}/properties"),"feature properties are an object or null")}Ok(())
 }
 fn root(&mut self,value:&JsonValue)->Result<()>{match text(self.member(value,"type")?){Some("FeatureCollection")=>{let Some(features)=items(self.member(value,"features")?)else{return refuse("/features","a FeatureCollection has a `features` array")};for(index,value)in features.iter().enumerate(){self.feature(value,&format!("/features/{index}"))?;}},Some("Feature")=>self.feature(value,"")?,Some(_)=>self.geometry(value,"",0)?,None=>return refuse("/type","a GeoJSON object has a string `type`")}Ok(())}
}
/// 🛡️ Walk the actual persisted JSON syntax and validate borrowed geometry with controlled work.
pub fn check_geojson_conformance_with_control<C:GeoJsonConformanceControl>(snapshot:&JsonSnapshot,control:&mut C)->std::result::Result<Vec<semio_framework_diagnostic::Diagnostic>,ValueError>{
 control.checkpoint(0,0)?;let mut gate=Gate{control,steps:0,rings:0,crs:GeoJsonSourceCrs::Rfc7946};let mut stack=vec![(&snapshot.value,0usize)];
 while let Some(&(value,index))=stack.last(){if index==0{if gate.steps>=gate.control.maximum_rows(){return Err(ValueError::new(ValueRefusalKind::WorkLimit,"GeoJSON conformance row limit"))}gate.tick().map_err(|failure|match failure{Failure::Control(error)=>error,Failure::Domain(error)=>ValueError::new(ValueRefusalKind::InvariantViolated,error.to_string())})?;if let JsonValue::Number{lexeme}=value{if !gate.control.number_valid(lexeme,gate.steps)?{return Ok(vec![diagnostic(CODE_NOT_GEOJSON,semio_framework_diagnostic::Severity::Error,"GeoJSON /: a number requires an RFC8259 lexeme".into())])}}}let child=match value{JsonValue::Array{items}=>items.get(index),JsonValue::Object{members}=>members.get(index).map(|member|&member.value),_=>None};if let Some(value)=child{stack.last_mut().unwrap().1+=1;stack.push((value,0));}else{stack.pop();}}
 if let Some(name)=member(&snapshot.value,"crs").and_then(|crs|member(crs,"properties")).and_then(|properties|text(member(properties,"name"))){if name.len()>65536{gate.control.checkpoint(gate.steps,0)?;}}
 let result=gate.source_crs(&snapshot.value).and_then(|crs|{gate.crs=crs;gate.root(&snapshot.value)});match result{Err(Failure::Control(error))=>Err(error),Err(Failure::Domain(error))=>Ok(vec![diagnostic(CODE_NOT_GEOJSON,semio_framework_diagnostic::Severity::Error,error.to_string())]),Ok(())=>{gate.control.checkpoint(gate.steps,gate.steps)?;let mut diagnostics=Vec::new();if gate.crs!=GeoJsonSourceCrs::Rfc7946{diagnostics.push(diagnostic(CODE_LEGACY_CRS,semio_framework_diagnostic::Severity::Warning,"the GJ2008 `crs` member was removed by RFC 7946 §4; coordinates are WGS 84 by definition".into()));}if gate.rings>0{diagnostics.push(diagnostic(CODE_LEFT_HANDED_RING,semio_framework_diagnostic::Severity::Warning,format!("{} linear ring(s) do not follow the right-hand rule (RFC 7946 §3.1.6)",gate.rings)));}Ok(diagnostics)}}
}

/// 🛡️ Neutral work and lexical-validation authority for semantic GeoJSON conformance.
pub trait GeoJsonConformanceControl {
fn checkpoint(&mut self, visited:usize, total:usize)->std::result::Result<(),ValueError>;
fn maximum_rows(&self)->usize;
fn number_valid(&mut self, lexeme:&str, visited:usize)->std::result::Result<bool,ValueError>;
}

pub(super) struct UnboundedGeoJsonConformance;
impl GeoJsonConformanceControl for UnboundedGeoJsonConformance {
fn checkpoint(&mut self,_:usize,_:usize)->std::result::Result<(),ValueError>{Ok(())}
fn maximum_rows(&self)->usize{usize::MAX}
fn number_valid(&mut self,lexeme:&str,_:usize)->std::result::Result<bool,ValueError>{Ok(valid_number_lexeme(lexeme))}
}

fn valid_number_lexeme(lexeme:&str)->bool {
let bytes=lexeme.as_bytes(); let mut cursor=usize::from(bytes.first()==Some(&b'-'));
match bytes.get(cursor){Some(b'0')=>cursor+=1,Some(b'1'..=b'9')=>{cursor+=1;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}},_=>return false}
if bytes.get(cursor)==Some(&b'.'){cursor+=1;let start=cursor;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}if cursor==start{return false}}
if matches!(bytes.get(cursor),Some(b'e'|b'E')){cursor+=1;if matches!(bytes.get(cursor),Some(b'+'|b'-')){cursor+=1;}let start=cursor;while bytes.get(cursor).is_some_and(u8::is_ascii_digit){cursor+=1;}if cursor==start{return false}}
cursor==bytes.len()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn neutral_number_lexemes_match_authored_vectors_and_serde_json() {
let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢️number-lexemes/🔣️.json")).expect("authored number vectors");
for case in corpus["cases"].as_array().expect("number cases") {
let lexeme=case["lexeme"].as_str().expect("number lexeme");
let expected=case["valid"].as_bool().expect("lexeme validity");
assert_eq!(valid_number_lexeme(lexeme),expected,"{lexeme}");
assert_eq!(serde_json::from_str::<serde_json::Value>(lexeme).is_ok(),expected,"serde_json {lexeme}");
}
}

struct RefusingControl;
impl GeoJsonConformanceControl for RefusingControl {
fn checkpoint(&mut self,_:usize,_:usize)->std::result::Result<(),ValueError>{Err(ValueError::new(ValueRefusalKind::WorkLimit,"fixture cancellation"))}
fn maximum_rows(&self)->usize{0}
fn number_valid(&mut self,_:&str,_:usize)->std::result::Result<bool,ValueError>{panic!("cancelled conformance cannot decode a number")}
}

#[test]
fn neutral_conformance_preserves_caller_refusal() {
let snapshot=JsonSnapshot{schema:"stdio.json".into(),value:JsonValue::Null};
let error=check_geojson_conformance_with_control(&snapshot,&mut RefusingControl).expect_err("caller cancellation");
assert!(error.to_string().contains("fixture cancellation"));
}
}
