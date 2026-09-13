/** 🧩️ Semantic layering policy owner. */

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();



//#region 🔖️CapabilityLayeringLint
/** 🗺️ The four layering roles this lint enforces — a subset of `🔣️taxonomy.json`'s `roles` (which also
 * lists `product`/`hub`/`testkit`/`tool`, deliberately out of scope: this lint mirrors exactly the three
 * directions `.dependency-cruiser.cjs`'s `framework-no-s`/`s-modules-no-plugins`/`no-plugin-to-extension-*`
 * already enforce on the TS/JS import graph, not a broader Cargo policy). */
type LayeringRole = "framework" | "s-module" | "plugin" | "extension";

const LAYERING_ROLES = new Set<string>(["framework", "s-module", "plugin", "extension"]);

/** 🚧️ Grandfathered layering violations — real, evidence-backed, and deliberately accepted, NOT
 * pre-existing noise like `KNOWN_CAPABILITY_VIOLATIONS` above. This ticket's one populated entry is C2
 * from `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` (see `📓️w5b-c2-verdict.md`):
 * `semio-s-plugin-procedural` has 7 real Cargo dependencies on `🌊️flow`'s extension crates, and unlinking
 * them needs new runtime infrastructure that does not exist yet (a host-side extension registry wired
 * into a real boot path, guest-side component-extension wiring for all 7 crates, and a resolution for the
 * shared brep-kernel `GeometryHandle` coupling) — tracked as a dedicated follow-up ticket, not mechanical
 * cleanup. Do not add an entry here to silence a failure without the same standard of evidence — every
 * other hit this lint finds is a REAL new violation, not noise.
 *
 * The second entry is `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s: `semio-framework-os-renderer-wgpu`
 * (role `framework`) has a live Cargo dependency on `semio-s-plugin-puzzle` (role `plugin`) — declared at
 * `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:30`
 * (`puzzle = { path = "…", package = "semio-s-plugin-puzzle" }`). No `puzzle::` call site was found in the
 * wgpu renderer's own sources when this was triaged, so the edge reads as dead/vestigial rather than a
 * real runtime coupling — but this lint deliberately does not try to prove "unused" from source text (a
 * feature-gated or macro-expanded call site would false-negative that check), so it stays a real, accepted
 * exception, not a false positive silenced away. **The real fix is deleting the unused Cargo dependency
 * line from the wgpu renderer's `Cargo.toml`** — that file and `puzzle` are both outside this ticket's
 * boundary (puzzle is held by another concurrent session per `📌️important.md`'s cross-session protocol),
 * so APA does not touch either; this entry keeps the gate green until whoever owns that boundary removes
 * the dependency, at which point this entry should be deleted, not left stale. */
const KNOWN_LAYERING_VIOLATIONS = new Set<string>([
  ...["brep", "math", "primitive", "logic", "dictionary", "list", "text"].map((ext) => `semio-s-plugin-procedural: plugin->extension dependency on semio-s-plugin-flow-extension-${ext}`),
  "semio-framework-os-renderer-wgpu: framework->plugin dependency on semio-s-plugin-puzzle", // 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
]);

/** 🧱️ `[package.metadata.semio]`'s `role` key, read the same way `PluginCapabilityLintScript`'s own
 * `[package.metadata.semio]` regex above reads `capabilities` — a plain-text scrape, not a TOML parser
 * dependency, for one field. `🔣️taxonomy.json`'s `ecosystems.🦀️rust.marker` documents this table as the
 * SSOT for a crate's role (see `📓️w6-investigation.md`, which used this exact table to settle a real-vs-
 * optics layering question about `semio-framework-os-kernel`). */
function extractSemioRole(manifestText: string): LayeringRole | null {
  const role = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?\brole\s*=\s*"([^"]+)"/)?.[1];
  return role && LAYERING_ROLES.has(role) ? (role as LayeringRole) : null;
}

