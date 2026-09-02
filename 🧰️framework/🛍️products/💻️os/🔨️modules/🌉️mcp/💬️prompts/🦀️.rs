//! 💬️ `prompts` — ticket 26/08/29/AI-MCP-END-TO-END packet W7: the `prompts/list`+`prompts/get`
//! surface, which shipped as an empty `InMemoryPromptRegistry` until now. Every prompt here is
//! **plugin-, app- and artifact-agnostic**: it teaches the agent this gateway's *protocol* — how to
//! discover capabilities, how to mutate safely through prepare→preview→commit, how to undo, how to
//! read an artifact together with its inferences — never a specific plugin's vocabulary. A newly
//! installed plugin changes nothing here, which is exactly the point: the prompts route the agent to
//! `capabilities_search`, and the catalog answers with whatever is really installed.
//!
//! Bilingual by construction (`📓️CLAUDE.md`: multiple languages, no default, English first then
//! German). Every prompt takes an optional `locale` argument; an unknown or absent locale resolves to
//! English rather than failing, because a prompt is guidance, not policy.

use crate::errors::{GatewayError, GatewayErrorCode};
use crate::protocol::{ContentBlock, InMemoryPromptRegistry, Prompt, PromptArgument, PromptGetResult, PromptMessage};

//#region 🔖️Locale
/// 🌍️ The two languages this gateway speaks, in the repo's declared order. Never a default field on
/// a struct — the caller always states one, and an unrecognised tag resolves to `En` explicitly here
/// rather than silently somewhere downstream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptLocale {
    En,
    De,
}

impl PromptLocale {
    /// 🏷️ Resolves an MCP `locale` argument. `None`, `""` and any unknown tag are English — stated
    /// once, here.
    pub fn resolve(raw: Option<&str>) -> Self {
        match raw.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
            Some(tag) if tag == "de" || tag.starts_with("de-") => Self::De,
            _ => Self::En,
        }
    }

    fn from_arguments(arguments: Option<&serde_json::Value>) -> Self {
        Self::resolve(arguments.and_then(|value| value.get("locale")).and_then(serde_json::Value::as_str))
    }
}
//#endregion 🔖️Locale

//#region 🔖️Definitions
/// 💬️ One prompt's full bilingual definition — the single source of truth for both its `prompts/list`
/// row and its `prompts/get` body, so the two can never drift apart.
struct PromptDefinition {
    name: &'static str,
    title_en: &'static str,
    title_de: &'static str,
    description_en: &'static str,
    description_de: &'static str,
    body_en: &'static str,
    body_de: &'static str,
}

impl PromptDefinition {
    fn prompt(&self) -> Prompt {
        Prompt {
            name: self.name.to_string(),
            title: Some(format!("{} / {}", self.title_en, self.title_de)),
            description: Some(format!("{} — {}", self.description_en, self.description_de)),
            arguments: vec![PromptArgument { name: "locale".to_string(), description: Some("`en` (default) or `de`.".to_string()), required: false }],
        }
    }

    fn result(&self, locale: PromptLocale) -> PromptGetResult {
        let (description, body) = match locale {
            PromptLocale::En => (self.description_en, self.body_en),
            PromptLocale::De => (self.description_de, self.body_de),
        };
        PromptGetResult { description: Some(description.to_string()), messages: vec![PromptMessage { role: "user".to_string(), content: ContentBlock::Text { text: body.to_string() } }] }
    }
}

