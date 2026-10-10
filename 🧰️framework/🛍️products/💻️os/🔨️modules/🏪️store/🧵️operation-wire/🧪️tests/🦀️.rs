//! 🧪️ Exact serde operation wire and same-thread allocator witnesses across admitted copy and cancellation turns.
use super::*;
use super::super::{ArtifactCanonicalJsonNode, ArtifactCanonicalJson};

struct OracleSource(serde_json::Value);
impl OracleSource {
    fn value(&self, path: &[usize]) -> Result<&serde_json::Value, String> {
        let mut value = &self.0;
        for index in path {
            value = match value {
                serde_json::Value::Array(values) => values.get(*index),
                serde_json::Value::Object(values) => values.values().nth(*index),
                _ => None,
            }.ok_or_else(|| "operation-wire.invalid-fixture-path".to_string())?;
        }
        Ok(value)
    }
}

struct TextOracle(serde_json::Value);
impl TextOracle {
    fn node(&self, path: &[usize]) -> &serde_json::Value {
        let mut node = &self.0;
        for index in path { node = &node["children"][*index]; }
        node
    }
    fn independent(node: &serde_json::Value, output: &mut Vec<u8>) {
        match node["kind"].as_str().unwrap() {
            "sequence" => {
                output.extend_from_slice(node["open"].as_str().unwrap().as_bytes());
                for (index, child) in node["children"].as_array().unwrap().iter().enumerate() {
                    if index > 0 { output.extend_from_slice(node["separator"].as_str().unwrap().as_bytes()); }
                    Self::independent(child, output);
                }
                output.extend_from_slice(node["close"].as_str().unwrap().as_bytes());
            }
            "hex" => for byte in node["text"].as_str().unwrap().as_bytes() { output.extend_from_slice(format!("{byte:02x}").as_bytes()); },
            _ => output.extend_from_slice(node["text"].as_str().unwrap().as_bytes()),
        }
    }
}
impl ArtifactOperationText for TextOracle {
    fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, ValueError> {
        let node = self.node(path);
        Ok(match node["kind"].as_str().unwrap() {
            "sequence" => ArtifactOperationTextNode::Sequence { length: node["children"].as_array().unwrap().len(), open: node["open"].as_str().unwrap().as_bytes(), separator: node["separator"].as_str().unwrap().as_bytes(), close: node["close"].as_str().unwrap().as_bytes() },
            "hex" => ArtifactOperationTextNode::Hex(node["text"].as_str().unwrap().as_bytes()),
            "scalar" => ArtifactOperationTextNode::Scalar,
            _ => ArtifactOperationTextNode::Bytes(node["text"].as_str().unwrap().as_bytes()),
        })
    }
    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), ArtifactPreparedOperationError> {
        let bytes = self.node(path)["text"].as_str().unwrap().as_bytes();
        let count = output.len().min(bytes.len() - offset);
        output[..count].copy_from_slice(&bytes[offset..offset + count]);
        Ok((count, offset + count == bytes.len()))
    }
}

#[test]
fn prepared_operation_wire_borrowed_text_matches_neutral_ordered_tuple_and_hex() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (index, row) in fixture["textCases"].as_array().unwrap().iter().enumerate() {
        let header: &'static [u8] = if index == 0 { &[1, 5] } else { &[1, 6] };
        let source = TextOracle(row["node"].clone());
        let mut expected = header.to_vec();
        TextOracle::independent(&source.0, &mut expected);
        for maximum in [1, 3, 7, 64, 256] {
            let (mut cursor, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactPreparedOperationCursor::default);
            assert_eq!((birth.requested_bytes, birth.released_bytes), (0, 0));
            let mut actual = Vec::with_capacity(expected.len());
            for _ in 0..expected.len() * 4 + 64 {
                let mut output = [0; 256];
                let wire = || ArtifactPreparedOperationSource::Text { header, body: &source };
                let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire(), &mut output, RetainedCloneGrant::default()).unwrap());
                assert_eq!(denied, ArtifactPreparedOperationProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire(), &mut output, RetainedCloneGrant::one_payload_turn(maximum, 64)).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(progress.processed_items, 1);
                assert!(progress.written_bytes <= maximum.min(64));
                actual.extend_from_slice(&output[..progress.written_bytes]);
                if progress.complete { break; }
            }
            assert_eq!(actual, expected);
        }
        for stop in fixture["cancelStops"].as_array().unwrap() {
            let mut cursor = ArtifactPreparedOperationCursor::default();
            for _ in 0..stop.as_u64().unwrap() { cursor.advance(ArtifactPreparedOperationSource::Text { header, body: &source }, &mut [0; 64], RetainedCloneGrant::one_payload_turn(64, 64)).unwrap(); }
            let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert!(progress.complete);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        println!("[DEBUG] prepared operation text row={index} exact-independent-bytes={} one structural event or copy<=64 heap0 five cancellation stops", expected.len());
    }
}
impl ArtifactCanonicalJson for OracleSource {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match self.value(path).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"operation-wire.invalid-fixture-path"))? {
            serde_json::Value::Null => N::Null,
            serde_json::Value::Bool(value) => N::Bool(*value),
            serde_json::Value::Number(value) => if let Some(value) = value.as_i64() { N::I64(value) } else if let Some(value) = value.as_u64() { N::U64(value) } else { N::F64(value.as_f64().unwrap()) },
            serde_json::Value::String(value) => N::String(value),
            serde_json::Value::Array(values) => N::Array(values.len()),
            serde_json::Value::Object(values) => N::Object(values.len()),
        })
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<crate::os_store::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        self.value(path).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"operation-wire.invalid-fixture-path"))?.as_object().and_then(|values| values.keys().nth(index)).map(|key| key.as_str().into()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "operation-wire.invalid-fixture-key"))
    }
}

