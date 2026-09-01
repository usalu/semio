//! 📥️ Observes the actual queue/scene frontier of one production enqueue.

use super::{enqueue_runtime_completion, RuntimeApply, RuntimeCompletion, RuntimeCompletionQueue, RuntimeHostWaker, RuntimePresentationAuthority};
use std::sync::{mpsc, Mutex, TryLockError};
use std::time::Duration;

//#region 📥️PublicationInterlock
struct PublicationInterlock {
    published: mpsc::Sender<()>,
    resume: mpsc::Receiver<()>,
}

thread_local! {
    static INTERLOCK: std::cell::RefCell<Option<PublicationInterlock>> = const { std::cell::RefCell::new(None) };
}

pub(super) fn pause_after_completion_publication() {
    INTERLOCK.with(|slot| {
        if let Some(interlock) = slot.borrow().as_ref() {
            interlock.published.send(()).unwrap();
            interlock.resume.recv_timeout(Duration::from_secs(2)).unwrap();
        }
    });
}
//#endregion 📥️PublicationInterlock

//#region 🧪️SingleEnqueue
#[test]
fn runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let count = |field: &str, key: &str| fixture[field][key].as_str().unwrap().parse::<u64>().unwrap();
    let queue = Mutex::new(RuntimeCompletionQueue::new());
    let presentation = RuntimePresentationAuthority::new();
    presentation.observe_input_generation(count("old", "observedBuildInputGeneration"));
    let old = presentation.current();
    let waker = Mutex::new(None::<RuntimeHostWaker>);
    let source = &fixture["source"];
    let source_revision = source["revision"].as_str().unwrap().parse().unwrap();
    let scalar = |field: &str| serde_json::from_value::<f32>(source[field].clone()).unwrap();
    let size = (scalar("width"), scalar("height"), scalar("dpr"));
    let completion = RuntimeCompletion {
        key: Some("window-metrics"),
        revision: source_revision,
        requires_interaction: true,
        apply: RuntimeApply::Resize { width: size.0, height: size.1, dpr: size.2 },
    };
    let (publication, observation, resumed, accepted, reader_joined) = std::thread::scope(|scope| {
        let (published, publication) = mpsc::channel();
        let (resume, resumed) = mpsc::channel();
        let queue = &queue;
        let presentation = &presentation;
        let waker = &waker;
        let writer = scope.spawn(move || {
            INTERLOCK.with(|slot| *slot.borrow_mut() = Some(PublicationInterlock { published, resume: resumed }));
            let accepted = enqueue_runtime_completion(queue, presentation, waker, completion);
            INTERLOCK.with(|slot| { slot.borrow_mut().take(); });
            accepted
        });
        let publication = publication.recv_timeout(Duration::from_secs(1));
        let (observed, observation) = mpsc::channel();
        let reader = scope.spawn(move || {
            let result = match queue.try_lock() {
                Ok(queue) => Ok((queue.ready.len(), presentation.current())),
                Err(TryLockError::WouldBlock) => Err(false),
                Err(TryLockError::Poisoned(_)) => Err(true),
            };
            let _ = observed.send(result);
        });
        let observation = observation.recv_timeout(Duration::from_millis(250));
        let resumed = resume.send(());
        let accepted = writer.join();
        let reader_joined = reader.join();
        (publication, observation, resumed, accepted, reader_joined)
    });
    let committed = presentation.current();
    let mut queue = queue.lock().unwrap_or_else(|poison| poison.into_inner());
    let published_count = queue.ready.len();
    let mut exact_source = false;
    while let Some(published) = queue.ready.pop_front() {
        exact_source = published.key == Some("window-metrics")
            && published.revision == source_revision
            && published.requires_interaction
            && matches!(&published.apply, RuntimeApply::Resize { width, height, dpr } if (*width, *height, *dpr) == size);
        drop(published);
    }
    drop(queue);
    eprintln!("[DEBUG] single enqueue publication={publication:?} observation={observation:?} resumed={resumed:?} old={old:?} committed={committed:?} publishedCount={published_count} exactSource={exact_source}");
    assert!(publication.is_ok() && resumed.is_ok() && reader_joined.is_ok());
    assert!(accepted.unwrap() && exact_source);
    assert_eq!(source_revision, count("source", "revision"));
    assert_eq!(old.scene_revision, count("old", "sceneRevision"));
    assert_eq!(published_count as u64, count("committed", "completionReadyCount"));
    assert_eq!(committed.scene_revision, count("committed", "sceneRevision"));
    assert_eq!(committed.input_generation, count("old", "observedBuildInputGeneration"));
    let observed = observation.expect("actual reader must return without awaiting the writer");
    let accepted_half = match observed {
        Ok((count, witness)) => !((count == 0 && witness == old) || (count == 1 && witness == committed)),
        Err(false) => false,
        Err(true) => panic!("actual committed-state receiver was poisoned"),
    };
    assert_eq!(accepted_half, fixture["invariants"]["halfOfOneEnqueueAccepted"].as_bool().unwrap());
}
//#endregion 🧪️SingleEnqueue
