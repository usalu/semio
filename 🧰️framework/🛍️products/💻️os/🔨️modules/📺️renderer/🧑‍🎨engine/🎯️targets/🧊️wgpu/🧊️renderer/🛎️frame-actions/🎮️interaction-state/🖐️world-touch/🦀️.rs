//! 🖐️ World3d touch lane: feeds the contacts of a touch pointer on a `world-3d` surface through the domain-neutral [`TouchSession`] and turns its steps into the interaction intents the surface already answers for a mouse. A tap or a drag is a primary press and drag (a drag is deferred until the finger left its slop, so a second finger never starts a marquee), a held finger is an additive press (shift) that asks the plugin for its long press at the deadline while it is still down, a one-finger orbit on tap-to-nudge devices is the alt-right-drag orbit, and two fingers are a shift-right-drag pan and a wheel dolly in the units that reproduce the web orbit rig.
//!
//! 🔗️ Pure part: `semio_framework::interaction::gesture::{TouchSession, pinch_navigation}`; the plugin decides the profile through `PluginShellHooks::touch_profile` and answers a long press or double tap through `handle_touch`.

use super::*;
use semio_framework::interaction::gesture::{pinch_navigation, OrbitProjection, TouchSession, TouchStep};
use semio_framework::{ShellTouchGesture, ShellTouchProfile};
use std::collections::BTreeMap;

thread_local! {
    static SESSIONS: std::cell::RefCell<BTreeMap<String, TouchSession>> = std::cell::RefCell::new(BTreeMap::new());
}

/// 🖐️ One contact event of a touch pointer.
#[derive(Clone, Copy, Debug)]
pub(super) enum Contact {
    Down { id: u64, x: f64, y: f64 },
    Move { id: u64, x: f64, y: f64 },
    Up { id: u64 },
    Cancel,
}

/// 🖐️ The steps one contact makes on the touch session of `host_id`; the profile configures the session when a finger lands on an idle one.
pub(super) fn feed(host_id: &str, profile: ShellTouchProfile, now: f64, contact: Contact) -> Vec<TouchStep> {
    SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        let session = sessions.entry(host_id.to_owned()).or_default();
        if !session.is_active() {
            session.one_finger_orbit = profile.one_finger_orbit;
            session.long_press = profile.long_press;
        }
        match contact {
            Contact::Down { id, x, y } => session.down(now, id, x, y),
            Contact::Move { id, x, y } => session.moved(now, id, x, y),
            Contact::Up { id } => session.up(now, id),
            Contact::Cancel => session.cancel(),
        }
    })
}

/// ⏳️ The wall-clock time (milliseconds of [`app_now_ms`]) at which the earliest still finger of any surface turns into a long press, or `None` when no finger waits on the clock; the shell publishes it as a wake-up deadline, so an idle app sleeps until then and never polls.
pub(super) fn next_deadline_ms() -> Option<f64> {
    SESSIONS.with(|sessions| sessions.borrow().values().filter_map(|session| session.next_deadline_ms()).min_by(f64::total_cmp))
}

/// ⏰️ The wake-up deadline of the shell on the monotonic clock: the earlier of the chrome deadline (`chrome`, monotonic seconds) and the long-press deadline of a held finger (`touch_ms`, wall milliseconds, `None` without one), translated to seconds from the same `now`. A deadline that already passed is due `now`; a non-finite clock answers the chrome deadline untouched.
pub(super) fn merge_wake(chrome: Option<f64>, monotonic_now_seconds: f64, wall_now_ms: f64, touch_ms: Option<f64>) -> Option<f64> {
    let touch = touch_ms.filter(|_| monotonic_now_seconds.is_finite() && wall_now_ms.is_finite()).map(|due_ms| monotonic_now_seconds + (due_ms - wall_now_ms).max(0.0) / 1000.0);
    match (chrome, touch) {
        (Some(chrome), Some(touch)) => Some(chrome.min(touch)),
        (chrome, touch) => chrome.or(touch),
    }
}

