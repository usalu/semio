use super::*;

#[path="⚠️refusal/🦀️.rs"]
mod refusal;
static DENY_ALLOCATION_SIZE:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap()}
struct Control{maximum:usize,owned:usize,events:Vec<DeflateEncodeProgress>,cancel:Option<DeflateEncodePhase>,cancel_at:usize}
impl Control{fn new(maximum:usize)->Self{Self{maximum,owned:0,events:Vec::new(),cancel:None,cancel_at:usize::MAX}}}
impl DeflateEncodeControl for Control{
 fn admit(&mut self,bytes:usize)->Result<(),semio_framework_value::ValueError>{self.owned=self.owned.checked_add(bytes).filter(|bytes|*bytes<=self.maximum).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"test admission refused"))?;Ok(())}
 fn checkpoint(&mut self,event:DeflateEncodeProgress)->Result<(),semio_framework_value::ValueError>{self.events.push(event);if self.cancel==Some(event.phase)&&event.completed>=self.cancel_at{Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"test canceled"))}else{Ok(())}}
}
fn samples()->Vec<Vec<u8>>{let f=fixture();let mut samples:Vec<_>=f["cases"].as_array().unwrap().iter().map(|case|{let hex=case["hex"].as_str().unwrap();let bytes:Vec<u8>=(0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect();bytes.repeat(case["repeat"].as_u64().unwrap() as usize)}).collect();samples.push(lcg_bytes(f["random"]["seed"].as_u64().unwrap(),f["random"]["length"].as_u64().unwrap() as usize));samples}

#[test]
fn deflate_controlled_exact_stream_and_independent_zlib_oracles(){
 use std::io::Write;use std::process::{Command,Stdio};
 let f=fixture();for raw in samples(){let mut control=Control::new(f["maximumBytes"].as_u64().unwrap() as usize);let stored=deflate_controlled(&raw,&mut control).unwrap();assert_eq!(stored,deflate(&raw));assert_eq!(btype_of(&stored),1);assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&stored,raw.len().max(1)).unwrap(),raw);
 let script="import{inflateRawSync}from'node:zlib';const input=new Uint8Array(await Bun.stdin.arrayBuffer());await Bun.write(Bun.stdout,inflateRawSync(input));";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&stored).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));assert_eq!(result.stdout,raw);assert_eq!(control.events.last(),Some(&DeflateEncodeProgress{phase:DeflateEncodePhase::Finish,completed:stored.len(),total:stored.len()}));}
}

#[test]
fn deflate_controlled_real_interior_phases_and_admission(){
 let f=fixture();let raw=lcg_bytes(f["random"]["seed"].as_u64().unwrap(),f["random"]["length"].as_u64().unwrap() as usize);for phase in[DeflateEncodePhase::Initialize,DeflateEncodePhase::ScanInput,DeflateEncodePhase::MatchSearch,DeflateEncodePhase::WriteOutput]{let mut control=Control::new(f["maximumBytes"].as_u64().unwrap() as usize);control.cancel=Some(phase);control.cancel_at=f["cancelAt"].as_u64().unwrap() as usize;let error=deflate_controlled(&raw,&mut control).unwrap_err();assert!(error.message.contains("canceled"),"phase {phase:?}: {error}");let event=control.events.last().unwrap();assert_eq!(event.phase,phase);assert!(event.completed>=control.cancel_at);if phase==DeflateEncodePhase::ScanInput{assert_eq!(event.total,raw.len());}}
 let mut denied=Control::new(f["deniedBytes"].as_u64().unwrap() as usize);assert!(deflate_controlled(&raw,&mut denied).unwrap_err().message.contains("admission"));assert_eq!(denied.owned,0);
 let mut success=Control::new(f["maximumBytes"].as_u64().unwrap() as usize);deflate_controlled(&raw,&mut success).unwrap();let mut exact=Control::new(success.owned);deflate_controlled(&raw,&mut exact).unwrap();let mut short=Control::new(success.owned-1);assert!(deflate_controlled(&raw,&mut short).is_err());
}