const EXPLORE_WORKSPACE: PromptDefinition = PromptDefinition {
    name: "explore_workspace",
    title_en: "Explore this workspace",
    title_de: "Diesen Arbeitsbereich erkunden",
    description_en: "Discover what this OS can do right now, without assuming any plugin is installed",
    description_de: "Herausfinden, was dieses OS gerade kann, ohne ein bestimmtes Plugin vorauszusetzen",
    body_en: "Find out what this semio workspace can do right now.\n\n1. Call `context_resolve` first. It opens your session and returns the catalog hash, the granted scopes and the active artifact, if any.\n2. Read `semio://workspace` for the bound space, and `semio://workspace/artifacts` for the artifact ids in it.\n3. Call `capabilities_search` with a plain-language description of the goal. The catalog is compiled from the plugins that are actually installed, so never assume a capability exists — search for it.\n4. Call `capabilities_describe` on any hit before using it, to read its real input schema and its required scopes.\n\nIf a tool answers `PLUGIN_UNAVAILABLE`, read the message: it names exactly which binding is missing (`--folder`/`--hub` for a workspace, an attached shell for UI). That is a tier, not a failure.",
    body_de: "Finde heraus, was dieser semio-Arbeitsbereich gerade kann.\n\n1. Rufe zuerst `context_resolve` auf. Das öffnet deine Sitzung und liefert den Katalog-Hash, die erteilten Scopes und – falls vorhanden – das aktive Artefakt.\n2. Lies `semio://workspace` für den gebundenen Raum und `semio://workspace/artifacts` für die darin enthaltenen Artefakt-IDs.\n3. Rufe `capabilities_search` mit einer umgangssprachlichen Beschreibung des Ziels auf. Der Katalog wird aus den tatsächlich installierten Plugins kompiliert – setze also nie voraus, dass eine Fähigkeit existiert, sondern suche danach.\n4. Rufe `capabilities_describe` für jeden Treffer auf, bevor du ihn verwendest, um sein echtes Eingabeschema und seine erforderlichen Scopes zu lesen.\n\nAntwortet ein Werkzeug mit `PLUGIN_UNAVAILABLE`, lies die Meldung: sie benennt genau die fehlende Bindung (`--folder`/`--hub` für einen Arbeitsbereich, eine verbundene Shell für die Oberfläche). Das ist eine Stufe, kein Fehler.",
};

const SAFE_MUTATION: PromptDefinition = PromptDefinition {
    name: "safe_mutation",
    title_en: "Change something safely",
    title_de: "Etwas sicher ändern",
    description_en: "The observe → prepare → preview → commit → verify loop every write must follow",
    description_de: "Die Schleife beobachten → vorbereiten → Vorschau → festschreiben → prüfen, der jeder Schreibvorgang folgen muss",
    body_en: "Change something in this workspace without risking a lost update.\n\n1. **Observe** — read the artifact and note its revision stamp.\n2. **Prepare** — call `action_prepare` with the capability id and your input. It validates the input, checks policy and dry-runs the change; the `PreparedActionReport` carries a preview and a prepared handle. Nothing has been written yet.\n3. **Preview** — read the preview. If it is not what you intended, call `action_cancel` and start over.\n4. **Commit** — call `action_invoke` with the prepared handle and the `expectedRevision` you observed. A stale stamp comes back as `REVISION_CONFLICT`: re-observe and prepare again, never retry blindly. Pass an `idempotencyKey` so a retried call replays its stored report instead of writing twice.\n5. **Verify** — re-read the artifact and confirm the change landed.\n\nTo make several changes atomically, prepare each one, bind them with `transaction_begin`, then `transaction_commit` (or `transaction_rollback`). Keep the returned undo token: `history_undo` takes it.\n\nA destructive capability requires approval. If approval is refused you get `PERMISSION_DENIED` and an audit row — do not try to route around it.",
    body_de: "Ändere etwas in diesem Arbeitsbereich, ohne eine verlorene Aktualisierung zu riskieren.\n\n1. **Beobachten** – lies das Artefakt und merke dir seinen Revisionsstempel.\n2. **Vorbereiten** – rufe `action_prepare` mit der Capability-ID und deiner Eingabe auf. Das prüft die Eingabe, kontrolliert die Richtlinien und führt die Änderung als Trockenlauf aus; der `PreparedActionReport` enthält eine Vorschau und ein vorbereitetes Handle. Es wurde noch nichts geschrieben.\n3. **Vorschau** – lies die Vorschau. Ist sie nicht wie beabsichtigt, rufe `action_cancel` auf und beginne von vorn.\n4. **Festschreiben** – rufe `action_invoke` mit dem vorbereiteten Handle und der beobachteten `expectedRevision` auf. Ein veralteter Stempel kommt als `REVISION_CONFLICT` zurück: erneut beobachten und neu vorbereiten, niemals blind wiederholen. Übergib einen `idempotencyKey`, damit ein wiederholter Aufruf seinen gespeicherten Bericht abspielt, statt zweimal zu schreiben.\n5. **Prüfen** – lies das Artefakt erneut und bestätige, dass die Änderung angekommen ist.\n\nFür mehrere Änderungen atomar: jede vorbereiten, mit `transaction_begin` bündeln, dann `transaction_commit` (oder `transaction_rollback`). Bewahre das zurückgegebene Undo-Token auf – `history_undo` benötigt es.\n\nEine destruktive Fähigkeit erfordert eine Freigabe. Wird sie verweigert, erhältst du `PERMISSION_DENIED` und einen Audit-Eintrag – versuche nicht, das zu umgehen.",
};

