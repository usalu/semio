
use semio_framework_plugin_reactor_core::package_asset;

#[test]
fn an_asset_the_host_delivered_on_open_is_readable_by_name() {
    admit_package_assets(vec![("package-assets-test/first.bin".to_string(), vec![1, 2, 3]), ("package-assets-test/second.bin".to_string(), vec![9])]);
    assert_eq!(package_asset("package-assets-test/first.bin"), Some(&[1u8, 2, 3][..]));
    assert_eq!(package_asset("package-assets-test/second.bin"), Some(&[9u8][..]));
}

#[test]
fn an_asset_the_host_never_delivered_reads_as_absent() {
    assert_eq!(package_asset("package-assets-test/never-delivered.bin"), None);
}

#[test]
fn a_later_open_repeating_a_name_keeps_the_first_bytes_and_new_names_are_added() {
    admit_package_assets(vec![("package-assets-test/repeat.bin".to_string(), vec![7, 7])]);
    admit_package_assets(vec![("package-assets-test/repeat.bin".to_string(), vec![0]), ("package-assets-test/added.bin".to_string(), vec![5])]);
    assert_eq!(package_asset("package-assets-test/repeat.bin"), Some(&[7u8, 7][..]));
    assert_eq!(package_asset("package-assets-test/added.bin"), Some(&[5u8][..]));
}
