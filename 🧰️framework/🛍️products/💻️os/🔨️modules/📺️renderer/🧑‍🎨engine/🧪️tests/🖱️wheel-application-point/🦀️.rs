//! 🎡️ Admitted wheels retain their samples through the production host and runtime cursor.
use serde_json::Value;
use ui_render::{DispatchEvent, EventModifiers, PointerId, PointerInfo, PointerKind};
const FIXTURE: &str = include_str!("../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");
fn number(value: &Value) -> f32 { value.as_f64().expect("fixture number") as f32 }
fn modifiers(value: &Value) -> EventModifiers {
    EventModifiers {
        shift: value["shift"].as_bool().unwrap_or(false), ctrl: value["ctrl"].as_bool().unwrap_or(false),
        alt: value["alt"].as_bool().unwrap_or(false), meta: value["meta"].as_bool().unwrap_or(false),
    }
}
#[test]
fn every_admitted_wheel_reaches_dispatch_at_its_original_point() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut queue = ui_host::EventQueue::new();
        let ui = ui_host::UiThreadToken::mint_for_host();
        for event in case["events"].as_array().unwrap() {
            let (x, y) = (number(&event["x"]), number(&event["y"]));
            let modifiers = modifiers(&event["modifiers"]);
            let event = match event["kind"].as_str().unwrap() {
                "scroll" => DispatchEvent::Scroll { x, y, delta_x: event["deltaX"].as_f64().unwrap_or(0.0) as f32, delta_y: number(&event["deltaY"]), modifiers },
                "pointer-move" => DispatchEvent::PointerMove { pointer: PointerInfo { id: PointerId(1), kind: PointerKind::Mouse, pressure: None, tilt: None }, x, y, modifiers },
                other => panic!("unknown fixture event {other}"),
            };
            assert_eq!(queue.enqueue(ui, event), ui_host::EnqueueOutcome::Accepted, "{}", case["name"]);
        }
        let mut actual = Vec::new();
        while !queue.is_empty() {
            let page = queue.drain_page(ui_host::WorkerContext::new(queue.current_generation()));
            let mut cursor = super::RuntimeDispatchCursor::new_for_generation(page, 1);
            while let Some(event) = cursor.take_next() {
                if let DispatchEvent::Scroll { x, y, delta_x, delta_y, modifiers } = event { actual.push((x, y, delta_x, delta_y, modifiers)); }
            }
            assert!(cursor.terminal_is_empty());
        }
        let expected: Vec<_> = case["applications"].as_array().unwrap().iter().map(|event| (
            number(&event["x"]), number(&event["y"]), event["deltaX"].as_f64().unwrap_or(0.0) as f32, number(&event["delta"]), modifiers(&event["modifiers"]),
        )).collect();
        assert_eq!(actual, expected, "{}", case["name"]);
        assert!(queue.close_step() || queue.close_step());
        assert!(queue.terminal_is_empty());
        println!("[DEBUG] wheel dispatch {} preserved {} original samples", case["name"], actual.len());
    }
}
