//! 💬️ Description law of the catalog: the manifest schema, the description problems, findings and verbs and the description validator. See [`super`].

use super::*;

//#region 🔖️DescriptionLaw
/// 📜️ The manifest's schema document — `CapabilityDescription` is declared there, beside the
/// `ActionSemantics.description` field it constrains.
const MANIFEST_SCHEMA: &str = include_str!("../../../../../../🔨️modules/🛂️manifest/🧬️schema/🔣️.json");

/// 💬️ One way an agent-published verb fails to explain itself — the `CapabilityDescription` contract
/// (`🛂️manifest/🧬️schema/🔣️.json`), in the order [`description_problems`] reports them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DescriptionProblem {
    /// 🕳️ No `semantics.description` at all.
    Missing,
    /// 📐️ A cell violates `CapabilityDescriptionSentence` (empty, too short or long, not one sentence line).
    Schema,
    /// 🌐️ A German cell is the English cell verbatim.
    Untranslated,
    /// 🔁️ A native cell only repeats the verb's own title in that locale.
    RepeatsTitle,
    /// 👯️ Another verb of the same app carries the same English description.
    SharedWithinApp,
}

impl DescriptionProblem {
    /// 🔤️ The fixture/wire spelling shared with the TypeScript twin.
    pub fn as_str(self) -> &'static str {
        match self {
            DescriptionProblem::Missing => "missing",
            DescriptionProblem::Schema => "schema",
            DescriptionProblem::Untranslated => "untranslated",
            DescriptionProblem::RepeatsTitle => "repeatsTitle",
            DescriptionProblem::SharedWithinApp => "sharedWithinApp",
        }
    }
}

/// 🗣️ One agent-published verb of one app as the description law reads it: the id, the native title
/// per locale, and the declared description in the `LocalizedLabel` wire shape a descriptor carries
/// (`{terminology: {locale: text}}`), `None` when undeclared.
#[derive(Clone, Debug, PartialEq)]
pub struct DescribedVerb {
    pub id: String,
    pub title_en: String,
    pub title_de: String,
    pub description: Option<serde_json::Value>,
}

impl DescribedVerb {
    fn from_declaration(id: String, label: &semio_framework_ui_locale::LocalizedLabel, description: Option<&semio_framework_ui_locale::LocalizedLabel>) -> Self {
        Self {
            id,
            title_en: label.resolve(Terminology::Native, Locale::En).to_string(),
            title_de: label.resolve(Terminology::Native, Locale::De).to_string(),
            description: description.map(|description| serde_json::to_value(description).expect("a LocalizedLabel always serializes")),
        }
    }
}

/// 🚨️ One `(capability, problem)` row of the description census. Its message line is
/// `<capability id> [<problem>] <reason>` — the `capability-audit-check` gate parses the first two
/// fields to hold the Rust census against the AJV one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescriptionFinding {
    pub capability_id: String,
    pub problem: DescriptionProblem,
}

impl DescriptionFinding {
    pub fn message(&self) -> String {
        let reason = match self.problem {
            DescriptionProblem::Missing => "declares no description — an agent sees a bare title",
            DescriptionProblem::Schema => "has a description cell that is not one en/de sentence line of 24–480 characters",
            DescriptionProblem::Untranslated => "has a German description cell identical to the English one",
            DescriptionProblem::RepeatsTitle => "has a description that only repeats its title",
            DescriptionProblem::SharedWithinApp => "shares its English description with another verb of the same app",
        };
        format!("{} [{}] {reason}", self.capability_id, self.problem.as_str())
    }
}

/// 📐️ `CapabilityDescription` compiled from the manifest schema document by the repo's owned
/// validator — the same `$defs` the TypeScript twin hands to AJV. Only the `CapabilityDescription*`
/// family is taken: the owned validator checks every def it is given, and sibling defs reference
/// other documents this law never needs.
fn description_validator() -> semio_framework_schema::OwnedJsonSchemaValidator {
    let manifest: serde_json::Value = serde_json::from_str(MANIFEST_SCHEMA).expect("the manifest schema is JSON");
    let family: serde_json::Map<String, serde_json::Value> = manifest["$defs"].as_object().expect("the manifest schema declares $defs").iter().filter(|(id, _)| id.starts_with("CapabilityDescription")).map(|(id, def)| (id.clone(), def.clone())).collect();
    let root = serde_json::json!({ "$schema": manifest["$schema"], "$id": manifest["$id"], "$defs": family, "$ref": "#/$defs/CapabilityDescription" });
    compile_validator(&root).expect("CapabilityDescription compiles")
}

fn description_cell<'value>(description: &'value serde_json::Value, terminology: &str, locale: &str) -> &'value str {
    description.get(terminology).and_then(|row| row.get(locale)).and_then(serde_json::Value::as_str).unwrap_or_default().trim()
}

