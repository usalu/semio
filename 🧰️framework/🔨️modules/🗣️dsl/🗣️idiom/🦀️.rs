//! 🗣️ Generic idiom hooks, language models and explicit registration.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::{TextError,TextSpan,TokenClass};
use crate::grammar::{GrammarFile,ProtocolFile,SemioDialect,parse_grammar,parse_protocol,verify_protocol_source};

pub trait DslIdiom {
    /// Stable registry id — the `lang` string a `#[dsl(lang = "...")]` field names.
    const LANG: &'static str;
    type Ast: Clone + PartialEq + Send + Sync;

    // 🚫️async: E4 fn-pointer slot — every method here is coerced into `IdiomHooks`'s plain `fn`
    // fields (`canonicalize`/`classify`/`complete`) in `hooks_for` below; an `fn` item's
    // pointer type is unnameable, so this whole trait must stay sync. See R2 E4.
    fn parse(text: &str) -> Result<Self::Ast, TextError>;
    /// LAW: `Self::parse(&Self::print(ast)) == Ok(ast)` for every `ast` the idiom can produce —
    /// the idiom's own round-trip law, the direct analogue of this engine's `parse ∘ print = id`
    /// for `RecordSpec` grammars.
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn print(ast: &Self::Ast) -> String;
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn classify(text: &str) -> Vec<(TokenClass, TextSpan)>;
    // 🚫️async: E4 fn-pointer slot — see `parse` above
    fn complete(_text: &str, _offset: usize) -> Vec<CompletionItem> {
        Vec::new()
    }
}

/// 🧩️ An owned language completion item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

/// 📇️ Type-erased vtable for one registered idiom — what `Shape::Embed` canonicalization
/// and `LanguageService` fence delegation call through, without depending on the idiom's own crate
/// (which would be a dependency cycle: the idiom depends on `dsl`, not the reverse).
#[derive(Clone, Copy)]
pub struct IdiomHooks {
    pub lang: &'static str,
    /// `print ∘ parse` — `Err` propagates the idiom's own parse diagnostic unchanged.
    pub canonicalize: fn(&str) -> Result<String, TextError>,
    pub classify: fn(&str) -> Vec<(TokenClass, TextSpan)>,
    pub complete: fn(&str, usize) -> Vec<CompletionItem>,
}

/// 🏗️ Derives an `IdiomHooks` vtable from a `DslIdiom` impl — the one place `Self::Ast`
/// needs to be named, so every other caller works with the type-erased `IdiomHooks` instead.
// 🚫️async: E4 fn-pointer slot — builds an `IdiomHooks` whose fields are plain `fn` pointers; an
// `fn`'s captured closure cannot coerce to `fn`, so this stays sync. See R2 E4.
pub fn hooks_for<I: DslIdiom>() -> IdiomHooks {
    IdiomHooks { lang: I::LANG, canonicalize: |text| I::parse(text).map(|ast| I::print(&ast)), classify: I::classify, complete: I::complete }
}

/// 🪞 Minimal hooks for binary/text facets that register a [`LanguageSpec`] without a custom
/// [`DslIdiom`] front-end — canonicalize is identity; classify/complete are empty.
// 🚫️async: E4 fn-pointer slot — see `hooks_for` above
pub fn passthrough_hooks(lang: &'static str) -> IdiomHooks {
    IdiomHooks { lang, canonicalize: |text| Ok(text.to_string()), classify: |_| Vec::new(), complete: |_, _| Vec::new() }
}

static IDIOM_REGISTRY: OnceLock<Mutex<HashMap<&'static str, IdiomHooks>>> = OnceLock::new();

// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
fn idiom_registry() -> &'static Mutex<HashMap<&'static str, IdiomHooks>> {
    IDIOM_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 📌️ Registers an idiom's hooks under its `LANG` id — called once at host/plugin init.
/// Re-registering the same `lang` overwrites the previous hooks rather than erroring, so a
/// hot-reloaded dev build never deadlocks on itself.
// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
pub fn register_idiom(hooks: IdiomHooks) {
    let mut registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(hooks.lang, hooks);
}

/// 🔍️ Looks up a previously-registered idiom's hooks by `lang` id. `None` for an
/// unregistered (or not-yet-registered) lang — callers must treat that as "pass through verbatim",
/// never as an error, since `Shape::Embed` text must remain parseable before any plugin has run
/// its own registration.
// 🚫️async: E1 pure accessor consumed by the E4 `IdiomHooks` cluster — see R9
pub fn idiom(lang: &str) -> Option<IdiomHooks> {
    let registry = idiom_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(lang).copied()
}

/// 🎭️ Which surface a registered [`LanguageSpec`] describes for the
/// `handcrafted-grammar-for-every-artifact` program.
///
/// Text roles carry a `.grammar.semio` (`grammar` / `grammar_path`): `Document` (`🗣️dsl`),
/// `Config`, `Ops` (`🔧️op`), `Embedded` (`Shape::Embed` idiom), and `Diff` (`🔺️diff`).
/// Binary roles carry a `.protocol.semio` (`protocol` / `protocol_path`): `Pack` (`🎒️pack`)
/// and `Spr` (`📡️spr`). Never put grammar files on pack/spr or protocol files on dsl/op/diff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageRole {
    Document,
    Config,
    Ops,
    Embedded,
    Diff,
    Pack,
    Spr,
}

/// 📖️ One artifact facet language, registered once at plugin init: identity, the extension
/// it opens (documents/configs only), optional hand-authored **grammar** text for text surfaces
/// (`🗣️dsl` / `🔧️op` / `🔺️diff`, `dialect grammar`), optional hand-authored **protocol** text for
/// binary surfaces (`🎒️pack` / `📡️spr`, `dialect protocol`), and the [`IdiomHooks`] vtable used by
/// text hosts (`LanguageSession`, writer). Additive alongside `IdiomHooks`/`register_idiom`.
#[derive(Clone, Copy)]
pub struct LanguageSpec {
    pub id: &'static str,
    pub extension: Option<&'static str>,
    pub role: LanguageRole,
    pub grammar: Option<&'static str>,
    pub grammar_path: Option<&'static str>,
    pub protocol: Option<&'static str>,
    pub protocol_path: Option<&'static str>,
    pub hooks: IdiomHooks,
}

impl LanguageSpec {
    /// 📝 Whether this role is a text grammar surface (dsl/op/diff/config/embed).
    // 🚫️async: E1 pure accessor — trivial enum match, no suspension point — see R9
    pub fn is_text_role(self) -> bool {
        matches!(self.role, LanguageRole::Document | LanguageRole::Config | LanguageRole::Ops | LanguageRole::Embedded | LanguageRole::Diff)
    }

    /// 📡️ Whether this role is a binary protocol surface (pack/spr).
    // 🚫️async: E1 pure accessor — trivial enum match, no suspension point — see R9
    pub fn is_binary_role(self) -> bool {
        matches!(self.role, LanguageRole::Pack | LanguageRole::Spr)
    }

    /// 📖️ Parses `grammar` via [`parse_grammar`], requiring [`SemioDialect::Grammar`].
    pub fn parsed_grammar(&self) -> Result<Option<GrammarFile>, TextError> {
        let Some(text) = self.grammar else {
            return Ok(None);
        };
        let file = parse_grammar(text)?;
        if file.dialect != SemioDialect::Grammar {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "LanguageSpec.grammar requires dialect grammar", TextSpan::at(1, 1)));
        }
        Ok(Some(file))
    }

    /// 📡️ Parses `protocol` via [`parse_protocol`].
    pub fn parsed_protocol(&self) -> Result<Option<ProtocolFile>, TextError> {
        let Some(text) = self.protocol else {
            return Ok(None);
        };
        Ok(Some(parse_protocol(text)?))
    }

    /// ✅ Verifies encoded bytes against this language's protocol when protocol text is present.
    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {
        let Some(text) = self.protocol else {
            return Ok(());
        };
        verify_protocol_source(text, bytes)
    }
}

/// 🪪 Pass-through [`IdiomHooks`] for binary facets (pack/spr) and text facets without a
/// dedicated `DslIdiom` yet — canonicalize is identity; classify/complete are empty.

static LANGUAGE_REGISTRY: OnceLock<Mutex<HashMap<&'static str, LanguageSpec>>> = OnceLock::new();

