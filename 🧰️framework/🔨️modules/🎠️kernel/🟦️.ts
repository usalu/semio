// #region 🎠️Kernel
/// <reference types="vitest/importMeta" />
/** @emoji 🎠️ `@semio-tech/framework` — plugin runtime, leases, invocation responses, and playground boot. */
import type { IconName } from "@semio-tech/assets";
import type { ShellLocale, ShellTerminology, LocalizedLabel } from "../🛂️manifest/🤖️generated/🟦️ui-axes.ts";

import type {
  PluginManifest,
  BuiltNode,
  PluginViewState,
  ProgramContributionEntry,
  WindowLayout,
  NamedLayout,
} from "../🛂️manifest/🟦️.ts";
import type { StoragePort } from "../🖥️platform/🟦️.ts";
import { ShardClient, type ShardAsset, type ShardBudget, type ShardCapabilityGrant, type ShardEventEnvelope } from "../🎭️actor/🧵️shard-client/🟦️.ts";
import { OwnedResidentLedger } from "../🌱️value/💾️resident/🟦️.ts";
import { TurnScheduler, type Backpressure, type CoalesceKey, type Lane } from "../🎭️actor/📦️packages/🟦️typescript/🟦️.ts";
export { KernelReturnContentFraming, type KernelReturnContentMetadata, type KernelReturnContentByte } from "./📤️return/📦️content/🟦️.ts";
export { KernelReturnUiOperationHeader, type KernelReturnUiOperationFields, type KernelReturnUiFieldName } from "./📤️return/📦️content/🟦️.ts";

//#region EphemeralLane
/** 🫧 Process-local box for module ephemeral values. */
export type EphemeralBox<T> = { current: T };

/** @emoji 🫧️ OS-owned authority for ephemeral local-only state. It deliberately has no storage,
 * serialization, history, sync, or undo surface; a shell/runtime may own an isolated instance while
 * module-level helpers share {@link defaultOsTransient}. */
export class OsTransient {
  private readonly boxes = new Map<string, EphemeralBox<unknown>>();
  private readonly maps = new Map<string, Map<unknown, unknown>>();
  private readonly sets = new Map<string, Set<unknown>>();
  private readonly weakMaps = new Map<string, WeakMap<object, unknown>>();

  box<T>(key: string, init: T): EphemeralBox<T> {
    let box = this.boxes.get(key) as EphemeralBox<T> | undefined;
    if (!box) {
      box = { current: init };
      this.boxes.set(key, box as EphemeralBox<unknown>);
    }
    return box;
  }

  map<K, V>(key: string): Map<K, V> {
    let map = this.maps.get(key) as Map<K, V> | undefined;
    if (!map) {
      map = new Map();
      this.maps.set(key, map as Map<unknown, unknown>);
    }
    return map;
  }

  set<T>(key: string): Set<T> {
    let set = this.sets.get(key) as Set<T> | undefined;
    if (!set) {
      set = new Set();
      this.sets.set(key, set as Set<unknown>);
    }
    return set;
  }

  weakMap<K extends object, V>(key: string): WeakMap<K, V> {
    let map = this.weakMaps.get(key) as WeakMap<K, V> | undefined;
    if (!map) {
      map = new WeakMap();
      this.weakMaps.set(key, map as WeakMap<object, unknown>);
    }
    return map;
  }

  /** 🧹️ Drops every local transient allocation owned by this runtime. Existing references remain
   * valid but are no longer returned by subsequent lookups, matching a shell/session teardown. */
  reset(): void {
    this.boxes.clear();
    this.maps.clear();
    this.sets.clear();
    this.weakMaps.clear();
  }
}

export const defaultOsTransient = new OsTransient();

/** 🫧 Get-or-create a mutable box keyed for OS draft snapshot.
 * Init is stored as-is — never treat a function-typed `T` as a lazy factory (that would
 * invoke identity/no-op resolvers and leave `.current` undefined). */
export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T> {
  return defaultOsTransient.box(key, init);
}

/** 🫧 Get-or-create a process-local Map owned by the ephemeral lane. */
export function ephemeralMap<K, V>(key: string): Map<K, V> {
  return defaultOsTransient.map(key);
}

/** 🫧 Get-or-create a process-local Set owned by the ephemeral lane. */
export function ephemeralSet<T>(key: string): Set<T> {
  return defaultOsTransient.set(key);
}

/** 🫧 Get-or-create a process-local WeakMap owned by the ephemeral lane. */
export function ephemeralWeakMap<K extends object, V>(key: string): WeakMap<K, V> {
  return defaultOsTransient.weakMap(key);
}
//#endregion EphemeralLane

//#region 📇️DescriptorAdmission
/** 📇️ Requires a published descriptor with the requested owner before any actor runtime is started. */
export async function fetchDescriptorManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal): Promise<PluginManifest> {
  signal?.throwIfAborted();
  const path = moduleUrl.split(/[?#]/u)[0]!;
  const descriptorUrl = path.slice(0, path.lastIndexOf("/") + 1) + "🔣️.json";
  const fault = (code: string, detail: string) => new SemioFaultError({
    origin: "os", code, severity: "error", message: `${code}: ${detail}`,
    scope: { pluginId }, retryable: true,
  });
  const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
  signal?.throwIfAborted();
  if (!response.ok) throw fault("plugin.descriptor-unavailable", `${descriptorUrl} (HTTP ${response.status})`);
  if (response.headers?.get?.("content-type")?.toLowerCase().includes("text/html")) throw fault("plugin.descriptor-invalid", `${descriptorUrl} returned HTML`);
  let descriptor: unknown;
  try { descriptor = await response.json(); }
  catch {
    signal?.throwIfAborted();
    throw fault("plugin.descriptor-invalid", `${descriptorUrl} is not JSON`);
  }
  signal?.throwIfAborted();
  const manifest = descriptor && typeof descriptor === "object" && "manifest" in descriptor ? descriptor.manifest : undefined;
  if (!manifest || typeof manifest !== "object" || !("pluginId" in manifest) || typeof manifest.pluginId !== "string") throw fault("plugin.descriptor-invalid", "missing manifest owner");
  if (manifest.pluginId !== pluginId) throw fault("plugin.descriptor-identity-mismatch", `expected ${pluginId}, received ${manifest.pluginId}`);
  if (!("apps" in manifest) || !Array.isArray(manifest.apps)) throw fault("plugin.descriptor-invalid", "missing app roster");
  return manifest as PluginManifest;
}
//#endregion 📇️DescriptorAdmission

//#region 🔖️TurnOutcomeBroadcast
/** 📨️ One instance's reply to whatever {@link PluginWasmHandle.enqueue} most recently queued for
 * it — the async-stream replacement for the old handle's synchronous `(instanceId, frames) ->
 * Promise<frames>` per-call RPC shape (`📌️important.md`'s "Replace, never wrap" list — the removed
 * method's name is deliberately not repeated here, see that list). That old shape assumed a command's
 * reply always lands on the SAME call that sent it; under
 * the turn model a reply may arrive N turns later, so `enqueue` returns nothing and a caller
 * correlates against this stream instead — `AppChannelClient` (`💻️os/🟦️.ts`) is the
 * host-side correlator (FIFO per `instanceId`, matching every real call site's own sequential-await
 * usage today). `frames` mirrors what the old method used to resolve with directly; `error` covers
 * what used to REJECT that promise (a turn submission failure, e.g. a trapped actor — an
 * `AppFrame::Error` frame is still an ordinary `frames` entry, decoded by the caller exactly as
 * before). */
export type TurnOutcome = { readonly instanceId: number; readonly frames: readonly Uint8Array[] } | { readonly instanceId: number; readonly error: unknown };

/** 📡️ Multicast queue backing {@link PluginWasmHandle.outcomes}: every independent
 * `[Symbol.asyncIterator]()` call (one per live `AppChannelClient`) gets its OWN subscription fed
 * every {@link push}ed value, rather than several callers racing to drain one shared FIFO — required
 * because more than one live instance's client iterates the SAME handle-wide stream at once, each
 * filtering to its own `instanceId`. A subscriber unregisters itself the instant its iterator's
 * `return()` is called (what `for await...of`'s `break`/an uncaught throw triggers automatically, and
 * what `AppChannelClient.dispose()` calls explicitly on teardown); {@link complete} force-closes every
 * still-live subscriber at once, for {@link PluginWasmHandle.dispose}. */
export function createTurnOutcomeBroadcast<T>(): { readonly push: (value: T) => void; readonly complete: () => void; readonly stream: AsyncIterable<T> } {
  const subscribers = new Set<{ queue: T[]; resolve: ((result: IteratorResult<T>) => void) | null }>();
  return {
    push: (value) => {
      for (const subscriber of subscribers) {
        if (subscriber.resolve) {
          const resolve = subscriber.resolve;
          subscriber.resolve = null;
          resolve({ value, done: false });
        } else {
          subscriber.queue.push(value);
        }
      }
    },
    complete: () => {
      for (const subscriber of subscribers) subscriber.resolve?.({ value: undefined as unknown as T, done: true });
      subscribers.clear();
    },
    stream: {
      [Symbol.asyncIterator](): AsyncIterator<T> {
        const subscriber: { queue: T[]; resolve: ((result: IteratorResult<T>) => void) | null } = { queue: [], resolve: null };
        subscribers.add(subscriber);
        return {
          next: (): Promise<IteratorResult<T>> => {
            if (subscriber.queue.length > 0) return Promise.resolve({ value: subscriber.queue.shift() as T, done: false });
            return new Promise<IteratorResult<T>>((resolve) => {
              subscriber.resolve = resolve;
            });
          },
          return: (): Promise<IteratorResult<T>> => {
            subscribers.delete(subscriber);
            return Promise.resolve({ value: undefined as unknown as T, done: true });
          },
        };
      },
    },
  };
}
//#endregion 🔖️TurnOutcomeBroadcast

//#region 🧪️TurnOutcomeBroadcastTests
/** 🧪️ Kept right against `createTurnOutcomeBroadcast` — this is the ONE new primitive
 * `PluginWasmHandle.outcomes` and `AppChannelClient` (`💻️os/🟦️.ts`) both depend on, so its
 * multicast/unsubscribe/force-close contract is worth pinning here rather than only indirectly via a
 * `loadPluginModule` integration test (which needs a real `Worker` this suite doesn't have — see
 * `PluginRuntime/🟦️.tsx`'s own header doc on that pre-existing limitation). */
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("createTurnOutcomeBroadcast", () => {
    it("multicasts one pushed value to EVERY live subscriber, not a shared drain-once FIFO", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 1, frames: [] });
      const [stepA, stepB] = await Promise.all([iteratorA.next(), iteratorB.next()]);
      expect(stepA).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
      expect(stepB).toEqual({ value: { instanceId: 1, frames: [] }, done: false });
    });

    it("queues a value pushed before next() is called, and delivers queued values in push order", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      broadcast.push({ instanceId: 2, frames: [] });
      broadcast.push({ instanceId: 3, frames: [] });
      expect(await iterator.next()).toEqual({ value: { instanceId: 2, frames: [] }, done: false });
      expect(await iterator.next()).toEqual({ value: { instanceId: 3, frames: [] }, done: false });
    });

    it("return() unsubscribes immediately — a later push never reaches a next() called after it", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iterator = broadcast.stream[Symbol.asyncIterator]();
      expect(await iterator.return?.()).toEqual({ value: undefined, done: true });
      const pending = iterator.next();
      broadcast.push({ instanceId: 4, frames: [] });
      // 🎯️ an unsubscribed iterator's next() must NOT resolve from this push — racing it against an
      // already-resolved promise proves it is still pending, not that it settled "not yet" by luck.
      const raceResult = await Promise.race([pending.then(() => "resolved" as const), Promise.resolve("not-yet" as const)]);
      expect(raceResult).toBe("not-yet");
    });

    it("complete() force-closes every still-live subscriber at once", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const iteratorA = broadcast.stream[Symbol.asyncIterator]();
      const iteratorB = broadcast.stream[Symbol.asyncIterator]();
      const pendingA = iteratorA.next();
      const pendingB = iteratorB.next();
      broadcast.complete();
      expect(await pendingA).toEqual({ value: undefined, done: true });
      expect(await pendingB).toEqual({ value: undefined, done: true });
    });
  });
}
//#endregion 🧪️TurnOutcomeBroadcastTests

export type PluginWasmHandle = {
  readonly manifest: () => Promise<Uint8Array>;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  /** 🧵 Takes one capped operation-owned export chunk; `undefined` is the exact terminal option. */
  readonly takeSegmentedDownloadChunk: (instanceId: number, operationId: bigint) => Promise<Uint8Array | undefined>;
  /** 📤️ Fire-and-forget: queues `events` (encoded `AppCommand` frames) for `instanceId`'s next turn
   * and returns immediately. The turn's result arrives later on {@link outcomes}, never as this call's
   * return value — replaces the old handle's synchronous per-call method, whose `Promise<Uint8Array[]>`
   * return shape wrongly assumed a reply always lands on the turn it was sent on (R2). */
  readonly enqueue: (instanceId: number, events: readonly Uint8Array[]) => void;
  /** 📥️ Every live instance's turn outcomes, multicast (see {@link createTurnOutcomeBroadcast}) —
   * a caller filters to the `instanceId`(s) it owns. */
  readonly outcomes: AsyncIterable<TurnOutcome>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    for (const topicContribution of entry.manifest.topicContributions ?? []) {
      entries.push({ pluginId: entry.pluginId, topicContribution });
    }
  }
  return JSON.stringify(entries);
}

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}



/**
 * 🧩️ Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended. Host mode (a launch that
 * hosts every plugin at once, e.g. a shell/studio session), or the absence of a primary id, passes
 * the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, hostMode = false): readonly PluginRegistryEntry[] {
  if (hostMode || !primaryPluginId) return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry] as const));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  // 🔗️ Transitive `dependencies` closure of primary + contribution matches — `consumes`/`contributes`
  // alone never pulls Cargo/`dependsOn` plugins (stdio, flow, cad, …), which left every demonstrator
  // pane boot with "needs X which is not installed" and an empty usable load order.
  const selected = new Map<string, PluginRegistryEntry>();
  const queue: PluginRegistryEntry[] = [...primaryEntries, ...contributorEntries];
  for (const entry of queue) selected.set(entry.pluginId, entry);
  for (let index = 0; index < queue.length; index++) {
    const entry = queue[index]!;
    for (const dependency of entry.dependencies ?? []) {
      if (selected.has(dependency.pluginId)) continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry) continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, PluginWasmHandle>;
  readonly contributorInstances: Map<string, number>;
  readonly viewState: PluginViewState;
};

export async function ensureContributorInstance(pluginId: string, appId: string, context: ExternalSlotResolverContext): Promise<number | null> {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null) return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}

export async function resolveExternalSlots(node: BuiltNode, context: ExternalSlotResolverContext): Promise<BuiltNode> {
  if (node.component.type === "extension") {
    const [pluginId = "", appId = pluginId] = node.component.extension.split("/");
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
    }
    // 🚧️ Rendering a contributor's UI body now goes through `AppChannelClient.refreshUi`
    // (`RefreshUi` → `UiSection` over the app-channel handle, os-product `🔖️AppChannelClient` region)
    // instead of the removed per-verb `render`/`renderWithDocument`. Wiring that dispatch loop into this
    // exact call site is the dedicated follow-up work package this ticket flags for the React
    // renderer's dispatch/refresh loops — until then an external slot degrades to unavailable
    // rather than silently guessing at `SectionProbe.kind`/body-key framing.
    return { ...node, component: { type: "text", value: `Extension unavailable: ${pluginId}`, emphasize: null, dataAttributes: null }, children: [] };
  }
  if (node.children.length === 0) return node;
  const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child, context)));
  return children.every((child, index) => child === node.children[index]) ? node : { ...node, children };
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
  /** 🔗️ Direct plugin dependencies this entry's manifest declares — mirrors Rust
   * `PluginManifest.dependencies` (`🛂️manifest/🦀️.rs`), ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS §3. */
  readonly dependencies?: readonly PluginDependency[];
};

/** 🔗️ Widens a {@link PluginCatalogTarget.dependsOn} plugin-id list (no version info, build-time
 * Cargo ground truth) into `PluginRegistryEntry.dependencies` — each id gets the always-satisfied `*`
 * requirement so {@link resolvePluginLoadOrder}/{@link validatePluginDependencyGraph} can still
 * validate presence and detect cycles today, ahead of any plugin adopting the runtime
 * `.depends_on(id, VersionReq)` API. */
function dependsOnToPluginDependencies(dependsOn: readonly string[] | undefined): readonly PluginDependency[] | undefined {
  return dependsOn?.map((pluginId) => ({ pluginId, version: "*" }));
}

//#region 🔖️PluginDependency
/** 🔢️ A frozen `major.minor.patch` version requirement string — one of `*`, `=X.Y.Z`, `^X.Y.Z`,
 * `~X.Y.Z`, `>=X.Y.Z` (contract freeze §3). Mirrors Rust `VersionReq`'s `Display`/`Serialize`
 * wire form exactly; parsing/matching stays server-side (Rust `resolve_load_order` et al.) — this
 * type only lets the browser host read/display/round-trip the requirement string. */
export type VersionReq = string;

/** 🔗️ One direct plugin dependency — mirrors Rust `PluginDependency`
 * (`🛂️manifest/🦀️.rs`). */
export type PluginDependency = {
  readonly pluginId: string;
  readonly version: VersionReq;
};
//#endregion 🔖️PluginDependency

//#region 🔖️ArtifactContribution
/** 🗂️ The `verb`/`entity`/`kind`/`record` semantic identity of one contributed mutation — mirrors
 * Rust `ContributedMutationSemantics`. */
export type ContributedMutationSemantics = {
  readonly verb: string;
  readonly entity: string;
  readonly kind: string;
  readonly record: string;
};

/** 🗂️ One mutation a plugin contributes onto an artifact kind it depends on — mirrors Rust
 * `ContributedMutationMetadata`. `mutationId` follows the contract freeze §3 contributed-id
 * grammar: `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"`. */
export type ContributedMutationMetadata = {
  readonly mutationId: string;
  readonly semantics: ContributedMutationSemantics;
  readonly schemaVersion: number;
  readonly algorithmVersion: number;
};

/** 💡️ One inference a plugin contributes onto an artifact kind it depends on — mirrors Rust
 * `ContributedInferenceMetadata` (the native `ArtifactInferenceServiceMetadata` fields plus
 * `contributor`/`dependsOn`). Registration gate (contract freeze §4): `owner === contributor`,
 * `artifactKind` equals the target artifact kind. */
export type ContributedInferenceMetadata = {
  readonly owner: string;
  readonly artifactKind: string;
  readonly artifactSchema: string;
  readonly artifactSchemaVersion: number;
  readonly documentSchema: string;
  readonly documentSchemaVersion: number;
  readonly inferenceSchema: string;
  readonly inferenceSchemaVersion: number;
  readonly algorithmVersion: number;
  readonly policyVersion: number;
  readonly contributor: string;
  readonly dependsOn?: readonly string[];
};

/** 🗂️ Everything one plugin contributes onto one artifact kind it depends on — mirrors Rust
 * `ArtifactContributionDescriptor`. Accepted only when `artifactKind`'s owning plugin is a direct
 * entry in the contributor's declared `PluginManifest.dependencies` (contract freeze §4). */
export type ArtifactContributionDescriptor = {
  readonly artifactKind: string;
  readonly mutations?: readonly ContributedMutationMetadata[];
  readonly inferences?: readonly ContributedInferenceMetadata[];
};
//#endregion 🔖️ArtifactContribution

//#region 🔖️AppRouter
/** 🎯️ Fully-qualified dialect coordinate — mirrors Rust `ArtifactDialect`
 * (`🔨️modules/🚪️io/🦀️.rs:50`, re-exported off `🛂️manifest/🦀️.rs`). Duplicated
 * locally rather than imported from `🛂️manifest/🟦️.ts`'s generated `AppDefinition` twin:
 * that twin's `apps` field is still `Record<string, unknown>[]` pending the owned schema regeneration for
 * contract freeze §1 C1, so this file reads the wire shape structurally instead of depending on a
 * codegen timing this lease doesn't control — same idiom as the 🔖️PluginDependency/
 * 🔖️ArtifactContribution regions above. */
export type ArtifactDialect = {
  readonly artifactKind: string;
  readonly standard: string;
  readonly subset: string;
};

function dialectEquals(a: ArtifactDialect, b: ArtifactDialect): boolean {
  return a.artifactKind === b.artifactKind && a.standard === b.standard && a.subset === b.subset;
}

/** 🪪️ `<artifact_kind>@<standard>/<subset>` — mirrors Rust `ArtifactDialect::to_coordinate`
 * (`🔨️modules/🚪️io/🦀️.rs:67`). */
