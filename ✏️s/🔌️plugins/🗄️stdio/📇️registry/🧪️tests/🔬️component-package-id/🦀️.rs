
#[test]
fn component_package_identity_comes_from_the_canonical_cargo_contract() {
    assert_eq!(super::component_package_id().expect("stdio component package identity"), "semio:stdio");
}
