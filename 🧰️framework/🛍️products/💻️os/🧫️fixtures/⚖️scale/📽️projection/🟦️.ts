/** 🧩️ Semantic scale fixture projection owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();



//#endregion 📤️DistributionBundle

//#region 🔖️ScaleFixture
/** 🧫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME F1-scale-fixture (design-workforce.md §3): seeded,
 * deterministic 50-plugin x 50-extension synthetic registry generator — 2550 records (50 x (1 +
 * 50)), proving the whole ticket's scale claim. `💻️os/🧫️fixtures/⚖️scale/` sits outside
 * `taxonomy.pluginAreas` on purpose (verified against `🔣️taxonomy.json`'s `pluginAreas`/
 * `rootDataDirNames` before this region was written), so this generator is entirely independent of
 * `📇️registry:generate`'s production catalog — same freshness (`generate` writes, `check` byte-
 * compares and never writes) idiom, separate data, separate files.
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-workforce.md
 */
const SCALE_FIXTURE_OWNER_REL = "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale";

const SCALE_FIXTURE_PROFILES = ["idle", "cpu", "ui", "io", "hang", "crash", "stateful"] as const;

type ScaleFixtureProfile = (typeof SCALE_FIXTURE_PROFILES)[number];

type ScaleFixtureActivationEvent = { type: "on-startup-finished" } | { type: "on-command"; id: string } | { type: "on-artifact-kind"; kind: string } | { type: "on-view-visible"; id: string };

/** 📋️ Exact shape the `semio-framework-os-scale-fixture` crate's `FixtureConfig` (`🎭️profile/
 * 🦀️.rs`) decodes from `instance-open`'s `config` pack — field names match its `serde`
 * `camelCase` rename 1:1 so a real host can hand this object, JSON-encoded, straight through. */
type ScaleFixtureConfig = {
  profile: ScaleFixtureProfile;
  cpuBusyMs: number;
  uiPatchesPerTurn: number;
  hangOverrunMultiplier: number;
  crashAfterTurns: number;
  ioCapabilityId: string;
};

type ScaleFixtureRecord = {
  id: string;
  kind: "plugin" | "extension";
  parentId: string | null;
  activationEvents: ScaleFixtureActivationEvent[];
  quotas: { deadlineMs: number; maxEffects: number; maxPatchBytes: number; maxFrames: number };
  capabilities: string[];
  scaleFixture: ScaleFixtureConfig;
};

type ScaleFixtureRegistry = {
  seed: number;
  pluginCount: number;
  extensionsPerPlugin: number;
  recordCount: number;
  records: ScaleFixtureRecord[];
};

type ScaleFixtureCatalogEntry = {
  pluginId: string;
  profile: ScaleFixtureProfile;
  extensionIds: string[];
  extensionProfileCounts: Record<ScaleFixtureProfile, number>;
};

type ScaleFixtureCatalog = {
  seed: number;
  pluginCount: number;
  extensionsPerPlugin: number;
  totalRecordCount: number;
  profileTotals: Record<ScaleFixtureProfile, number>;
  plugins: ScaleFixtureCatalogEntry[];
};

/** 🎲️ mulberry32 — tiny seedable PRNG, deterministic across platforms/runs (no `crypto`, no
 * `Date.now()`, no engine-dependent `Math.random()`): the whole determinism proof rests on this
 * being the ONLY source of variation in `buildScaleFixtureRegistry`. */
function scaleFixtureRng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function scaleFixturePick<T>(rng: () => number, options: readonly T[]): T {
  return options[Math.floor(rng() * options.length) % options.length]!;
}

/** 🚀️ ≈5% `on-startup-finished`, the rest split across `on-command`/`on-artifact-kind`/
 * `on-view-visible` (design-workforce.md §3's exact split). */
function scaleFixtureActivationEvent(rng: () => number, index: number): ScaleFixtureActivationEvent {
  if (rng() < 0.05) return { type: "on-startup-finished" };
  const kind = scaleFixturePick(rng, ["on-command", "on-artifact-kind", "on-view-visible"] as const);
  if (kind === "on-command") return { type: "on-command", id: `scale-fixture.command-${index}` };
  if (kind === "on-artifact-kind") return { type: "on-artifact-kind", kind: `scale-fixture.artifact-${index}` };
  return { type: "on-view-visible", id: `scale-fixture.view-${index}` };
}

