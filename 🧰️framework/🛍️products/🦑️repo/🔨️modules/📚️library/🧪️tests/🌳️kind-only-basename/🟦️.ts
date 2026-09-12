import { describe, expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { tmpdir } from "node:os";
import Ajv from "ajv";
import fastGlob from "fast-glob";
import * as TOML from "@iarna/toml";
import { cargoPackageRootBuildScriptPath, clearDiscoveryCache, discoverPackageProblems, discoverPackages, implementationLeafBasenameFinding, taxonomyImplementationFilesystemFindings, type Taxonomy } from "../../🔍️discovery/🟦️.ts";

type Finding = Readonly<{ path: string; breachId: string; fileKindId: string | null; expectedBasename: string | null }>;
type Fixture = Readonly<{
  fileKinds: readonly Readonly<{ id: string; emoji: string; extensionChains: readonly string[]; role: string; implementation: boolean }>[];
  externalContracts: readonly Readonly<{ id: string; path: string; scope: string }>[];
  files: readonly Readonly<{ path: string; content: string }>[];
  expectedFindings: readonly Finding[];
  topology: Readonly<{ packagesDirectory: string; targetsDirectory: string; packageLanguageDirectories: readonly string[]; semanticExecutionCollections: readonly string[]; obsoleteExecutionCollections: readonly string[] }>;
}>;

const owner = join(import.meta.dir, "../../🧫️fixtures/🌳️kind-only-basename");
const fixture = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🌳️kind-only-basename/🔣️.json"), "utf8"));
const taxonomy = JSON.parse(readFileSync(join(import.meta.dir, "../../🔣️taxonomy.json"), "utf8")) as Taxonomy;
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const sorted = (rows: readonly Finding[]): Finding[] => [...rows].sort((left, right) => Buffer.from(`${left.path}\0${left.breachId}`).compare(Buffer.from(`${right.path}\0${right.breachId}`)));

function materialize(): string {
  const root = mkdtempSync(join(tmpdir(), "semio-kind-only-basename-"));
  for (const file of fixture.files) {
    const path = join(root, file.path);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, file.content);
  }
  return root;
}

async function independentOracle(root: string): Promise<Finding[]> {
  const entries = (await fastGlob("**/*", { cwd: root, dot: true, onlyFiles: false, followSymbolicLinks: false })).map((path) => path.normalize("NFC")).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  const files = entries.filter((path) => lstatSync(join(root, path)).isFile());
  const fixed = new Set<string>();
  for (const manifest of files.filter((path) => basename(path) === "Cargo.toml")) try {
    const parsed = TOML.parse(readFileSync(join(root, manifest), "utf8")) as { readonly package?: { readonly name?: unknown; readonly build?: unknown } };
    if (typeof parsed.package?.name !== "string" || parsed.package.build !== undefined && parsed.package.build !== "build.rs") continue;
    const buildScript = `${dirname(manifest)}/build.rs`;
    if (files.includes(buildScript)) fixed.add(buildScript);
  } catch {}
  const ajv = new Ajv({ strict: true });
  const predicates = new Map<string, ReturnType<Ajv["compile"]>>();
  const findings: Finding[] = [];
  for (const path of files) {
    const name = basename(path).toLowerCase();
    const matches = fixture.fileKinds.flatMap((kind) => kind.extensionChains.filter((extension) => name.endsWith(extension)).map((extension) => ({ kind, extension })));
    const longest = Math.max(0, ...matches.map((match) => match.extension.length));
    const resolved = matches.filter((match) => match.extension.length === longest);
    if (resolved.length !== 1 || !resolved[0]!.kind.implementation || fixed.has(path)) continue;
    const expectedBasename = `${resolved[0]!.kind.emoji}${resolved[0]!.extension}`;
    let validate = predicates.get(expectedBasename);
    if (!validate) {
      validate = ajv.compile({ type: "string", const: expectedBasename });
      predicates.set(expectedBasename, validate);
    }
    if (!validate(basename(path))) findings.push({ path, breachId: "taxonomy/kind-only-basename", fileKindId: resolved[0]!.kind.id, expectedBasename });
  }
  const languages = new Set(fixture.topology.packageLanguageDirectories);
  for (const path of entries.filter((entry) => lstatSync(join(root, entry)).isDirectory())) {
    const segments = path.split("/");
    const packageIndex = segments.indexOf(fixture.topology.packagesDirectory);
    if (packageIndex < 0 || !languages.has(segments[packageIndex + 1] ?? "") || segments[packageIndex + 2] !== fixture.topology.targetsDirectory) continue;
    findings.push({ path: segments.slice(0, packageIndex + 3).join("/"), breachId: "taxonomy/target-inside-package-boundary", fileKindId: null, expectedBasename: null });
  }
  return sorted([...new Map(findings.map((finding) => [`${finding.path}\0${finding.breachId}`, finding])).values()]);
}

