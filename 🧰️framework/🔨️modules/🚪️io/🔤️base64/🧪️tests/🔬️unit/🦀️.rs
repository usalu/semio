
use super::*;

#[test]
fn initial_invalid_length_is_borrowed_without_invented_work(){let mut cursor=Base64DecodeCursor::new(Base64Input::Borrowed(b"A"));assert_eq!(cursor.refusal(),Some(&Base64ControlError::Codec(Base64Error::InvalidLength)));assert_eq!(cursor.next_work_demand(0),0);let mut callback=|_|true;let mut control=Base64Control{maximum_output_bytes:1,progress:&mut callback};assert_eq!(cursor.step(0,&mut control),Err(Base64ControlError::Codec(Base64Error::InvalidLength)));assert_eq!(cursor.last_turn_receipt(),Default::default());}

#[test]
fn adaptive_padding_prescan_keeps_the_same_first_refusal(){let mut wire=vec![b'A';8192];wire[0]=b'?';wire[100]=b'=';for budget in [4,8,4096,8192]{let mut cursor=Base64DecodeCursor::new(Base64Input::Borrowed(&wire));let mut callback=|_|true;let mut control=Base64Control{maximum_output_bytes:8192,progress:&mut callback};let mut refusal=None;for _ in 0..10000{match cursor.step(budget,&mut control){Err(error)=>{refusal=Some(error);break},Ok(_)=>{}}}assert_eq!(refusal,Some(Base64ControlError::Codec(Base64Error::InvalidPadding)));}}

/// 🎟️ The original finite work budget selects an actual quartet and its exact output quote.
#[test]
fn adaptive_quartet_budgets_match_neutral_cases_and_independent_codec(){
 use base64::Engine as _;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧮️accounting/🔣️budget.json")).unwrap();
 for row in cases.as_array().unwrap(){let length=row["length"].as_u64().unwrap()as usize;let expected=row["output"].as_u64().unwrap()as usize;let mut wire=vec![b'A';length];let padding=length/4*3-expected;for byte in &mut wire[length-padding..]{*byte=b'='}let oracle=base64::engine::general_purpose::STANDARD.decode(&wire).unwrap();let mut cursor=Base64DecodeCursor::new(Base64Input::OwnedBytes(wire));let mut callback=|_|true;let mut control=Base64Control{maximum_output_bytes:expected,progress:&mut callback};cursor.step(1,&mut control).unwrap();let target=if row["phase"]=="validate"{Base64Phase::Validate}else{Base64Phase::Decode};while cursor.progress().phase!=target{let demand=cursor.next_work_demand(8192);assert!(demand>0);cursor.step(demand,&mut control).unwrap();}if target==Base64Phase::Decode{cursor.step(1,&mut control).unwrap();}let offset=row["offset"].as_u64().unwrap()as usize;while cursor.progress().completed<offset{cursor.step(4,&mut control).unwrap();}let budget=row["budget"].as_u64().unwrap()as usize;assert_eq!(cursor.next_work_demand(budget),row["work"].as_u64().unwrap()as usize);assert_eq!(cursor.next_copy_byte_demand(budget),row["copy"].as_u64().unwrap()as usize);let before=cursor.progress();let result=cursor.step(budget,&mut control).unwrap();let receipt=cursor.last_turn_receipt();assert!(receipt.work_units<=budget);assert_eq!(receipt.copied_bytes,row["copy"].as_u64().unwrap()as usize);if row["work"]==0{assert_eq!(result,Base64DecodeState::Pending);assert_eq!(cursor.progress(),before);assert_eq!(receipt,Base64DecodeReceipt::default())}for _ in 0..100_000{if cursor.is_complete(){break}cursor.step(4096,&mut control).unwrap();}assert!(cursor.is_complete());assert_eq!(cursor.take_output().unwrap(),oracle);}
}

