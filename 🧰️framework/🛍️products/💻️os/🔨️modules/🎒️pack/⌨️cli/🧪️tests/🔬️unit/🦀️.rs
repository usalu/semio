fn test_command_context()->crate::os_pack::control::CommandContext{let token=semio_framework_async::CancelToken::root_now();let transport=crate::os_pack::control::admit_command_transport(1048576,token.clone(),|_|{}).unwrap();crate::os_pack::control::CommandContext::try_new(transport,1048576,token,|_|{}).unwrap()}
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

//#region 🔖️Fixtures
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

async fn temp_path(name: &str) -> std::path::PathBuf {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("pack_cli_test_{}_{counter}_{name}", std::process::id()))
}

fn sample_record(name: &str, age: u64, active: bool) -> semio_framework_dsl_record::RecordValue {
    let mut fields = semio_framework_dsl_record::RecordFields::default();
    fields.insert(1, semio_framework_dsl_record::FieldValue::Text(name.to_string()));
    fields.insert(2, semio_framework_dsl_record::FieldValue::UInt(age));
    fields.insert(3, semio_framework_dsl_record::FieldValue::Bool(active));
    semio_framework_dsl_record::RecordValue { fields }
}

fn sample_pack_bytes(name: &str, age: u64, active: bool) -> Vec<u8> {
    let spec = sample_spec();
    let record = sample_record(name, age, active);
    crate::os_pack::record::encode_document(&spec, &record, &crate::os_pack::record::EncodeOptions::default()).unwrap()
}
//#endregion 🔖️Fixtures

//#region 🔖️Inspect
#[semio_framework_async_macros::async_test]
async fn cli_inspect_verify_hash_on_valid_pack() {
    let bytes = sample_pack_bytes("Ada Lovelace", 42, true);
    let path = temp_path("valid.spk").await;
    std::fs::write(&path, &bytes).unwrap();
    let path_str = path.to_string_lossy().to_string();

    assert_eq!(main_impl(&[String::from("inspect"), path_str.clone()], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), path_str.clone()], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("verify"), path_str.clone(), String::from("--level=full")], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("hash"), path_str], &test_command_context()).await, 0);

    std::fs::remove_file(&path).ok();
}
//#endregion 🔖️Inspect

//#region 🔖️Corrupt
#[semio_framework_async_macros::async_test]
async fn cli_verify_fails_on_corrupted_pack_without_panicking() {
    let mut bytes = sample_pack_bytes("Grace Hopper", 85, false);
    let mid = bytes.len() / 2;
    bytes[mid] ^= 0xFF;
    let path = temp_path("corrupt.spk").await;
    std::fs::write(&path, &bytes).unwrap();
    let path_str = path.to_string_lossy().to_string();

    assert_ne!(main_impl(&[String::from("verify"), path_str.clone(), String::from("--level=full")], &test_command_context()).await, 0);
    let inspect_code = main_impl(&[String::from("inspect"), path_str.clone()], &test_command_context()).await;
    assert!(inspect_code == 0 || inspect_code == 1);
    let hash_code = main_impl(&[String::from("hash"), path_str], &test_command_context()).await;
    assert!(hash_code == 0 || hash_code == 1);

    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_handles_truncated_pack_without_panicking() {
    let bytes = sample_pack_bytes("Alan Turing", 41, true);
    let truncated = &bytes[..bytes.len() / 2];
    let path = temp_path("truncated.spk").await;
    std::fs::write(&path, truncated).unwrap();
    let path_str = path.to_string_lossy().to_string();

    assert_eq!(main_impl(&[String::from("verify"), path_str.clone()], &test_command_context()).await, 1);
    assert_eq!(main_impl(&[String::from("inspect"), path_str.clone()], &test_command_context()).await, 1);
    assert_eq!(main_impl(&[String::from("hash"), path_str], &test_command_context()).await, 1);

    std::fs::remove_file(&path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_reports_missing_file_without_panicking() {
    let missing = temp_path("does-not-exist.spk").await.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("inspect"), missing.clone()], &test_command_context()).await, 1);
    assert_eq!(main_impl(&[String::from("verify"), missing.clone()], &test_command_context()).await, 1);
    assert_eq!(main_impl(&[String::from("hash"), missing], &test_command_context()).await, 1);
}
//#endregion 🔖️Corrupt

