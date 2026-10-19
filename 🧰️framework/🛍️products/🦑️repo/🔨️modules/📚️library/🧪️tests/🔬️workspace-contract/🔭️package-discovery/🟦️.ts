import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { policyPluginDependencyParityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts";
import { areaOf, clearDiscoveryCache, discoverBurndown, discoverOwners, discoverPackageProblems, discoverPackages, readSemioMarker } from "../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { computeWorkspaces, diffWorkspaces, getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyDiscoveredAllowlist } from "../../../🚨️policy/🔒️allowlist/🟦️.ts";
import { buildSemanticCensus } from "../../../🔍️discovery/🧩️semantic-census/🟦️.ts";

describe("areaOf", () => {
  test("longest-prefix matches a plugin path to its declared area", () => {
    // 🕵️ "✏️s/🔌️plugins" graduated "legacy" -> "clean" in an earlier wave of
    // 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT; this assertion was never updated to match.
    expect(areaOf("🌎️hub/🧩️compositions/✒️writer/📦️packages/🦀️rust")).toBe("clean");
  });

  test("longest-prefix matches framework paths to clean", () => {
    expect(areaOf("🧰️framework/🛍️products/💻️os")).toBe("clean");
  });

  test("returns undefined outside every declared area", () => {
    expect(areaOf("some/unknown/path")).toBeUndefined();
  });
});

describe("readSemioMarker", () => {
  test("reads role = \"hub\" from the writer composition's Cargo.toml", () => {
    const root = getWorkspaceRoot();
    const manifestPath = join(root, "🌎️hub/🧩️compositions/✒️writer/📦️packages/🦀️rust/Cargo.toml");
    expect(readSemioMarker(manifestPath, "🦀️rust")).toEqual({ role: "hub" });
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
    const manifestPath = join(root, "🌎️hub/🧩️compositions/✒️writer/📦️packages/🦀️rust/Cargo.toml");
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

  test.if(testLevelAtLeast("long"))("finds every migrated plugin under the real repo root", () => {
    const root = getWorkspaceRoot();
    const catalog = discoverPackages(root);
    const pluginOwners = [...new Set(catalog.filter((pkg) => pkg.role === "plugin").map((pkg) => pkg.ownerRel))].sort();
    expect(pluginOwners).toEqual(migratedPluginDirs(root));
    const writerEntry = catalog.find((pkg) => pkg.ownerRel === "✏️s/🔌️plugins/✒️writer");
    expect(writerEntry?.area).toBe("clean");
    expect(writerEntry?.lang).toBe("🦀️rust");
    expect(writerEntry?.id).toBe("semio-hub-writer");
  }, 600_000);

  test.if(testLevelAtLeast("long"))("only ever reports roles from the declared vocabulary", () => {
    const taxonomy = loadTaxonomy();
    const catalog = discoverPackages(getWorkspaceRoot());
    expect(catalog.filter((pkg) => !taxonomy.roles.includes(pkg.role))).toEqual([]);
    expect(catalog.filter((pkg) => !taxonomy.langs.includes(pkg.lang))).toEqual([]);
  }, 600_000);

  test.if(testLevelAtLeast("long"))("discovers every moved target-first React package with its exact owner and package root", () => {
    const catalog = discoverPackages(getWorkspaceRoot(), loadCatalogTaxonomy());
    const expected = [
      { id: "ui-react", role: "framework", ownerRel: "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react", target: "⚛️react" },
      { id: "renderer-react", role: "framework", ownerRel: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react", target: "⚛️react" },
      { id: "puzzle-5d-react", role: "plugin", ownerRel: "✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react", target: "⚛️5d-react" },
    ] as const;
    for (const row of expected) {
      const packageRel = `${row.ownerRel}/📦️packages/🟦️typescript`;
      expect(catalog.filter((pkg) => pkg.id === row.id).map(({ id, role, ownerRel, target, packageRel, manifestPath }) => ({ id, role, ownerRel, target, packageRel, manifestPath }))).toEqual([{ ...row, packageRel, manifestPath: `${packageRel}/package.json` }]);
    }
  }, 600_000);

  test.if(testLevelAtLeast("long"))("an in-flight plugin is discovered as mixed, never as a problem", () => {
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
  }, 600_000);

  test.if(testLevelAtLeast("long"))("the real repo raises no discovery problems", () => {
    expect(discoverPackageProblems(getWorkspaceRoot())).toEqual([]);
  }, 600_000);
});

describe("discoverOwners", () => {
  test.if(testLevelAtLeast("long"))("every owner groups its own packages and derives maturity from its residuals", () => {
    const owners = discoverOwners(getWorkspaceRoot());
    expect(owners.length).toBeGreaterThan(0);
    for (const owner of owners) {
      expect(owner.packages.every((pkg) => pkg.ownerRel === owner.ownerRel)).toBe(true);
      expect(owner.packages.every((pkg) => pkg.maturity === owner.maturity)).toBe(true);
      expect(owner.maturity).toBe(owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed");
      expect(owner.langs).toEqual([...new Set(owner.packages.map((pkg) => pkg.lang))]);
    }
  });

  test.if(testLevelAtLeast("long"))("nested owners are their own rows, not folded into the enclosing plugin", () => {
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

  test.if(testLevelAtLeast("long"))("counts residual ⚡️implementations dirs the same way an independent walk does", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const perPluginArea = burndown.implDirsByArea["clean"] ?? 0;
    expect(perPluginArea).toBe(countImplDirs(join(root, "✏️s/🔌️plugins")));
    expect(burndown.implDirsTotal).toBeGreaterThanOrEqual(perPluginArea);
  }, 600_000);

  test.if(testLevelAtLeast("long"))("owner residual counts add up to their area total", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const owners = discoverOwners(root).filter((owner) => owner.area === "mixed");
    const summed = owners.reduce((total, owner) => total + owner.residualImplDirs, 0);
    expect(summed).toBe(burndown.implDirsByArea["mixed"] ?? 0);
    expect(burndown.mixedOwners.every((owner) => owner.residualImplDirs > 0 || owner.entryFilesAtOwnerRoot.length > 0)).toBe(true);
    expect(burndown.cleanOwners + burndown.mixedOwners.length).toBe(burndown.ownersTotal);
  }, 600_000);

  test.if(testLevelAtLeast("long"))("markerless package manifests stay visible even where they are a silent skip", () => {
    const root = getWorkspaceRoot();
    const burndown = discoverBurndown(root);
    const catalogPaths = new Set(discoverPackages(root).map((pkg) => pkg.manifestPath));
    expect(burndown.unmarkedManifests.every((manifest) => !catalogPaths.has(manifest.path))).toBe(true);
    expect(burndown.packagingViolations.every((violation) => violation.path.includes("📦️packages/"))).toBe(true);
  });

  test.if(testLevelAtLeast("exhaustive"))("discoverPackageProblems promotes packaging violations into census problems", () => {
    const root = getWorkspaceRoot();
    clearDiscoveryCache();
    const problems = discoverPackageProblems(root);
    expect(problems.some((problem) => problem.kind === "packaging-violation")).toBe(true);
    expect(problems.every((problem) => problem.path.length > 0 && problem.message.includes(problem.path))).toBe(true);
    const censusProblems = buildSemanticCensus(root).problems.filter((problem) => problem.code === "packaging-violation");
    expect(censusProblems.length).toBe(problems.filter((problem) => problem.kind === "packaging-violation").length);
  }, 1_800_000);

  test.if(testLevelAtLeast("long"))("clearDiscoveryCache forces a fresh walk", () => {
    const root = getWorkspaceRoot();
    const first = discoverPackages(root);
    clearDiscoveryCache();
    expect(discoverPackages(root).map((pkg) => pkg.manifestPath)).toEqual(first.map((pkg) => pkg.manifestPath));
  }, 600_000);
});

describe("computeWorkspaces", () => {
  /** 🧪️ Models independent packages, explicitly exported payloads, and excluded trees. */
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
    write("c/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/c-rs", exports: "./pkg/🟨️.js" }));
    write("c/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"c\"\n");
    write("c/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/c-rs" }));
    write("c/📦️packages/🦀️rust/pkg/🟨️.js", "export {};\n");
    write("d/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"d\"\n");
    write("d/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/d-wasm" }));
    write("f/📦️packages/🦀️rust/package.json", JSON.stringify({ name: "@t/f-rs" }));
    write("f/📦️packages/🦀️rust/Cargo.toml", "[package]\nname = \"f\"\n");
    write("f/📦️packages/🦀️rust/pkg/package.json", JSON.stringify({ name: "@t/f-wasm" }));
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

  test("an explicitly exported same-name package payload is excluded", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("c/📦️packages/🦀️rust");
      expect(result).not.toContain("c/📦️packages/🦀️rust/pkg");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a differently named nested package remains independent", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("d/📦️packages/🦀️rust/pkg");
      expect(result).toContain("f/📦️packages/🦀️rust");
      expect(result).toContain("f/📦️packages/🦀️rust/pkg");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a pkg directory has ordinary membership independently of Cargo", () => {
    const root = buildFixture();
    try {
      const result = computeWorkspaces(root);
      expect(result).toContain("e/strayroot/pkg");
      expect(result).toContain("e/strayroot/pkg/junk");
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

  test.if(testLevelAtLeast("long"))("against the real repo: packages are physical, unambiguous, and preserve their owned payload boundaries", () => {
    const root = getWorkspaceRoot();
    const result = computeWorkspaces(root);
    for (const entry of result) expect(existsSync(join(root, entry, "package.json"))).toBe(true);
    expect(new Set(result).size).toBe(result.length);
    expect(result).toContain("🧰️framework/📦️packages/🟦️typescript");
    expect(result).toContain("✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript");
    expect(result).toContain("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust");
    expect(result).not.toContain("🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg");
  }, 600_000);
});

describe("policyPluginDependencyParityBreaches", () => {
  test("excludes nested extensions from parent plugin walk while checking extensions independently", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-hub-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n[package.metadata.semio]\nrole = "extension"\nextends = "cad"\n[dependencies]\nsemio-hub-cad = { path = "../../../📦️packages/🦀️rust" }\n`);
      writeFileSync(join(extDir, "🦀️.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad" && b.priority === "high")).toEqual([]);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building")).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("flags a builder-declared runtime dependency that the crate's semio metadata never declares", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const pluginDir = join(root, "✏️s", "🔌️plugins", "📐️cad");
      const extDir = join(pluginDir, "🧩️extensions", "🏢️aec-building");
      mkdirSync(join(pluginDir, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(extDir, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(pluginDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-hub-cad"\n`);
      writeFileSync(join(pluginDir, "🦀️.rs"), `pub struct CadPlugin;\n`);
      writeFileSync(join(extDir, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-s-plugin-cad-aec-building"\n`);
      writeFileSync(join(extDir, "🦀️.rs"), `pub fn configure() { b.depends_on("cad", "^1.0.0"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad")).toEqual([]);
      const extBreaches = breaches.filter((b) => b.scope === "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building");
      expect(extBreaches.length).toBe(1);
      expect(extBreaches[0]!.priority).toBe("high");
      expect(extBreaches[0]!.id).toBe("plugin-dependency-undeclared-metadata-✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building-cad");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a sibling Cargo library link is not a runtime dependency, but a metadata depends-on the builder omits is", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-plugin-parity-policy-"));
    try {
      const linker = join(root, "✏️s", "🔌️plugins", "🖨️raster");
      const declarer = join(root, "✏️s", "🔌️plugins", "🎪️demonstrator");
      mkdirSync(join(linker, "📦️packages", "🦀️rust"), { recursive: true });
      mkdirSync(join(declarer, "📦️packages", "🦀️rust"), { recursive: true });
      writeFileSync(join(linker, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-hub-raster"\n[package.metadata.semio]\nrole = "plugin"\n[dependencies]\nsemio-s-plugin-stdio = { path = "../../🗄️stdio/📦️packages/🦀️rust" }\n`);
      writeFileSync(join(linker, "🦀️.rs"), `pub fn plugin() { Plugin::builder("raster"); }\n`);
      writeFileSync(join(declarer, "📦️packages", "🦀️rust", "Cargo.toml"), `[package]\nname = "semio-hub-demonstrator"\n[package.metadata.semio]\nrole = "plugin"\ndepends-on = ["cad"]\n`);
      writeFileSync(join(declarer, "🦀️.rs"), `pub fn plugin() { Plugin::builder("demonstrator"); }\n`);

      const breaches = policyPluginDependencyParityBreaches(root);
      expect(breaches.filter((b) => b.scope === "✏️s/🔌️plugins/🖨️raster")).toEqual([]);
      const declarerBreaches = breaches.filter((b) => b.scope === "✏️s/🔌️plugins/🎪️demonstrator");
      expect(declarerBreaches.map((b) => b.id)).toEqual(["plugin-dependency-undeclared-runtime-✏️s/🔌️plugins/🎪️demonstrator-cad"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("🔒️ policy allowlist", () => {
  const repoRoot = getWorkspaceRoot();

  test.if(testLevelAtLeast("long"))("an area contributes its own policy exemptions, and the router merges them by discovery", () => {
    const merged = policyDiscoveredAllowlist(repoRoot, "semantic-vocabulary");
    expect(merged.size).toBeGreaterThan(0);
    // 🧭️Every entry must live under the area that contributed it — an exemption never outlives the
    // code it exempts, which is the whole point of moving it out of the root router.
    for (const entry of merged) expect(entry.length).toBeGreaterThan(0);
    expect(policyDiscoveredAllowlist(repoRoot, "no-such-rule-key").size).toBe(0);
  }, 600_000);
});
