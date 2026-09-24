
use super::*;

#[test]
fn resolves_longest_multi_word_alias() {
    let catalog = vec![PlaygroundEntry { variant: "puzzle2d".into(), aliases: vec!["puzzle 2d".into(), "2d".into()], ..Default::default() }, PlaygroundEntry { variant: "puzzle3d".into(), aliases: vec!["puzzle 3d".into()], ..Default::default() }];
    let segments = ["puzzle".to_string(), "3d".to_string(), "fixture".to_string(), "concrete".to_string()];
    let (row, rest) = resolve_playground(&catalog, &segments).expect("resolves");
    assert_eq!(row.variant, "puzzle3d");
    assert_eq!(rest, vec!["fixture", "concrete"]);
}

#[test]
fn leaves_unknown_catalog_result_unresolved() {
    let catalog = vec![PlaygroundEntry { variant: "puzzle2d".into(), ..Default::default() }];
    assert!(resolve_playground(&catalog, &["unregistered".into()]).is_none());
}