export function dialectCoordinate(dialect: ArtifactDialect): string {
  return `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
}

/** 🪪️ Inverse of {@link dialectCoordinate} — mirrors Rust `ArtifactDialect::parse_coordinate`
 * (`🔨️modules/🚪️io/🦀️.rs:74`): `@` splits at its FIRST occurrence, the LAST `/🧰️framework/🔨️modules/🎠️kernel` splits
 * standard from subset. */
export function parseDialectCoordinate(coordinate: string): ArtifactDialect {
  const atIndex = coordinate.indexOf("@");
  if (atIndex < 0) throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} missing '@'`);
  const kind = coordinate.slice(0, atIndex);
  const rest = coordinate.slice(atIndex + 1);
  const slashIndex = rest.lastIndexOf("/");
  if (slashIndex < 0) throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} missing '/'`);
  const standard = rest.slice(0, slashIndex);
  const subset = rest.slice(slashIndex + 1);
  if (kind === "" || standard === "" || subset === "") throw new Error(`dialect coordinate ${JSON.stringify(coordinate)} has an empty component`);
  return { artifactKind: kind, standard, subset };
}

/** 👁️✏️ Mirrors Rust `AppRole` (`🛂️manifest/🦀️.rs:2641`) — exactly `"viewer"`/`"editor"`,
 * contract freeze §1 C1. Wire-identical to the `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE` env values. */
export type AppRole = "viewer" | "editor";

/** 🎯️ Mirrors Rust `AppRef` (`🛂️manifest/🦀️.rs:2672`). */
export type AppRef = {
  readonly pluginId: string;
  readonly appId: string;
};

function appRefEquals(a: AppRef, b: AppRef): boolean {
  return a.pluginId === b.pluginId && a.appId === b.appId;
}

/** 🪪️ `<artifact_kind>@<standard>/<subset>#<role>` — mirrors Rust `surface_app_id`
 * (`🛂️manifest/🦀️.rs:2678`). */
export function surfaceAppId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🪪️ Inverse of {@link surfaceAppId} — mirrors Rust `parse_surface_app_id`
 * (`🛂️manifest/🦀️.rs:2683`): the LAST `#` splits off the role suffix. */
export function parseSurfaceAppId(id: string): { readonly dialect: ArtifactDialect; readonly role: AppRole } {
  const hashIndex = id.lastIndexOf("#");
  if (hashIndex < 0) throw new Error(`surface id ${JSON.stringify(id)} missing '#'`);
  const coordinate = id.slice(0, hashIndex);
  const roleStr = id.slice(hashIndex + 1);
  const dialect = parseDialectCoordinate(coordinate);
  if (roleStr !== "viewer" && roleStr !== "editor") {
    throw new Error(`surface id ${JSON.stringify(id)}: unknown app role ${JSON.stringify(roleStr)}, expected "viewer" or "editor"`);
  }
  return { dialect, role: roleStr };
}

/** 🧯️ The five frozen fault codes contract freeze §2.3 pins for the surface/viewer vocabulary —
 * `origin` is `FaultOrigin::Framework` on every one of them (Rust `dsl::diagnostic::FaultOrigin`,
 * `💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️.rs:149` — landed by lane 1-A). {@link FaultOrigin}
 * below now carries the `"framework"` member too (parity reconciliation, `📓️w1-d-report.md`), so
 * {@link surfaceFault} writes the literal directly instead of the type-assertion this file
 * previously needed while the two sides were out of sync. */
export const SURFACE_FAULT_CODES = {
  ViewerReadOnly: "viewer.read-only",
  UnknownDialect: "surface.unknown-dialect",
  ContributionNotPermitted: "surface.contribution-not-permitted",
  Conflict: "surface.conflict",
  MissingOwnerSurface: "surface.missing-owner-surface",
} as const;

function surfaceFault(code: string, message: string, scope: FaultScope = {}): Fault {
  return { origin: "framework", code, severity: "error", message, scope, retryable: false };
}

/** 🗂️ The minimal per-plugin shape {@link AppRouter} needs — deliberately narrower than (and
 * structurally compatible with) `🛂️manifest/🟦️.ts`'s `PluginManifest`: `apps` stays
 * `Record<string, unknown>[]` there pending the C1 owned schema regeneration, and `artifactKinds` (this
 * plugin's OWNED kinds, Rust `PluginManifest.artifact_kinds`, `🛂️manifest/🦀️.rs:3218`)
 * isn't mirrored on that type at all yet. A caller passes the real `PluginManifest` array straight
 * through once it starts carrying `artifactKinds` — nothing here needs to change. */
export type AppRouterManifest = {
  readonly pluginId: string;
  readonly apps: readonly Record<string, unknown>[];
  /** 🗂️ This plugin's OWNED artifact kinds — the "owner plugin's surface first" ordering rule and
   * the `surface.missing-owner-surface`/`surface.contribution-not-permitted` checks read this,
   * never an app's own produces/consumes `artifactKinds` (ambiguous once a contributor registers
   * a surface on a kind it doesn't own — contract freeze §2.4 `ArtifactContribution`). */
  readonly artifactKinds?: readonly { readonly id: string }[];
  readonly dependencies?: readonly PluginDependency[];
};

function readManifestAppSurface(app: Record<string, unknown>): { readonly appId: string; readonly dialect: ArtifactDialect; readonly role: AppRole } | undefined {
  const id = app.id;
  const role = app.role;
  const dialect = app.dialect as Record<string, unknown> | undefined;
  if (typeof id !== "string") return undefined;
  if (role !== "viewer" && role !== "editor") return undefined;
  if (!dialect || typeof dialect.artifactKind !== "string" || typeof dialect.standard !== "string" || typeof dialect.subset !== "string") return undefined;
  return { appId: id, role, dialect: { artifactKind: dialect.artifactKind, standard: dialect.standard, subset: dialect.subset } };
}

function coordinateRoleKey(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/**
 * 🧭️ TS twin of Rust `AppRouter` (contract freeze §3, C3; reconciled against the real Rust
 * `AppRouter`/`AppRouterState` — `💻️os/🔌️plugin/🖥️host/🦀️.rs:1723-1857` — in
 * `📓️w1-d-report.md`). `(dialect, role) -> AppRef[]`, built from every loaded manifest: the owner
 * plugin's entry first, then the rest sorted `pluginId` then `appId` ascending. A duplicate
 * `AppRef` or an unauthorized cross-plugin contribution fails {@link AppRouter.build} outright —
 * an `AppRouter` therefore never exists in an invalid state.
 */
export class AppRouter {
  private readonly entriesByCoordinateRole: ReadonlyMap<string, readonly AppRef[]>;
  private readonly dialectsByCoordinate: ReadonlyMap<string, ArtifactDialect>;
  private readonly ownerByArtifactKind: ReadonlyMap<string, string>;

  private constructor(entriesByCoordinateRole: ReadonlyMap<string, readonly AppRef[]>, dialectsByCoordinate: ReadonlyMap<string, ArtifactDialect>, ownerByArtifactKind: ReadonlyMap<string, string>) {
    this.entriesByCoordinateRole = entriesByCoordinateRole;
    this.dialectsByCoordinate = dialectsByCoordinate;
    this.ownerByArtifactKind = ownerByArtifactKind;
  }

  /** 🏗️ Builds the router. Loaded manifests are ordered dependency-first before ownership is
   * claimed, matching the Rust host's resolved plugin load order even when browser workers finish
   * nondeterministically. Dependencies not loaded yet are ignored for this transient rebuild; the
   * router is rebuilt again as each plugin arrives. Within that order, first its own
   * `artifactKinds` claim any still-unclaimed kind, then each app claims its dialect's
   * `artifactKind` if still unclaimed. Throws {@link SemioFaultError} with
   * `"surface.contribution-not-permitted"` (checked first) or `"surface.conflict"` (checked
   * second) per app — same order as Rust `register_manifest` (`🦀️.rs:1755-1785`). */
  static build(manifests: readonly AppRouterManifest[]): AppRouter {
    const ownerByArtifactKind = new Map<string, string>();
    const seenRefs = new Set<string>();
    const grouped = new Map<string, { readonly dialect: ArtifactDialect; readonly role: AppRole; readonly entries: AppRef[] }>();
    const loadedIds = new Set(manifests.map((manifest) => manifest.pluginId));
    const dependencyNodes = manifests.map((manifest) => ({
      pluginId: manifest.pluginId,
      dependencies: (manifest.dependencies ?? []).filter((dependency) => loadedIds.has(dependency.pluginId)),
    }));
    const resolved = resolvePluginLoadOrder(dependencyNodes);
    const byId = new Map(manifests.map((manifest) => [manifest.pluginId, manifest] as const));
    const ordered = resolved.errors.length === 0 ? resolved.order.map((pluginId) => byId.get(pluginId)!).filter(Boolean) : [...manifests];
    for (const manifest of ordered) {
      for (const kind of manifest.artifactKinds ?? []) {
        if (!ownerByArtifactKind.has(kind.id)) ownerByArtifactKind.set(kind.id, manifest.pluginId);
      }

      for (const raw of manifest.apps) {
        const surface = readManifestAppSurface(raw);
        if (!surface) continue;

        let owner = ownerByArtifactKind.get(surface.dialect.artifactKind);
        if (owner === undefined) {
          owner = manifest.pluginId;
          ownerByArtifactKind.set(surface.dialect.artifactKind, owner);
        }
        if (owner !== manifest.pluginId) {
          const permitted = (manifest.dependencies ?? []).some((dependency) => dependency.pluginId === owner);
          if (!permitted) {
            throw new SemioFaultError(
              surfaceFault(
                SURFACE_FAULT_CODES.ContributionNotPermitted,
                `plugin ${JSON.stringify(manifest.pluginId)} contributes a surface for ${JSON.stringify(dialectCoordinate(surface.dialect))} without depending on owner ${JSON.stringify(owner)}`,
                { pluginId: manifest.pluginId },
              ),
            );
          }
        }

        const ref: AppRef = { pluginId: manifest.pluginId, appId: surface.appId };
        const refKey = `${ref.pluginId} ${ref.appId}`;
        if (seenRefs.has(refKey)) {
          throw new SemioFaultError(
            surfaceFault(SURFACE_FAULT_CODES.Conflict, `AppRef {pluginId: ${JSON.stringify(ref.pluginId)}, appId: ${JSON.stringify(ref.appId)}} registered twice`, { pluginId: ref.pluginId, appId: ref.appId }),
          );
        }
        seenRefs.add(refKey);

        const key = coordinateRoleKey(surface.dialect, surface.role);
        let group = grouped.get(key);
        if (!group) {
          group = { dialect: surface.dialect, role: surface.role, entries: [] };
          grouped.set(key, group);
        }
        group.entries.push(ref);
      }
    }

    const entriesByCoordinateRole = new Map<string, readonly AppRef[]>();
    const dialectsByCoordinate = new Map<string, ArtifactDialect>();
    for (const [key, group] of grouped) {
      const owner = ownerByArtifactKind.get(group.dialect.artifactKind);
      const sorted = [...group.entries].sort((a, b) => (a.pluginId === b.pluginId ? a.appId.localeCompare(b.appId) : a.pluginId.localeCompare(b.pluginId)));
      const ordered = owner === undefined ? sorted : [...sorted.filter((ref) => ref.pluginId === owner), ...sorted.filter((ref) => ref.pluginId !== owner)];
      entriesByCoordinateRole.set(key, ordered);
      dialectsByCoordinate.set(dialectCoordinate(group.dialect), group.dialect);
    }
    return new AppRouter(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind);
  }

  /** 📋️ Every registered surface for `(dialect, role)`, owner first — empty when none registered. */
  entriesFor(dialect: ArtifactDialect, role: AppRole): readonly AppRef[] {
    return this.entriesByCoordinateRole.get(coordinateRoleKey(dialect, role)) ?? [];
  }

  /** 🪪️ The plugin that owns `artifactKind`, or `undefined` when no loaded manifest claims it. */
  ownerPluginId(artifactKind: string): string | undefined {
    return this.ownerByArtifactKind.get(artifactKind);
  }

  /** 🩺️ "At plugin load, every owned subset must resolve for both roles" (contract freeze §3) —
   * mirrors Rust `AppRouter::owned_surface_gaps` (`🦀️.rs:1836`) exactly: pure, total,
   * never throws. Every dialect with at least one registered surface whose kind is owned but
   * missing a viewer or editor surface contributes one `Fault` (code
   * `"surface.missing-owner-surface"`) to the result — the caller decides whether to log (W1) or
   * hard-fail (W3). Renamed from the prior `assertOwnedSurfacesComplete` (which threw on the
   * FIRST breach instead of collecting all of them, unlike Rust) during the parity
   * reconciliation — no product code called it yet, so the rename carries no migration burden
   * (`📓️w1-d-report.md`). Scoped to dialects that already have at least one registered surface —
   * this class only sees loaded manifests, not the full on-disk taxonomy. */
  ownedSurfaceGaps(): readonly Fault[] {
    const gaps: Fault[] = [];
    for (const [coordinate, dialect] of this.dialectsByCoordinate) {
      const owner = this.ownerByArtifactKind.get(dialect.artifactKind);
      if (owner === undefined) continue;
      for (const role of ["viewer", "editor"] as const) {
        if (this.entriesFor(dialect, role).length === 0) {
          gaps.push(surfaceFault(SURFACE_FAULT_CODES.MissingOwnerSurface, `owned subset ${JSON.stringify(coordinate)} has no ${role} surface`, { pluginId: owner }));
        }
      }
    }
    return gaps;
  }
}

//#region 🧪️AppRouterTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("AppRouter", () => {
    it("orders a loaded aggregate plugin after the foreign surface owner it depends on", () => {
      const cadDialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
      const aggregate: AppRouterManifest = {
        pluginId: "demonstrator",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }],
        dependencies: [{ pluginId: "cad", version: "*" }],
      };
      const owner: AppRouterManifest = {
        pluginId: "cad",
        apps: [{ id: "s.cad.cad@1/*#editor", role: "editor", dialect: cadDialect }],
      };
      const router = AppRouter.build([aggregate, owner]);
      expect(router.ownerPluginId("s.cad.cad")).toBe("cad");
      expect(router.entriesFor(cadDialect, "editor")).toEqual([
        { pluginId: "cad", appId: "s.cad.cad@1/*#editor" },
        { pluginId: "demonstrator", appId: "s.cad.cad@1/*#editor" },
      ]);
    });
  });
}
//#endregion 🧪️AppRouterTests
//#endregion 🔖️AppRouter

//#region 🔖️IoRouter
/** ⚖️ Mirrors Rust `io_schema::IoFidelity` (`🔨️modules/🚪️io/🧬️schema/🦀️component.rs`) — declared
 * strongest io fidelity one hop achieves. No `#[serde(rename_all)]` on the Rust enum, so the wire
 * form is the bare Rust variant name. */
export type IoFidelity = "Exact" | "Canonical" | "Semantic" | "Lossy";

function ioFidelityRank(fidelity: IoFidelity): number {
  switch (fidelity) {
    case "Exact":
      return 3;
    case "Canonical":
      return 2;
    case "Semantic":
      return 1;
    case "Lossy":
      return 0;
  }
}

function ioFidelityFromRank(rank: number): IoFidelity {
  if (rank >= 3) return "Exact";
  if (rank === 2) return "Canonical";
  if (rank === 1) return "Semantic";
  return "Lossy";
}

/** 🎚️ Mirrors Rust `io_schema::Confidence` — how sure an `io-identify`/`io-sniff` is that a payload
 * is a given dialect. Same no-`rename_all` wire form as {@link IoFidelity}. */
export type IoConfidence = "None" | "Low" | "Medium" | "High";

function ioConfidenceRank(confidence: IoConfidence): number {
  switch (confidence) {
    case "None":
      return 0;
    case "Low":
      return 1;
    case "Medium":
      return 2;
    case "High":
      return 3;
  }
}

/** 🌉️ Inverse of {@link ioConfidenceRank} — the WIT `io-sniff` guest export returns a raw `u8` rank
 * byte (`Confidence::rank()`); the caller of {@link ioIdentify} reconstructs the typed value. */
export function ioConfidenceFromRank(rank: number): IoConfidence {
  if (rank >= 3) return "High";
  if (rank === 2) return "Medium";
  if (rank === 1) return "Low";
  return "None";
}

/** 🗄️ Carrier dialects — mirrors Rust `io_schema::CARRIER_BINARY`/`CARRIER_TEXT`: the payload law's
 * two exceptions, whose native encoding IS the raw external file content. */
export const CARRIER_BINARY_DIALECT: ArtifactDialect = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
export const CARRIER_TEXT_DIALECT: ArtifactDialect = { artifactKind: "s.stdio.txt", standard: "utf-8", subset: "*" };

/** 📇️ Mirrors Rust `io_schema::IoEntryDescriptor` — one registered io hop, erased to wire data
 * (`#[serde(rename_all = "camelCase")]` on the Rust side). */
export type IoEntryDescriptor = {
  readonly from: ArtifactDialect;
  readonly into: ArtifactDialect;
  readonly fidelity: IoFidelity;
  readonly sniffs: boolean;
};

/** 🗺️ Mirrors Rust `io_schema::IoRoute` — a resolved hop sequence, `camelCase` wire form. */
export type IoRoute = {
  readonly hops: readonly IoEntryDescriptor[];
  readonly fidelity: IoFidelity;
};

/** 🗂️ One plugin's `list-io-entries` roster, as `IoEntryGraph.build` consumes it. */
export type IoEntryGraphPlugin = {
  readonly pluginId: string;
  readonly entries: readonly IoEntryDescriptor[];
};

function ioEntryKey(from: ArtifactDialect, into: ArtifactDialect): string {
  return `${dialectCoordinate(from)}->${dialectCoordinate(into)}`;
}

/**
 * 🧭️ TS twin of the host `IoRouter`'s NEW io-mechanism graph (`💻️os/🔌️plugin/🖥️host/🦀️.rs`,
 * region `🔖️IoRouter` — the `io_entries`/`resolve_io_route`/`run_io`/`identify` additions,
 * `📓️w1-d-report.md`). `(from, into) -> owning pluginId` merged from every loaded plugin's
 * `list-io-entries` roster, plus deterministic route resolution: highest minimum fidelity, then
 * fewest hops, then lexicographic `into` coordinate order — a pure function of the (from,into) KEY
 * SET, never of plugin registration order (mirrors Rust `resolve_io_route`'s `BTreeMap` +
 * full-candidate-set-sorted-at-the-end shape). Parity with the Rust side is asserted by running the
 * identical fixture through both — see `🧪️w1d-io-router-parity.ts` in this ticket's folder.
 */
export class IoEntryGraph {
  private readonly ownerByEntry: ReadonlyMap<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>;

  private constructor(ownerByEntry: ReadonlyMap<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>) {
    this.ownerByEntry = ownerByEntry;
  }

  /** 🏗️ Merges every `(pluginId, entries)` roster into one graph — mirrors Rust `IoRouter::
   * register_plugin`'s io-entries half. A `(from,into)` key already owned by a DIFFERENT plugin
   * throws (`IoEntryRouteConflict`'s TS twin); re-registering the SAME plugin's own key is
   * idempotent (first registration wins). */
  static build(plugins: readonly IoEntryGraphPlugin[]): IoEntryGraph {
    const ownerByEntry = new Map<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>();
    for (const plugin of plugins) {
      for (const descriptor of plugin.entries) {
        const key = ioEntryKey(descriptor.from, descriptor.into);
        const existing = ownerByEntry.get(key);
        if (existing) {
          if (existing.pluginId !== plugin.pluginId) {
            throw new Error(`io entry route conflict for ${key}: ${JSON.stringify(existing.pluginId)} already owns it; ${JSON.stringify(plugin.pluginId)} cannot replace it`);
          }
          continue;
        }
        ownerByEntry.set(key, { pluginId: plugin.pluginId, descriptor });
      }
    }
    return new IoEntryGraph(ownerByEntry);
  }

