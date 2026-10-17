/// 🗣️ Chrome i18n and switch order: bilingual chrome literal laws, chrome key coverage and the create-before-retire session switch.

//#region 🗣️ChromeI18n

/// 🗣️ No user-visible chrome literal may bypass the shell's own bilingual paths. Every
/// `Label::data("…")`/`LocalizedLabel::data("…")` literal left in this file must be a genuinely
/// locale-invariant proper noun or endonym, named here with its reason; everything else routes
/// through `shell_chrome_string` (EN/DE table) or `LocalizedLabel::native(en, de)` (the exhaustive
/// per-`Locale` constructor, which fails to compile when a locale is added untranslated).
#[test]
fn no_user_visible_chrome_literal_bypasses_the_bilingual_paths() {
    const LOCALE_INVARIANT: [(&str, &str); 4] = [
        ("Semio", "the product's own name — a proper noun, identical in every locale"),
        ("Mono", "the built-in theme's own name — a proper noun, identical in every locale"),
        ("English", "a language ENDONYM: a locale picker names each tongue in its own tongue, never translated"),
        ("Deutsch", "a language ENDONYM: a locale picker names each tongue in its own tongue, never translated"),
    ];
    let mut offenders = Vec::new();
    let mut allowed = 0;
    for (index, line) in WGPU_SHELL_SOURCE().lines().enumerate() {
        let mut rest = line;
        while let Some(at) = rest.find("Label::data(\"") {
            let literal_start = at + "Label::data(\"".len();
            let tail = &rest[literal_start..];
            let literal = tail.split('"').next().unwrap_or_default();
            if LOCALE_INVARIANT.iter().any(|(allowed_literal, _)| *allowed_literal == literal) {
                allowed += 1;
            } else {
                offenders.push(format!("{}:{literal:?}", index + 1));
            }
            rest = &tail[literal.len()..];
        }
    }
    assert!(offenders.is_empty(), "these chrome literals bypass the EN/DE paths — route them through shell_chrome_string or LocalizedLabel::native: {offenders:?}");
    assert_eq!(allowed, 6, "exactly the six declared locale-invariant proper-noun/endonym call sites remain");
}

/// 🗣️ The `Label::data` family is only one way an English-only literal reaches a reader. This law
/// closes the rest: ANY capitalised multi-word string literal in the shell is user-visible copy
/// unless it is one of four declared exceptions — a `LocalizedLabel::native(en, de)` argument (which
/// carries both tongues by construction), an arm of one of the two EN/DE tables, or an internal
/// `Shell …` fault diagnostic that never reaches a reader. It is what caught `Find in page`,
/// `Go Home`, `Apply Layout`, `Check In`, `Move Window` and `Resize Window` after the Command Palette
/// sweep had already declared itself done.
#[test]
fn no_capitalised_chrome_phrase_escapes_the_bilingual_tables() {
    let table = region_of(WGPU_SHELL_SOURCE(), "//#region 🗣️ChromeI18n", "//#endregion 🗣️ChromeI18n");
    let administration = region_of(WGPU_SHELL_SOURCE(), "fn shell_space_administration_label", "\n}\n");
    let mut offenders = Vec::new();
    let mut exempted = 0;
    let mut offset = 0;
    for (line_number, line) in WGPU_SHELL_SOURCE().lines().enumerate() {
        let at = offset;
        offset += line.len() + 1;
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue;
        }
        if table.contains(&at) || administration.contains(&at) || line.contains("LocalizedLabel::native(") {
            exempted += 1;
            continue;
        }
        for literal in line.split('"').skip(1).step_by(2) {
            if !is_capitalised_phrase(literal) {
                continue;
            }
            if literal.starts_with("Shell ") {
                exempted += 1;
                continue;
            }
            offenders.push(format!("{}:{literal:?}", line_number + 1));
        }
    }
    assert!(offenders.is_empty(), "these chrome phrases never reach the EN/DE tables — route them through shell_chrome_string or LocalizedLabel::native: {offenders:?}");
    assert!(exempted > 40, "the exemptions themselves must stay visible to this law, saw {exempted}");
}

/// 🔤️ `Word word…` / `Word Word…` — a capitalised phrase of at least two words, which is what UI
/// copy looks like and what an id (`shell.window.close`), a wire tag (`meshingFaces`) or a single
/// proper noun (`Semio`) never does.
fn is_capitalised_phrase(literal: &str) -> bool {
    let mut words = literal.split(' ');
    let Some(first) = words.next() else { return false };
    if first.len() < 2 || !first.starts_with(|c: char| c.is_ascii_uppercase()) || !first[1..].chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    let rest: Vec<&str> = words.collect();
    !rest.is_empty() && rest.iter().all(|word| !word.is_empty() && word.chars().all(|c| c.is_ascii_alphabetic()))
}