// 🚫️async: E1 pure accessor — plain `OnceLock`/`Mutex` init, no suspension point — see R9. Its two
// PUBLIC callers that cross into `🔌️plugin/🦀️.rs` (a live ATOMIC packet's file, not mine to
// touch) stay `fn` themselves — see `preflight_languages`/`register_languages` below — so that
// external `` call shape needs no change; only this private accessor and the purely-local
// lookups (`language`/`language_for_extension`/…) revert to sync.
fn language_registry() -> &'static Mutex<HashMap<&'static str, LanguageSpec>> {
    LANGUAGE_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// ⚠️ Language registration rejects a distinct owner for an established language id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageRegistryError {
    pub id: String,
}

impl std::fmt::Display for LanguageRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "language registration conflicts for {}", self.id)
    }
}

impl std::error::Error for LanguageRegistryError {}

fn same_language(left: LanguageSpec, right: LanguageSpec) -> bool {
    left.id == right.id
        && left.extension == right.extension
        && left.role == right.role
        && left.grammar == right.grammar
        && left.grammar_path == right.grammar_path
        && left.protocol == right.protocol
        && left.protocol_path == right.protocol_path
        && left.hooks.lang == right.hooks.lang
        && std::ptr::fn_addr_eq(left.hooks.canonicalize, right.hooks.canonicalize)
        && std::ptr::fn_addr_eq(left.hooks.classify, right.hooks.classify)
        && std::ptr::fn_addr_eq(left.hooks.complete, right.hooks.complete)
}

/// 🔬️ Verifies language specifications against established and intra-batch identities without mutation.
/// Stays `fn` — no internal suspension point (`language_registry` is sync, R9), but
/// `🔌️plugin/🦀️.rs` (a live ATOMIC packet's file, not mine to touch) already awaits this,
/// so per R9 rule 3 the external `` call shape wins over reverting it to match a leaf helper.
#[must_use]
pub fn preflight_languages(specs: &[LanguageSpec]) -> Result<(), LanguageRegistryError> {
    let mut proposed = HashMap::new();
    for spec in specs {
        match proposed.insert(spec.id, *spec) {
            Some(existing) if same_language(existing, *spec) => {}
            Some(_) => return Err(LanguageRegistryError { id: spec.id.to_string() }),
            None => {}
        }
    }
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    for spec in specs {
        if let Some(existing) = registry.get(spec.id) {
            if !same_language(*existing, *spec) {
                return Err(LanguageRegistryError { id: spec.id.to_string() });
            }
        }
    }
    Ok(())
}

/// 📌️ Registers language specifications only after the whole candidate set is conflict-free.
/// Stays `fn` — see `preflight_languages` above, same external-caller reason.
#[must_use]
pub fn register_languages(specs: Vec<LanguageSpec>) -> Result<(), LanguageRegistryError> {
    preflight_languages(&specs)?;
    let mut registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    for spec in specs {
        registry.entry(spec.id).or_insert(spec);
    }
    Ok(())
}

/// 📌️ Registers one grammar under its `id` — called once per grammar at plugin init,
/// alongside (not instead of) `register_document_codec_for_app`. Overwrites on re-registration,
/// matching `register_idiom`'s hot-reload-safe behavior.
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn register_language(spec: LanguageSpec) {
    let mut registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.insert(spec.id, spec);
}

/// 🔍️ Looks up a registered grammar by its `id` (e.g. `"fem2d"`, `"fem2dcfg"`, `"jack"`).
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language(id: &str) -> Option<LanguageSpec> {
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.get(id).copied()
}

/// 🔍️ Looks up a registered grammar by legacy file-extension suffix (e.g. `"note"`, `"jack"`).
// 🚫️async: E1 pure accessor — see `language_registry` above
pub fn language_for_extension(extension: &str) -> Option<LanguageSpec> {
    let suffix = extension.strip_prefix('.').unwrap_or(extension);
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.values().find(|spec| spec.extension == Some(suffix)).copied()
}

/// 🔍️ Finds a registered language by an explicit role and extension selection.
pub fn language_for_role_extension(role: LanguageRole, extension: &str) -> Option<LanguageSpec> {
    let registry = language_registry().lock().unwrap_or_else(|poison| poison.into_inner());
    registry.values().find(|spec| spec.role == role && spec.extension == Some(extension)).copied()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
