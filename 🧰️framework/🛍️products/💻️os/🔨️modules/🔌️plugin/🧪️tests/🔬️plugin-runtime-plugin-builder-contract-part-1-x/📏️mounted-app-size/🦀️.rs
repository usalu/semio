/// 📏️ The mounted app handle stays within its recorded budget, so no thread stack or future holds a megabyte; every larger owner is a separate heap allocation.
#[test]
fn mounted_app_handle_stays_within_the_recorded_size_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️mounted-app-size/🔣️.json")).expect("size budget fixture");
    let ceiling = fixture["stackCeilingBytes"].as_u64().expect("stack ceiling") as usize;
    let measured = [
        ("composedParentVcsApp", std::mem::size_of::<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>>()),
        ("composedParentApp", std::mem::size_of::<ComposedParentApp>()),
    ];
    for (id, actual) in measured {
        let budget = fixture["budgets"].as_array().expect("budgets").iter().find(|row| row["id"] == id).unwrap_or_else(|| panic!("{id} has no recorded budget"))["maxBytes"].as_u64().expect("budget bytes") as usize;
        assert!(budget <= ceiling, "{id} budget {budget} exceeds the {ceiling} byte stack ceiling");
        assert!(actual <= budget, "{id} grew to {actual} bytes, budget {budget}");
    }
    let retained = VcsArtifactApp::<ComposedParentApp, RecursiveTestMembers>::heap_bytes();
    assert!(retained > std::mem::size_of::<VcsArtifactApp<ComposedParentApp, RecursiveTestMembers>>(), "boxed owners are accounted as retained heap bytes");
}