  /** 🌉️ SAME deterministic ranking rule as Rust `resolve_io_route`/`io::io_mechanism::
   * resolve_route`: breadth-bounded (`maxHops` clamped to ≤3), cycle-free simple-path enumeration,
   * ranked by (highest minimum fidelity, fewest hops, lexicographic joined `into` coordinate) —
   * the FULL candidate set is sorted at the END (never short-circuited), so the winner never
   * depends on iteration/insertion/registration order. */
  route(from: ArtifactDialect, into: ArtifactDialect, maxHops = 3): IoRoute {
    const bound = Math.min(maxHops, 3);
    if (bound <= 0) throw new Error(`io_routes ${dialectCoordinate(from)} -> ${dialectCoordinate(into)}: max hops clamped to 0`);
    const candidates: IoEntryDescriptor[][] = [];
    const path: IoEntryDescriptor[] = [];
    const visited = new Set<string>([dialectCoordinate(from)]);
    const walk = (current: ArtifactDialect, remainingHops: number): void => {
      if (remainingHops === 0) return;
      for (const { descriptor } of this.ownerByEntry.values()) {
        if (!dialectEquals(descriptor.from, current)) continue;
        const nextCoordinate = dialectCoordinate(descriptor.into);
        if (visited.has(nextCoordinate)) continue;
        path.push(descriptor);
        if (dialectEquals(descriptor.into, into)) {
          candidates.push([...path]);
        } else {
          visited.add(nextCoordinate);
          walk(descriptor.into, remainingHops - 1);
          visited.delete(nextCoordinate);
        }
        path.pop();
      }
    };
    walk(from, bound);
    if (candidates.length === 0) throw new Error(`no io route from ${dialectCoordinate(from)} to ${dialectCoordinate(into)} within ${bound} hops`);
    const rank = (hops: readonly IoEntryDescriptor[]): readonly [number, number, string] => {
      const minFidelity = Math.min(...hops.map((hop) => ioFidelityRank(hop.fidelity)));
      const joined = hops.map((hop) => dialectCoordinate(hop.into)).join(",");
      return [-minFidelity, hops.length, joined];
    };
    const sorted = [...candidates].sort((a, b) => {
      const [aInverseFidelity, aLength, aJoined] = rank(a);
      const [bInverseFidelity, bLength, bJoined] = rank(b);
      if (aInverseFidelity !== bInverseFidelity) return aInverseFidelity - bInverseFidelity;
      if (aLength !== bLength) return aLength - bLength;
      return aJoined.localeCompare(bJoined);
    });
    const best = sorted[0]!;
    const minFidelityRank = Math.min(...best.map((hop) => ioFidelityRank(hop.fidelity)));
    return { hops: best, fidelity: ioFidelityFromRank(minFidelityRank) };
  }

  /** 🪪️ The plugin that owns hop `(from,into)`, or `undefined`. */
  ownerOf(from: ArtifactDialect, into: ArtifactDialect): string | undefined {
    return this.ownerByEntry.get(ioEntryKey(from, into))?.pluginId;
  }

  /** 🔍️ Every registered hop whose `from` is `carrier` and which declares a `sniff` — the fan-out
   * set {@link ioIdentify} sniffs, mirrors Rust `IoRouter::identify`'s carrier filter. */
  carrierEntries(carrier: ArtifactDialect): ReadonlyArray<{ readonly into: ArtifactDialect; readonly pluginId: string }> {
    const found: Array<{ readonly into: ArtifactDialect; readonly pluginId: string }> = [];
    for (const { pluginId, descriptor } of this.ownerByEntry.values()) {
      if (dialectEquals(descriptor.from, carrier) && descriptor.sniffs) found.push({ into: descriptor.into, pluginId });
    }
    return found;
  }
}

/** 🌉️ Runs one hop of a resolved {@link IoRoute} — the caller's bridge into an actual loaded
 * plugin's `io-run` export (this domain-neutral framework module never calls a plugin worker
 * itself, same boundary {@link AppRouter}/{@link ArtifactMutationRouter} already draw). */
export type IoHopRunner = (pluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array) => Promise<Uint8Array> | Uint8Array;

/**
 * 🧭️ TS twin of Rust `IoRouter::run_io` — resolves the WHOLE `from -> into` route over `graph`,
 * then, BEFORE running any hop, refuses the ENTIRE route (no partial execution) if any hop is
 * owned by `callingPluginId` itself: executing that hop would call back into the calling plugin's
 * own in-flight worker call — the same reentrancy hazard the Rust guard exists to prevent. Each
 * hop's output payload feeds the next hop's input via `runHop`.
 */
export async function ioRun(graph: IoEntryGraph, callingPluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array, runHop: IoHopRunner, maxHops = 3): Promise<Uint8Array> {
  const route = graph.route(from, into, maxHops);
  const hops = route.hops.map((hop) => {
    const owner = graph.ownerOf(hop.from, hop.into);
    if (owner === undefined) throw new Error(`io-run: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} vanished from the graph between resolve and execute`);
    if (owner === callingPluginId) {
      throw new Error(
        `io-run refused: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} is owned by the calling plugin ${JSON.stringify(callingPluginId)} itself — executing it would re-enter that plugin's own in-flight worker call`,
      );
    }
    return { hop, owner };
  });
  let current = payload;
  for (const { hop, owner } of hops) {
    current = await runHop(owner, hop.from, hop.into, current);
  }
  return current;
}

/** 🔍️ Sniffs one plugin's `(from,into)` hop — the caller's bridge into an actual loaded plugin's
 * `io-sniff` export, returning the raw `Confidence::rank()` byte. Same DI boundary as {@link IoHopRunner}. */
export type IoSniffRunner = (pluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array) => Promise<number> | number;

/**
 * 🧭️ TS twin of Rust `IoRouter::identify` — fans {@link IoSniffRunner} out across every OTHER
 * plugin's `carrier`-`from` entries (skipping `callingPluginId`'s own, same reentrancy reason
 * {@link ioRun} refuses a self-owned hop — a fan-out is best-effort, so this SKIPS rather than
 * refuses the whole call), merges by confidence descending then coordinate ascending.
 */
export async function ioIdentify(graph: IoEntryGraph, callingPluginId: string, carrier: ArtifactDialect, payload: Uint8Array, sniffHop: IoSniffRunner): Promise<ReadonlyArray<readonly [ArtifactDialect, IoConfidence]>> {
  const candidates = graph.carrierEntries(carrier).filter((entry) => entry.pluginId !== callingPluginId);
  const found: Array<[ArtifactDialect, IoConfidence]> = [];
  for (const { into, pluginId } of candidates) {
    const confidence = ioConfidenceFromRank(await sniffHop(pluginId, carrier, into, payload));
    if (confidence !== "None") found.push([into, confidence]);
  }
  found.sort((a, b) => {
    const rankDiff = ioConfidenceRank(b[1]) - ioConfidenceRank(a[1]);
    if (rankDiff !== 0) return rankDiff;
    return dialectCoordinate(a[0]).localeCompare(dialectCoordinate(b[0]));
  });
  return found;
}
//#endregion 🔖️IoRouter

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

//#region 🗂️PluginCatalog
/** 🗂️ Framework-owned mirror of the OS product's generated `PluginBuildTarget` row — kept
 * shape-compatible so `🛍️products/💻️os/…/🟦️.ts` can build one straight off the generated array
 * without a mapping layer drifting out of sync. */
export type PluginCatalogTarget = {
  readonly pluginId: string;
  readonly wasmOut: string;
  readonly role: "plugin" | "extension";
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  /** 🔗️ Direct plugin dependency ids, Cargo-ground-truth (`semio-s-plugin-<id>` crate deps, `extends`
   * target first for an extension) — mirrors the generated `PluginBuildTarget.dependsOn` 2-C's
   * registry lane added (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS
   * §W2-C report). No `VersionReq` travels with these yet (the registry's build-time view has none to
   * derive it from) — `resolvePlaygroundBoot` maps each id to a `"*"` requirement, which is enough for
   * {@link PluginGraph} to validate presence/cycles and compute load order even before a plugin
   * adopts the runtime `.depends_on(id, VersionReq)` API. */
  readonly dependsOn?: readonly string[];
};

/** 🗂️ Framework-owned mirror of the OS product's generated `PlaygroundBuildTarget` row — only the
 * columns the kernel's playground resolvers actually read. */
export type PlaygroundCatalogTarget = {
  readonly variant: string;
  readonly pluginId: string;
  readonly app?: string;
  readonly aliases: readonly string[];
};

/**
 * 🗂️ Everything the kernel's plugin/playground resolvers need, injected by the caller instead of
 * imported from a specific product's generated build output — inverts the upward dependency a generic
 * framework module must never have on a product's build artifacts. The OS product's
 * `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` is the one place allowed to import the generated
 * registry and build this shape; every other product wanting kernel resolvers builds its own.
 */
export interface PluginCatalog {
  readonly plugins: readonly PluginCatalogTarget[];
  readonly extensions: readonly PluginCatalogTarget[];
  readonly hosts: readonly PluginHostConfig[];
  readonly playgrounds: readonly PlaygroundCatalogTarget[];
  moduleUrl(pluginId: string, wasmOut: string): string;
  extensionModuleUrl(pluginId: string, wasmOut: string): string;
}
//#endregion 🗂️PluginCatalog

//#region InvocationResponse
/** @emoji 🕰️ Hybrid logical clock stamp carried by every kernel operation. */
export type HybridLogicalTimestamp = { readonly wall: number; readonly counter: number };

/** @emoji 🩹️ A schema-tagged artifact mutation payload (forward diff or inverse diff). */
export type ArtifactDiff = { readonly schemaId: string; readonly payload: unknown };

/** @emoji ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

/** @emoji ↩️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseMutation = {
  readonly targetOperation: string;
  readonly inverseDiff: ArtifactDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** @emoji 🔁️ One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelMutation = {
  readonly id: string;
  readonly artifact: number;
  readonly baseVersion: number;
  readonly invocationId: string;
  readonly diff: ArtifactDiff;
  readonly inverse: InverseMutation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🧩️ One member edit folded into a group undo — pairs the owning document handle with the
 * edit id inside it (composite/child-document dispatch). Mirrors Rust `kernel::EditRef`. */
export type EditRef = {
  readonly document: number;
  readonly editId: string;
};

/** @emoji 🎁️ The undo group binding an invocation (action or command) to its operations + inverses. */
export type UndoGroup = {
  readonly invocationId: string;
  readonly mutations: readonly string[];
  readonly inverseMutations: readonly InverseMutation[];
  readonly memberEdits?: readonly EditRef[];
};

/** @emoji 📣️ An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };

/** @emoji 🩺️ Canonical severity for faults and diagnostics — TS twin of Rust `os_dsl::Severity`
 * (`🗣️dsl/⚠️diagnostic/🦀️.rs`, `#[serde(rename_all = "camelCase")]`). Declaration order
 * `Info < Warning < Error < Fatal` (0..3, `as_u8`/`from_u8`) mirrors Rust's `derive(Ord)`; `Hint` was
 * removed repo-wide by ticket `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`
 * §C1 and replaced by `Info` everywhere, including `Fault.severity`/`Diagnostic.severity` here. */
export type Severity = "info" | "warning" | "error" | "fatal";

const SEVERITY_ORDER: readonly Severity[] = ["info", "warning", "error", "fatal"];

/** 🔢️ TS twin of Rust `Severity::as_u8` — stable numeric mirror of declaration order, 0..3. */
export function severityAsU8(severity: Severity): number {
  return SEVERITY_ORDER.indexOf(severity);
}

/** 🔢️ TS twin of Rust `Severity::from_u8`; `undefined` for any value outside 0..3. */
export function severityFromU8(value: number): Severity | undefined {
  return SEVERITY_ORDER[value];
}

/** @emoji 🧭️ Layer that produced a fault. `"framework"` mirrors Rust `FaultOrigin::Framework`
 * (`💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️.rs:149`) — the origin for the five ticket
 * 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET `surface.*`/`viewer.*` fault codes. */
export type FaultOrigin = "edge" | "renderer" | "os" | "module" | "plugin" | "app" | "extension" | "framework";

export type FaultScope = {
  readonly pluginId?: string;
  readonly appId?: string;
  readonly instanceId?: string;
  readonly module?: string;
  readonly bodyKey?: string;
};

export type FaultCause = { readonly message: string; readonly code?: string };

export type TextSpan = { readonly line: number; readonly column: number; readonly length: number };

/** @emoji 🧯️ Structured abort report shared across Rust, WIT, and TypeScript. */
export type Fault = {
  readonly origin: FaultOrigin;
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope: FaultScope;
  readonly span?: TextSpan;
  readonly causes?: readonly FaultCause[];
  readonly retryable: boolean;
};

/** @emoji 🩺️ A diagnostic emitted alongside an action result. */
export type Diagnostic = {
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope?: FaultScope;
  readonly span?: TextSpan;
};

/** @emoji 🧯️ Error subclass carrying a structured {@link Fault}. */
export class SemioFaultError extends Error {
  readonly fault: Fault;
  constructor(fault: Fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}

/**
 * @emoji 🐚️ A typed side effect the guest emits toward the host. Mirrors the Rust `Effect` enum
 * (`🎠️kernel/🦀️.rs` `🔖️Effect` region — replaces `HostEffect` now that plugins and
 * extensions share one `actor` world; externally tagged: unit variants are the plain tag string,
 * struct variants are a single-key object keyed by the camelCase variant name). `openWindow`/
 * `requestFileOpen`/`requestMediaFrames`/`spawnPluginInstance`/`openDialog`/`dispatchAction` gained
 * `req` now that they complete; `invokeExtension` lost `responseAction` and gained `req`.
 */
export type Effect =
  | "requestSync"
  | { readonly openWindow: { readonly req: number; readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** @emoji 📂️ Replaces the active app instance's document with pack+spr bytes — host-owned
   * counterpart of `loadAppArtifactPack` for catalog/example studio opens. */
  | { readonly loadDocument: { readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly req: number; readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly multiple?: boolean } }
  /** @emoji 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
        readonly req: number;
        readonly accept: string;
        readonly frameAction: string;
        readonly doneAction: string;
        readonly fallbackAction: string;
        readonly sampleStride?: number;
        readonly maxFrames?: number;
        readonly maxLongEdgePx?: number;
        readonly fpsHint?: number;
        readonly payload?: string;
        readonly args?: unknown;
      };
    }
  | { readonly spawnPluginInstance: { readonly req: number; readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly req: number; readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** @emoji 🔁️ Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a program
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly req: number; readonly action: string; readonly args?: unknown; readonly delayMs: number } }
  | { readonly clipboardWrite: { readonly fragment: unknown } }
  | { readonly replayShellCommand: { readonly actionId: string; readonly args?: unknown } }
  /** @emoji 🔁️ Asks the shell to invoke an extension capability — the SDK resumes the awaiting
   * future on a `completed` event carrying the same `req` instead of a `responseAction` redispatch. */
  | {
      readonly invokeExtension: {
        readonly req: bigint;
        readonly extensionId: string;
        readonly capability: string;
        readonly requestJson: string;
      };
    }
  // --- new variants (📓️design-abi.md §2's table; nothing constructs these yet) ---
  | { readonly sendMessage: { readonly target: unknown; readonly payload: readonly number[] } }
  | { readonly publishEvent: { readonly topic: string; readonly payload: readonly number[] } }
  | { readonly blobWrite: { readonly req: number; readonly mediaType: unknown; readonly bytes: readonly number[] } }
  | { readonly blobLoad: { readonly req: number; readonly hash: string } }
  | { readonly httpRequest: { readonly req: number; readonly method: string; readonly url: string; readonly headers?: readonly (readonly [string, string])[]; readonly body?: readonly number[]; readonly stream?: boolean } }
  | { readonly documentRead: { readonly req: number; readonly doc: string; readonly lane: string } }
  | { readonly documentWrite: { readonly req: number; readonly doc: string; readonly lane: string; readonly ops: readonly number[] } }
  | { readonly linkResolve: { readonly req: number; readonly link: string } }
  | { readonly registryQuery: { readonly req: number; readonly kind: string; readonly filter?: unknown } }
  | { readonly ioCompose: { readonly req: number; readonly key: string; readonly sources: readonly string[] } }
  | { readonly cacheDerive: { readonly req: number; readonly engineId: string; readonly input: readonly number[] } }
  | { readonly cacheRead: { readonly req: number; readonly engineId: string; readonly key: string } }
  | { readonly setTimer: { readonly id: number; readonly afterMs: number; readonly repeat?: boolean } }
  | { readonly spawnJob: { readonly job: number; readonly kind: string; readonly input: readonly number[]; readonly placement: "inline" | "isolated" | "exclusive" } }
  | { readonly cancelJob: { readonly job: number } }
  | { readonly respond: { readonly req: number; readonly result: unknown } }
  | { readonly storageRead: { readonly req: number; readonly key: string } }
  | { readonly storageWrite: { readonly req: number; readonly key: string; readonly bytes: readonly number[] } }
  | { readonly storageDelete: { readonly req: number; readonly key: string } }
  | { readonly requestCapability: { readonly req: number; readonly capability: unknown } }
  | { readonly releaseCapability: { readonly id: unknown } }
  | { readonly subscribe: { readonly topic: string } }
  | { readonly unsubscribe: { readonly topic: string } };

/**
 * @emoji 🐢️ Mirrors the Rust `UiDirtyScope` — which rendered UI sections an action actually
 * invalidates. Absent (`undefined`) on an `InvocationResponse` means the same as the Rust side's missing
 * field: treat as `{kind: "full"}` (see {@link resolveUiDirtyScope}) — every program that doesn't emit
 * this yet keeps today's whole-shell-refresh behavior.
 */
export type UiDirtyScope =
  | { readonly kind: "full" }
  | { readonly kind: "none" }
  | {
      readonly kind: "partial";
      readonly windowBodies?: readonly string[];
      readonly panelBodies?: readonly string[];
      readonly utilities?: boolean;
      readonly tools?: boolean;
      readonly engagements?: boolean;
      readonly measures?: boolean;
      readonly labels?: boolean;
    };

/** @emoji 🐢️ Normalizes a possibly-absent `UiDirtyScope` — missing (older program, or a response built without one) means `full`. */
export function resolveUiDirtyScope(scope: UiDirtyScope | undefined): UiDirtyScope {
  return scope ?? { kind: "full" };
}

/** @emoji 🧾️ One host-projectable command-history row, mirrored from Rust `HistoryEntry`. */
export type HistoryEntry = {
  readonly seq: number;
  readonly actionId: string;
  readonly label: string;
  readonly kind: string;
  readonly timestamp: string;
  readonly opLines?: readonly string[];
  readonly applied?: boolean;
  readonly revertible?: boolean;
  readonly count?: number;
};

/** @emoji 🧾️ Ordered history delta carried with an accepted invocation response. */
export type HistoryPatch = {
  readonly cursor: number;
  readonly upserts?: readonly HistoryEntry[];
  readonly canUndo?: boolean;
  readonly canRedo?: boolean;
  readonly activeAlternativeId?: string;
  readonly currentCheckpointId?: string;
  readonly commandFilter?: string;
};

/**
 * @emoji 📤️ Typed result of a plugin `handle-action`/`handle-command` call — mirrors the Rust
 * `InvocationResult`. Replaces the legacy `string[]` JSON-patch shape: operations are now typed
 * `KernelMutation`s with true inverses, and the shell applies `requestedEffects` through
 * `applyHostEffects` (WS-E).
 */
export type InvocationResponse = {
  readonly output: unknown;
  readonly mutations: readonly KernelMutation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly Effect[];
  readonly events?: readonly AppEvent[];
  readonly uiScope?: UiDirtyScope;
  readonly historyPatch?: HistoryPatch;
};

// 🐢️ `uiScope` deliberately left unset here (not `{kind: "none"}`) — `resolveUiDirtyScope` treats a
// missing scope as `full`, the safe default for the rare failure paths that return this constant
// (unparseable response, stub module missing `handleAction`/`handleCommand`).
const EMPTY_INVOCATION_RESPONSE: InvocationResponse = {
  output: null,
  mutations: [],
  inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] },
};

/** @emoji 📥️ Parses a raw program `handle-action`/`handle-command` response string into a typed {@link InvocationResponse}. */
export function parseInvocationResponse(raw: string): InvocationResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<InvocationResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.mutations)) {
      return parsed as InvocationResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_INVOCATION_RESPONSE;
}
//#endregion InvocationResponse

//#region 🔖️MergeOutcome
/** @emoji ⚖️ How strict an authority is about accepting a `MutationOutcome` whose messages reach a
 * given {@link Severity} — TS twin of Rust `MergePolicy` (`📡️spr/🧾️wire/🦀️.rs` region
 * `🔖️Policies`). Declaration order IS `as_u8`/`from_u8`'s 0..2 (`LaissezFaire, Normal, Vigilant`).
 * Unlike {@link Severity}, Rust's `MergePolicy` carries no `#[serde(rename_all)]`, so its
 * pack-decoded JSON form (`MergeReport.policy`/`DispatchReport.policy`) is the bare Rust variant
 * name, not camelCased. Local/authority state only — never carried on a `MutationEnvelope`/
 * `BackboneMessage`, never part of an artifact's shared history. */
export type MergePolicy = "LaissezFaire" | "Normal" | "Vigilant";

/** @emoji ⚖️ `#[default]` policy (Rust `MergePolicy::default()`) every fresh instance boots with
 * until a persisted `🛡️change-merge-policy` config triad overrides it or a caller sends
 * `AppChannelClient.setMergePolicy`. */
export const DEFAULT_MERGE_POLICY: MergePolicy = "Normal";

const MERGE_POLICY_ORDER: readonly MergePolicy[] = ["LaissezFaire", "Normal", "Vigilant"];