/// 🧮️ Actual visited bytes and retained allocations remain observable on the first codec refusal.
#[test]
fn retained_decode_receipts_match_original_effects_and_independent_codec(){
 use base64::Engine as _;
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧮️accounting/cases.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let wire=format!("{}{}",row["prefix"].as_str().unwrap(),row["word"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize));
  let oracle=base64::engine::general_purpose::STANDARD.decode(&wire);assert_eq!(oracle.is_ok(),row["valid"].as_bool().unwrap());
  let pointer=wire.as_ptr();let mut cursor=Base64DecodeCursor::new(Base64Input::OwnedText(wire));let mut receipt=Base64DecodeReceipt::default();
  let mut callback=|event:Base64Progress|match row["cancel"].as_str().unwrap(){"validate-start"=>event.phase!=Base64Phase::Validate||event.completed!=0,"decode-start"=>event.phase!=Base64Phase::Decode||event.completed!=0,"decode-end"=>event.phase!=Base64Phase::Decode||event.completed==0,_=>true};
  let mut control=Base64Control{maximum_output_bytes:row["limit"].as_u64().unwrap()as usize,progress:&mut callback};
  let refused=loop{
   let demand=cursor.next_work_demand(8192);let before=cursor.progress();
   if demand>0{let denied=if cursor.next_work_demand(demand-1)==0{demand-1}else{0};let(result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.step(denied,&mut control));assert_eq!(result,Ok(Base64DecodeState::Pending));assert_eq!(cursor.last_turn_receipt(),Default::default());assert_eq!(cursor.progress(),before);assert_eq!((allocated,released),(0,0));}
   let(result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.step(demand,&mut control));let actual=cursor.last_turn_receipt();assert_eq!(actual.allocated_capacity_bytes,allocated);assert_eq!(released,0);assert!(actual.work_units<=demand);
   receipt.work_units+=actual.work_units;receipt.read_bytes+=actual.read_bytes;receipt.copied_bytes+=actual.copied_bytes;receipt.allocated_capacity_bytes+=actual.allocated_capacity_bytes;
   match result{Err(_)=>break true,Ok(Base64DecodeState::Complete)=>break false,Ok(Base64DecodeState::Pending)=>{}}
  };
  assert_eq!((receipt.work_units,receipt.read_bytes,receipt.copied_bytes,receipt.allocated_capacity_bytes),(row["work"].as_u64().unwrap()as usize,row["read"].as_u64().unwrap()as usize,row["copy"].as_u64().unwrap()as usize,row["capacity"].as_u64().unwrap()as usize),"{}",row["id"]);
  let(_,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.step(usize::MAX,&mut control));assert_eq!((allocated,released),(0,0));assert_eq!(cursor.last_turn_receipt(),Default::default());
  let(parts,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.into_parts());assert_eq!((allocated,released),(0,0));assert_eq!(matches!(parts.outcome,Base64DecodeOutcome::Refused(_)),refused);
  match parts.input{Base64Input::OwnedText(input)=>assert_eq!(input.as_ptr(),pointer),_=>unreachable!()}
  let bytes=parts.output.unwrap_or_default();assert_eq!(bytes.as_slice(),row["output"].as_str().unwrap().as_bytes());
  if !refused{assert_eq!(bytes,oracle.unwrap());}
 }
}

/// 🧺️ Multiple funded phases accumulate one receipt and a finished caller turn resets it.
#[test]
fn retained_decode_receipt_accumulates_all_phases_in_one_turn(){
 let mut cursor=Base64DecodeCursor::new(Base64Input::Borrowed(b"Zm9v"));let mut callback=|_|true;let mut control=Base64Control{maximum_output_bytes:3,progress:&mut callback};
 let(result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.step(usize::MAX,&mut control));
 assert_eq!(result,Ok(Base64DecodeState::Complete));assert_eq!((allocated,released),(3,0));assert_eq!(cursor.last_turn_receipt(),Base64DecodeReceipt{work_units:12,read_bytes:8,copied_bytes:3,allocated_capacity_bytes:3});
 assert_eq!(cursor.take_output().unwrap(),b"foo");assert_eq!(cursor.step(0,&mut control),Ok(Base64DecodeState::Complete));assert_eq!(cursor.last_turn_receipt(),Default::default());
}

