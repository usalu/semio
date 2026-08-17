import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { NEO4J_GRAPH_DATABASE_NAMES, getAllNeo4jGraphExportSpecs, joinNeo4jGraphDatabaseName, parseExtraNeo4jGraphDatabaseNamesFromEnv, partitionNeo4jGraphCliArgv, policyCanonicalArtifactKindBreaches, policyChildSlotKindDagBreaches, policyDissolvedKindRedefinitionBreaches, policyEmojiPrefixBreaches, policyModeCompletenessBreaches, policyPluginDependencyParityBreaches, policyWindowCompletenessBreaches } from "../../../../../../../📜️script.ts";
import { BundleScript, ScriptRouter, DAEMON_BUDGET_MS, ORCHESTRATOR_BUDGET_MS, budgetTimeoutHint, canReuseDevPort, daemonBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, dispatchSubcommand, findRepoRoot, gitSpawnEnv, goLevelTestArgs, isDevPortInUse, orchestratorBudgetMs, orchestratorBudgetOpts, resolveCargoPackageName, resolveCargoPackageNames, resolveDevPort, runCmd, runCmdStatus, runProbe, testLevelBudgetMs, vitestLevelArgs, wgpuDevPlayUrl } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { defineLint, type FileLinter } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, isAdapterBoundaryFile, parseTsImportSpecs } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import {
  PLAYGROUND_PORTS,
  PLAYGROUND_LOCKED_EXAMPLE_ENV,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  frameworkOsPlaygroundDevEnv,
  resolveFrameworkOsPlaygroundPlugin,
  loadFrameworkOsPlaygroundCatalog,
  playgroundPlayViteDefine,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { playgroundStaticSiteBuildOptions } from "../../../../../../🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts";
import { areaOf, clearDiscoveryCache, discoverBurndown, discoverOwners, discoverPackageProblems, discoverPackages, getWorkspaceRoot, loadTaxonomy, readSemioMarker, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { artifactFacetPathIsDeclared, buildSemanticCensus, renderSemanticCensusJson, resolveRustPathAttributes, type Taxonomy } from "../../🔍️discovery/🟦️component.ts";
import { computeWorkspaces, diffWorkspaces } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { mkdirSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";

//#region 🧪️EmojiPrefixPolicy
describe("emoji-prefix policy", () => {
  test("requires prefixes on renamable files and directories but exempts ecosystem filenames", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "plain-dir"), { recursive: true });
      writeFileSync(join(owner, "plain.ts"), "");
      writeFileSync(join(owner, "Cargo.toml"), "");
      const scopes = policyEmojiPrefixBreaches(root).map((breach) => breach.scope);
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain-dir");
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain.ts");
      expect(scopes).not.toContain("✏️s/🔌️plugins/🧪️probe/Cargo.toml");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate sibling emoji identities after VS16 normalization", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(owner, { recursive: true });
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      mkdirSync(join(owner, "🧭️second"), { recursive: true });
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind === "taxonomy/emoji-prefix-uniqueness")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate identities shared by sibling files and directories", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "📄️folder"), { recursive: true });
      writeFileSync(join(owner, "📄️file.ts"), "");
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind === "taxonomy/emoji-prefix-uniqueness")).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("accepts distinct VS16 emoji identities", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      writeFileSync(join(owner, "📄️second.ts"), "");
      expect(policyEmojiPrefixBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️EmojiPrefixPolicy

//#region 🪟️WindowCompletenessPolicy
describe("window completeness policy", () => {
  test("requires item-only capability facets and their language mirrors", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-window-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Windows now live under a subset's surface (👁️viewer/✏️editor), not 🎛️apps (W3 dissolution,
    // ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) — mirrors the on-disk shape every other
    // fixture in this file already uses for 🏅️standards/🪆️subsets.
    const window = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode", "🪟️windows", "🧪️window");
    const crate = { shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(window, { recursive: true });
      // 🔣️ The required set is taxonomy-driven, never a literal here — adding a lane to
      // windowRequiredChildDirs must not require editing this assertion.
      const required = loadTaxonomy().windowRequiredChildDirs;
      const missingFacets = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingFacets.filter((breach) => breach.kind === "taxonomy/window-completeness")).toHaveLength(required.length);
      expect(required).toContain("🎚️config");
      expect(required).toContain("🫧️transient");
      for (const facet of required) {
        const dir = join(window, facet);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "📌️empty.md"), "");
      }
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      const action = join(window, "🎬️actions", "🧪️action");
      mkdirSync(action);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-component")).toHaveLength(2);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-empty-facet")).toHaveLength(1);
      rmSync(join(window, "🎬️actions", "📌️empty.md"));
      writeFileSync(join(action, "🦀️component.rs"), "");
      writeFileSync(join(action, "🟦️component.ts"), "");
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      writeFileSync(join(window, "🎬️actions", "🦀️component.rs"), "");
      expect(policyWindowCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/window-facet-component"]);
      rmSync(join(window, "🎬️actions", "🦀️component.rs"));
      rmSync(join(action, "🟦️component.ts"));
      const missingMirror = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingMirror.map((breach) => breach.kind)).toEqual(["taxonomy/window-component"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("requires every mode to declare its windows collection and its three state lanes", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-mode-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Modes now live under a subset's surface, not 🎛️apps (W3 dissolution) — see the sibling
    // window-completeness test above for the same shape rationale.
    const mode = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode");
    const crate = { shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(mode, { recursive: true });
      const required = loadTaxonomy().modeRequiredChildDirs;
      expect(required).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
      expect(policyModeCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/mode-completeness")).toHaveLength(required.length);
      for (const child of required) {
        mkdirSync(join(mode, child), { recursive: true });
      }
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(required.map(() => "taxonomy/mode-empty-child"));
      for (const child of required) {
        writeFileSync(join(mode, child, "📌️empty.md"), "");
      }
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
      mkdirSync(join(mode, "🪟️windows", "🧪️window"), { recursive: true });
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/mode-empty-child"]);
      rmSync(join(mode, "🪟️windows", "📌️empty.md"));
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🪟️WindowCompletenessPolicy

//#region 🧪️CompositionPolicy
describe("composition policy (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W6)", () => {
  test("canonical-artifact-kind rejects legacy ArtifactKindSpec.id grammar but accepts s.<plugin>.<artifact>", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-kind-policy-"));
    const artifact = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    try {
      mkdirSync(artifact, { recursive: true });
      writeFileSync(
        join(artifact, "🦀️component.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "3d.mesh".into(),', "    }", "}"].join("\n"),
      );
      const legacy = policyCanonicalArtifactKindBreaches(root);
      expect(legacy.map((breach) => breach.kind)).toEqual(["composition/canonical-artifact-kind"]);
      expect(legacy[0]?.priority).toBe("medium");

      writeFileSync(
        join(artifact, "🦀️component.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "s.probe.probe".into(),', "    }", "}"].join("\n"),
      );
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);

      // 🩹 Doc-comment prose quoting the pattern must not trip the rule (the exact false-positive
      // class flagged in the UCAS ticket's own 📌️important.md sweep-the-pattern lesson).
      writeFileSync(join(artifact, "🦀️component.rs"), '/// see `ArtifactKindSpec { id: "3d.mesh" }` for the old shape\n');
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("child-slot-kind-dag accepts an acyclic composition graph and rejects a cycle", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-dag-policy-"));
    const schemaA = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probea", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema");
    const schemaB = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probeb", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema");
    try {
      mkdirSync(schemaA, { recursive: true });
      mkdirSync(schemaB, { recursive: true });
      writeFileSync(join(schemaA, "🦀️component.rs"), '#[child(kind = "s.probe.probeb")] pub content: store::ArtifactChild<ProbeB>,\n');
      writeFileSync(join(schemaB, "🦀️component.rs"), "pub other: i32,\n");
      expect(policyChildSlotKindDagBreaches(root)).toEqual([]);

      writeFileSync(join(schemaB, "🦀️component.rs"), '#[child(kind = "s.probe.probea")] pub back: store::ArtifactChild<ProbeA>,\n');
      const cycles = policyChildSlotKindDagBreaches(root);
      expect(cycles).toHaveLength(1);
      expect(cycles[0]?.kind).toBe("composition/child-slot-kind-dag");
      expect(cycles[0]?.priority).toBe("high");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("dissolved-kind-redefinition bans redeclaring a frozen 🧿️semio subset type outside 🗄️stdio", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-dissolved-policy-"));
    const plugin = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    const stdio = join(root, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio");
    try {
      mkdirSync(plugin, { recursive: true });
      mkdirSync(stdio, { recursive: true });
      writeFileSync(join(plugin, "🦀️component.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      writeFileSync(join(stdio, "🦀️component.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      const breaches = policyDissolvedKindRedefinitionBreaches(root);
      expect(breaches).toHaveLength(1);
      expect(breaches[0]?.kind).toBe("composition/dissolved-kind-redefinition");
      expect(breaches[0]?.scope).toBe("✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️probe/🦀️component.rs");

      rmSync(join(plugin, "🦀️component.rs"));
      writeFileSync(join(plugin, "🦀️component.rs"), "pub struct ProbeOwnType { pub vertices: Vec<f64> }\n");
      expect(policyDissolvedKindRedefinitionBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️CompositionPolicy

describe("Neo4j graph database registry", () => {
  test("joins name segments with hyphen", () => {
    expect(joinNeo4jGraphDatabaseName(["compose", "kit"])).toBe("compose-kit");
  });

  test("partitions argv into name segments and uvx passthrough", () => {
    expect(partitionNeo4jGraphCliArgv(["metabolism", "--verbose"])).toEqual({
      nameParts: ["metabolism"],
      passthrough: ["--verbose"],
    });
  });

  test("product graphs are fixed four joined names", () => {
    expect(NEO4J_GRAPH_DATABASE_NAMES).toEqual(["compose", "elements", "coda", "reuse"]);
  });

  test("NEO4J_EXTRA_GRAPH_DATABASES extends export specs", () => {
    const env = { NEO4J_EXTRA_GRAPH_DATABASES: " foo , bar-baz " };
    expect(parseExtraNeo4jGraphDatabaseNamesFromEnv(env)).toEqual(["foo", "bar-baz"]);
    const names = getAllNeo4jGraphExportSpecs(env).map((s) => joinNeo4jGraphDatabaseName(s));
    expect(names).toContain("foo");
    expect(names).toContain("bar-baz");
  });
});

describe("isDevPortInUse", () => {
  test("returns false for a high ephemeral port", () => {
    expect(isDevPortInUse("127.0.0.1", 59_999)).toBe(false);
  });

  test("returns true when puzzle 3d play port is listening", () => {
    if (!isDevPortInUse("127.0.0.1", 6013) && !isDevPortInUse("127.0.0.1", 6014)) return;
    expect(isDevPortInUse("127.0.0.1", 6013) || isDevPortInUse("127.0.0.1", 6014)).toBe(true);
  });
});

describe("resolveDevPort", () => {
  test("returns preferred port when free", () => {
    expect(resolveDevPort("127.0.0.1", 59_990)).toBe(59_990);
  });

  test("skips occupied ports", () => {
    if (!isDevPortInUse("127.0.0.1", 6013)) return;
    expect(resolveDevPort("127.0.0.1", 6013)).toBeGreaterThan(6013);
  });

  test("honors skipPorts", () => {
    expect(resolveDevPort("127.0.0.1", 59_991, 5, new Set([59_991, 59_992]))).toBe(59_993);
  });
});

describe("devServerUrl", () => {
  test("maps 0.0.0.0 to loopback", () => {
    expect(devServerUrl("0.0.0.0", 6019)).toBe("http://127.0.0.1:6019/");
  });
});

describe("wgpuDevPlayUrl", () => {
  test("builds root and legacy play urls", () => {
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly", "/renderer-modules/wgpu/")).toBe("http://127.0.0.1:6178/renderer-modules/wgpu/?plugin=lowpoly");
  });
});

describe("canReuseDevPort", () => {
  test("returns true for an active dev HTTP server", () => {
    if (!isDevPortInUse("127.0.0.1", 6019)) return;
    expect(canReuseDevPort("127.0.0.1", 6019)).toBe(true);
  });
});

describe("describeDevPortOccupant", () => {
  test("returns undefined for a free port", () => {
    expect(describeDevPortOccupant(59_998)).toBeUndefined();
  });
});

describe("bundle-script", () => {
  test("ScriptRouter usage lists registered commands", () => {
    class A extends BundleScript {
      run(): void {}
    }
    const router = new ScriptRouter(import.meta.dir).register("a", A).register("b", A);
    expect(router.hasCommands()).toBe(true);
    expect(router.usage()).toContain("a");
    expect(router.usage()).toContain("b");
  });

  test("gitRepoRoot uses monorepo toplevel from repo/lib/js", async () => {
    const { gitRepoRoot } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { spawnSync } = await import("node:child_process");
    const top = gitRepoRoot(import.meta.dir);
    const n = Number(spawnSync("git", ["ls-files"], { cwd: top, encoding: "utf8" }).stdout?.split("\n").filter(Boolean).length ?? 0);
    expect(n).toBeGreaterThan(1000);
  });

  test("findRepoRoot reaches monorepo from repo/lib/js", () => {
    const root = findRepoRoot(import.meta.dir);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      {
        go: (rest) => {
          ran = rest.join(",");
        },
      },
      "unused",
    );
    expect(ran).toBe("x");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});

describe("dependency-boundary", () => {
  test("detects adapter region marker", () => {
    expect(isAdapterBoundaryFile("pkg/foo.ts", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/main.py", "# #region 🔌️Adapters\nimport fastapi")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/index.ts", "//#region 🌐️RsWasmTransport\nexport async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/kit-store.🟦️worker.ts", "export async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/bin/assistant/mcp-app.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("framework/platform/renderer/react/index.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import { z } from "zod";\nexport const a = 1;\n`;
    const file = "compose/client/lib/js/boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("dependencyBoundaryBreachesForBundleDir walks nested tsx", () => {
    const repoRoot = new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
    const dir = "framework/playground/renderer/react/puzzle";
    const breachs = dependencyBoundaryBreachesForBundleDir(repoRoot, dir);
    expect(breachs.every((b) => b.scope.startsWith("framework/playground/renderer/react/puzzle"))).toBe(true);
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌️Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌️Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs).toEqual([]);
  });
});

describe("ui scrollbar styling", () => {
  test("🎨️ui.css defines scrollbar tokens and native plus Scrollable rules", () => {
    const repoRoot = findRepoRoot(import.meta.dir);
    const css = readFileSync(join(repoRoot, "framework/module/ui/styling/js/🎨️ui.css"), "utf8");
    expect(css).toContain("--scrollbar-size:");
    expect(css).toContain("--scrollbar-thumb:");
    expect(css).toContain("scrollbar-color:");
    expect(css).toContain("*::-webkit-scrollbar-thumb");
    expect(css).toContain('[data-slot="scroll-area-thumb"]');
  });
});

describe("micro-commit", () => {
  function gitDirFor(root: string): string {
    const rel = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim() ?? ".git";
    return rel.startsWith("/") ? rel : join(root, rel);
  }

  test("safeGitEnv replaces an unusable configured global Git config", async () => {
    const { safeGitEnv } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { devNull } = await import("node:os");
    const env = safeGitEnv({ GIT_CONFIG_GLOBAL: join(process.cwd(), ".missing-global-gitconfig") });
    expect(env.GIT_CONFIG_GLOBAL).toBe(devNull);
  });

  test("branchValidationError distinguishes lookup failure, detached HEAD, and a wrong branch", async () => {
    const { branchValidationError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(branchValidationError("micro-commit", { ok: false, error: "permission denied" })).toBe("micro-commit: cannot read current branch: permission denied");
    expect(branchValidationError("micro-commit", { ok: true, name: "" })).toContain("detached HEAD");
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/feature" })).toContain('current branch "🐙️ueli/feature"');
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
    expect(branchValidationError("micro-commit", { ok: true, name: "🐙️ueli/⛳️wip" })).toBeNull();
  });

  test("extractCounterFromSubject reads formatted subject lines", async () => {
    const { extractCounterFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractCounterFromSubject("🧑️ueli🎆️26🌙️06☀️02🚩️009")).toEqual({ nnn: 9, line1Base: "🧑️ueli🎆️26🌙️06☀️02" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️397")).toEqual({ nnn: 397, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️396")).toEqual({ nnn: 396, line1Base: "🐙️ueli🎆️26🌙️06☀️04" });
    expect(extractCounterFromSubject("33")).toBeNull();
    expect(extractCounterFromSubject("Merge branch foo")).toBeNull();
  });

  test("extractNumericCounterFromSubject reads GitKraken numeric subjects", async () => {
    const { extractNumericCounterFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractNumericCounterFromSubject("299")).toBe(299);
    expect(extractNumericCounterFromSubject("001")).toBe(1);
    expect(extractNumericCounterFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("line1BaseFromBundleTag reads WIP epoch from squash tag", async () => {
    const { line1BaseFromBundleTag } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️")).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(line1BaseFromBundleTag("🐙️ueli🎆️26🌙️06☀️04🚩️151")).toBeNull();
  });

  test("bumpCounterFromHistory uses max across formatted commits", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["🐙️ueli🎆️26🌙️06☀️02🚩️033", "🐙️ueli🎆️26🌙️06☀️02🚩️032", "unrelated"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-06-02T12:00:00"));
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️02");
    expect(bumped.nnn).toBe("034");
    const fresh = bumpCounterFromHistory(["unrelated"], contributor, new Date("2026-06-02T12:00:00"));
    expect(fresh.nnn).toBe("001");
  });

  test("bumpCounterFromHistory preserves selector-free history with canonical output", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const bumped = bumpCounterFromHistory(["🦢️other🎆️26🌙️07☀️31🚩️999", "🐙️ueli🎆️26🌙️06☀️04🚩️397", "🐙️ueli🎆️26🌙️06☀️04🚩️396"], contributor, new Date("2026-07-31T12:00:00"));
    expect(bumped).toEqual({ line1Base: "🐙️ueli🎆️26🌙️06☀️04", nnn: "398" });
    expect(() => bumpCounterFromHistory(["🐙️ueli🎆️26🌙️6☀️04🚩️397"], contributor, new Date("2026-07-31T12:00:00"))).toThrow("refusing to reset counter to 001");
  });

  test("bumpCounterFromHistory continues numeric GitKraken subjects with WIP epoch", async () => {
    const { bumpCounterFromHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli", email: "u@example.com" };
    const subjects = ["299", "298", "297"];
    const bumped = bumpCounterFromHistory(subjects, contributor, new Date("2026-07-17T12:00:00"), "🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.line1Base).toBe("🐙️ueli🎆️26🌙️06☀️04");
    expect(bumped.nnn).toBe("300");
  });

  test("normalizeBulletLines strips uloc block lines", async () => {
    const { normalizeBulletLines } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bullets = normalizeBulletLines("🎆️Summary\n📊️metric📃uloc💯️65k➕️1✏️1🟰️2\n📊️metric🟦️typescript📃uloc💯️65k➕️1✏️1🟰️2\n🐛️Fix bug");
    expect(bullets).toEqual(["🎆️Summary", "🐛️Fix bug"]);
  });

  test("bulletEmojiValidationError rejects fireworks emoji on bullets", async () => {
    const { bulletLeadEmoji, bulletEmojiValidationError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(bulletLeadEmoji("🎆️Drop stacked intro")).toBe("🎆️");
    expect(bulletEmojiValidationError(["🎆️All bullets wrongly use fireworks"])).toContain("🎆️");
    expect(bulletEmojiValidationError(["🐛️Fix real bug"])).toBeNull();
    expect(bulletEmojiValidationError(["🧬️Tune WASM flush timing"])).toBeNull();
    expect(bulletEmojiValidationError(["📊️uloc block"])).toContain("📊️");
  });

  test("normalizeBulletLines enforces compact {emoji}{description} format", async () => {
    const { normalizeBulletLines, formatMicroCommitBulletLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(formatMicroCommitBulletLine("- 🐛️ Fix PDF")).toBe("🐛️Fix PDF");
    expect(normalizeBulletLines("🐛️ Fix PDF\n- 🖼️ Tweak UI")).toEqual(["🐛️Fix PDF", "🖼️Tweak UI"]);
    expect(normalizeBulletLines(Array.from({ length: 10 }, (_, i) => `🎆️item ${i}`).join("\n"))).toHaveLength(8);
  });

  test("buildMicroCommitMessage separates GitKraken summary and description", async () => {
    const { buildMicroCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = process.cwd();
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const msg = buildMicroCommitMessage(root, contributor, ["🎆️LLM-authored change summary"], {
      countRepoByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),
    });
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🚩️\d{3}$/);
    expect(lines[1]).toMatch(/^🎆️/);
    expect(lines.some((l) => l.includes("LLM-authored"))).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
    expect(lines.at(-2)).toBe("");
    const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const metricsIdx = lines.findIndex((l) => l.startsWith(ulocHeader));
    expect(metricsIdx).toBeGreaterThan(2);
    expect(lines[metricsIdx - 1]).toBe("");
    expect(lines[metricsIdx]?.startsWith(ulocHeader)).toBe(true);
    expect(lines[metricsIdx + 1]).toMatch(/^📊️metric/);
  });

  test("buildMicroCommitMessage rejects output without the required uloc footer", async () => {
    const { buildMicroCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-empty-uloc-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      expect(() =>
        buildMicroCommitMessage(root, contributor, ["🐛️Reject incomplete commit messages"], {
          countRepoByLanguage: () => ({}),
        }),
      ).toThrow("required 📊️metric footer");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("formatMicroCommitMetricsLines uses compact loc and delta counts", async () => {
    const {
      formatMicroCommitMetricLine,
      formatMicroCommitMetricsLines,
      formatMetricLocCount,
      formatMetricSizeCount,
      formatMetricRatio,
      COMMIT_METRIC_HEADER,
    } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(COMMIT_METRIC_HEADER).toBe("📊️metric");
    expect(formatMetricSizeCount(1024)).toBe("1.02KB");
    expect(formatMetricSizeCount(10_400_000_000)).toBe("10.4GB");
    expect(formatMetricLocCount(200_000)).toBe("200k");
    expect(formatMetricLocCount(1_300_000)).toBe("1.3M");
    expect(formatMetricLocCount(769_000)).toBe("769k");
    expect(formatMetricLocCount(422_377)).toBe("422k");
    expect(formatMetricLocCount(52_759)).toBe("52.8k");
    expect(formatMetricLocCount(500)).toBe("500");
    expect(formatMetricRatio(0.001)).toBe("0.1");
    expect(formatMetricRatio(0.0000854)).toBe("0.009");
    expect(formatMetricRatio(0.0305)).toBe("3.05");
    expect(formatMetricRatio(0.00264)).toBe("0.264");
    expect(formatMetricRatio(0.10061)).toBe("10.1");
    expect(formatMetricRatio(1.21001)).toBe("121");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 0, added: 0, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 2, removed: 0 })).toBe("📊️metric🐚️shell📃uloc💯️2k📈️2➗️0.1➕️2✏️2🟰️4");
    expect(formatMicroCommitMetricLine({ lang: "Shell", emoji: "🐚️", code: 2000, edited: 2, added: 0, removed: 2 })).toBe("📊️metric🐚️shell📃uloc💯️2k📉️2➗️0.1✏️2➖️2🟰️4");
    const lines = formatMicroCommitMetricsLines([{ lang: "Rust", emoji: "🦀️", code: 200_000, edited: 2220, added: 2000, removed: 500 }]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
    expect(lines[1]).toBe("📊️metric🦀️rust📃uloc💯️200k📈️1.5k➗️0.756➕️2k✏️2.22k➖️500🟰️4.72k");
  });

  test("formatMicroCommitMetricsLines totals all languages on the first row", async () => {
    const { formatMicroCommitMetricsLines } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const lines = formatMicroCommitMetricsLines([
      { lang: "TypeScript", emoji: "🟦️", code: 3000, edited: 10, added: 8, removed: 0 },
      { lang: "Markdown", emoji: "📝️", code: 44, edited: 0, added: 0, removed: 0 },
    ]);
    expect(lines[0]).toBe("📊️metric📃uloc💯️3.04k📈️8➗️0.264➕️8✏️10🟰️18");
    expect(lines[1]).toBe("📊️metric🟦️typescript📃uloc💯️3k📈️8➗️0.267➕️8✏️10🟰️18");
    expect(lines[2]).toBe("📊️metric📝️markdown📃uloc💯️44");
  });

  test("buildMicroCommitMetrics merges uloc and git numstat by language", async () => {
    const { buildMicroCommitMetrics } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = process.cwd();
    const metrics = buildMicroCommitMetrics(root, {
      countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }),
    });
    expect(metrics.some((m) => m.lang === "Rust" && m.code === 100)).toBe(true);
  });

  test("isUlocCachePlausible rejects partial caches", async () => {
    const { isUlocCachePlausible, gitRepoRoot } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    const root = gitRepoRoot(import.meta.dir);
    const tracked = spawnSync("git", ["ls-files"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.split("\n").filter(Boolean).length ?? 0;
    try {
      expect(isUlocCachePlausible(root, {})).toBe(false);
      if (tracked <= 1000) return;
      expect(isUlocCachePlausible(root, { TypeScript: 3000, JavaScript: 78, Markdown: 44, JSON: 43 })).toBe(false);
      expect(
        isUlocCachePlausible(root, {
          JSON: 400_000,
          TypeScript: 150_000,
          Go: 90_000,
          Rust: 44_000,
          Markdown: 10_000,
          Python: 30_000,
        }),
      ).toBe(true);
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
    }
  });

  test("shouldSkipPathForUloc skips dot paths license templates and .🦑️repo", async () => {
    const { shouldSkipPathForUloc } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = process.cwd();
    expect(shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, ".agents/skills/micro-commit/SKILL.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "compose/client/ui/LICENSE.md")).toBe(true);
    expect(shouldSkipPathForUloc(root, "repo/AGENTS.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "repo/CHANGELOG.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, ".🦑️repo/⚡️cache/x")).toBe(true);
    expect(shouldSkipPathForUloc(root, "framework/README.md")).toBe(false);
    expect(shouldSkipPathForUloc(root, "puzzle/3d/src/foo.ts")).toBe(false);
  });

  test("countJsonKeys counts nested object keys", async () => {
    const { countJsonKeys } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(countJsonKeys('{"a":1,"b":{"c":2}}')).toBe(3);
  });

  test("appendGitDeltaSuffix formats legacy delta-only suffixes", async () => {
    const { appendGitDeltaSuffix, formatBundleUlocSuffix } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(appendGitDeltaSuffix("🟦️65k", { added: 700, edited: 200, removed: 10 })).toBe("🟦️65k➕️700✏️200➖️10🟰️910");
    expect(formatBundleUlocSuffix({ added: 700, edited: 200, removed: 10 }, 65_000)).toBe("📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910");
  });

  test("splitGitNumstatDelta separates replaced lines from net added and removed", async () => {
    const { splitGitNumstatDelta } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(splitGitNumstatDelta(4, 2)).toEqual({ edited: 2, added: 2, removed: 0 });
    expect(splitGitNumstatDelta(2, 4)).toEqual({ edited: 2, added: 0, removed: 2 });
    expect(splitGitNumstatDelta(5, 5)).toEqual({ edited: 5, added: 0, removed: 0 });
    expect(splitGitNumstatDelta(10, 0)).toEqual({ edited: 0, added: 10, removed: 0 });
    expect(splitGitNumstatDelta(0, 7)).toEqual({ edited: 0, added: 0, removed: 7 });
  });

  test("countUnifiedLocForFile uses physical lines for code and keys for json", async () => {
    const { countUnifiedLocForFile } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(countUnifiedLocForFile("x.rs", "// c\nfn main() {}\n")).toBe(3);
    expect(countUnifiedLocForFile("x.json", '{"k":1}')).toBe(1);
  });

  test("classifyPathForMetrics maps TeX ecosystem extensions", async () => {
    const { classifyPathForMetrics, langMetricsEmoji } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(classifyPathForMetrics("mit-bestand/bericht/zwischenbericht/🖋️zwischenbericht.tex")).toBe("TeX");
    expect(classifyPathForMetrics("print/semio.sty")).toBe("TeX");
    expect(classifyPathForMetrics("print/🖋️semio.cls")).toBe("TeX");
    expect(classifyPathForMetrics("report/📚️references.bib")).toBe("TeX");
    expect(classifyPathForMetrics("doc/sample.ltx")).toBe("TeX");
    expect(langMetricsEmoji("TeX")).toBe("📐️");
  });

  test("uncoveredStagedAreas flags missing cursor-plans and product coverage", async () => {
    const { uncoveredStagedAreas } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md", "framework/product/playground/renderer/react/index.tsx"];
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain(".cursor/plans");
    expect(uncoveredStagedAreas(["🫡️Only micro-commit skill wording"], staged)).toContain("product");
    const ok = uncoveredStagedAreas(["📋️Plan brush edge resurrection guard and sync", "🖌️Playground renderer restores brush placement after structural deletes"], staged);
    expect(ok).toEqual([]);
  });

  test("validateBulletsAgainstStaged rejects bullets that ignore staged product code", async () => {
    const root = getWorkspaceRoot();
    const stagedHasPresentation = spawnSync("git", ["diff", "--cached", "--name-only"], { cwd: root, encoding: "utf8" }).stdout?.includes("presentation/");
    if (!stagedHasPresentation) return;
    const prev = process.env.REPO_ROOT;
    process.env.REPO_ROOT = root;
    const stdin = ["🫡️Only micro-commit skill docs"].join("\n");
    const r = spawnSync(process.execPath, ["./📜️script.ts", "micro-commit", "prepare"], {
      cwd: root,
      input: stdin,
      encoding: "utf8",
      timeout: 120_000,
    });
    if (prev === undefined) delete process.env.REPO_ROOT;
    else process.env.REPO_ROOT = prev;
    expect(r.status).not.toBe(0);
  }, 120_000);

  test("installMicroCommitGitHooks writes portable hooks and bun pin", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      const init = spawnSync("git", ["init"], { cwd: root, encoding: "utf8" });
      expect(init.status).toBe(0);
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("compose_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(existsSync(join(root, ".🧬semio/🦑️repo/compose-micro-commit-bun"))).toBe(true);
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msgFile = join(root, ".git", "COMMIT_EDITMSG");
      writeFileSync(msgFile, "🐙️ueli manual subject\n", "utf8");
      handlePrepareCommitMsg(root, msgFile, "template");
      expect(readFileSync(msgFile, "utf8")).toContain("manual subject");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("wipeAfterCommit clears all GK templates and prepare state", async () => {
    const { wipeAfterCommit, writeMicroCommitTemplates } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-wipe-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "u@example.com"], { cwd: root, env: gitSpawnEnv() });
      spawnSync("git", ["config", "user.name", "U"], { cwd: root, env: gitSpawnEnv() });
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️test reset\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      writeFileSync(join(gitDir, "gkcommittemplate-099.txt"), "stale numbered", "utf8");
      wipeAfterCommit(root);
      const gkTemplate = join(gitDir, "gkcommittemplate.txt");
      expect(existsSync(gkTemplate)).toBe(true);
      expect(readFileSync(gkTemplate, "utf8")).toBe("");
      const gkLeft = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gkLeft).toEqual(["gkcommittemplate.txt"]);
      expect(existsSync(join(gitDir, "compose-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(gitDir, "COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const { mkdtempSync, readdirSync, rmSync, readFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-tpl-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️bullet\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      const gk = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gk).toEqual(["gkcommittemplate.txt"]);
      expect(readFileSync(join(gitDir, "gkcommittemplate.txt"), "utf8")).toContain("🚩️");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("shouldRefreshPreparedCommitMessage keeps user edits", async () => {
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
});

describe("playground static sites", () => {
  test("PLAYGROUND_PORTS keeps cad and dag on distinct reserved dev ports", () => {
    expect(playgroundDevPort("cad")).toBe(6020);
    expect(playgroundDevPort("dag")).toBe(6017);
    expect(allPlaygroundReservedPorts().size).toBeGreaterThanOrEqual(Object.keys(PLAYGROUND_PORTS).length);
  });

  test("resolveFrameworkOsPlaygroundPlugin maps CLI segments to OS plugin ids", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["dag"])).toEqual({ plugin: "dag", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["gis", "2d"])).toEqual({ plugin: "gis2d", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["procedural", "3d", "fixture", "hexagonal-column"])).toEqual({
      plugin: "procedural3d",
      rest: ["fixture", "hexagonal-column"],
    });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["trinity", "jack"])).toEqual({ plugin: "trinity-jack", rest: [] });
    expect(resolveFrameworkOsPlaygroundPlugin(catalog, ["unknown"])).toBeNull();
    const resolvableSegments = catalog.reduce((sum, row) => sum + 1 + row.aliases.length, 0);
    expect(resolvableSegments).toBeGreaterThan(20);
  });

  test("frameworkOsPlaygroundDevEnv defaults wgpu renderer and resolves catalog dev port", () => {
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const dagEnv = frameworkOsPlaygroundDevEnv(catalog, "dag", {}, {});
    expect(dagEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(dagEnv.SEMIO_PLUGIN).toBe("dag");
    expect(dagEnv.S_OS_PORT).toBe("6117");

    const cadEnv = frameworkOsPlaygroundDevEnv(catalog, "cad", {}, { S_OS_PORT: "6020" });
    expect(cadEnv.SEMIO_RENDERER).toBe("wgpu");
    expect(cadEnv.SEMIO_PLUGIN).toBe("cad");
    expect(cadEnv.S_OS_PORT).toBe("6020");
  });

  test("assigns a unique port per dev and test slot", () => {
    const seen = new Set<number>();
    for (const spec of Object.values(PLAYGROUND_PORTS)) {
      expect(seen.has(spec.dev)).toBe(false);
      seen.add(spec.dev);
      if (spec.test !== undefined) {
        expect(seen.has(spec.test)).toBe(false);
        seen.add(spec.test);
      }
    }
  }, 30_000);

  test("playgroundPlayViteDefine embeds locked fixture env", () => {
    const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
    try {
      delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('""');
      process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
      expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID"]).toBe('"concrete-forest"');
    } finally {
      if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
      else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
    }
  });

  test("playgroundStaticSiteBuildOptions uses relative-base dist output", () => {
    expect(playgroundStaticSiteBuildOptions()).toEqual({
      target: "esnext",
      outDir: "dist",
      emptyOutDir: true,
    });
    expect(playgroundStaticSiteBuildOptions({ sourcemap: true }).sourcemap).toBe(true);
  });
});

describe("package boundary guards", () => {
  const repoRoot = findRepoRoot();

  test("vite and vitest configs avoid cross-package @semio-tech aliases", () => {
    const offenders: string[] = [];
    const walk = (dir: string) => {
      for (const entry of require("node:fs").readdirSync(dir, { withFileTypes: true })) {
        if (entry.name === "node_modules" || entry.name === ".git" || entry.name === ".🧬semio" || entry.name === "dist" || entry.name === "target" || entry.name === ".claude") continue;
        const full = join(dir, entry.name);
        if (entry.isDirectory()) {
          walk(full);
          continue;
        }
        if (!/vite.*\.config\.ts$/.test(entry.name) && entry.name !== "🧪️vitest.config.ts") continue;
        const pkgDir = findNearestPackageDir(full, repoRoot);
        const text = readFileSync(full, "utf8");
        for (const match of text.matchAll(/find:\s*(?:\/\^)?["']@semio-tech\/[^"']+["']/g)) {
          const line = text.slice(0, match.index).split("\n").length;
          const snippet = text.split("\n")[line - 1] ?? "";
          if (/path\.resolve\(__dirname,\s*["']\.\./.test(snippet) && pkgDir && !snippet.includes("node_modules")) {
            offenders.push(`${full.replace(repoRoot + "/", "")}:${line}`);
          }
        }
      }
    };
    walk(repoRoot);
    expect(offenders).toEqual([]);
  });

  test("legacy package aliases are absent from source imports", () => {
    const result = spawnSync("rg", ["-l", "@compose/ui|@ui/react|@elements/", "--glob", "*.{ts,tsx}", "--glob", "!**/.🧬semio/**", "--glob", "!**/🧪️index.test.ts"], {
      cwd: repoRoot,
      encoding: "utf8",
    });
    const files = (result.stdout ?? "")
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    expect(files).toEqual([]);
  });

  test("framework renderer host has no per-technology registerUi surface host APIs", () => {
    const indexPath = join(repoRoot, "framework/product/os/module/renderer/js/react/index.tsx");
    const indexSource = readFileSync(indexPath, "utf8");
    expect(indexSource).not.toMatch(/registerUi(?:Draw|Flow|Layout|Note|Puzzle2d|Puzzle3d|Puzzle5d|Sequence|Writer|Raster|Forms|Trinity|Procedural|Shooting|Gis|Cad|Dag|Lowpoly|Imperative|S)SurfaceHost/);
    expect(indexSource).toContain("bootFrameworkOs");
  });
});

function findNearestPackageDir(filePath: string, repoRoot: string): string | undefined {
  let dir = join(filePath, "..");
  while (dir.startsWith(repoRoot)) {
    if (existsSync(join(dir, "package.json"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return undefined;
}

describe("commit", () => {
  test("parseCommitBundleBody reads emoji scopes dates and bullets", async () => {
    const { parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = parseCommitBundleBody("🏘️compose✍️sketchpad\n🎆️26🌙️06☀️04\n🗺️Map work\n🎆️26🌙️06☀️03\n🧪️Playground\n\n🖱️ui⚛️react\n🎆️26🌙️06☀️02\n🖥️Shell");
    expect(bundles).toHaveLength(2);
    expect(bundles[0]?.label).toBe("🏘️compose✍️sketchpad");
    expect(bundles[0]?.dates).toHaveLength(2);
    expect(bundles[0]?.dates[0]?.bullets[0]).toBe("🗺️Map work");
  });

  test("parseCommitBundleBody rejects path prefixes and reserved emojis", async () => {
    const { parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(() => parseCommitBundleBody("compose/foo|🏘️compose\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🏘️compose🔀️📊️metric\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
    expect(() => parseCommitBundleBody("🗺️🧩️🕸️\n🎆️26🌙️06☀️04\n🗺️Map work")).toThrow();
  });

  test("normalizeBundleScopeLabel strips reserved and uloc suffix", async () => {
    const { normalizeBundleScopeLabel } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(normalizeBundleScopeLabel("🏘️compose🔀️📊️metric📃uloc➕️1")).toBe("🏘️compose");
  });

  test("isBundleScopeLine accepts area and technology root labels", async () => {
    const { isBundleScopeLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(isBundleScopeLine("🌐️gis📍️map")).toBe(true);
    expect(isBundleScopeLine("🖱️ui⚛️react")).toBe(true);
    expect(isBundleScopeLine("🥅️framework")).toBe(true);
    expect(isBundleScopeLine("🧪️Playground")).toBe(false);
    expect(isBundleScopeLine("🗺️Single emoji line")).toBe(false);
  });

  test("extractBundleDateLineFromSubject reads calendar day from micro-commit subject", async () => {
    const { extractBundleDateLineFromSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(extractBundleDateLineFromSubject("🐙️ueli🎆️26🌙️06☀️04🚩️012")).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromSubject("unrelated")).toBeNull();
  });

  test("extractBundleDateLineFromCommit prefers body timestamp over subject checkpoint day", async () => {
    const { extractBundleDateLineFromCommit, extractBundleDateLineFromCommitBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const body = "🎆️26🌙️06☀️04⏰️02⌚️38⏱️38\n🗺️Map work\n";
    expect(extractBundleDateLineFromCommitBody(body)).toBe("🎆️26🌙️06☀️04");
    expect(extractBundleDateLineFromCommit("🐙️ueli🎆️26🌙️06☀️02🚩️084", body)).toBe("🎆️26🌙️06☀️04");
  });

  test("pathsFromNumstatRow expands rename paths", async () => {
    const { pathsFromNumstatRow } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(pathsFromNumstatRow("old/a.ts\tnew/b.ts")).toEqual(["old/a.ts", "new/b.ts"]);
    expect(pathsFromNumstatRow("dir/{old.ts => new.ts}")).toEqual(["old.ts", "new.ts"]);
  });

  test("pathMatchesBundleIndex does not treat empty prefix set as match-all", async () => {
    const { pathMatchesBundleIndex } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [
      { label: "🏘️compose✍️sketchpad", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["✍️x"] }] },
      { label: "🏘️compose🗃️fixtures", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗃️y"] }] },
    ];
    const prefixSets = [[], ["compose/fixture"]];
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 0, prefixSets, bundles)).toBe(false);
    expect(pathMatchesBundleIndex("compose/fixture/a.json", 1, prefixSets, bundles)).toBe(true);
  });

  test("formatBundleDateLine appends per-day uloc suffix", async () => {
    const { formatBundleDateLine } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(formatBundleDateLine("🎆️26🌙️06☀️04", { added: 700, edited: 200, removed: 10 }, { added: 788_000, edited: 0, removed: 0 }, 65_000, 10_400_000_000)).toBe(
      "🎆️26🌙️06☀️04📊️metric📃uloc💯️65k📈️690➗️1.07➕️700✏️200➖️10🟰️910📊️metric💾size💯️10.4GB📈️788KB➗️0.008➕️788KB🟰️788KB",
    );
  });

  test("commitBundleBodyError rejects per-day uloc on stdin", async () => {
    const { commitBundleBodyError } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(commitBundleBodyError("🏘️compose\n🎆️26🌙️06☀️04📊️metric📃uloc➕️1\n🗺️Work")).toMatch(/per-day/);
  });

  test("validateMicroCommitLangMetricsDeltaSum passes when language rows sum to footer", async () => {
    const { validateMicroCommitLangMetricsDeltaSum } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(() =>
      validateMicroCommitLangMetricsDeltaSum([
        { lang: "TypeScript", emoji: "🟦️", code: 100, edited: 5, added: 3, removed: 0 },
        { lang: "Rust", emoji: "🦀️", code: 50, edited: 2, added: 1, removed: 1 },
      ]),
    ).not.toThrow();
  });

  test("validateBundleCommitAttribution requires bundle headers to sum to range total", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { validateBundleCommitAttribution, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-check-sum-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      mkdirSync(join(root, "other"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      writeFileSync(join(root, "other/b.ts"), "b\nc\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      const msg = join(root, "mc.txt");
      writeFileSync(msg, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️04⏰️12⌚️00⏱️00\n🔧️Work\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg], { cwd: root });
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Only repo");
      expect(() => validateBundleCommitAttribution(root, wip, "HEAD", bundles)).toThrow(/not attributed to any bundle|do not add up/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("validateBundleDayDeltasAttribution rejects unlisted micro-commit day", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [{ label: "📚️repo🔧️js", dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Net"] }] }];
    const prefixSets = [["repo/js/"]];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️03", { added: 4, removed: 0, edited: 0 }],
          ["🎆️26🌙️06☀️04", { added: 0, removed: 4, edited: 0 }],
        ]),
      ],
    ]);
    const bundleTotals = [{ added: 0, removed: 0, edited: 0 }];
    expect(() => validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, bundleTotals)).toThrow(/missing from your bundle body/);
  });

  test("validateBundleDayDeltasAttribution rejects when listed days do not sum to bundle total", async () => {
    const { validateBundleDayDeltasAttribution } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const bundles = [
      {
        label: "📚️repo🔧️js",
        dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🔧️Only day four listed"] }],
      },
    ];
    const dateDeltas = new Map([
      [
        0,
        new Map([
          ["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }],
          ["🎆️26🌙️06☀️03", { added: 5, edited: 0, removed: 0 }],
        ]),
      ],
    ]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltas, [{ added: 2, edited: 0, removed: 0 }])).toThrow(/missing from your bundle body/);
    const dateDeltasOneDay = new Map([[0, new Map([["🎆️26🌙️06☀️04", { added: 2, edited: 0, removed: 0 }]])]]);
    expect(() => validateBundleDayDeltasAttribution(bundles, [["repo/js"]], dateDeltasOneDay, [{ added: 7, edited: 0, removed: 0 }])).toThrow(/does not add up/);
  });

  test("buildCommitMessage appends per-day uloc from micro-commit dates", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { buildCommitMessage, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-commit-day-uloc-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "repo", "js"), { recursive: true });
      writeFileSync(join(root, "repo/js/a.ts"), "a\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "🐙️ueli🎆️26🌙️06☀️01🔀️"], { cwd: root });
      const wip = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg1 = join(tmpdir(), `compose-mc1-${Date.now()}.txt`);
      writeFileSync(msg1, "🐙️ueli🎆️26🌙️06☀️01🚩️001\n\n🎆️26🌙️06☀️03⏰️12⌚️00⏱️00\n🔧️Day three\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg1], { cwd: root });
      writeFileSync(join(root, "repo/js/a.ts"), "a\nb\nc\nd\n", "utf8");
      spawnSync("git", ["add", "repo/js/a.ts"], { cwd: root });
      const msg2 = join(tmpdir(), `compose-mc2-${Date.now()}.txt`);
      writeFileSync(msg2, "🐙️ueli🎆️26🌙️06☀️01🚩️002\n\n🎆️26🌙️06☀️04⏰️12⌚️01⏱️00\n🔧️Day four\n", "utf8");
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-F", msg2], { cwd: root });
      const contributor = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
      const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Day four\n🎆️26🌙️06☀️03\n🔧️Day three");
      const msg = buildCommitMessage(root, contributor, bundles, wip, "HEAD");
      expect(msg).toMatch(/🎆️26🌙️06☀️04📊️metric/);
      expect(msg).toMatch(/🎆️26🌙️06☀️03📊️metric/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("sortCommitBundlesByEditTotal orders bundles by descending gitDeltaLineTotal", async () => {
    const { mkdtempSync, writeFileSync, mkdirSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { sortCommitBundlesByEditTotal } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const mk = (label: string) => ({
      label,
      dates: [{ dateLine: "🎆️26🌙️06☀️04", bullets: ["🗺️change"] }],
    });
    const root = mkdtempSync(join(tmpdir(), "compose-commit-sort-"));
    try {
      spawnSync("git", ["init"], { cwd: root });
      spawnSync("git", ["config", "user.email", "t@e.com"], { cwd: root });
      spawnSync("git", ["config", "user.name", "T"], { cwd: root });
      spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root });
      mkdirSync(join(root, "compose"), { recursive: true });
      mkdirSync(join(root, "ui"), { recursive: true });
      mkdirSync(join(root, "framework"), { recursive: true });
      writeFileSync(join(root, "compose/a.ts"), "a\n", "utf8");
      writeFileSync(join(root, "ui/b.ts"), "b\n", "utf8");
      writeFileSync(join(root, "framework/c.ts"), "c\n", "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "init"], { cwd: root });
      const base = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      writeFileSync(join(root, "compose/a.ts"), `${"a\n".repeat(11)}`, "utf8");
      writeFileSync(join(root, "ui/b.ts"), `${"b\n".repeat(151)}`, "utf8");
      writeFileSync(join(root, "framework/c.ts"), `${"c\n".repeat(5)}`, "utf8");
      spawnSync("git", ["add", "-A"], { cwd: root });
      spawnSync("git", ["-c", "commit.gpgsign=false", "commit", "-m", "delta"], { cwd: root });
      const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).stdout?.trim()!;
      const bundles = [mk("🏘️compose"), mk("🖱️ui"), mk("🥅️framework")];
      const paths = [["compose/a.ts"], ["ui/b.ts"], ["framework/c.ts"]];
      const sorted = sortCommitBundlesByEditTotal(root, base, head, bundles, paths);
      expect(sorted.bundles.map((b) => b.label)).toEqual(["🖱️ui", "🏘️compose", "🥅️framework"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("buildCommitMessage renders bundle subject and footer", async () => {
    const { buildCommitMessage, parseCommitBundleBody } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const contributor = { alias: "ueli", emoji: "🐙️", name: "Ueli Saluz", email: "ueli@semio-tech.com" };
    const bundles = parseCommitBundleBody("📚️repo🔧️js\n🎆️26🌙️06☀️04\n🔧️Tooling");
    const msg = buildCommitMessage(process.cwd(), contributor, bundles, "0000000000000000000000000000000000000000", "0000000000000000000000000000000000000000", { countRepoByLanguage: () => ({ TypeScript: 1000 }) });
    const lines = msg.trimEnd().split("\n");
    expect(lines[0]).toMatch(/🔀️$/);
    expect(lines.some((l) => l.includes("📊️metric"))).toBe(true);
    expect(lines.at(-1)).toMatch(/^Signed-off-by: /);
  });

  test("formatBundleTagName and formatBundleSubject use contributor date emojis", async () => {
    const { formatBundleTagName, formatBundleSubject } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const c = { alias: "ueli", emoji: "🐙️", name: "U", email: "u@e.com" };
    const now = new Date("2026-06-04T12:00:00");
    expect(formatBundleTagName(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(formatBundleSubject(c, now)).toBe("🐙️ueli🎆️26🌙️06☀️04🔀️");
  });

  test("formatCommitPrepareCommands emits four fenced git blocks", async () => {
    const { formatCommitPrepareCommands } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const out = formatCommitPrepareCommands({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc123def456",
      messageFile: ".git/compose-commit-message",
    });
    const blocks = out.trimEnd().split("\n\n");
    expect(blocks).toHaveLength(4);
    expect(blocks[0]).toBe("```\ngit tag -s -m '🐙️ueli🎆️26🌙️06☀️04🚩️' '🐙️ueli🎆️26🌙️06☀️04🚩️' HEAD\n```");
  });

  test("formatCommitPrepareAgentReply ends with tag name and commit message blocks", async () => {
    const { formatCommitPrepareAgentReply } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const commitMessage = "🐙️ueli🎆️26🌙️06☀️04🔀️\n\n🏘️compose✍️sketchpad📊️metric\n🎆️26🌙️06☀️04\n🗺️Work\n\n📊️metric📃uloc➕️1🟰️1\n\nSigned-off-by: U <u@e.com>\n";
    const out = formatCommitPrepareAgentReply({
      tagName: "🐙️ueli🎆️26🌙️06☀️04🚩️",
      wipSha: "abc",
      commitMessage,
    });
    const fenceBodies = [...out.matchAll(/```\n([\s\S]*?)\n```/g)].map((m) => m[1]!);
    expect(fenceBodies).toHaveLength(6);
    expect(fenceBodies[4]).toBe("🐙️ueli🎆️26🌙️06☀️04🚩️");
    expect(fenceBodies[5]).toBe(commitMessage.trimEnd());
  });

  test("parseCommitSteps treats cs as squash without tag", async () => {
    const { parseCommitSteps } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(parseCommitSteps(["cs"])).toEqual({ tag: false, squash: true, push: false });
    expect(parseCommitSteps(["ct", "cs", "cp"])).toEqual({ tag: true, squash: true, push: true });
  });

  test("bulletMatchesCommitHistory detects verbatim prior commit lines", async () => {
    const { bulletMatchesCommitHistory } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    const history = new Set(["🗺️copied line from an old micro-commit"]);
    expect(bulletMatchesCommitHistory("🗺️copied line from an old micro-commit", history)).toBe(true);
    expect(bulletMatchesCommitHistory("🗺️fresh summary written from git diff", history)).toBe(false);
  });
});

describe("command budgets", () => {
  const indexPath = join(import.meta.dir, "index.ts");

  /** ⏱️Runs `runCmd` in a fresh subprocess (its budget-exceeded path calls `process.exit`, which would kill the test runner in-process) against a child that sleeps far longer than its budget. */
  function spawnBudgetedSleep(budgetMs: number, envOverride?: Record<string, string>): ReturnType<typeof spawnSync> {
    const script = `const { runCmd } = await import(${JSON.stringify(indexPath)}); runCmd(${JSON.stringify(process.execPath)}, ["-e", "await Bun.sleep(10000)"], ${budgetMs === -1 ? "{}" : `{ budgetMs: ${budgetMs} }`});`;
    return spawnSync(process.execPath, ["-e", script], { encoding: "utf8", env: { ...process.env, ...envOverride } });
  }

  test("kills a command that exceeds its explicit budget", () => {
    const start = Date.now();
    const result = spawnBudgetedSleep(250);
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
    expect(Date.now() - start).toBeLessThan(5000);
  });

  test("SEMIO_CMD_BUDGET_MS env override kills a command with no explicit budget", () => {
    const result = spawnBudgetedSleep(-1, { SEMIO_CMD_BUDGET_MS: "250" });
    expect(result.status).not.toBe(0);
    expect(result.stderr).toContain("[budget]");
  });

  test("orchestratorBudgetOpts supplies a bounded orchestrator budget", () => {
    expect(orchestratorBudgetOpts()).toEqual({ budgetMs: orchestratorBudgetMs() });
    expect(() => runCmd(process.execPath, ["-e", "1"], orchestratorBudgetOpts())).not.toThrow();
  });

  test("daemonBudgetOpts returns the daemon budget class", () => {
    expect(daemonBudgetOpts()).toEqual({ budgetMs: daemonBudgetMs() });
    expect(daemonBudgetMs()).toBe(DAEMON_BUDGET_MS);
    expect(orchestratorBudgetMs()).toBe(ORCHESTRATOR_BUDGET_MS);
  });

  test("goLevelTestArgs includes -timeout derived from the level budget", () => {
    const args = goLevelTestArgs("fundamental");
    expect(args[0]).toBe("-timeout");
    expect(args[1]).toMatch(/^\d+s$/);
    expect(args[1]).toBe(`${Math.ceil(testLevelBudgetMs("fundamental") / 1000)}s`);
  });

  test("vitestLevelArgs returns per-test timeout flags", () => {
    const ms = String(testLevelBudgetMs("quick"));
    expect(vitestLevelArgs("quick")).toEqual(["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms]);
  });

  test("orchestratorBudgetMs defaults to 4h and honors SEMIO_ORCHESTRATOR_BUDGET_MS", () => {
    expect(orchestratorBudgetMs()).toBe(4 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
    process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = "12345";
    try {
      expect(orchestratorBudgetMs()).toBe(12345);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_ORCHESTRATOR_BUDGET_MS;
      else process.env.SEMIO_ORCHESTRATOR_BUDGET_MS = prev;
    }
  });

  test("daemonBudgetMs defaults to 24h and honors SEMIO_DAEMON_BUDGET_MS", () => {
    expect(daemonBudgetMs()).toBe(24 * 60 * 60 * 1000);
    const prev = process.env.SEMIO_DAEMON_BUDGET_MS;
    process.env.SEMIO_DAEMON_BUDGET_MS = "67890";
    try {
      expect(daemonBudgetMs()).toBe(67890);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_DAEMON_BUDGET_MS;
      else process.env.SEMIO_DAEMON_BUDGET_MS = prev;
    }
  });

  test("testLevelBudgetMs maps levels and honors SEMIO_TEST_BUDGET_MS", async () => {
    const { TEST_LEVEL_BUDGET_MS } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts");
    expect(testLevelBudgetMs("fundamental")).toBe(TEST_LEVEL_BUDGET_MS.fundamental);
    expect(testLevelBudgetMs("exhaustive")).toBe(TEST_LEVEL_BUDGET_MS.exhaustive);
    const prev = process.env.SEMIO_TEST_BUDGET_MS;
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    try {
      expect(testLevelBudgetMs("quick")).toBe(42000);
    } finally {
      if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
      else process.env.SEMIO_TEST_BUDGET_MS = prev;
    }
  });

  test("runProbe captures stdout under budget", () => {
    const { status, stdout } = runProbe(process.execPath, ["-e", "console.log('probe-ok')"]);
    expect(status).toBe(0);
    expect(stdout.trim()).toBe("probe-ok");
  });

  test("runCmdStatus returns the exit status instead of throwing", () => {
    expect(runCmdStatus(process.execPath, ["-e", "process.exit(3)"])).toBe(3);
  });

  test("budgetTimeoutHint defaults cargo to the target-dir lock-contention hint", () => {
    expect(budgetTimeoutHint("cargo")).toContain("target-dir lock contention");
  });

  test("budgetTimeoutHint honors an explicit override", () => {
    expect(budgetTimeoutHint("git", "custom hint")).toBe("custom hint");
  });

  test("budgetTimeoutHint gives non-cargo commands a generic hint", () => {
    expect(budgetTimeoutHint("git")).not.toContain("target-dir lock contention");
  });
});

describe("resolveCargoPackageName", () => {
  test("resolves short lib names to full package names", () => {
    const root = process.cwd();
    expect(resolveCargoPackageName("db_actor", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/📦️packages/🦀️rust"))).toBe("semio-framework-os-kernel-db-actor");
    expect(resolveCargoPackageName("semio-s-plugin-architect", join(root, "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust"))).toBe("semio-s-plugin-architect");
    expect(resolveCargoPackageName("semio-s-plugin-energy", join(root, "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust"))).toBe("semio-s-plugin-energy");
  });

  test("resolves empty package list to local Cargo.toml package name", () => {
    const root = process.cwd();
    const dbActorDir = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/📦️packages/🦀️rust");
    expect(resolveCargoPackageNames([], dbActorDir)).toEqual(["semio-framework-os-kernel-db-actor"]);
  });
});

describe("loadTaxonomy", () => {
  test("parses 🔣️taxonomy.json into the expected shape", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.mutationChildDirs).toEqual(["🦠️mutation", "🔺️diff", "↩️inverse"]);
    expect(taxonomy.snapshotChildDirs).toEqual(["🧬️schema", "🎒️pack"]);
    expect(taxonomy.diffChildDirs).toEqual(["🧬️schema"]);
    expect(taxonomy.schemaFormats).toEqual({
      "🦀️rust": { leafFilename: "🦀️component.rs", extension: ".rs", fieldCasing: "snake" },
      "🟦️typescript": { leafFilename: "🟦️component.ts", extension: ".ts", fieldCasing: "camel" },
      "🔗️graphql": { leafFilename: "🔗️component.graphql", extension: ".graphql", fieldCasing: "camel" },
      "🔣️jsonschema": { leafFilename: "🔣️component.json", extension: ".json", fieldCasing: "camel" },
      "🛰️protobuf": { leafFilename: "🛰️component.proto", extension: ".proto", fieldCasing: "snake" },
    });
    expect(taxonomy.artifactSchemaSpecFilenames).toEqual({
      "🧬️schema": "🔣️component.json",
      "📸️snapshot/🧬️schema": "🔣️component.json",
      "🔺️diff/🧬️schema": "🔣️component.json",
    });
    expect(taxonomy.artifactSpecFilenames["📸️snapshot/🎒️pack"]).toBe("📡️component.protocol.semio");
    expect("🎒️pack" in taxonomy.artifactSpecFilenames).toBe(false);
    expect(taxonomy.windowChildDirs).toEqual(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "🎚️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.windowRequiredChildDirs).toEqual(["🎬️actions", "🪛️utilities", "🎚️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeRequiredChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.pluginRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.osRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.transientChildDirs).toEqual(["🧬️schema"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceSchemaSpecFilenames["🫧️transient/🧬️schema"]).toBe("🔣️component.json");
    expect(taxonomy.windowComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowEmptyFacetFilename).toBe("📌️empty.md");
    expect(taxonomy.taxonomyLeafFilenames["🦀️rust"]).toBe("🦀️component.rs");
    expect(taxonomy.taxonomyLeafFilenames["🟦️typescript"]).toBe("🟦️component.ts");
    expect(taxonomy.artifactSpecFilenames["🗣️dsl"]).toBe("📖️component.grammar.semio");
    expect(taxonomy.artifactSpecFilenames["📡️spr"]).toBe("📡️component.protocol.semio");
    expect(taxonomy.libWiringLineBudget).toBe(150);
    expect(taxonomy.packagesDirName).toBe("📦️packages");
    expect(Object.keys(taxonomy.areas).length).toBeGreaterThan(0);
  });

  test("declares direct plugin-root facets without a nested directory taxonomy field", () => {
    const taxonomy = loadTaxonomy();
    expect("pluginDirName" in taxonomy).toBe(false);
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules"]);
    expect(taxonomy.osChildDirs).toEqual(["🎮️commands"]);
  });

  test("keeps the artifact completeness set and the artifact structural set as two separate lists", () => {
    const taxonomy = loadTaxonomy();
    // 🗿️ Completeness is 🧬️schema + 🚪️io — never ⚙️engine; structural-only extra is 📚️examples.
    expect(taxonomy.artifactChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "🔨️modules"]);
    expect(taxonomy.artifactChildDirs.filter((dir) => !taxonomy.artifactComponentDirs.includes(dir))).toEqual(["📚️examples", "🔨️modules"]);
  });

  test("derives lifecycle capabilities instead of declaring artifact facet directories", () => {
    const taxonomy = loadTaxonomy();
    const lifecycleDirs = ["🏗️builder", "🧐️analyzer", "🎹️composer"];
    expect(taxonomy.newArtifactComponentDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.newArtifactChildDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.standardComponentDirs).toEqual(["🪆️subsets"]);
    expect(taxonomy.standardChildDirs).toEqual(["🪆️subsets", "🔨️modules"]);
    expect(taxonomy.subsetComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.subsetChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"]);
    expect(taxonomy.subsetArchetypes).toEqual(["owning", "derived"]);
    expect(taxonomy.ioFidelityClasses).toEqual(["exact", "canonical", "semantic", "lossy"]);
    expect([
      ...taxonomy.newArtifactChildDirs,
      ...taxonomy.standardChildDirs,
      ...taxonomy.subsetChildDirs,
    ].filter((dir) => lifecycleDirs.includes(dir))).toEqual([]);
  });

  test("describes the per-example assets/tests shape instead of plural facet dirs", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleAssetsDirName).toBe("🖼️assets");
    expect(taxonomy.exampleTestsDirName).toBe("🧪️tests");
    expect(taxonomy.exampleSlugPattern).toBe("^.+\\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$");
    expect(taxonomy.exampleAssetKindPrefixes).toEqual({
      dsl: "🗣️",
      op: "🔧️",
      spr: "📡️",
      pack: "🎒️",
      diff: "🔺️",
      cmd: "🎮️",
    });
    expect(taxonomy.exampleMediaKindPrefixes).toEqual({
      image: "🖼️",
      mesh: "🧊️",
      document: "📄️",
      video: "🎬️",
    });
    expect(taxonomy.exampleLeafFilenames).toEqual({
      "🦀️rust": "🦀️component.rs",
      "🟦️typescript": "🟦️component.ts",
    });
    expect(taxonomy.exampleTestLeafFilenames).toEqual({
      "🦀️rust": "🦀️test.rs",
      "🟦️typescript": "🟦️test.ts",
    });
    expect(taxonomy.forbiddenExampleSlugs).toEqual(["♻️reuse", "♻️default", "📕️default", "♻️semio"]);
    expect(taxonomy.forbiddenExamplePluralDirs).toEqual(["🎒️packs", "🗣️dsls", "🔧️ops", "📡️sprs"]);
    expect(taxonomy.surfaceChildDirs).toContain("📚️examples");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🎒️packs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🗣️dsls");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🔧️ops");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("📡️sprs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🖼️assets");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🧪️tests");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️mutations");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🦠️mutation");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("↩️inverse");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🔺️diff");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️schema");
    expect("exampleComponentDirs" in taxonomy).toBe(false);
  });

  test("forbids both the plural and singular implementations spellings", () => {
    expect(loadTaxonomy().forbiddenPathSegments).toEqual(["⚡️implementations", "⚡️implementation"]);
  });

  test("describes every declared lang with a manifest/marker/leaf contract", () => {
    const taxonomy = loadTaxonomy();
    for (const lang of taxonomy.langs) {
      const ecosystem = taxonomy.ecosystems[lang];
      expect(ecosystem).toBeDefined();
      expect(ecosystem.leafFilename.includes("component")).toBe(true);
    }
    expect(taxonomy.ecosystems["🦀️rust"].marker).toEqual({ in: "manifest", format: "toml", table: "package.metadata.semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🟦️typescript"].marker).toEqual({ in: "manifest", format: "json", table: "semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🐍️python"].marker?.table).toBe("tool.semio");
    // 🐹️ Go's manifest is 📋️project.json (go.mod must stay at the owner root — a Go module root has to contain its sources).
    expect(taxonomy.ecosystems["🐹️go"].manifestFilename).toBe("📋️project.json");
    expect(taxonomy.ecosystems["🐹️go"].moduleRootFilename).toBe("go.mod");
    expect(taxonomy.ecosystems["🐹️go"].marker?.table).toBe("metadata.semio");
  });

  test("encodes Shape V2 entry location and both valid rust #[path] base conventions", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.entryLocation).toBe("packages");
    expect(taxonomy.rustEntryPathRules.entryDirFromOwner).toBe("📦️packages/🦀️rust");
    expect(taxonomy.rustEntryPathRules.resolution).toBe("cumulative");
    expect(taxonomy.rustEntryPathRules.conventions.map((convention) => convention.id)).toEqual(["leaf-prefixed", "once-reset"]);
    // 🥞️ A grouping `#[path = "."]` reset is never prefixed under either convention — the bug that only a real compile catches.
    expect(taxonomy.rustEntryPathRules.conventions.every((convention) => convention.groupingReset === ".")).toBe(true);
  });

  test("declares the area-state enum that replaces LEGACY_LAYOUT_TOLERANT", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.areaStates).toEqual(["legacy", "mixed", "clean", "exempt"]);
    expect(taxonomy.migratedMarker).toBe("packages-dir-exists");
  });
});

describe("validateTaxonomy", () => {
  test("the shipped vocabulary is internally consistent", () => {
    expect(validateTaxonomy()).toEqual([]);
  });

  test("declares schema facet kinds partitioning data vs interface formats", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaFacetKinds?.["🧬️data"]?.normativeFormat).toBe("🔣️jsonschema");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.normativeFormat).toBe("📜️wit");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.formats).toEqual(["📜️wit"]);
    expect(taxonomy.schemaFormats?.["📜️wit"]?.fieldCasing).toBe("kebab");
    expect(taxonomy.packagingDirNames).toEqual(["🎯️targets", "fixture", "app"]);
    expect(taxonomy.ecosystems["🦀️rust"]?.packagingDirNames).toEqual(["benches", "🟦️typescript"]);
  });

  test("declares canonical semantic collection and module ownership contracts", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.semanticManifestFilename).toBe("🔣️component.json");
    expect(taxonomy.semanticExtensionKey).toBe("x-semio");
    expect(taxonomy.semanticConsumerMinimum).toBe(2);
    expect(taxonomy.semanticCollections["🔨️modules"]?.kind).toBe("module");
    expect(taxonomy.semanticCollections["💡️inferences"]?.kind).toBe("inference");
    expect(taxonomy.semanticCollections["🚪️io/🧬️mutations"]).toEqual({ kind: "io", direction: "transport" });
    expect(taxonomy.ioSemanticCollectionDirNames).toEqual(["📸️snapshot", "🔺️diff", "💡️inferences", "🧬️mutations"]);
    expect(artifactFacetPathIsDeclared("🚪️io/🧬️mutations/📝️text", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/🧬️mutations/💾️binary", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/💾️binary", taxonomy)).toBe(false);
    expect(validateTaxonomy({ ...taxonomy, ioSemanticCollectionDirNames: ["💡️inferences"] }).some((problem) => problem.includes('must include "🧬️mutations"'))).toBe(true);
  });

  test("declares the surface vocabulary ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET froze", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaVersion).toBe(6);
    expect(taxonomy.viewerDirName).toBe("👁️viewer");
    expect(taxonomy.editorDirName).toBe("✏️editor");
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: "👁️viewer", editor: "✏️editor" });
    expect(taxonomy.subsetChildDirs).toContain("👁️viewer");
    expect(taxonomy.subsetChildDirs).toContain("✏️editor");
    expect(taxonomy.subsetRequiredSurfaceDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.contributedSubsetChildDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.surfaceChildDirs).toContain(taxonomy.modesDirName);
    expect(taxonomy.surfaceComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowLeafLangs).toEqual(["🦀️rust", "🟦️typescript"]);
  });

  test("surfaceRoles order is load-bearing — it is the AppRole declaration order and the u8 channel tag", () => {
    const taxonomy = loadTaxonomy();
    const swapped = { ...taxonomy, surfaceRoles: ["editor", "viewer"] };
    expect(validateTaxonomy(swapped).some((problem) => problem.includes("surfaceRoles must be exactly"))).toBe(true);
  });

  test("a surface dir name and its role mapping cannot drift apart", () => {
    const taxonomy = loadTaxonomy();
    const drifted = { ...taxonomy, surfaceDirNames: { ...taxonomy.surfaceDirNames, viewer: "🫣️viewer" } };
    expect(validateTaxonomy(drifted).some((problem) => problem.includes("surfaceDirNames.viewer must equal viewerDirName"))).toBe(true);
  });

  test("a required surface child dir must be in the structural set and cover every state lane", () => {
    const taxonomy = loadTaxonomy();
    const missingStructural = { ...taxonomy, surfaceChildDirs: taxonomy.surfaceChildDirs.filter((dir) => dir !== "🎚️config") };
    expect(validateTaxonomy(missingStructural).some((problem) => problem.includes("surfaceRequiredChildDirs member \"🎚️config\" is missing from surfaceChildDirs"))).toBe(true);
    const missingLane = { ...taxonomy, surfaceRequiredChildDirs: taxonomy.surfaceRequiredChildDirs.filter((dir) => dir !== "👥️presence") };
    expect(validateTaxonomy(missingLane).some((problem) => problem.includes("surfaceRequiredChildDirs must include the state lane \"👥️presence\""))).toBe(true);
  });

  test("a surface dir declared outside subsetChildDirs is rejected", () => {
    const taxonomy = loadTaxonomy();
    const orphaned = { ...taxonomy, subsetChildDirs: taxonomy.subsetChildDirs.filter((dir) => dir !== "✏️editor") };
    expect(validateTaxonomy(orphaned).some((problem) => problem.includes("is missing from subsetChildDirs"))).toBe(true);
  });

  test("reports an area state outside the declared enum", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, "🧰️framework": "taxonomy" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("areaStates"))).toBe(true);
  });

  test("keeps plugin package-layout and taxonomy-tree graduation independently valid", () => {
    const taxonomy = loadTaxonomy();
    expect(validateTaxonomy({ ...taxonomy, pluginTaxonomyStates: {} }).some((problem) => problem.includes("is missing plugin area"))).toBe(true);
    expect(validateTaxonomy({ ...taxonomy, pluginTaxonomyStates: { ...taxonomy.pluginTaxonomyStates, "🧪️unknown": "mixed" } }).some((problem) => problem.includes("non-plugin area"))).toBe(true);
  });

  test("reports a completeness dir missing from the structural set", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactChildDirs: taxonomy.artifactChildDirs.filter((dir) => dir !== "🚪️io") };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🚪️io"))).toBe(true);
  });

  test("rejects explicit artifact lifecycle directories", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, subsetChildDirs: [...taxonomy.subsetChildDirs, "🧐️analyzer"] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("derived lifecycle facet"))).toBe(true);
  });

  test("reports mutationChildDirs missing from taxonomyLeafParentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      taxonomyLeafParentDirs: taxonomy.taxonomyLeafParentDirs.filter((dir) => dir !== "🦠️mutation"),
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🦠️mutation"))).toBe(true);
  });

  test("reports empty mutationChildDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, mutationChildDirs: [] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("mutationChildDirs"))).toBe(true);
  });

  test("reports an invalid required window capability set", () => {
    const taxonomy = loadTaxonomy();
    const missing = { ...taxonomy, windowRequiredChildDirs: [] };
    expect(validateTaxonomy(missing).some((problem) => problem.includes("windowRequiredChildDirs must be a non-empty array"))).toBe(true);
    const outsideAllowlist = { ...taxonomy, windowRequiredChildDirs: [...taxonomy.windowRequiredChildDirs, "🧪️unknown"] };
    expect(validateTaxonomy(outsideAllowlist).some((problem) => problem.includes('"🧪️unknown" is missing from windowChildDirs'))).toBe(true);
  });

  test("reports command facets missing from plugin and os ownership", () => {
    const taxonomy = loadTaxonomy();
    expect(validateTaxonomy({ ...taxonomy, pluginRequiredChildDirs: [] }).some((problem) => problem.includes("pluginRequiredChildDirs must be a non-empty array"))).toBe(true);
    expect(validateTaxonomy({ ...taxonomy, osChildDirs: [] }).some((problem) => problem.includes('osRequiredChildDirs member "🎮️commands" is missing from osChildDirs'))).toBe(true);
  });

  test("reports a state lane missing from any state-owning scope", () => {
    const taxonomy = loadTaxonomy();
    expect(validateTaxonomy(taxonomy).filter((problem) => problem.includes("state lane"))).toEqual([]);
    const droppedFromModes = { ...taxonomy, modeChildDirs: taxonomy.modeChildDirs.filter((dir) => dir !== "🫧️transient") };
    expect(validateTaxonomy(droppedFromModes).some((problem) => problem.includes('modeChildDirs must include the state lane "🫧️transient"'))).toBe(true);
    const droppedFromWindows = { ...taxonomy, windowRequiredChildDirs: taxonomy.windowRequiredChildDirs.filter((dir) => dir !== "🎚️config") };
    expect(validateTaxonomy(droppedFromWindows).some((problem) => problem.includes('windowRequiredChildDirs must include the state lane "🎚️config"'))).toBe(true);
    const modelessWindows = { ...taxonomy, modeChildDirs: taxonomy.modeChildDirs.filter((dir) => dir !== taxonomy.windowsDirName) };
    expect(validateTaxonomy(modelessWindows).some((problem) => problem.includes("modeChildDirs must include"))).toBe(true);
  });

  test("reports an invalid window component language set", () => {
    const taxonomy = loadTaxonomy();
    const missing = { ...taxonomy, windowComponentLangs: [] };
    expect(validateTaxonomy(missing).some((problem) => problem.includes("windowComponentLangs must be a non-empty array"))).toBe(true);
    const unknown = { ...taxonomy, windowComponentLangs: [...taxonomy.windowComponentLangs, "🧪️unknown"] };
    expect(validateTaxonomy(unknown).some((problem) => problem.includes('"🧪️unknown" has no taxonomyLeafFilenames entry'))).toBe(true);
    const missingMarker = { ...taxonomy, windowEmptyFacetFilename: "" };
    expect(validateTaxonomy(missingMarker).some((problem) => problem.includes("windowEmptyFacetFilename must be a non-empty string"))).toBe(true);
  });

  test("reports empty pluginChildDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, pluginChildDirs: [] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("pluginChildDirs"))).toBe(true);
  });

  test("requires a commands facet at every command-owning scope", () => {
    const taxonomy = loadTaxonomy();
    for (const key of ["surfaceChildDirs", "modeChildDirs", "pluginChildDirs", "osChildDirs"] as const) {
      const broken = { ...taxonomy, [key]: taxonomy[key].filter((dir) => dir !== "🎮️commands") };
      expect(validateTaxonomy(broken).some((problem) => problem.includes(`${key} must include "🎮️commands"`))).toBe(true);
    }
  });

  test("rejects empty direct plugin-root facet declarations", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, pluginChildDirs: [...taxonomy.pluginChildDirs, ""] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("empty entry"))).toBe(true);
  });

  test("rejects plural example component dirs in taxonomyLeafParentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      taxonomyLeafParentDirs: [...taxonomy.taxonomyLeafParentDirs, "🎒️packs"],
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🎒️packs"))).toBe(true);
  });

  test("rejects missing example slug pattern", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, exampleSlugPattern: "" };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("exampleSlugPattern"))).toBe(true);
  });

  test("reports empty snapshotChildDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, snapshotChildDirs: [] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("snapshotChildDirs"))).toBe(true);
  });

  test("reports empty diffChildDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, diffChildDirs: [] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("diffChildDirs"))).toBe(true);
  });

  test("rejects bare pack in artifactComponentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactComponentDirs: [...taxonomy.artifactComponentDirs, "🎒️pack"], artifactChildDirs: [...taxonomy.artifactChildDirs, "🎒️pack"] };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('bare "🎒️pack"'))).toBe(true);
  });

  test("rejects bare pack key in artifactSpecFilenames", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactSpecFilenames: { ...taxonomy.artifactSpecFilenames, "🎒️pack": "📡️component.protocol.semio" } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('bare "🎒️pack" key'))).toBe(true);
  });

  test("reports schemaFormats leafFilename/extension mismatch", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      schemaFormats: {
        ...taxonomy.schemaFormats,
        "🦀️rust": { ...taxonomy.schemaFormats["🦀️rust"], leafFilename: "🦀️component.ts" },
      },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("leafFilename must end with its extension"))).toBe(true);
  });

  test("reports artifactSchemaSpecFilenames drift from jsonschema leaf", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      artifactSchemaSpecFilenames: { ...taxonomy.artifactSchemaSpecFilenames, "🧬️schema": "wrong.json" },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("artifactSchemaSpecFilenames"))).toBe(true);
  });
});