/// 📐️ The byte range of one `//#region`-delimited (or brace-delimited) stretch of the shell source.
fn region_of(source: &str, start: &str, end: &str) -> std::ops::Range<usize> {
    let Some(from) = source.find(start) else { return 0..0 };
    let to = source[from..].find(end).map(|offset| from + offset + end.len()).unwrap_or(source.len());
    from..to
}

/// 🗣️ The EN/DE table itself: every key the shell asks for answers in both tongues, and no key
/// answers with the raw key (the fallback), which is what a missing translation would look like.
#[test]
fn every_chrome_key_the_shell_asks_for_answers_in_both_tongues() {
    let mut keys: Vec<&str> = Vec::new();
    let mut rest = WGPU_SHELL_SOURCE();
    while let Some(at) = rest.find("shell_chrome_string(\"") {
        let tail = &rest[at + "shell_chrome_string(\"".len()..];
        let key = tail.split('"').next().unwrap_or_default();
        if !keys.contains(&key) {
            keys.push(key);
        }
        rest = tail;
    }
    assert!(keys.len() >= 18, "the shell asks the table for {} keys", keys.len());
    for key in &keys {
        let (en, de) = (shell_chrome_string(key, false), shell_chrome_string(key, true));
        assert_ne!(en, *key, "{key}: the English arm falls through to the raw key");
        assert_ne!(de, *key, "{key}: the German arm falls through to the raw key");
        assert!(!en.is_empty() && !de.is_empty(), "{key}: both tongues carry text");
    }
}

//#endregion 🗣️ChromeI18n

//#region 🔀️SwitchOrder

/// 🔀️ The SHARED subsequence both shells hold: seal before retire, create before retire, retire
/// before publish, publish before refresh. React's `quiesce` and `seed` steps have no wgpu
/// counterpart yet — this target has no pending-guest-work tracker to quiesce and seeds its
/// landing layout inside the same `sync_dock` the publish step runs — so they are deliberately
/// NOT asserted against this body rather than faked.
#[test]
fn the_session_switch_creates_before_it_retires_and_leaks_no_instance() {
    let fixture: Value = serde_json::from_str(SURFACE_SWITCH_FIXTURE).expect("🔀️ fixture parses");
    let switched = fixture["switch"].as_array().expect("fixture switch rows").iter().find(|case| case["expected"]["status"] == "switched").expect("the fixture declares at least one completed switch");
    let declared: Vec<String> = switched["expected"]["steps"].as_array().expect("fixture steps").iter().map(|step| step.as_str().expect("fixture step").split(':').next().expect("step verb").to_string()).collect();
    let body_start = WGPU_SHELL_SOURCE().find("async fn run_session_app_switch").expect("the shell declares the ordered session switch");
    let body = &WGPU_SHELL_SOURCE()[body_start..];
    let body = &body[..body.find("\n    /// 🔁️ Steps the active mode").expect("the switch body ends before the next item")];
    let at = |needle: &str| body.find(needle).unwrap_or_else(|| panic!("the ordered switch performs {needle}"));
    let (create, seal, retire, publish, refresh) = (at("create_app("), at("retire_documents_outside("), at("destroy_app("), at("push_contributions("), at("refresh_ui("));
    assert!(create < seal && seal < retire, "create → seal → retire: the successor exists before the predecessor's documents and instance go");
    assert!(retire < publish && publish < refresh, "retire → publish → refresh");
    assert_eq!(body.matches("create_app(").count(), 1, "created − retired === 0: one create per switch");
    assert_eq!(body.matches("destroy_app(").count(), 1, "created − retired === 0: one retire per switch");
    let declared_at = |verb: &str| declared.iter().position(|step| step == verb).unwrap_or_else(|| panic!("the shared fixture declares {verb}"));
    assert!(declared_at("seal") < declared_at("retire"));
    assert!(declared_at("create") < declared_at("retire"));
    assert!(declared_at("retire") < declared_at("publish"));
    assert!(declared_at("publish") < declared_at("refresh"));
}

#[test]
fn asking_for_the_role_already_mounted_creates_and_retires_nothing() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let dialect = ArtifactDialect { artifact_kind: "s.procedural.generation3d".into(), standard: "1".into(), subset: "*".into() };
    let app = parity_app("s.procedural.generation3d@1/*#editor", AppRole::Editor, dialect, "Editor", "Editor", &["edit", "generate"]);
    shell.docs.session = Some(ActiveSession { plugin_id: "procedural".into(), instance_id: 7, app, view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).switch_to_session_role(AppRole::Editor)).expect("a no-op switch never fails");
    let session = shell.docs.session.as_ref().expect("the session survives a no-op switch");
    assert_eq!(session.instance_id, 7, "the mounted instance is untouched");
    assert_eq!(session.app.role, AppRole::Editor);
}

//#endregion 🔀️SwitchOrder
