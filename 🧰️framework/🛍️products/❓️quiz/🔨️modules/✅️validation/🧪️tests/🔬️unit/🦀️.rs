//! 🧐️ Unit tests of the validation: issue paths and codes, answer validity and completeness, schema violations.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;
use crate::schema::{BadgeRule, ClassificationAnswer, Introduction, LeaderboardPeriod, MatchingAnswer, SheetClassificationTask, SheetDimension, SheetMatchingTask, SheetSortingTask, SortingAnswer, TaskKind};
use crate::sheet::tests::{quantity, quiz, text};
use unicode_normalization::{is_nfc, UnicodeNormalization};

fn issue(path: &str, code: IssueCode) -> ValidationIssue {
    ValidationIssue { path: path.to_string(), code }
}

fn badge(id: &str, rule: BadgeRule) -> Badge {
    Badge { id: id.to_string(), emoji: "🏅".to_string(), label: text(id), description: text(id), rule }
}

fn catalog() -> Catalog {
    Catalog {
        json_schema: None,
        schema: CATALOG_SCHEMA.to_string(),
        id: "architecture".to_string(),
        title: text("Architecture"),
        introduction: Introduction { title: text("Welcome"), paragraphs: vec![text("Hello")] },
        quizzes: vec!["energy/🔣️.json".to_string()],
        badges: vec![badge("perfect-energy", BadgeRule::PerfectQuiz { quiz: "energy".to_string() }), badge("sorter", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Sorting), quiz: None }), badge("done", BadgeRule::CompletedQuizzes)],
    }
}

#[test]
fn slugs_follow_the_schema_pattern() {
    for valid in ["a", "a-b", "0-9", "abc-123-x", &"a".repeat(64)] {
        assert!(is_slug(valid), "{valid}");
    }
    for invalid in ["", "-a", "a-", "a--b", "A", "a_b", "ä", "a b", &"a".repeat(65)] {
        assert!(!is_slug(invalid), "{invalid}");
    }
    assert_eq!(pointer("/profile", "a/~b"), "/profile/a~1~0b");
}

#[test]
fn the_reference_quiz_and_catalog_are_clean() {
    assert_eq!(quiz_issues(&quiz()), []);
    assert_eq!(catalog_issues(&catalog(), &[quiz()]), []);
}

#[test]
fn quiz_structure_issues_are_reported_by_pointer() {
    let mut quiz = quiz();
    quiz.schema = "semio.quiz/v2".to_string();
    quiz.id = "Energy".to_string();
    quiz.emoji = String::new();
    quiz.title.de = String::new();
    let Task::Classification(classification) = &mut quiz.tasks[0] else { unreachable!() };
    classification.draw = Some(9);
    if let Some(axes) = classification.axes.as_mut() {
        axes.truncate(2);
    }
    let Task::Sorting(sorting) = &mut quiz.tasks[1] else { unreachable!() };
    sorting.draw = Some(1);
    sorting.quantity.unit = "x".repeat(33);
    sorting.items[1].id = "s0".to_string();
    let Task::Matching(matching) = &mut quiz.tasks[2] else { unreachable!() };
    matching.items.truncate(1);
    matching.dimensions[0].quantity = quantity("", Scale::Linear);
    assert_eq!(
        quiz_issues(&quiz),
        [
            issue("/emoji", IssueCode::LengthInvalid),
            issue("/id", IssueCode::SlugInvalid),
            issue("/schema", IssueCode::ValueInvalid),
            issue("/tasks/0/axes", IssueCode::ItemsTooFew),
            issue("/tasks/0/categories/0/profile/cost", IssueCode::AxisUnknown),
            issue("/tasks/0/categories/1/profile/cost", IssueCode::AxisUnknown),
            issue("/tasks/0/categories/2/profile/cost", IssueCode::AxisUnknown),
            issue("/tasks/0/draw", IssueCode::DrawExceedsItems),
            issue("/tasks/1/draw", IssueCode::BelowMinimum),
            issue("/tasks/1/items/1/id", IssueCode::DuplicateId),
            issue("/tasks/1/quantity/unit", IssueCode::LengthInvalid),
            issue("/tasks/2/dimensions/0/quantity/label/en", IssueCode::LengthInvalid),
            issue("/tasks/2/dimensions/0/quantity/unit", IssueCode::LengthInvalid),
            issue("/tasks/2/items", IssueCode::ItemsTooFew),
            issue("/title/de", IssueCode::LengthInvalid),
        ]
    );
}

