use super::*;
use super::retained_encoding::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::io::Write;
use std::process::{Command, Stdio};
#[path="../../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
/// 🪢️ Runs one cursor action inside one actual original effect; the heap scope belongs to the closure.
fn eff<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> R { effect::with_effect(grant, 1 << 20, &mut None, operation) }

fn exact(cursor: &RetainedDeflateEncoder) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_copy_byte_demand().unwrap(), maximum_capacity_bytes: cursor.next_capacity_byte_demand().unwrap(), maximum_release_bytes: 0, maximum_depth: cursor.next_depth_demand().unwrap() }
}

fn close(cursor: &mut RetainedDeflateEncoder) {
    cursor.begin_close();
    for _ in 0..200_000 {
        if cursor.terminal_is_empty() { return; }
        let demand = cursor.retirement_demands().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let (step, allocated, released) = eff(grant,|effect|super::tests::controlled::observe_backing(||cursor.close_step(effect).unwrap()));
        let receipt = match step { RetainedCloneStep::Progress(receipt) | RetainedCloneStep::Complete(receipt) => receipt };
        assert_eq!((allocated, released), (receipt.retained_capacity_bytes, receipt.released_bytes));
        assert!(receipt.copied_items <= 1 && receipt.released_bytes <= 4096);
    }
    panic!("retained Deflate close did not terminate");
}

fn encode(raw: &[u8], reject: bool) -> Vec<u8> {
    let mut cursor = RetainedDeflateEncoder::default();
    let mut pending = None;
    let mut read = 0;
    let mut output = [0];
    let mut stored = Vec::with_capacity(raw.len() * 2 + 2);
    for _ in 0..5_000_000 {
        if pending.is_none() && read < raw.len() { pending = Some(raw[read]); read += 1; }
        let grant = exact(&cursor);
        if reject {
            for short in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }] {
                if short == grant { continue; }
                let before = (cursor.phase(), cursor.input_len(), cursor.output_len(), pending);
                let (step, allocated, released) = eff(short,|effect|super::tests::controlled::observe_backing(||cursor.advance(&mut pending, read == raw.len(), &mut output,effect).unwrap()));
                assert_eq!(step.progress, RetainedCloneProgress::default());
                assert_eq!((allocated, released), (0, 0));
                assert_eq!((cursor.phase(), cursor.input_len(), cursor.output_len(), pending), before);
            }
        }
        let (step, allocated, released) = eff(grant,|effect|super::tests::controlled::observe_backing(||cursor.advance(&mut pending, read == raw.len(), &mut output,effect).unwrap()));
        assert_eq!((allocated, released), (step.progress.retained_capacity_bytes, step.progress.released_bytes));
        assert!(allocated <= 4096 && step.progress.copied_items <= 1 && step.progress.copied_bytes <= grant.maximum_copy_bytes);
        stored.extend_from_slice(&output[..step.written_bytes]);
        if step.complete { close(&mut cursor); return stored; }
    }
    close(&mut cursor);
    panic!("retained Deflate producer did not terminate");
}

/// 🔮️ Exact portable streams and long matching windows agree with ordinary output and system zlib.
#[test]
fn deflate_controlled_retained_exact_oracles_and_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert!(size_of::<RetainedDeflateEncoder>() <= fixture["maximumInlineBytes"].as_u64().unwrap() as usize);
    let mut inputs = Vec::new();
    for vector in fixture["vectors"].as_array().unwrap() {
        let raw: Vec<u8> = vector["raw"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
        let expected: Vec<u8> = vector["stored"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
        assert_eq!(encode(&raw, true), expected);
        inputs.push(raw);
    }
    inputs.extend([b"abcabcabbcabc".repeat(3000), b"A".repeat(70_000), super::tests::lcg_bytes(179, 40_000)]);
    for raw in inputs {
        let stored = encode(&raw, false);
        assert_eq!(stored, deflate(&raw));
        assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&stored, raw.len().max(1)).unwrap(), raw);
        let mut child = Command::new("bun").args(["-e", "import{inflateRawSync}from'node:zlib';process.stdout.write(inflateRawSync(new Uint8Array(await Bun.stdin.arrayBuffer())));"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&stored).unwrap();
        let oracle = child.wait_with_output().unwrap();
        assert!(oracle.status.success(), "{}", String::from_utf8_lossy(&oracle.stderr));
        assert_eq!(oracle.stdout, raw);
    }
}

