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