const INSPECT_ARTIFACT: PromptDefinition = PromptDefinition {
    name: "inspect_artifact",
    title_en: "Inspect an artifact",
    title_de: "Ein Artefakt untersuchen",
    description_en: "Read one artifact together with its schema, validation, inferences and history",
    description_de: "Ein Artefakt zusammen mit Schema, Validierung, Inferenzen und Historie lesen",
    body_en: "Build a complete picture of one artifact.\n\n- `artifact_open` — its identity, kind and current revision.\n- `semio://artifact/{id}` — its real bytes.\n- `artifact_validate` — what its own plugin says about its validity. This is the plugin's answer, not a guess.\n- `inference_list` then `inference_get` (or `semio://artifact/{id}/inference[/{field}]`) — the derived values its plugin declares. Each result reports whether it was cached and the dep-hash it was keyed on; treat a stale hash as a reason to recompute, not to trust.\n- `semio://artifact/{id}/history` — the edits applied to it.\n- `artifact_snapshot` — a point-in-time content snapshot.\n- `artifact_export` — the formats its plugin really offers. Ask before assuming any specific format exists.\n\nEverything here is generic over artifact kind. If a sub-resource is unavailable you get a typed error naming what is missing — never a fabricated body.",
    body_de: "Verschaffe dir ein vollständiges Bild eines Artefakts.\n\n- `artifact_open` – Identität, Art und aktuelle Revision.\n- `semio://artifact/{id}` – die echten Bytes.\n- `artifact_validate` – was sein eigenes Plugin über seine Gültigkeit sagt. Das ist die Antwort des Plugins, keine Vermutung.\n- `inference_list`, dann `inference_get` (oder `semio://artifact/{id}/inference[/{field}]`) – die abgeleiteten Werte, die sein Plugin deklariert. Jedes Ergebnis meldet, ob es zwischengespeichert war, und den Dep-Hash, unter dem es abgelegt wurde; ein veralteter Hash ist ein Grund zur Neuberechnung, nicht zum Vertrauen.\n- `semio://artifact/{id}/history` – die angewandten Änderungen.\n- `artifact_snapshot` – eine Momentaufnahme des Inhalts.\n- `artifact_export` – die Formate, die sein Plugin wirklich anbietet. Frage nach, statt ein bestimmtes Format vorauszusetzen.\n\nAlles hier ist generisch über die Artefaktart. Ist eine Unterressource nicht verfügbar, erhältst du einen typisierten Fehler, der benennt, was fehlt – niemals einen erfundenen Inhalt.",
};