fn original_json_policy(fixture: &serde_json::Value) -> RetainedCloneGrant {
    let row = &fixture["jsonTurnPolicy"];
    RetainedCloneGrant { maximum_items: row["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: row["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: row["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: row["maximumDepth"].as_u64().unwrap() as usize }
}

#[test]
fn prepared_operation_wire_borrowed_json_matches_serde_without_birth_or_owner_copy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (row_index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let header: &'static [u8] = if row_index == 2 { &[1, 128, 1] } else { &[1, 6] };
        let mut body = row["body"].clone();
        if row_index == 0 { body["value"] = serde_json::Value::String("δ😀\0\n\"\\".repeat(512)); }
        let source = OracleSource(body);
        for hex in [false, true] {
        let mut expected = header.to_vec();
        let raw = serde_json::to_vec(&source.0).unwrap();
        if hex { expected.extend_from_slice(b"snapshot="); for byte in raw { expected.push(b"0123456789abcdef"[usize::from(byte >> 4)]); expected.push(b"0123456789abcdef"[usize::from(byte & 15)]); } } else { expected.extend(raw); }
        let wire_source = || if hex { ArtifactPreparedOperationSource::HexJson { header, prefix: b"snapshot=", body: &source } } else { ArtifactPreparedOperationSource::CanonicalJson { header, body: &source } };
        for maximum in [1, 3, 7, 64, 256] {
            let (mut cursor, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactPreparedOperationCursor::default);
            assert_eq!((birth.requested_bytes, birth.released_bytes), (0, 0));
            let mut actual = Vec::with_capacity(expected.len());
            for _ in 0..expected.len() * 2 + 8 {
                let mut output = [0; 256];
                for grant in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: maximum, ..Default::default() }] {
                    let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire_source(), &mut output, grant).unwrap());
                    assert_eq!(progress, ArtifactPreparedOperationProgress::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                }
                let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.advance(wire_source(), &mut output[..maximum.min(64)], original_json_policy(&fixture)).unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert!(progress.processed_items <= 1);
                assert!(progress.written_bytes <= maximum.min(64));
                actual.extend_from_slice(&output[..progress.written_bytes]);
                if progress.complete { break; }
            }
            assert_eq!(actual, expected);
            if !hex { assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual[header.len()..]).unwrap(), source.0); }
            assert_eq!(cursor.next_minimum_copy_bytes(), 0);
            assert_eq!((cursor.next_capacity_byte_demand().unwrap(), cursor.next_close_byte_demand().unwrap()), (0, 0));
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        for stop in fixture["cancelStops"].as_array().unwrap() {
            let mut cursor = ArtifactPreparedOperationCursor::default();
            for _ in 0..stop.as_u64().unwrap() { cursor.advance(wire_source(), &mut [0; 64], original_json_policy(&fixture)).unwrap(); }
            let before = &source.0 as *const serde_json::Value;
            let (progress, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
            assert!(progress.complete);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(&source.0 as *const serde_json::Value, before);
        }
        println!("[DEBUG] prepared operation wire row={row_index} hex={hex} exact-serde-bytes={} copy<=64 no births/frees all five cancellation stops preserve original typed owner", expected.len());
        }
    }
}

