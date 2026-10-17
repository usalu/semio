/** 🔐️ Scope runtime keys, hub session storage, shell props, announced page streams, the hub bootstrap origin, identity mutation envelopes and the shell actor and surface id helpers.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type HubConnectionStorageV1 } from "@semio-tech/framework/directory/sign-in";
// #region 🔌️Adapters
import {
  type AppRole,
  type ArtifactDialect,
  dialectCoordinate,
  createBrowserStoragePort,
  createMemoryStoragePort,
  createScopedStoragePort,
  type PluginRegistryEntry,
  type ShellBrand,
  type StoragePort
} from "@semio-tech/framework";
import { type DocumentScope } from "@semio-tech/framework/document/scope";
import { STREAM_MUX_ANNOUNCEMENT, documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { type MutationEnvelope } from "@semio-tech/framework/replication";
import { IDENTITY_CONFIG_SCHEMA } from "@semio-tech/framework/backbone/config-lane";
import { type Identity, type IdentityConfigMutation, applyIdentityConfigMutation } from "@semio-tech/framework/config/mutations";
import { type UiLocale } from "@semio-tech/ui-react";
import { type AppSurfaceSessionFactory } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { type FrameworkOsDefaults, type ResolvedShellLocks } from "../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { SILENT_STREAM_MUX_ENDPOINT_V1, StreamMuxChannelV1, pageStreamMuxChannelV1, type StreamMuxEndpointV1 } from "@semio-tech/framework/io/stream-mux";
import type { PluginCatalog } from "@semio-tech/framework";
import { type InstalledServicePresentationV1 } from "../🪪️host-bootstrap/🟦️.tsx";
import type { ShellHostPortsV1 } from "../🪝️host-ports/🟦️.ts";

export function scopeRuntimeKey(message: { readonly documentId: string; readonly scope?: DocumentScope }): string | null {
  const scope = message.scope;
  if (scope === undefined || scope.documentId !== message.documentId) return null;
  return documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...scope });
}

/** 🗄️ Where one browsing context remembers the hub session it minted. `sessionStorage`, not
 * `localStorage`: the connection BOOK (which hubs exist, which one is selected) is durable identity
 * and belongs to the profile, but a session capability belongs to the browsing context that signed
 * in — it must survive a reload, and it must not outlive the tab or leak into a second one. A
 * blocked or throwing store yields `null`, which is simply "no session", never a boot failure. */
export function hubSessionStorageV1(): HubConnectionStorageV1 | null {
  try {
    return typeof globalThis.sessionStorage === "undefined" ? null : globalThis.sessionStorage;
  } catch {
    return null;
  }
}
// #endregion 🔌️Adapters

//#region FrameworkOsShell
//#region 🐚️ShellMount
/** 🐚️ Public props for {@link FrameworkOsShell} — the multi-instance-safe entry point. `shellId`,
 * `storageNamespace`, and `ownsPage` exist so several shells can be mounted on one page: `ownsPage`
 * gates the handful of behaviors that are legitimately page-global (document title, browser history
 * sync via `bootFrameworkOs`), `storageNamespace` prefixes this shell's durable storage keys so
 * co-mounted shells don't share `semio.os.dock`/`ui.chrome.*` state. */
export interface FrameworkOsShellProps {
  readonly catalog: PluginCatalog;
  readonly backboneWorkerFactory?: () => Worker;
  readonly documentServices?: readonly InstalledServicePresentationV1[];
  readonly hostPorts?: ShellHostPortsV1;
  readonly surfaceSessionFactories?: readonly AppSurfaceSessionFactory[];
  readonly pluginFilter?: string;
  readonly plugins: readonly PluginRegistryEntry[];
  readonly appId?: string;
  /** 👁️✏️ Boot-time surface role preference (contract freeze §5) — resolved by
   * `resolveBootAppRole`/`VITE_SEMIO_APP_ROLE` at the `bootFrameworkOs` call site, default `"editor"`.
   * Used only to prefer a same-role app when `appId` doesn't pin one explicitly; the actual role of an
   * open session always comes from `session.app.role`, never this prop. */
  readonly appRole?: AppRole;
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly initialLocale?: UiLocale;
  readonly brand?: ShellBrand;
  readonly shellId?: string;
  readonly storageNamespace?: string;
  readonly ownsPage?: boolean;
  /** 🐚️ Skips the brand/app introduction auto-start (and any brand-owned tutorial's own auto-considered
   * reveal) for a shell that's mounted but not the one the user is actually looking at — a live
   * multi-shell page (e.g. the mit-bestand demonstrator's background panes) has no iframe boundary for
   * the existing `window.self !== window.top` heuristic below to key off, so several shells would
   * otherwise all auto-play their onboarding at once the moment they boot. Defaults to `false` (existing
   * single-shell-per-page behavior unchanged). */
  readonly suppressAutoIntroduction?: boolean;
}

/** 📶️ The page's stream channel when the serve that hosts this page announces one (`STREAM_MUX_ANNOUNCEMENT`, defined by
 * the dev serve's backbone plugin), else the silent endpoint: a production bundle or a test harness has no channel to dial, and
 * dialing one anyway made every jsdom law that mounts the shell log a refused WebSocket. */
export function announcedPageStreams(): StreamMuxChannelV1 | StreamMuxEndpointV1 {
  let path: unknown;
  try {
    path = (import.meta as unknown as { readonly env?: Readonly<Record<string, unknown>> }).env?.[STREAM_MUX_ANNOUNCEMENT];
  } catch {
    path = undefined;
  }
  return typeof path === "string" && path.startsWith("/") ? pageStreamMuxChannelV1(path) : SILENT_STREAM_MUX_ENDPOINT_V1;
}