/** 🔢️ TS twin of Rust `MergePolicy::as_u8` — the ordinal `AppCommand::SetMergePolicy.policy` carries. */
export function mergePolicyAsU8(policy: MergePolicy): number {
  return MERGE_POLICY_ORDER.indexOf(policy);
}

/** 🔢️ TS twin of Rust `MergePolicy::from_u8`; `undefined` for any value outside 0..2. */
export function mergePolicyFromU8(value: number): MergePolicy | undefined {
  return MERGE_POLICY_ORDER[value];
}

/** @emoji ✅️❌️ What a human/authority decided to do with an `Open` {@link Conflict} — TS twin of
 * Rust `ConflictResolution` (`📡️spr/⚔️conflict/🦀️.rs`, `#[serde(rename_all =
 * "camelCase")]` unit enum — single-word variants so its JSON form is just lowercase). */
export type ConflictResolution = "accept" | "discard";

const CONFLICT_RESOLUTION_ORDER: readonly ConflictResolution[] = ["accept", "discard"];

/** 🔢️ The ordinal `AppCommand::ResolveConflict.resolution` carries — declaration order 0..1. */
export function conflictResolutionAsU8(resolution: ConflictResolution): number {
  return CONFLICT_RESOLUTION_ORDER.indexOf(resolution);
}

export function conflictResolutionFromU8(value: number): ConflictResolution | undefined {
  return CONFLICT_RESOLUTION_ORDER[value];
}

/** @emoji 📨️ One outcome-carried diagnostic from a `Mutation`/`MutationKind::diff` — TS twin of
 * Rust `MutationMessage` (`📡️spr/🎮️command/🦀️.rs` region `🔖️Message`,
 * `#[serde(rename_all = "camelCase")]`). `level` reuses {@link Severity}; `code` is one of the
 * frozen seven `mutation.*` codes (contract-freeze §C2 — no per-plugin codes, ever); `message` is
 * English prose (UI localizes by `code`, never by parsing `message`); `target`/`opIndex` are
 * `#[serde(skip_serializing_if)]` on the Rust side, so both are absent (not merely empty) from the
 * pack-decoded JSON when unset. */
export type MutationMessage = {
  readonly level: Severity;
  readonly code: string;
  readonly message: string;
  readonly target?: readonly string[];
  readonly opIndex?: number;
};

/** @emoji 🚫️ Schema mirror of Rust `MutationApplyError` (`📡️spr/🎮️command/🦀️.rs`,
 * `#[serde(rename_all = "camelCase")]`). This is the complete cross-implementation contract for
 * a diff rejected against its supplied base: stable machine `code`, diagnostic `message`, and
 * outermost-first `target`. Rust omits an empty target during serialization, so it is optional
 * on decoded wire values and semantically equivalent to `[]`. */
export type MutationApplyError = {
  readonly code: string;
  readonly message: string;
  readonly target?: readonly string[];
};

/** 🧬️ Runtime JSON Schema for the Rust/TypeScript `MutationApplyError` wire shape. */
export const MUTATION_APPLY_ERROR_SCHEMA = {
  type: "object",
  additionalProperties: false,
  required: ["code", "message"],
  properties: {
    code: { type: "string" },
    message: { type: "string" },
    target: { type: "array", items: { type: "string" } },
  },
} as const;

/** 🔁️ Shared Rust/TypeScript parity vector for the exact serialized apply-error shape. */
export const MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR = {
  json: '{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}',
  value: {
    code: "mutation.apply.invalid-index",
    message: "index 4 exceeds length 2",
    target: ["slides", "4"],
  } satisfies MutationApplyError,
} as const;

/** @emoji 🆔️ Content-addressed conflict identity — TS twin of Rust `ConflictId`
 * (`#[serde(transparent)]`, decodes to a bare string: `conflict-<blake3 hex>`). */
export type ConflictId = string;

/** @emoji 🚧️ What kind of conflict this is — TS twin of Rust `ConflictKind`
 * (`#[serde(tag = "kind", rename_all = "camelCase")]`, internally tagged). `rename_all` on an enum
 * renames only the `kind` discriminant, not a struct variant's own fields, so `edit_ids` stays
 * snake_case exactly as Rust declared it. `envelopes` is `Vec<MutationEnvelope>` serialized through
 * the generic DSL-value bridge (NOT the dedicated causal-envelope wire codec `AppCommand::
 * ApplyEnvelopes`/🧰️framework/🔨️modules/🎠️kernel`AppFrame::DocumentChanged` use) — left as `unknown` here; no consumer needs a
 * typed shape for it yet. */
export type ConflictKind = { readonly kind: "quarantined"; readonly envelopes: readonly unknown[] } | { readonly kind: "degraded"; readonly edit_ids: readonly string[] };

/** @emoji 🚦️ A conflict's own lifecycle, independent of the `MutationMessage`s it carries — TS twin
 * of Rust `ConflictStatus` (`#[serde(rename_all = "camelCase")]`). */
export type ConflictStatus = "open" | "accepted" | "discarded";

/** @emoji ⚔️ One first-class conflict — TS twin of Rust `Conflict` (`📡️spr/⚔️conflict/
 * 🦀️.rs`, `#[serde(rename_all = "camelCase")]`). `timestamp` mirrors
 * `HybridLogicalTimestamp` from `📡️spr/🆔️ids/🦀️.rs` (a DIFFERENT shape than this file's
 * own wall/counter {@link HybridLogicalTimestamp} above — that one is the kernel operation clock,
 * this one the SPR authority clock — so it's inlined rather than reusing the name). */
export type Conflict = {
  readonly id: ConflictId;
  readonly kind: ConflictKind;
  readonly status: ConflictStatus;
  readonly messages: readonly MutationMessage[];
  readonly actors: readonly string[];
  readonly timestamp: { readonly actor: number; readonly physical_ms: number; readonly logical: number };
};

/** @emoji 📨️ One edit's worth of `MutationMessage`s — TS twin of Rust `EditMessages`. */
export type EditMessages = { readonly edit_id: string; readonly messages: readonly MutationMessage[] };

/** @emoji 📤️ The report one LOCAL dispatch produces — TS twin of Rust `DispatchReport`. Packed onto
 * the wire as `AppFrame::Invocation.messages` (successful dispatch) and `AppFrame::Error.report`
 * (rejected dispatch, `Fault.code == "mutation.rejected"`). */
export type DispatchReport = {
  readonly policy: MergePolicy;
  readonly worst: Severity | null;
  readonly messages: readonly MutationMessage[];
};

/** @emoji 🔀️ The report one `ingest_remote`/`merge_remote_snapshot`/`resolve_conflict` merge
 * produces — TS twin of Rust `MergeReport`. Packed onto the wire as `AppFrame::MergeReport.report`,
 * pushed unsolicited after every ingest alongside `DocumentChanged`. */
export type MergeReport = {
  readonly policy: MergePolicy;
  readonly accepted: boolean;
  readonly insertionIndex: number;
  readonly replayed: readonly EditMessages[];
  readonly worst: Severity | null;
  readonly conflict: ConflictId | null;
};
//#endregion 🔖️MergeOutcome

// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `SerializedPluginWasm` (`pluginErrorText`/
// `isPluginInstanceBusyError`/`withSerializedPluginWasmHandle`) and `PluginWorkerClient` — one Worker
// per plugin, capping the browser at ~20 plugins on V8's 4 GiB wasm-module guard region per worker —
// are DELETED (grepped clean: neither had a live caller left outside this file once
// `loadPluginModuleViaWorker`/`loadPluginModuleUncached` went with them; see
// `📓️terra-H2-web-shard-report.md`). Replaced by `ShardClient`
// (`🎭️actor/🧵️shard-client/🟦️.ts` — a bounded pool multiplexed by actorId) and
// `ActivationRegistry` below. `runSerialized`'s busy-retry/reload loop has no equivalent: the new
// ABI's traps are `ActivationRegistry`'s `FailurePolicy` job (design-runtime.md §1) — drop + restore
// from checkpoint — not a local blind-retry loop with no visibility into checkpoint state.

export function relayPluginBackboneOutbound(uri: string, message: Uint8Array): void {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}

/** @emoji 🌉️ A direct-import (main-thread, no-worker) plugin's generated `🟨️.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed program reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, message: Uint8Array) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** @emoji 🌉️ Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `🟨️.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, Uint8Array[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, Uint8Array[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

/**
 * @emoji 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2) GAP — read before relying on this. The
 * per-worker fast path this function used to take (`activeWorkerByPluginId.get(pluginId)` →
 * `PluginWorkerClient.postBackboneInbound`, a raw `"backboneInbound"` postMessage type) is gone along
 * with `PluginWorkerClient` itself, and its counterpart on the guest side is ALSO gone:
 * `🟨️.js` now implements only the `pure` WIT interface (`log`/`now-ms`/`trace-span`) —
 * `backboneSend`/`backbonePoll`/`backboneStatus` were deleted (design-runtime.md §3), because
 * `world actor` has no synchronous host import for them anymore. Every read/write/network/backbone
 * -shaped call now flows through the effect/event turn loop instead (`events::message-event`
 * replaces "the `backbone-poll` push", per `component.wit`'s own doc comment on that variant).
 *
 * This function is kept — `pluginId` stays a real parameter, `ShellHost/🟦️.tsx` (registrar-
 * only, not this packet's to edit) still imports it — but its ONLY remaining path is the main-thread
 * global queue below, which nothing on the guest side drains anymore either post-flip. Wiring
 * `message-event`-addressed delivery through `ActivationRegistry`/`ShardClient` is real, non-mechanical
 * work belonging to whichever packet finishes rewiring `ShellHost` off the pre-flip `PluginWasmHandle`
 * ABI entirely (see this packet's report for the full list of what that touches) — flagged here rather
 * than silently left to look functional.
 */
export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly Uint8Array[]): void {
  void pluginId;
  pushMainThreadPluginBackboneInbound(uri, messages);
}

//#region 🐚️PluginBackboneRouting
/** @emoji 🐚️ Extracts the `<documentId>` a plugin's `actor://<documentId>` backbone uri names — the
 * `framework/sync` `ChannelBackbone::pair` convention (see the react renderer's `openArtifact`). Falls
 * back to the whole uri for any other scheme so an unrecognized realm still gets a routing key instead
 * of being silently dropped. */
function pluginBackboneDocumentIdFromUri(uri: string): string {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}

const pluginBackboneRoutes = new Map<string, (uri: string, message: Uint8Array) => void>();

/**
 * @emoji 🐚️ Routes a plugin's outbound backbone bytes for one document to whichever shell instance owns
 * it — replaces the old page-global relay slot (`setPluginBackboneOutboundRelay`), which a second
 * mounted shell silently overwrote: misrouting the first shell's document sync into the second shell's
 * backbone worker, then severing it entirely the moment that second shell unmounted (it cleared the
 * slot to `null`). Register at the same point a shell learns it owns `documentId` (the react renderer's
 * `openArtifact`) and call the returned unregister function at the matching `closeArtifact`/unmount.
 */
export function registerPluginBackboneRoute(documentId: string, relay: (uri: string, message: Uint8Array) => void): () => void {
  pluginBackboneRoutes.set(documentId, relay);
  return () => {
    if (pluginBackboneRoutes.get(documentId) === relay) pluginBackboneRoutes.delete(documentId);
  };
}
//#endregion 🐚️PluginBackboneRouting

// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `LeasePool`/`createLeasePool` RELOCATE unchanged
// to `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` under `//#region 🪶️LeasePool` — its non-plugin
// consumers (the renderer's engine-session cache and others; see `📓️luna-consumers-audit.md`) keep
// working from there. `PluginModuleLease`/`acquirePluginModule`/`evictPluginModule` and the trailing
// `loadPluginModuleUncached`/`pluginHandleForBridge` are DELETED outright (no relocation — they were
// plugin-specific, per `📌️important.md`'s "must not exist" list), replaced by `ActivationRegistry`.

//#region 🐚️ActivationRegistry
/**
 * @emoji 🐚️ Replaces the deleted `LeasePool`/`PluginModuleLease`/`acquirePluginModule` trio
 * (design-runtime.md §3). Manifest-only records seeded from a `PluginCatalog` (build-time descriptors
 * — no worker/module is touched until an actor actually activates); `events::activation-event` maps
 * onto `activate()`, which calls `ShardClient.activate` (design's `Kernel::activate`, on the web
 * transport); LRU + memory-pressure suspension checkpoints a resident actor and drops it before a new
 * activation would exceed `maxResidentActors`; `resume()` re-activates and restores that checkpoint.
 *
 * `actorId` is a caller-minted string key here, not the real bit-packed `RuntimeActorId` (that
 * encoding lives in the pure `semio-framework-actor` crate — packet A1); this registry only needs a
 * stable key for shard routing and residency bookkeeping, same as `ShardClient` itself.
 */
export type ActivationReason = "on-command" | "on-view-visible" | "on-file-type" | "on-artifact-kind" | "on-extension-request" | "on-startup-finished" | "manual";

export interface ActivationManifestEntry {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly caps: readonly ShardCapabilityGrant[];
}

/** 🔐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the web mirror of
 * `semio_framework_actor::intersect_capabilities` (`🎭️actor/🦀️.rs`) — an extension must
 * never hold a capability its host plugin lacks. Matched by `ShardCapabilityGrant.id` (this file's
 * capability-name field, the web counterpart of the Rust `CapabilityGrant.capability` string); a
 * requested grant survives only when `granted` already carries one with the same `id`. An actual
 * intersection, not "grant what was asked for" — the result is always a subset of `requested`. */
export function intersectCapabilityGrants(granted: readonly ShardCapabilityGrant[], requested: readonly ShardCapabilityGrant[]): readonly ShardCapabilityGrant[] {
  const grantedIds = new Set(granted.map((grant) => grant.id));
  return requested.filter((request) => grantedIds.has(request.id));
}

/** 🪶️ GUESTSLIM (design-runtime.md §3): the typst default font set, fetched ONCE and reused across
 * every actor this registry activates — same fetch-once/reuse contract the deleted
 * `guestSlimAssetsForModule` had (`📇️registry/📜️script.ts`'s `ensureGuestSlimTypstFontsAsset` served
 * layout), just no longer tied to a single worker's lifetime. Delivered as a declared asset attached
 * to the guest's `instance-open` event (see `🟦️.ts`'s `shardWorkerSource`) rather
 * than a worker-bootstrap special case — it must be resident before the first `surface-visible`. */
