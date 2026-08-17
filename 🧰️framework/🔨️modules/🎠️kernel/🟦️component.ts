// #region 🎠️Kernel
/// <reference types="vitest/importMeta" />
/** @emoji 🎠️ `@semio-tech/framework` — plugin runtime, leases, invocation responses, and playground boot. */
import type { IconName } from "@semio-tech/assets";
import type { ShellLocale, ShellTerminology, LocalizedLabel } from "../🛂️manifest/🤖️generated/🟦️ui-axes.ts";

import type {
  PluginManifest,
  PluginUiNode,
  PluginViewState,
  ProgramContributionEntry,
  WindowLayout,
  NamedLayout,
} from "../🛂️manifest/🟦️component.ts";
import type { StoragePort } from "../🖥️platform/🟦️component.ts";

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

export type PluginWasmHandle = {
  readonly manifest: () => Promise<Uint8Array>;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    for (const contribution of entry.manifest.contributions ?? []) {
      entries.push({ pluginId: entry.pluginId, contribution });
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

export async function resolveExternalSlots(node: PluginUiNode, context: ExternalSlotResolverContext): Promise<PluginUiNode> {
  if (node.type === "externalSlot") {
    const pluginId = String(node.pluginId ?? "");
    const appId = String(node.appId ?? pluginId);
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    // 🚧️ Rendering a contributor's UI body now goes through `AppChannelClient.refreshUi`
    // (`RefreshUi` → `UiSection` over `exchange`, os-product `🔖️AppChannelClient` region) instead
    // of the removed per-verb `render`/`renderWithDocument`. Wiring that dispatch loop into this
    // exact call site is the dedicated follow-up work package this ticket flags for the React
    // renderer's dispatch/refresh loops — until then an external slot degrades to unavailable
    // rather than silently guessing at `SectionProbe.kind`/body-key framing.
    return { type: "text", value: `Extension unavailable: ${pluginId}` };
  }
  if (node.type === "stack" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  if (node.type === "section" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  return node;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
  /** 🔗️ Direct plugin dependencies this entry's manifest declares — mirrors Rust
   * `PluginManifest.dependencies` (`🛂️manifest/🦀️component.rs`), ticket
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
 * (`🛂️manifest/🦀️component.rs`). */
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
 * (`🔨️modules/🚪️io/🦀️component.rs:50`, re-exported off `🛂️manifest/🦀️component.rs`). Duplicated
 * locally rather than imported from `🛂️manifest/🟦️component.ts`'s generated `AppDefinition` twin:
 * that twin's `apps` field is still `Record<string, unknown>[]` pending the ts-rs regen for
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
 * (`🔨️modules/🚪️io/🦀️component.rs:67`). */
export function dialectCoordinate(dialect: ArtifactDialect): string {
  return `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}`;
}

/** 🪪️ Inverse of {@link dialectCoordinate} — mirrors Rust `ArtifactDialect::parse_coordinate`
 * (`🔨️modules/🚪️io/🦀️component.rs:74`): `@` splits at its FIRST occurrence, the LAST `/` splits
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

/** 👁️✏️ Mirrors Rust `AppRole` (`🛂️manifest/🦀️component.rs:2641`) — exactly `"viewer"`/`"editor"`,
 * contract freeze §1 C1. Wire-identical to the `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE` env values. */
export type AppRole = "viewer" | "editor";

/** 🎯️ Mirrors Rust `AppRef` (`🛂️manifest/🦀️component.rs:2672`). */
export type AppRef = {
  readonly pluginId: string;
  readonly appId: string;
};

function appRefEquals(a: AppRef, b: AppRef): boolean {
  return a.pluginId === b.pluginId && a.appId === b.appId;
}

/** 🪪️ `<artifact_kind>@<standard>/<subset>#<role>` — mirrors Rust `surface_app_id`
 * (`🛂️manifest/🦀️component.rs:2678`). */
export function surfaceAppId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🪪️ Inverse of {@link surfaceAppId} — mirrors Rust `parse_surface_app_id`
 * (`🛂️manifest/🦀️component.rs:2683`): the LAST `#` splits off the role suffix. */
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
 * `💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs:149` — landed by lane 1-A). {@link FaultOrigin}
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
 * structurally compatible with) `🛂️manifest/🟦️component.ts`'s `PluginManifest`: `apps` stays
 * `Record<string, unknown>[]` there pending the C1 ts-rs regen, and `artifactKinds` (this
 * plugin's OWNED kinds, Rust `PluginManifest.artifact_kinds`, `🛂️manifest/🦀️component.rs:3218`)
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
 * `AppRouter`/`AppRouterState` — `💻️os/🔌️plugin/🖥️host/🦀️component.rs:1723-1857` — in
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

  /** 🏗️ Builds the router. Ownership claiming is a SINGLE pass over `manifests` in order,
   * interleaved per manifest — first its own `artifactKinds` claim any still-unclaimed kind, THEN
   * each of its apps claims its dialect's `artifactKind` if still unclaimed (first plugin to
   * touch a kind, whether by declaring it or merely by registering a surface for it, wins —
   * mirrors Rust `state.owners.entry(...).or_insert_with(...)`,
   * `💻️os/🔌️plugin/🖥️host/🦀️component.rs:1759-1763`). Throws {@link SemioFaultError} with
   * `"surface.contribution-not-permitted"` (checked first) or `"surface.conflict"` (checked
   * second) per app — same order as Rust `register_manifest` (`🦀️component.rs:1755-1785`). A
   * prior version of this method computed ownership from `artifactKinds` in one pass BEFORE any
   * app was processed, which silently diverged from Rust whenever a plugin claimed a kind only
   * implicitly, by being first to register a surface for it (Rust test
   * `step3_first_entry_when_the_owner_has_no_surface_for_this_role`, `🦀️component.rs:2101`) —
   * fixed here, see `📓️w1-d-report.md` parity table. */
  static build(manifests: readonly AppRouterManifest[]): AppRouter {
    const ownerByArtifactKind = new Map<string, string>();
    const seenRefs = new Set<string>();
    const grouped = new Map<string, { readonly dialect: ArtifactDialect; readonly role: AppRole; readonly entries: AppRef[] }>();
    // 🔢️ Ownership is claimed by whoever registers a kind FIRST (the single-pass rule above, kept for
    // Rust parity), which makes the result depend on the ORDER manifests arrive in. The Rust host
    // registers in a resolved, dependency-respecting order; the browser host collects manifests as ~55
    // plugin workers finish loading, i.e. in nondeterministic completion order. A contributor
    // extension could therefore land before the plugin that actually declares the kind and be recorded
    // as its owner, after which the real owner — or a sibling extension — was rejected as an
    // impermissible contributor and the whole router threw, leaving the shell with no surfaces at all.
    // Sorting declarers ahead of pure contributors restores the host-independent order without
    // changing the per-surface semantics.
    const ordered = [...manifests].sort((left, right) => Number((right.artifactKinds?.length ?? 0) > 0) - Number((left.artifactKinds?.length ?? 0) > 0));
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
   * mirrors Rust `AppRouter::owned_surface_gaps` (`🦀️component.rs:1836`) exactly: pure, total,
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
 * 🧭️ TS twin of the host `IoRouter`'s NEW io-mechanism graph (`💻️os/🔌️plugin/🖥️host/🦀️component.rs`,
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
 * `💻️os/🎚️config/🧬️schema/🟦️component.ts`. Duplicated (not imported) — a domain-neutral framework
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
 * (`💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:15`) — `#[serde(tag = "mutation",
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
 * shape-compatible so `🛍️products/💻️os/…/🟦️catalog.ts` can build one straight off the generated array
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
 * `🔌️plugin/📦️packages/🟦️typescript/🟦️catalog.ts` is the one place allowed to import the generated
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
 * (`🗣️dsl/⚠️diagnostic/🦀️component.rs`, `#[serde(rename_all = "camelCase")]`). Declaration order
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
 * (`💻️os/🔨️modules/🗣️dsl/⚠️diagnostic/🦀️component.rs:149`) — the origin for the five ticket
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
 * @emoji 🐚️ A typed side effect the shell performs on the app's behalf. Mirrors the Rust
 * `HostEffect` enum (externally tagged: unit variants are the plain tag string, struct variants are
 * a single-key object keyed by the camelCase variant name).
 */
export type HostEffect =
  | "requestSync"
  | { readonly openWindow: { readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** @emoji 📂️ Replaces the active app instance's document with a VCS envelope JSON — host-owned
   * counterpart of `loadAppArtifact` for catalog/example studio opens. */
  | { readonly loadArtifact: { readonly pack?: readonly number[]; readonly spr?: readonly number[]; readonly artifactJson?: string } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly multiple?: boolean } }
  /** @emoji 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
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
  | { readonly spawnPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly artifactJson?: string } }
  | { readonly openPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** @emoji 🔁️ Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a program
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly action: string; readonly args?: unknown; readonly delayMs: number } }
  /** @emoji 🎯️ Patches world-3d selection chrome and document-tree `selectedIds` without a composite re-render. */
  | {
      readonly patchWorld3dChrome: {
        readonly selectionJson: string;
        readonly vorticesJson?: string;
        readonly documentSelectedIds: readonly string[];
        readonly documentHighlightedIds?: readonly string[];
      };
    }
  | { readonly clipboardWrite: { readonly fragment: unknown } }
  | { readonly replayShellCommand: { readonly actionId: string; readonly args?: unknown } }
  | {
      readonly invokeExtension: {
        readonly extensionId: string;
        readonly capability: string;
        readonly requestJson: string;
        readonly responseAction: string;
      };
    };

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
  readonly requestedEffects?: readonly HostEffect[];
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
 * given {@link Severity} — TS twin of Rust `MergePolicy` (`📡️spr/🧾️wire/🦀️component.rs` region
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
 * Rust `ConflictResolution` (`📡️spr/⚔️conflict/🦀️component.rs`, `#[serde(rename_all =
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
 * Rust `MutationMessage` (`📡️spr/🎮️command/🦀️component.rs` region `🔖️Message`,
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

/** @emoji 🚫️ Schema mirror of Rust `MutationApplyError` (`📡️spr/🎮️command/🦀️component.rs`,
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
 * ApplyEnvelopes`/`AppFrame::DocumentChanged` use) — left as `unknown` here; no consumer needs a
 * typed shape for it yet. */
export type ConflictKind = { readonly kind: "quarantined"; readonly envelopes: readonly unknown[] } | { readonly kind: "degraded"; readonly edit_ids: readonly string[] };

/** @emoji 🚦️ A conflict's own lifecycle, independent of the `MutationMessage`s it carries — TS twin
 * of Rust `ConflictStatus` (`#[serde(rename_all = "camelCase")]`). */
export type ConflictStatus = "open" | "accepted" | "discarded";

/** @emoji ⚔️ One first-class conflict — TS twin of Rust `Conflict` (`📡️spr/⚔️conflict/
 * 🦀️component.rs`, `#[serde(rename_all = "camelCase")]`). `timestamp` mirrors
 * `HybridLogicalTimestamp` from `📡️spr/🆔️ids/🦀️component.rs` (a DIFFERENT shape than this file's
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

//#region SerializedPluginWasm
/** @emoji 🧾️ Flattens jco/component errors — message is often `[object Object] (see error.payload)` while the real text lives on `payload.val`. */
export function pluginErrorText(error: unknown): string {
  if (error instanceof Error) {
    const withPayload = error as Error & { payload?: unknown };
    const payload = withPayload.payload;
    if (payload && typeof payload === "object") {
      const record = payload as { val?: unknown; tag?: unknown; message?: unknown };
      if (typeof record.val === "string" && record.val.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
      if (typeof record.message === "string" && record.message.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
    }
    return withPayload.message;
  }
  if (error && typeof error === "object" && "payload" in error) {
    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }
  return String(error);
}

/** @emoji 🔒️ True when a plugin call hit the single-flight instance lock (or a poisoned guard after a trap). */
export function isPluginInstanceBusyError(error: unknown): boolean {
  const message = pluginErrorText(error);
  return message.includes("plugin instance busy") || message.includes("plugin busy");
}

/** @emoji 🔒️ Serializes wasm program entry points — the host keeps instances in one RefCell. */
export function withSerializedPluginWasmHandle(handle: PluginWasmHandle): PluginWasmHandle {
  let tail: Promise<void> = Promise.resolve();
  const runSerialized = <T>(fn: () => Promise<T>): Promise<T> => {
    const job = tail.then(async () => {
      for (let attempt = 0; attempt < 8; attempt += 1) {
        try {
          return await fn();
        } catch (error) {
          if (!isPluginInstanceBusyError(error)) throw error;
          await new Promise((resolve) => setTimeout(resolve, attempt + 1));
        }
      }
      return fn();
    });
    tail = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  };
  return {
    manifest: () => runSerialized(() => handle.manifest()),
    createApp: (appId) => runSerialized(() => handle.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => handle.destroyApp(instanceId)),
    exchange: (instanceId, frames) => runSerialized(() => handle.exchange(instanceId, frames)),
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

//#region PluginWorkerClient
/** @emoji 🧵️ Message types the generated `🟨️plugin-worker.js` dispatches (framework/os/dev/script.ts `pluginWorkerSource`). */
type PluginWorkerMessageType = "init" | "manifest" | "createApp" | "destroy" | "exchange" | "error";

/** @emoji ⏱️ Logs only, never kills the worker — a plugin action owns in-flight, possibly undo-relevant
 * state, so abandoning it mid-call (the wgpu renderer's timeout+restart policy) would corrupt it. */
const PLUGIN_WORKER_UNRESPONSIVE_MS = 10000;

/** @emoji 🔌️ Derives the generic worker bootstrap script's URL from a plugin module URL — same directory,
 * `🟨️plugin-worker.js` instead of the plugin's own bridge filename. The bootstrap script itself never
 * needs cache-busting (it's plugin-version-agnostic; the *actual* module URL, `?v=`-busted or not, only
 * ever travels as the `init` request's `moduleUrl` payload — see `start()` below), so any `?query` or
 * `#hash` on `moduleUrl` (from `PluginSource.moduleUrl`'s hot-reload cache-busting) is stripped first —
 * otherwise the trailing `.js` no longer sits at the string's end and the replace silently no-ops,
 * pointing the worker at the plugin's own module instead of its bootstrap script. */
/** @emoji 🪶️ GUESTSLIM: the typst default font set (see `infinite_canvas`'s `render` feature doc),
 * static-served alongside every plugin's own output at `_vendor/guestslim-typst-fonts.bin`
 * (`📇️registry/📜️script.ts`'s `ensureGuestSlimTypstFontsAsset`). Fetched once and reused across every
 * plugin worker this tab spins up — the file itself never changes at runtime (pinned crate version). */
const GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE = 1;
let guestSlimTypstFontsPromise: Promise<ArrayBuffer> | null = null;

/** @emoji 🛡️ Best-effort: most plugins never call `read-asset` at all, and the guest-side Rust already
 * degrades gracefully (empty font list → typst compile yields no glyphs → `BoardResolvedIcon::None`)
 * when no reader is registered — so a fetch hiccup here must never block a plugin worker from booting. */
async function guestSlimAssetsForModule(moduleUrl: string): Promise<ReadonlyArray<readonly [number, ArrayBuffer]>> {
  guestSlimTypstFontsPromise ??= (async () => {
    const vendorUrl = moduleUrl.split(/[?#]/)[0]!.replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
    const response = await fetch(vendorUrl);
    if (!response.ok) throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  })();
  try {
    const buffer = await guestSlimTypstFontsPromise;
    return [[GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE, buffer]];
  } catch (error) {
    console.warn("[DEBUG] GuestSlim typst fonts asset unavailable; affected plugins fall back to blank typst/emoji/text icons", error);
    guestSlimTypstFontsPromise = null;
    return [];
  }
}

export function pluginWorkerUrl(moduleUrl: string): string {
  const bare = moduleUrl.split(/[?#]/)[0]!;
  return bare.replace(/\/[^/]+\.js$/, "/🟨️plugin-worker.js");
}

/**
 * @emoji 🧵️ Runs a component-model plugin's WASM inside a Web Worker so `handleAction` — including
 * long-running precompute — never blocks the UI thread. Mirrors `framework/os/renderer/wgpu/js/🟦️boot.ts`'s
 * `PluginWorkerClient`, minus its 5s timeout+restart.
 */
class PluginWorkerClient {
  private worker: Worker | null = null;
  private readonly pending = new Map<string, { resolve: (value: Record<string, unknown>) => void; reject: (error: Error) => void; watchdog: number }>();
  onBackboneOutbound?: (uri: string, message: Uint8Array) => void;

  private readonly pluginId: string;
  private readonly moduleUrl: string;

  constructor(
    pluginId: string,
    moduleUrl: string,
  ) {
    this.pluginId = pluginId;
    this.moduleUrl = moduleUrl;
  }

  private clearPending(error: Error): void {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.watchdog);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }

  private attachWorker(worker: Worker): void {
    worker.onmessage = (event: MessageEvent) => {
      const message = event.data as {
        requestId?: string;
        type?: PluginWorkerMessageType | "backboneOutbound";
        uri?: string;
        message?: string;
      };
      if (message.type === "backboneOutbound" && message.uri && message.message != null) {
        const bytes = message.message instanceof Uint8Array ? message.message : new Uint8Array(message.message as ArrayBuffer);
        this.onBackboneOutbound?.(message.uri, bytes);
        return;
      }
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.watchdog);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `program worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] program worker ${this.pluginId} crashed`, error);
      this.worker = null;
      this.clearPending(new Error(`program worker ${this.pluginId} crashed`));
    };
  }

  async start(): Promise<void> {
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    // 🪶️ GUESTSLIM: structured-clone copy, not a transfer — `guestSlimAssetsForModule` caches and
    // reuses the same master `ArrayBuffer` across every plugin worker this tab starts; transferring
    // it would detach (neuter) it after the first worker, breaking every subsequent one.
    const guestSlimAssets = await guestSlimAssetsForModule(this.moduleUrl);
    await this.request("init", { moduleUrl: this.moduleUrl, guestSlimAssets });
  }

  private request(type: PluginWorkerMessageType, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`program worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const watchdog = window.setTimeout(() => {
        console.warn(`[DEBUG] program worker ${this.pluginId} unresponsive for ${PLUGIN_WORKER_UNRESPONSIVE_MS}ms: ${type}`);
      }, PLUGIN_WORKER_UNRESPONSIVE_MS);
      this.pending.set(requestId, { resolve, reject, watchdog });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }

  async manifest(): Promise<Uint8Array> {
    return ((await this.request("manifest", {})).value as Uint8Array | undefined) ?? new Uint8Array();
  }

  async createApp(appId: string): Promise<number> {
    return Number((await this.request("createApp", { appId })).instanceId);
  }

  async destroyApp(instanceId: number): Promise<void> {
    await this.request("destroy", { instanceId });
  }

  async exchange(instanceId: number, frames: Uint8Array[]): Promise<Uint8Array[]> {
    return ((await this.request("exchange", { instanceId, frames })).value as Uint8Array[] | undefined) ?? [];
  }

  dispose(): void {
    this.clearPending(new Error(`program worker ${this.pluginId} disposed`));
    this.worker?.terminate();
    this.worker = null;
  }

  postBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
    this.worker?.postMessage({ type: "backboneInbound", uri, messages });
  }
}

/**
 * @emoji 🧵️ Worker-backed `PluginWasmHandle` for component-model plugins (the ABI the generated
 * `🟨️plugin-worker.js` supports). Caller falls back to the direct main-thread import on failure (no
 * `🟨️plugin-worker.js` alongside this module, wasm-bindgen-only program, or `Worker` unavailable).
 *
 * Keyed by `moduleUrl` (not `pluginId`): a hot reload acquires a *second* worker at a fresh
 * cache-busted URL for the same `pluginId` while the old one still serves live instances, so a
 * `pluginId`-keyed map would have the new worker's `set()` silently clobber the old entry and then
 * the old worker's `dispose()` delete the new one out from under it. `activeWorkerByPluginId` tracks
 * which of a plugin's (possibly several, during a swap) worker clients is the one inbound backbone
 * traffic should reach.
 */
const pluginWorkerClients = new Map<string, PluginWorkerClient>();
const activeWorkerByPluginId = new Map<string, PluginWorkerClient>();

async function loadPluginModuleViaWorker(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  pluginWorkerClients.set(moduleUrl, client);
  client.onBackboneOutbound = (uri, message) => relayPluginBackboneOutbound(uri, message);
  await client.start();
  activeWorkerByPluginId.set(pluginId, client);
  console.log(`[DEBUG] plugin worker + ${pluginId} (${pluginWorkerClients.size} live)`);
  return withSerializedPluginWasmHandle({
    manifest: () => client.manifest(),
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    exchange: (instanceId, frames) => client.exchange(instanceId, frames),
    dispose: () => {
      if (pluginWorkerClients.get(moduleUrl) === client) pluginWorkerClients.delete(moduleUrl);
      if (activeWorkerByPluginId.get(pluginId) === client) activeWorkerByPluginId.delete(pluginId);
      client.dispose();
      console.log(`[DEBUG] plugin worker - ${pluginId} (${pluginWorkerClients.size} live)`);
    },
  });
}
//#endregion PluginWorkerClient

export function relayPluginBackboneOutbound(uri: string, message: Uint8Array): void {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}

/** @emoji 🌉️ A direct-import (main-thread, no-worker) plugin's generated `🟨️host-shim.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed program reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, message: Uint8Array) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** @emoji 🌉️ Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `🟨️host-shim.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, Uint8Array[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, Uint8Array[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly Uint8Array[]): void {
  const client = activeWorkerByPluginId.get(pluginId);
  if (client) {
    client.postBackboneInbound(uri, messages);
    return;
  }
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

//#region 🪶️LeasePool
/** @emoji 🪶️ One caller's reference to a {@link LeasePool}-managed resource. `release()` is idempotent —
 * a second call is a no-op — and drops this caller's refcount; the pool only disposes the underlying
 * resource once every issued lease on that key has released (and, unless `lingerMs` is 0, only after
 * the linger window below elapses with no re-acquire). */
export interface Lease<T> {
  readonly value: T;
  release(): void;
}

export interface LeasePoolStats {
  readonly key: string;
  readonly refs: number;
  readonly state: "loading" | "resident" | "lingering";
}

export interface LeasePool<T> {
  acquire(key: string): Promise<Lease<T>>;
  /** Forces disposal of `key` (or every entry when omitted) right now, bypassing any linger timer.
   * A no-op (logged, not thrown) for a key with active leases — evicting a resource a caller still
   * holds would leave that caller's `Lease.value` silently dead underneath it. */
  evictNow(key?: string): void;
  stats(): readonly LeasePoolStats[];
}

type LeasePoolEntry<T> = {
  readonly promise: Promise<T>;
  refs: number;
  lingerTimer: ReturnType<typeof setTimeout> | null;
  settled: T | undefined;
};

/**
 * @emoji 🪶️ Generic refcounted resource pool with linger-based eviction — the shared mechanism both
 * {@link acquirePluginModule} (plugin worker modules) and the renderer's engine-session cache build on
 * top of, instead of each hand-rolling its own refcounting. A resource loads once per `key` and is
 * shared by every caller; when the last lease on a key releases, the resource isn't disposed
 * immediately — it lingers for `lingerMs` (default 30s) so a caller that re-acquires the same key
 * shortly after (e.g. reopening a just-closed window) reuses the still-live resource instead of paying
 * full reload cost. `lingerMs: 0` disposes the instant refs hit zero, matching the pre-`LeasePool`
 * `acquirePluginModule` behavior exactly.
 */
export function createLeasePool<T>(load: (key: string) => Promise<T>, dispose: (value: T) => void, options?: { readonly lingerMs?: number; readonly label?: string }): LeasePool<T> {
  const lingerMs = options?.lingerMs ?? 30_000;
  const label = options?.label ?? "resource";
  const entries = new Map<string, LeasePoolEntry<T>>();

  function disposeEntry(key: string, entry: LeasePoolEntry<T>): void {
    if (entries.get(key) !== entry) return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }

  return {
    async acquire(key: string): Promise<Lease<T>> {
      let entry = entries.get(key);
      if (!entry) {
        const created: LeasePoolEntry<T> = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then(
          (value) => {
            created.settled = value;
          },
          () => {
            if (entries.get(key) === created) entries.delete(key);
          },
        );
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released) return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0) return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          },
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key?: string): void {
      for (const [entryKey, entry] of key ? ([[key, entries.get(key)]] as const) : entries) {
        if (!entry) continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped — ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null) clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats(): readonly LeasePoolStats[] {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident",
      }));
    },
  };
}
//#endregion 🪶️LeasePool

//#region 🐚️PluginModuleLease
export interface PluginModuleLease {
  readonly handle: PluginWasmHandle;
  /** Releases this caller's reference to the shared module — idempotent, a second call is a no-op.
   * The underlying worker/module disposes once every lease on this `moduleUrl` has released and the
   * pool's linger window (see {@link createLeasePool}) elapses with no re-acquire. */
  release(): void;
}

// 🐚️ The pool's `load` callback only receives the key (`moduleUrl` — already globally unique per
// plugin, matching the pre-pool cache's key exactly), but `loadPluginModuleUncached` also wants a
// human-readable `pluginId` for its worker/log labels. `acquirePluginModule` records that association
// here just before acquiring; safe as a plain overwrite since a given `moduleUrl` only ever maps to
// one `pluginId` in practice.
const pluginModuleIdByUrl = new Map<string, string>();
const pluginModulePool = createLeasePool<PluginWasmHandle>((moduleUrl) => loadPluginModuleUncached(pluginModuleIdByUrl.get(moduleUrl) ?? moduleUrl, moduleUrl), (handle) => handle.dispose(), { label: "plugin module" });

/**
 * @emoji 🐚️ Refcounted replacement for the old `loadPluginModule` — several shells (or several plugin
 * instances within one shell) loading the SAME `moduleUrl` share one worker/module, but each caller
 * gets its own {@link PluginModuleLease} and must `release()` it on unmount/teardown. Built on
 * {@link createLeasePool}: the shared module lingers briefly after the last lease releases (a shell
 * closed and immediately reopened reuses it) rather than disposing that instant — under the pre-pool
 * cache, a loaded module was in practice *never* disposed at all (its promise was cached forever with
 * nothing to evict it; `dispose()` was only ever reachable on load *failure*), so this is strictly a
 * bugfix on top of a lifecycle improvement.
 */
export async function acquirePluginModule(pluginId: string, moduleUrl: string): Promise<PluginModuleLease> {
  pluginModuleIdByUrl.set(moduleUrl, pluginId);
  const lease = await pluginModulePool.acquire(moduleUrl);
  return { handle: lease.value, release: lease.release };
}

/** @emoji 🔁️ Forces immediate disposal of a stale `moduleUrl` after a hot reload has released its last
 * lease — a no-op with a `[DEBUG]` warning (see {@link createLeasePool.evictNow}) if a caller still
 * holds the old lease, so a reload sequence must release before evicting. Skipping this after a
 * cache-busted reload would leave the old worker lingering for the pool's full 30s window per swap. */
export function evictPluginModule(moduleUrl: string): void {
  pluginModulePool.evictNow(moduleUrl);
}

/** @emoji 🔭️ Debug-only runtime snapshot — live plugin worker ids and the plugin module pool's lease
 * states — for verifying eager-boot-vs-lazy-residency changes from devtools without instrumenting call
 * sites by hand. Intentionally global rather than exported: this is a console/devtools aid, not API. */
(globalThis as unknown as { __semioPluginRuntimeStats?: () => unknown }).__semioPluginRuntimeStats = () => ({
  workerModuleUrls: Array.from(pluginWorkerClients.keys()),
  workerCount: pluginWorkerClients.size,
  activePluginIds: Array.from(activeWorkerByPluginId.keys()),
  modulePool: pluginModulePool.stats(),
});
//#endregion 🐚️PluginModuleLease

/**
 * 🌉️ Direct main-thread import fallback for {@link loadPluginModuleViaWorker} (no `Worker` global —
 * vitest/node — or no `🟨️plugin-worker.js` alongside this module). Only the component-model
 * `createPluginApi` ABI is supported: the pre-ABI-flip flat `semio_plugin_*` wasm-bindgen export
 * surface (one JS function per verb: `semio_plugin_handle_action`, `semio_plugin_render`, ...)
 * predates the binary `exchange` ABI entirely and has no equivalent under it, so it is dropped
 * rather than adapted — this is a greenfield codebase with no legacy-ABI support obligation.
 */
async function loadPluginModuleUncached(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  // 🧵️ Worker-backed by default so a plugin's `exchange` (e.g. puzzle-3d's collision precompute) can
  // never block the UI thread. Falls back to the direct main-thread import below when unavailable: no
  // `Worker` global (vitest/node) or no `🟨️plugin-worker.js` alongside this module.
  if (typeof Worker !== "undefined") {
    try {
      return await loadPluginModuleViaWorker(pluginId, moduleUrl);
    } catch (error) {
      console.warn(`[DEBUG] program ${pluginId} worker-backed load failed, falling back to main thread: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<Uint8Array>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
    }>;
  };
  if (module.default) await module.default();
  if (!module.createPluginApi) {
    throw new Error(`[DEBUG] program ${pluginId} missing createPluginApi export`);
  }
  const api = await module.createPluginApi();
  return withSerializedPluginWasmHandle({
    manifest: () => api.manifest(),
    createApp: (appId) => api.createApp(appId),
    destroyApp: async (instanceId) => {
      await api.destroyApp?.(instanceId);
    },
    exchange: (instanceId, frames) => api.exchange(instanceId, frames),
    dispose() {},
  });
}

/** 🌉️ Adapts a {@link PluginWasmHandle} to a plain-object shape safe to close over across a
 * `postMessage`/global-bridge boundary (see the wgpu renderer's own program-worker embedding) — a
 * pass-through now that the whole ABI is already binary (`manifest`/`exchange` bytes cross
 * structured clone natively, same as `Uint8Array` payloads elsewhere on this bridge). */
export function pluginHandleForBridge(handle: PluginWasmHandle) {
  return {
    manifest: () => handle.manifest(),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    exchange: (instanceId: number, frames: Uint8Array[]) => handle.exchange(instanceId, frames),
  };
}
//#endregion PluginRuntime

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
 * module, e.g. {@link loadPluginModuleUncached}'s `Worker` check). */
export function createDevPluginSource(registry: readonly PluginRegistryEntry[]): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
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
          listener(JSON.parse(event.data) as PluginSourceEvent);
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
 * (`🛂️manifest/🦀️component.rs`), narrowed to exactly what {@link resolvePluginLoadOrder}/
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
    // 🧭️ SAME fixture as the Rust twin (`💻️os/🔌️plugin/🖥️host/🦀️component.rs`,
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