fn repeats_title(cell: &str, title: &str) -> bool {
    !cell.is_empty() && cell.trim_end_matches(['.', '!', '?']).trim().to_lowercase() == title.trim().to_lowercase()
}

/// ⚖️ The `CapabilityDescription` contract over one app's agent-published verbs: per verb, in
/// declaration order, every [`DescriptionProblem`] it has in enum order (`Missing` alone when there
/// is nothing to judge). Replayed by `🧫️fixtures/💬️capability-description.json` against the
/// TypeScript twin `capabilityDescriptionProblems` (AJV), which must report the identical list.
pub fn description_problems(verbs: &[DescribedVerb]) -> Vec<(String, DescriptionProblem)> {
    let validator = description_validator();
    let mut english: BTreeMap<&str, usize> = BTreeMap::new();
    for description in verbs.iter().filter_map(|verb| verb.description.as_ref()) {
        let cell = description_cell(description, "native", "en");
        if !cell.is_empty() {
            *english.entry(cell).or_insert(0) += 1;
        }
    }
    let mut problems = Vec::new();
    for verb in verbs {
        let Some(description) = &verb.description else {
            problems.push((verb.id.clone(), DescriptionProblem::Missing));
            continue;
        };
        if validate(&validator, description).is_err() {
            problems.push((verb.id.clone(), DescriptionProblem::Schema));
        }
        if ["native", "reuse"].iter().any(|terminology| {
            let en = description_cell(description, terminology, "en");
            !en.is_empty() && en == description_cell(description, terminology, "de")
        }) {
            problems.push((verb.id.clone(), DescriptionProblem::Untranslated));
        }
        if repeats_title(description_cell(description, "native", "en"), &verb.title_en) || repeats_title(description_cell(description, "native", "de"), &verb.title_de) {
            problems.push((verb.id.clone(), DescriptionProblem::RepeatsTitle));
        }
        if english.get(description_cell(description, "native", "en")).is_some_and(|count| *count > 1) {
            problems.push((verb.id.clone(), DescriptionProblem::SharedWithinApp));
        }
    }
    problems
}

/// 🧾️ The description census over a `CatalogSource`: [`description_problems`] once per app of every
/// descriptor (its agent-published window-kind, app-scope, app and mode verbs, framework-injected
/// ids excluded), once per plugin-scope command set, and once over the framework-injected verbs every
/// app receives (the framework's own definitions, deduped by id). Empty means every verb an agent
/// can be offered explains itself in English and German. Sorted by capability id.
pub fn description_findings(source: &CatalogSource) -> Vec<DescriptionFinding> {
    let mut findings = Vec::new();
    let mut collect = |prefix: &str, verbs: Vec<DescribedVerb>| {
        for (verb, problem) in description_problems(&verbs) {
            findings.push(DescriptionFinding { capability_id: format!("{prefix}{verb}"), problem });
        }
    };
    let mut framework: BTreeMap<String, DescribedVerb> = BTreeMap::new();
    for descriptor in &source.descriptors {
        let plugin_id = descriptor.manifest.plugin_id.as_str();
        for app in descriptor.manifest.apps.iter() {
            let mut verbs = Vec::new();
            if let Ok(declared) = app_action_verbs(app) {
                for (action, _) in declared {
                    if !is_framework_injected_action(action) && manifest::resolve_audience(action) == manifest::CapabilityAudience::Agent {
                        verbs.push(DescribedVerb::from_declaration(action.id.clone(), &action.label, action.semantics.description.as_ref()));
                    }
                }
            }
            for command in &app.commands {
                if manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent {
                    verbs.push(DescribedVerb::from_declaration(format!("cmd.{}", command.id), &command.label, command.semantics.description.as_ref()));
                }
            }
            for mode in app.modes.iter() {
                for command in &mode.commands {
                    if manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent {
                        verbs.push(DescribedVerb::from_declaration(format!("mode.{}.{}", mode.id, command.id), &command.label, command.semantics.description.as_ref()));
                    }
                }
            }
            collect(&format!("{plugin_id}.{}.", app.id), verbs);
            for action in framework_action_definitions(app) {
                if manifest::resolve_audience(&action) == manifest::CapabilityAudience::Agent {
                    framework.entry(action.id.clone()).or_insert_with(|| DescribedVerb::from_declaration(action.id.clone(), &action.label, action.semantics.description.as_ref()));
                }
            }
        }
        let commands = descriptor.manifest.commands.iter().filter(|command| manifest::resolve_command_audience(command) == manifest::CapabilityAudience::Agent);
        collect(&format!("{plugin_id}.cmd."), commands.map(|command| DescribedVerb::from_declaration(command.id.clone(), &command.label, command.semantics.description.as_ref())).collect());
    }
    collect("framework.", framework.into_values().collect());
    findings.sort_by(|left, right| left.capability_id.cmp(&right.capability_id).then(left.problem.cmp(&right.problem)));
    findings
}
//#endregion 🔖️DescriptionLaw
