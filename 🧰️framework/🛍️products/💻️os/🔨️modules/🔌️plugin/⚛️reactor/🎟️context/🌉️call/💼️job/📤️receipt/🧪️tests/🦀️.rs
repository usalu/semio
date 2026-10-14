use super::*;

fn bytes(hex: &str) -> std::vec::Vec<u8> { hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect() }

/// 💼️ Real fixed job frames preserve all current variants and original actor currencies with no heap activity.
#[cfg_attr(test,test)]
pub fn original_owned_job_header_matches_portable_oracle_without_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let actor: serde_json::Value = serde_json::from_str(include_str!("../../../📤️receipt/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let base = bytes(actor["failureHex"].as_str().unwrap());
    let original = OriginalActorFixedReceipt::read(&base).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut expected = [0u8; 184];
        expected[..8].copy_from_slice(&bytes(row["prefixHex"].as_str().unwrap()));
        expected[8..144].copy_from_slice(&base);
        let mut body = [0u64; 5];
        for (index, value) in row["body"].as_array().unwrap().iter().enumerate() { body[index] = value.as_u64().unwrap(); expected[144 + index * 8..152 + index * 8].copy_from_slice(&body[index].to_le_bytes()); }
        let refusal = match row["refusal"].as_str().unwrap() { "none" => None, "semantic" => Some(OriginalJobCallFailure::Semantic), "ownershipLimit" => { expected[14] = 3; Some(OriginalJobCallFailure::Authority(ValueRefusalKind::OwnershipLimit)) }, _ => unreachable!() };
        let status = match row["kind"].as_str().unwrap() {
            "start" => OriginalOwnedJobStatus::Start { admitted: false, returned_source: true },
            "step" => OriginalOwnedJobStatus::Step { outcome: Some(OriginalJobOutcome { job: body[0], operation: body[1], generation: body[2], state: OriginalJobOutputState::Done, byte_length: body[4] }), source_closed: false },
            "output" => OriginalOwnedJobStatus::Output { chunk: Some(OriginalOwnedJobChunk { offset: body[0], byte_length: Some(body[1]), output_closed: true }), source_closed: false },
            "acknowledge" => OriginalOwnedJobStatus::Acknowledge { acknowledged: true, source_closed: true },
            "cancel" => OriginalOwnedJobStatus::Cancel { admitted: false },
            "close" => OriginalOwnedJobStatus::Close { admitted: false },
            _ => unreachable!(),
        };
        let call = OriginalOwnedJobCall { input: original.input, spent: original.spent, work_units_spent: original.work_units_spent, status, refusal };
        let (mut receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(OriginalOwnedJobFixedReceipt::empty);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| receipt.write(call));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(receipt.bytes(), &expected);
        let (decoded, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| OriginalOwnedJobFixedReceipt::read(receipt.bytes()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let decoded = decoded.unwrap();
        assert_eq!(decoded.input, original.input);
        assert_eq!(decoded.spent, original.spent);
        assert_eq!(decoded.work_units_spent, original.work_units_spent);
        receipt.write(decoded);
        assert_eq!(receipt.bytes(), &expected);
        for (index, value) in [(0, 0), (4, 7), (5, 16), (6, 3), (7, 1), (8, 0)] {
            let mut invalid = expected;
            invalid[index] = value;
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| OriginalOwnedJobFixedReceipt::read(&invalid));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(matches!(result, Err(error) if error.kind == ValueRefusalKind::InvalidValue));
        }
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| OriginalOwnedJobFixedReceipt::read(&expected[..183]));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(result.is_err());
    }
    println!("[DEBUG] Original owned job fixed184 six physical frames and42 refused headers allocate/release0, preserve all original currencies");
}

/// 🔒️ Actual resident metadata cannot be overwritten or acknowledge a different physical loan.
#[cfg_attr(test,test)]
pub fn original_owned_job_metadata_loan_denies_overwrite_before_body_without_heap(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../📤️receipt/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let original=OriginalActorFixedReceipt::read(&bytes(fixture["failureHex"].as_str().unwrap())).unwrap();let call=OriginalOwnedJobCall{input:original.input,spent:original.spent,work_units_spent:original.work_units_spent,status:OriginalOwnedJobStatus::Close{admitted:false},refusal:None};
 let(mut loan,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(OriginalOwnedJobReceiptLoan::empty);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(pointer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||loan.produce(||call).unwrap().as_ptr()as usize);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut entered=false;
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||loan.produce(||{entered=true;call}).map(|_|()));assert!(result.is_err());assert!(!entered);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let handbacks:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in handbacks["loanHandbacks"].as_array().unwrap(){let candidate=if row["pointer"].as_str().unwrap()=="original"{pointer}else{pointer+1};assert_eq!(loan.hand_back(candidate,row["extent"].as_u64().unwrap()as usize),row["accepted"].as_bool().unwrap());}assert!(!loan.hand_back(pointer,184));
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=loan.produce(||call).unwrap();let decoded=OriginalOwnedJobFixedReceipt::read(bytes).unwrap();assert_eq!(decoded.input,call.input);assert_eq!(decoded.spent,call.spent);assert_eq!(decoded.work_units_spent,call.work_units_spent)});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(loan.hand_back(pointer,184));println!("[DEBUG] Original owned metadata184 loan exactPointerExtent=true overwriteBeforeBody=false heap0/0 rawReceiptUnchanged=true bodyLoansUnaffected=true");
}
