//! 🧪️ Compares borrowed paged list field projections with a Serde array.

use super::*;
use native_encoding::FieldProjectionView;
use semio_framework_value_persistent::list::PagedList;

#[test]
fn paged_list_projects_original_elements_without_conversion() {
    let fixture = semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗣️dsl/🧪️tests/🧫️fixtures/🪆️binding/📋️paged-list/🔣️.json");
    let expected: Vec<String> = serde_json::from_str(fixture).unwrap();
    let list: PagedList<String, 16> = expected.clone().into();
    let FieldProjectionView::List(length) = list.projection_view(&[]).unwrap() else {
        panic!("expected a borrowed list");
    };
    let actual = (0..length).map(|index| {
        let FieldProjectionView::Text(value) = list.projection_view(&[index]).unwrap() else {
            panic!("expected borrowed text");
        };
        assert_eq!(value.as_ptr(), list.get(index).unwrap().as_ptr());
        value
    }).collect::<Vec<_>>();
    assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(expected).unwrap());
    assert!(list.projection_view(&[length]).is_err());
    assert!(list.projection_view(&[0, 0]).is_err());
}