#[test]
fn quiz_semantic_issues_are_reported_by_pointer() {
    let mut quiz = quiz();
    quiz.tasks.push(quiz.tasks[1].clone());
    let Task::Classification(classification) = &mut quiz.tasks[0] else { unreachable!() };
    classification.items[3].category = "future".to_string();
    classification.items[2].category = "Old".to_string();
    let axes = classification.axes.as_deref_mut().unwrap_or_default();
    axes[1].max = axes[1].min;
    let profile = classification.categories[0].profile.get_or_insert_with(BTreeMap::new);
    profile.remove("heat");
    profile.insert("light/~x".to_string(), 1.0);
    profile.insert("cost".to_string(), 11.0);
    classification.categories[1].profile = Some(BTreeMap::new());
    let Task::Sorting(sorting) = &mut quiz.tasks[1] else { unreachable!() };
    sorting.items[0].value = 0.0;
    let Task::Matching(matching) = &mut quiz.tasks[2] else { unreachable!() };
    matching.items[0].values.remove("load");
    matching.items[1].values.insert("demand".to_string(), -1.0);
    matching.items[2].values.insert("area".to_string(), 3.0);
    assert_eq!(
        quiz_issues(&quiz),
        [
            issue("/tasks/0/axes/1/max", IssueCode::AxisRangeInvalid),
            issue("/tasks/0/categories/0/profile/cost", IssueCode::ProfileOutOfRange),
            issue("/tasks/0/categories/0/profile/heat", IssueCode::ProfileIncomplete),
            issue("/tasks/0/categories/0/profile/light~1~0x", IssueCode::AxisUnknown),
            issue("/tasks/0/categories/0/profile/light~1~0x", IssueCode::SlugInvalid),
            issue("/tasks/0/categories/1/profile", IssueCode::PropertiesTooFew),
            issue("/tasks/0/categories/1/profile/cool", IssueCode::ProfileIncomplete),
            issue("/tasks/0/categories/1/profile/cost", IssueCode::ProfileIncomplete),
            issue("/tasks/0/categories/1/profile/heat", IssueCode::ProfileIncomplete),
            issue("/tasks/0/items/2/category", IssueCode::SlugInvalid),
            issue("/tasks/0/items/3/category", IssueCode::CategoryUnknown),
            issue("/tasks/1/items/0/value", IssueCode::ValueNotPositive),
            issue("/tasks/2/items/0/values/load", IssueCode::ValueMissing),
            issue("/tasks/2/items/1/values/demand", IssueCode::ValueNotPositive),
            issue("/tasks/2/items/2/values/area", IssueCode::DimensionUnknown),
            issue("/tasks/3/id", IssueCode::DuplicateId),
        ]
    );
}

#[test]
fn profiles_without_axes_report_missing_axes_only() {
    let mut quiz = quiz();
    let Task::Classification(classification) = &mut quiz.tasks[0] else { unreachable!() };
    classification.axes = None;
    assert_eq!(quiz_issues(&quiz), (0..3).map(|index| issue(&format!("/tasks/0/categories/{index}/profile"), IssueCode::AxesMissing)).collect::<Vec<_>>());
}

#[test]
fn quiz_emoji_is_required_and_holds_one_to_sixteen_code_points() {
    let with = |emoji: &str| quiz_issues(&Quiz { emoji: emoji.to_string(), ..quiz() });
    for valid in ["⚡", "❄️", "👨‍👩‍👧‍👦", &"x".repeat(16)] {
        assert_eq!(with(valid), [], "{valid}");
    }
    for invalid in ["", &"🔥".repeat(17)] {
        assert_eq!(with(invalid), [issue("/emoji", IssueCode::LengthInvalid)], "{invalid}");
    }
    let mut document = serde_json::to_value(quiz()).unwrap_or_default();
    if let Some(object) = document.as_object_mut() {
        object.remove("emoji");
    }
    assert!(serde_json::from_value::<Quiz>(document).is_err());
}

