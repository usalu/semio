//! 📨️ Fixed inline test instrumentation records delivery without allocating inside a host callback.
use semio_plugin_decl_emission::HostEvent;
const TEXT:usize=128;
const EVENTS:usize=64;
#[derive(Clone,Copy)]
struct Text{bytes:[u8;TEXT],len:u8}
impl Text{
 fn capture(value:&str)->Option<Self>{if value.len()>TEXT{return None}let mut output=Self{bytes:[0;TEXT],len:value.len()as u8};output.bytes[..value.len()].copy_from_slice(value.as_bytes());Some(output)}
 fn value(self)->String{std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("captured UTF8").to_owned()}
}
#[derive(Clone,Copy)]
struct Observation{kind:u8,window:Text,from:Option<Text>,to:Option<Text>}
/// 🧪️ Exhaustion is remembered without allocating; inspecting an incomplete trace fails explicitly.
pub struct HostEventTrace{events:[Option<Observation>;EVENTS],len:usize,overflow:bool}
impl HostEventTrace{
 pub const fn new()->Self{Self{events:[None;EVENTS],len:0,overflow:false}}
 pub fn push(&mut self,event:&HostEvent){
  let (kind,from,to)=match event{HostEvent::WindowBlurred{..}=>(0,None,None),HostEvent::PointerCaptureLost{..}=>(1,None,None),HostEvent::UtilityChanged{from,to,..}=>(2,from.as_deref(),to.as_deref()),HostEvent::TimeTravelFrozen{..}=>(3,None,None),HostEvent::BaseMoved{..}=>(4,None,None),HostEvent::Retiring{..}=>(5,None,None)};
  if self.len==EVENTS{self.overflow=true;return}let Some(window)=Text::capture(event.window_id())else{self.overflow=true;return};let from=match from{Some(value)=>match Text::capture(value){Some(value)=>Some(value),None=>{self.overflow=true;return}},None=>None};let to=match to{Some(value)=>match Text::capture(value){Some(value)=>Some(value),None=>{self.overflow=true;return}},None=>None};self.events[self.len]=Some(Observation{kind,window,from,to});self.len+=1;
 }
 pub fn clear(&mut self){*self=Self::new()}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn take(&mut self)->Vec<HostEvent>{
  assert!(!self.overflow,"host-event test trace exceeded its explicit inline bounds");let mut output=Vec::with_capacity(self.len);for entry in &self.events[..self.len]{let value=entry.expect("initialized event slot");let window_id=value.window.value();output.push(match value.kind{0=>HostEvent::WindowBlurred{window_id},1=>HostEvent::PointerCaptureLost{window_id},2=>HostEvent::UtilityChanged{window_id,from:value.from.map(Text::value),to:value.to.map(Text::value)},3=>HostEvent::TimeTravelFrozen{window_id},4=>HostEvent::BaseMoved{window_id},5=>HostEvent::Retiring{window_id},_=>unreachable!()});}self.clear();output
 }
}
impl Default for HostEventTrace{fn default()->Self{Self::new()}}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn fixed_observer_roundtrips_neutral_delivery_and_refuses_incomplete_traces(){
  let law:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();assert_eq!(TEXT,law["maximumTextBytes"].as_u64().unwrap()as usize);assert_eq!(EVENTS,law["maximumEvents"].as_u64().unwrap()as usize);let text=Text::capture(law["windowId"].as_str().unwrap()).unwrap();let bytes=law["windowUtf8"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(&text.bytes[..usize::from(text.len)],bytes.as_slice());let mut trace=HostEventTrace::new();let mut expected=Vec::new();for kind in law["cases"].as_array().unwrap(){let window_id=law["windowId"].as_str().unwrap().to_owned();let event=match kind.as_str().unwrap(){"blur"=>HostEvent::WindowBlurred{window_id},"captureLost"=>HostEvent::PointerCaptureLost{window_id},"utilityChanged"=>HostEvent::UtilityChanged{window_id,from:Some(law["utilityFrom"].as_str().unwrap().into()),to:Some(law["utilityTo"].as_str().unwrap().into())},"frozen"=>HostEvent::TimeTravelFrozen{window_id},"baseMoved"=>HostEvent::BaseMoved{window_id},"retiring"=>HostEvent::Retiring{window_id},_=>unreachable!()};trace.push(&event);expected.push(event);}assert_eq!(trace.take(),expected);assert!(trace.is_empty());let event=HostEvent::Retiring{window_id:String::new()};for _ in 0..=EVENTS{trace.push(&event)}assert!(trace.overflow);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||trace.take())).is_err());trace.clear();trace.push(&HostEvent::Retiring{window_id:"x".repeat(TEXT+1)});assert!(trace.overflow);
 }
}