function defaultGuestSlimAssetFetcher(moduleUrl: string): Promise<readonly ShardAsset[]> {
  const vendorUrl = moduleUrl.split(/[?#]/)[0]!.replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
  return fetch(vendorUrl)
    .then((response) => {
      if (!response.ok) throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
      return response.arrayBuffer();
    })
    .then((buffer): readonly ShardAsset[] => [["guestslim-typst-fonts", buffer]]);
}

//#region 🧮️MemoryPressureCap
/** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): what {@link residentActorCapFromMemory}
 * reads — `navigator.deviceMemory` (Chromium-only, coarse GiB bucket) and/or
 * `performance.memory.jsHeapSizeLimit` (Chromium-only heap ceiling). Safari/Firefox report neither, in
 * which case `ActivationRegistryOptions.maxResidentActors`'s hardcoded fallback still applies. A plain
 * data record, not a live binding — {@link MemoryProbe} is what makes it injectable. */
export interface MemoryProbeReading {
  readonly deviceMemoryGiB?: number;
  readonly jsHeapSizeLimitBytes?: number;
}

/** 🧮️ Injectable seam — CLAUDE.md forbids this class depending on the ambient `navigator`/
 * `performance` globals directly (an external implementation detail); tests inject a fake reading
 * instead of touching a real browser. {@link defaultMemoryProbe} is the only production caller of
 * those globals. */
export type MemoryProbe = () => MemoryProbeReading;

/** 🧮️ The pre-existing hardcoded LRU cap (design-runtime.md §1 `FailurePolicy`) — now the FALLBACK for
 * when neither memory signal is available, not the only source. */
export const DEFAULT_MAX_RESIDENT_ACTORS = 24;

const MIN_MAX_RESIDENT_ACTORS = 4;
const MAX_MAX_RESIDENT_ACTORS = 96;
/** 🧮️ Heuristic: one resident actor's worker-side wasm instance + checkpoint costs roughly this many
 * bytes of device-memory headroom — tuned so a ~4 GiB `deviceMemory` bucket (a typical mid-range
 * laptop) lands near {@link DEFAULT_MAX_RESIDENT_ACTORS}. */
const RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB = 6;
const BYTES_PER_RESIDENT_ACTOR = 64 * 1024 * 1024;

function clampResidentActors(value: number): number {
  return Math.min(MAX_MAX_RESIDENT_ACTORS, Math.max(MIN_MAX_RESIDENT_ACTORS, Math.round(value)));
}

/** 🧮️ `navigator.deviceMemory` first (coarser but a direct GiB figure), else
 * `performance.memory.jsHeapSizeLimit`, else {@link DEFAULT_MAX_RESIDENT_ACTORS} unchanged. Both casts
 * are needed because neither field is in the standard DOM lib — `deviceMemory` is the Chromium-only
 * Device Memory API, `performance.memory` a Chromium-only non-standard extension. */
export function defaultMemoryProbe(): MemoryProbeReading {
  const nav = globalThis.navigator as (Navigator & { readonly deviceMemory?: number }) | undefined;
  const perf = globalThis.performance as (Performance & { readonly memory?: { readonly jsHeapSizeLimit?: number } }) | undefined;
  return { deviceMemoryGiB: nav?.deviceMemory, jsHeapSizeLimitBytes: perf?.memory?.jsHeapSizeLimit };
}

/** 🧮️ Pure — same reasoning as `runtimeMetricsDue` below for being its own exported function rather
 * than inlined into the constructor: testable without touching a real `navigator`/`performance`. */
export function residentActorCapFromMemory(reading: MemoryProbeReading, fallback: number = DEFAULT_MAX_RESIDENT_ACTORS): number {
  if (typeof reading.deviceMemoryGiB === "number" && reading.deviceMemoryGiB > 0) return clampResidentActors(reading.deviceMemoryGiB * RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB);
  if (typeof reading.jsHeapSizeLimitBytes === "number" && reading.jsHeapSizeLimitBytes > 0) return clampResidentActors(reading.jsHeapSizeLimitBytes / BYTES_PER_RESIDENT_ACTOR);
  return fallback;
}
//#endregion 🧮️MemoryPressureCap

//#region 🧵️QueuedTurn
const DEFAULT_TURN_MAILBOX_CAPACITY = 32;

/** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): what {@link ActivationRegistry.enqueueTurn}
 * hands the {@link TurnScheduler}. `generation` is this registry's own mirror of the native
 * `RuntimeActorId`'s bit-packed `generation` field (design-runtime.md §1: "generation makes
 * restart-after-trap addressable without id reuse") — this registry's `actorId` stays the caller-
 * minted string key it always was (see this region's own header doc), so generation lives OUT OF BAND
 * in `ActivationRegistry`'s own `actorGeneration` map instead of inside the id, and is checked at
 * dispatch time in `runQueuedTurn` so a turn queued before a restore can never run against the actor's
 * post-restore instance, even one that slips past the synchronous `cancelQueued` in `restoreActor`. */
interface QueuedTurnPayload {
  readonly events: readonly ShardEventEnvelope[];
  readonly generation: number;
}
//#endregion 🧵️QueuedTurn

interface ActivationResidentEntry {
  readonly actorId: string;
  readonly pluginId: string;
}

export interface ActivationRegistryOptions {
  readonly shardClient: ShardClient;
  readonly defaultBudget: ShardBudget;
  /** LRU cap driving memory-pressure suspension (design-runtime.md §1 `FailurePolicy`) — activating
   * beyond this count checkpoints + suspends the least-recently-touched resident actor first. Explicit
   * override; omit to derive the cap from {@link memoryProbe} instead (see that option's own doc). */
  readonly maxResidentActors?: number;
  /** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): injectable memory signal
   * {@link residentActorCapFromMemory} derives `maxResidentActors` from when that option itself is
   * omitted — defaults to {@link defaultMemoryProbe} (real `navigator.deviceMemory`/
   * `performance.memory`). Tests inject a fake reading; this is what keeps the derivation testable
   * without a real browser (CLAUDE.md: no direct dependency on an external implementation detail). */
  readonly memoryProbe?: MemoryProbe;
  readonly fetchAssets?: (moduleUrl: string) => Promise<readonly ShardAsset[]>;
  /** ⏱️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): clock for `runtimeMetricsSnapshot`'s
   * `sampledAtMs`/`startRuntimeMetricsPublisher`'s cadence gate — injectable so both are testable
   * without real timers, same pattern `ShardClient`'s own `options.now` already uses. */
  readonly now?: () => number;
  /** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): per-actor `TurnScheduler` mailbox
   * capacity — see `📬️mailbox.ts`'s own doc for the accept/coalesced/dropped/rejected contract
   * {@link ActivationRegistry.enqueueTurn} surfaces verbatim. */
  readonly turnMailboxCapacity?: number;
  /** 🧵️ A turn's `ShardClient.turn` result — no effect/`UiPatch` routing exists on this side of the
   * boundary yet (that belongs to the renderer's `ProgramBridge`), so the default is a documented
   * no-op rather than a silent drop of something anyone actually reads. */
  readonly onTurnResult?: (actorId: string, result: unknown) => void;
  /** 🧵️ A rejected queued turn — default logs via `console.error`, same "never let one actor's
   * failure wedge the dispatch loop" contract `TurnScheduler.onTurnError` documents on its own. */
  readonly onTurnError?: (actorId: string, error: unknown) => void;
  /** 📈️ Set `true` to auto-start `startRuntimeMetricsPublisher` in the constructor, wired to this
   * registry's own {@link ActivationRegistry.metricsBus}. Defaults to `false` — opt-in, not opt-out —
   * so every OTHER existing/future construction site across the tree (this file's own tests, the
   * `TaskManager` component's, …) keeps building a plain object with no live `setInterval`, exactly
   * as before this option existed; a real caller (ShellHost, once it mounts the task-manager window)
   * turns this on explicitly. */
  readonly autoStartMetricsPublisher?: boolean;
}

export class ActivationRegistry {
  private readonly manifests = new Map<string, ActivationManifestEntry>();
  private readonly resident = new Map<string, ActivationResidentEntry>();
  private readonly residencyOrder: string[] = [];
  private readonly checkpoints = new Map<string, Uint8Array>();
  private readonly actorPlugin = new Map<string, string>();
  /** 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): see `QueuedTurnPayload`'s own doc —
   * this registry's mirror of the native `RuntimeActorId.generation` field, out of band since this
   * class's `actorId` is a plain caller-minted string. */
  private readonly actorGeneration = new Map<string, number>();
  /** 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the web mirror of the
   * native `ExtensionIndex` — every registered extension's `pluginId`, grouped by its parent plugin
   * id (`PluginCatalogTarget.dependsOn[0]`, guaranteed == `extends` by the same builder assertion the
   * native descriptor pipeline enforces at build time). Stores IDs, not manifest snapshots —
   * `activateExtensionsOf` looks the manifest up FRESH from `manifests` at activation time, the same
   * way `activate()` itself already resolves the parent's own manifest, so a `registerManifest` call
   * that updates an extension's entry after `registerCatalog` (e.g. once a real capability broker
   * starts populating `caps`) is honoured rather than shadowed by a stale copy. Populated by
   * `registerCatalog`; a bare `registerManifest` call (no catalog) leaves this empty, so a manually-
   * seeded manifest never cascades — matching `registerManifest`'s own pre-existing "manifest-only,
   * no side effects" contract exactly. */
  private readonly extensionsByParent = new Map<string, string[]>();
  /** 🧩️ parent actorId → the child actorIds `activateExtensionsOf` minted for it — the cascade
   * topology `suspend`/`resume`/`cancel` walk (leaves-first for suspend/cancel, parent-first for
   * resume — see each method's own doc). */
  private readonly extensionChildren = new Map<string, string[]>();
  private readonly shardClient: ShardClient;
  private readonly defaultBudget: ShardBudget;
  private readonly maxResidentActors: number;
  private readonly fetchAssets: (moduleUrl: string) => Promise<readonly ShardAsset[]>;
  private assetsPromise: Promise<readonly ShardAsset[]> | null = null;
  private readonly now: () => number;
  private lastRuntimeMetricsPublishMs: number | null = null;
  private readonly turnScheduler: TurnScheduler<QueuedTurnPayload, ShardBudget>;
  private readonly onTurnResult: (actorId: string, result: unknown) => void;
  private readonly stopMetricsPublisher: () => void;
  /** 📡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): the platform's own pub-sub
   * primitive, not a bespoke one — no topic-subscriber bus exists anywhere in this codebase yet
   * (native or web; see `TaskManager/🟦️.tsx`'s own header doc on why a real window mount is
   * still registrar-only work), so `startRuntimeMetricsPublisher`'s sink dispatches a
   * `CustomEvent(topic, { detail: snapshot })` here rather than inventing a second bus. Populated
   * (via `startRuntimeMetricsPublisher`) only when `autoStartMetricsPublisher: true` is passed; a real
   * consumer (once `ShellHost` mounts the task-manager window) subscribes with a plain
   * `registry.metricsBus.addEventListener("os.runtime.metrics", ...)`. */
  readonly metricsBus: EventTarget = new EventTarget();

  constructor(options: ActivationRegistryOptions) {
    this.shardClient = options.shardClient;
    this.defaultBudget = options.defaultBudget;
    this.maxResidentActors = options.maxResidentActors ?? residentActorCapFromMemory((options.memoryProbe ?? defaultMemoryProbe)());
    this.fetchAssets = options.fetchAssets ?? defaultGuestSlimAssetFetcher;
    this.now = options.now ?? (() => Date.now());
    this.onTurnResult = options.onTurnResult ?? (() => {});
    const onTurnError = options.onTurnError ?? ((actorId: string, error: unknown) => console.error(`[DEBUG] ActivationRegistry: turn failed for ${actorId}`, error));
    this.turnScheduler = new TurnScheduler<QueuedTurnPayload, ShardBudget>({
      mailboxCapacity: options.turnMailboxCapacity ?? DEFAULT_TURN_MAILBOX_CAPACITY,
      budgetFor: () => this.defaultBudget,
      runTurn: (actorId, payload, budget) => this.runQueuedTurn(actorId, payload, budget),
      onTurnError,
    });
    this.stopMetricsPublisher =
      options.autoStartMetricsPublisher === true
        ? this.startRuntimeMetricsPublisher((topic, snapshot) => this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot })))
        : () => {};
  }

  //#region 📖️Manifest
  registerManifest(entry: ActivationManifestEntry): void {
    this.manifests.set(entry.pluginId, entry);
  }

  /** 📖️ Seeds every plugin + extension row from a `PluginCatalog` (build-time descriptors) — no
   * worker/module is touched until `activate()` for one of these ids actually runs. Also indexes
   * every extension by its parent (`extensionsByParent`) so `activate()`'s cascade has something to
   * walk — descriptor-driven, zero special-casing per target. */
  registerCatalog(catalog: PluginCatalog): void {
    for (const target of catalog.plugins) this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.moduleUrl(target.pluginId, target.wasmOut), caps: [] });
    for (const target of catalog.extensions) {
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut), caps: [] });
      const parentId = target.dependsOn?.[0];
      if (!parentId) continue;
      const siblings = this.extensionsByParent.get(parentId) ?? [];
      siblings.push(target.pluginId);
      this.extensionsByParent.set(parentId, siblings);
    }
  }

  manifestFor(pluginId: string): ActivationManifestEntry | undefined {
    return this.manifests.get(pluginId);
  }
  //#endregion 📖️Manifest

  private loadAssets(moduleUrl: string): Promise<readonly ShardAsset[]> {
    this.assetsPromise ??= this.fetchAssets(moduleUrl).catch((error: unknown) => {
      console.warn("[DEBUG] ActivationRegistry: guestSlim asset fetch failed; affected actors render without it", error);
      this.assetsPromise = null;
      return [];
    });
    return this.assetsPromise;
  }

  private markResident(actorId: string, pluginId: string): void {
    this.resident.set(actorId, { actorId, pluginId });
    this.actorPlugin.set(actorId, pluginId);
    this.touch(actorId);
  }

  /** ⏱️ Refreshes `actorId`'s LRU position — call on every turn, not just activation, or a
   * long-resident-but-idle actor never yields to memory pressure ahead of one that's actually busy. */
  touch(actorId: string): void {
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
    this.residencyOrder.push(actorId);
  }

  //#region ▶️Activate
  /** ▶️ `events::activation-event` → `Kernel::activate`. Cascades to every registered extension of
   * `pluginId` — see `activateExtensionsOf`. */
  async activate(pluginId: string, actorId: string, _reason: ActivationReason): Promise<void> {
    const manifest = this.manifests.get(pluginId);
    if (!manifest) throw new Error(`[DEBUG] ActivationRegistry.activate: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    this.markResident(actorId, pluginId);
    await this.activateExtensionsOf(pluginId, actorId);
  }

  /** 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the cascade half of
   * `activate` — `design-unified.md` §M6's web mirror, "per-extension `shardClient.activate` with
   * parent affinity, local link records, symmetric cascade." Every registered extension whose
   * descriptor names `pluginId` as its parent (`registerCatalog`'s own `extensionsByParent` index)
   * activates alongside it under a deterministic child actorId, `caps` scoped to
   * {@link intersectCapabilityGrants} of the parent's own granted set against the extension's
   * request. Best-effort per extension (one broken extension must not fail the parent's own
   * `activate()` call) — logs and continues, matching this class's existing `evictForMemoryPressure`/
   * `loadAssets` failure posture elsewhere.
   *
   * 🕳️ Honest gap, not worked around: `ShardClient.activate` has no pinned-shard/worker-affinity
   * parameter (its own `assignShard` is a private least-loaded placement, mirroring the native
   * `ShardTable::pin`) — every extension lands on whichever shard `ShardClient` picks, not
   * necessarily the parent's own. A lease-request for a small additive `ShardClient.activate`
   * overload is open against `🎭️actor/🧵️shard-client/🟦️.ts` (out of this
   * packet's `path_scope`); see this ticket's report. The cascade LINK (`extensionChildren`), and
   * therefore zero-orphan teardown via `suspend`/`cancel`, is unaffected by this gap. */
  private async activateExtensionsOf(pluginId: string, parentActorId: string): Promise<void> {
    const extensionIds = this.extensionsByParent.get(pluginId);
    if (!extensionIds || extensionIds.length === 0) return;
    const parentCaps = this.manifests.get(pluginId)?.caps ?? [];
    const children: string[] = [];
    for (const extensionId of extensionIds) {
      const manifest = this.manifests.get(extensionId);
      if (!manifest) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} has no registered manifest, skipping`);
        continue;
      }
      const childActorId = `${parentActorId}::${extensionId}`;
      try {
        const scopedCaps = intersectCapabilityGrants(parentCaps, manifest.caps);
        const assets = await this.loadAssets(manifest.moduleUrl);
        await this.shardClient.activate(childActorId, manifest.moduleUrl, scopedCaps, this.defaultBudget, assets);
        this.markResident(childActorId, extensionId);
        children.push(childActorId);
      } catch (error) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} failed to activate`, error);
      }
    }
    if (children.length > 0) this.extensionChildren.set(parentActorId, children);
  }
  //#endregion ▶️Activate

  //#region 🧵️TurnDispatch
  /** 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): routes one turn through the
   * `TurnScheduler` instead of a caller reaching `ShardClient.turn` directly — lane priority,
   * latest-wins coalescing, and cancellation-on-suspend/teardown/restore all come from the scheduler,
   * not from this method. Returns the same {@link Backpressure} the scheduler itself returns; `rejected`
   * must surface to the UI as busy, same contract `TurnScheduler.enqueue` documents on its own. */
  enqueueTurn(actorId: string, lane: Lane, events: readonly ShardEventEnvelope[], options?: { readonly coalesce?: CoalesceKey }): Backpressure {
    const generation = this.actorGeneration.get(actorId) ?? 0;
    return this.turnScheduler.enqueue(actorId, { lane, coalesce: options?.coalesce, payload: { events, generation } });
  }

  /** 🧵️ The `TurnScheduler`'s `runTurn` seam. Drops (rather than dispatches) a turn whose snapshotted
   * generation no longer matches this actor's current one — it was queued against an instance that has
   * since been restored (see `QueuedTurnPayload`'s own doc and `restoreActor`), so running it now would
   * be exactly the "receives pre-restart queued work" bug this packet exists to prevent. */
  private async runQueuedTurn(actorId: string, payload: QueuedTurnPayload, budget: ShardBudget): Promise<void> {
    const currentGeneration = this.actorGeneration.get(actorId) ?? 0;
    if (payload.generation !== currentGeneration) {
      console.warn(`[DEBUG] ActivationRegistry: dropping turn for ${actorId} queued against generation ${payload.generation}, now at ${currentGeneration} (restored in between)`);
      return;
    }
    this.touch(actorId);
    const result = await this.shardClient.turn(actorId, payload.events, budget);
    this.onTurnResult(actorId, result);
  }
  //#endregion 🧵️TurnDispatch

  //#region 🚑️SuspendResume
  private async evictForMemoryPressure(): Promise<void> {
    while (this.residencyOrder.length >= this.maxResidentActors) {
      await this.suspend(this.residencyOrder[0]!);
    }
  }

  /** 🚑️ Checkpoints and drops `actorId`'s worker-side residency — LRU eviction and an explicit call
   * both go through here. A no-op for an already-suspended (or never-activated) actorId.
   *
   * 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): cancels every turn still queued (not
   * yet dispatched) for `actorId` FIRST, synchronously, before the `checkpoint`/`dispose` round trip
   * even starts — a suspended actor's worker-side instance is about to go away, so anything still
   * queued must be cancelled rather than risk `TurnScheduler`'s pump dispatching it against a
   * dead/disposed instance mid-suspend. */
  async suspend(actorId: string): Promise<void> {
    if (!this.resident.has(actorId)) return;
    // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): `cancelQueued` MUST
    // stay the first synchronous action after the residency check, before any `await` — this
    // method's own pre-existing doc comment ("cancels every turn still queued... FIRST,
    // synchronously, before the checkpoint/dispose round trip even starts") and a real regression
    // this ordering fixes: an `await` inserted ahead of `cancelQueued` (even one that resolves
    // immediately, e.g. calling an async fn with nothing to do) yields to the microtask queue at
    // least once, which is enough for `TurnScheduler`'s own microtask-scheduled pump to dispatch an
    // already-enqueued turn before `cancelQueued` ever runs — caught by
    // `ActivationRegistry.suspend cancels queued turns`'s existing test.
    this.turnScheduler.cancelQueued(actorId);
    // 🧩️ terra-extension-activation: leaves-first — every cascade-activated extension is suspended
    // (checkpointed) before its parent's own checkpoint/dispose below, so an extension never outlives
    // its parent's worker-side teardown. `extensionChildren`'s entry is left in place (not deleted)
    // — `resume` needs it to restore the same children, parent-first, on the way back up.
    await this.suspendExtensionsOf(actorId);
    const checkpoint = await this.shardClient.checkpoint(actorId);
    this.checkpoints.set(actorId, checkpoint);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
  }

  /** 🧩️ terra-extension-activation: cascade half of `suspend` — every child `activateExtensionsOf`
   * minted for `parentActorId`, suspended in turn (recursing naturally through each child's own
   * `suspend`, so a deeper cascade would still work correctly if one ever existed — today an
   * extension's `dependsOn[0]` always names a plugin, never another extension, so this is exactly
   * one level). A no-op for a parent with no tracked extensions. */
  private async suspendExtensionsOf(parentActorId: string): Promise<void> {
    const children = this.extensionChildren.get(parentActorId);
    if (!children) return;
    for (const child of children) await this.suspend(child);
  }

  /** 🚑️ Re-activates a suspended actorId and restores its last checkpoint — a plain cold `activate()`
   * (no `restore()` call) if it was never checkpointed. */
  async resume(actorId: string): Promise<void> {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId) throw new Error(`[DEBUG] ActivationRegistry.resume: unknown actor ${actorId} (never activated)`);
    const manifest = this.manifests.get(pluginId);
    if (!manifest) throw new Error(`[DEBUG] ActivationRegistry.resume: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    const checkpoint = this.checkpoints.get(actorId);
    if (checkpoint) await this.shardClient.restore(actorId, checkpoint);
    this.markResident(actorId, pluginId);
    // 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): parent-first restore
    // — the symmetric direction to `suspend`'s leaves-first teardown (design doc M6: "symmetric
    // cascade, restore: parent first"). A child's own worker-side instance is only useful once its
    // parent is running again.
    await this.resumeExtensionsOf(actorId);
  }

  /** 🧩️ terra-extension-activation: cascade half of `resume` — every tracked child that was
   * suspended (has a checkpoint, not currently resident) resumes after its parent. A child never
   * suspended at all (e.g. it failed to activate in the first place, per `activateExtensionsOf`'s
   * best-effort policy) has no checkpoint and is correctly skipped rather than cold-activated here —
   * `activate()` is the only entry point that MINTS a fresh extension cascade. */
  private async resumeExtensionsOf(parentActorId: string): Promise<void> {
    const children = this.extensionChildren.get(parentActorId);
    if (!children) return;
    for (const child of children) {
      if (this.checkpoints.has(child) && !this.resident.has(child)) await this.resume(child);
    }
  }

  /** 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): the web mirror of native's
   * "Trapped → drop + re-instantiate (generation++) + restore last checkpoint" (design-runtime.md §1
   * `FailurePolicy`) — called from a `ShardClient`'s own `onShardLost` for every actorId that WAS
   * pinned to the shard that just died (see {@link handleShardLost}). Bumps this actor's generation
   * BEFORE cancelling its queue: the bump is what protects a turn that gets enqueued during the
   * `resume()` await below (since `enqueueTurn` always reads the CURRENT generation at call time), and
   * the immediately-following `cancelQueued` is belt-and-suspenders for anything already queued at the
   * moment loss is detected. A no-op for an actorId this registry never activated or has already fully
   * `cancel()`ed — the shard's own bookkeeping and this registry's can disagree briefly across a
   * teardown race, and only actors this registry still recognizes are ours to restore. */
  private async restoreActor(actorId: string): Promise<void> {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId) return;
    this.actorGeneration.set(actorId, (this.actorGeneration.get(actorId) ?? 0) + 1);
    this.turnScheduler.cancelQueued(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
    await this.resume(actorId);
  }

  /** 🚑️ Restores every actorId the caller reports as lost together — see {@link restoreActor}. Runs
   * every restoration concurrently (independent actors, no ordering dependency between them) and never
   * lets one actor's restore failure block another's, same "one actor's failure never wedges the rest"
   * reasoning `TurnScheduler.onTurnError`'s own doc gives. */
  async restoreActors(actorIds: readonly string[]): Promise<void> {
    await Promise.all(actorIds.map((actorId) => this.restoreActor(actorId).catch((error: unknown) => console.error(`[DEBUG] ActivationRegistry.restoreActors: failed to restore ${actorId}`, error))));
  }

  /** 🚑️ Bound convenience handler for `ShardClientOptions.onShardLost` — pass this directly, e.g.
   * `new ShardClient({ …, onShardLost: registry.handleShardLost })`. `ShardClient`'s own callback
   * contract is synchronous `void` (the shard transport only reports loss; restoration is the
   * kernel-side registry's job, per that option's own doc comment), so the restore promise is
   * deliberately fire-and-forget here — failures are still visible via `restoreActors`' own
   * `console.error`, nothing new swallows them. */
  readonly handleShardLost = (_shardIndex: number, actorIds: readonly string[]): void => {
    void this.restoreActors(actorIds);
  };

  /** 🛑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1, wiring the task manager's "cancel" action to a
   * REAL dispatch path): the web mirror of `Payload::Cancel`'s now-landed native semantics
   * (`🧵️shard/🦀️.rs`'s `ShardLoop::pump`, K1 — "cancels the actor's running jobs +
   * unregisters the instance"). Unlike `suspend()`, this is NOT resumable: no checkpoint is taken,
   * and every bookkeeping entry (including `actorPlugin`) is dropped, so a later `resume(actorId)`
   * correctly throws "unknown actor" rather than silently reviving it. A no-op for an actorId this
   * registry has never heard of.
   *
   * 🚧️ Honest gap, NOT fixed here (would need a file outside this packet's `path_scope`): this
   * class has no per-actor job-id bookkeeping (`ShardClient.cancelJob` needs a specific job id,
   * tracked by whoever calls `startJob`/`stepJob` — not `ActivationRegistry`/`ShardClient`), so
   * "cancels the actor's running jobs" is only reachable here via `dispose()` tearing down the whole
   * worker-side instance (which implicitly ends any in-flight job), not via an explicit per-job
   * cancel message. See `📓️terra-T1-report.md` `## honest gaps`. */
  cancel(actorId: string): void {
    if (!this.actorPlugin.has(actorId)) return;
    // 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): leaves-first, and —
    // unlike `suspend` — PERMANENT: every cascade-activated extension is cancelled before the parent
    // itself, and the cascade edge is dropped (no checkpoint survives a `cancel`, so there is nothing
    // for a later `resume` to restore). "A parent kill takes its extensions down" (design doc M6's
    // own acceptance wording) is exactly this recursion.
    const children = this.extensionChildren.get(actorId);
    if (children) {
      for (const child of children) this.cancel(child);
      this.extensionChildren.delete(actorId);
    }
    this.turnScheduler.teardownActor(actorId);
    this.actorGeneration.delete(actorId);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    this.checkpoints.delete(actorId);
    this.actorPlugin.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1) this.residencyOrder.splice(index, 1);
  }

  isResident(actorId: string): boolean {
    return this.resident.has(actorId);
  }

  /** ⏏️ Stops the constructor's own auto-started metrics-publish loop (a no-op if
   * `autoStartMetricsPublisher: false` was passed) — call once on full teardown, mirroring
   * `ShardClient.disposeAll`'s own real-`setInterval` cleanup. Does not touch `shardClient` itself
   * (this registry doesn't own its lifecycle — it was handed one already built). */
  dispose(): void {
    this.stopMetricsPublisher();
  }
  //#endregion 🚑️SuspendResume

  //#region 📈️RuntimeMetrics
  /** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one row per actor this registry has ever
   * activated (`actorPlugin` — populated on `markResident`, never cleared on `suspend`, so a
   * suspended-but-not-forgotten actor still gets a row with `resident: false`). Field-compatible
   * with the Rust `ActorMetricsSample` the native host publishes, minus the fields only a live
   * `Kernel`/guest turn can produce (`turns`/`traps`/`wallUsP95`/…) — this registry never held a
   * `Kernel` (it delegates straight to `ShardClient`, see this file's own header doc), so those are
   * an honest gap here, not a silent zero-fill. */
  runtimeMetricsActorRows(): readonly RuntimeMetricsActorRow[] {
    return [...this.actorPlugin.entries()].map(([actorId, pluginId]) => ({ actorId, pluginId, resident: this.resident.has(actorId), shard: this.shardClient.shardIndexFor(actorId) ?? null }));
  }

  /** 📈️ The `os.runtime.metrics` payload this side of the boundary can build: per-actor residency
   * rows plus `ShardClient.shardMetricsSamples` (see that method's own doc comment). `sampledAtMs`
   * defaults to this registry's own injected clock, same convention as `runtime_metrics_snapshot`'s
   * `sampled_at_ms` on the Rust side. */
  runtimeMetricsSnapshot(sampledAtMs: number = this.now()): RuntimeMetricsSnapshot {
    return { actors: this.runtimeMetricsActorRows(), shards: this.shardClient.shardMetricsSamples(sampledAtMs), sampledAtMs };
  }

  /** ⏱️ Starts a 2Hz (`RUNTIME_METRICS_PUBLISH_INTERVAL_MS`) publish loop calling `sink(topic,
   * snapshot)` — `topic` is always `"os.runtime.metrics"`, matching the Rust side's bus topic name.
   * Returns a `stop()` disposer. Real `setInterval`, not the injected `now` (browser timer loops are
   * not something the pure-crate clock-injection discipline applies to — only `🎭️actor` itself must
   * never read a clock); `runtimeMetricsDue` below is what stays unit-testable without real timers.
   *
   * ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): this now HAS a real caller and sink —
   * pass `autoStartMetricsPublisher: true` and the constructor starts this itself, with a sink that
   * dispatches on `this.metricsBus`; see that field's own doc. Calling this method directly (with a
   * different sink) is still supported for a caller that wants its own delivery instead of the bus.
   *
   * 🚧️ Honest remaining gap: no real CONSUMER subscribes to `metricsBus` yet anywhere in this codebase
   * (native or web) — mounting the task-manager window that would (`TaskManager/🟦️.tsx`'s own
   * header doc) is registrar-only, lease-requested work outside this packet's `path_scope`. */
  startRuntimeMetricsPublisher(sink: (topic: string, snapshot: RuntimeMetricsSnapshot) => void): () => void {
    const interval = setInterval(() => {
      const nowMs = this.now();
      if (!runtimeMetricsDue(this.lastRuntimeMetricsPublishMs, nowMs)) return;
      this.lastRuntimeMetricsPublishMs = nowMs;
      sink("os.runtime.metrics", this.runtimeMetricsSnapshot(nowMs));
    }, RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
    return () => clearInterval(interval);
  }
  //#endregion 📈️RuntimeMetrics
}