#[test]
fn catalog_issues_cover_paths_quizzes_and_badges() {
    let mut catalog = catalog();
    catalog.schema = "semio.quiz/v1".to_string();
    catalog.introduction.paragraphs.clear();
    catalog.quizzes.push("energy/🔣️.json".to_string());
    catalog.quizzes.push(String::new());
    catalog.badges.push(badge("sorter", BadgeRule::PerfectQuiz { quiz: "heating".to_string() }));
    catalog.badges.push(badge("matcher", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Matching), quiz: Some("cooling".to_string()) }));
    catalog.badges.push(badge("none", BadgeRule::PerfectTasks { task_kind: Some(TaskKind::Classification), quiz: Some("other".to_string()) }));
    catalog.badges.push(badge("bad", BadgeRule::PerfectQuiz { quiz: "Heating".to_string() }));
    catalog.badges[0].emoji = String::new();
    let mut other = quiz();
    other.id = "other".to_string();
    other.tasks.remove(0);
    assert_eq!(
        catalog_issues(&catalog, &[quiz(), other, quiz(), quiz()]),
        [
            issue("/badges/0/emoji", IssueCode::LengthInvalid),
            issue("/badges/3/id", IssueCode::DuplicateId),
            issue("/badges/3/rule/quiz", IssueCode::QuizUnknown),
            issue("/badges/4/rule/quiz", IssueCode::QuizUnknown),
            issue("/badges/5/rule", IssueCode::BadgeUnreachable),
            issue("/badges/6/rule/quiz", IssueCode::SlugInvalid),
            issue("/introduction/paragraphs", IssueCode::ItemsTooFew),
            issue("/quizzes", IssueCode::QuizCountMismatch),
            issue("/quizzes/1", IssueCode::DuplicatePath),
            issue("/quizzes/2", IssueCode::DuplicateId),
            issue("/quizzes/2", IssueCode::LengthInvalid),
            issue("/quizzes/3", IssueCode::DuplicateId),
            issue("/schema", IssueCode::ValueInvalid),
        ]
    );
}

#[test]
fn issues_serialize_as_path_and_kebab_code() {
    let json = serde_json::to_string(&issue("/tasks/0/id", IssueCode::ValueNotPositive)).unwrap_or_default();
    assert_eq!(json, r#"{"path":"/tasks/0/id","code":"value-not-positive"}"#);
    for code in [IssueCode::TypeInvalid, IssueCode::ProfileOutOfRange, IssueCode::BadgeUnreachable, IssueCode::ItemsTooFew] {
        assert_eq!(serde_json::to_value(code).ok(), Some(serde_json::Value::String(code.as_str().to_string())));
    }
}

fn items(ids: &[&str]) -> Vec<SheetItem> {
    ids.iter().map(|id| SheetItem { id: (*id).to_string(), label: text(id), icon: None }).collect()
}

fn classification_sheet() -> SheetTask {
    let Task::Classification(task) = &quiz().tasks[0] else { unreachable!() };
    SheetTask::Classification(SheetClassificationTask { id: "c".to_string(), title: text("c"), prompt: text("c"), icon: None, axes: None, categories: task.categories.clone(), items: items(&["a", "b"]) })
}

fn matching_sheet() -> SheetTask {
    SheetTask::Matching(SheetMatchingTask { id: "m".to_string(), title: text("m"), prompt: text("m"), icon: None, dimensions: vec![SheetDimension { id: "load".to_string(), quantity: quantity("W", Scale::Linear), icon: None, cards: vec![1.0, 2.0] }], items: items(&["x", "y"]) })
}

fn classify(pairs: &[(&str, &str)]) -> Answer {
    Answer::Classification(ClassificationAnswer { assignments: pairs.iter().map(|(item, category)| ((*item).to_string(), (*category).to_string())).collect() })
}

fn order(ids: &[&str]) -> Answer {
    Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: BTreeMap::new() })
}

fn assign(dimensions: &[(&str, &[(&str, usize)])]) -> Answer {
    Answer::Matching(MatchingAnswer { assignments: dimensions.iter().map(|(dimension, cards)| ((*dimension).to_string(), cards.iter().map(|(item, card)| ((*item).to_string(), *card)).collect())).collect() })
}