//#region 🧩️SemanticCollections
function semanticFixture(options: { readonly secondProductionConsumer?: boolean; readonly rootBehavior?: boolean; readonly rustRelativeConsumers?: boolean; readonly glueConsumer?: boolean } = {}): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-semantic-census-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🟦️component.ts", "export const measure = (value: number) => value;\n");
  if (options.rustRelativeConsumers || options.glueConsumer) write("🧰️framework/🔨️modules/📏measure/🦀️component.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️component.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: options.secondProductionConsumer ? ["height", "width"] : ["width"] } }] } }));
  write("🧰️framework/💡️inferences/📏width/🟦️component.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️component.ts";\nexport const width = measure(1);\n');
  if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🟦️component.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️component.ts";\nexport const height = measure(1);\n');
  if (options.rustRelativeConsumers) {
    write("🧰️framework/💡️inferences/📏width/🦀️component.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
    if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🦀️component.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  }
  if (options.glueConsumer) write("🧰️framework/💡️inferences/📏width/📦️packages/🦀️rust/📦️glue.rs", '#[path = "../../../../🔨️modules/📏measure/🦀️component.rs"]\npub mod measure;\n');
  write("🧰️framework/💡️inferences/🔣️component.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    ...(options.secondProductionConsumer ? [{ directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } }] : []),
  ] } }));
  if (options.rootBehavior) write("🧰️framework/💡️inferences/🟦️component.ts", "export function analyze() { return 1; }\n");
  return { root, taxonomy };
}

function schemaMountedRustModulesFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-schema-mounted-rust-modules-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🦀️component.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️component.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: ["height", "width"] } }] } }));
  write("🧰️framework/🧬️schema/💡️inferences/📏width/🦀️component.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/↕️height/🦀️component.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/🔣️component.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    { directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } },
  ] } }));
  return { root, taxonomy };
}

function mutationTransportFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-mutation-transport-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🚪️io/🧬️mutations/📝️text/🟦️component.ts", "export const encodeMutationText = (value: string) => value;\n");
  write("🧰️framework/🚪️io/🧬️mutations/💾️binary/🟦️component.ts", "export const encodeMutationBinary = (value: Uint8Array) => value;\n");
  write("🧰️framework/🚪️io/🧬️mutations/🔣️component.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📝️text", id: "framework.io.mutation-text", kind: "io", responsibility: "frozen mutation text transport", io: { format: "Semio mutation text", direction: "transport" } },
    { directory: "💾️binary", id: "framework.io.mutation-binary", kind: "io", responsibility: "frozen mutation binary transport", io: { format: "Semio mutation binary", direction: "transport" } },
  ] } }));
  return { root, taxonomy };
}

describe("semantic collection census", () => {
  test("accepts a genuine two-production-component module at its lowest common owner", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.computedLowestCommonOwner).toBe("🧰️framework");
      expect(renderSemanticCensusJson(census)).toBe(renderSemanticCensusJson(buildSemanticCensus(fixture.root, {}, fixture.taxonomy)));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves relative Rust and TypeScript imports to terminal production components", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true, rustRelativeConsumers: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const imports = census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import");
      expect(imports.map((edge) => edge.from)).toEqual(["height", "height", "width", "width"]);
      expect(imports.map((edge) => edge.target.endsWith("🦀️component.rs") || edge.target.endsWith("🟦️component.ts"))).toEqual([true, true, true, true]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves logical schema modules to their closest physical module collection", () => {
    const fixture = schemaMountedRustModulesFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import").map((edge) => edge.from)).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects a one-consumer module and authored list-root behavior", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("module-consumer-minimum");
      expect(codes).toContain("module-production-consumer-minimum");
      expect(codes).toContain("collection-authored-behavior");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("includes unregistered collection-path findings beneath a semantic owner scope", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const manifestPath = join(fixture.root, "🧰️framework/💡️inferences/🔣️component.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].id = "framework.inference.width";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      const scoped = buildSemanticCensus(fixture.root, { scope: "framework" }, fixture.taxonomy);
      expect(scoped.records.map((record) => record.id)).toEqual(["framework.inference.width"]);
      expect(scoped.problems.some((problem) => problem.code === "collection-authored-behavior" && problem.path.endsWith("💡️inferences/🟦️component.ts"))).toBe(true);
      expect(scoped.problems.some((problem) => problem.code === "module-production-consumer-minimum" && problem.path.endsWith("🔨️modules/📏measure"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("accepts mutation codecs only as explicit bidirectional I/O transport boundaries", () => {
    const fixture = mutationTransportFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.map((record) => record.id)).toEqual(["framework.io.mutation-binary", "framework.io.mutation-text"]);
      const manifestPath = join(fixture.root, "🧰️framework/🚪️io/🧬️mutations/🔣️component.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].io.direction = "import";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      expect(buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code)).toContain("io-contract-missing");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a test call site as a second production component", () => {
    const fixture = semanticFixture();
    try {
      const testPath = join(fixture.root, "🧰️framework/💡️inferences/📏width/🧪️tests/🟦️component.ts");
      mkdirSync(join(testPath, ".."), { recursive: true });
      writeFileSync(testPath, 'import { measure } from "../../../🔨️modules/📏measure/🟦️component.ts";\nexport const checked = measure(1);\n');
      const module = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).records.find((record) => record.id === "measure");
      expect(module?.productionConsumers).toEqual(["width"]);
      expect(module?.excludedConsumers).toEqual(["width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a package glue mount as a production consumer", () => {
    const fixture = semanticFixture({ glueConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const glueEdge = census.graph.edges.find((edge) => edge.source.endsWith("📦️glue.rs") && edge.to === "measure");
      expect(glueEdge?.production).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects missing manifest members, stale declarations, and generic stems", () => {
    const fixture = semanticFixture();
    try {
      const collection = join(fixture.root, "🧰️framework/💡️inferences");
      mkdirSync(join(collection, "🧪️orphan"), { recursive: true });
      writeFileSync(join(collection, "🧪️orphan/🟦️component.ts"), "export const orphan = true;\n");
      const manifest = JSON.parse(readFileSync(join(collection, "🔣️component.json"), "utf8"));
      manifest["x-semio"].members.push({ directory: "🧹️stale", id: "stale", kind: "inference", responsibility: "stale result", inference: { inputs: ["x"], target: "stale" } });
      manifest["x-semio"].members[0].directory = "🧪️shared";
      writeFileSync(join(collection, "🔣️component.json"), JSON.stringify(manifest));
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("manifest-child-missing");
      expect(codes).toContain("manifest-child-extra");
      expect(codes).toContain("member-generic-stem");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves cumulative nested Rust path attributes", () => {
    const root = join(tmpdir(), "semantic-rust-path");
    const source = [
      '#[path = "../../."]',
      "pub mod gltf {",
      '  #[path = "."]',
      "  pub mod schema {",
      '    #[path = "metrics/🦀️component.rs"]',
      "    pub mod metric;",
      "  }",
      "}",
    ].join("\n");
    expect(resolveRustPathAttributes(join(root, "📦️glue.rs"), source).at(-1)?.target).toBe(resolve(root, "../../metrics/🦀️component.rs"));
  });

  test("places inference codecs below I/O rather than the inference result collection", () => {
    const taxonomy = loadTaxonomy();
    expect(artifactFacetPathIsDeclared("🚪️io/💡️inferences/📝️text", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/💡️inferences/💾️binary", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/💾️binary", taxonomy)).toBe(false);
  });
});
//#endregion 🧩️SemanticCollections

describe("areaOf", () => {
  test("longest-prefix matches a plugin path to its declared area", () => {
    // 🕵️ "✏️s/🔌️plugins" graduated "legacy" -> "clean" in an earlier wave of
    // 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT; this assertion was never updated to match.
    expect(areaOf("✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust")).toBe("clean");
  });

  test("longest-prefix matches framework paths to mixed", () => {
    // 🕵️ W7 of 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT graduated "🧰️framework" (and
    // "✏️s/🔨️modules") "legacy" -> "mixed": most of the tree is still pre-Shape-V2, but enough
    // packages have migrated (schema self-registration, geometry relocation, s.*->os.* naming, WIT
    // extension-world) that "legacy" undersold it — "clean" would overclaim full Shape V2 purity.
    expect(areaOf("🧰️framework/🛍️products/💻️os")).toBe("mixed");
  });

  test("returns undefined outside every declared area", () => {
    expect(areaOf("some/unknown/path")).toBeUndefined();
  });
});

describe("readSemioMarker", () => {
  test("reads role = \"plugin\" from the writer plugin's migrated Cargo.toml", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml");
    expect(readSemioMarker(manifestPath, "🦀️rust")).toEqual({ role: "plugin" });
  });

  test("returns undefined for a non-existent manifest", () => {
    const root = getWorkspaceRoot();
    expect(readSemioMarker(join(root, "does/not/exist/Cargo.toml"), "🦀️rust")).toBeUndefined();
  });

  test("reads role + id from a typescript package.json's \"semio\" key", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json");
    expect(readSemioMarker(manifestPath, "🟦️typescript")).toEqual({ role: "framework", id: "ui-styling-ts" });
  });

  test("returns undefined for a non-existent manifest", () => {
    const root = getWorkspaceRoot();
    expect(readSemioMarker(join(root, "does/not/exist/Cargo.toml"), "🦀️rust")).toBeUndefined();
  });

  test("does not mistake one ecosystem's manifest for another's", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml");
    expect(readSemioMarker(manifestPath, "🐹️go")).toBeUndefined();
    expect(readSemioMarker(manifestPath, "🐍️python")).toBeUndefined();
  });

  test("reads the go and python marker contracts those ecosystems will carry", () => {
    // 🌱️ No go/python package declares a marker on disk yet (W8 restructures them); this pins the contract
    // the vocabulary promises so the reader is written and proven before the first real manifest lands.
    const dir = mkdtempSync(join(tmpdir(), "semio-marker-"));
    try {
      const goManifest = join(dir, "📋️project.json");
      writeFileSync(goManifest, JSON.stringify({ name: "@semio-tech/repo-cli", metadata: { semio: { role: "product", id: "repo-cli" } } }));
      expect(readSemioMarker(goManifest, "🐹️go")).toEqual({ role: "product", id: "repo-cli" });
      const pyManifest = join(dir, "pyproject.toml");
      writeFileSync(pyManifest, '[project]\nname = "x"\n\n[tool.semio]\nrole = "framework"\nid = "ui-styling-py"\n');
      expect(readSemioMarker(pyManifest, "🐍️python")).toEqual({ role: "framework", id: "ui-styling-py" });
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });
});

describe("discoverPackages", () => {
  /** 🧭️ Every plugin dir that carries a rust package manifest — derived from disk, never a hand-maintained list (`taxonomy.migratedMarker`). */
  const migratedPluginDirs = (root: string): string[] =>
    readdirSync(join(root, "✏️s/🔌️plugins"), { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && existsSync(join(root, "✏️s/🔌️plugins", entry.name, "📦️packages/🦀️rust/Cargo.toml")))
      .map((entry) => `✏️s/🔌️plugins/${entry.name}`)
      .sort();

  test("finds every migrated plugin under the real repo root", () => {
    const root = getWorkspaceRoot();
    const catalog = discoverPackages(root);
    const pluginOwners = [...new Set(catalog.filter((pkg) => pkg.role === "plugin").map((pkg) => pkg.ownerRel))].sort();
    expect(pluginOwners).toEqual(migratedPluginDirs(root));
    const writerEntry = catalog.find((pkg) => pkg.ownerRel === "✏️s/🔌️plugins/✒️writer");
    expect(writerEntry?.area).toBe("mixed");
    expect(writerEntry?.lang).toBe("🦀️rust");
    expect(writerEntry?.id).toBe("semio-s-plugin-writer");
  });

  test("only ever reports roles from the declared vocabulary", () => {
    const taxonomy = loadTaxonomy();
    const catalog = discoverPackages(getWorkspaceRoot());
    expect(catalog.filter((pkg) => !taxonomy.roles.includes(pkg.role))).toEqual([]);
    expect(catalog.filter((pkg) => !taxonomy.langs.includes(pkg.lang))).toEqual([]);
  });

  test("resolves the three-level 🎯️targets shape for framework ui", () => {
    const catalog = discoverPackages(getWorkspaceRoot());
    const uiTargets = catalog.filter((pkg) => pkg.ownerRel === "🧰️framework/🔨️modules/🖱️ui").map((pkg) => pkg.target).sort();
    expect(uiTargets).toEqual(["⌨️tui", "⚛️react", "🧊️wgpu"]);
    expect(catalog.filter((pkg) => pkg.ownerRel === "🧰️framework/🔨️modules/🖱️ui").every((pkg) => pkg.role === "framework")).toBe(true);
  });

  test("an in-flight plugin is discovered as mixed, never as a problem", () => {
    const root = getWorkspaceRoot();
    // 🏗️ fem is the last plugin still mid-migration: a marked new-contract package PLUS residual
    // ⚡️implementations sandwiches and a Shape V1 owner-root entry file. Tolerating that state (instead of
    // erroring on it) is the whole point of deriving maturity from disk. Once fem lands this flips to clean,
    // which the assertions below allow for without going vacuous.
    const owners = discoverOwners(root);
    const fem = owners.find((owner) => owner.ownerRel === "✏️s/🔌️plugins/🏗️fem");
    expect(fem).toBeDefined();
    expect(fem?.roles).toEqual(["plugin"]);
    expect(fem?.maturity).toBe(fem!.residualImplDirs === 0 && fem!.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed");
    expect(discoverPackageProblems(root).filter((problem) => problem.path.startsWith("✏️s/🔌️plugins/🏗️fem"))).toEqual([]);
  });

  test("the real repo raises no discovery problems", () => {
    expect(discoverPackageProblems(getWorkspaceRoot())).toEqual([]);
  });
});

describe("discoverOwners", () => {
  test("every owner groups its own packages and derives maturity from its residuals", () => {
    const owners = discoverOwners(getWorkspaceRoot());
    expect(owners.length).toBeGreaterThan(0);
    for (const owner of owners) {
      expect(owner.packages.every((pkg) => pkg.ownerRel === owner.ownerRel)).toBe(true);
      expect(owner.packages.every((pkg) => pkg.maturity === owner.maturity)).toBe(true);
      expect(owner.maturity).toBe(owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed");
      expect(owner.langs).toEqual([...new Set(owner.packages.map((pkg) => pkg.lang))]);
    }
  });

  test("nested owners are their own rows, not folded into the enclosing plugin", () => {
    const owners = discoverOwners(getWorkspaceRoot()).map((owner) => owner.ownerRel);
    // 🖍️ draw ships a proc-macro sibling crate; a nested 📦️packages dir is an owner in its own right.
    expect(owners).toContain("✏️s/🔌️plugins/🖍️draw");
    expect(owners.some((owner) => owner.startsWith("✏️s/🔌️plugins/🖍️draw/"))).toBe(true);
  });
});

describe("discoverBurndown", () => {
  /** 🔥️ Independent recursive count of forbidden-segment dirs under one path — deliberately not sharing the discovery walk. */
  const countImplDirs = (absDir: string): number => {
    let total = 0;
    for (const entry of readdirSync(absDir, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "target" || entry.name === "pkg") continue;
      if (loadTaxonomy().forbiddenPathSegments.includes(entry.name)) {
        total += 1;
        continue;
      }
      total += countImplDirs(join(absDir, entry.name));
    }
    return total;
  };

  test("counts residual ⚡️implementations dirs the same way an independent walk does", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const perPluginArea = burndown.implDirsByArea["mixed"] ?? 0;
    expect(perPluginArea).toBe(countImplDirs(join(root, "✏️s/🔌️plugins")));
    expect(burndown.implDirsTotal).toBeGreaterThanOrEqual(perPluginArea);
  });

  test("owner residual counts add up to their area total", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const owners = discoverOwners(root).filter((owner) => owner.area === "mixed");
    const summed = owners.reduce((total, owner) => total + owner.residualImplDirs, 0);
    expect(summed).toBe(burndown.implDirsByArea["mixed"] ?? 0);
    expect(burndown.mixedOwners.every((owner) => owner.residualImplDirs > 0 || owner.entryFilesAtOwnerRoot.length > 0)).toBe(true);
    expect(burndown.cleanOwners + burndown.mixedOwners.length).toBe(burndown.ownersTotal);
  });

  test("markerless package manifests stay visible even where they are a silent skip", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const catalogPaths = new Set(discoverPackages(root).map((pkg) => pkg.manifestPath));
    expect(burndown.unmarkedManifests.every((manifest) => !catalogPaths.has(manifest.path))).toBe(true);
    expect(burndown.packagingViolations.every((violation) => violation.path.includes("📦️packages/"))).toBe(true);
  });

  test("discoverPackageProblems promotes packaging violations into census problems", () => {
    const root = getWorkspaceRoot();
    clearDiscoveryCache();
    const problems = discoverPackageProblems(root);
    expect(problems.every((problem) => problem.kind === "packaging-violation" || problem.kind === "unknown-lang")).toBe(true);
    const censusProblems = buildSemanticCensus(root).problems.filter((problem) => problem.kind === "packaging-violation");
    expect(censusProblems.length).toBe(problems.filter((problem) => problem.kind === "packaging-violation").length);
  });

  test("clearDiscoveryCache forces a fresh walk", () => {
    const root = getWorkspaceRoot();
    const first = discoverPackages(root);
    clearDiscoveryCache();
    expect(discoverPackages(root).map((pkg) => pkg.manifestPath)).toEqual(first.map((pkg) => pkg.manifestPath));
  });
});

describe("computeWorkspaces", () => {
  /** 🧪️ Builds a synthetic repo tree covering every hazard `🗂️workspaces.ts` was written to handle:
   * a plain Shape V1 package, a nested Shape V2-ish package, a skip-dir (`node_modules`) and a dot-dir
   * that must never surface, a wasm `pkg/` shadowed by its outer wrapper's identical name, a wasm `pkg/`
   * with a genuinely different name (included regardless of whether anything depends on it via
   * `workspace:*` — a real `bun install` only requires a listed workspace dir to exist on disk, and
   * 🌊️flow's real `flow-extension-bim` has no such dependent at all since it's loaded by path at runtime),
   * and a stray `pkg/` with no sibling `Cargo.toml` (must be skipped and never descended into). */
  function buildFixture(): string {
    const root = mkdtempSync(join(tmpdir(), "workspaces-fixture-"));
    const write = (relPath: string, content: string) => {
      const absPath = join(root, relPath);
      mkdirSync(join(absPath, ".."), { recursive: true });
      writeFileSync(absPath, content);
    };
    write("a/package.json", JSON.stringify({ name: "@t/a" }));
    write("b/nested/package.json", JSON.stringify({ name: "@t/b-nested" }));
    write("node_modules/pkgx/package.json", JSON.stringify({ name: "@t/should-be-skipped" }));
    write(".hidden/package.json", JSON.stringify({ name: "@t/also-should-be-skipped" }));
    // c: wasm pkg/ shares its outer wrapper's exact name -> shadowed, pkg/ excluded.
    write("c/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/c-rs" }));
    write("c/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"c\"\n");
    write("c/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/c-rs" }));
    // d: wasm pkg/ has its OWN name and no outer wrapper at all -> included (flow-extension-bim's real shape).
    write("d/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"d\"\n");
    write("d/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/d-wasm" }));
    // f: wasm pkg/ differs from its outer wrapper's name too -> included the same way (flow-core's real shape).
    write("f/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/f-rs" }));
    write("f/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"f\"\n");
    write("f/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/f-wasm" }));
    // e: a pkg/ dir with no sibling Cargo.toml -- a broken/stray wasm-pack emission, must never surface
    // and must never be descended into (its own nested junk must not surface either).
    write("e/strayroot/pkg/junk/package.json", JSON.stringify({ name: "@t/e-stray-nested" }));
    write("e/strayroot/pkg/package.json", JSON.stringify({ name: "@t/e-stray" }));
    return root;
  }

  test("includes plain and nested packages, excludes node_modules/dot-dirs", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("a");
      expect(result).toContain("b/nested");
      expect(result.some((entry) => entry.includes("node_modules"))).toBe(false);
      expect(result.some((entry) => entry.includes(".hidden"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a wasm pkg/ shadowed by its outer wrapper's identical name is excluded", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("c/📦️packages/🦀️rust");
      expect(result).not.toContain("c/📦️packages/🦀️rust/pkg");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a differently-named wasm pkg/ is included whether or not it has an outer wrapper", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("d/📦️packages/🦀️rust/pkg"); // no outer wrapper at all
      expect(result).toContain("f/📦️packages/🦀️rust");
      expect(result).toContain("f/📦️packages/🦀️rust/pkg"); // differs from its outer wrapper's name
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a pkg/ dir with no sibling Cargo.toml is skipped and never descended into", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result.some((entry) => entry.startsWith("e/"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("throws on a genuine duplicate package name (never silently produces an ambiguous array)", () => {
    const root = mkdtempSync(join(tmpdir(), "workspaces-collision-"));
    try {
      mkdirSync(join(root, "x"), { recursive: true });
      mkdirSync(join(root, "y"), { recursive: true });
      writeFileSync(join(root, "x/package.json"), JSON.stringify({ name: "@t/dup" }));
      writeFileSync(join(root, "y/package.json"), JSON.stringify({ name: "@t/dup" }));
      expect(() => computeWorkspaces(root)).toThrow(/duplicate package name/);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("output is de-duplicated and lexicographically sorted", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(new Set(result).size).toBe(result.length);
      expect(result).toEqual([...result].sort((a, b) => a.localeCompare(b)));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("diffWorkspaces reports exactly what's missing/stale against a given current array", () => {
    const root = buildFixture();
    try {
      const expected = computeWorkspaces(root);
      expect(diffWorkspaces(root, []).missing).toEqual(expected);
      expect(diffWorkspaces(root, expected).missing).toEqual([]);
      expect(diffWorkspaces(root, expected).stale).toEqual([]);
      const withBogus = [...expected, "bogus/nonexistent"];
      expect(diffWorkspaces(root, withBogus).stale).toEqual(["bogus/nonexistent"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("against the real repo: every entry is a real dir with its own package.json, no duplicates, and known Shape V1/V2 + math drift cases resolve as expected", () => {
    const root = getWorkspaceRoot();
    const result = computeWorkspaces(root);
    for (const entry of result) expect(existsSync(join(root, entry, "package.json"))).toBe(true);
    expect(new Set(result).size).toBe(result.length);
    // Shape V1 (not yet migrated) framework core must still resolve — dropping it would break bun install repo-wide.
    expect(result).toContain("🧰️framework/📦️packages/🟦️typescript");
    // Shape V2 (already migrated) flow plugin TS residual.
    expect(result).toContain("✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript");
    // The documented drift this generator fixes: math npm-wrapper packages were resolving via nx only.
    expect(result).toContain("🧰️framework/🔨️modules/🧮️math/➕️algebra/📦️packages/🦀️rust");
    // flow-core's wasm pkg/ is the one real differently-named, really-depended-on case.
    expect(result).toContain("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/pkg");
    // editor's wasm pkg/ shares its outer wrapper's name -> must stay excluded (would collide otherwise).
    expect(result).not.toContain("🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg");
  });
});

//#region 🧪️PluginDependencyParityPolicy
describe("policyPluginDependencyParityBreaches", () => {
  test("excludes nested extensions from parent plugin walk while checking extensions independently", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️component.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n[dependencies]\nsemio-s-plugin-cad = { path = "../../../📦️packages/🦀️rust" }\n`);
      writeFileSync(join(extDir, "🦀️component.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad" && b.priority === "high")).toEqual([]);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building")).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("flags declared runtime dependency with missing Cargo dependency at extension scope", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️component.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n`);
      writeFileSync(join(extDir, "🦀️component.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad")).toEqual([]);
      const extBreaches = breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building");
      expect(extBreaches.length).toBe(1);
      expect(extBreaches[0]!.priority).toBe("high");
      expect(extBreaches[0]!.id).toBe("plugin-dependency-missing-cargo-✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building-cad");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
//#endregion 🧪️PluginDependencyParityPolicy

