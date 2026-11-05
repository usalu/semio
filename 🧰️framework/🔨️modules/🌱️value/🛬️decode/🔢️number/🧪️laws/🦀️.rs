//! 🧪️ Bounded original decimal normalization matches unbounded Rust and independent serde scalar oracles.
use super::*;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:128}}
fn bits(value:Option<f64>)->Option<u64>{value.map(f64::to_bits)}
fn observed<T>(run:impl FnOnce()->T)->T{let(output,heap)=crate::observe_retirement_allocations(run);assert_eq!(heap,(0,0),"a bounded float word must not allocate or release storage");output}
fn drive(word:&str,cancel:Option<u8>){
 let expected=word.parse::<f64>().ok();let pointer=word.as_ptr();let mut work=OriginalFloatWord::new();let g=grant();let mut output=None;
 for turn in 0..10000{
  if cancel==Some(work.phase()){break}
  let before=(work.phase(),work.offset());let rounding=work.rounding;let d=work.next_demands(word);
  for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:0,..g}]{
   let result=observed(||work.advance(word,denied));assert!(result.result.unwrap().is_none());assert_eq!(result.work,0);assert_eq!(work.step_progress(),RetainedCloneProgress::default());assert_eq!((work.phase(),work.offset()),before);assert_eq!(work.rounding,rounding);
  }
  let result=observed(||work.advance(word,g));let next=result.result.unwrap();assert!(result.work!=0);assert_eq!(word.as_ptr(),pointer);assert!(work.offset().saturating_sub(before.1)<=64);
  if let Some(value)=next{output=Some(value);break}assert!(turn+1<10000,"original float word did not complete");
 }
 if cancel.is_none(){assert_eq!(output.map(bits),Some(bits(expected)),"word length {}",word.len());let output=output.unwrap();work.restore_output(output);let restored=observed(||work.advance(word,g));assert_eq!(restored.result.unwrap().map(bits),Some(bits(expected)))}
 work.begin_close();for turn in 0..10{if work.terminal_is_empty(){break}let before=work.close_demands(4096).unwrap();let denied=observed(||work.close_step(RetainedCloneGrant{maximum_items:0,..g}));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(work.close_demands(4096).unwrap(),before);observed(||work.close_step(g)).result.unwrap();assert!(turn+1<10)}assert!(work.terminal_is_empty());
}
fn coefficient(numerator:u64,power:i32)->String{
 let mut digits=numerator.to_string().bytes().rev().map(|byte|byte-b'0').collect::<Vec<_>>();let factor=if power>=0{5}else{2};
 for _ in 0..power.unsigned_abs(){let mut carry=0u16;for digit in &mut digits{let n=u16::from(*digit)*factor+carry;*digit=(n%10)as u8;carry=n/10}while carry!=0{digits.push((carry%10)as u8);carry/=10}}
 digits.into_iter().rev().map(|digit|char::from(b'0'+digit)).collect()
}
fn predecessor(number:&str)->String{let mut bytes=number.as_bytes().to_vec();for index in (0..bytes.len()).rev(){if bytes[index]==b'0'{bytes[index]=b'9'}else{bytes[index]-=1;break}}String::from_utf8(bytes).unwrap()}
#[test]
fn original_float_word_matches_exact_midpoints_grammar_and_serde_without_heap_birth(){
 let f:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();assert_eq!(f["maximumSignificantDigits"],768);assert_eq!(f["maximumNormalizedBytes"],776);
 for item in f["simple"].as_array().unwrap(){let word=item["word"].as_str().unwrap();assert_eq!(word.parse::<f64>().is_ok(),item["valid"].as_bool().unwrap(),"{word:?}");drive(word,None)}
 for item in f["serdeFiniteWords"].as_array().unwrap(){let word=item.as_str().unwrap();let expected: f64=serde_json::from_str(word).unwrap();assert_eq!(word.parse::<f64>().unwrap().to_bits(),expected.to_bits());drive(word,None)}
 for item in f["midpoints"].as_array().unwrap(){let n=item["numerator"].as_str().unwrap().parse::<u64>().unwrap();let d=item["denominatorPower"].as_i64().unwrap()as i32;let c=coefficient(n,d);let exponent=if d>=0{-d}else{0};
  for count in f["tailZeroes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize){
   let zeroes="0".repeat(count);let words=[format!("{c}e{exponent}"),format!("{c}{zeroes}5e{}",exponent-i32::try_from(count).unwrap()-1),format!("{}{}e{}",predecessor(&c),"9".repeat(count+1),exponent-i32::try_from(count).unwrap()-1),format!("{zeroes}{c}e{exponent}"),format!("-{c}{zeroes}e{}",exponent-i32::try_from(count).unwrap())];
   for word in words{drive(&word,None)}
  }
 }
}
#[test]
fn original_float_word_cancel_and_changed_source_preserve_the_exact_field(){
 let word=format!("{}{}5e-{}",coefficient(1,1075),"0".repeat(4096),1075+4097);for phase in 0..=3{drive(&word,Some(phase))}
 let source="1.5".to_owned();let other=source.clone();let mut work=OriginalFloatWord::new();observed(||work.advance(&source,grant())).result.unwrap();let original=(work.phase(),work.offset());let failure=observed(||work.advance(&other,grant()));assert_eq!(failure.work,0);assert_eq!(failure.result.unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!((work.phase(),work.offset()),original);work.begin_close();observed(||work.close_step(grant())).result.unwrap();assert!(work.terminal_is_empty());
}

fn quantum_word(case:&serde_json::Value)->String{
 match case["kind"].as_str().unwrap(){
  "literal"=>case["word"].as_str().unwrap().to_owned(),
  "repeat"=>format!("{}{}{}",case["prefix"].as_str().unwrap(),case["repeat"].as_str().unwrap().repeat(case["count"].as_u64().unwrap()as usize),case["suffix"].as_str().unwrap()),
  "midpoint"=>{
   let n=case["numerator"].as_str().unwrap().parse().unwrap();let d=case["denominatorPower"].as_i64().unwrap()as i32;let c=coefficient(n,d);let e=if d>=0{-d}else{0};
   match case["variant"].as_str().unwrap(){"exact"=>format!("{c}e{e}"),"above"=>format!("{c}5e{}",e-1),"below"=>format!("{}9e{}",predecessor(&c),e-1),_=>panic!("unknown midpoint variant")}
  },
  _=>panic!("unknown quantum word"),
 }
}
fn close_quantum(work:&mut OriginalFloatWord){
 work.begin_close();let d=work.close_demands(4096).unwrap();let before=work.rounding;
 for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant()}]{
  let result=observed(||work.close_step(denied));assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),Default::default());assert_eq!(work.rounding,before);assert_eq!(work.close_demands(4096).unwrap(),d);
 }
 observed(||work.close_step(grant())).result.unwrap();assert!(work.terminal_is_empty());
}
#[test]
fn original_float_word_one_work_events_match_neutral_bits_within_the_declared_quantum(){
 let f:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();let q=&f["quantum"];let maximum=q["maximumEvents"].as_u64().unwrap()as usize;
 assert!(size_of::<OriginalFloatWord>()<=q["maximumCursorBytes"].as_u64().unwrap()as usize);
 for case in q["cases"].as_array().unwrap(){
  let word=quantum_word(case);let expected=case["expectedBits"].as_str().map(|text|u64::from_str_radix(text,16).unwrap());assert_eq!(bits(word.parse().ok()),expected,"{case}");
  let mut work=OriginalFloatWord::new();let mut output=None;
  for _ in 0..maximum{
   let quote=work.next_demands(&word);assert_eq!(work.next_work_demand(),q["maximumWorkPerEvent"].as_u64().unwrap());assert!(quote.copy_bytes<=grant().maximum_copy_bytes);
   let outcome=observed(||work.advance(&word,grant()));assert_eq!(outcome.work,1);assert!(work.step_progress().copied_bytes<=quote.copy_bytes);
   if let Some(value)=outcome.result.unwrap(){output=Some(bits(value));break}
  }
  assert_eq!(output,Some(expected),"one-work events exceeded {maximum}: {case}");close_quantum(&mut work);
 }
}
#[test]
fn original_float_word_partial_rounding_refusal_and_cancellation_preserve_custody(){
 let word=format!("{}e-1075",coefficient(1,1075));
 for frontier in 0..7{
  let mut work=OriginalFloatWord::new();let mut reached=false;
  for _ in 0..200{
   let c=work.rounding.control;
   let matches=work.phase()==2&&match frontier{
    0=>c.phase==RoundPhase::Prefix,1=>c.phase==RoundPhase::Lower,2=>c.phase==RoundPhase::Upper,3=>c.phase==RoundPhase::Seed,
    4=>c.phase==RoundPhase::Multiply&&c.index>0&&c.index<c.limit,
    5=>c.phase==RoundPhase::Multiply&&c.index==c.limit&&c.carry!=0,
    6=>c.phase==RoundPhase::Compare&&c.compared>0,_=>unreachable!(),
   };
   if matches{
    let before=work.rounding;let outer=(work.phase(),work.offset());let quote=work.next_demands(&word);
    for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:quote.copy_bytes-1,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{
     let result=observed(||work.advance(&word,denied));assert!(result.result.unwrap().is_none());assert_eq!(result.work,0);assert_eq!(work.step_progress(),Default::default());assert_eq!(work.rounding,before);assert_eq!((work.phase(),work.offset()),outer);
    }
    let other=word.clone();let refused=observed(||work.advance(&other,grant()));assert_eq!(refused.result.unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(refused.work,0);assert_eq!(work.step_progress(),Default::default());assert_eq!(work.rounding,before);assert_eq!((work.phase(),work.offset()),outer);
    reached=true;break
   }
   let outcome=observed(||work.advance(&word,grant()));assert!(outcome.result.unwrap().is_none(),"frontier {frontier} was not reached before output");
  }
  assert!(reached,"missing rounding frontier {frontier}");close_quantum(&mut work);
 }
}
