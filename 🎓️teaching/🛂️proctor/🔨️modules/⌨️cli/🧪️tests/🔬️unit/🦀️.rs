use super::*;
use crate::catalog::tests::fixture_path;
use crate::storage::tests::scratch;

fn code(exit: ExitCode) -> String {
    format!("{exit:?}")
}

#[test]
fn check_accepts_the_fixture_and_refuses_a_broken_catalog() {
    assert_eq!(code(check(&fixture_path())), code(ExitCode::SUCCESS));
    let directory = scratch("cli-check");
    std::fs::write(directory.0.join("catalog.json"), "{\"schema\":\"semio.quiz.catalog/v1\"}").unwrap();
    assert_eq!(code(check(&directory.0.join("catalog.json"))), code(ExitCode::FAILURE));
}

#[tokio::test]
async fn a_wrong_command_line_prints_usage_and_exits_two() {
    let nothing = |_: &str| None;
    assert_eq!(code(run(&[], nothing).await), code(ExitCode::from(2)));
    assert_eq!(code(run(&["check".to_string()], nothing).await), code(ExitCode::from(2)));
    assert_eq!(code(run(&["serve".to_string(), "now".to_string()], nothing).await), code(ExitCode::from(2)));
    assert_eq!(code(run(&["serve".to_string()], nothing).await), code(ExitCode::FAILURE));
}

#[tokio::test]
async fn rebuild_refolds_a_data_directory() {
    let directory = scratch("cli-rebuild");
    let data = directory.0.to_string_lossy().into_owned();
    let catalog = fixture_path().to_string_lossy().into_owned();
    let environment = move |name: &str| match name {
        "PROCTOR_DATA" => Some(data.clone()),
        "PROCTOR_CATALOG" => Some(catalog.clone()),
        _ => None,
    };
    assert_eq!(code(run(&["rebuild".to_string()], &environment).await), code(ExitCode::SUCCESS));
    assert!(directory.0.join(crate::storage::DATABASE_FILE).is_file());
}

#[test]
fn progress_is_announced_in_ten_percent_steps() {
    let mut reported = 0;
    let steps: Vec<u64> = [5, 12, 19, 40, 41, 100]
        .into_iter()
        .filter_map(|position| {
            let before = reported;
            announce("test", Progress { position, head: 100, folded: position }, &mut reported);
            (reported != before).then_some(reported)
        })
        .collect();
    assert_eq!(steps, [12, 40, 100]);
}