describe("kind-only implementation leaf taxonomy", () => {
  test("validates the portable filesystem contract and binds its kind records to taxonomy", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    for (const kind of fixture.fileKinds) {
      expect(taxonomy.fileKinds[kind.id]).toEqual({ emoji: kind.emoji, extensionChains: kind.extensionChains, role: kind.role });
      expect(taxonomy.implementationLeafPolicy.roles.includes(taxonomy.fileKinds[kind.id]!.role) || taxonomy.implementationLeafPolicy.fileKindIds.includes(kind.id)).toBe(kind.implementation);
    }
    expect(taxonomy.packagesDirName).toBe(fixture.topology.packagesDirectory);
    expect(taxonomy.targetsDirName).toBe(fixture.topology.targetsDirectory);
    expect(Object.keys(taxonomy.ecosystems)).toEqual(fixture.topology.packageLanguageDirectories);
    expect(taxonomy.fixedFilenameContracts[fixture.externalContracts[0]!.id]?.scope).toEqual({ kind: "package-root", ecosystemId: "🦀️rust" });
  });

  test("matches an independent fast-glob census and Ajv filename predicates", async () => {
    const root = materialize();
    try {
      const expected = sorted(fixture.expectedFindings);
      const progress: string[] = [];
      const observed = sorted((await taxonomyImplementationFilesystemFindings(root, taxonomy, { yieldEvery: 2, onProgress: (row) => progress.push(row.phase) })).map(({ path, breachId, fileKindId, expectedBasename }) => ({ path, breachId, fileKindId, expectedBasename })));
      expect(observed).toEqual(expected);
      expect(await independentOracle(root)).toEqual(expected);
      expect(progress).toContain("complete");
      console.info("[DEBUG] Kind-only taxonomy fixture", JSON.stringify({ files: fixture.files.length, findings: observed.length }));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("cancels the focused census before filesystem traversal", async () => {
    const controller = new AbortController();
    controller.abort();
    await expect(taxonomyImplementationFilesystemFindings(import.meta.dir, taxonomy, { signal: controller.signal })).rejects.toMatchObject({ name: "AbortError" });
  });

  test("binds Cargo's conventional build filename to an active package manifest", () => {
    const manifests = [
      "[package]\nname = \"default-build\"\nversion = \"0.1.0\"\n",
      "[package]\nname = \"explicit-build\"\nversion = \"0.1.0\"\nbuild = \"build.rs\"\n",
      "[package]\nname = \"disabled-build\"\nversion = \"0.1.0\"\nbuild = false\n",
      "[package]\nname = \"custom-build\"\nversion = \"0.1.0\"\nbuild = \"🦀️.rs\"\n",
      "[workspace]\nmembers = []\n",
    ] as const;
    const oracle = manifests.map((source) => {
      const parsed = TOML.parse(source) as { readonly package?: { readonly name?: unknown; readonly build?: unknown } };
      return typeof parsed.package?.name === "string" && (parsed.package.build === undefined || parsed.package.build === "build.rs") ? "build.rs" : null;
    });
    expect(manifests.map(cargoPackageRootBuildScriptPath)).toEqual(oracle);
    expect(implementationLeafBasenameFinding("🧰️naked/📦️packages/🦀️rust/build.rs", taxonomy)).toMatchObject({ breachId: "taxonomy/kind-only-basename", expectedBasename: "🦀️.rs", exemptionAuthorityId: null });
  });

  test("derives every focused implementation command from the launch seed authority", () => {
    const expected = [
      ["🧹clean🧩️taxonomy🧪️kind-only-basename", "bun nx run @semio-tech/repo-lib:test-kind-only-basename"],
      ["📦️verify🧩️taxonomy🌳️implementation📋️report", "bun nx run workspace:verify-taxonomy-implementation-report"],
      ["📦️verify🧩️taxonomy🌳️implementation🚦️enforce", "bun nx run workspace:verify-taxonomy-implementation-enforce"],
    ] as const;
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const document = Bun.JSONC.parse(readFileSync(join(repoRoot, path), "utf8")) as { readonly configurations: readonly { readonly name?: string; readonly command?: string }[] };
      for (const [name, command] of expected) expect(document.configurations.filter((row) => row.name === name && row.command === command), `${path}: ${name}`).toHaveLength(1);
    }
  });

  test("discovers only target-first packages and rejects the inverse topology", () => {
    const root = materialize();
    try {
      clearDiscoveryCache();
      const packages = discoverPackages(root, taxonomy);
      expect(packages.find((entry) => entry.id === "fixture-wgpu")).toMatchObject({ ownerRel: "🧰️owner/🎯️targets/🧊️wgpu", target: "🧊️wgpu", packageRel: "🧰️owner/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" });
      expect(packages.find((entry) => entry.id === "fixture-react")).toMatchObject({ ownerRel: "🧰️web/🎯️targets/⚛️react", target: "⚛️react", packageRel: "🧰️web/🎯️targets/⚛️react/📦️packages/🟦️typescript" });
      expect(packages.some((entry) => entry.id === "inverse-wgpu")).toBe(false);
      expect(discoverPackageProblems(root, taxonomy)).toContainEqual({ kind: "target-inside-package-boundary", path: "🧰️inverse/📦️packages/🦀️rust/🎯️targets", message: '"🧰️inverse/📦️packages/🦀️rust/🎯️targets" places a target subtree inside a package boundary.' });
      console.info("[DEBUG] Target-first package discovery", JSON.stringify({ packages: packages.map((entry) => entry.id) }));
    } finally {
      clearDiscoveryCache();
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps generator, probe, and oracle implementations under semantic owners with ordinary packages", async () => {
    const collections = fixture.topology.semanticExecutionCollections.join(",");
    const obsolete = fixture.topology.obsoleteExecutionCollections.join(",");
    const entries = await fastGlob([`✏️s/**/{${collections}}/**/{Cargo.toml,*.rs}`, `✏️s/**/{${obsolete}}`], {
      cwd: repoRoot,
      onlyFiles: false,
      followSymbolicLinks: false,
      ignore: ["**/target/**"],
    });
    const manifests = entries.filter((path) => path.endsWith("/Cargo.toml")).sort();
    const sources = entries.filter((path) => path.endsWith(".rs")).sort();
    const canonicalSuffix = `${fixture.topology.packagesDirectory}/🦀️rust/Cargo.toml`;
    const noncanonicalManifests = manifests.filter((path) => {
      const segments = path.split("/");
      const ownerIndex = Math.max(...fixture.topology.semanticExecutionCollections.map((collection) => segments.lastIndexOf(collection)));
      return !segments.slice(ownerIndex + 1).join("/").endsWith(canonicalSuffix);
    });
    const obsoleteCollections = entries.filter((path) => fixture.topology.obsoleteExecutionCollections.includes(basename(path))).sort();
    expect(noncanonicalManifests).toEqual([]);
    expect(obsoleteCollections).toEqual([]);
    expect(sources.filter((path) => basename(path) !== "🦀️.rs")).toEqual([]);
    for (const manifest of manifests) {
      const parsed = TOML.parse(readFileSync(join(repoRoot, manifest), "utf8")) as { readonly lib?: { readonly path?: unknown }; readonly bin?: readonly { readonly path?: unknown }[] };
      const targets = [parsed.lib?.path, ...(parsed.bin ?? []).map((target) => target.path)].filter((path): path is string => typeof path === "string");
      expect(targets.length, manifest).toBeGreaterThan(0);
      for (const target of targets) {
        const targetPath = resolve(repoRoot, dirname(manifest), target);
        expect(basename(targetPath), `${manifest}: ${target}`).toBe("🦀️.rs");
        expect(relative(resolve(repoRoot, dirname(manifest)), targetPath), `${manifest}: ${target}`).toMatch(/^\.\.(?:[\\/]|$)/u);
        expect(existsSync(targetPath), `${manifest}: ${target}`).toBe(true);
      }
    }
    console.info("[DEBUG] Semantic generator topology", JSON.stringify({ manifests: manifests.length, sources: sources.length }));
  }, 30_000);
});