//#region 🔖️Dsl
#[semio_framework_async_macros::async_test]
async fn cli_to_dsl_and_from_dsl_round_trip_via_registry() {
    let bytes = sample_pack_bytes("Ada Lovelace", 42, true);
    let path = temp_path("roundtrip.spk").await;
    std::fs::write(&path, &bytes).unwrap();
    let path_str = path.to_string_lossy().to_string();

    assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone(), String::from("--schema"), String::from("sample")], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone(), String::from("--schema"), String::from("bogus")], &test_command_context()).await, 2);
    assert_eq!(main_impl(&[String::from("to-dsl"), path_str.clone()], &test_command_context()).await, 2);

    let dsl_path = temp_path("roundtrip.dsl").await;
    let spec = sample_spec();
    let record = sample_record("Grace Hopper", 7, false);
    let mut writer = semio_framework_dsl_record::Writer::new();
    semio_framework_dsl_record::print_record(&record, &spec, &mut writer);
    std::fs::write(&dsl_path, writer.render(semio_framework_dsl_record::JoinMode::Document)).unwrap();
    let dsl_path_str = dsl_path.to_string_lossy().to_string();

    let out_path = temp_path("fromdsl.spk").await;
    let out_path_str = out_path.to_string_lossy().to_string();
    assert_eq!(main_impl(&[String::from("from-dsl"), dsl_path_str, String::from("--schema"), String::from("sample"), String::from("--out"), out_path_str.clone(),], &test_command_context()).await, 0);
    assert!(out_path.exists());
    assert_eq!(main_impl(&[String::from("verify"), out_path_str.clone()], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("diff"), path_str.clone(), out_path_str.clone(), String::from("--schema"), String::from("sample")], &test_command_context()).await, 1);
    assert_eq!(main_impl(&[String::from("diff"), path_str.clone(), path_str.clone()], &test_command_context()).await, 0);

    std::fs::remove_file(&path).ok();
    std::fs::remove_file(&out_path).ok();
}

#[semio_framework_async_macros::async_test]
async fn cli_from_dsl_reports_parse_failure_without_panicking() {
    let bad_dsl_path = temp_path("bad.dsl").await;
    std::fs::write(&bad_dsl_path, "name=").unwrap();
    let out_path = temp_path("bad-out.spk").await;
    assert_eq!(main_impl(&[String::from("from-dsl"), bad_dsl_path.to_string_lossy().to_string(), String::from("--schema"), String::from("sample"), String::from("--out"), out_path.to_string_lossy().to_string(),], &test_command_context()).await, 1);
    assert!(!out_path.exists());

    std::fs::remove_file(&bad_dsl_path).ok();
}
//#endregion 🔖️Dsl

//#region 🔖️Cli
#[semio_framework_async_macros::async_test]
async fn cli_help_and_unknown_subcommand() {
    assert_eq!(main_impl(&[], &test_command_context()).await, 2);
    assert_eq!(main_impl(&[String::from("help")], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("--help")], &test_command_context()).await, 0);
    assert_eq!(main_impl(&[String::from("bogus-subcommand")], &test_command_context()).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn cli_parse_args_splits_flags_and_positionals() {
    let args = vec![String::from("a.spk"), String::from("--level=full"), String::from("--schema"), String::from("sample"), String::from("b.spk")];
    let (positional, flags) = parse_args(&args).await;
    assert_eq!(positional, vec!["a.spk".to_string(), "b.spk".to_string()]);
    assert_eq!(flags.get("level"), Some(&"full".to_string()));
    assert_eq!(flags.get("schema"), Some(&"sample".to_string()));
}
//#endregion 🔖️Cli