/// 📐️ RFC 4648 §10 vectors, kept as a language-agnostic fixture so any implementation in any
/// language can be checked against the same table.
#[test]
fn matches_rfc4648_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️rfc4648-base64-vectors.json")).expect("fixture JSON");
    for case in fixture["cases"].as_array().expect("cases array") {
        let input = case["input_utf8"].as_str().expect("input_utf8");
        let expected = case["encoded"].as_str().expect("encoded");
        assert_eq!(base64_standard_encode(input), expected, "encode({input:?})");
        assert_eq!(base64_standard_decode(expected).expect("decode fixture"), input.as_bytes(), "decode({expected:?})");
    }
}

#[test]
fn round_trips_every_byte_and_chunk_remainder() {
    let raw: Vec<u8> = (0u8..=u8::MAX).chain([0, 1, 2, 3, 4]).collect();
    let encoded = base64_standard_encode(&raw);
    assert_eq!(base64_standard_decode(encoded.as_bytes()), Ok(raw));
}

#[test]
fn rejects_malformed_and_noncanonical_inputs() {
    assert_eq!(base64_standard_decode(b"Zg" as &[u8]), Err(Base64Error::InvalidLength));
    assert_eq!(base64_standard_decode(b"Z g=" as &[u8]), Err(Base64Error::InvalidByte { index: 1, byte: b' ' }));
    assert_eq!(base64_standard_decode(b"=m9v" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zm=v" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zg==Zm8=" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zh==" as &[u8]), Err(Base64Error::NonCanonicalTrailingBits));
    assert_eq!(base64_standard_decode(b"Zm9=" as &[u8]), Err(Base64Error::NonCanonicalTrailingBits));
}

/// 🎲️ A tiny deterministic linear-congruential generator — no `rand` dependency, but still
/// exercises many distinct, reproducible byte strings across a differential run.
struct Lcg(u64);

impl Lcg {
    fn next_byte(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as u8
    }
}

/// 🔬️ Differential oracle: round-trips deterministic pseudo-random byte strings of every
/// length from 0 to 128 through both this codec and the third-party `base64` crate (a
/// dev-only dependency, never a runtime one) and asserts byte-for-byte agreement both ways.
#[test]
fn matches_third_party_base64_oracle() {
    use base64::Engine as _;
    let oracle = base64::engine::general_purpose::STANDARD;
    let mut lcg = Lcg(0x9E3779B97F4A7C15);
    for length in 0..=128usize {
        let bytes: Vec<u8> = (0..length).map(|_| lcg.next_byte()).collect();
        let ours_encoded = base64_standard_encode(&bytes);
        let oracle_encoded = oracle.encode(&bytes);
        assert_eq!(ours_encoded, oracle_encoded, "encode mismatch at length {length}");
        let oracle_decoded = oracle.decode(&ours_encoded).expect("oracle decode of our encoding");
        assert_eq!(oracle_decoded, bytes, "oracle decode of our encoding at length {length}");
        let ours_decoded = base64_standard_decode(&oracle_encoded).expect("our decode of oracle encoding");
        assert_eq!(ours_decoded, bytes, "our decode of oracle encoding at length {length}");
    }
}

/// 🔗️ RFC 4648 §10 vectors re-spelled for §5 base64url without padding, plus URL-alphabet rows.
#[test]
fn matches_rfc4648_base64url_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️rfc4648-base64url-vectors.json")).expect("fixture JSON");
    for case in fixture["cases"].as_array().expect("cases array") {
        let bytes = case["input_hex"].as_str().map(|hex| (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex")).collect::<Vec<u8>>()).expect("input_hex");
        let expected = case["encoded"].as_str().expect("encoded");
        assert_eq!(base64_url_encode(&bytes), expected, "encode({bytes:?})");
        assert_eq!(base64_url_decode(expected).expect("decode fixture"), bytes, "decode({expected:?})");
    }
    for case in fixture["rejected"].as_array().expect("rejected array") {
        assert!(base64_url_decode(case["encoded"].as_str().expect("encoded")).is_err(), "{} must be rejected", case["encoded"]);
    }
}

/// 🔮️ Differential oracle for base64url: the third-party `URL_SAFE_NO_PAD` engine both ways.
#[test]
fn matches_third_party_base64url_oracle() {
    use base64::Engine as _;
    let oracle = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let mut lcg = Lcg(0xD1B5_4A32_D192_ED03);
    for length in 0..=128usize {
        let bytes: Vec<u8> = (0..length).map(|_| lcg.next_byte()).collect();
        let ours = base64_url_encode(&bytes);
        assert_eq!(ours, oracle.encode(&bytes), "encode mismatch at length {length}");
        assert_eq!(base64_url_decode(oracle.encode(&bytes)).expect("our decode of oracle encoding"), bytes);
        assert_eq!(oracle.decode(&ours).expect("oracle decode of our encoding"), bytes);
    }
}
/// 🧮️ Allocation-free quartet decoding agrees with the independent standard engine.
#[test]
fn quartets_match_third_party_and_refuse_noncanonical_groups(){
    use base64::Engine as _;
    let engine=base64::engine::general_purpose::STANDARD;
    for length in 1..=128{let bytes:Vec<u8>=(0..length).map(|i|((i*73+length*19)&255)as u8).collect();let text=engine.encode(&bytes);let mut output=Vec::new();
        for (index,quad) in text.as_bytes().as_chunks::<4>().0.iter().enumerate(){let (value,count)=decode_standard_quad(*quad,index*4,(index+1)*4==text.len()).unwrap();output.extend_from_slice(&value[..count]);}
        assert_eq!(output,bytes);assert_eq!(output,engine.decode(&text).unwrap());
    }
    for quad in [*b"=m9v",*b"Zm=v",*b"Zh==",*b"Zm9=",*b"Z g=",*b"Zm_="]{assert!(decode_standard_quad(quad,0,true).is_err());}
    for quad in [*b"Zg==",*b"Zm8="]{assert!(decode_standard_quad(quad,0,false).is_err());}
}

fn controlled_corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🎛️controlled/🔣️.json")).unwrap()
}

fn controlled_bytes(length: usize, corpus: &serde_json::Value) -> Vec<u8> {
    let generator = &corpus["generator"];
    let multiplier = generator["multiplier"].as_u64().unwrap() as usize;
    let length_multiplier = generator["lengthMultiplier"].as_u64().unwrap() as usize;
    let modulus = generator["modulus"].as_u64().unwrap() as usize;
    (0..length).map(|index| ((index * multiplier + length * length_multiplier) % modulus) as u8).collect()
}

/// 🎛️ The declared allowance covers every codec-owned request; validation owns no temporary buffer.
#[test]
fn controlled_output_is_the_only_allocation_and_matches_independent_engine() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    let oracle = base64::engine::general_purpose::STANDARD;
    for length in corpus["lengths"].as_array().unwrap() {
        let length = length.as_u64().unwrap() as usize;
        let bytes = controlled_bytes(length, &corpus);
        let text = oracle.encode(&bytes);
        let validation_requests = std::cell::Cell::new(0);
        let mut accepted = |progress: Base64Progress| {
            if progress.phase == Base64Phase::Validate {
                validation_requests.set(semio_framework_trace_memory_testing::observed_requested_bytes().unwrap_or(0));
            }
            true
        };
        let mut control = Base64Control { maximum_output_bytes: length, progress: &mut accepted };
        let (decoded, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| base64_standard_decode_controlled(text.as_bytes(), &mut control));
        let decoded = decoded.unwrap();
        assert_eq!(decoded, bytes);
        assert_eq!(decoded, oracle.decode(&text).unwrap());
        assert_eq!(validation_requests.get(), 0, "validation length {length}");
        assert_eq!(requests, length, "decode requests length {length}");
        assert_eq!(releases, 0, "decoder retains only its returned output");
        assert_eq!(decoded.capacity(), length);
        let mut accepted = |_| true;
        let mut control = Base64Control { maximum_output_bytes: text.len(), progress: &mut accepted };
        let (encoded, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| base64_standard_encode_controlled(&bytes, &mut control));
        let encoded = encoded.unwrap();
        assert_eq!(encoded, text);
        assert_eq!(requests, text.len(), "encode requests length {length}");
        assert_eq!(releases, 0, "encoder retains only its returned output");
        assert_eq!(encoded.capacity(), text.len());
        if length != 0 {
            let mut control = Base64Control { maximum_output_bytes: length - 1, progress: &mut accepted };
            let (result, requests) = semio_framework_trace_memory_testing::observe(|| base64_standard_decode_controlled(text.as_bytes(), &mut control));
            assert_eq!(result, Err(Base64ControlError::OutputLimit));
            assert_eq!(requests, 0);
            let mut control = Base64Control { maximum_output_bytes: text.len() - 1, progress: &mut accepted };
            let (result, requests) = semio_framework_trace_memory_testing::observe(|| base64_standard_encode_controlled(&bytes, &mut control));
            assert_eq!(result, Err(Base64ControlError::OutputLimit));
            assert_eq!(requests, 0);
        }
    }
}

/// 🛑️ Cancellation closes the sole output owner and creates none during validation.
#[test]
fn controlled_cancellation_preserves_phase_and_exact_output_custody() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    let bytes = controlled_bytes(corpus["cancellationInputLength"].as_u64().unwrap() as usize, &corpus);
    let text = base64::engine::general_purpose::STANDARD.encode(&bytes);
    for case in corpus["cancellations"].as_array().unwrap() {
        let phase = match case["phase"].as_str().unwrap() { "validate" => Base64Phase::Validate, "decode" => Base64Phase::Decode, "encode" => Base64Phase::Encode, _ => unreachable!() };
        let stop = case["completed"].as_u64().unwrap() as usize;
        let last = std::cell::Cell::new(None);
        let mut callback = |progress: Base64Progress| {
            last.set(Some(progress));
            !(progress.phase == phase && progress.completed == stop)
        };
        let expected = if phase == Base64Phase::Encode { text.len() } else { bytes.len() };
        let mut control = Base64Control { maximum_output_bytes: expected, progress: &mut callback };
        let (result, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| {
            if phase == Base64Phase::Encode { base64_standard_encode_controlled(&bytes, &mut control).map(|_| ()) }
            else { base64_standard_decode_controlled(text.as_bytes(), &mut control).map(|_| ()) }
        });
        assert_eq!(result, Err(Base64ControlError::Cancelled));
        let event = last.get().unwrap();
        assert_eq!((event.phase, event.completed), (phase, stop));
        assert_eq!(requests, if case["ownsOutput"].as_bool().unwrap() { expected } else { 0 });
        assert_eq!(releases, requests);
    }
}

