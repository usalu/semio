//! 🌉️ Compose dispatch: the local-first compose dispatch with its settable cross-plugin fallback.
//!
//! Holds the `🔖️Dispatch` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️Dispatch
/// 🌉️ The seam that makes cross-plugin compose real. `io_dispatch` ALWAYS tries the local
/// registry first (the fast, common, same-crate case every existing caller already used via
/// `resolve`+`compose`); on a local miss it falls through to a settable hook instead of failing
/// outright. Native shells install a router-backed hook (resolves through every loaded plugin,
/// see the plugin host's `IoRouter`); a wasm guest installs a hook that calls the `io-compose`
/// host import, which the host then routes to whichever OTHER plugin actually owns the key. Until
/// a hook is installed (or in a context with nothing to fall through to, e.g. a bare unit test)
/// this behaves exactly like `resolve`+`compose` did before -- existing single-crate callers are
/// unaffected by this seam's mere existence.
/// 🔌️ A host-owned erased fallback executable for cross-plugin IO dispatch. Async to match
/// `AsyncComposeFn` (io-async-signatures): `None` means "no opinion, fall through to the
/// `IoResolveError`"; `Some(future)` is the fallback's own compose future, exactly as if it were
/// a local `ComposerEntry.compose` call.
pub type IoFallback = dyn for<'a> Fn(&'a IoKey, &'a [ErasedComposeSource]) -> Option<ComposeFuture<'a>> + Send + Sync;

/// 🪪️ A fallback descriptor plus the exact executable allocation that owns it.
#[derive(Clone)]
pub struct IoFallbackDispatcher {
    pub identity: String,
    pub dispatch: std::sync::Arc<IoFallback>,
}

static IO_FALLBACK: std::sync::OnceLock<IoFallbackDispatcher> = std::sync::OnceLock::new();

/// ⚠️ A fallback identity was already registered with a different descriptor or executable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoFallbackRegistrationError {
    pub established_identity: String,
    pub incoming_identity: String,
}

/// 🔌️ Install the fallback dispatcher. Call exactly once, before any `io_dispatch` call that
/// should reach it (host boot / the guest runtime ensure `plugin_exports!` generates). Re-registration is idempotent
/// only for the same descriptor and executable identity; every other race is a typed conflict.
pub async fn set_io_fallback_dispatcher(dispatcher: IoFallbackDispatcher) -> Result<(), IoFallbackRegistrationError> {
    match IO_FALLBACK.get() {
        Some(existing) if existing.identity == dispatcher.identity && std::sync::Arc::ptr_eq(&existing.dispatch, &dispatcher.dispatch) => Ok(()),
        Some(existing) => Err(IoFallbackRegistrationError { established_identity: existing.identity.clone(), incoming_identity: dispatcher.identity }),
        None => match IO_FALLBACK.set(dispatcher) {
            Ok(()) => Ok(()),
            Err(incoming) => match IO_FALLBACK.get() {
                Some(existing) => Err(IoFallbackRegistrationError { established_identity: existing.identity.clone(), incoming_identity: incoming.identity }),
                None => Err(IoFallbackRegistrationError { established_identity: incoming.identity.clone(), incoming_identity: incoming.identity }),
            },
        },
    }
}

/// 🎹️ Resolve `key` locally; on a local miss, ask the installed fallback (if any). Returns the
/// SAME `IoResolveError`-shaped message as a local-only `resolve` when nothing (local or
/// fallback) has the key, so existing error-message-matching callers don't need to change.
pub async fn io_dispatch(key: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    match resolve(key).await {
        Ok(entry) => validate_composed_subset((entry.compose)(sources).await?).await,
        Err(local_err) => match IO_FALLBACK.get().and_then(|dispatcher| (dispatcher.dispatch)(key, sources)) {
            Some(fallback) => validate_composed_subset(fallback.await?).await,
            None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
        },
    }
}

async fn validate_composed_subset(mut composed: ComposedArtifact) -> Result<ComposedArtifact, ComposeError> {
    run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).await.map_err(|error| ComposeError { message: format!("subset validation failed: {error:?}"), diagnostics: Vec::new() })?;
    Ok(composed)
}

/// 🌉️🌉️ Two-hop compose: resolve+compose `hub` from `sources` via `io_dispatch`, then feed that
/// hop's `ComposedArtifact` as the SINGLE source for resolving+composing `target`, also via
/// `io_dispatch` (so both hops get the fallback dispatcher and subset validation `io_dispatch`
/// already gives a single-hop compose, for free). Built for the domain-plugin hub-and-spoke shape
/// (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): a domain
/// artifact composes into a semio subset (the hub), then the semio subset composes into the real
/// target format — never more than 2 hops.
///
/// **Max-2-hops invariant**: `hub` MUST be resolvable directly from `sources` (hop 1), and `target`
/// MUST be resolvable from hub's OWN composed output alone (hop 2) — never from `sources` again and
/// never chained through a third key. This is a deliberate ceiling, not an oversight: an unbounded
/// transitive walk over the registry can cycle (A resolves via B resolves via A) or blow up
/// combinatorially as more dialects register, and neither failure mode is diagnosable from a single
/// stack frame the way a fixed 2-hop call is. Callers that need a longer chain compose it themselves
/// as repeated `io_compose_via`/`io_dispatch` calls, each one an explicit, auditable hop.
pub async fn io_compose_via(hub: &IoKey, target: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let hub_composed = io_dispatch(hub, sources).await?;
    let hop_source = ErasedComposeSource { dialect: hub_composed.dialect, payload: hub_composed.payload };
    io_dispatch(target, std::slice::from_ref(&hop_source)).await
}
//#endregion 🔖️Dispatch