const DRIVE_THE_UI: PromptDefinition = PromptDefinition {
    name: "drive_the_ui",
    title_en: "See and drive the user interface",
    title_de: "Die Benutzeroberfläche sehen und steuern",
    description_en: "Read what the human sees and act in their shell — only while one is attached",
    description_de: "Lesen, was der Mensch sieht, und in seiner Shell handeln – nur solange eine verbunden ist",
    body_en: "Work alongside the human in their running shell.\n\nRead first:\n- `semio://window` — the window and panel inventory.\n- `semio://ui/active-context` — what the shell considers active.\n- `semio://ui/selection` — what is selected right now.\n\nThen act:\n- `ui_focus` — bring a window forward.\n- `ui_reveal` — make a panel visible and navigate it to a path.\n\nThese need a live shell dialed into the gateway's bridge. Without one you get a retryable error saying no shell is attached — that is the normal headless state, and it resolves by itself the moment someone opens the app. Do not treat it as a bug and do not fabricate UI state.\n\nLong-running work returns a job id: poll `job_get` for status and progress, and `job_cancel` to stop it. Cancellation is cooperative — confirm with `job_get` rather than assuming.",
    body_de: "Arbeite gemeinsam mit dem Menschen in seiner laufenden Shell.\n\nZuerst lesen:\n- `semio://window` – Bestand an Fenstern und Panels.\n- `semio://ui/active-context` – was die Shell als aktiv betrachtet.\n- `semio://ui/selection` – was gerade ausgewählt ist.\n\nDann handeln:\n- `ui_focus` – ein Fenster in den Vordergrund holen.\n- `ui_reveal` – ein Panel sichtbar machen und zu einem Pfad navigieren.\n\nDies erfordert eine Shell, die sich mit der Bridge des Gateways verbunden hat. Ohne sie erhältst du einen wiederholbaren Fehler, der besagt, dass keine Shell verbunden ist – das ist der normale kopflose Zustand und löst sich von selbst, sobald jemand die App öffnet. Behandle es nicht als Fehler und erfinde keinen Oberflächenzustand.\n\nLang laufende Arbeit liefert eine Job-ID: frage `job_get` für Status und Fortschritt ab und `job_cancel`, um sie zu stoppen. Der Abbruch ist kooperativ – bestätige ihn mit `job_get`, statt ihn vorauszusetzen.",
};

const UNDO_LAST_CHANGE: PromptDefinition = PromptDefinition {
    name: "undo_last_change",
    title_en: "Undo or redo a change",
    title_de: "Eine Änderung rückgängig machen oder wiederholen",
    description_en: "Walk this workspace's history without corrupting a collaborator's work",
    description_de: "Die Historie dieses Arbeitsbereichs durchlaufen, ohne die Arbeit anderer zu beschädigen",
    body_en: "Undo or redo a change you made.\n\n1. Every committed `action_invoke` and `transaction_commit` returns an undo token. Use that token — do not guess one.\n2. `history_undo` fans the reversal out to every member the invocation or saga touched, so a multi-part change reverses as one unit.\n3. `history_redo` re-applies it, with the same token.\n4. Read `semio://artifact/{id}/history` to confirm the result rather than assuming it.\n\nThis workspace is multi-user and event-sourced: other people's edits arrive while you work. Undo reverses *your* change group, it does not rewind the artifact to an earlier moment in time. If the token is gone or the group is no longer reversible you get a typed error — re-read the history and decide again rather than forcing it.",
    body_de: "Mache eine Änderung rückgängig oder wiederhole sie.\n\n1. Jedes festgeschriebene `action_invoke` und `transaction_commit` liefert ein Undo-Token. Verwende dieses Token – rate keines.\n2. `history_undo` verteilt die Umkehrung auf jedes Mitglied, das der Aufruf oder die Saga berührt hat, sodass eine mehrteilige Änderung als Einheit zurückgenommen wird.\n3. `history_redo` wendet sie mit demselben Token erneut an.\n4. Lies `semio://artifact/{id}/history`, um das Ergebnis zu bestätigen, statt es vorauszusetzen.\n\nDieser Arbeitsbereich ist mehrbenutzerfähig und ereignisbasiert: Änderungen anderer treffen ein, während du arbeitest. Undo nimmt *deine* Änderungsgruppe zurück, es spult das Artefakt nicht auf einen früheren Zeitpunkt zurück. Ist das Token verschwunden oder die Gruppe nicht mehr umkehrbar, erhältst du einen typisierten Fehler – lies die Historie erneut und entscheide neu, statt es zu erzwingen.",
};

/// 💬️ Every prompt this gateway serves, in `prompts/list` order.
const DEFINITIONS: [PromptDefinition; 5] = [EXPLORE_WORKSPACE, SAFE_MUTATION, INSPECT_ARTIFACT, DRIVE_THE_UI, UNDO_LAST_CHANGE];

/// 💬️ The prompt names, as a census the tests assert the registry against.
pub const GATEWAY_PROMPT_NAMES: [&str; 5] = ["explore_workspace", "safe_mutation", "inspect_artifact", "drive_the_ui", "undo_last_change"];
//#endregion 🔖️Definitions

