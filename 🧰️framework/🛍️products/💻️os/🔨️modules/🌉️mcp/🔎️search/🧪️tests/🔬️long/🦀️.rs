use super::*;
use crate::catalog::{audit_source, compile, CapabilityAudience, CapabilityDefinition, CapabilityOwner, Catalog, CatalogAuditFinding, CatalogSource};
use semio_framework::{Locale, Terminology};

/// 🧾️ The plugins whose every agent-published verb carries a hand-authored en+de description, a
/// reviewed audience and a destructive mark where it discards or writes outside history
/// (`📓️m5b-mcp-catalog-descriptions-classification-destructive.md`). `gis` joins once the freeze on
/// its crate lifts.
const AUTHORED_PLUGINS: &[&str] = &["draw", "note", "raster", "layout", "forms", "cad"];

/// 📇️ The catalog source an agent actually meets: the repo's generated plugin registry and every
/// committed `🔣️.json` descriptor, discovered exactly as `semio-os-mcp stdio --folder .` does.
fn installed_source() -> CatalogSource {
    let root = crate::find_repo_root().expect("the tests run inside the repo");
    crate::discover_catalog_source(Some(&root))
}

fn installed_catalog(locale: Locale) -> Catalog {
    compile(&installed_source(), locale, Terminology::Native).expect("the installed catalog compiles")
}

fn plugin_of(capability: &CapabilityDefinition) -> Option<&str> {
    match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, .. } => Some(plugin_id.as_str()),
        _ => None,
    }
}

fn verb_of(capability_id: &str) -> &str {
    capability_id.rsplit('.').next().unwrap_or(capability_id)
}

fn published<'catalog>(catalog: &'catalog Catalog, plugin: &str, verb: &str) -> Vec<&'catalog CapabilityDefinition> {
    catalog.entries.iter().filter(|entry| plugin_of(entry) == Some(plugin) && verb_of(entry.id.as_str()) == verb).collect()
}

/// 🎯️ The three G8 queries answer with one clear, non-tied winner of the right class over the real
/// installed catalog — not over a fixture.
#[test]
fn the_installed_catalog_answers_three_intent_queries_with_a_clear_winner() {
    let catalog = installed_catalog(Locale::En);
    let rectangle = search(&catalog, "draw rectangle", &SearchFilters::default());
    assert_eq!(rectangle[0].capability_id, "draw.s.draw.drawing@1/*#editor.addLayer", "{:?}", &rectangle[..3]);
    assert!(rectangle[0].score > rectangle[1].score * 1.5, "addLayer must win clearly: {:?}", &rectangle[..3]);
    let top: BTreeSet<String> = rectangle.iter().take(5).map(|hit| format!("{:.4}", hit.score)).collect();
    assert_eq!(top.len(), 5, "no two of the top five may tie: {:?}", &rectangle[..5]);
    let layer = search(&catalog, "add layer", &SearchFilters::default());
    assert_eq!(verb_of(&layer[0].capability_id), "addLayer", "{:?}", &layer[..3]);
    assert!(layer[0].score > layer[1].score, "the top `add layer` hit must not tie: {:?}", &layer[..3]);
    let export = search(&catalog, "export pdf", &SearchFilters::default());
    assert!(verb_of(&export[0].capability_id).to_ascii_lowercase().contains("export"), "{:?}", &export[..3]);
    assert!(export[0].score > export[1].score, "the top `export pdf` hit must not tie: {:?}", &export[..3]);
}

/// 🖱️ No raw pointer/keyboard/engagement event of any installed plugin is published, and no
/// gesture-named route reached the agent catalog undeclared.
#[test]
fn the_installed_catalog_publishes_no_raw_input_event() {
    let catalog = installed_catalog(Locale::En);
    for (plugin, verb) in [("draw", "canvasPointerMove"), ("draw", "canvasEscape"), ("draw", "canvasPointerDown"), ("draw", "engagementInput"), ("layout", "canvasPointerMove"), ("cad", "worldPointerDown"), ("note", "inkApplyEvents")] {
        assert!(published(&catalog, plugin, verb).is_empty(), "{plugin}.{verb} must not be published to agents");
    }
    assert!(catalog.entries.iter().all(|entry| entry.audience == CapabilityAudience::Agent));
    for query in ["canvas pointer move", "escape", "pointer down"] {
        for hit in search(&catalog, query, &SearchFilters::default()) {
            assert!(!["canvasPointerMove", "canvasEscape", "canvasPointerDown"].contains(&verb_of(&hit.capability_id)), "{} leaked into `{query}`", hit.capability_id);
        }
    }
    let undeclared: Vec<CatalogAuditFinding> = audit_source(&installed_source()).into_iter().filter(|finding| matches!(finding, CatalogAuditFinding::UndeclaredGestureRoute { .. })).collect();
    assert_eq!(undeclared, Vec::new());
}

/// 💬️ Every verb the authored plugins publish carries a description in English AND German — no
/// default language, no empty text for an agent to choose between.
#[test]
fn the_authored_plugins_describe_every_published_verb_in_en_and_de() {
    for locale in [Locale::En, Locale::De] {
        let catalog = installed_catalog(locale);
        let blank: Vec<&str> = catalog.entries.iter().filter(|entry| plugin_of(entry).is_some_and(|plugin| AUTHORED_PLUGINS.contains(&plugin)) && entry.description.trim().is_empty()).map(|entry| entry.id.as_str()).collect();
        assert_eq!(blank, Vec::<&str>::new(), "undescribed in {locale:?}");
    }
}

/// ⚠️ The authored plugins mark every delete/clear/reset/replace verb and every export/save to a
/// user path destructive, so `ApprovalMode::WhenDestructive` asks a human before an agent commits it;
/// camera poses and preview-runner steps are chrome and never published.
#[test]
fn the_authored_plugins_gate_destructive_and_user_path_verbs_and_keep_chrome_out() {
    let findings: Vec<CatalogAuditFinding> = audit_source(&installed_source()).into_iter().filter(|finding| AUTHORED_PLUGINS.iter().any(|plugin| finding.capability_id().starts_with(&format!("{plugin}.")))).collect();
    assert_eq!(findings, Vec::new());
    let catalog = installed_catalog(Locale::En);
    for (plugin, verb) in [("layout", "exportPdf"), ("layout", "exportPng"), ("draw", "exportDocument"), ("draw", "commitDocument"), ("draw", "setSnapshot"), ("note", "saveDownload"), ("forms", "exportFixture"), ("cad", "saveCurrent")] {
        let hits = published(&catalog, plugin, verb);
        assert!(!hits.is_empty(), "{plugin}.{verb} is published");
        for capability in hits {
            assert!(capability.effects.destructive, "{} must be destructive", capability.id);
            assert_eq!(capability.policy.approval, semio_framework::manifest::ApprovalMode::WhenDestructive, "{} must gate on approval", capability.id);
        }
    }
    for (plugin, verb) in [("cad", "setCamera"), ("cad", "setProjectionParam"), ("forms", "resetTry"), ("forms", "previousStep"), ("forms", "nextStep")] {
        assert!(published(&catalog, plugin, verb).is_empty(), "{plugin}.{verb} is chrome and must not be published to agents");
    }
}
