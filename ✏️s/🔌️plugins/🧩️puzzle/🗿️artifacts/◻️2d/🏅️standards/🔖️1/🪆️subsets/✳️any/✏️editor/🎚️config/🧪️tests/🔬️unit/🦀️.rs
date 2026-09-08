
use super::*;

fn fixed_fill_runtime_contract(source: &str) -> bool {
    let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
    let Some(start) = production.find("pub struct Puzzle2dFillText") else { return false };
    let Some(end_relative) = production[start..].find("//#endregion 🧵️FillLifecycle") else { return false };
    let runtime = &production[start..start + end_relative];
    let Some(mutation_start) = production.find("pub enum Puzzle2dConfigMutation") else { return false };
    let mutation = &production[mutation_start..];
    runtime.contains("bytes: [u8; PUZZLE2D_FILL_TEXT_CAPACITY]")
        && runtime.contains("pub struct Puzzle2dFillRuntime")
        && runtime.contains("pub fill_job_stage: Puzzle2dFillText")
        && runtime.contains("pub fill_job_fault_code: Option<Puzzle2dFillText>")
        && !runtime.contains("Vec<")
        && !runtime.contains("BTreeMap")
        && !runtime.contains("String")
        && mutation.contains("Fill { runtime: Puzzle2dFillRuntime }")
        && mutation.contains("Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime::from_config(base) }")
}

/// 🧱️ Fill text owns exactly its fixed admitted backing at MAX and rejects MAX+1.
#[test]
fn fill_text_capacity_is_exact() {
    let maximum = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY);
    let over = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY + 1);
    let Some(text) = Puzzle2dFillText::try_from_str(&maximum) else { panic!("MAX fill text must fit") };
    assert_eq!(text.as_str(), maximum);
    assert!(Puzzle2dFillText::try_from_str(&over).is_none());
}

/// 🧬️ Dynamic fill runtime backing or whole-snapshot fill inverse mutations fail the source law.
#[test]
fn dynamic_fill_runtime_mutations_are_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(fixed_fill_runtime_contract(source));
    let dynamic = source.replacen("pub fill_job_stage: Puzzle2dFillText,", "pub brush_candidates: Vec<Value>, pub fill_job_stage: String,", 1);
    assert!(!fixed_fill_runtime_contract(&dynamic));
    let snapshot = source.replacen("Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime::from_config(base) }", "Puzzle2dConfigMutation::Snapshot { config: base.clone() }", 1);
    assert!(!fixed_fill_runtime_contract(&snapshot));
}