function scaleFixtureRecordConfig(rng: () => number, id: string): ScaleFixtureConfig {
  return {
    profile: scaleFixturePick(rng, SCALE_FIXTURE_PROFILES),
    cpuBusyMs: 2 + Math.floor(rng() * 8),
    uiPatchesPerTurn: 1 + Math.floor(rng() * 4),
    hangOverrunMultiplier: 2 + Math.floor(rng() * 3),
    crashAfterTurns: 1 + Math.floor(rng() * 3),
    ioCapabilityId: `${id}.io`,
  };
}

function scaleFixtureRecord(rng: () => number, id: string, kind: "plugin" | "extension", parentId: string | null, index: number): ScaleFixtureRecord {
  const scaleFixture = scaleFixtureRecordConfig(rng, id);
  return {
    id,
    kind,
    parentId,
    activationEvents: [scaleFixtureActivationEvent(rng, index)],
    quotas: {
      deadlineMs: 8 + Math.floor(rng() * 24),
      maxEffects: 4 + Math.floor(rng() * 12),
      maxPatchBytes: 1024 + Math.floor(rng() * 7168),
      maxFrames: 1 + Math.floor(rng() * 4),
    },
    capabilities: scaleFixture.profile === "io" ? [scaleFixture.ioCapabilityId] : [],
    scaleFixture,
  };
}

/** 🏗️ Builds the full `pluginCount x (1 + extensionsPerPlugin)`-record registry deterministically
 * from `seed` — fixed iteration order (plugin 0..N, that plugin's own record, then its M
 * extensions 0..M) so two calls with the same `(pluginCount, extensionsPerPlugin, seed)` are
 * byte-identical once serialized (proven by `ScaleFixtureCheckScript`/this packet's acceptance
 * run, not merely asserted). */
function buildScaleFixtureRegistry(pluginCount: number, extensionsPerPlugin: number, seed: number): ScaleFixtureRegistry {
  const rng = scaleFixtureRng(seed);
  const records: ScaleFixtureRecord[] = [];
  for (let p = 0; p < pluginCount; p++) {
    const pluginId = `scale-fixture-plugin-${String(p).padStart(4, "0")}`;
    records.push(scaleFixtureRecord(rng, pluginId, "plugin", null, p));
    for (let e = 0; e < extensionsPerPlugin; e++) {
      const extensionId = `${pluginId}-ext-${String(e).padStart(4, "0")}`;
      records.push(scaleFixtureRecord(rng, extensionId, "extension", pluginId, p * extensionsPerPlugin + e));
    }
  }
  return { seed, pluginCount, extensionsPerPlugin, recordCount: records.length, records };
}

function scaleFixtureZeroProfileCounts(): Record<ScaleFixtureProfile, number> {
  return Object.fromEntries(SCALE_FIXTURE_PROFILES.map((profile) => [profile, 0])) as Record<ScaleFixtureProfile, number>;
}

/** 📚️ Per-plugin rollup (id, own profile, its extensions and THEIR profile distribution) plus a
 * fleet-wide profile total — the "playground catalog" analog `plugin-registry`'s own
 * `🤖️generated/🗂️catalog/🔣️.json` plays for this fixture registry, derived here rather than
 * hand-duplicated. */
function buildScaleFixtureCatalog(registry: ScaleFixtureRegistry): ScaleFixtureCatalog {
  const profileTotals = scaleFixtureZeroProfileCounts();
  const plugins: ScaleFixtureCatalogEntry[] = [];
  const byId = new Map<string, ScaleFixtureCatalogEntry>();
  for (const record of registry.records) {
    profileTotals[record.scaleFixture.profile]++;
    if (record.kind === "plugin") {
      const entry: ScaleFixtureCatalogEntry = { pluginId: record.id, profile: record.scaleFixture.profile, extensionIds: [], extensionProfileCounts: scaleFixtureZeroProfileCounts() };
      plugins.push(entry);
      byId.set(record.id, entry);
    }
  }
  for (const record of registry.records) {
    if (record.kind === "extension" && record.parentId) {
      const entry = byId.get(record.parentId);
      if (!entry) continue;
      entry.extensionIds.push(record.id);
      entry.extensionProfileCounts[record.scaleFixture.profile]++;
    }
  }
  return { seed: registry.seed, pluginCount: registry.pluginCount, extensionsPerPlugin: registry.extensionsPerPlugin, totalRecordCount: registry.recordCount, profileTotals, plugins };
}