#[test]
fn deflate_controlled_fixed_huffman_capacity_proof(){let lit=fixed_literal_length_lengths();let dist=fixed_distance_lengths();for length in MIN_MATCH..=MAX_MATCH{let index=length_index_for(length as u16);for distance in 1..=WINDOW_SIZE{let di=distance_index_for(distance as u32);assert!((lit[257+index]+LENGTH_EXTRA[index]+dist[di]+DIST_EXTRA[di]) as usize<=9*length);}}for raw in samples(){let output=deflate(&raw);assert!(output.len()<=(9*raw.len()+17)/8);}}

struct ObservedAllocator;
static OBSERVE_ALLOCATIONS:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
static WATCHED_SIZE:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static WATCHED_ALLOCATIONS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static LARGEST_ALLOCATION:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
unsafe impl std::alloc::GlobalAlloc for ObservedAllocator{
 unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{observe_allocation(layout.size());if DENY_ALLOCATION_SIZE.load(std::sync::atomic::Ordering::Relaxed)==layout.size(){return std::ptr::null_mut()}unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)}}
 unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{observe_allocation(layout.size());if DENY_ALLOCATION_SIZE.load(std::sync::atomic::Ordering::Relaxed)==layout.size(){return std::ptr::null_mut()}unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)}}
 unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{observe_allocation(size);if DENY_ALLOCATION_SIZE.load(std::sync::atomic::Ordering::Relaxed)==size{return std::ptr::null_mut()}unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)}}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
fn observe_allocation(bytes:usize){if OBSERVE_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed){LARGEST_ALLOCATION.fetch_max(bytes,std::sync::atomic::Ordering::Relaxed);if bytes==WATCHED_SIZE.load(std::sync::atomic::Ordering::Relaxed){WATCHED_ALLOCATIONS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);}}}
#[global_allocator]
static OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;
#[test]
fn deflate_controlled_actual_allocation_observes_refusal_before_large_buffers(){
 let f=fixture();let raw=lcg_bytes(f["random"]["seed"].as_u64().unwrap(),f["random"]["length"].as_u64().unwrap() as usize);
 for canceled in[false,true]{let mut control=Control::new(if canceled{f["maximumBytes"].as_u64().unwrap() as usize}else{f["deniedBytes"].as_u64().unwrap() as usize});if canceled{control.cancel=Some(DeflateEncodePhase::Initialize);control.cancel_at=f["cancelAt"].as_u64().unwrap() as usize;}
 LARGEST_ALLOCATION.store(0,std::sync::atomic::Ordering::Relaxed);OBSERVE_ALLOCATIONS.store(true,std::sync::atomic::Ordering::Relaxed);let result=deflate_controlled(&raw,&mut control);OBSERVE_ALLOCATIONS.store(false,std::sync::atomic::Ordering::Relaxed);
 assert!(result.is_err());assert!(LARGEST_ALLOCATION.load(std::sync::atomic::Ordering::Relaxed)<1024,"large table allocated before refusal");}
}

#[test]
fn deflate_controlled_output_phase_refusal_precedes_output_reservation(){
 let f=fixture();let raw=lcg_bytes(f["random"]["seed"].as_u64().unwrap(),f["random"]["length"].as_u64().unwrap() as usize);let mut control=Control::new(f["maximumBytes"].as_u64().unwrap() as usize);control.cancel=Some(DeflateEncodePhase::WriteOutput);control.cancel_at=0;
 WATCHED_SIZE.store((9*raw.len()+17)/8,std::sync::atomic::Ordering::Relaxed);WATCHED_ALLOCATIONS.store(0,std::sync::atomic::Ordering::Relaxed);OBSERVE_ALLOCATIONS.store(true,std::sync::atomic::Ordering::Relaxed);let result=deflate_controlled(&raw,&mut control);OBSERVE_ALLOCATIONS.store(false,std::sync::atomic::Ordering::Relaxed);
 assert!(result.unwrap_err().message.contains("canceled"));assert_eq!(WATCHED_ALLOCATIONS.load(std::sync::atomic::Ordering::Relaxed),0);assert_eq!(control.events.last().unwrap().phase,DeflateEncodePhase::WriteOutput);assert_eq!(control.events.last().unwrap().completed,0);
}