/** 📈️ Mirrors `semio_framework_actor::ActorMetricsSample` where this registry has the data for it —
 * see `ActivationRegistry.runtimeMetricsActorRows`'s own doc comment for exactly which fields are an
 * honest gap here (no live `Kernel` on this side of the boundary). */
export interface RuntimeMetricsActorRow {
  readonly actorId: string;
  readonly pluginId: string;
  readonly resident: boolean;
  readonly shard: number | null;
}

/** 📈️ Mirrors `semio_framework_actor::RuntimeMetricsSnapshot`'s shape — `kernel` (the aggregate
 * `KernelMetrics`) is omitted here since this registry never holds a `Kernel` to sample it from.
 * `shards` is spelled as `ShardClient["shardMetricsSamples"]`'s own return type rather than importing
 * `ShardMetricsSample` by name — this file's `path_scope` is the `🐚️ActivationRegistry` region only,
 * and the top-of-file import list sits outside it. */
export interface RuntimeMetricsSnapshot {
  readonly actors: readonly RuntimeMetricsActorRow[];
  readonly shards: ReturnType<ShardClient["shardMetricsSamples"]>;
  readonly sampledAtMs: number;
}

/** ⏱️ 2Hz, matching `semio_framework_actor::RUNTIME_METRICS_PUBLISH_INTERVAL_MS`. */
export const RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500;

/** ⏱️ Pure cadence gate — the exact TS mirror of `semio_framework_actor::runtime_metrics_due`, kept
 * as its own exported function (not inlined into `startRuntimeMetricsPublisher`) so it is testable
 * without fake timers, same reasoning as the Rust side's own doc comment. */
export function runtimeMetricsDue(lastPublishedMs: number | null, nowMs: number): boolean {
  if (lastPublishedMs === null) return true;
  return nowMs - lastPublishedMs >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
}

//#region 🧪️RuntimeMetricsTests
/** 🧪️ Kept inside the `🐚️ActivationRegistry` region on purpose — this file's other test blocks
 * (`🧪️ExpandPluginRegistryTests`/`🧪️IoRouterTests`) live at end-of-file, but this packet's
 * `path_scope` is this region only, and a peer holds `🔖️IoRouter` (must stay byte-identical). */
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  const BUDGET_FIXTURE: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };

  /** 🧪️ A `ShardWorkerLike` that immediately auto-replies success to every request-bearing message —
   * enough for `ActivationRegistry.activate`/`suspend` to resolve without hand-delivering replies
   * (unlike `shard-client.ts`'s own `FakeShardWorker`, which is deliberately manual for THAT file's
   * out-of-order-reply tests; this region only needs "the round trip completes"). Factored out (web-
   * activation) so tests that need their own `ShardClient` construction (custom `onShardLost`/
   * `exclusiveShardCount`, not just the one-liner {@link fakeShardClient} covers) can reuse it too. */
  function createAutoReplyWorker(): { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } {
    const worker: { postMessage: (message: unknown) => void; terminate: () => void; onmessage: ((event: { readonly data: unknown }) => void) | null; onerror: ((event: unknown) => void) | null } = {
      postMessage: (message) => {
        const requestId = (message as { readonly requestId?: string }).requestId;
        if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
      },
      terminate: () => {},
      onmessage: null,
      onerror: null,
    };
    return worker;
  }

  function fakeShardClient(shardCount = 1): ShardClient {
    return new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount, createWorker: () => createAutoReplyWorker() });
  }

  function fixtureResidentLedger(): OwnedResidentLedger { return new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }); }

  /** 🧪️ Advances `n` real microtask ticks with no real timer/sleep involved — enough hops for a
   * `TurnScheduler` pump + a fake-worker's `queueMicrotask` reply + this registry's own
   * `runQueuedTurn` await chain to settle deterministically. */
  async function flushMicrotasks(n = 10): Promise<void> {
    for (let i = 0; i < n; i += 1) await Promise.resolve();
  }

  describe("ActivationRegistry.runtimeMetricsActorRows / runtimeMetricsSnapshot", () => {
    it("rows cover both resident and suspended actors, never activated-and-forgotten ones", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 500, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);

      await registry.suspend("actor-1");
      const afterSuspend = registry.runtimeMetricsActorRows();
      expect(afterSuspend).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);
    });

    it("snapshot combines actor rows with ShardClient.shardMetricsSamples at the given clock reading", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, now: () => 999, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      const snapshot = registry.runtimeMetricsSnapshot(1_000);
      expect(snapshot.sampledAtMs).toBe(1_000);
      expect(snapshot.actors).toHaveLength(1);
      expect(snapshot.shards).toEqual(shardClient.shardMetricsSamples(1_000));
    });
  });

  describe("runtimeMetricsDue", () => {
    it("gates at the 500ms / 2Hz interval, always due on the first call", () => {
      expect(runtimeMetricsDue(null, 0)).toBe(true);
      expect(runtimeMetricsDue(1_000, 1_200)).toBe(false);
      expect(runtimeMetricsDue(1_000, 1_500)).toBe(true);
    });
  });

  describe("ActivationRegistry.startRuntimeMetricsPublisher", () => {
    it("calls the sink with the os.runtime.metrics topic at the 2Hz interval, and stop() cancels it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const calls: Array<{ readonly topic: string; readonly snapshot: RuntimeMetricsSnapshot }> = [];
        const stop = registry.startRuntimeMetricsPublisher((topic, snapshot) => calls.push({ topic, snapshot }));

        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(1);
        expect(calls[0]!.topic).toBe("os.runtime.metrics");

        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(calls).toHaveLength(2);

        stop();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 3);
        expect(calls).toHaveLength(2);
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("ActivationRegistry.cancel", () => {
    it("disposes the worker-side instance and forgets the actor entirely — resume() afterward throws unknown actor", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      expect(registry.isResident("actor-1")).toBe(true);

      registry.cancel("actor-1");

      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      expect(shardClient.shardIndexFor("actor-1")).toBeUndefined(); // dispose() cleared the routing entry
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
    });

    it("is a no-op for an actor this registry never activated", () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(() => registry.cancel("ghost")).not.toThrow();
    });

    it("cancelling a suspended (non-resident but still tracked) actor still forgets it", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");
      await registry.suspend("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: false, shard: null }]);

      registry.cancel("actor-1");
      expect(registry.runtimeMetricsActorRows()).toEqual([]);
    });
  });

  //#region 🧪️ExtensionCascadeTests
  /** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): a minimal `PluginCatalog`
   * with ONE plugin (`p1`) and ONE extension (`p1-ext`) whose `dependsOn: ["p1"]` names it as the
   * parent — exactly the shape `registerCatalog`'s own `extensionsByParent` index groups by. */
  function catalogWithOneExtension(): PluginCatalog {
    return {
      plugins: [{ pluginId: "p1", wasmOut: "p1.wasm", role: "plugin", contributes: [], consumes: [] }],
      extensions: [{ pluginId: "p1-ext", wasmOut: "p1-ext.wasm", role: "extension", contributes: [], consumes: [], dependsOn: ["p1"] }],
      hosts: [],
      playgrounds: [],
      moduleUrl: (pluginId, wasmOut) => `https://x/${pluginId}/${wasmOut}`,
      extensionModuleUrl: (pluginId, wasmOut) => `https://x/ext/${pluginId}/${wasmOut}`,
    };
  }

  describe("intersectCapabilityGrants", () => {
    it("keeps only requested grants the parent's own granted set also carries, matched by id", () => {
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      const granted = [grant("fs.read"), grant("net.fetch")];
      const requested = [grant("fs.read"), grant("fs.admin")];
      expect(intersectCapabilityGrants(granted, requested).map((g) => g.id)).toEqual(["fs.read"]);
    });

    it("is empty when the parent holds nothing, never escalates an ungranted request", () => {
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      expect(intersectCapabilityGrants([], [grant("fs.admin")])).toEqual([]);
    });
  });

  describe("ActivationRegistry extension cascade (registerCatalog)", () => {
    it("activate() cascades to every registered extension of the plugin, under a deterministic child actorId", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());

      await registry.activate("p1", "actor-1", "manual");

      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
      const rows = registry.runtimeMetricsActorRows();
      expect(rows).toContainEqual({ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 });
      expect(rows).toContainEqual({ actorId: "actor-1::p1-ext", pluginId: "p1-ext", resident: true, shard: 0 });
    });

    it("a plugin with no registered extensions activates with no cascade side effects", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });

      await registry.activate("p1", "actor-1", "manual");

      expect(registry.runtimeMetricsActorRows()).toEqual([{ actorId: "actor-1", pluginId: "p1", resident: true, shard: 0 }]);
    });

    it("suspend() cascades leaves-first, resume() cascades parent-first — zero orphans either way", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");

      await registry.suspend("actor-1");
      expect(registry.isResident("actor-1")).toBe(false);
      expect(registry.isResident("actor-1::p1-ext")).toBe(false);
      // still tracked (suspended, not cancelled) — resume must find both again.
      expect(registry.runtimeMetricsActorRows().map((r) => r.actorId).sort()).toEqual(["actor-1", "actor-1::p1-ext"]);

      await registry.resume("actor-1");
      expect(registry.isResident("actor-1")).toBe(true);
      expect(registry.isResident("actor-1::p1-ext")).toBe(true);
    });

    it("cancel() on the parent takes its extension down too — permanently, zero orphans", async () => {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      await registry.activate("p1", "actor-1", "manual");

      registry.cancel("actor-1");

      expect(registry.runtimeMetricsActorRows()).toEqual([]);
      await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
      await expect(registry.resume("actor-1::p1-ext")).rejects.toThrow(/unknown actor/);
    });

    it("scopes an extension's activated caps to the intersection with its parent's own granted set", async () => {
      const shardClient = fakeShardClient();
      const sentCaps = new Map<string, readonly ShardCapabilityGrant[]>();
      const worker = createAutoReplyWorker();
      const originalPostMessage = worker.postMessage;
      worker.postMessage = (message) => {
        const msg = message as { readonly kind?: string; readonly actorId?: string; readonly caps?: readonly ShardCapabilityGrant[] };
        if (msg.kind === "activate" && msg.actorId) sentCaps.set(msg.actorId, msg.caps ?? []);
        originalPostMessage(message);
      };
      const client = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 1, createWorker: () => worker });
      const registry = new ActivationRegistry({ shardClient: client, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerCatalog(catalogWithOneExtension());
      const grant = (id: string): ShardCapabilityGrant => ({ id, token: "t", scope: "s", expiresMs: null });
      // Override the parent's manifest (registerCatalog seeds `caps: []`) so there is something real
      // to intersect against, and the extension's own manifest with a request that only PARTIALLY
      // overlaps — `fs.read` must survive, `fs.admin` must not, the parent never held it.
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1/p1.wasm", caps: [grant("fs.read")] });
      registry.registerManifest({ pluginId: "p1-ext", moduleUrl: "https://x/ext/p1-ext/p1-ext.wasm", caps: [grant("fs.read"), grant("fs.admin")] });

      await registry.activate("p1", "actor-1", "manual");

      expect(sentCaps.get("actor-1::p1-ext")?.map((g) => g.id)).toEqual(["fs.read"]);
    });
  });
  //#endregion 🧪️ExtensionCascadeTests

  //#region 🧪️TurnDispatchTests
  describe("ActivationRegistry.enqueueTurn lane priority", () => {
    it("dispatches turns by lane priority end-to-end through the registry, not enqueue order", async () => {
      const shardClient = fakeShardClient();
      const order: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: (actorId) => order.push(actorId) });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "low", "manual");
      await registry.activate("p1", "high", "manual");
      await registry.activate("p1", "mid", "manual");

      registry.enqueueTurn("low", "Background", []);
      registry.enqueueTurn("high", "Interactive", []);
      registry.enqueueTurn("mid", "UserVisible", []);

      await flushMicrotasks();
      expect(order).toEqual(["high", "mid", "low"]);
    });
  });

  describe("ActivationRegistry.suspend cancels queued turns", () => {
    it("a suspended actor's queued turns are cancelled, never delivered", async () => {
      const shardClient = fakeShardClient();
      const delivered: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      registry.enqueueTurn("actor-1", "Interactive", []); // queued but not yet dispatched
      await registry.suspend("actor-1"); // cancels it synchronously, before checkpoint/dispose even starts

      await flushMicrotasks();
      expect(delivered).toEqual([]);

      await registry.resume("actor-1");
      await flushMicrotasks();
      expect(delivered).toEqual([]); // still nothing — the cancelled turn never resurfaces after resume
    });
  });
  //#endregion 🧪️TurnDispatchTests

  //#region 🧪️ShardLossRestoreTests
  describe("ActivationRegistry.handleShardLost / restoreActors", () => {
    it("is a valid ShardClientOptions.onShardLost value", () => {
      let registry!: ActivationRegistry;
      const shardClient = new ShardClient({
        residentLedger: fixtureResidentLedger(),
        shardCount: 1,
        createWorker: () => createAutoReplyWorker(),
        onShardLost: (shardIndex, actorIds) => registry.handleShardLost(shardIndex, actorIds),
      });
      registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      expect(typeof registry.handleShardLost).toBe("function");
    });

    it("restores exactly the actors that were on the lost shard, leaving an actor on a different shard untouched", async () => {
      const shardClient = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 2, exclusiveShardCount: 0, createWorker: () => createAutoReplyWorker() });
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });

      await registry.activate("p1", "on-shard-0", "manual");
      await registry.activate("p1", "on-shard-1", "manual");
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0);
      expect(shardClient.shardIndexFor("on-shard-1")).toBe(1);

      // simulate what checkHeartbeats' own 3-strike ladder does — terminate + rebuild shard 0 only,
      // then hand its actorIds to the registry exactly as ShardClient's own onShardLost callback would.
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);
      expect(lostActorIds).toEqual(["on-shard-0"]);

      await registry.restoreActors(lostActorIds);

      expect(registry.isResident("on-shard-0")).toBe(true); // restored
      expect(registry.isResident("on-shard-1")).toBe(true); // never touched — different shard
      expect(shardClient.shardIndexFor("on-shard-0")).toBe(0); // re-activated on the rebuilt shard
    });
  });

  describe("ActivationRegistry restore ordering", () => {
    it("a restored actor does not receive turns that were queued before the restore, but does receive turns queued after", async () => {
      const shardClient = new ShardClient({ residentLedger: fixtureResidentLedger(), shardCount: 1, createWorker: () => createAutoReplyWorker() });
      const delivered: string[] = [];
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], onTurnResult: () => delivered.push("delivered") });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      await registry.activate("p1", "actor-1", "manual");

      // Enqueue, then lose the shard, all synchronously — nothing yields to the scheduler's own
      // microtask pump until after `restoreActors` has already cancelled the queue below.
      registry.enqueueTurn("actor-1", "Interactive", []);
      const lostActorIds = shardClient.terminate(0);
      shardClient.rebuild(0);

      await registry.restoreActors(lostActorIds);
      await flushMicrotasks();
      expect(delivered).toEqual([]); // the pre-restart turn never ran

      registry.enqueueTurn("actor-1", "Interactive", []); // enqueued AFTER the restore completed
      await flushMicrotasks();
      expect(delivered).toEqual(["delivered"]); // proves the actor is alive again, not permanently dropped
    });
  });
  //#endregion 🧪️ShardLossRestoreTests

  //#region 🧪️MemoryPressureCapTests
  describe("residentActorCapFromMemory", () => {
    it("derives the cap from deviceMemoryGiB when present, clamped to [4, 96]", () => {
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 1 })).toBe(6);
      expect(residentActorCapFromMemory({ deviceMemoryGiB: 16 })).toBe(96);
    });

    it("falls back to jsHeapSizeLimitBytes when deviceMemoryGiB is absent", () => {
      expect(residentActorCapFromMemory({ jsHeapSizeLimitBytes: 256 * 1024 * 1024 })).toBe(4);
    });

    it("falls back to the hardcoded constant when neither signal is present", () => {
      expect(residentActorCapFromMemory({})).toBe(DEFAULT_MAX_RESIDENT_ACTORS);
    });
  });

  describe("ActivationRegistry.maxResidentActors derived from an injected memory probe", () => {
    async function activateAndCountResident(memoryProbe: MemoryProbe, activationCount: number): Promise<number> {
      const shardClient = fakeShardClient();
      const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [], memoryProbe });
      registry.registerManifest({ pluginId: "p1", moduleUrl: "https://x/p1.js", caps: [] });
      for (let i = 0; i < activationCount; i += 1) await registry.activate("p1", `actor-${i}`, "manual");
      let resident = 0;
      for (let i = 0; i < activationCount; i += 1) if (registry.isResident(`actor-${i}`)) resident += 1;
      return resident;
    }

    it("a small deviceMemoryGiB reading evicts down to its (small) derived cap", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 1 }), 10);
      expect(resident).toBe(residentActorCapFromMemory({ deviceMemoryGiB: 1 })); // 6
      expect(resident).toBeLessThan(10);
    });

    it("a large deviceMemoryGiB reading keeps every one of the same 10 activations resident", async () => {
      const resident = await activateAndCountResident(() => ({ deviceMemoryGiB: 16 }), 10);
      expect(resident).toBe(10); // well under the derived 96 cap — nothing evicted
    });
  });
  //#endregion 🧪️MemoryPressureCapTests

  //#region 🧪️MetricsBusTests
  describe("ActivationRegistry.metricsBus (autoStartMetricsPublisher)", () => {
    it("publishes os.runtime.metrics as a CustomEvent on metricsBus at the 2Hz interval, driven by the injected clock, and dispose() stops it", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        let simulatedNowMs = 0;
        const registry = new ActivationRegistry({
          shardClient,
          defaultBudget: BUDGET_FIXTURE,
          fetchAssets: async () => [],
          now: () => simulatedNowMs,
          autoStartMetricsPublisher: true,
        });
        const received: RuntimeMetricsSnapshot[] = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push((event as CustomEvent<RuntimeMetricsSnapshot>).detail));

        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(1);
        expect(received[0]!.sampledAtMs).toBe(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);

        simulatedNowMs = RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 2;
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
        expect(received).toHaveLength(2);

        registry.dispose();
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 5);
        expect(received).toHaveLength(2); // dispose() stopped the loop
      } finally {
        vi.useRealTimers();
      }
    });

    it("stays empty (no live interval, no bus traffic) when autoStartMetricsPublisher is left at its default", () => {
      vi.useFakeTimers();
      try {
        const shardClient = fakeShardClient();
        const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET_FIXTURE, fetchAssets: async () => [] });
        const received: RuntimeMetricsSnapshot[] = [];
        registry.metricsBus.addEventListener("os.runtime.metrics", (event) => received.push((event as CustomEvent<RuntimeMetricsSnapshot>).detail));
        vi.advanceTimersByTime(RUNTIME_METRICS_PUBLISH_INTERVAL_MS * 10);
        expect(received).toEqual([]);
        registry.dispose(); // no-op, nothing was started — must not throw
      } finally {
        vi.useRealTimers();
      }
    });
  });
  //#endregion 🧪️MetricsBusTests
}
//#endregion 🧪️RuntimeMetricsTests
//#endregion 🐚️ActivationRegistry