/// ⏱️ The surfaces whose held finger reached the long-press deadline at `now`, with the steps their clock tick makes.
pub(super) fn due(now: f64) -> Vec<(String, Vec<TouchStep>)> {
    SESSIONS.with(|sessions| {
        sessions
            .borrow_mut()
            .iter_mut()
            .filter(|(_, session)| session.next_deadline_ms().is_some_and(|deadline| now >= deadline))
            .map(|(host_id, session)| (host_id.clone(), session.tick(now)))
            .collect()
    })
}

fn with_keys(base: &PointerModifiers, shift: bool, alt: bool) -> PointerModifiers {
    PointerModifiers { shift, alt, ..base.clone() }
}

/// 🖐️ The interaction intents one touch step stands for, at most two (a pinch is a wheel dolly and a shift-right-drag pan); a long press, tap or double tap is no intent but a gesture for the plugin, returned second.
pub(super) fn intents_of(step: &TouchStep, projection: OrbitProjection, height: f64, base: &PointerModifiers) -> ([Option<WorldInteractionIntent>; 2], Option<(ShellTouchGesture, f32, f32)>) {
    let mut intents = [None, None];
    let mut gesture = None;
    match *step {
        TouchStep::Press { x, y, additive } => intents[0] = Some(WorldInteractionIntent::pointer_button(x as f32, y as f32, true, 0, &with_keys(base, base.shift || additive, base.alt))),
        TouchStep::Drag { x, y, dx, dy, additive } => intents[0] = Some(WorldInteractionIntent::pointer_move(x as f32, y as f32, dx as f32, dy as f32, true, 0, &with_keys(base, base.shift || additive, base.alt))),
        TouchStep::Release { x, y, additive } => intents[0] = Some(WorldInteractionIntent::pointer_button(x as f32, y as f32, false, 0, &with_keys(base, base.shift || additive, base.alt))),
        TouchStep::Orbit { x, y, dx, dy } => intents[0] = Some(WorldInteractionIntent::pointer_move(x as f32, y as f32, dx as f32, dy as f32, true, 2, &with_keys(base, false, true))),
        TouchStep::Camera(pinch) => {
            let navigation = pinch_navigation(pinch, projection, height);
            if navigation.wheel != 0.0 {
                intents[0] = Some(WorldInteractionIntent::wheel(pinch.centroid_x as f32, pinch.centroid_y as f32, navigation.wheel as f32, &with_keys(base, false, false)));
            }
            if navigation.pan_x != 0.0 || navigation.pan_y != 0.0 {
                intents[1] = Some(WorldInteractionIntent::pointer_move(pinch.centroid_x as f32, pinch.centroid_y as f32, navigation.pan_x as f32, navigation.pan_y as f32, true, 2, &with_keys(base, true, false)));
            }
        }
        TouchStep::LongPress { x, y } => gesture = Some((ShellTouchGesture::LongPress, x as f32, y as f32)),
        TouchStep::Tap { x, y } => gesture = Some((ShellTouchGesture::Tap, x as f32, y as f32)),
        TouchStep::DoubleTap { x, y } => gesture = Some((ShellTouchGesture::DoubleTap, x as f32, y as f32)),
        TouchStep::CameraEnd => {}
    }
    (intents, gesture)
}

