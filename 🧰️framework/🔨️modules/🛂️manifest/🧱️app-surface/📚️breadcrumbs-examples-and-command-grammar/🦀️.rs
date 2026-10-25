//! 📚️ Breadcrumbs, examples and command grammar of an app: the app breadcrumb and window label, the example definitions, the config spec and the command grammar. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
/// 🪜️ Formats a canonical app breadcrumb for chrome.
pub fn app_breadcrumb(breadcrumb: &[String]) -> String {
    breadcrumb.join(" · ")
}

/// 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to the canonical breadcrumb.
pub fn resolve_app_breadcrumb<'a>(app: &'a AppDefinition, terminology: &str) -> &'a [String] {
    app.terminology_breadcrumbs.get(terminology).map_or(&app.breadcrumb, Vec::as_slice)
}

/// 🗂️ Formats a window tab within its canonical app breadcrumb, resolved under the active terminology
/// and `locale` (needed to resolve the now-`LocalizedLabel` `app.label` for the dedup comparison below).
pub fn app_window_label(app: &AppDefinition, terminology: &str, locale: Locale, window_label: &str) -> String {
    let mut breadcrumb = resolve_app_breadcrumb(app, terminology).to_vec();
    let normalized_window = window_label.trim().to_lowercase();
    let normalized_app = app.label.resolve(Terminology::parse(terminology).unwrap_or(Terminology::Native), locale).trim().to_lowercase();
    if !normalized_window.is_empty() && normalized_window != normalized_app && breadcrumb.last().is_none_or(|segment| segment.to_lowercase() != normalized_window) {
        breadcrumb.push(normalized_window);
    }
    app_breadcrumb(&breadcrumb)
}

// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ExampleDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub artifact_json: String,
    /// 🎯️ The DIALECT this example is authored for, never one app of it. An example is a document of
    /// an artifact's subset, and every surface bound to that subset — the editor and the viewer
    /// alike — opens the same eight fixtures; `SubsetDeclaration.examples` already models them that
    /// way. Stamped at registration from the registering app's own `AppDefinition.dialect`, and
    /// resolved by [`examples_for_app`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub dialect: ArtifactDialect,
}

/// 🏷️ One asset-name component: everything outside `[0-9A-Za-z._-]` folds to `-` so a dialect
/// coordinate (`s.puzzle.5d@1/*`) and an example id yield a single stable path segment.
pub fn asset_name_segment(raw: &str) -> String {
    raw.chars().map(|character| if character.is_ascii_alphanumeric() || character == '.' || character == '_' || character == '-' { character } else { '-' }).collect()
}

/// 📦️ The directory-and-stem prefix every externalized example body of `example_id` travels under.
/// The suffix after it names the BYTES that were declared: `.json` for a body the descriptor
/// emitter moved out of the manifest, and the authored fixture's own extension for a body the
/// plugin never materialised at all (see `ExampleSource::deferred`). Consumers match the prefix,
/// never one fixed extension, because those two declarations hash different bytes on purpose.
pub fn example_body_asset_prefix(dialect: &ArtifactDialect, example_id: &str) -> String {
    format!(
        "📚️examples/{}.{}.{}/{}",
        asset_name_segment(&dialect.artifact_kind),
        asset_name_segment(&dialect.standard),
        asset_name_segment(&dialect.subset),
        asset_name_segment(example_id)
    )
}

/// 🔤️ The full `AssetDeclaration.name` for one example body — [`example_body_asset_prefix`] plus the
/// suffix of the bytes actually declared (`.json`, `.dsl.semio`, …).
pub fn example_body_asset_name(dialect: &ArtifactDialect, example_id: &str, suffix: &str) -> String {
    format!("{}{}", example_body_asset_prefix(dialect, example_id), suffix)
}

/// 🧪️ THE example-picker predicate — every example authored for `dialect`, in manifest order,
/// deduplicated by id. The react shell's navbar select (`ShellHost`'s `exampleOptions`) answers it,
/// and so does the TypeScript twin `examplesForDialect` (`🛂️manifest/🟦️.ts`); both are pinned against the shared
/// `🧫️fixtures/📚️example-picker.json` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn examples_for_dialect<'a>(examples: &'a [ExampleDefinition], dialect: &ArtifactDialect) -> Vec<&'a ExampleDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    examples.iter().filter(|example| &example.dialect == dialect).filter(|example| seen.insert(example.id.clone())).collect()
}

/// 🎁️ The examples one surface may offer — [`examples_for_dialect`] against that surface's own
/// dialect. Role plays no part: an editor and its viewer are two surfaces of ONE dialect and offer
/// exactly the same picker.
pub fn examples_for_app<'a>(examples: &'a [ExampleDefinition], app: &AppDefinition) -> Vec<&'a ExampleDefinition> {
    examples_for_dialect(examples, &app.dialect)
}

//#region 🔖️MediaVocabulary
//#region 🔖️ConfigSpec
/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries: one [`ActionArgDef`] per field (its `id` is the config key), so a config field,
/// an action argument and a mutation input share one input vocabulary and one renderer.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default)]
    #[value(default)]
    pub fields: Vec<ActionArgDef>,
}

impl ConfigSpec {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar — one
/// [`ActionArgDef`] per field, `required` where the grammar demands the field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<ActionArgDef>,
}

/// ⌨️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default)]
    #[value(default)]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️CommandGrammar
//#endregion 🔖️MediaVocabulary
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