#[test]
fn classification_answers_must_reference_the_sheet() {
    let sheet = classification_sheet();
    assert_eq!(answer_rejection(&sheet, &classify(&[("a", "low")])), None);
    assert_eq!(answer_rejection(&sheet, &classify(&[])), None);
    assert_eq!(answer_rejection(&sheet, &classify(&[("c", "low")])), Some(Rejection::AnswerInvalid));
    assert_eq!(answer_rejection(&sheet, &classify(&[("a", "future")])), Some(Rejection::AnswerInvalid));
    assert_eq!(answer_rejection(&sheet, &order(&["a", "b"])), Some(Rejection::AnswerInvalid));
    assert!(!answer_complete(&sheet, Some(&classify(&[("a", "low")]))));
    assert!(answer_complete(&sheet, Some(&classify(&[("a", "low"), ("b", "old")]))));
    assert!(!answer_complete(&sheet, None));
}

#[test]
fn sorting_answers_must_be_a_bijection_onto_the_sheet_items() {
    let sheet = SheetTask::Sorting(SheetSortingTask { id: "s".to_string(), title: text("s"), prompt: text("s"), icon: None, quantity: quantity("W", Scale::Linear), items: items(&["a", "b", "c"]) });
    assert_eq!(answer_rejection(&sheet, &order(&["c", "a", "b"])), None);
    for invalid in [&["a", "b"][..], &["a", "b", "b"], &["a", "b", "c", "a"], &["a", "b", "d"]] {
        assert_eq!(answer_rejection(&sheet, &order(invalid)), Some(Rejection::AnswerInvalid), "{invalid:?}");
    }
    assert!(answer_complete(&sheet, Some(&order(&["c", "a", "b"]))));
    assert!(!answer_complete(&sheet, Some(&classify(&[]))));
}

fn guessed(ids: &[&str], guesses: &[(&str, f64)]) -> Answer {
    Answer::Sorting(SortingAnswer { order: ids.iter().map(|id| (*id).to_string()).collect(), guesses: guesses.iter().map(|(item, guess)| ((*item).to_string(), *guess)).collect() })
}

#[test]
fn sorting_guesses_must_name_items_be_finite_and_ascend_along_the_order() {
    let sorting = |scale| SheetTask::Sorting(SheetSortingTask { id: "s".to_string(), title: text("s"), prompt: text("s"), icon: None, quantity: quantity("W", scale), items: items(&["a", "b", "c"]) });
    let (linear, logarithmic) = (sorting(Scale::Linear), sorting(Scale::Logarithmic));
    let rejected = |sheet: &SheetTask, answer: Answer| assert_eq!(answer_rejection(sheet, &answer), Some(Rejection::AnswerInvalid));
    for valid in [guessed(&["a", "b", "c"], &[("a", 1.0), ("b", 2.0), ("c", 3.0)]), guessed(&["a", "b", "c"], &[("a", 1.0), ("b", 1.0), ("c", 1.0)]), guessed(&["c", "a", "b"], &[("c", 0.5), ("b", 9.0)]), guessed(&["c", "a", "b"], &[("a", 7.0)]), guessed(&["a", "b", "c"], &[])] {
        assert_eq!(answer_rejection(&linear, &valid), None, "{valid:?}");
        assert_eq!(answer_rejection(&logarithmic, &valid), None, "{valid:?}");
        assert!(answer_complete(&linear, Some(&valid)));
    }
    assert_eq!(answer_rejection(&linear, &guessed(&["a", "b", "c"], &[("a", -3.0), ("b", 0.0), ("c", 1.0)])), None, "a linear quantity may be guessed at or below zero");
    rejected(&logarithmic, guessed(&["a", "b", "c"], &[("a", 0.0)]));
    rejected(&logarithmic, guessed(&["a", "b", "c"], &[("c", -1.0)]));
    rejected(&linear, guessed(&["a", "b", "c"], &[("a", 1.0), ("d", 2.0)]));
    rejected(&linear, guessed(&["a", "b", "c"], &[("a", 3.0), ("b", 2.0)]));
    rejected(&linear, guessed(&["a", "b", "c"], &[("a", 3.0), ("c", 2.0)]));
    rejected(&linear, guessed(&["c", "a", "b"], &[("a", 1.0), ("c", 2.0)]));
    for guess in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        rejected(&linear, guessed(&["a", "b", "c"], &[("b", guess)]));
    }
    rejected(&linear, guessed(&["a", "b"], &[("a", 1.0), ("b", 2.0)]));
    assert!(serde_json::from_value::<Answer>(serde_json::json!({"kind": "sorting", "order": ["a"], "guesses": {"a": "1"}})).is_err());
    assert!(serde_json::from_value::<Answer>(serde_json::json!({"kind": "sorting", "order": ["a"], "guesses": [1]})).is_err());
}