/// 🚨️ Strict codec errors and existing resource precedence refuse before any owned storage.
#[test]
fn controlled_malformed_inputs_allocate_nothing_and_preserve_refusal_precedence() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    for case in corpus["malformed"].as_array().unwrap() {
        let encoded = case["encoded"].as_str().unwrap();
        let error = match case["kind"].as_str().unwrap() {
            "invalidLength" => Base64Error::InvalidLength,
            "invalidPadding" => Base64Error::InvalidPadding,
            "nonCanonicalTrailingBits" => Base64Error::NonCanonicalTrailingBits,
            "invalidByte" => Base64Error::InvalidByte { index: case["index"].as_u64().unwrap() as usize, byte: case["byte"].as_u64().unwrap() as u8 },
            _ => unreachable!(),
        };
        let mut accepted = |_| true;
        let mut control = Base64Control { maximum_output_bytes: encoded.len(), progress: &mut accepted };
        let (result, requests) = semio_framework_trace_memory_testing::observe(|| base64_standard_decode_controlled(encoded.as_bytes(), &mut control));
        assert_eq!(result, Err(Base64ControlError::Codec(error)));
        assert_eq!(requests, 0, "malformed {encoded}");
        assert!(base64::engine::general_purpose::STANDARD.decode(encoded).is_err());
    }
    let mut encoded = "Zm9v".repeat(2048).into_bytes();
    encoded[4097] = b' ';
    let mut accepted = |_| true;
    let mut control = Base64Control { maximum_output_bytes: encoded.len(), progress: &mut accepted };
    let (result, requests) = semio_framework_trace_memory_testing::observe(|| base64_standard_decode_controlled(&encoded, &mut control));
    assert_eq!(result, Err(Base64ControlError::Codec(Base64Error::InvalidByte { index: 4097, byte: b' ' })));
    assert_eq!(requests, 0);
    let competing = &corpus["competingRefusal"];
    encoded = competing["block"].as_str().unwrap().repeat(competing["repeats"].as_u64().unwrap() as usize).into_bytes();
    encoded[competing["invalidByteOffset"].as_u64().unwrap() as usize] = b' ';
    encoded[competing["paddingOffset"].as_u64().unwrap() as usize] = b'=';
    let (result, requests) = semio_framework_trace_memory_testing::observe(|| base64_standard_decode_controlled(&encoded, &mut control));
    assert_eq!(result, Err(Base64ControlError::Codec(Base64Error::InvalidPadding)));
    assert_eq!(requests, 0);
    let mut denied = |_| false;
    let mut control = Base64Control { maximum_output_bytes: 0, progress: &mut denied };
    assert_eq!(base64_standard_decode_controlled(b"Zh==", &mut control), Err(Base64ControlError::OutputLimit));
    assert_eq!(base64_standard_decode_controlled(b"Zg", &mut control), Err(Base64ControlError::Codec(Base64Error::InvalidLength)));
    let mut control = Base64Control { maximum_output_bytes: 3, progress: &mut denied };
    assert_eq!(base64_standard_decode_controlled(b"Zh==", &mut control), Err(Base64ControlError::Cancelled));
}

