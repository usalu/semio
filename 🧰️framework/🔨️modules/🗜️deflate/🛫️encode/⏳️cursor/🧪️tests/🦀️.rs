use super::*;
use super::retained_encoding::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::io::Write;
use std::process::{Command, Stdio};

fn exact(cursor: &RetainedDeflateEncoder) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_copy_byte_demand().unwrap(), maximum_capacity_bytes: cursor.next_capacity_byte_demand().unwrap(), maximum_release_bytes: 0, maximum_depth: cursor.next_depth_demand().unwrap() }
}

fn close(cursor: &mut RetainedDeflateEncoder) {
    cursor.begin_close();
    for _ in 0..200_000 {
        if cursor.terminal_is_empty() { return; }
        let demand = cursor.retirement_demands().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let (step, allocated, released) = super::tests::controlled::observe_backing(|| cursor.close_step(grant).unwrap());
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
                let (step, allocated, released) = super::tests::controlled::observe_backing(|| cursor.advance(&mut pending, read == raw.len(), &mut output, short).unwrap());
                assert_eq!(step.progress, RetainedCloneProgress::default());
                assert_eq!((allocated, released), (0, 0));
                assert_eq!((cursor.phase(), cursor.input_len(), cursor.output_len(), pending), before);
            }
        }
        let (step, allocated, released) = super::tests::controlled::observe_backing(|| cursor.advance(&mut pending, read == raw.len(), &mut output, grant).unwrap());
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
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
        println!("[DEBUG] retained Deflate {} raw bytes -> {} canonical bytes; system zlib agrees", raw.len(), stored.len());
    }
}

/// 🍂️ Every retained cancellation phase closes original heap pages under exact release grants.
#[test]
fn deflate_controlled_retained_cancellation_frontiers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
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
            cursor.advance(&mut pending, read == raw.len(), &mut output, grant).unwrap();
        }
        close(&mut cursor);
        assert!(reached, "frontier {phase}");
        println!("[DEBUG] retained Deflate cancellation frontier {phase} reached terminal emptiness");
    }
}

#[test]
fn deflate_controlled_retained_restart_preserves_canonical_independent_windows(){
 let mut cursor=RetainedDeflateEncoder::default();let inputs=[b"A".repeat(40000),b"abc".repeat(10000),Vec::new(),b"A".repeat(60000)];
 for (index,raw) in inputs.iter().enumerate(){let mut pending=None;let mut read=0;let mut actual=Vec::new();let mut output=[0;1];let mut born=0;let mut turns=0;
  for _ in 0..1000000{if pending.is_none()&&read<raw.len(){pending=Some(raw[read]);read+=1;}let grant=exact(&cursor);let(step,birth,free)=super::tests::controlled::observe_backing(||cursor.advance(&mut pending,read==raw.len(),&mut output,grant).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));born+=birth;actual.extend_from_slice(&output[..step.written_bytes]);turns+=1;if step.complete{break;}}
  let complete=cursor.is_complete();assert_eq!(actual,deflate(raw));assert_eq!(miniz_oxide::inflate::decompress_to_vec(&actual).unwrap(),*raw);if index!=0{assert_eq!(born,0);assert!(turns<300000);}if index+1<inputs.len(){cursor.restart().unwrap();assert_eq!(cursor.phase(),RetainedDeflatePhase::Input);}assert!(complete);eprintln!("[DEBUG] retained Deflate generation{index} exact{}bytes no extra backing after first generation, paidturns{turns}",actual.len());
 }close(&mut cursor);
}
