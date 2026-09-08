mod typed_operation_result_exchange_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn concrete_exchange_is_installed_and_retains_the_page_until_the_receiver_accepts_the_exact_ack() {
        install_mounted_typed_operation_result_exchange();
        assert!(typed_operation_result_exchange().get().is_some());

        let exchange = MountedTypedOperationResultExchange::new();
        let token = TypedOperationResultToken { receiver: u32::MAX, operation: u64::MAX, generation: u64::MAX - 1, sequence: u32::MAX, attempt: u8::MAX };
        let page = TypedOperationResultPage::try_copy_from(token, u8::MAX, b"retained-page").expect("bounded typed-operation page");
        let attempts = Arc::new(AtomicUsize::new(0));
        let callback_attempts = attempts.clone();
        let published = exchange.publish(page, Arc::new(move |acknowledged| acknowledged == token && callback_attempts.fetch_add(1, Ordering::SeqCst) > 0));
        assert!(published.is_ok(), "first retained page slot");

        let wrong = TypedOperationResultToken { sequence: token.sequence - 1, ..token };
        assert!(!exchange.acknowledge(wrong));
        assert!(!exchange.acknowledge(token));
        assert_eq!(exchange.take_page(token.receiver).expect("rejected ACK retains the exact page").bytes(), b"retained-page");
        assert!(exchange.acknowledge(token));
        assert!(exchange.take_page(token.receiver).is_none());
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn renderer_wire_maximum_plus_one_and_exchange_saturation_preserve_exact_owners() {
        let token = TypedOperationResultToken { receiver: 7, operation: 11, generation: 13, sequence: 17, attempt: 1 };
        assert!(TypedOperationResultPage::try_copy_from(token, 0, &[0; TYPED_OPERATION_RESULT_PAGE_BYTES]).is_ok());
        assert!(TypedOperationResultPage::try_copy_from(token, 0, &[0; TYPED_OPERATION_RESULT_PAGE_BYTES + 1]).is_err());
        let mut wire = Vec::from(TypedOperationResultPage::PAGE_MAGIC);
        wire.extend_from_slice(&token.receiver.to_le_bytes());
        wire.extend_from_slice(&token.operation.to_le_bytes());
        wire.extend_from_slice(&token.generation.to_le_bytes());
        wire.extend_from_slice(&token.sequence.to_le_bytes());
        wire.push(token.attempt);
        wire.push(0);
        wire.extend_from_slice(&(TYPED_OPERATION_RESULT_PAGE_BYTES as u32).to_le_bytes());
        wire.extend_from_slice(&[0x2a; TYPED_OPERATION_RESULT_PAGE_BYTES]);
        assert_eq!(TypedOperationResultPage::decode_guest_message(&wire).expect("exact maximum guest page").bytes(), &[0x2a; TYPED_OPERATION_RESULT_PAGE_BYTES]);
        wire.push(0);
        assert!(TypedOperationResultPage::decode_guest_message(&wire).is_none(), "maximum plus one wire byte is rejected");

        let exchange = MountedTypedOperationResultExchange::new();
        for sequence in 0..64 {
            let token = TypedOperationResultToken { sequence, ..token };
            let page = TypedOperationResultPage::try_copy_from(token, 0, &[sequence as u8]).expect("bounded saturation page");
            assert!(exchange.publish(page, Arc::new(|_| true)).is_ok());
        }
        let overflow_token = TypedOperationResultToken { sequence: 64, ..token };
        let overflow = TypedOperationResultPage::try_copy_from(overflow_token, 0, b"overflow").expect("bounded overflow owner");
        let returned = match exchange.publish(overflow, Arc::new(|_| true)) {
            Ok(()) => panic!("maximum plus one page must be returned"),
            Err(returned) => returned,
        };
        assert_eq!(returned.token, overflow_token);
        assert_eq!(returned.bytes(), b"overflow");

        let retries = MountedTypedOperationResultExchange::new();
        let first = TypedOperationResultPage::try_copy_from(token, 0, b"first").expect("first attempt");
        retries.publish(first, Arc::new(|_| true)).unwrap_or_else(|_| panic!("first attempt slot"));
        let latest_token = TypedOperationResultToken { attempt: 2, ..token };
        let latest = TypedOperationResultPage::try_copy_from(latest_token, 0, b"latest").expect("latest attempt");
        retries.publish(latest, Arc::new(|_| true)).unwrap_or_else(|_| panic!("latest attempt replaces stable identity"));
        assert!(!retries.acknowledge(token), "stale attempt must not consume the replacement owner");
        assert_eq!(retries.take_page(token.receiver).expect("latest owner retained").token, latest_token);
    }
}