/// 🪜️ Exact work demand makes zero or undersized grants inert and preserves terminal-only ownership.
#[test]
fn retained_decode_uses_exact_work_grants_and_original_input_owner() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    let engine = base64::engine::general_purpose::STANDARD;
    for length in corpus["lengths"].as_array().unwrap() {
        let length = length.as_u64().unwrap() as usize;
        let bytes = controlled_bytes(length, &corpus);
        let source = engine.encode(&bytes);
        let pointer = source.as_ptr();
        let (mut cursor, requests) = semio_framework_trace_memory_testing::observe(|| Base64DecodeCursor::new(Base64Input::OwnedText(source)));
        assert_eq!(requests, 0);
        assert_eq!(cursor.output_capacity(), 0);
        assert!(cursor.take_output().is_none());
        let callbacks = std::cell::Cell::new(0);
        let mut accepted = |_| { callbacks.set(callbacks.get() + 1); true };
        let mut control = Base64Control { maximum_output_bytes: length, progress: &mut accepted };
        let mut turns = 0;
        while !cursor.is_complete() {
            let demand = cursor.next_work_demand(8192);
            let before = cursor.progress();
            assert!(demand > 0 && demand <= 8192);
            for grant in corpus["stepGrants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).filter(|grant| *grant < demand) {
                if cursor.next_work_demand(grant)!=0 { continue; }
                let prior_callbacks = callbacks.get();
                let (state, requested) = semio_framework_trace_memory_testing::observe(|| cursor.step(grant, &mut control));
                assert_eq!(state.unwrap(), Base64DecodeState::Pending);
                assert_eq!(cursor.progress(), before);
                assert_eq!(requested, 0);
                assert_eq!(callbacks.get(), prior_callbacks);
            }
            let prior_capacity=cursor.output_capacity();
            let (_,requested)=semio_framework_trace_memory_testing::observe(||cursor.step(demand, &mut control).unwrap());
            assert_eq!(cursor.output_capacity(),prior_capacity+requested);
            assert!(cursor.output_capacity()==0||cursor.output_capacity()==length);
            turns += 1;
            assert!(turns < 100000);
            if !cursor.is_complete() { assert!(cursor.take_output().is_none()); }
        }
        let output = cursor.take_output().unwrap();
        assert_eq!(cursor.output_capacity(),0);
        assert_eq!(output, bytes);
        assert_eq!(output, engine.decode(match cursor.input() { Base64Input::OwnedText(text) => text, _ => unreachable!() }).unwrap());
        assert!(cursor.take_output().is_none());
        let parts = cursor.into_parts();
        assert_eq!(parts.outcome, Base64DecodeOutcome::Complete);
        assert!(parts.output.is_none());
        match parts.input { Base64Input::OwnedText(text) => assert_eq!(text.as_ptr(), pointer), _ => unreachable!() }
        let text = engine.encode(&bytes);
        let mut borrowed = Base64DecodeCursor::new(Base64Input::Borrowed(text.as_bytes()));
        while !borrowed.is_complete() { borrowed.step(borrowed.next_work_demand(8192), &mut control).unwrap(); }
        assert_eq!(borrowed.take_output().unwrap(), bytes);
        assert_eq!(match borrowed.into_parts().input { Base64Input::Borrowed(input) => input.as_ptr(), _ => unreachable!() }, text.as_ptr());
    }
}

