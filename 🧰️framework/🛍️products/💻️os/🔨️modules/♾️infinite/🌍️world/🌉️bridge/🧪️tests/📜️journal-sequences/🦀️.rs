//! 🧪️ bridge tests (📜️journal-sequences): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_components::WorldNavigationSnapshot;
use semio_framework_os_infinite_world_bridge::classify_world_navigation_gestures;

/// 🧭️ The classifier is React's `classifyWorldNavigationGestures`, thresholds and push order
/// included — a set, never one verdict, so a drag that pans while it orbits reports both.
#[test]
fn the_navigation_classifier_answers_reacts_own_thresholds_and_order() {
    let still = WorldNavigationSnapshot { position: [0.0, 0.0, 10.0], target: [0.0, 0.0, 0.0], zoom: 1.0, parallel: false };
    assert_eq!(classify_world_navigation_gestures(still, still), [false, false, false]);
    let panned = WorldNavigationSnapshot { target: [4.0, 0.0, 0.0], position: [4.0, 0.0, 10.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, panned), [true, false, false]);
    let dollied = WorldNavigationSnapshot { position: [0.0, 0.0, 5.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, dollied), [false, true, false]);
    let orbited = WorldNavigationSnapshot { position: [10.0, 0.0, 0.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, orbited), [false, false, true]);
    let below_every_threshold = WorldNavigationSnapshot { position: [0.0, 0.0, 10.1], target: [0.0, 0.0, 0.01], ..still };
    assert_eq!(classify_world_navigation_gestures(still, below_every_threshold), [false, false, false], "a gesture under React's thresholds is no gesture");
}
