//! 📚️ Example source and app: the example source body and leaf, the built app with its examples and the asynchronous builder entry. See [`super`].

use super::*;

/// 📚️ How an example leaf carries its document body. `Inline` is a body that already exists as a
/// string when the bundle is assembled; `Deferred` is a body that is DERIVED from authored
/// fixture bytes by a producer the leaf hands over, and is therefore never materialised unless
/// something actually asks for the document.
///
/// 🐛️ `📓️a3-descriptor-regeneration.md` §3 / `📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §4:
/// `🧩️puzzle`'s `capsule-dream` leaf parsed 3 035 200 B of DSL and re-serialised it to JSON
/// inside a `LazyLock` that the subset's `examples()` dereferenced — so the parse ran while the
/// bundle was being ASSEMBLED, before `describe()` had built anything, and `describe` could not
/// finish inside its 1 800 s guest epoch. A `Deferred` body moves that work behind the producer
/// and declares the example by the identity of the bytes the author actually wrote, which needs
/// no parse: `source.len()` and `sha256(source)`.
#[derive(Clone, Debug, PartialEq)]
pub enum ExampleSourceBody {
    Inline(String),
    Deferred { produce: fn() -> String, source_suffix: &'static str, source: &'static [u8] },
}

/// 📚️ Value type exported by an example definition leaf (`📚️examples/<slug>/🦀️.rs`):
/// stable id, localized label, icon, and document/payload accessors — converts into
/// [`ExampleDefinition`] for [`PluginManifest::examples`].
#[derive(Clone, Debug, PartialEq)]
pub struct ExampleSource {
    pub(super) id: String,
    pub(super) label: LocalizedLabel,
    pub(super) icon_id: IconName,
    body: ExampleSourceBody,
}

impl ExampleSource {
    /// 🧱 Builds an example source from id, label, document payload, and icon.
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), body: ExampleSourceBody::Inline(document_json.into()) }
    }

    /// 🧱 Builds an example source whose body is DERIVED from authored fixture bytes: `produce`
    /// is a non-capturing producer the leaf owns (typically "parse the DSL, re-serialise it to
    /// JSON"), `source` are those authored bytes and `source_suffix` their own file extension
    /// (`".dsl.semio"`, `".json"`, …). Nothing calls `produce` until a consumer asks for the
    /// document — the manifest row and the descriptor's `AssetDeclaration` are both answered
    /// from `source` alone. See [`ExampleSourceBody`] for the measured defect this exists for.
    pub fn deferred(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>, source_suffix: &'static str, source: &'static [u8], produce: fn() -> String) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), body: ExampleSourceBody::Deferred { produce, source_suffix, source } }
    }

    /// 🏷️ Stable example id (navbar picker / `setActiveExample`).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 🗣️ Localized label sourced from the definition leaf.
    pub fn label(&self) -> &LocalizedLabel {
        &self.label
    }

    /// 🖼️ Icon id sourced from the definition leaf.
    pub fn icon_id(&self) -> IconName {
        self.icon_id
    }

    /// 📚️ How this leaf carries its body — see [`ExampleSourceBody`].
    pub fn body(&self) -> &ExampleSourceBody {
        &self.body
    }

    /// 📄️ Document JSON payload registered on the manifest. A deferred body is produced HERE and
    /// nowhere else, so a caller that only wants the example's identity never pays for it.
    pub fn document_json(&self) -> String {
        match &self.body {
            ExampleSourceBody::Inline(document_json) => document_json.clone(),
            ExampleSourceBody::Deferred { produce, .. } => produce(),
        }
    }

    /// 📄️ Alias of [`Self::document_json`] for leaf call sites that speak in document terms.
    pub fn document(&self) -> String {
        self.document_json()
    }

    /// 📒️ Alias of [`Self::document_json`] for leaf call sites that speak in payload terms.
    pub fn payload(&self) -> String {
        self.document_json()
    }

    /// 📦️ The descriptor-side declaration of a DEFERRED body: name, media type, exact size and
    /// SHA-256 of the AUTHORED bytes the producer derives the document from. `None` for an
    /// inline body — those are declared, if they need to be, by
    /// `describe::externalize_oversized_example_bodies` from the body it actually holds.
    pub fn deferred_body_asset(&self, dialect: &ArtifactDialect, media_type: MediaType) -> Option<semio_framework::AssetDeclaration> {
        let ExampleSourceBody::Deferred { source_suffix, source, .. } = &self.body else {
            return None;
        };
        Some(semio_framework::AssetDeclaration { name: semio_framework::example_body_asset_name(dialect, &self.id, source_suffix), media_type, size_bytes: source.len() as u64, sha256: semio_framework_hash::sha256_hex(source) })
    }

    /// 🧬️ Converts into a manifest [`ExampleDefinition`] for `dialect` — the coordinate the
    /// example is authored against, taken from the registering surface's own
    /// `AppDefinition.dialect` (see `Plugin::register_app_factory`). There is deliberately no
    /// `From<ExampleSource> for ExampleDefinition`: a source alone cannot know its dialect, and
    /// an unstamped definition would be a manifest row no picker can resolve.
    ///
    /// A DEFERRED body leaves `artifact_json` empty, exactly as an externalized oversized body
    /// does: the row keeps its identity so every picker resolves it, and the body travels as the
    /// `AssetDeclaration` [`Self::deferred_body_asset`] declares.
    pub fn into_example_definition(self, dialect: ArtifactDialect) -> ExampleDefinition {
        let artifact_json = match self.body {
            ExampleSourceBody::Inline(document_json) => document_json,
            ExampleSourceBody::Deferred { .. } => String::new(),
        };
        ExampleDefinition { id: self.id, label: self.label, icon_id: self.icon_id, artifact_json, dialect }
    }
}