//#region 🔖️Identity
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3 — reads one
 * `VITE_S_*` compile-time define (`💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`'s
 * `define` block). Guarded for non-Vite embeds (SSR/tests/other bundlers) where `import.meta.env` is
 * absent — mirrors `🐚️Shell/🟦️.tsx`'s `readViteAppRoleEnv` idiom (that file is out of this
 * lane's lease, so re-implemented locally rather than imported). Returns `undefined` for an unset/
 * empty define, never `""` — every call site treats "no hub env" as "skip identity entirely" (§C3
 * "No hub env ⇒ skip all of it and keep today's local-only behaviour exactly"). */
export function readViteSEnv(name: "VITE_S_HUB_URL" | "VITE_S_DATA_DIR"): string | undefined {
  try {
    const env = (import.meta as unknown as { readonly env?: Readonly<Record<string, string | undefined>> }).env;
    return env?.[name] || undefined;
  } catch {
    return undefined;
  }
}

/** 🌐️ The origin the hub workspace signs in against. `VITE_S_HUB_URL` is the hub's own origin (the
 * same value `hubEnv` uses for the directory lane); it is normalized to a bare origin because
 * `parseHubOriginV1` refuses a path, credentials, a query or a fragment. Without that define the
 * page origin is the only honest guess — a shell embedded beside its own hub. */
export function hubBootstrapOriginV1(): string {
  const declared = readViteSEnv("VITE_S_HUB_URL");
  if (declared !== undefined) {
    try {
      return new URL(declared).origin;
    } catch {
      /* 🚫️ A malformed define is not a reason to fail the shell; the page origin still renders the
       * surface, and signing in against it reports `unreachable` rather than crashing. */
    }
  }
  return typeof globalThis.location === "undefined" ? "http://127.0.0.1:7777" : globalThis.location.origin;
}

/** 🪪️ One `MutationEnvelope` wrapping an {@link IdentityConfigMutation} — mirrors the exact shape the
 * in-source `foldIdentityEvent` tests build (`🏪️store/👷️worker/🟦️.ts`'s `🔖️DirectoryLaneTests`/identity
 * fold vectors), since this facet's diff is whole-record (never merged) and has no history/undo chrome
 * wired to it this wave (no real inverse chain needed beyond a structurally valid envelope). */
export function identityMutationEnvelope(actor: string, mutation: IdentityConfigMutation, base: Identity | null): MutationEnvelope {
  const nextPayload = applyIdentityConfigMutation(base, mutation);
  return {
    id: `identity-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`,
    actor,
    document: IDENTITY_CONFIG_SCHEMA,
    schemaVersion: IDENTITY_CONFIG_SCHEMA,
    payloadHash: "",
    diff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: nextPayload },
    inverse: { targetOperation: "sign-in", inverseDiff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: base }, baseVersion: 0, undoPolicy: "exactBaseOnly" },
  };
}

/** 🪪️ `decodePayload` for {@link foldIdentityEvent} — a remote/cross-tab envelope's `diff.payload` is
 * either a full `Identity` record or `null` (signed out); anything else (a different facet's envelope
 * riding the same `BroadcastChannel`, contract §C6 has no such case today but this stays defensive)
 * returns `undefined` so the fold leaves `base` untouched. */
export function decodeIdentityPayload(payload: unknown): Identity | null | undefined {
  if (payload === null) return null;
  if (typeof payload !== "object") return undefined;
  const candidate = payload as Partial<Identity>;
  if (typeof candidate.userId === "string" && typeof candidate.email === "string" && typeof candidate.hubBaseUrl === "string") return candidate as Identity;
  return undefined;
}

/** 🪪️ Mints the shell actor id (contract §C0: `user:{userId}#{shellSessionId}`) once identity
 * resolves; the pre-identity default stays `client-{shellSessionId}` (unchanged shape, just the
 * random-suffix source now shared with the post-sign-in id so a reload's actor id is stable relative
 * to its own tab even before/without a hub). */
export function shellActorId(sessionId: string, identity: Identity | null): string {
  return identity ? `user:${identity.userId}#${sessionId}` : `client-${sessionId}`;
}

/** 🪪️ Canonical surface id (contract §C0 `<kind>@<standard>/<subset>#<role>`) for local UI routing.
 * Hub transport surface authority comes only from the complete verified installed execution target. */
export function canonicalSurfaceId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🔌️ Accepts extension-only hot swaps while protecting a plugin-owned active app session. */
export function reloadRetainsActiveApp(apps: readonly { readonly id: string }[], activeAppId?: string): boolean {
  return activeAppId === undefined || apps.some((app) => app.id === activeAppId);
}
//#endregion 🔖️Identity

/** 🐚️ Resolves the {@link ShellScope.storage} port for a shell mount: ephemeral brands always get
 * an in-memory port (never durable, regardless of namespace); a namespaced non-ephemeral shell gets a
 * scoped view over browser storage; a bare non-ephemeral shell (the historical single-app-per-page
 * case) gets the plain shared browser port. */
export function resolveShellScopeStorage(ephemeral: boolean, storageNamespace: string | undefined): StoragePort {
  if (ephemeral) return createMemoryStoragePort();
  const browser = createBrowserStoragePort();
  return storageNamespace ? createScopedStoragePort(browser, storageNamespace) : browser;
}
//#endregion 🐚️ShellMount
//#endregion FrameworkOsShell