#[test]
fn shared_answer_vectors_of_the_python_reference_hold() {
    use crate::schema::tests::{entries, fixture, typed};
    let vectors = fixture("answer-validation");
    let sheet_tasks: Vec<SheetTask> = entries(&vectors["sheetTasks"]).iter().map(typed).collect();
    for vector in entries(&vectors["vectors"]) {
        let sheet_task = sheet_tasks.iter().find(|task| Some(task.id().as_str()) == vector["sheetTask"].as_str()).unwrap_or_else(|| panic!("{}", vector["sheetTask"]));
        let answer: Option<Answer> = vector.get("answer").map(typed);
        if let Some(answer) = &answer {
            assert_eq!(answer_rejection(sheet_task, answer).map(|rejection| rejection.as_str()), vector["expected"]["rejection"].as_str(), "{}", vector["id"]);
        }
        if let Some(complete) = vector["expected"]["complete"].as_bool() {
            assert_eq!(answer_complete(sheet_task, answer.as_ref()), complete, "{}", vector["id"]);
        }
    }
    for vector in entries(&vectors["malformed"]) {
        assert!(serde_json::from_value::<Answer>(vector["answer"].clone()).is_err(), "{} violates the schema, so serde refuses it", vector["id"]);
        assert_eq!(vector["expected"]["rejection"].as_str(), Some(Rejection::AnswerInvalid.as_str()), "{}", vector["id"]);
    }
}

#[test]
fn schema_violations_are_refused_by_serde_or_reported() {
    use crate::schema::tests::{entries, fixture, typed};
    for vector in entries(&fixture("schema-conformance")["rejected"]) {
        let document = vector["document"].clone();
        let issues = match vector["definition"].as_str() {
            Some("Quiz") => serde_json::from_value::<Quiz>(document).map(|quiz| quiz_issues(&quiz)),
            Some("Catalog") => serde_json::from_value::<Catalog>(document).map(|catalog| catalog_issues(&catalog, &[]).into_iter().filter(|issue| issue.code != IssueCode::QuizCountMismatch).collect()),
            other => panic!("{other:?}"),
        };
        assert!(issues.as_ref().map_or(true, |issues| !issues.is_empty()), "{} ({}) was accepted", vector["id"], vector["violates"]);
    }
    for case in ["sheet-assembly", "badge-rules", "learner-lifecycle", "leaderboard"] {
        let vectors = fixture(case);
        let quizzes: Vec<Quiz> = entries(&vectors["quizzes"]).iter().map(typed).collect();
        for quiz in &quizzes {
            assert_eq!(quiz_issues(quiz), [], "{case}/{}", quiz.id);
        }
        if let Some(catalog) = vectors.get("catalog") {
            assert_eq!(catalog_issues(&typed(catalog), &quizzes), [], "{case}/catalog");
        }
    }
}

#[test]
fn matching_answers_use_each_card_once_per_dimension() {
    let sheet = matching_sheet();
    assert_eq!(answer_rejection(&sheet, &assign(&[("load", &[("x", 1), ("y", 0)])])), None);
    assert_eq!(answer_rejection(&sheet, &assign(&[("load", &[("x", 1)])])), None);
    assert_eq!(answer_rejection(&sheet, &assign(&[("load", &[("x", 1), ("y", 1)])])), Some(Rejection::AnswerInvalid));
    assert_eq!(answer_rejection(&sheet, &assign(&[("load", &[("x", 2)])])), Some(Rejection::AnswerInvalid));
    assert_eq!(answer_rejection(&sheet, &assign(&[("load", &[("z", 0)])])), Some(Rejection::AnswerInvalid));
    assert_eq!(answer_rejection(&sheet, &assign(&[("area", &[("x", 0)])])), Some(Rejection::AnswerInvalid));
    assert!(answer_complete(&sheet, Some(&assign(&[("load", &[("x", 1), ("y", 0)])]))));
    assert!(!answer_complete(&sheet, Some(&assign(&[("load", &[("x", 1)])]))));
    assert!(!answer_complete(&sheet, Some(&assign(&[]))));
}

