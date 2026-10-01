use super::*;

#[test]
fn casings_cover_upper_title_and_lower() {
    assert_eq!(apply_rename_casings("MODEL Model model", "model", "representation"), "REPRESENTATION Representation representation");
}

#[test]
fn rename_moves_deepest_paths_first() {
    let workspace = Workspace::new().with_file("model/model.ts", "const model = 1;\n").with_directory("model");
    let plan = plan_rename(&workspace, "model", "shape", "").expect("plan");
    let moves: Vec<&Change> = plan.changes.iter().filter(|c| matches!(c, Change::Move { .. })).collect();
    assert_eq!(moves.first(), Some(&&Change::Move { from: "model/model.ts".to_string(), to: "model/shape.ts".to_string() }));
    assert_eq!(moves.last(), Some(&&Change::Move { from: "model".to_string(), to: "shape".to_string() }));
}

#[test]
fn workspace_applies_a_directory_move() {
    let mut workspace = Workspace::new().with_file("a/b/c.txt", "x");
    workspace.apply(&Change::Move { from: "a".to_string(), to: "z".to_string() }).expect("move");
    assert_eq!(workspace.file("z/b/c.txt"), Some("x"));
    assert!(!workspace.exists("a/b/c.txt"));
}

#[test]
fn section_move_rewrites_both_markers() {
    let content = "// #region 🔖️Alpha\nconst a = 1;\n// #endregion 🔖️Alpha\n";
    let workspace = Workspace::new().with_file("x.ts", content);
    let plan = plan_section_move(&workspace, "x.ts", "Alpha", "Beta").expect("plan");
    match &plan.changes[0] {
        Change::Write { content, .. } => assert_eq!(content, "// #region 🔖️Beta\nconst a = 1;\n// #endregion 🔖️Beta\n"),
        other => panic!("unexpected change {other:?}"),
    }
}

fn sealed_workspace(sealed: &[&str]) -> Workspace {
    let mut contracts = vec![r#""gone":{"path":"history/retired.json","retired":{"ticket":"2026/01/01/SEAL-PROBE","reason":"ticket-close-generated-output-removed"}}"#.to_string()];
    contracts.extend(sealed.iter().enumerate().map(|(index, path)| format!(r#""sealed-{index}":{{"path":"{path}"}}"#)));
    let taxonomy = format!(r#"{{"frozenCoordinateEvidenceContracts":{{{}}},"frozenMarkdownCoordinateEvidenceContracts":{{}}}}"#, contracts.join(","));
    Workspace::new()
        .with_file(SEALED_TAXONOMY_PATH, &taxonomy)
        .with_file("history/sealed.json", "model\n")
        .with_file("history/retired.json", "model\n")
        .with_file("model/model.ts", "model\n")
}

#[test]
fn rename_keeps_sealed_evidence_bytes() {
    let mut workspace = sealed_workspace(&["history/sealed.json"]);
    let plan = plan_rename(&workspace, "model", "shape", "history").expect("plan");
    assert_eq!((plan.stats["filesChanged"], plan.stats["filesSealed"]), (1, 1));
    execute(&plan, &mut workspace).expect("apply");
    assert_eq!((workspace.file("history/sealed.json"), workspace.file("history/retired.json")), (Some("model\n"), Some("shape\n")));
}

#[test]
fn rename_refuses_to_relocate_sealed_evidence() {
    let workspace = sealed_workspace(&["model/model.ts"]);
    assert_eq!(plan_rename(&workspace, "model", "shape", ""), Err(MoveError("Rename would relocate digest-sealed evidence: model/model.ts".to_string())));
}

#[test]
fn moves_refuse_sealed_evidence() {
    let workspace = sealed_workspace(&["history/sealed.json"]);
    let refusal = Err(MoveError("Refusing to move digest-sealed evidence: history/sealed.json".to_string()));
    assert_eq!(plan_folder_move(&workspace, "history", "archive"), refusal);
    assert_eq!(plan_file_move(&workspace, "history/sealed.json", "history/moved.json"), refusal);
    assert!(plan_file_move(&workspace, "history/retired.json", "history/moved.json").is_ok());
}