//#region 🔖️Registry
/// 🏗️ Builds the real `PromptRegistry` the gateway serves — five protocol-teaching prompts, each
/// bilingual, none of them naming a plugin, an app or an artifact kind.
pub fn build_prompt_registry() -> InMemoryPromptRegistry {
    let mut registry = InMemoryPromptRegistry::new();
    for definition in DEFINITIONS {
        registry.register(definition.prompt(), move |arguments| Ok(definition.result(PromptLocale::from_arguments(arguments.as_ref()))));
    }
    registry
}

/// 🔎️ Looks one prompt up by name outside the registry — used by callers that want a prompt's body
/// without going through `prompts/get`'s argument plumbing.
pub fn prompt_body(name: &str, locale: PromptLocale) -> Result<PromptGetResult, GatewayError> {
    DEFINITIONS.iter().find(|definition| definition.name == name).map(|definition| definition.result(locale)).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("unknown prompt: {name}")))
}
//#endregion 🔖️Registry

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;
    use crate::protocol::PromptRegistry;

    #[test]
    fn the_prompt_census_matches_the_registry_exactly() {
        let registry = build_prompt_registry();
        let mut listed: Vec<String> = registry.list().into_iter().map(|prompt| prompt.name).collect();
        listed.sort();
        let mut census: Vec<String> = GATEWAY_PROMPT_NAMES.iter().map(|name| (*name).to_string()).collect();
        census.sort();
        assert_eq!(listed, census);
    }

    #[test]
    fn every_prompt_answers_in_both_languages_with_distinct_bodies() {
        let registry = build_prompt_registry();
        for name in GATEWAY_PROMPT_NAMES {
            let english = registry.get(name, Some(serde_json::json!({ "locale": "en" }))).expect("english resolves");
            let german = registry.get(name, Some(serde_json::json!({ "locale": "de" }))).expect("german resolves");
            let (ContentBlock::Text { text: english_text }, ContentBlock::Text { text: german_text }) = (&english.messages[0].content, &german.messages[0].content) else {
                panic!("{name} must answer with text");
            };
            assert!(!english_text.is_empty() && !german_text.is_empty(), "{name} has an empty body");
            assert_ne!(english_text, german_text, "{name} is not actually translated");
        }
    }

    #[test]
    fn an_absent_or_unknown_locale_resolves_to_english_rather_than_failing() {
        let registry = build_prompt_registry();
        let default = registry.get("safe_mutation", None).expect("a missing locale still resolves");
        let nonsense = registry.get("safe_mutation", Some(serde_json::json!({ "locale": "kl" }))).expect("an unknown locale still resolves");
        let english = registry.get("safe_mutation", Some(serde_json::json!({ "locale": "en" }))).expect("english resolves");
        assert_eq!(default.messages, english.messages);
        assert_eq!(nonsense.messages, english.messages);
    }

    #[test]
    fn a_regional_german_tag_resolves_to_german() {
        assert_eq!(PromptLocale::resolve(Some("de-CH")), PromptLocale::De);
        assert_eq!(PromptLocale::resolve(Some("DE")), PromptLocale::De);
        assert_eq!(PromptLocale::resolve(Some(" de ")), PromptLocale::De);
        assert_eq!(PromptLocale::resolve(None), PromptLocale::En);
    }

    #[test]
    fn an_unknown_prompt_name_is_a_well_formed_not_found() {
        let registry = build_prompt_registry();
        let error = registry.get("no_such_prompt", None).expect_err("unknown prompts are not found");
        assert_eq!(error.code, GatewayErrorCode::NotFound);
        assert!(prompt_body("no_such_prompt", PromptLocale::En).is_err());
    }

    /// 🔌️ The whole point of this facet: a prompt teaches the PROTOCOL, so it must never hardcode a
    /// plugin's vocabulary. If a prompt names a specific plugin, installing a different plugin set
    /// would silently make it wrong.
    #[test]
    fn no_prompt_names_a_specific_plugin_or_artifact_kind() {
        for definition in DEFINITIONS {
            for body in [definition.body_en, definition.body_de] {
                let lowered = body.to_ascii_lowercase();
                for forbidden in ["note", "cad", "puzzle", "sketchpad", "procedural"] {
                    assert!(!lowered.contains(forbidden), "{} names the plugin `{forbidden}`", definition.name);
                }
            }
        }
    }
}
//#endregion 🧪️Tests
