use super::*;
use super::retained_encoding::{HeadSlot, HISTORY, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
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
    let policy = &fixture["policy"];
    assert_eq!((MAX_CHAIN as u64, NICE_MATCH as u64, MAX_INSERT as u64, u64::from(SKIP_SHIFT)), (policy["maximumChain"].as_u64().unwrap(), policy["niceMatch"].as_u64().unwrap(), policy["maximumInsert"].as_u64().unwrap(), policy["skipShift"].as_u64().unwrap()), "the compressor runs the policy of the shared schema");
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

/// 📏️ Drives one segment under exact grants and returns its stored bytes with the work the Pack charges for it: every quoted copy byte plus one bookkeeping byte per step once the matching pages are initialized (a fixed cost of a first segment, shared by every later one).
fn counted(raw: &[u8]) -> (Vec<u8>, usize, usize, std::collections::BTreeMap<usize, usize>) {
    let mut cursor = RetainedDeflateEncoder::default();
    let (mut pending, mut read, mut output, mut stored, mut work, mut init, mut steps) = (None, 0, [0], Vec::new(), 0usize, 0usize, std::collections::BTreeMap::new());
    for _ in 0..20_000_000 {
        if pending.is_none() && read < raw.len() { pending = Some(raw[read]); read += 1; }
        let grant = exact(&cursor);
        let initializing = cursor.phase() == RetainedDeflatePhase::Initialize;
        let step = eff(grant, |effect| cursor.advance(&mut pending, read == raw.len(), &mut output, effect)).unwrap();
        if initializing { init += step.progress.copied_bytes + 1; *steps.entry(step.progress.copied_bytes + 1).or_insert(0) += 1; } else { work += step.progress.copied_bytes + 1; }
        stored.extend_from_slice(&output[..step.written_bytes]);
        if step.complete { close(&mut cursor); return (stored, work, init, steps); }
    }
    close(&mut cursor);
    panic!("retained Deflate producer did not terminate");
}

/// 🧬️ A record-like body: a few hundred field shapes repeated with varying numbers, the structure of a typed Pack document.
fn record_like(count: usize) -> Vec<u8> {
    let noise = super::tests::lcg_bytes(7, count * 3);
    let mut body = Vec::new();
    for index in 0..count {
        let shape = noise[index * 3] % 12;
        body.extend_from_slice(format!("\u{1}op{shape}\u{2}operands[{}, {}]\u{3}", noise[index * 3 + 1], noise[index * 3 + 2]).as_bytes());
        body.extend_from_slice(b"\x00\x00\x00\x00\x01\x00\x00\x00path/to/the/common/prefix/");
    }
    body
}

/// 🔮️ The work per input byte follows from the cost of one token, never from how repetitive the input is. A position that is searched and found no match costs its input (2), Search (12), Token (2), insert (24), at most `MAX_CHAIN` probes of a mismatching first byte (3 Compare, 5 chain advance) and its output (3): `SEARCHED_LITERAL`. A position skipped after a literal run costs its input, Token and output only: `SKIPPED_LITERAL`. A match costs one Search, probes that each ended below `NICE_MATCH` (so at most `3 * NICE_MATCH + 5` each), its own compare at 3 per byte, and at most `MAX_INSERT` inserts, over at least three bytes, which stays under `SEARCHED_LITERAL` per byte. So no input costs more than `SEARCHED_LITERAL` per byte, an input without matches costs what the skip schedule searches (one position per `1 + misses >> SKIP_SHIFT`), and the matching pages cost their initialization once.
#[test]
fn deflate_retained_work_per_input_byte_is_bounded_by_the_chain_and_nice_limits() {
    const SEARCHED_LITERAL: usize = 2 + 12 + 2 + 24 + 8 * MAX_CHAIN + 3;
    const SKIPPED_LITERAL: usize = 2 + 2 + 3;
    let inputs = [b"A".repeat(70_000), b"abc".repeat(30_000), b"abcabcabbcabc".repeat(5_000), super::tests::lcg_bytes(11, 40_000), record_like(4_000), (0..60_000u32).map(|index| (index % 251) as u8).collect()];
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let initialization = fixture["initializationCopyBytes"].as_u64().unwrap() as usize;
    let tables = HASH_SIZE * size_of::<HeadSlot>() + WINDOW_SIZE * size_of::<u32>() + HISTORY;
    assert!(initialization > tables, "initialization writes at least every table byte ({tables}) and pays for its page headers");
    let mut costs = Vec::new();
    for raw in &inputs {
        let (stored, work, init, steps) = counted(raw);
        assert_eq!(init, initialization, "initializing the matching pages costs the schema-pinned copy (every table byte plus the page reservations); charged steps (bytes, count): {steps:?}");
        assert_eq!(stored, deflate(raw));
        assert_eq!(miniz_oxide::inflate::decompress_to_vec_with_limit(&stored, raw.len()).unwrap(), *raw);
        assert!(work <= raw.len() * SEARCHED_LITERAL, "{work} B of work for {} input bytes, above {SEARCHED_LITERAL} per byte", raw.len());
        costs.push(work);
    }
    assert!(!costs.is_empty());
    let (mut position, mut searched) = (0usize, 0usize);
    while position < inputs[3].len() { searched += 1; position += 1 + ((position + 1) >> SKIP_SHIFT); }
    let model = searched * SEARCHED_LITERAL + (inputs[3].len() - searched) * SKIPPED_LITERAL;
    let (random_stored, ..) = counted(&inputs[3]);
    assert!(random_stored.len() >= inputs[3].len(), "the incompressible input holds no match, so the skip schedule is never reset");
    assert!(costs[3] <= model, "incompressible input costs {} B, above the {model} B its skip schedule searches ({searched} positions of {})", costs[3], inputs[3].len());
}
