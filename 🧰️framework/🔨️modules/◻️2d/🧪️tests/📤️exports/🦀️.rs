//! 📤️ The public facade retains the same implemented types used by real geometry consumers.

#[test]
fn declared_exports_retain_internal_type_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📤️exports/🔣️.json")).unwrap();
    assert_eq!(fixture["package"], env!("CARGO_PKG_NAME"));
    assert_eq!(fixture["modules"].as_array().unwrap().len(), 8);
    assert_eq!(std::any::TypeId::of::<crate::PathSegment>(), std::any::TypeId::of::<semio_framework_2d_engine::PathSegment>());
    assert_eq!(std::any::TypeId::of::<crate::DrawingError>(), std::any::TypeId::of::<semio_framework_2d_engine::DrawingError>());
    let point: crate::Vec2 = [1.0, 2.0];
    let direct: semio_framework_2d_engine::Vec2 = point;
    assert_eq!(direct, [1.0, 2.0]);
    let _ = std::any::TypeId::of::<crate::compute::EngineHandles>();
    let _ = std::any::TypeId::of::<crate::flatten::PathFlattenJob>();
    let _ = std::any::TypeId::of::<crate::stroke::StrokeOutlineJob>();
    let _ = std::any::TypeId::of::<crate::booleans::BooleanJob>();
    let _ = std::any::TypeId::of::<crate::booleans::paths::PathBooleanJob>();
    let _ = std::any::TypeId::of::<crate::trace::BitmapTraceJob<Vec<u8>>>();
    let _ = std::any::TypeId::of::<crate::regions::Region>();
    assert_eq!(crate::text::drawing_text_lines("first\nsecond").collect::<Vec<_>>(), ["first", "second"]);
    assert!(crate::retirement::WorkRetirementCounter::default().advance(1, || true).unwrap().done);
}