/** 🧱️ Cargo-metadata-driven counterpart to `.dependency-cruiser.cjs`'s `framework-no-s`/
 * `s-modules-no-plugins`/`no-plugin-to-extension-*` rules: those see only the TS/JS import graph (`compose
 * 🧰️framework ✏️s 🌎️hub ♻️mit-bestand`), so a real *Cargo* dependency edge violating the same three
 * directions (framework→{s-module,plugin,extension}, s-module→{plugin,extension}, plugin→extension) is
 * invisible to them — this is exactly how C2 (`🌀️procedural`→7 `🌊️flow` extension crates) went
 * undetected. Classifies every workspace crate by its own declared `[package.metadata.semio].role` (SSOT,
 * never a directory-path guess) and walks `cargo metadata`'s real dependency edges, `kind: null` (normal/
 * runtime) only — `dev`/`build` edges are test/build-time-only and not a real production coupling (mirrors
 * this file's own `dsl-fixture-sweep`-style "test-only harness, not a runtime violation" precedent). W7 of
 * `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`. Was deliberately NOT wired into `plugin lint`/the
 * root `verify gate` because a dry run surfaced one real, undocumented `framework->plugin` edge
 * (`semio-framework-os-renderer-wgpu` → `semio-s-plugin-puzzle`) nobody had yet evaluated. That finding is
 * now triaged and grandfathered in `KNOWN_LAYERING_VIOLATIONS` above (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`),
 * so this now runs as part of `plugin lint` (see the `"plugin"`/`"lint"` router entry below) — the same
 * gate `semio-framework-os-dev:plugin lint` already gets invoked by from the repo-root `verify gate`
 * (`📜️script.ts`), so wiring happens here rather than by touching that file. Still directly runnable
 * standalone too: `bun ./📜️script.ts layer-lint` from this package, or
 * `bun nx run @semio-tech/framework-os-dev:layer-lint`. */
class CapabilityLayeringLintScript extends BundleScript {
  async run(): Promise<void> {
    const metadataResult = runProbe("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot, budgetMs: buildBudgetMs() });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout || "{}") as {
      packages: Array<{ name: string; manifest_path: string; dependencies: Array<{ name: string; kind: string | null }> }>;
    };
    const roleByName = new Map<string, LayeringRole>();
    for (const pkg of metadata.packages) {
      const role = extractSemioRole(await Bun.file(pkg.manifest_path).text());
      if (role) roleByName.set(pkg.name, role);
    }
    const forbiddenTargets: Record<LayeringRole, LayeringRole[]> = {
      framework: ["s-module", "plugin", "extension"],
      "s-module": ["plugin", "extension"],
      plugin: ["extension"],
      extension: [],
    };
    const failures: string[] = [];
    let checkedEdgeCount = 0;
    for (const pkg of metadata.packages) {
      const fromRole = roleByName.get(pkg.name);
      if (!fromRole) continue;
      for (const dep of pkg.dependencies) {
        if (dep.kind !== null) continue; // 🕵️ dev/build deps are not a real runtime coupling
        const toRole = roleByName.get(dep.name);
        if (!toRole || dep.name === pkg.name) continue;
        checkedEdgeCount++;
        if (forbiddenTargets[fromRole].includes(toRole)) {
          failures.push(`${pkg.name}: ${fromRole}->${toRole} dependency on ${dep.name}`);
        }
      }
    }
    const grandfathered = failures.filter((f) => KNOWN_LAYERING_VIOLATIONS.has(f));
    const blocking = failures.filter((f) => !KNOWN_LAYERING_VIOLATIONS.has(f));
    for (const warning of grandfathered) console.warn(`[capability-layering-lint] WARN (grandfathered C2, see 📓️w5b-c2-verdict.md): ${warning}`);
    if (blocking.length > 0) {
      for (const failure of blocking) console.error(`[capability-layering-lint] ${failure}`);
      throw new Error(`capability layering lint failed (${blocking.length} issue(s), ${checkedEdgeCount} cross-role edge(s) evaluated)`);
    }
    console.log(`capability layering lint passed (${checkedEdgeCount} cross-role edge(s) evaluated, ${grandfathered.length} grandfathered warning(s))`);
  }
}

export { CapabilityLayeringLintScript, KNOWN_LAYERING_VIOLATIONS, LAYERING_ROLES, LayeringRole, extractSemioRole };
