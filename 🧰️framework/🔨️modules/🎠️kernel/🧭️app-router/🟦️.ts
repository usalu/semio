/** 🧭️ The app router: which app opens an artifact dialect in a role, from the manifests of the loaded plugins, with its surface fault codes.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import { dialectCoordinate } from "../../🚪️io/🧬️schema/🟦️.ts";
import { type AppRole, type AppRef } from "@semio-tech/framework/manifest/schema";
import { type PluginDependency } from "../🗂️plugin-registry/🟦️.ts";
import { type Fault, type FaultScope } from "../🚨️fault/🟦️.ts";
import { resolvePluginLoadOrder } from "../🕸️plugin-graph/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️AppRouter
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

export function surfaceFault(code: string, message: string, scope: FaultScope = {}): Fault {
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
 * `AppRef` or an unauthorized cross-plugin contribution excludes THAT plugin — all of its surfaces,
 * never a partial registration — and records one typed {@link Fault} under its plugin id
 * ({@link AppRouter.pluginFaults}); every other plugin still routes. An `AppRouter` therefore never
 * exists in an invalid state and one malformed manifest can no longer make every app unroutable
 * (ticket 26/09/05/S-END-TO-END lane H — `demonstrator` shipped surfaces for `s.cad.cad@1/*` with no
 * declared `cad` dependency and took the whole shell's routing down with it).
 */
export class AppRouter {
  private readonly entriesByCoordinateRole: ReadonlyMap<string, readonly AppRef[]>;
  private readonly dialectsByCoordinate: ReadonlyMap<string, ArtifactDialect>;
  private readonly ownerByArtifactKind: ReadonlyMap<string, string>;
  private readonly faultByPluginId: ReadonlyMap<string, Fault>;

  private constructor(entriesByCoordinateRole: ReadonlyMap<string, readonly AppRef[]>, dialectsByCoordinate: ReadonlyMap<string, ArtifactDialect>, ownerByArtifactKind: ReadonlyMap<string, string>, faultByPluginId: ReadonlyMap<string, Fault>) {
    this.entriesByCoordinateRole = entriesByCoordinateRole;
    this.dialectsByCoordinate = dialectsByCoordinate;
    this.ownerByArtifactKind = ownerByArtifactKind;
    this.faultByPluginId = faultByPluginId;
  }

  /** 🏗️ Builds the router — total, never throws. Loaded manifests are ordered dependency-first
   * before ownership is claimed, matching the Rust host's resolved plugin load order even when
   * browser workers finish nondeterministically. Dependencies not loaded yet are ignored for this
   * transient rebuild; the router is rebuilt again as each plugin arrives. Within that order, first
   * its own `artifactKinds` claim any still-unclaimed kind, then each app claims its dialect's
   * `artifactKind` if still unclaimed. A manifest breaching `"surface.contribution-not-permitted"`
   * (checked first) or `"surface.conflict"` (checked second — same order as Rust
   * `register_manifest`) contributes NO surface at all and one fault under its plugin id instead;
   * its `artifactKinds` ownership claims survive, exactly like Rust `unregister_plugin`, so a later
   * contributor never silently inherits an excluded plugin's kind. Vectors:
   * `🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json` (shared with the Rust twin). */
  static build(manifests: readonly AppRouterManifest[]): AppRouter {
    const ownerByArtifactKind = new Map<string, string>();
    const seenRefs = new Set<string>();
    const grouped = new Map<string, { readonly dialect: ArtifactDialect; readonly role: AppRole; readonly entries: AppRef[] }>();
    const faultByPluginId = new Map<string, Fault>();
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

      const staged: { readonly key: string; readonly dialect: ArtifactDialect; readonly role: AppRole; readonly ref: AppRef; readonly refKey: string }[] = [];
      const stagedRefKeys = new Set<string>();
      let fault: Fault | undefined;
      for (const raw of manifest.apps) {
        const surface = readManifestAppSurface(raw);
        if (!surface) continue;

        let owner = ownerByArtifactKind.get(surface.dialect.artifactKind);
        if (owner === undefined) {
          owner = manifest.pluginId;
          ownerByArtifactKind.set(surface.dialect.artifactKind, owner);
        }
        if (owner !== manifest.pluginId && !(manifest.dependencies ?? []).some((dependency) => dependency.pluginId === owner)) {
          fault = surfaceFault(
            SURFACE_FAULT_CODES.ContributionNotPermitted,
            `plugin ${JSON.stringify(manifest.pluginId)} contributes a surface for ${JSON.stringify(dialectCoordinate(surface.dialect))} without depending on owner ${JSON.stringify(owner)}`,
            { pluginId: manifest.pluginId, appId: surface.appId },
          );
          break;
        }

        const ref: AppRef = { pluginId: manifest.pluginId, appId: surface.appId };
        const refKey = `${ref.pluginId} ${ref.appId}`;
        if (seenRefs.has(refKey) || stagedRefKeys.has(refKey)) {
          fault = surfaceFault(SURFACE_FAULT_CODES.Conflict, `AppRef {pluginId: ${JSON.stringify(ref.pluginId)}, appId: ${JSON.stringify(ref.appId)}} registered twice`, { pluginId: ref.pluginId, appId: ref.appId });
          break;
        }
        stagedRefKeys.add(refKey);
        staged.push({ key: coordinateRoleKey(surface.dialect, surface.role), dialect: surface.dialect, role: surface.role, ref, refKey });
      }
      if (fault) {
        faultByPluginId.set(manifest.pluginId, fault);
        continue;
      }
      for (const entry of staged) {
        seenRefs.add(entry.refKey);
        let group = grouped.get(entry.key);
        if (!group) {
          group = { dialect: entry.dialect, role: entry.role, entries: [] };
          grouped.set(entry.key, group);
        }
        group.entries.push(entry.ref);
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
    return new AppRouter(entriesByCoordinateRole, dialectsByCoordinate, ownerByArtifactKind, faultByPluginId);
  }

  /** 🧯️ Every excluded plugin's fault, sorted by plugin id — mirrors Rust `AppRouter::plugin_faults`.
   * Empty for a clean catalogue; a shell surfaces these on the plugin's own status so an excluded
   * plugin is never silently dropped. */
  pluginFaults(): readonly Fault[] {
    return [...this.faultByPluginId.keys()].sort().map((pluginId) => this.faultByPluginId.get(pluginId)!);
  }

  /** 🧯️ The fault that excluded `pluginId` from this router, or `undefined` when it routes. */
  faultFor(pluginId: string): Fault | undefined {
    return this.faultByPluginId.get(pluginId);
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
//#endregion 🔖️AppRouter
// #endregion 🎠️Kernel
