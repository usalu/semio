/** 📖️ Opening preferences: the default app per dialect and role, their config mutations and fold, and `resolveOpeningApp`.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import { dialectCoordinate } from "../../🚪️io/🧬️schema/🟦️.ts";
import { type AppRole, type AppRef } from "@semio-tech/framework/manifest/schema";
import { appRefEquals, dialectEquals } from "../🔀️dialect-equality/🟦️.ts";
import { AppRouter, SURFACE_FAULT_CODES, surfaceFault } from "../🧭️app-router/🟦️.ts";
import { SemioFaultError } from "../🚨️fault/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️OpeningResolver
/** 🎚️ One user-pinned default — mirrors Rust `DefaultApp`
 * (`💻️os/🎚️config/🧬️schema/🦀️component.rs:17`) and its product-scoped TS twin
 * `💻️os/🎚️config/🧬️schema/🟦️.ts`. Duplicated (not imported) — a domain-neutral framework
 * module must not depend on a product's config facet, same boundary this file already draws
 * around `PluginCatalog` below. */
export type DefaultApp = {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly app: AppRef;
};

/** 🎚️ `os.config.opening` materialized state — mirrors Rust `OpeningPreferences`
 * (`💻️os/🎚️config/🧬️schema/🦀️component.rs:26`). */
export type OpeningPreferences = {
  readonly defaults: readonly DefaultApp[];
};

export const EMPTY_OPENING_PREFERENCES: OpeningPreferences = { defaults: [] };

/** 📥️ Narrows a decoded JSON value into a whole {@link OpeningPreferences} snapshot, or
 * `undefined` for anything else. Distinct from {@link decodeOpeningConfigMutation}: this facet's
 * `Mutation::diff` is whole-record (`impl MutationDiff<OpeningPreferences> for OpeningPreferences`,
 * `💻️os/🎚️config/🧬️schema/🦀️component.rs:36` — `apply` ignores `base` entirely), so a synced
 * `MutationEnvelope.diff.payload` for this facet decodes straight to the NEXT full state, not an
 * operation to replay. */
export function decodeOpeningPreferences(value: unknown): OpeningPreferences | undefined {
  if (!value || typeof value !== "object" || !("defaults" in value) || !Array.isArray((value as Record<string, unknown>).defaults)) return undefined;
  const defaults: DefaultApp[] = [];
  for (const raw of (value as Record<string, unknown>).defaults as unknown[]) {
    if (!raw || typeof raw !== "object") return undefined;
    const record = raw as Record<string, unknown>;
    const dialect = record.dialect as Record<string, unknown> | undefined;
    const role = record.role;
    const app = record.app as Record<string, unknown> | undefined;
    if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") return undefined;
    if (role !== "viewer" && role !== "editor") return undefined;
    if (!app || typeof app.pluginId !== "string" || typeof app.appId !== "string") return undefined;
    defaults.push({ dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset }, role, app: { pluginId: app.pluginId, appId: app.appId } });
  }
  return { defaults };
}

/** 🧬️ Mirrors Rust `OpeningConfigMutation`'s two handcrafted kinds
 * (`💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:16`) — `#[serde(tag = "mutation",
 * rename_all = "camelCase")]`, so the wire JSON shape is `{mutation: "setDefaultApp" |
 * "clearDefaultApp", ...}`. */
export type OpeningConfigMutation =
  | { readonly mutation: "setDefaultApp"; readonly dialect: ArtifactDialect; readonly role: AppRole; readonly app: AppRef }
  | { readonly mutation: "clearDefaultApp"; readonly dialect: ArtifactDialect; readonly role: AppRole };

/** 📥️ Narrows a decoded JSON value into an {@link OpeningConfigMutation}, or `undefined` for
 * anything else — never throws, so a caller folding a mixed op log can skip what it doesn't
 * recognize instead of aborting the whole fold. */
export function decodeOpeningConfigMutation(value: unknown): OpeningConfigMutation | undefined {
  if (!value || typeof value !== "object") return undefined;
  const record = value as Record<string, unknown>;
  const dialect = record.dialect as Record<string, unknown> | undefined;
  const role = record.role;
  if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") return undefined;
  if (role !== "viewer" && role !== "editor") return undefined;
  const typedDialect: ArtifactDialect = { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset };
  if (record.mutation === "setDefaultApp") {
    const app = record.app as Record<string, unknown> | undefined;
    if (!app || typeof app.pluginId !== "string" || typeof app.appId !== "string") return undefined;
    return { mutation: "setDefaultApp", dialect: typedDialect, role, app: { pluginId: app.pluginId, appId: app.appId } };
  }
  if (record.mutation === "clearDefaultApp") {
    return { mutation: "clearDefaultApp", dialect: typedDialect, role };
  }
  return undefined;
}

/** 🔺️ Real handcrafted construction from `base`, never apply-then-capture — mirrors Rust
 * `set-default-app`/`clear-default-app`'s `🔺️diff` leaves exactly: `setDefaultApp` drops any
 * existing `(dialect, role)` entry then appends the new pin; `clearDefaultApp` only drops. */
function applyOpeningConfigMutation(base: OpeningPreferences, mutation: OpeningConfigMutation): OpeningPreferences {
  const defaults = base.defaults.filter((entry) => !(dialectEquals(entry.dialect, mutation.dialect) && entry.role === mutation.role));
  if (mutation.mutation === "setDefaultApp") defaults.push({ dialect: mutation.dialect, role: mutation.role, app: mutation.app });
  return { defaults };
}

/** 🧮️ Event-sourced fold over the `os.config.opening` op log — NEVER a mutable map (contract
 * freeze §4: "the resolver reads a fold over the config op log, never a mutable map"). Each step
 * recomputes a fresh `defaults` array; nothing here is ever mutated in place. */
export function foldOpeningPreferences(ops: readonly OpeningConfigMutation[], base: OpeningPreferences = EMPTY_OPENING_PREFERENCES): OpeningPreferences {
  return ops.reduce(applyOpeningConfigMutation, base);
}

/**
 * 🧭️ TS twin of Rust `OpeningResolver::resolve` (contract freeze §3 — same "hadn't landed a
 * concrete struct yet" caveat as {@link AppRouter} above). Four-step precedence, in order:
 * 1. the pinned default from `prefs`, if it is STILL present in `router`;
 * 2. the owner plugin's surface;
 * 3. the first router entry;
 * 4. otherwise throws {@link SemioFaultError} with `"surface.unknown-dialect"`.
 */
export function resolveOpeningApp(router: AppRouter, dialect: ArtifactDialect, role: AppRole, prefs: OpeningPreferences): AppRef {
  const entries = router.entriesFor(dialect, role);
  const pinned = prefs.defaults.find((entry) => dialectEquals(entry.dialect, dialect) && entry.role === role);
  if (pinned && entries.some((ref) => appRefEquals(ref, pinned.app))) return pinned.app;
  const owner = router.ownerPluginId(dialect.artifactKind);
  if (owner !== undefined) {
    const ownerEntry = entries.find((ref) => ref.pluginId === owner);
    if (ownerEntry) return ownerEntry;
  }
  const first = entries[0];
  if (first) return first;
  throw new SemioFaultError(surfaceFault(SURFACE_FAULT_CODES.UnknownDialect, `no surface registered for ${dialectCoordinate(dialect)}#${role}`, {}));
}
//#endregion 🔖️OpeningResolver
// #endregion 🎠️Kernel
