//! 📫️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use super::*;

impl<A: ArtifactApp> InstanceStores<A> {
    /// 🪪️ A command's whole-document replacement names this store's identity, not whatever the app
    /// minted: see [`store::os_store::text_format::stamp_document_spr_identity`].
    pub(crate) fn stamp_load_document_identity(&self, effect: Effect) -> Result<Effect, Fault> {
        let Effect::LoadDocument { pack, spr } = effect else { return Ok(effect) };
        let envelope = self.store.envelope();
        let identity = ArtifactRef { artifact_id: envelope.id.clone(), dialect: A::DIALECT.into() };
        let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::stamp_document_spr_identity(&spr, &identity.artifact_id, A::DOCUMENT_SCHEMA, &identity.dialect, envelope.owner.as_ref()))
            .map_err(|error| plugin_sdk_fault(format!("load-document identity stamp failed: {error}")))?;
        Ok(Effect::LoadDocument { pack, spr })
    }

    /// 🚪️ EVERY effect leaving this instance passes the stamp, not just the mounted typed-operation
    /// ladder's. An app mints its replacement log with `store::os_store::text_format::empty_document_spr`, which carries no
    /// composition section at all, and the archive loader's parent hydration refuses such a log
    /// outright (`MemberOpenDiagnostic::Identity`, `💧️hydration/🦀️.rs` `Phase::Begin`) before any of
    /// the three replacement legs even runs. The stamp is therefore not an optimisation on one lane,
    /// it is the whole contract between `Effect::LoadDocument` and `load_document_archive` — and the
    /// plain dispatch lane and the refresh poll both used to skip it, which is why every
    /// `setActiveExample` whose verb is not a mounted retained tool died in the browser.
    pub(crate) fn stamp_load_document_effects(&self, effects: Vec<Effect>) -> Result<Vec<Effect>, Fault> {
        if !effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return Ok(effects);
        }
        effects.into_iter().map(|effect| self.stamp_load_document_identity(effect)).collect()
    }

    /// 🪪️ The refresh poll's twin — that lane has no error channel, so an undecodable log rides on
    /// unchanged and the archive loader reports its own identity refusal, rather than the effect
    /// disappearing without a word.
    pub(crate) fn stamp_polled_load_document_effects(&self, effects: Vec<Effect>) -> Vec<Effect> {
        if !effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })) {
            return effects;
        }
        let envelope = self.store.envelope();
        let dialect: ArtifactDialect = A::DIALECT.into();
        effects
            .into_iter()
            .map(|effect| match effect {
                Effect::LoadDocument { pack, spr } => {
                    let stamped = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::stamp_document_spr_identity(&spr, &envelope.id, A::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref())).ok();
                    Effect::LoadDocument { pack, spr: stamped.unwrap_or(spr) }
                }
                other => other,
            })
            .collect()
    }
}
