mod tests {
    use super::*;
    use semio_framework::{AbiMessage, AbiPageBytes, decode_abi_message};

    const OS_HOST_CODEC_LEDGER_FIXTURE: &str = include_str!("../../🧫️fixtures/📊️.tsv");

    #[derive(Default)]
    struct FixtureFormatResolver;

    impl OsHostFormatResolver for FixtureFormatResolver {
        fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
            Ok(match kind {
                "stdio.dwg" | "dwg" => Some(OsHostCodecFormat { short_id: "dwg".into(), extensions: vec![".dwg".into()] }),
                "stdio.step" | "step" => Some(OsHostCodecFormat { short_id: "step".into(), extensions: vec![".step".into(), ".stp".into()] }),
                _ => None,
            })
        }
    }

    fn request(operation: OsHostCodecOperation, request_id: u64, generation: u32, input_len: usize) -> AbiRequest {
        let mut metadata = vec![1];
        metadata.extend_from_slice(&(input_len as u32).to_le_bytes());
        AbiRequest { operation: operation.abi(), request_id: AbiRequestId(request_id), generation, bytes: AbiBytes::try_new(metadata).unwrap() }
    }

    fn page(handle: AbiHandle, index: u32, bytes: &[u8]) -> AbiPage {
        AbiPage::try_new(handle, index, bytes.to_vec()).unwrap()
    }

    fn admit_input<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle, bytes: &[u8]) {
        service.offer(handle, page(handle, 0, bytes)).unwrap();
        loop {
            let step = service.step(handle, AbiWorkBudget::credits(99)).unwrap();
            if step.state == OsHostCodecStepState::InputAcknowledged {
                assert_eq!(step.input_acknowledgement, Some(AbiControl::Acknowledge { handle, index: 0 }));
                break;
            }
        }
        service.seal(handle).unwrap();
    }

    fn finish<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle) -> (Vec<u8>, AbiReply) {
        let mut output = Vec::new();
        let mut output_progress = 0_u64;
        loop {
            let step = service.step(handle, AbiWorkBudget::credits(99)).unwrap();
            if step.event.bytes.as_slice()[1] == OsHostCodecPhase::Output as u8 {
                let completed = u64::from_le_bytes(step.event.bytes.as_slice()[2..10].try_into().unwrap());
                let total = u64::from_le_bytes(step.event.bytes.as_slice()[10..18].try_into().unwrap());
                assert_eq!(completed, output_progress + 1);
                assert!(completed <= total);
                output_progress = completed;
            }
            if let Some(page) = step.page {
                output.extend_from_slice(page.bytes.as_slice());
                service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
            }
            if let Some(reply) = step.reply {
                return (output, reply);
            }
        }
    }

    fn payload(bytes: &[u8]) -> &[u8] {
        let len = u32::from_le_bytes(bytes[2..6].try_into().unwrap()) as usize;
        assert_eq!(bytes.len(), len + 6);
        &bytes[6..]
    }

    fn run<R: OsHostFormatResolver>(resolver: R, operation: OsHostCodecOperation, request_id: u64, bytes: &[u8]) -> (Vec<u8>, AbiReply) {
        let mut service = RetainedOsHostCodecService::new(resolver);
        let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
        if bytes.is_empty() {
            service.seal(handle).unwrap();
        } else {
            admit_input(&mut service, handle, bytes);
        }
        finish(&mut service, handle)
    }

    fn hex_bytes(hex: &str) -> Vec<u8> {
        hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }

    const CANONICAL_DSL: &[u8] = b"name=fixture\ngraph {\n}\ndirty-node-ids=[ ]\nexpected-deliveries [edge-id:TEXT] {\n}\n";

    fn structural_pack(dsl: &[u8]) -> Vec<u8> {
        let mut bytes = WORKFLOW_PACK_MAGIC.to_vec();
        bytes.push(1);
        bytes.extend_from_slice(&(dsl.len() as u32).to_le_bytes());
        bytes.extend_from_slice(dsl);
        bytes
    }

    fn admit_page<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle, index: u32, bytes: &[u8]) {
        service.offer(handle, page(handle, index, bytes)).unwrap();
        loop {
            let step = service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
            if step.state == OsHostCodecStepState::InputAcknowledged {
                assert_eq!(step.input_acknowledgement, Some(AbiControl::Acknowledge { handle, index }));
                return;
            }
        }
    }

    #[test]
    fn schema_and_language_neutral_fixture_cover_every_operation() {
        for name in ["decodeWorkflowFixturePack", "parseWorkflowFixtureDsl", "mediaAcceptFilterKinds", "normalizeStdioFormatKind"] {
            assert!(OS_HOST_CODEC_SCHEMA_JSON.contains(name));
        }
        for name in ["workflowPackOrder", "canonicalDslBytes:u32le", "workflowDslCanonical", "filterRetainedState", "normalizeRetainedState", "one-byte-or-completed-item-opportunity-per-grant", "transferPages"] {
            assert!(OS_HOST_CODEC_SCHEMA_JSON.contains(name));
        }
        for name in ["decode_pack_request", "parse_dsl_request", "filter_request", "normalize_request"] {
            assert!(OS_HOST_CODEC_LEDGER_FIXTURE.contains(name));
        }
        for line in OS_HOST_CODEC_LEDGER_FIXTURE.lines().skip(1).take(5) {
            let (_, hex) = line.split_once('\t').unwrap();
            assert!(matches!(decode_abi_message(&hex_bytes(hex)), Ok(AbiMessage::Request(_) | AbiMessage::Reply(_))));
        }
        let rows: Vec<_> = OS_HOST_CODEC_LEDGER_FIXTURE.lines().filter_map(|line| line.split_once('\t')).collect();
        let dsl = hex_bytes(rows.iter().find(|(name, _)| *name == "workflow_dsl_canonical").unwrap().1);
        let pack = hex_bytes(rows.iter().find(|(name, _)| *name == "workflow_pack_structural").unwrap().1);
        assert_eq!(&pack[..4], &WORKFLOW_PACK_MAGIC);
        assert_eq!(&pack[9..], dsl);
    }

    #[test]
    fn valid_pack_and_dsl_are_equivalent_deterministic_paged_replies() {
        let pack = structural_pack(CANONICAL_DSL);
        let mut replies = Vec::new();
        for (operation, bytes) in [(OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()), (OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL)] {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(operation, 7, 1, bytes.len())).unwrap();
            admit_input(&mut service, handle, bytes);
            let (output, reply) = finish(&mut service, handle);
            assert_eq!(payload(&output), CANONICAL_DSL);
            assert_eq!(reply.status, AbiStatus::OK);
            replies.push((output, reply.bytes.into_vec()));
        }
        assert_eq!(replies[0], replies[1]);
    }

    #[test]
    fn workflow_pack_and_dsl_accept_every_byte_and_field_split() {
        let pack = structural_pack(CANONICAL_DSL);
        for (operation, bytes) in [(OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()), (OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL)] {
            for split in 0..=bytes.len() {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, 70, 1, bytes.len())).unwrap();
                admit_page(&mut service, handle, 0, &bytes[..split]);
                admit_page(&mut service, handle, 1, &bytes[split..]);
                service.seal(handle).unwrap();
                let (output, reply) = finish(&mut service, handle);
                assert_eq!(payload(&output), CANONICAL_DSL, "split={split}");
                assert_eq!(reply.status, AbiStatus::OK, "split={split}");
            }
        }
    }

    #[test]
    fn filter_and_normalize_preserve_registered_format_behavior() {
        let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 8, 1, filter.len())).unwrap();
        admit_input(&mut service, handle, &filter);
        let (output, _) = finish(&mut service, handle);
        assert_eq!(payload(&output), b".dwg,.step,.stp");

        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 9, 1, 9)).unwrap();
        admit_input(&mut service, handle, b"stdio.dwg");
        let (output, _) = finish(&mut service, handle);
        assert_eq!(payload(&output), b"dwg");
    }

    #[test]
    fn filter_and_normalize_accept_every_byte_and_field_split() {
        let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
        for (request_id, operation, bytes, expected) in
            [(81, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice(), b".dwg,.step,.stp".as_slice()), (82, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice(), b"dwg".as_slice())]
        {
            for split in 0..=bytes.len() {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                admit_page(&mut service, handle, 0, &bytes[..split]);
                admit_page(&mut service, handle, 1, &bytes[split..]);
                service.seal(handle).unwrap();
                let (output, reply) = finish(&mut service, handle);
                assert_eq!(payload(&output), expected, "split={split}");
                assert_eq!(reply.status, AbiStatus::OK, "split={split}");
            }
        }

        struct UnicodeKindResolver;
        impl OsHostFormatResolver for UnicodeKindResolver {
            fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                Ok((kind == "dωg").then(|| OsHostCodecFormat { short_id: "unicode".into(), extensions: vec![".unicode".into()] }))
            }
        }
        let kind = "dωg".as_bytes();
        let mut filter = vec![1, 1, 0];
        filter.extend_from_slice(&(kind.len() as u16).to_le_bytes());
        filter.extend_from_slice(kind);
        for (request_id, operation, bytes, expected) in [(103, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice(), b".unicode".as_slice()), (104, OsHostCodecOperation::NormalizeStdioFormatKind, kind, b"unicode".as_slice())] {
            for split in 0..=bytes.len() {
                let mut service = RetainedOsHostCodecService::new(UnicodeKindResolver);
                let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                admit_page(&mut service, handle, 0, &bytes[..split]);
                admit_page(&mut service, handle, 1, &bytes[split..]);
                service.seal(handle).unwrap();
                let (output, reply) = finish(&mut service, handle);
                assert_eq!(payload(&output), expected, "unicode split={split}");
                assert_eq!(reply.status, AbiStatus::OK, "unicode split={split}");
            }
        }
    }

    #[test]
    fn filter_and_normalize_zero_max_and_plus_one_bounds_precede_item_copy() {
        struct AnyKindResolver;
        impl OsHostFormatResolver for AnyKindResolver {
            fn resolve_format(&mut self, _: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                Ok(Some(OsHostCodecFormat { short_id: "x".into(), extensions: Vec::new() }))
            }
        }

        let (output, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 83, &[1, 0, 0]);
        assert!(payload(&output).is_empty());
        assert_eq!(reply.status, AbiStatus::OK);

        let mut maximum_count = vec![1, 0, 1];
        for _ in 0..OS_HOST_CODEC_MAX_KIND_COUNT {
            maximum_count.extend_from_slice(&0_u16.to_le_bytes());
        }
        let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 84, &maximum_count);
        assert_eq!(reply.status, AbiStatus::OK);

        let plus_one_count = [1, 1, 1];
        let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 85, &plus_one_count);
        assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::InputLimit as u16);

        let mut maximum_kind = vec![1, 1, 0];
        maximum_kind.extend_from_slice(&(OS_HOST_CODEC_MAX_KIND_BYTES as u16).to_le_bytes());
        maximum_kind.extend(std::iter::repeat_n(b'x', OS_HOST_CODEC_MAX_KIND_BYTES));
        let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 86, &maximum_kind);
        assert_eq!(reply.status, AbiStatus::OK);
        let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::NormalizeStdioFormatKind, 87, &maximum_kind[5..]);
        assert_eq!(reply.status, AbiStatus::OK);

        let plus_one_kind = (OS_HOST_CODEC_MAX_KIND_BYTES + 1) as u16;
        let filter_plus_one = [1, 1, 0, plus_one_kind as u8, (plus_one_kind >> 8) as u8];
        let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 88, &filter_plus_one);
        assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::InputLimit as u16);

        let mut service = RetainedOsHostCodecService::new(AnyKindResolver);
        let rejected = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 89, 1, OS_HOST_CODEC_MAX_KIND_BYTES + 1)).unwrap_err();
        assert_eq!(rejected.0, AbiErrorCode::LimitExceeded);
        assert_eq!(rejected.1.request_id, AbiRequestId(89));
    }

    #[test]
    fn filter_and_normalize_reject_malformed_truncated_and_invalid_utf8_incrementally() {
        let cases: &[(OsHostCodecOperation, &[u8], OsHostCodecErrorCode)] = &[
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[2, 0, 0], OsHostCodecErrorCode::MalformedRequest),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1], OsHostCodecErrorCode::MalformedRequest),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3], OsHostCodecErrorCode::MalformedRequest),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3, 0, b'd'], OsHostCodecErrorCode::MalformedRequest),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 1, 0, 0xc3], OsHostCodecErrorCode::InvalidUtf8),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 0, 0, 0], OsHostCodecErrorCode::MalformedRequest),
            (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3, 0, b'w', b'a', b't'], OsHostCodecErrorCode::UnknownKind),
            (OsHostCodecOperation::NormalizeStdioFormatKind, &[0xc3], OsHostCodecErrorCode::InvalidUtf8),
            (OsHostCodecOperation::NormalizeStdioFormatKind, &[0xff], OsHostCodecErrorCode::InvalidUtf8),
            (OsHostCodecOperation::NormalizeStdioFormatKind, &[], OsHostCodecErrorCode::MalformedRequest),
        ];
        for (index, (operation, bytes, code)) in cases.iter().enumerate() {
            let (_, reply) = run(FixtureFormatResolver, *operation, 90 + index as u64, bytes);
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), *code as u16, "case={index}");
            assert_eq!(reply.status.code, AbiStatusCode::Rejected, "case={index}");
        }
    }

    #[test]
    fn malformed_pack_dsl_missing_array_and_unknown_kind_are_owned_failures() {
        for (operation, bytes, code) in [
            (OsHostCodecOperation::DecodeWorkflowFixturePack, b"bad".as_slice(), OsHostCodecErrorCode::MalformedPack),
            (OsHostCodecOperation::ParseWorkflowFixtureDsl, b"bad".as_slice(), OsHostCodecErrorCode::MalformedDsl),
            (OsHostCodecOperation::MediaAcceptFilterKinds, b"".as_slice(), OsHostCodecErrorCode::MissingKindArray),
            (OsHostCodecOperation::NormalizeStdioFormatKind, b"wat".as_slice(), OsHostCodecErrorCode::UnknownKind),
        ] {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(operation, 11, 1, bytes.len())).unwrap();
            if bytes.is_empty() {
                service.seal(handle).unwrap();
            } else {
                admit_input(&mut service, handle, bytes);
            }
            let (_, reply) = finish(&mut service, handle);
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), code as u16);
            assert_eq!(reply.status.code, AbiStatusCode::Rejected);
        }
    }

    #[test]
    fn truncated_pack_dsl_and_invalid_utf8_fail_after_retained_decode() {
        let mut truncated_pack = structural_pack(CANONICAL_DSL);
        truncated_pack.pop();
        let mut truncated_dsl = CANONICAL_DSL.to_vec();
        truncated_dsl.pop();
        let mut invalid_utf8 = CANONICAL_DSL.to_vec();
        invalid_utf8.insert(invalid_utf8.len() - 1, 0xff);
        for (request_id, operation, bytes, code) in [
            (12, OsHostCodecOperation::DecodeWorkflowFixturePack, truncated_pack, OsHostCodecErrorCode::MalformedPack),
            (13, OsHostCodecOperation::ParseWorkflowFixtureDsl, truncated_dsl, OsHostCodecErrorCode::MalformedDsl),
            (14, OsHostCodecOperation::ParseWorkflowFixtureDsl, invalid_utf8, OsHostCodecErrorCode::InvalidUtf8),
        ] {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
            admit_input(&mut service, handle, &bytes);
            let (_, reply) = finish(&mut service, handle);
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), code as u16);
            assert_eq!(reply.status.code, AbiStatusCode::Rejected);
        }
    }

    #[test]
    fn exact_input_and_output_limits_reject_plus_one_without_consuming_request() {
        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let maximum = request(OsHostCodecOperation::DecodeWorkflowFixturePack, 20, 1, OS_HOST_CODEC_MAX_INPUT_BYTES);
        assert!(service.begin(maximum).is_ok());
        let mut plus_one = vec![1];
        plus_one.extend_from_slice(&((OS_HOST_CODEC_MAX_INPUT_BYTES + 1) as u32).to_le_bytes());
        let rejected = AbiRequest { operation: OsHostCodecOperation::DecodeWorkflowFixturePack.abi(), request_id: AbiRequestId(21), generation: 1, bytes: AbiBytes::try_new(plus_one).unwrap() };
        let rejected = service.begin(rejected).unwrap_err();
        assert_eq!(rejected.0, AbiErrorCode::LimitExceeded);
        assert_eq!(rejected.1.request_id, AbiRequestId(21));

        struct LargeFormatResolver(usize);
        impl OsHostFormatResolver for LargeFormatResolver {
            fn resolve_format(&mut self, _: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                Ok(Some(OsHostCodecFormat { short_id: "x".repeat(self.0), extensions: Vec::new() }))
            }
        }
        let mut service = RetainedOsHostCodecService::new(LargeFormatResolver(OS_HOST_CODEC_MAX_OUTPUT_BYTES - 6));
        let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 22, 1, 1)).unwrap();
        service.offer(handle, page(handle, 0, b"x")).unwrap();
        service.step(handle, AbiWorkBudget::credits(1)).unwrap();
        service.seal(handle).unwrap();
        assert_eq!(service.step(handle, AbiWorkBudget::credits(1)).unwrap().state, OsHostCodecStepState::Progress);
        assert_eq!(service.handles.get_mut(handle).unwrap().output_bytes, OS_HOST_CODEC_MAX_OUTPUT_BYTES);
        let mut service = RetainedOsHostCodecService::new(LargeFormatResolver(OS_HOST_CODEC_MAX_OUTPUT_BYTES - 5));
        let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 23, 1, 1)).unwrap();
        service.offer(handle, page(handle, 0, b"x")).unwrap();
        service.step(handle, AbiWorkBudget::credits(1)).unwrap();
        service.seal(handle).unwrap();
        let reply = service.step(handle, AbiWorkBudget::credits(1)).unwrap().reply.unwrap();
        assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::OutputLimit as u16);
    }

    #[test]
    fn page_count_limit_precedes_sequence_classification_and_returns_the_page() {
        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let handle = service.begin(request(OsHostCodecOperation::DecodeWorkflowFixturePack, 24, 1, 0)).unwrap();
        let page = AbiPage { handle, index: ABI_MAX_PAGES_PER_TRANSFER, bytes: AbiPageBytes::default() };
        let rejected = service.offer(handle, page).unwrap_err();
        assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
        assert_eq!(rejected.index, ABI_MAX_PAGES_PER_TRANSFER);
        assert!(rejected.bytes.is_empty());
    }

    #[test]
    fn cancel_mid_every_structural_cursor_returns_exact_page_and_blocks_progress() {
        let pack = structural_pack(CANONICAL_DSL);
        let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
        for (request_id, operation, bytes) in [
            (30, OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()),
            (31, OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL),
            (33, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice()),
            (34, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice()),
        ] {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(operation, request_id, 4, bytes.len())).unwrap();
            service.offer(handle, page(handle, 0, bytes)).unwrap();
            let copied = bytes.len() / 2;
            for _ in 0..copied {
                service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
            }
            let outcome = service.control(AbiControl::Cancel { request_id: AbiRequestId(request_id), generation: 4 }).unwrap().unwrap();
            assert_eq!(outcome.page.unwrap().bytes.as_slice(), bytes);
            assert_eq!((outcome.admitted_byte_credits, outcome.copied_bytes), (bytes.len(), copied));
            assert_eq!(service.step(handle, AbiWorkBudget::credits(1)), Err(AbiErrorCode::Cancelled));
        }
    }

    #[test]
    fn deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor() {
        let filter = [1, 1, 0, 3, 0, b'd', b'w', b'g'];
        for (request_id, operation, bytes) in
            [(32, OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL), (35, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice()), (36, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice())]
        {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
            service.offer(handle, page(handle, 0, bytes)).unwrap();
            assert_eq!(service.step(handle, AbiWorkBudget::credits(0)), Err(AbiErrorCode::NoCredit));
            assert_eq!(service.step(handle, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
            assert_eq!(service.step(handle, AbiWorkBudget { now_ms: 5, deadline_ms: Some(5), ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::DeadlineExceeded));
            let step = service.step(handle, AbiWorkBudget::credits(1)).unwrap();
            assert_eq!(step.event.bytes.as_slice()[2..10], 1_u64.to_le_bytes());
            assert_eq!(service.handles.get_mut(handle).unwrap().input_bytes_received, 1);
        }
    }

    #[test]
    fn handle_loss_stale_generation_duplicate_ack_and_interrupted_close_are_exact() {
        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let first = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 40, 1, 3)).unwrap();
        service.lose(first).unwrap();
        assert_eq!(service.step(first, AbiWorkBudget::credits(1)), Err(AbiErrorCode::UnknownHandle));
        let second = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 40, 2, 3)).unwrap();
        assert_eq!(service.step(first, AbiWorkBudget::credits(1)), Err(AbiErrorCode::AbaHandle));
        admit_input(&mut service, second, b"dwg");
        let mut output_page = None;
        while output_page.is_none() {
            output_page = service.step(second, AbiWorkBudget::credits(1)).unwrap().page;
        }
        let output_page = output_page.unwrap();
        let ack = AbiControl::Acknowledge { handle: second, index: output_page.index };
        service.control(ack).unwrap();
        assert_eq!(service.control(ack), Err(AbiErrorCode::DuplicateAcknowledgement));
        assert_eq!(service.close_step(AbiControl::Close { handle: second }, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
        while !service.close_step(AbiControl::Close { handle: second }, AbiWorkBudget::credits(1)).unwrap() {}
    }

    #[test]
    fn filter_output_acknowledgement_and_interrupted_close_are_exact() {
        let filter = [1, 1, 0, 4, 0, b's', b't', b'e', b'p'];
        let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
        let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 41, 1, filter.len())).unwrap();
        admit_input(&mut service, handle, &filter);
        let output_page = loop {
            if let Some(page) = service.step(handle, AbiWorkBudget::credits(1)).unwrap().page {
                break page;
            }
        };
        assert_eq!(payload(output_page.bytes.as_slice()), b".step,.stp");
        let acknowledgement = AbiControl::Acknowledge { handle, index: output_page.index };
        service.control(acknowledgement).unwrap();
        assert_eq!(service.control(acknowledgement), Err(AbiErrorCode::DuplicateAcknowledgement));
        assert_eq!(service.close_step(AbiControl::Close { handle }, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
        while !service.close_step(AbiControl::Close { handle }, AbiWorkBudget::credits(1)).unwrap() {}
    }

    #[test]
    fn public_interactive_route_has_no_batch_or_whole_input_capability() {
        let source = include_str!("../../🦀️.rs");
        let start = source.find("pub mod codec_abi {").unwrap();
        let tests = source[start..].find("//#region 🧪️Tests").unwrap() + start;
        let production = &source[start..tests];
        for forbidden in ["UiForbidden", "ArtifactPack", "ArtifactDsl", "decode_pack(", "parse_dsl(", "decode_workflow_fixture_pack", "parse_workflow_fixture_dsl"] {
            assert!(!production.contains(forbidden), "public route contains forbidden batch edge {forbidden}");
        }
        for forbidden in [concat!("Bytes(Vec<", "u8>)"), concat!("Self::", "Bytes("), concat!("input.", "bytes()"), concat!("execute_", "filter(")] {
            assert!(!production.contains(forbidden), "public route contains forbidden whole-input edge {forbidden}");
        }
        assert!(production.contains("RegisteredOsHostFormatResolver"));
        assert!(production.contains("WorkflowStructuralCursor"));
        assert!(production.contains("FilterKindsStructuralCursor"));
        assert!(production.contains("NormalizeKindStructuralCursor"));
    }

    #[cfg(any(feature = "os-host-full", feature = "space-guest"))]
    #[test]
    fn public_service_runs_the_retained_workflow_cursor_without_a_format_backend() {
        let mut service = OsHostCodecService::new();
        let handle = service.begin(request(OsHostCodecOperation::ParseWorkflowFixtureDsl, 80, 1, CANONICAL_DSL.len())).unwrap();
        service.offer(handle, page(handle, 0, CANONICAL_DSL)).unwrap();
        loop {
            if service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                break;
            }
        }
        service.seal(handle).unwrap();
        let mut output = Vec::new();
        loop {
            let step = service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
            if let Some(page) = step.page {
                output.extend_from_slice(page.bytes.as_slice());
                service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
            }
            if let Some(reply) = step.reply {
                assert_eq!(reply.status, AbiStatus::OK);
                break;
            }
        }
        assert_eq!(payload(&output), CANONICAL_DSL);
    }

    #[cfg(any(feature = "os-host-full", feature = "space-guest"))]
    #[test]
    fn public_service_runs_filter_and_normalize_structural_cursors() {
        let mut service = OsHostCodecService::new();
        let filter = [1, 0, 0];
        let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 101, 1, filter.len())).unwrap();
        service.offer(handle, page(handle, 0, &filter)).unwrap();
        loop {
            if service.step(handle, AbiWorkBudget::credits(1)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                break;
            }
        }
        service.seal(handle).unwrap();
        let mut output = Vec::new();
        loop {
            let step = service.step(handle, AbiWorkBudget::credits(1)).unwrap();
            if let Some(page) = step.page {
                output.extend_from_slice(page.bytes.as_slice());
                service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
            }
            if let Some(reply) = step.reply {
                assert_eq!(reply.status, AbiStatus::OK);
                break;
            }
        }
        assert!(payload(&output).is_empty());

        let mut service = OsHostCodecService::new();
        let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 102, 1, 3)).unwrap();
        service.offer(handle, page(handle, 0, b"wat")).unwrap();
        loop {
            if service.step(handle, AbiWorkBudget::credits(1)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                break;
            }
        }
        service.seal(handle).unwrap();
        let reply = service.step(handle, AbiWorkBudget::credits(1)).unwrap().reply.unwrap();
        assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::UnknownKind as u16);
    }
}
