//! 🪧️ Aggregator pane smoke: the Nakagin example loads through the example tool job, one `worldRelocate` applies, undo restores the loaded pose, redo reapplies it and the app closes through its exact retained close.

use crate::context::*;
use super::*;

/// 🧩️ LAW: the Aggregator pane switches to a bundled example, relocates an object, undoes and redoes it and closes with the store drop witness satisfied.
#[semio_framework_async_macros::async_test]
async fn aggregator_pane_loads_applies_undoes_and_closes() {
    let mut app = app().await;
    let booted = object_count(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin loads");
    assert_eq!(object_count(&app), NAKAGIN_EXAMPLE_SNAPSHOT.objects.len(), "the bundled example replaced the booted document");
    assert_ne!(object_count(&app), booted);
    let object_id = first_object_id(&app);
    let loaded = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert_ne!(moved, loaded, "the typed action moved the object");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), loaded, "undo restores the loaded pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the pose");
    assert!(close_witness(&mut app).expect("original Puzzle3D app retirement"), "the app reaches terminal-empty ownership");
}