/// 🧷️ Payload selection preserves the original allocation and excludes wrapper storage from progress.
#[test]
fn retained_input_ranges_preserve_backing_and_span_progress() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    let engine = base64::engine::general_purpose::STANDARD;
    for length in corpus["lengths"].as_array().unwrap() {
        let length = length.as_u64().unwrap() as usize;
        let bytes = controlled_bytes(length, &corpus);
        let encoded = engine.encode(&bytes);
        for row in corpus["inputRanges"].as_array().unwrap() {
            let text = format!("{}{}{}", row["prefix"].as_str().unwrap(), encoded, row["suffix"].as_str().unwrap());
            let pointer = text.as_ptr();
            let capacity = text.capacity();
            let range = Base64InputRange { start: row["utf8Start"].as_u64().unwrap() as usize, end: row["utf8Start"].as_u64().unwrap() as usize + encoded.len() };
            let (mut cursor, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| Base64DecodeCursor::new_range(Base64Input::OwnedText(text), range));
            assert_eq!((requests, releases), (0, 0));
            let mut progress = |event: Base64Progress| { assert_eq!(event.total, encoded.len()); assert!(event.completed <= event.total); true };
            let mut control = Base64Control { maximum_output_bytes: length, progress: &mut progress };
            let (_, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| {
                while !cursor.is_complete() { cursor.step(cursor.next_work_demand(8192), &mut control).unwrap(); }
            });
            assert_eq!((requests, releases), (length, 0));
            assert_eq!(cursor.take_output().unwrap(), bytes);
            let (parts, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| cursor.into_parts());
            assert_eq!((requests, releases), (0, 0));
            assert_eq!(parts.range, range);
            assert_eq!(parts.outcome, Base64DecodeOutcome::Complete);
            match parts.input {
                Base64Input::OwnedText(text) => { assert_eq!(text.as_ptr(), pointer); assert_eq!(text.capacity(), capacity); assert_eq!(engine.decode(&text.as_bytes()[range.start..range.end]).unwrap(), bytes); }
                _ => unreachable!(),
            }
        }
    }
    for row in corpus["invalidInputRanges"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap().to_owned();
        let pointer = input.as_ptr();
        let range = Base64InputRange { start: row["utf8"]["start"].as_u64().unwrap() as usize, end: row["utf8"]["end"].as_u64().unwrap() as usize };
        let (mut cursor, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| Base64DecodeCursor::new_range(Base64Input::OwnedText(input), range));
        assert_eq!((requests, releases), (0, 0));
        let mut progress = |_| panic!("invalid range reached progress");
        assert_eq!(cursor.step(100, &mut Base64Control { maximum_output_bytes: 100, progress: &mut progress }), Err(Base64ControlError::InputRange));
        let parts = cursor.into_parts();
        assert_eq!(parts.range, range);
        assert!(parts.output.is_none());
        assert_eq!(parts.outcome, Base64DecodeOutcome::Refused(Base64ControlError::InputRange));
        match parts.input { Base64Input::OwnedText(input) => assert_eq!(input.as_ptr(), pointer), _ => unreachable!() }
    }
    let bytes = controlled_bytes(corpus["cancellationInputLength"].as_u64().unwrap() as usize, &corpus);
    let text = format!("pk:{}:end", engine.encode(&bytes));
    let pointer = text.as_ptr();
    let range = Base64InputRange { start: 3, end: text.len() - 4 };
    let mut cursor = Base64DecodeCursor::new_range(Base64Input::OwnedText(text), range);
    let mut progress = |event: Base64Progress| event.phase != Base64Phase::Decode || event.completed != 4096;
    let mut control = Base64Control { maximum_output_bytes: bytes.len(), progress: &mut progress };
    let (result, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| {
        loop { match cursor.step(cursor.next_work_demand(8192), &mut control) { Ok(Base64DecodeState::Pending) => {}, result => break result } }
    });
    assert_eq!(result, Err(Base64ControlError::Cancelled));
    assert_eq!((requests, releases), (bytes.len(), 0));
    let parts = cursor.into_parts();
    assert_eq!(parts.range, range);
    assert_eq!(parts.output.unwrap(), bytes[..3072]);
    match parts.input { Base64Input::OwnedText(text) => assert_eq!(text.as_ptr(), pointer), _ => unreachable!() }
    for input in [Base64Input::Borrowed(b"pk:TWFu:end"), Base64Input::OwnedBytes(b"pk:TWFu:end".to_vec())] {
        let range = Base64InputRange { start: 3, end: 7 };
        let mut cursor = Base64DecodeCursor::new_range(input, range);
        let mut progress = |_| true;
        let mut control = Base64Control { maximum_output_bytes: 3, progress: &mut progress };
        while !cursor.is_complete() { cursor.step(cursor.next_work_demand(8192), &mut control).unwrap(); }
        assert_eq!(cursor.take_output().unwrap(), b"Man");
        assert_eq!(cursor.into_parts().range, range);
    }
}