//#region 🔌️PluginSource
/** @emoji 🔌️ Dev-server SSE endpoint a `PluginSource` availability stream connects to (see
 * {@link createDevPluginSource}) — mounted by the dev runner's `semioPluginHotSwapVitePlugin`
 * alongside the `/plugin-modules` static alias it watches. Shared here (rather than duplicated as a
 * literal in both the dev vite plugin and the shell) so the two ends can't drift apart. */
export const PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch";

/** @emoji 🔌️ One entry of an availability stream: either the full set of currently-built plugins sent
 * once on connect (a reconnecting/late-connecting browser must not miss builds that already finished),
 * or a single plugin's rebuild landing. `rebuiltAt` is the artifact's build timestamp and doubles as
 * the cache-busting query value {@link PluginSource.moduleUrl} mints. */
export type PluginSourceEvent = { readonly kind: "snapshot"; readonly plugins: readonly { readonly pluginId: string; readonly rebuiltAt: number }[] } | { readonly kind: "built"; readonly pluginId: string; readonly rebuiltAt: number };

/**
 * @emoji 🔌️ Where the shell's incremental plugin runtime (install/uninstall/reload — see the react
 * renderer's plugin panel) gets its catalog and availability notifications from. `createDevPluginSource`
 * is the only implementation today; a future `HubPluginSource` (fetching manifests and artifacts from
 * the plugin hub over HTTP/SSE instead of the local dev server) implements the same three methods and
 * needs no changes anywhere else — the shell only ever depends on this interface.
 */
export interface PluginSource {
  readonly id: string;
  /** Every plugin this source can currently install (built or not — the panel shows "available"
   * entries that haven't finished their first build yet). */
  list(): Promise<readonly PluginRegistryEntry[]>;
  /** Mints a concrete, cache-busted module URL for one install/reload of `pluginId`. Omitting
   * `rebuiltAt` (initial install, before any `built` event) falls back to the registry's own
   * `moduleUrl`, unbusted — correct for a first load, where there is nothing stale to bust. */
  moduleUrl(pluginId: string, rebuiltAt?: number): string;
  /** Subscribes to availability events; returns an unsubscribe function. Fires an immediate `snapshot`
   * on subscribe against sources that support it (the dev source's SSE endpoint always sends one). */
  subscribe(listener: (event: PluginSourceEvent) => void): () => void;
}

/** @emoji 🔌️ `PluginSource` backed by the dev server's static `/plugin-modules` output and its
 * {@link PLUGIN_SOURCE_WATCH_PATH} SSE stream. `EventSource` is unavailable under vitest/node, so
 * `subscribe` there is a harmless no-op (matches every other browser-only feature detection in this
 * module). */
export function createDevPluginSource(registry: readonly PluginRegistryEntry[]): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  const bootVersion = Date.now();
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      const separator = entry.moduleUrl.includes("?") ? "&" : "?";
      return `${entry.moduleUrl}${separator}v=${rebuiltAt ?? bootVersion}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined") return () => {};
      const source = new EventSource(PLUGIN_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data) as PluginSourceEvent);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "dev" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    },
  };
}

/** @emoji 🧩️ Dev-server SSE endpoint for {@link createExtensionSource} — paired with the `/extensions`
 * static route the extension store materializes at install time. */
export const EXTENSION_SOURCE_WATCH_PATH = "/extensions/watch";

type ExtensionSourceWireEvent =
  | { readonly kind: "snapshot"; readonly extensions: readonly { readonly extensionId: string; readonly installedAt: number }[] }
  | { readonly kind: "installed"; readonly extensionId: string; readonly installedAt: number }
  | { readonly kind: "uninstalled"; readonly extensionId: string };

/** @emoji 🔁️ Converts the extension store's install vocabulary into the runtime's plugin
 * availability vocabulary. Uninstall events have no availability equivalent and are ignored. */
export function extensionSourceEventToPluginSourceEvent(event: ExtensionSourceWireEvent): PluginSourceEvent | undefined {
  if (event.kind === "snapshot") {
    if (!Array.isArray(event.extensions)) throw new Error("snapshot extensions must be an array");
    return { kind: "snapshot", plugins: event.extensions.map((extension) => ({ pluginId: extension.extensionId, rebuiltAt: extension.installedAt })) };
  }
  if (event.kind === "installed") return { kind: "built", pluginId: event.extensionId, rebuiltAt: event.installedAt };
  if (event.kind === "uninstalled") return undefined;
  throw new Error("unknown extension source event kind");
}

/** @emoji 🧩️ `PluginSource` backed by the extension store's `/extensions` HTTP tree and its watch SSE
 * stream. Catalog rows come from the injected {@link PluginCatalog}'s `extensions`; runtime installs
 * add artifacts under each extension id without changing this list. */
export function createExtensionSource(catalog: PluginCatalog): PluginSource {
  const registry: readonly PluginRegistryEntry[] = catalog.extensions.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: catalog.extensionModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn),
  }));
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "extensions",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "extensions" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined") return () => {};
      const source = new EventSource(EXTENSION_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          const normalized = extensionSourceEventToPluginSourceEvent(JSON.parse(event.data) as ExtensionSourceWireEvent);
          if (normalized) listener(normalized);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "extensions" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    },
  };
}

/** @emoji 🔌️ Merges multiple {@link PluginSource} implementations — dev `/plugin-modules` plus extension
 * `/extensions` — into one catalog the shell's incremental runtime can treat as a single source. */
export function multiplexPluginSources(...sources: readonly PluginSource[]): PluginSource {
  if (sources.length === 0) throw new Error("[DEBUG] multiplexPluginSources requires at least one source");
  if (sources.length === 1) return sources[0];
  return {
    id: sources.map((source) => source.id).join("+"),
    async list() {
      const merged = new Map<string, PluginRegistryEntry>();
      for (const entries of await Promise.all(sources.map((source) => source.list()))) {
        for (const entry of entries) merged.set(entry.pluginId, entry);
      }
      return [...merged.values()];
    },
    moduleUrl(pluginId, rebuiltAt) {
      for (const source of sources) {
        try {
          return source.moduleUrl(pluginId, rebuiltAt);
        } catch {
          continue;
        }
      }
      throw new Error(`[DEBUG] multiplexed plugin sources have no registry entry for ${pluginId}`);
    },
    subscribe(listener) {
      const unsubscribes = sources.map((source) => source.subscribe(listener));
      return () => {
        for (const unsubscribe of unsubscribes) unsubscribe();
      };
    },
  };
}
//#endregion 🔌️PluginSource

// #region 🎮️PlaygroundResolution
/** @emoji 🎮️ Finds the injected catalog's playground row for a variant id or one of its aliases. */
function findPlaygroundVariant(catalog: PluginCatalog, playgroundPluginId: string): PlaygroundCatalogTarget | undefined {
  return catalog.playgrounds.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}

/** @emoji 🎯️ Resolves a playground filter/alias (e.g. "3d", "sourcing") to its underlying wasm component registry id. */
export function resolvePluginRegistryId(catalog: PluginCatalog, playgroundPluginId: string): string {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.pluginId ?? playgroundPluginId;
}

/** @emoji 🎯️ Resolves a playground filter/alias to the app id that should be instantiated by default within its plugin's manifest. */
export function resolvePlaygroundDefaultAppId(catalog: PluginCatalog, playgroundPluginId: string): string | undefined {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.app;
}

export type PlaygroundBootSession = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

export type PlaygroundBoot = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
  /** 🧯 Any {@link PluginGraphError}s that kept an entry out of `plugins` — empty for the
   * `session`-reuse fast path (nothing was recomputed). A caller renders these through
   * {@link pluginGraphErrorMessage} instead of leaving the gap silent. */
  readonly dependencyErrors: readonly PluginGraphError[];
};

/** @emoji 🎮️ Resolves the wasm plugin list and default app for one playground variant; when the on-disk
 * `generated/🟦️session.ts` was overwritten by another concurrent dev variant, rebuilds from the injected
 * {@link PluginCatalog} instead of trusting the stale program rows. */
export function resolvePlaygroundBoot(catalog: PluginCatalog, variant: string, session?: PlaygroundBootSession): PlaygroundBoot {
  const defaultAppId = resolvePlaygroundDefaultAppId(catalog, variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins, dependencyErrors: [] };
  }
  const registryPluginId = resolvePluginRegistryId(catalog, variant);
  const hostMode = resolvePluginHostConfig(catalog, variant) !== undefined;
  const catalogPlugins: PluginRegistryEntry[] = [...catalog.plugins, ...catalog.extensions].map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: target.role === "extension" ? catalog.extensionModuleUrl(target.pluginId, target.wasmOut) : catalog.moduleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn),
  }));
  const expanded = expandPluginRegistry(catalogPlugins, hostMode ? undefined : registryPluginId, hostMode);
  // 🎯️ Boot activates in dependency order, not array order (scout-2 §4) — entries a dependency-graph
  // fault blocks are simply left out of THIS list (best-effort degrade, contract freeze §4 rule 5);
  // the caller surfaces `errors` through `pluginGraphErrorMessage` for the dependency-fault UI rather
  // than this resolver throwing and taking the whole boot down with it.
  const { order, errors } = orderPluginRegistryEntries(expanded);
  if (errors.length > 0) {
    for (const error of errors) console.error(`[DEBUG] resolvePlaygroundBoot(${variant}): ${pluginGraphErrorMessage(error, "en")}`);
  }
  return {
    variant,
    defaultAppId,
    plugins: order,
    dependencyErrors: errors,
  };
}

//#region 🏠️🧳️PluginHostConfig
/** 🏠️🧳️ Declares, for a plugin whose manifest offers a host-style multi-app experience (one app is the
 * landing/default view, another hosts other apps as spawned sub-instances — e.g. "s"'s home/studio
 * pair), which app ids play which role. Callers resolve controller ids and default panel tabs from
 * the *loaded manifest*'s own `controllerId`/`panelTabs` on those apps rather than hardcoding separate
 * literals — this table only ever needs to carry app-id role assignments. A pluginFilter absent here
 * simply boots through the ordinary single-app path (`resolvePlaygroundDefaultAppId`). Mirrored by
 * `PLUGIN_HOST_CONFIGS`/`resolve_plugin_host_config` in `framework/os/renderer/wgpu/rs/lib.rs`'s
 * `program_bridge` module for the WGPU renderer. */
export type PluginHostConfig = {
  readonly pluginId: string;
  readonly landingAppId: string;
  readonly hostAppId: string;
};

/** 🎯️ Resolves a playground filter/alias to its plugin's host config, or `undefined` when that program doesn't offer a host-style multi-app experience. */
export function resolvePluginHostConfig(catalog: PluginCatalog, playgroundPluginId: string): PluginHostConfig | undefined {
  const registryId = resolvePluginRegistryId(catalog, playgroundPluginId);
  return catalog.hosts.find((entry) => entry.pluginId === registryId);
}
//#endregion 🏠️🧳️PluginHostConfig
// #endregion 🎮️PlaygroundResolution

//#region 🔖️PluginGraph
/** 🔗️ One node of the plugin dependency graph — a plugin's own id/version plus the dependencies its
 * manifest declares. Mirrors Rust `PluginManifest`'s `pluginId`/`version`/`dependencies` triple
 * (`🛂️manifest/🦀️.rs`), narrowed to exactly what {@link resolvePluginLoadOrder}/
 * {@link validatePluginDependencyGraph} need — a caller with only a `PluginRegistryEntry` (no
 * `version` yet, contract freeze §3-era catalogs) still validates presence/cycles correctly. */
export type PluginGraphNode = {
  readonly pluginId: string;
  readonly version?: string;
  readonly dependencies?: readonly PluginDependency[];
};

/** 🧯 One dependency-graph fault — reuses the frozen transaction rejection codes (contract freeze §5
 * rejection taxonomy, ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS)
 * since plugin-load rejection and transaction contribution resolution share the same three failure
 * shapes (contract freeze §4 rule 5: "Dependency graph: missing dependency, version mismatch, or
 * cycle ⇒ plugin load rejected with a typed error"). */
export type PluginGraphError =
  | { readonly code: "transaction.dependency-missing"; readonly pluginId: string; readonly dependsOn: string }
  | { readonly code: "transaction.version-mismatch"; readonly pluginId: string; readonly dependsOn: string; readonly required: VersionReq; readonly actual: string }
  | { readonly code: "transaction.cycle"; readonly members: readonly string[] };

type ParsedVersion = { readonly major: number; readonly minor: number; readonly patch: number };

function parseVersion(raw: string | undefined): ParsedVersion | null {
  if (!raw) return null;
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match) return null;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]) };
}

function compareVersions(a: ParsedVersion, b: ParsedVersion): number {
  if (a.major !== b.major) return a.major - b.major;
  if (a.minor !== b.minor) return a.minor - b.minor;
  return a.patch - b.patch;
}

type ParsedVersionReq =
  | { readonly kind: "any" }
  | { readonly kind: "exact"; readonly version: ParsedVersion }
  | { readonly kind: "caret"; readonly version: ParsedVersion }
  | { readonly kind: "tilde"; readonly version: ParsedVersion }
  | { readonly kind: "atLeast"; readonly version: ParsedVersion };

/** 🔢️ Parses the frozen version-requirement grammar (contract freeze §3): `*`, `=X.Y.Z`, `^X.Y.Z`,
 * `~X.Y.Z`, `>=X.Y.Z`. Mirrors Rust `VersionReq::parse`'s accepted syntax exactly. */
function parseVersionReq(raw: VersionReq): ParsedVersionReq | null {
  const trimmed = raw.trim();
  if (trimmed === "*") return { kind: "any" };
  const opMatch = /^(=|\^|~|>=)(\d+\.\d+\.\d+)$/.exec(trimmed);
  if (!opMatch) return null;
  const version = parseVersion(opMatch[2]);
  if (!version) return null;
  switch (opMatch[1]) {
    case "=":
      return { kind: "exact", version };
    case "^":
      return { kind: "caret", version };
    case "~":
      return { kind: "tilde", version };
    case ">=":
      return { kind: "atLeast", version };
    default:
      return null;
  }
}

/** ✅️ True when `actual` (a plain `major.minor.patch` string) satisfies `requirement`. An
 * unparseable `actual`/`requirement` is treated as unsatisfied — never throws, matching the "typed
 * error, never a panic" law the Rust planner's law tests hold `PlanError` to (contract freeze §1 law 4). */
export function versionSatisfies(actual: string, requirement: VersionReq): boolean {
  const req = parseVersionReq(requirement);
  if (!req) return false;
  if (req.kind === "any") return true;
  const version = parseVersion(actual);
  if (!version) return false;
  if (req.kind === "exact") return compareVersions(version, req.version) === 0;
  if (req.kind === "atLeast") return compareVersions(version, req.version) >= 0;
  if (req.kind === "tilde") {
    return version.major === req.version.major && version.minor === req.version.minor && version.patch >= req.version.patch;
  }
  // caret — leading-zero-tier semver semantics: the first nonzero component of the REQUIREMENT pins
  // the upper bound; when every component is zero, only that exact version matches.
  if (compareVersions(version, req.version) < 0) return false;
  if (req.version.major > 0) return version.major === req.version.major;
  if (req.version.minor > 0) return version.major === 0 && version.minor === req.version.minor;
  return version.major === 0 && version.minor === 0 && version.patch === req.version.patch;
}

/** 🧯 Validates every node's declared `dependencies` resolve (present, version-satisfying) — does
 * NOT detect cycles (see {@link resolvePluginLoadOrder}, which layers cycle detection on top only
 * once every missing/mismatched edge has already been reported). A node with no `version` skips the
 * version check for edges pointing at it (nothing to compare against) rather than failing closed.
 * Mirrors Rust `validate_dependency_graph`. */
export function validatePluginDependencyGraph(nodes: readonly PluginGraphNode[]): readonly PluginGraphError[] {
  const byId = new Map(nodes.map((node) => [node.pluginId, node] as const));
  const errors: PluginGraphError[] = [];
  for (const node of nodes) {
    for (const dependency of node.dependencies ?? []) {
      const target = byId.get(dependency.pluginId);
      if (!target) {
        errors.push({ code: "transaction.dependency-missing", pluginId: node.pluginId, dependsOn: dependency.pluginId });
        continue;
      }
      if (target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {
        errors.push({ code: "transaction.version-mismatch", pluginId: node.pluginId, dependsOn: dependency.pluginId, required: dependency.version, actual: target.version });
      }
    }
  }
  return errors;
}

/** 🔁️ DFS cycle extraction restricted to `leftover` (the toposort leftover set) — names every plugin
 * actually on a cycle rather than the whole leftover set (which may include acyclic nodes downstream
 * of the real cycle). Falls back to the sorted leftover set only if no back-edge is found (should not
 * happen given `leftover` is non-empty and the full graph already passed structural validation). */
function findCycleMembers(byId: ReadonlyMap<string, PluginGraphNode>, leftover: ReadonlySet<string>): readonly string[] {
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const stack: string[] = [];
  let cycle: string[] | null = null;

  function visit(id: string): void {
    if (cycle || !leftover.has(id) || visited.has(id)) return;
    if (visiting.has(id)) {
      const start = stack.indexOf(id);
      cycle = stack.slice(start);
      return;
    }
    visiting.add(id);
    stack.push(id);
    for (const dependency of byId.get(id)?.dependencies ?? []) {
      if (leftover.has(dependency.pluginId)) visit(dependency.pluginId);
      if (cycle) return;
    }
    stack.pop();
    visiting.delete(id);
    visited.add(id);
  }

  for (const id of [...leftover].sort()) {
    visit(id);
    if (cycle) break;
  }
  return cycle ?? [...leftover].sort();
}

/** 🔁️ Kahn toposort with lexicographically-smallest-id tie-breaking — mirrors Rust
 * `resolve_load_order` exactly, including the deterministic tie-break. `errors` is non-empty and
 * `order` empty on any missing dependency or version mismatch (reported before a cycle would be,
 * matching the Rust validate-then-sort order) or on a real cycle (members individually named via
 * {@link findCycleMembers}, not just the toposort leftover set). */
export function resolvePluginLoadOrder(nodes: readonly PluginGraphNode[]): { readonly order: readonly string[]; readonly errors: readonly PluginGraphError[] } {
  const structural = validatePluginDependencyGraph(nodes);
  if (structural.length > 0) return { order: [], errors: structural };

  const byId = new Map(nodes.map((node) => [node.pluginId, node] as const));
  const indegree = new Map<string, number>();
  const dependents = new Map<string, string[]>();
  for (const node of nodes) {
    indegree.set(node.pluginId, indegree.get(node.pluginId) ?? 0);
    for (const dependency of node.dependencies ?? []) {
      indegree.set(node.pluginId, (indegree.get(node.pluginId) ?? 0) + 1);
      const list = dependents.get(dependency.pluginId) ?? [];
      list.push(node.pluginId);
      dependents.set(dependency.pluginId, list);
    }
  }

  const order: string[] = [];
  const remaining = new Map(indegree);
  const queue = [...indegree.entries()].filter(([, count]) => count === 0).map(([id]) => id);
  while (queue.length > 0) {
    queue.sort();
    const id = queue.shift()!;
    order.push(id);
    for (const dependent of dependents.get(id) ?? []) {
      const next = (remaining.get(dependent) ?? 0) - 1;
      remaining.set(dependent, next);
      if (next === 0) queue.push(dependent);
    }
  }

  if (order.length === nodes.length) return { order, errors: [] };
  const leftover = new Set(nodes.map((node) => node.pluginId).filter((id) => !order.includes(id)));
  return { order: [], errors: [{ code: "transaction.cycle", members: findCycleMembers(byId, leftover) }] };
}

/** 🔎️ Direct dependents of `pluginId` — every node that declares `pluginId` in its own `dependencies`
 * — sorted. Mirrors Rust `dependents`. */
export function pluginDependents(nodes: readonly PluginGraphNode[], pluginId: string): readonly string[] {
  return nodes
    .filter((node) => (node.dependencies ?? []).some((dependency) => dependency.pluginId === pluginId))
    .map((node) => node.pluginId)
    .sort();
}

/** 🕸️ Convenience wrapper over the pure {@link validatePluginDependencyGraph}/
 * {@link resolvePluginLoadOrder}/{@link pluginDependents} functions for a caller that wants to hold
 * one graph instance across several queries (boot ordering, then later a hot-reload/unload guard). */
export class PluginGraph {
  private readonly nodes: readonly PluginGraphNode[];
  constructor(nodes: readonly PluginGraphNode[]) {
    this.nodes = nodes;
  }
  validate(): readonly PluginGraphError[] {
    return validatePluginDependencyGraph(this.nodes);
  }
  loadOrder(): { readonly order: readonly string[]; readonly errors: readonly PluginGraphError[] } {
    return resolvePluginLoadOrder(this.nodes);
  }
  dependents(pluginId: string): readonly string[] {
    return pluginDependents(this.nodes, pluginId);
  }
  /** 🚫️ Contract freeze §4 rule 5's "unload refused while dependents are loaded" — `loadedIds` is
   * every plugin id currently resident (not merely declared in the graph), so a dependent that was
   * never actually loaded doesn't block `pluginId`'s unload. */
  canUnload(pluginId: string, loadedIds: ReadonlySet<string>): boolean {
    return this.dependents(pluginId).every((dependent) => !loadedIds.has(dependent));
  }
}

/** 🎯️ Orders `entries` by {@link PluginGraph.loadOrder}; entries the graph can't place (missing
 * dependency, version mismatch, or on a cycle) are dropped from the returned order and reported in
 * `errors` instead of silently keeping their original array position — contract freeze §4 rule 5's
 * "plugin load rejected with a typed error". Every other entry still boots, in dependency order
 * (best-effort degrade, matching the shell's existing fail-soft posture toward a single unavailable
 * plugin module). An entry with no declared `dependencies` at all always validates trivially, so a
 * registry that hasn't adopted `dependsOn` yet round-trips through this function unchanged (array
 * order in, array order out). */
export function orderPluginRegistryEntries(entries: readonly PluginRegistryEntry[]): { readonly order: readonly PluginRegistryEntry[]; readonly errors: readonly PluginGraphError[] } {
  const nodes: PluginGraphNode[] = entries.map((entry) => ({ pluginId: entry.pluginId, dependencies: entry.dependencies }));
  const { order, errors } = new PluginGraph(nodes).loadOrder();
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry] as const));
  if (errors.length === 0) {
    return { order: order.map((id) => byId.get(id)).filter((entry): entry is PluginRegistryEntry => entry !== undefined), errors: [] };
  }
  // 🔁 Retry on the remaining (non-blocked) subset — a single missing/mismatched/cyclic entry must
  // not degrade every OTHER entry back to plain array order; it only takes itself (and, for a cycle,
  // its fellow cycle members) out of the graph. Each retry strictly shrinks `entries`, so this always
  // terminates.
  const blocked = new Set(errors.flatMap((error) => (error.code === "transaction.cycle" ? error.members : [error.pluginId])));
  const remaining = entries.filter((entry) => !blocked.has(entry.pluginId));
  const retried = orderPluginRegistryEntries(remaining);
  return { order: retried.order, errors: [...errors, ...retried.errors] };
}
//#endregion 🔖️PluginGraph

//#region 🌐️DependencyFault
/** 🌐️ Picks the best string out of a {@link LocalizedLabel}-shaped `{en, de}` record for `locale` —
 * falls back to English, then to whatever key exists, since this repo supports multiple languages
 * with no default language but a fault MUST still render something rather than nothing. */
function resolveLocalizedLabel(label: Record<string, string>, locale: ShellLocale): string {
  return label[locale] ?? label.en ?? Object.values(label)[0] ?? "";
}

/** 🌐️ Turns a {@link PluginGraphError} into a real, localized (English + German) message — the
 * dependency-fault UI this ticket requires instead of a bare console error. Callers needing a
 * console-safe fallback can still log the same string; this is the single source of the wording so
 * a boot banner and an in-shell notification never drift apart. */
export function pluginGraphErrorMessage(error: PluginGraphError, locale: ShellLocale): string {
  switch (error.code) {
    case "transaction.dependency-missing":
      return resolveLocalizedLabel(
        {
          en: `Plugin "${error.pluginId}" needs "${error.dependsOn}", which is not installed.`,
          de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“, welches nicht installiert ist.`,
        },
        locale,
      );
    case "transaction.version-mismatch":
      return resolveLocalizedLabel(
        {
          en: `Plugin "${error.pluginId}" needs "${error.dependsOn}" ${error.required}, but ${error.actual} is installed.`,
          de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“ ${error.required}, installiert ist jedoch ${error.actual}.`,
        },
        locale,
      );
    case "transaction.cycle":
      return resolveLocalizedLabel(
        {
          en: `Plugin dependency cycle: ${error.members.join(" → ")}.`,
          de: `Zyklische Plugin-Abhängigkeit: ${error.members.join(" → ")}.`,
        },
        locale,
      );
  }
}
//#endregion 🌐️DependencyFault

//#region 🔖️InstanceDirectory
/** 🗺️ Where one artifact instance lives — mirrors the Rust host's `InstanceDirectory` entry shape
 * (contract freeze, W2 ownership doc: "`InstanceDirectory` mapping an artifact ref to `(pluginId,
 * instanceId, artifactKind)`"). */
export type ArtifactInstanceRef = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly artifactKind: string;
};

/** 🗺️ Host-side registry from artifact id to the plugin instance that owns it — the transaction
 * coordinator's `InstanceDirectory(target) → (plugin, instance)` lookup (contract freeze §5.3).
 * Registration/unregistration is the caller's responsibility (on `createApp`/`loadDocument` and on
 * `destroyApp`), matching how the Rust host's directory is populated. */
export class InstanceDirectory {
  private readonly byArtifactId = new Map<string, ArtifactInstanceRef>();

  register(artifactId: string, ref: ArtifactInstanceRef): void {
    this.byArtifactId.set(artifactId, ref);
  }

  unregister(artifactId: string): void {
    this.byArtifactId.delete(artifactId);
  }

  resolve(artifactId: string): ArtifactInstanceRef | undefined {
    return this.byArtifactId.get(artifactId);
  }

  entries(): ReadonlyArray<readonly [string, ArtifactInstanceRef]> {
    return [...this.byArtifactId.entries()];
  }
}
//#endregion 🔖️InstanceDirectory

//#region 🔖️ArtifactRouters
/** 🧯 Thrown by both routers' `registerContributed` when the same `(artifactKind, key)` is claimed
 * twice with non-identical metadata — contract freeze §4 rule 3's conflict rule. */
export class ArtifactRouterConflictError extends Error {
  readonly code = "artifact-router.conflict" as const;
  constructor(artifactKind: string, key: string) {
    super(`[DEBUG] router conflict: ${artifactKind}#${key} already registered with different metadata`);
    this.name = "ArtifactRouterConflictError";
  }
}

