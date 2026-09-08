use super::*;
#[test]
fn nullable_selection_serde_text_and_binary_round_trip() {
    for value in [None, Some("null".into()), Some("".into()), Some("☃\n".into())] {
        let mutation = ChangeTestConfigSelection { selected: value };
        assert_eq!(serde_json::from_str::<ChangeTestConfigSelection>(&serde_json::to_string(&mutation).unwrap()).unwrap(), mutation);
        assert_eq!(ChangeTestConfigSelection::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(ChangeTestConfigSelection::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for raw in ["{}", "{\"selected\":1}", "{\"selected\":null,\"other\":true}"] {
        assert!(serde_json::from_str::<ChangeTestConfigSelection>(raw).is_err());
    }
}
#[test]
fn structural_config_diff_serde_preserves_identity_clear_and_set() {
    for diff in [TestConfigDiff::Identity, TestConfigDiff::Clear, TestConfigDiff::Set("next".into())] {
        assert_eq!(serde_json::from_str::<TestConfigDiff>(&serde_json::to_string(&diff).unwrap()).unwrap(), diff);
    }
}