#[test]
fn prepared_operation_output_preserves_every_original_prefix_and_physical_cancel_cut(){
    use semio_framework_value::{ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
    let rows:serde_json::Value=serde_json::from_str(include_str!("../📦️output/🧫️fixtures/🔣️.json")).unwrap();
    let p=&rows["grant"];let policy=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize};
    let maximum=rows["limits"]["maximumPayloadBytes"].as_u64().unwrap()as usize;let capacity=rows["limits"]["maximumCapacityBytes"].as_u64().unwrap()as usize;
    for row in rows["examples"].as_array().unwrap(){
        let source=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap_or(1)as usize);let original=source.as_ptr();
        for cut in rows["cancelStops"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).chain(std::iter::once(10000)){
            let(mut owner,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactPreparedOperationOutput::empty(maximum,capacity).unwrap());assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));
            let mut offset=0;let mut retained=0;
            for _ in 0..cut.min(10000){
                if offset==source.len(){break;}
                let end=(offset+64).min(source.len());
                let(zero,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.append(&source.as_bytes()[offset..end],RetainedCloneGrant{maximum_items:0,..policy}).unwrap());assert_eq!(zero,RetainedCloneProgress::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
                let(progress,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.append(&source.as_bytes()[offset..end],policy).unwrap());assert!(progress.fits(policy));assert_eq!(event.requested_bytes,progress.retained_capacity_bytes);assert_eq!(event.released_bytes,0);retained+=event.requested_bytes;offset+=progress.copied_bytes;assert_eq!(owner.len(),offset);for index in 0..offset{assert_eq!(owner.byte_at(index),Some(source.as_bytes()[index]));}
            }
            assert_eq!(source.as_ptr(),original);assert_eq!(owner.allocated_bytes(),retained);
            let mut released=0;owner.begin_close();
            for _ in 0..10000{
                let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(policy).unwrap());let progress=step.progress();assert!(progress.fits(policy));assert_eq!(event.requested_bytes,progress.retained_capacity_bytes);assert_eq!(event.released_bytes,progress.released_bytes);released+=event.released_bytes;if owner.terminal_is_empty(){break;}
            }
            assert!(owner.terminal_is_empty());assert_eq!(released,retained);
        }
    }
    eprintln!("[DEBUG] original operation output fixed five-axis policy preserves UTF8/page prefixes and every System backing across cancellation cuts");
}

