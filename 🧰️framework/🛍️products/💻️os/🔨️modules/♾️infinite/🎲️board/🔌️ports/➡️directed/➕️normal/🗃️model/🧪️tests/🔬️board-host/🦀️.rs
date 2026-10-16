//! 🧪️ normal-model tests (🔬️board-host): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_normal_model::{BOARD_EVENT_ITEM_CAPACITY, BoardEventKind, BoardEventQueue, BoardOwnedEvent};

#[cfg(test)]
    #[test]
    fn typed_event_queue_preserves_fifo_saturation_and_one_event_close_progress() {
        let mut queue = BoardEventQueue::default();
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let payload = format!(r#"{{"id":"node-{index}"}}"#);
            queue.push(BoardOwnedEvent::from_payload(BoardEventKind::NodeMove, &payload, Some("node")).unwrap()).unwrap();
        }
        let overflow = BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":[]}"#, None).unwrap();
        assert!(queue.push(overflow).is_err());
        assert_eq!(queue.len(), BOARD_EVENT_ITEM_CAPACITY);
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let event = queue.pop().expect("fifo event");
            assert_eq!(event.kind(), BoardEventKind::NodeMove);
            assert!(event.payload_json().contains(&format!("node-{index}")));
        }
        assert!(queue.terminal_is_empty());

        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":["a"]}"#, None).unwrap()).unwrap();
        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":"a"}"#, None).unwrap()).unwrap();
        assert!(!queue.close_step());
        assert!(!queue.close_step());
        assert!(queue.close_step());
        assert!(queue.terminal_is_empty());
    }