/// 🖐️ Applies touch steps to a `world-3d` surface: the intents go to the interaction authority exactly like a mouse's; a long press, tap or double tap is collected for the plugin.
pub(super) fn apply(state: &mut infinite_world::world::World3dState, steps: &[TouchStep], base: &PointerModifiers, gestures: &mut Vec<(ShellTouchGesture, f32, f32)>) -> Result<(), semio_ui_action::BoundedActionFault> {
    let height = f64::from(state.session.bounds.h);
    let projection = if state.camera.orbit.projection.is_parallel() { OrbitProjection::Orthographic { frustum_height: height } } else { OrbitProjection::Perspective { fov_y_radians: f64::from(state.camera.orbit.fov_y) } };
    for step in steps {
        let (intents, gesture) = intents_of(step, projection, height, base);
        gestures.extend(gesture);
        for intent in intents.into_iter().flatten() {
            enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).map_err(|_| semio_ui_action::BoundedActionFault::ItemCredits)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::interaction::gesture::PinchStep;

    fn phase(intent: &WorldInteractionIntent) -> String {
        format!("{:?}", intent.phase)
    }

    #[test]
    fn touch_steps_become_the_mouse_intents_the_authority_already_answers() {
        let base = PointerModifiers::default();
        let perspective = OrbitProjection::Perspective { fov_y_radians: 0.8 };
        let ([press, none], gesture) = intents_of(&TouchStep::Press { x: 10.0, y: 20.0, additive: true }, perspective, 600.0, &base);
        let press = press.expect("a press");
        assert!(none.is_none() && gesture.is_none());
        assert_eq!((phase(&press), press.down, press.button, press.shift, press.alt), ("PointerButton".to_owned(), true, 0, true, false), "a held finger is a shift press, the additive merge");
        let ([tap, _], _) = intents_of(&TouchStep::Press { x: 10.0, y: 20.0, additive: false }, perspective, 600.0, &base);
        assert!(!tap.expect("a press").shift, "a plain tap replaces the selection");
        let ([drag, _], _) = intents_of(&TouchStep::Drag { x: 12.0, y: 20.0, dx: 2.0, dy: 0.0, additive: false }, perspective, 600.0, &base);
        let drag = drag.expect("a drag");
        assert_eq!((phase(&drag), drag.down, drag.button, drag.dx), ("PointerMove".to_owned(), true, 0, 2.0));
        let ([release, _], _) = intents_of(&TouchStep::Release { x: 12.0, y: 20.0, additive: true }, perspective, 600.0, &base);
        let release = release.expect("a release");
        assert_eq!((phase(&release), release.down, release.shift), ("PointerButton".to_owned(), false, true));
        let ([orbit, _], _) = intents_of(&TouchStep::Orbit { x: 30.0, y: 40.0, dx: 4.0, dy: -3.0 }, perspective, 600.0, &PointerModifiers { shift: true, ..PointerModifiers::default() });
        let orbit = orbit.expect("an orbit");
        assert_eq!((phase(&orbit), orbit.button, orbit.alt, orbit.shift, orbit.dx, orbit.dy), ("PointerMove".to_owned(), 2, true, false, 4.0, -3.0), "the alt-right drag orbits, a held shift cannot turn it into a pan");
    }

    #[test]
    fn two_fingers_are_a_wheel_dolly_and_a_shift_right_drag_pan_in_the_units_of_the_web_orbit_rig() {
        let base = PointerModifiers::default();
        let pinch = PinchStep { scale: 2.0, pan_x: 12.0, pan_y: -6.0, rotation: 0.0, centroid_x: 300.0, centroid_y: 200.0 };
        let projection = OrbitProjection::Perspective { fov_y_radians: 45.0_f64.to_radians() };
        let expected = pinch_navigation(pinch, projection, 800.0);
        let ([wheel, pan], gesture) = intents_of(&TouchStep::Camera(pinch), projection, 800.0, &base);
        let (wheel, pan) = (wheel.expect("a dolly"), pan.expect("a pan"));
        assert!(gesture.is_none());
        assert_eq!((phase(&wheel), wheel.delta), ("Wheel".to_owned(), expected.wheel as f32));
        assert!(wheel.delta > 0.0, "spreading the fingers dollies in");
        assert_eq!((phase(&pan), pan.button, pan.shift, pan.alt, pan.dx, pan.dy), ("PointerMove".to_owned(), 2, true, false, expected.pan_x as f32, expected.pan_y as f32));
        let still = PinchStep { scale: 1.0, pan_x: 0.0, pan_y: 0.0, ..pinch };
        let ([none, other], _) = intents_of(&TouchStep::Camera(still), projection, 800.0, &base);
        assert!(none.is_none() && other.is_none(), "fingers that did not move do not move the camera");
        let ([end, _], _) = intents_of(&TouchStep::CameraEnd, projection, 800.0, &base);
        assert!(end.is_none());
    }

    #[test]
    fn a_long_press_a_tap_and_a_double_tap_are_gestures_for_the_plugin_and_no_intent() {
        let projection = OrbitProjection::Perspective { fov_y_radians: 0.8 };
        let ([none, _], long) = intents_of(&TouchStep::LongPress { x: 5.0, y: 6.0 }, projection, 600.0, &PointerModifiers::default());
        assert!(none.is_none());
        assert_eq!(long, Some((ShellTouchGesture::LongPress, 5.0, 6.0)));
        let (_, tap) = intents_of(&TouchStep::Tap { x: 3.0, y: 4.0 }, projection, 600.0, &PointerModifiers::default());
        assert_eq!(tap, Some((ShellTouchGesture::Tap, 3.0, 4.0)));
        let (_, double) = intents_of(&TouchStep::DoubleTap { x: 1.0, y: 2.0 }, projection, 600.0, &PointerModifiers::default());
        assert_eq!(double, Some((ShellTouchGesture::DoubleTap, 1.0, 2.0)));
    }

    #[test]
    fn the_long_press_deadline_is_a_wake_up_next_to_the_chrome_deadline_on_the_monotonic_clock() {
        assert_eq!(merge_wake(None, 100.0, 5_000.0, Some(5_450.0)), Some(100.45), "an idle app wakes 450 ms after the finger landed");
        assert_eq!(merge_wake(Some(100.2), 100.0, 5_000.0, Some(5_450.0)), Some(100.2), "an earlier chrome deadline still wins");
        assert_eq!(merge_wake(Some(101.0), 100.0, 5_000.0, Some(5_450.0)), Some(100.45), "a later chrome deadline yields");
        assert_eq!(merge_wake(None, 100.0, 5_000.0, Some(4_000.0)), Some(100.0), "an overdue deadline is due now");
        assert_eq!(merge_wake(Some(7.0), 100.0, 5_000.0, None), Some(7.0), "no finger leaves the chrome deadline alone");
        assert_eq!(merge_wake(None, 100.0, 5_000.0, None), None);
        assert_eq!(merge_wake(Some(7.0), f64::NAN, 5_000.0, Some(5_450.0)), Some(7.0), "a broken clock never invents a deadline");
    }

    #[test]
    fn each_surface_keeps_its_own_session_and_a_held_finger_is_due_after_the_long_press_deadline() {
        let profile = ShellTouchProfile::default();
        feed("a", profile, 0.0, Contact::Down { id: 1, x: 10.0, y: 10.0 });
        feed("b", profile, 0.0, Contact::Down { id: 7, x: 90.0, y: 90.0 });
        assert!(due(100.0).is_empty(), "nothing is due before the deadline");
        assert_eq!(next_deadline_ms(), Some(450.0), "an idle host sleeps until the earliest held finger is due");
        let mut hosts: Vec<String> = due(500.0).into_iter().map(|(host, steps)| {
            assert!(matches!(steps.as_slice(), [TouchStep::Press { additive: true, .. }, TouchStep::LongPress { .. }]));
            host
        }).collect();
        hosts.sort();
        assert_eq!(hosts, ["a", "b"]);
        assert!(due(900.0).is_empty(), "a pressed finger waits for nothing");
        assert_eq!(next_deadline_ms(), None, "no finger waits on the clock any more");
        assert!(matches!(feed("a", profile, 600.0, Contact::Up { id: 1 }).as_slice(), [TouchStep::Release { additive: true, .. }]), "the lift releases and does not ask for the menu again");
        feed("b", profile, 600.0, Contact::Cancel);
    }
}