impl From<&ExampleSource> for ExampleSource {
    fn from(source: &ExampleSource) -> Self {
        source.clone()
    }
}

/// 📚️ `examples` stays a `Vec<ExampleSource>` all the way to registration: an example belongs to
/// a DIALECT, and the dialect is only known where `definition` is (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END).
pub struct App {
    pub definition: AppDefinition,
    pub examples: Vec<ExampleSource>,
}

impl App {
    pub async fn builder(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> AppBuilder {
        AppBuilder::new(id, label).await
    }

    /// 🗂️ Resource kinds declared via `.artifact_kind(...)` — see `AppDefinition.artifact_kinds`
    /// (round-trips through the plugin manifest; `semio_framework_os`'s artifact catalog registry
    /// consumes it from there at plugin registration time).
    pub async fn from_builder(builder: AppBuilder) -> Self {
        Self { definition: builder.build_definition(), examples: Vec::new() }
    }

    /// 🧷️ Fallible twin of `from_builder` — see `AppBuilder::try_build_definition`.
    pub async fn try_from_builder(builder: AppBuilder) -> Result<Self, PluginAssemblyError> {
        Ok(Self { definition: builder.try_build_definition()?, examples: Vec::new() })
    }

    /// 📚️ Registers an example from a definition-leaf [`ExampleSource`] (canonical path).
    pub async fn example_source(mut self, source: impl Into<ExampleSource>) -> Self {
        self.examples.push(source.into());
        self
    }

    /// 📚️ Registers an example by restating id/label/payload/icon.
    /// Prefer [`Self::example_source`] once the definition leaf owns those fields; kept so
    /// existing `create_*_app()` call sites keep compiling during the example-shape migration.
    pub async fn example(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
        self.example_source(ExampleSource::new(id, label, document_json, icon_id)).await
    }

    /// 🚧️ B1 stub: `WorkflowDefinition` + `PluginManifest.workflows` were deleted from framework-core
    /// (WP-0.1, concurrent) — real workflow-palette derivation moves to `register_app_io`/
    /// `workflow_palette()` in Wave 1 (`AppIo`-driven, not this free-text `yields`). Kept as a no-op
    /// so the ~50 existing `.workflow(...)` call sites across every app manifest keep compiling
    /// unchanged until their own wave touches them.
    pub async fn workflow(self, _workflow_step_id: impl Into<String>, _label: impl Into<String>, _yields: impl Into<String>) -> Self {
        self
    }
}