#[test]
fn handles_are_trimmed_collapsed_folded_and_keyed_in_lowercase() {
    let normalized = |display: &str, key: &str| Some(NormalizedHandle { display: display.to_string(), key: key.to_string() });
    assert_eq!(normalize_handle("  Ada \t Lovelace\u{3000}"), normalized("Ada Lovelace", "ada lovelace"));
    assert_eq!(normalize_handle("\u{2003}ÄRGER\u{a0}"), normalized("ÄRGER", "ärger"));
    assert_eq!(normalize_handle("GROẞ Straße"), normalized("GROẞ Straße", "groß straße"));
    assert_eq!(normalize_handle("O\u{2019}Brien"), normalized("O'Brien", "o'brien"));
    assert_eq!(normalize_handle("Anna-Lena_2 jr."), normalized("Anna-Lena_2 jr.", "anna-lena_2 jr."));
    assert_eq!(normalize_handle("\u{130}pek"), normalized("\u{130}pek", "i\u{307}pek"));
    assert_eq!(normalize_handle(&"x".repeat(HANDLE_MAX)).map(|handle| handle.key.len()), Some(64));
    assert_eq!(normalize_handle(&format!("{}a", " ".repeat(HANDLE_INPUT_MAX - 1))).map(|handle| handle.display), Some("a".to_string()));
    for refused in ["", " \n ", "'", "...", "' -", &"ü".repeat(HANDLE_MAX + 1), &format!("{}a", " ".repeat(HANDLE_INPUT_MAX))] {
        assert_eq!(normalize_handle(refused), None, "{refused:?}");
    }
}

#[test]
fn handles_refuse_invisible_bidirectional_control_and_combining_characters_and_other_scripts() {
    for point in [0x00u32, 0x07, 0x1B, 0x1F, 0x7F, 0x80, 0xAD, 0x180E, 0x200B, 0x200C, 0x200D, 0x200E, 0x200F, 0x202A, 0x202E, 0x2060, 0x2066, 0x2069, 0xFEFF, 0x0301, 0x0308, 0x0430, 0x03BF, 0xFF21, 0x1D400, 0x1F98A] {
        let character = char::from_u32(point).unwrap_or_else(|| unreachable!());
        assert_eq!(normalize_handle(&format!("Ada{character}Lovelace")), None, "U+{point:04X}");
    }
    let decomposed: String = "André".nfd().collect();
    assert_eq!(normalize_handle(&decomposed), None);
    assert_eq!(normalize_handle(&decomposed.nfc().collect::<String>()).map(|handle| handle.key), Some("andré".to_string()));
}

#[test]
fn the_handle_tables_are_the_ones_the_unicode_data_derives() {
    let blocks = [(0x0000u32, 0x024Fu32), (0x1E00, 0x1EFF)];
    for character in (0..=0x10FFFFu32).filter_map(char::from_u32) {
        let point = u32::from(character);
        assert_eq!(within(&WHITE_SPACE, point), character.is_whitespace(), "white space U+{point:04X}");
        let letter = within(&blocks, point) && (character.is_uppercase() || character.is_lowercase()) && std::iter::once(character).nfkc().eq(std::iter::once(character));
        assert_eq!(within(&HANDLE_LETTERS, point), letter, "letter U+{point:04X}");
    }
}

#[test]
fn the_handle_alphabet_spells_only_nfc() {
    let members: Vec<char> = (0..=0x10FFFFu32).filter_map(char::from_u32).filter(|character| normalize_handle(&format!("a{character}a")).is_some_and(|handle| handle.display == format!("a{character}a"))).collect();
    assert_eq!(members.len(), 681 + 10 + 1 + 4);
    for &member in &members {
        assert!(is_nfc(&member.to_string()), "U+{:04X}", u32::from(member));
    }
    let mut pair = String::new();
    for &first in &members {
        for &second in &members {
            pair.clear();
            pair.push(first);
            pair.push(second);
            assert!(is_nfc(&pair), "U+{:04X} U+{:04X}", u32::from(first), u32::from(second));
        }
    }
}