/** 🧯 Thrown when a contributor registers onto an artifact kind whose owning plugin is not a direct
 * entry in the contributor's declared `dependencies` — contract freeze §4 rule 1. Carries the frozen
 * transaction rejection code since an unpermitted contribution is exactly what that code names. */
export class ArtifactContributionNotPermittedError extends Error {
  readonly code = "transaction.contribution-not-permitted" as const;
  constructor(contributorPluginId: string, ownerPluginId: string) {
    super(`[DEBUG] "${contributorPluginId}" may not contribute onto "${ownerPluginId}"'s artifact kind — not a direct dependency`);
    this.name = "ArtifactContributionNotPermittedError";
  }
}

/** 🔢️ Deterministic deep stringify (sorted object keys) — the "byte-identical metadata" idempotence
 * check contract freeze §4 rule 3 asks for, without requiring an actual byte-level codec on the TS
 * side (mirrors the same rule Rust's `ArtifactInferenceRouter::register_plugin` already enforces). */
function stableStringify(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(stableStringify).join(",")}]`;
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableStringify(record[key])}`).join(",")}}`;
}

export type ArtifactRouterOwnership = { readonly kind: "owner" } | { readonly kind: "contributed"; readonly pluginId: string };

/** 🗂️ Shared conflict-checked `(artifactKind, key) -> ownership` table both routers below build on —
 * contract freeze §4 rule 3's conflict rule in one place instead of duplicated per router. */
class ConflictCheckedRegistry {
  private readonly entries = new Map<string, { readonly ownership: ArtifactRouterOwnership; readonly fingerprint: string }>();

  register(artifactKind: string, key: string, ownership: ArtifactRouterOwnership, metadata: unknown): void {
    const compositeKey = `${artifactKind} ${key}`;
    const fingerprint = stableStringify(metadata);
    const existing = this.entries.get(compositeKey);
    if (existing && existing.fingerprint !== fingerprint) throw new ArtifactRouterConflictError(artifactKind, key);
    this.entries.set(compositeKey, { ownership, fingerprint });
  }

  resolve(artifactKind: string, key: string): ArtifactRouterOwnership | undefined {
    return this.entries.get(`${artifactKind} ${key}`)?.ownership;
  }
}

/** 🗂️ `(artifactKind, mutationId) -> Owner | Contributed{pluginId}` — the transaction coordinator's
 * `ArtifactMutationRouter` lookup (contract freeze §5.3). */
export class ArtifactMutationRouter {
  private readonly registry = new ConflictCheckedRegistry();

  registerOwner(artifactKind: string, mutationId: string): void {
    this.registry.register(artifactKind, mutationId, { kind: "owner" }, { kind: "owner", artifactKind, mutationId });
  }

  /** Contract freeze §4 rule 1 registration gate: `contributorDependsOnOwner` must already be true —
   * callers derive it from a {@link PluginGraph} lookup (the contributor's declared `dependencies`
   * directly naming `ownerPluginId`) before calling this. */
  registerContributed(artifactKind: string, contributorPluginId: string, ownerPluginId: string, metadata: ContributedMutationMetadata, contributorDependsOnOwner: boolean): void {
    if (!contributorDependsOnOwner) throw new ArtifactContributionNotPermittedError(contributorPluginId, ownerPluginId);
    this.registry.register(artifactKind, metadata.mutationId, { kind: "contributed", pluginId: contributorPluginId }, metadata);
  }

  resolve(artifactKind: string, mutationId: string): ArtifactRouterOwnership | undefined {
    return this.registry.resolve(artifactKind, mutationId);
  }
}

/** 💡️ `(artifactKind, inferenceSchema) -> Owner | Contributed{pluginId}`, plus the contributed
 * `dependsOn` DAG (contract freeze §5.3/§6's `dependencies` list on `artifact-inference-request`) —
 * the transaction coordinator's contributor-aware `ArtifactInferenceRouter`. */
export class ArtifactInferenceRouter {
  private readonly registry = new ConflictCheckedRegistry();
  private readonly dependsOn = new Map<string, readonly string[]>();

  registerOwner(artifactKind: string, inferenceSchema: string): void {
    this.registry.register(artifactKind, inferenceSchema, { kind: "owner" }, { kind: "owner", artifactKind, inferenceSchema });
  }

  /** Contract freeze §4 rules 1+4: the contributor must directly depend on the owner, and the
   * metadata's own `owner`/`contributor` must match each other and `artifactKind` must match the
   * target. */
  registerContributed(artifactKind: string, metadata: ContributedInferenceMetadata, contributorDependsOnOwner: boolean): void {
    if (metadata.owner !== metadata.contributor) {
      throw new Error(`[DEBUG] contributed inference owner/contributor mismatch: ${metadata.owner} !== ${metadata.contributor}`);
    }
    if (metadata.artifactKind !== artifactKind) {
      throw new Error(`[DEBUG] contributed inference artifactKind mismatch: ${metadata.artifactKind} !== ${artifactKind}`);
    }
    if (!contributorDependsOnOwner) throw new ArtifactContributionNotPermittedError(metadata.contributor, artifactKind);
    this.registry.register(artifactKind, metadata.inferenceSchema, { kind: "contributed", pluginId: metadata.contributor }, metadata);
    this.dependsOn.set(`${artifactKind} ${metadata.inferenceSchema}`, metadata.dependsOn ?? []);
  }

  resolve(artifactKind: string, inferenceSchema: string): ArtifactRouterOwnership | undefined {
    return this.registry.resolve(artifactKind, inferenceSchema);
  }

  /** 🔗️ Topological order of every registered contributed inference's `(artifactKind,
   * inferenceSchema)` key honoring the `dependsOn` DAG (an inference that itself needs another
   * contributed inference's output runs after it) — Kahn toposort over `dependsOn` edges, same
   * lexicographic tie-break as {@link resolvePluginLoadOrder}. Throws {@link Error} naming the
   * cyclic keys on a cycle (never silently drops an entry). */
  dependencyOrder(): readonly string[] {
    const keys = [...this.dependsOn.keys()];
    const indegree = new Map<string, number>(keys.map((key) => [key, 0]));
    const dependents = new Map<string, string[]>();
    for (const key of keys) {
      for (const dependency of this.dependsOn.get(key) ?? []) {
        if (!indegree.has(dependency)) continue; // an unregistered dependency is a registration-time error, not this pass's concern
        indegree.set(key, (indegree.get(key) ?? 0) + 1);
        const list = dependents.get(dependency) ?? [];
        list.push(key);
        dependents.set(dependency, list);
      }
    }
    const order: string[] = [];
    const remaining = new Map(indegree);
    const queue = keys.filter((key) => (indegree.get(key) ?? 0) === 0);
    while (queue.length > 0) {
      queue.sort();
      const key = queue.shift()!;
      order.push(key);
      for (const dependent of dependents.get(key) ?? []) {
        const next = (remaining.get(dependent) ?? 0) - 1;
        remaining.set(dependent, next);
        if (next === 0) queue.push(dependent);
      }
    }
    if (order.length !== keys.length) {
      const leftover = keys.filter((key) => !order.includes(key)).sort();
      throw new Error(`[DEBUG] ArtifactInferenceRouter.dependencyOrder: cycle among ${leftover.join(", ")}`);
    }
    return order;
  }
}
//#endregion 🔖️ArtifactRouters
// #endregion 🎠️Kernel


//#region 🧪️ExpandPluginRegistryTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("expandPluginRegistry", () => {
    it("includes transitive dependsOn of primary and consume-matched contributors", () => {
      const plugins = [
        { pluginId: "host-plugin", moduleUrl: "a", consumes: ["ext.tag"], dependencies: [{ pluginId: "core", version: "*" }] },
        { pluginId: "core", moduleUrl: "b", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "stdio", moduleUrl: "c" },
        { pluginId: "ext", moduleUrl: "d", contributes: ["ext.tag"], dependencies: [{ pluginId: "flow", version: "*" }] },
        { pluginId: "flow", moduleUrl: "e", dependencies: [{ pluginId: "stdio", version: "*" }] },
        { pluginId: "unrelated", moduleUrl: "f" },
      ] as const;
      const expanded = expandPluginRegistry(plugins, "host-plugin", false);
      const ids = new Set(expanded.map((entry) => entry.pluginId));
      expect(ids.has("host-plugin")).toBe(true);
      expect(ids.has("core")).toBe(true);
      expect(ids.has("stdio")).toBe(true);
      expect(ids.has("ext")).toBe(true);
      expect(ids.has("flow")).toBe(true);
      expect(ids.has("unrelated")).toBe(false);
    });
  });
}
//#endregion 🧪️ExpandPluginRegistryTests

//#region 🧪️IoRouterTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("IoEntryGraph", () => {
    // 🧭️ SAME fixture as the Rust twin (`💻️os/🔌️plugin/🖥️host/🦀️.rs`,
    // `io_router_w1d_fixture_entries`) and `🧪️w1d-io-router-parity.ts` — `stdio` owns one Exact
    // hop, `gif` owns a Canonical migration hop AND a competing Lossy direct shortcut.
    const binaryRaw: ArtifactDialect = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
    const gif87a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" };
    const gif89a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "89a", subset: "*" };
    const fixturePlugins: IoEntryGraphPlugin[] = [
      { pluginId: "stdio", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true }] },
      {
        pluginId: "gif",
        entries: [
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
          { from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true },
        ],
      },
    ];

    it("resolves the highest-minimum-fidelity route regardless of registration order", () => {
      const forward = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a);
      const reversed = IoEntryGraph.build([...fixturePlugins].reverse()).route(binaryRaw, gif89a);
      expect(forward).toEqual(reversed);
      expect(forward).toEqual({
        hops: [
          { from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true },
          { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
        ],
        fidelity: "Canonical",
      });
    });

    it("respects maxHops, picking the direct (weaker) shortcut when bounded to 1", () => {
      const route = IoEntryGraph.build(fixturePlugins).route(binaryRaw, gif89a, 1);
      expect(route).toEqual({ hops: [{ from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }], fidelity: "Lossy" });
    });

    it("rejects a different plugin claiming an already-owned (from,into) key", () => {
      expect(() => IoEntryGraph.build([...fixturePlugins, { pluginId: "intruder", entries: [{ from: binaryRaw, into: gif87a, fidelity: "Lossy", sniffs: false }] }])).toThrow(/conflict/);
    });

    it("ownerOf reports the registering plugin", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      expect(graph.ownerOf(binaryRaw, gif87a)).toBe("stdio");
      expect(graph.ownerOf(gif87a, gif89a)).toBe("gif");
      expect(graph.ownerOf(gif89a, binaryRaw)).toBeUndefined();
    });

    it("carrierEntries returns only the sniff-declaring hops whose from is the given carrier", () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const entries = graph.carrierEntries(binaryRaw);
      expect(entries.map((entry) => ({ into: entry.into, pluginId: entry.pluginId }))).toEqual([
        { into: gif87a, pluginId: "stdio" },
        { into: gif89a, pluginId: "gif" },
      ]);
    });

    it("ioRun executes the whole route hop by hop, feeding each hop's output to the next", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const calls: string[] = [];
      const result = await ioRun(graph, "norm", binaryRaw, gif89a, new Uint8Array([1]), (pluginId, from, into, payload) => {
        calls.push(`${pluginId}:${dialectCoordinate(from)}->${dialectCoordinate(into)}`);
        return new Uint8Array([...payload, payload.length]);
      });
      expect(calls).toEqual(["stdio:s.stdio.binary@raw/*->s.stdio.gif@87a/*", "gif:s.stdio.gif@87a/*->s.stdio.gif@89a/*"]);
      expect(Array.from(result)).toEqual([1, 1, 2]);
    });

    it("ioRun refuses the WHOLE route (no partial execution) when the calling plugin owns any hop", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      let ran = false;
      await expect(
        ioRun(graph, "gif", binaryRaw, gif89a, new Uint8Array(), () => {
          ran = true;
          return new Uint8Array();
        }),
      ).rejects.toThrow(/refused/);
      expect(ran).toBe(false);
    });

    it("ioIdentify fans sniffHop out across carrier entries, skipping the calling plugin's own, sorted by confidence then coordinate", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "norm", binaryRaw, new Uint8Array(), (pluginId) => (pluginId === "stdio" ? 3 : 1));
      expect(results).toEqual([
        [gif87a, "High"],
        [gif89a, "Low"],
      ]);
    });

    it("ioIdentify skips the calling plugin's own carrier entries", async () => {
      const graph = IoEntryGraph.build(fixturePlugins);
      const results = await ioIdentify(graph, "stdio", binaryRaw, new Uint8Array(), () => 3);
      expect(results).toEqual([[gif89a, "High"]]);
    });
  });
}
//#endregion 🧪️IoRouterTests
