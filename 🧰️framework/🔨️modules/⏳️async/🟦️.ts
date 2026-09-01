/** ⏳️ TypeScript surface for `semio-framework-async`: the owned-schema mirror of the pure async-runtime
 * vocabulary (`TraceId`/`CapabilityTokenId`/`CancelState`/`ScopeId`/`ScopeDrainReport`/
 * `ChannelPolicy`/`ThreadPlan`/`ThreadRole`). Regenerate via
 * `bun nx run @semio-tech/framework-async-rs:typegen`.
 *
 * `OperationContext`/`CancelToken`/`ScopeHandle`/`HostAsyncRuntime`/`ManualRuntime` are NOT
 * mirrored here: they carry a live in-process handle (`CancelToken`'s `Arc`) or are a Rust-only
 * trait/executor, never wire data — same reasoning `🦀️.rs`'s own doc comments give for
 * why those types skip `Serialize`/`Deserialize`.
 */
export * from "./🤖️generated/🟦️async.js";

//#region 🌐️WebAsyncScope
/** 🌐️ Documented seam for a future browser/worker-based `HostAsyncRuntime` host — NOT implemented
 * yet (no runtime backs this interface anywhere in the tree today; the concrete host is a later
 * packet). It mirrors the shape of the Rust trait's scope-lifecycle methods
 * (`open_scope`/`cancel_scope`) for a host that would be driven by `postMessage`/`Worker`/
 * `requestIdleCallback` instead of tokio — `spawn_scoped`/`run_blocking`/`sleep_until`/`now_ms`
 * are intentionally left out of this seam since a web host would drive those through its own
 * event loop rather than exposing them as discrete calls.
 *
 * 🇩🇪 Dokumentierte Anschlussstelle für einen zukünftigen Browser/Worker-basierten
 * `HostAsyncRuntime`-Host — NOCH NICHT implementiert. Kein Laufzeitsystem setzt dieses Interface
 * heute irgendwo im Baum um.
 */
export interface WebAsyncScope {
  /** 🌳️ Opens a scope; `parentId` names an already-open scope's id, if any. Returns the new scope's id. */
  openScope(owner: WebAsyncScopeOwner, parentId?: string): string;
  /** 🛑️ Cancels the scope named by `scopeId` (and its descendants), waiting up to `graceMs` before reporting. */
  cancelScope(scopeId: string, graceMs: number): Promise<WebAsyncScopeDrainReport>;
}

/** 🌳️ TS-side mirror of the Rust `ScopeOwner` shape this seam would need — hand-written because
 * `ScopeOwner` itself is intentionally excluded from the generated mirror above (see its Rust doc
 * comment: it is in-process identity, not wire data). Kept local to this seam rather than promoted
 * to the generated mirror until a real web host gives it a use.
 */
export type WebAsyncScopeOwner = { readonly kind: "actor"; readonly actor: number } | { readonly kind: "package"; readonly packageId: string } | { readonly kind: "service"; readonly service: string };

/** 📊️ TS-side mirror of `ScopeDrainReport`'s shape for this seam, same reasoning as {@link WebAsyncScopeOwner}. */
export type WebAsyncScopeDrainReport = { readonly finished: number; readonly cancelled: number; readonly leaked: number };
//#endregion 🌐️WebAsyncScope
