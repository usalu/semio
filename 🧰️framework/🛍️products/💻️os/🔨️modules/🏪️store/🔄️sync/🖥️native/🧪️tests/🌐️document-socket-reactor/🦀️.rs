//! 🌐️ The document-socket reactor port: one shared reactor, entered scopes, detached tasks and the ordered event pump. See [`super`].

use super::{pump_events, DocumentSocketReactor};
use semio_framework_os_store_sync::ArtifactEvent;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn the_reactor_is_one_process_wide_instance() {
    let first = DocumentSocketReactor::shared().expect("the reactor starts");
    let second = DocumentSocketReactor::shared().expect("the reactor starts");
    assert!(std::ptr::eq(first, second));
}

#[test]
fn a_spawned_task_runs_on_the_reactor_thread_without_an_ambient_runtime() {
    let reactor = DocumentSocketReactor::shared().expect("the reactor starts");
    let (sender, receiver) = mpsc::channel();
    reactor.spawn(async move {
        sender.send(std::thread::current().name().map(str::to_string)).expect("the test is waiting");
    });
    let thread = receiver.recv_timeout(Duration::from_secs(5)).expect("the task ran");
    assert_eq!(thread.as_deref(), Some("semio-document-io"));
}

#[test]
fn an_entered_scope_lets_a_plain_thread_poll_timer_futures() {
    let reactor = DocumentSocketReactor::shared().expect("the reactor starts");
    let polled = std::thread::spawn(move || {
        let _scope = reactor.enter();
        semio_framework_async::block_on(async { tokio::time::timeout(Duration::from_secs(5), std::future::ready(7)).await })
    })
    .join()
    .expect("the polling thread returns");
    assert_eq!(polled, Ok(7));
}

#[test]
fn the_pump_delivers_every_event_in_order_and_ends_when_the_document_closes() {
    let (sender, receiver) = tokio::sync::broadcast::channel(8);
    sender.send(ArtifactEvent::DocumentBackbone { message: b"first".to_vec() }).expect("a receiver is subscribed");
    sender.send(ArtifactEvent::DocumentBackbone { message: b"second".to_vec() }).expect("a receiver is subscribed");
    drop(sender);
    let mut seen = Vec::new();
    semio_framework_async::block_on(pump_events(receiver, |event| {
        if let ArtifactEvent::DocumentBackbone { message } = event {
            seen.push(String::from_utf8(message).expect("the fixture bytes are text"));
        }
    }));
    assert_eq!(seen, vec!["first".to_string(), "second".to_string()]);
}

#[test]
fn a_lagging_receiver_skips_the_lost_events_and_keeps_pumping() {
    let (sender, receiver) = tokio::sync::broadcast::channel(2);
    for label in ["a", "b", "c", "d"] {
        sender.send(ArtifactEvent::DocumentBackbone { message: label.as_bytes().to_vec() }).expect("a receiver is subscribed");
    }
    drop(sender);
    let mut seen = Vec::new();
    semio_framework_async::block_on(pump_events(receiver, |event| {
        if let ArtifactEvent::DocumentBackbone { message } = event {
            seen.push(String::from_utf8(message).expect("the fixture bytes are text"));
        }
    }));
    assert_eq!(seen, vec!["c".to_string(), "d".to_string()]);
}