/// 🍂️ Every retained cancellation phase closes original heap pages under exact release grants.
#[test]
fn deflate_controlled_retained_cancellation_frontiers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let raw = b"aaaaabaaaaaab".repeat(40);
    for phase in fixture["cancellationPhases"].as_array().unwrap() {
        let mut cursor = RetainedDeflateEncoder::default();
        let mut pending = None;
        let mut read = 0;
        let mut output = [0];
        let mut reached = false;
        for _ in 0..200_000 {
            if cursor.phase().as_str() == phase.as_str().unwrap() { reached = true; break; }
            if pending.is_none() && read < raw.len() { pending = Some(raw[read]); read += 1; }
            let grant = exact(&cursor);
            eff(grant,|effect|cursor.advance(&mut pending,read == raw.len(),&mut output,effect)).unwrap();
        }
        close(&mut cursor);
        assert!(reached, "frontier {phase}");
    }
}

#[test]
fn deflate_controlled_retained_restart_preserves_canonical_independent_windows(){
 let mut cursor=RetainedDeflateEncoder::default();let inputs=[b"A".repeat(40000),b"abc".repeat(10000),Vec::new(),b"A".repeat(60000)];
 for (index,raw) in inputs.iter().enumerate(){let mut pending=None;let mut read=0;let mut actual=Vec::new();let mut output=[0;1];let mut born=0;let mut turns=0;
  for _ in 0..1000000{if pending.is_none()&&read<raw.len(){pending=Some(raw[read]);read+=1;}let grant=exact(&cursor);let(step,birth,free)=eff(grant,|effect|super::tests::controlled::observe_backing(||cursor.advance(&mut pending,read==raw.len(),&mut output,effect).unwrap()));assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));born+=birth;actual.extend_from_slice(&output[..step.written_bytes]);turns+=1;if step.complete{break;}}
  let complete=cursor.is_complete();assert_eq!(actual,deflate(raw));assert_eq!(miniz_oxide::inflate::decompress_to_vec(&actual).unwrap(),*raw);if index!=0{assert_eq!(born,0);assert!(turns<300000);}if index+1<inputs.len(){cursor.restart().unwrap();assert_eq!(cursor.phase(),RetainedDeflatePhase::Input);}assert!(complete);
 }close(&mut cursor);
}

/// 🪙️ Fixed matching pages fit a bounded original opportunity before any input and during closure.
#[test]
fn deflate_retained_matching_pages_fit_original_opportunity(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let mut cursor=RetainedDeflateEncoder::default();let mut pending=None;let mut output=[0];let mut initialization=0;
 while cursor.phase()==RetainedDeflatePhase::Initialize{let grant=exact(&cursor);eff(grant,|effect|cursor.advance(&mut pending,false,&mut output,effect)).unwrap();initialization+=1;}
 cursor.begin_close();let mut closure=0;while !cursor.terminal_is_empty(){let d=cursor.retirement_demands().unwrap();eff(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth},|effect|cursor.close_step(effect)).unwrap();closure+=1;}
 assert!(initialization<=fixture["maximumInitializationTurns"].as_u64().unwrap()as usize,"matching-page initialization spent {initialization} turns");assert!(closure<=fixture["maximumClosureTurns"].as_u64().unwrap()as usize,"matching-page closure spent {closure} turns");
 for vector in fixture["vectors"].as_array().unwrap(){let raw:Vec<u8>=vector["raw"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let stored=encode(&raw,true);assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&stored,raw.len().max(1)).unwrap(),raw);}
}

/// 🧱️ Reserved append depth and Copy collection retirement quote the real original work.
#[test]
fn deflate_retained_batch_depth_and_retirement_grants(){
 let mut cursor=RetainedDeflateEncoder::default();let mut pending=None;let mut output=[0];while cursor.next_capacity_byte_demand().unwrap()!=0{let grant=exact(&cursor);eff(grant,|effect|cursor.advance(&mut pending,false,&mut output,effect)).unwrap();}
 let depth=cursor.next_depth_demand().unwrap();let mut owner=semio_framework_value_persistent::list::PagedList::<u64,32768>::empty();while !owner.has_reserved_slot(){eff(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:0,maximum_depth:4096},|effect|owner.reserve_one(effect)).result.unwrap();}let expected=owner.next_push_depth_demand().unwrap()+1;
 let grant=exact(&cursor);let initialized=eff(grant,|effect|cursor.advance(&mut pending,false,&mut output,effect)).unwrap().progress.copied_bytes;cursor.begin_close();let d=cursor.retirement_demands().unwrap();let short=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:initialized.saturating_sub(1),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:d.depth};let(step,allocated,released)=eff(short,|effect|super::tests::controlled::observe_backing(||cursor.close_step(effect).unwrap()));let refused=step.progress();let unchanged=cursor.retirement_demands().unwrap()==d;let quoted=d.copy_bytes;close(&mut cursor);
 assert_eq!(depth,expected,"reserved append depth");assert_eq!(quoted,initialized,"Copy collection payload quote");assert_eq!(refused,RetainedCloneProgress::default());assert!(unchanged);assert_eq!((allocated,released),(0,0));
}