#[test]
fn original_text_failure_preserves_initialized_prefix_and_real_system_receipt() {
    struct OriginalScalar(&'static [u8]);
    impl ArtifactOperationText for OriginalScalar {
        fn operation_text_node(&self,_:&[usize])->Result<ArtifactOperationTextNode<'_>,ValueError>{Ok(ArtifactOperationTextNode::Scalar)}
        fn operation_text_scalar(&self,_:&[usize],_:usize,output:&mut[u8])->Result<(usize,bool),ArtifactPreparedOperationError>{
            output[..2].copy_from_slice(&self.0[..2]);
            Err(ArtifactPreparedOperationError{written_bytes:2,reason:ValueError::literal(ValueRefusalKind::InvariantViolated,"original scalar refusal")})
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["textFailure"];let p=&row["policy"];
    let grant=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize};
    let original=OriginalScalar("Tür🚪".as_bytes());let pointer=original.0.as_ptr();let mut cursor=ArtifactPreparedOperationCursor::default();let mut output=[0xa5;64];
    let (first,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(ArtifactPreparedOperationSource::Text{header:&[1,5],body:&original},&mut output,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((first.written_bytes,first.copied_bytes),(2,2));let header=[output[0],output[1]];
    output.fill(0xa5);let (failure,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(ArtifactPreparedOperationSource::Text{header:&[1,5],body:&original},&mut output,grant).unwrap_err());
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(failure.written_bytes,2);assert_eq!(failure.reason.retained_progress().copied_items,1);assert_eq!(failure.reason.retained_progress().copied_bytes,2);assert!(failure.reason.retained_progress().fits(grant));assert_eq!(&output[2..],&[0xa5;62]);assert_eq!(original.0.as_ptr(),pointer);
    let expected:Vec<u8>=row["expectedPrefix"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();assert_eq!([&header[..],&output[..failure.written_bytes]].concat(),expected);assert_eq!(first.copied_bytes+failure.reason.retained_progress().copied_bytes,row["expectedCopiedBytes"].as_u64().unwrap()as usize);
    let (closed,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close(grant).unwrap());assert!(closed.complete);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] original text failure canonical prefix2 cumulative4 System0 original identity exact retained receipt");
}

#[test]
fn original_text_frontier_uses_supplied_depth_and_zero_system_birth() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let policy=&fixture["textFailure"]["policy"];
 for row in fixture["textDepthRows"].as_array().unwrap(){
  let source=TextOracle(row["node"].clone());let pointer=&source.0 as *const _;let mut actual=Vec::new();let(mut cursor,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(ArtifactPreparedOperationCursor::default);assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));
  let grant=RetainedCloneGrant{maximum_items:policy["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};let mut refused=false;let mut complete=false;
  for _ in 0..128{
   let mut output=[0xa5;64];let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(ArtifactPreparedOperationSource::Text{header:&[],body:&source},&mut output,grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   match result{Ok(step)=>{assert!(step.written_bytes<=grant.maximum_copy_bytes);assert_eq!(step.copied_bytes,step.written_bytes);assert!(output[step.written_bytes..].iter().all(|v|*v==0xa5));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{complete=true;break;}},Err(error)=>{assert_eq!(error.reason.kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert!(error.reason.retained_progress().fits(grant));assert_eq!(error.written_bytes,0);assert!(output.iter().all(|v|*v==0xa5));refused=true;break;}}
  }
  if refused{let mut output=[0xa5;64];let(failure,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(ArtifactPreparedOperationSource::Text{header:&[],body:&source},&mut output,grant).unwrap_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(failure.written_bytes,0);assert_eq!(failure.reason.retained_progress(),Default::default());assert!(output.iter().all(|v|*v==0xa5));}
  assert_eq!(&source.0 as *const _,pointer);if row["refused"].as_bool()==Some(true){assert!(refused&&!complete);assert_eq!(actual,row["expectedPrefix"].as_str().unwrap().as_bytes());}else{assert!(complete&&!refused);assert_eq!(actual,row["expectedText"].as_str().unwrap().as_bytes());}
  let(closed,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close(grant).unwrap());assert!(closed.complete);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
 eprintln!("[DEBUG] original Text supplied parent2/3 immutable copy policy preserves original identity, zero System birth and refused child prefix");
}

#[test]
fn original_operation_contiguous_handoff_conserves_supplied_birth_copy_and_cancel_receipts(){
 use semio_framework_value::{ErasedSnapshotRetirement,retained_clone::RetainedCloneGrant};
 let rows:serde_json::Value=serde_json::from_str(include_str!("../📦️output/🧫️fixtures/🔣️.json")).unwrap();let p=&rows["grant"];
 let policy=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize};
 let maximum=rows["limits"]["maximumPayloadBytes"].as_u64().unwrap()as usize;let capacity=rows["limits"]["maximumCapacityBytes"].as_u64().unwrap()as usize;
 for row in rows["examples"].as_array().unwrap(){let original=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap_or(1)as usize);let pointer=original.as_ptr();
  for cut in rows["cancelStops"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize).chain(std::iter::once(12000)){
   let mut owner=ArtifactPreparedOperationOutput::empty(maximum,capacity).unwrap();let mut live=0;let mut offset=0;
   for _ in 0..10000{if offset==original.len(){break;}let end=(offset+64).min(original.len());let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.append(&original.as_bytes()[offset..end],policy).unwrap());assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;offset+=progress.copied_bytes;}
   assert_eq!(offset,original.len());let mut transferred=None;
   for _ in 0..cut{
    let(prefix,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.contiguous_prefix().map(|value|(value.as_ptr(),value.len())));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.prepare_contiguous(RetainedCloneGrant{maximum_items:0,..policy}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.contiguous_prefix().map(|value|(value.as_ptr(),value.len())),prefix);
    if owner.contiguous_prefix().is_none()&&!original.is_empty(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.prepare_contiguous(RetainedCloneGrant{maximum_capacity_bytes:original.len()-1,..policy}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(owner.contiguous_prefix().is_none());assert_eq!(owner.len(),original.len());}
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.prepare_contiguous(policy).unwrap());let progress=step.progress();assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;
    if let Some(prefix)=owner.contiguous_prefix(){assert_eq!(prefix,&original.as_bytes()[..prefix.len()]);}
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_contiguous(policy).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some((bytes,progress))=result{assert!(progress.fits(policy));assert_eq!(bytes.as_slice(),original.as_bytes());transferred=Some(bytes);break;}
   }
   assert_eq!(original.as_ptr(),pointer);owner.begin_close();for _ in 0..12000{if owner.terminal_is_empty(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(heap.released_bytes,step.progress().released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;}
   assert!(owner.terminal_is_empty());if let Some(bytes)=transferred{assert!(bytes.capacity()<=policy.maximum_release_bytes);assert!(policy.maximum_items>0&&policy.maximum_depth>0);let capacity=bytes.capacity();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,capacity);live-=heap.released_bytes;}assert_eq!(live,0);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }
 eprintln!("[DEBUG] original protocol contiguous octets3 immutable supplied policy, denied original page/prefix identity, every cancellation cut and exact System birth/copy/release conservation");
}

#[test]
fn original_operation_rows_preserve_each_buffer_and_paid_collection_cancel_cut(){
 use semio_framework_value::{ErasedSnapshotRetirement,retained_clone::RetainedCloneGrant};
 let rows:serde_json::Value=serde_json::from_str(include_str!("../📦️output/🧫️fixtures/🔣️.json")).unwrap();let p=&rows["grant"];
 let policy=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize};
 let maximum=rows["maximumOperations"].as_u64().unwrap()as usize;let capacity=rows["limits"]["maximumCapacityBytes"].as_u64().unwrap()as usize;
 for example in rows["operationRows"].as_array().unwrap(){let expected:Vec<Vec<u8>>=serde_json::from_value(example["expected"].clone()).unwrap();
  for cut in (0..24).chain(std::iter::once(200)){
   let mut owner=ArtifactPreparedOperationsOwner::empty(maximum,capacity).unwrap();let mut live=0;let mut pointers=Vec::new();
   for value in example["values"].as_array().unwrap(){let original_values:Vec<u8>=serde_json::from_value(value.clone()).unwrap();let(original,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_values.as_slice().to_vec());live+=heap.requested_bytes;live-=heap.released_bytes;let pointer=original.as_ptr();pointers.push(pointer);
    let((returned,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.append_original(original,RetainedCloneGrant{maximum_items:0,..policy}).unwrap());assert_eq!(progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut pending=returned;
    for _ in 0..20{let Some(original)=pending.take()else{break};assert_eq!(original.as_ptr(),pointer);let((returned,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.append_original(original,policy).unwrap());assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;pending=returned;}
    assert!(pending.is_none());assert_eq!(owner.row_at(owner.len()-1).unwrap().as_ptr(),pointer);
   }
   let mut transferred=None;
   for _ in 0..cut{
    let prefix=owner.prepared_prefix().map(|values|(values.as_ptr(),values.len()));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.prepare_handoff(RetainedCloneGrant{maximum_items:0,..policy}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.prepared_prefix().map(|values|(values.as_ptr(),values.len())),prefix);
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.prepare_handoff(policy).unwrap());let progress=step.progress();assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;
    if let Some(prefix)=owner.prepared_prefix(){for(index,value)in prefix.iter().enumerate(){assert_eq!(value,&expected[index]);assert_eq!(value.as_ptr(),pointers[index]);}}
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_prepared(policy).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some((values,progress))=result{assert!(progress.fits(policy));assert_eq!(values,expected);for(index,value)in values.iter().enumerate(){assert_eq!(value.as_ptr(),pointers[index]);}transferred=Some(values);break;}
   }
   owner.begin_close();for _ in 0..200{if owner.terminal_is_empty(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(policy).unwrap());let progress=step.progress();assert!(progress.fits(policy));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);live+=heap.requested_bytes;live-=heap.released_bytes;}
   assert!(owner.terminal_is_empty());if let Some(mut values)=transferred{for value in &mut values{let capacity=value.capacity();assert!(capacity<=policy.maximum_release_bytes&&policy.maximum_copy_bytes>=std::mem::size_of::<Vec<u8>>()*2);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(std::mem::take(value)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));live-=heap.released_bytes;}let capacity=values.capacity()*std::mem::size_of::<Vec<u8>>();assert!(capacity<=policy.maximum_release_bytes&&policy.maximum_copy_bytes>=std::mem::size_of::<Vec<Vec<u8>>>());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(values));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));live-=heap.released_bytes;}
   assert_eq!(live,0);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
 }
 eprintln!("[DEBUG] original operation rows2 fixed policy preserves exact buffers across50 paid protocol and cancellation cuts with System conservation");
}