#[test]
fn handle_streams_are_named_by_the_hex_of_the_key_and_back() {
    assert_eq!(handle_actor_id("ada"), "616461");
    assert_eq!(handle_actor_id("jürgen müller"), "6ac3bc7267656e206dc3bc6c6c6572");
    assert_eq!(handle_key_of(&handle_actor_id("groß straße")).as_deref(), Some("groß straße"));
    for refused in ["", "6", "6G", "C3", "c3", "c328", "ff", "616461 "] {
        assert_eq!(handle_key_of(refused), None, "{refused:?}");
    }
}

#[test]
fn ids_are_32_lowercase_hex_and_commands_and_queries_are_held_to_them() {
    let id = "0123456789abcdef0123456789abcdef";
    assert!(is_id(id));
    for refused in ["0123456789ABCDEF0123456789ABCDEF", &id[1..], &format!("{id}0"), &format!("{id}\n"), &format!(" {}", &id[1..]), &"g".repeat(32), "", "enroll:architecture:roster:1"] {
        assert!(!is_id(refused), "{refused:?}");
    }
    let start = |run: &str, quiz: &str| Command::StartRun { id: id.to_string(), learner: id.to_string(), run: run.to_string(), quiz: quiz.to_string() };
    assert_eq!(command_rejection(&start(id, "cooling-basics")), None);
    assert_eq!(command_rejection(&start("run-1", "cooling-basics")), Some(Rejection::IdInvalid));
    assert_eq!(command_rejection(&start(id, "Cooling")), Some(Rejection::IdInvalid));
    assert_eq!(command_rejection(&Command::SubmitRun { id: "x".repeat(4096), learner: id.to_string(), run: id.to_string() }), Some(Rejection::IdInvalid));
    assert_eq!(command_rejection(&Command::RecordAnswer { id: id.to_string(), learner: id.to_string(), run: id.to_string(), task: "../loads".to_string(), answer: Answer::Sorting(SortingAnswer { order: Vec::new(), guesses: BTreeMap::new() }) }), Some(Rejection::IdInvalid));
    let board = |quiz: Option<&str>, learner: Option<&str>| Query::Leaderboard { period: LeaderboardPeriod::Weekly, quiz: quiz.map(str::to_string), learner: learner.map(str::to_string) };
    assert_eq!(query_rejection(&board(None, None)), None);
    assert_eq!(query_rejection(&board(Some("cooling-basics"), Some(id))), None);
    assert_eq!(query_rejection(&board(None, Some("0d07d623"))), Some(Rejection::IdInvalid));
    assert_eq!(query_rejection(&board(Some("Cooling"), Some(id))), Some(Rejection::IdInvalid));
    assert_eq!(query_rejection(&Query::Run { run: id.to_uppercase() }), Some(Rejection::IdInvalid));
    assert_eq!(query_rejection(&Query::Crowd { quiz: "Cooling".to_string() }), Some(Rejection::IdInvalid));
    assert_eq!(query_rejection(&Query::Handle { handle: " Ada ".to_string() }), None);
    assert_eq!(query_rejection(&Query::Handle { handle: "A\u{200b}da".to_string() }), Some(Rejection::HandleInvalid));
}

#[test]
fn shared_identity_shapes_of_the_python_reference_hold() {
    use crate::schema::tests::{assert_close, entries, fixture, json, typed};
    let vectors = fixture("identity-shapes");
    assert!(!entries(&vectors["handles"]).is_empty() && !entries(&vectors["shapes"]).is_empty());
    for vector in entries(&vectors["handles"]) {
        let normalized = normalize_handle(vector["handle"].as_str().unwrap_or_default()).map(|handle| serde_json::json!({ "display": handle.display, "key": handle.key, "actor": handle_actor_id(&handle.key) }));
        assert_close(&format!("handles/{}", vector["id"]), &normalized.unwrap_or(serde_json::Value::Null), &vector["expected"]);
    }
    for vector in entries(&vectors["shapes"]) {
        let rejection = if vector["definition"] == "Command" { command_rejection(&typed::<Command>(&vector["document"])) } else { query_rejection(&typed::<Query>(&vector["document"])) };
        assert_close(&format!("shapes/{}", vector["id"]), &json(&rejection), &vector["expected"]);
    }
}