/// 🫴️ Cancellation retains the first cause and transfers original partial storage without a clone or release.
#[test]
fn retained_decode_refusal_transfers_exact_original_input_and_partial_output() {
    use base64::Engine as _;
    let corpus = controlled_corpus();
    let bytes = controlled_bytes(corpus["cancellationInputLength"].as_u64().unwrap() as usize, &corpus);
    let source = base64::engine::general_purpose::STANDARD.encode(&bytes).into_bytes();
    let source_pointer = source.as_ptr();
    let mut cursor = Base64DecodeCursor::new(Base64Input::OwnedBytes(source));
    let canceled = std::cell::Cell::new(false);
    let mut callback = |event: Base64Progress| {
        let accepted = event.phase != Base64Phase::Decode || event.completed != 4096;
        if !accepted { canceled.set(true); }
        accepted
    };
    let mut control = Base64Control { maximum_output_bytes: bytes.len(), progress: &mut callback };
    let (result, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| {
        loop { match cursor.step(cursor.next_work_demand(8192), &mut control) { Ok(Base64DecodeState::Pending) => {}, result => break result } }
    });
    assert_eq!(result, Err(Base64ControlError::Cancelled));
    assert!(canceled.get());
    assert_eq!(requests, bytes.len());
    assert_eq!(releases, 0);
    assert_eq!(cursor.output_capacity(),bytes.len());
    assert!(cursor.take_output().is_none());
    control.maximum_output_bytes = 0;
    assert_eq!(cursor.step(8192, &mut control), Err(Base64ControlError::Cancelled));
    let (parts, requests, releases) = semio_framework_trace_memory_testing::observe_backing(|| cursor.into_parts());
    assert_eq!((requests, releases), (0, 0));
    assert_eq!(parts.outcome, Base64DecodeOutcome::Refused(Base64ControlError::Cancelled));
    let output = parts.output.unwrap();
    assert_eq!(output.len(), 3072);
    assert_eq!(output.capacity(), bytes.len());
    assert_eq!(output, bytes[..3072]);
    match parts.input { Base64Input::OwnedBytes(source) => assert_eq!(source.as_ptr(), source_pointer), _ => unreachable!() }
}