function scaleFixtureGeneratedDir(repoRoot: string): string {
  return join(repoRoot, SCALE_FIXTURE_OWNER_REL, "🤖️generated");
}

function renderScaleFixtureArtifacts(pluginCount: number, extensionsPerPlugin: number, seed: number): { registryJson: string; catalogJson: string; registry: ScaleFixtureRegistry } {
  const registry = buildScaleFixtureRegistry(pluginCount, extensionsPerPlugin, seed);
  const catalog = buildScaleFixtureCatalog(registry);
  return { registryJson: `${JSON.stringify(registry, null, 2)}\n`, catalogJson: `${JSON.stringify(catalog, null, 2)}\n`, registry };
}

function writeScaleFixtureArtifacts(repoRoot: string, pluginCount: number, extensionsPerPlugin: number, seed: number): ScaleFixtureRegistry {
  const dir = scaleFixtureGeneratedDir(repoRoot);
  mkdirSync(join(dir, "📇️registry"), { recursive: true });
  mkdirSync(join(dir, "🗂️catalog"), { recursive: true });
  const { registryJson, catalogJson, registry } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, seed);
  const expected = new Set(["📇️registry", "🗂️catalog"]);
  for (const name of readdirSync(dir)) if (!expected.has(name)) throw new Error(`Unexpected scale-fixture output requires explicit review: ${join(dir, name)}`);
  writeFileSync(join(dir, "📇️registry/🔣️.json"), registryJson);
  writeFileSync(join(dir, "🗂️catalog/🔣️.json"), catalogJson);
  console.log(`scale-fixture generate: ${registry.recordCount} records (plugins=${pluginCount} extensions=${extensionsPerPlugin} seed=${seed}) -> ${dir}`);
  return registry;
}

/** ✅️ `plugin-registry:check`'s exact idiom: re-derive from the on-disk registry's own
 * `(pluginCount, extensionsPerPlugin, seed)` and byte-compare — never writes. */
function checkScaleFixtureArtifacts(repoRoot: string): boolean {
  const dir = scaleFixtureGeneratedDir(repoRoot);
  const registryPath = join(dir, "📇️registry/🔣️.json");
  const catalogPath = join(dir, "🗂️catalog/🔣️.json");
  if (!existsSync(registryPath) || !existsSync(catalogPath)) return false;
  const onDiskRegistryJson = readFileSync(registryPath, "utf8");
  const onDiskCatalogJson = readFileSync(catalogPath, "utf8");
  let parsed: ScaleFixtureRegistry;
  try {
    parsed = JSON.parse(onDiskRegistryJson) as ScaleFixtureRegistry;
  } catch {
    return false;
  }
  const { registryJson, catalogJson } = renderScaleFixtureArtifacts(parsed.pluginCount, parsed.extensionsPerPlugin, parsed.seed);
  return registryJson === onDiskRegistryJson && catalogJson === onDiskCatalogJson;
}

function scaleFixtureFlag(segments: readonly string[], name: string, fallback: number): number {
  for (let i = 0; i < segments.length; i++) {
    const seg = segments[i]!;
    if (seg === `--${name}` && segments[i + 1] !== undefined) return Number(segments[i + 1]);
    if (seg.startsWith(`--${name}=`)) return Number(seg.slice(name.length + 3));
  }
  return fallback;
}

export { SCALE_FIXTURE_OWNER_REL, SCALE_FIXTURE_PROFILES, ScaleFixtureActivationEvent, ScaleFixtureCatalog, ScaleFixtureCatalogEntry, ScaleFixtureConfig, ScaleFixtureProfile, ScaleFixtureRecord, ScaleFixtureRegistry, buildScaleFixtureCatalog, buildScaleFixtureRegistry, checkScaleFixtureArtifacts, renderScaleFixtureArtifacts, scaleFixtureActivationEvent, scaleFixtureFlag, scaleFixtureGeneratedDir, scaleFixturePick, scaleFixtureRecord, scaleFixtureRecordConfig, scaleFixtureRng, scaleFixtureZeroProfileCounts, writeScaleFixtureArtifacts };
