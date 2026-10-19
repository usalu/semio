/** 🗺️ The stdio text walk, the dwg artifact ownership check and the home io surface script of the hub stdio package. */
import { buildBudgetMs } from "@semio-tech/process/budget";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { devToolingEnv, runCmd } from "@semio-tech/process/execution";
import { BundleScript } from "@semio-tech/process/routing";
import { prepareStdioComposition } from "../../../🧩️composition/🟦️.ts";
import { ARTIFACT_OWNER, COMPONENT_MANIFEST } from "../🧮️component-constants/🟦️.ts";
import { compileStdioScopeExport } from "../🧾️catalog-artifact-io/🟦️.ts";

export function stdioWalkText(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && ["target", "node_modules", "🗑️generated"].includes(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) stdioWalkText(path, files);
    else if (/[.](?:rs|ts|js|json|feature|semio)$/u.test(entry.name) || ["Cargo.toml", "package.json"].includes(entry.name)) files.push(path);
  }
  return files;
}

function readScannedText(path: string): string | undefined {
  try {
    return readFileSync(path, "utf8");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
}

/** 🖊️ Checks codec ownership against an independent JSON Schema oracle and the framework source tree. */
async function runDwgArtifactOwnership(root: string, repoRoot: string): Promise<void> {
  
  const fixtureRoot = join(root, "../../🧪️tests/🧫️fixtures/🖊️dwg-artifact-ownership");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as { cases: { path: string; allowed: boolean }[]; forbiddenFrameworkPatterns: string[] };
  const validate = await compileStdioScopeExport(join(root, "../.."), "StdioDwgArtifactOwnership");
  const ownsCodec = (path: string): boolean => {
    const parts = path.split("/");
    return parts[0] === "✏️s" && parts[1] === "🔌️plugins" && Boolean(parts[2]) && Boolean(parts[4]) && parts[3] === "🗿️artifacts" && (parts[4] === "🖊️dwg" || parts.slice(5).includes("🚪️io"));
  };
  for (const test of fixture.cases) {
    assert.equal(ownsCodec(test.path), test.allowed, test.path);
    assert.equal(validate(test.path), test.allowed, `AJV ownership oracle: ${test.path}`);
  }
  const forbidden = fixture.forbiddenFrameworkPatterns.map((pattern) => new RegExp(pattern, "u"));
  const sources = ["🧰️framework", "✏️s/🔨️modules"].flatMap((directory) => stdioWalkText(join(repoRoot, directory)));
  const failures = sources.filter((path) => /[.](?:rs|ts|js|json)$|Cargo[.]toml$/u.test(path) && !["/🧩️extension-modules/", "/🔌️plugin-modules/", "/📤️distribution/", "/dist/", "/🧫️fixtures/", "/🧪️tests/"].some((directory) => path.split(sep).join("/").includes(directory))).flatMap((path) => {
    const source = readScannedText(path)?.replace(/\/\*[\s\S]*?\*\/|^\s*\/\/.*$/gmu, "") ?? "";
    return forbidden.some((pattern) => pattern.test(source)) ? [relative(repoRoot, path)] : [];
  });
  assert.deepEqual(failures, [], "DWG codecs escaped artifact I/O ownership");
  const misplaced = stdioWalkText(join(repoRoot, "✏️s/🔌️plugins")).filter((path) => /[.](?:rs|ts)$/u.test(path) && path.split(sep).join("/").includes("/🗿️artifacts/") && !ownsCodec(relative(repoRoot, path).split(sep).join("/"))).filter((path) => {
    const source = readScannedText(path)?.replace(/\/\*[\s\S]*?\*\/|^\s*\/\/.*$/gmu, "") ?? "";
    return /(?:semio_hub_stdio|crate)::artifacts::dwg::|\bDwg(?:Drawing|Geometry|Entity|Color|Importer|Exporter)\b|\b(?:export|import)_dwg(?:_sync)?\b/u.test(source);
  }).map((path) => relative(repoRoot, path));
  assert.deepEqual(misplaced, [], "Other artifact facets contain DWG codecs");
  console.log(`stdio-dwg-artifact-ownership: cases=${fixture.cases.length} AJV=1 framework=clean modules=clean artifact-facets=clean`);
}

type HomeIoSurfaceFixture = {
  readonly schema: "semio.stdio.home-io-surface/v3";
  readonly catalogPackage: "semio-hub-stdio-catalog";
  readonly libraryPackage: "semio-hub-stdio";
  readonly componentPackage: "semio-hub-stdio-component";
  readonly componentAppAssembly: "component-app-assembly";
  readonly directArtifacts: readonly ["csv", "json", "xlsx", "zip"];
  readonly sharedCodecs: readonly ["binary", "deflate", "txt", "xml"];
  readonly fullArtifactCount: number;
  readonly nativeCodecCount: number;
};

function cargoTomlFiles(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && [".git", ".nx", ".🧬semio", "node_modules", "target", "🗑️generated"].includes(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) cargoTomlFiles(path, files);
    else if (entry.name === "Cargo.toml") files.push(path);
  }
  return files;
}

/** 🏠️ Proves Space selects only the four Home I/O families plus their exact shared codec closure. */
export class HomeIoSurfaceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    prepareStdioComposition(this.repoRoot, resolve(this.root, "../.."));
    await runDwgArtifactOwnership(this.root, this.repoRoot);
    const mode = segments[0] ?? "source";
    if (mode !== "source" && mode !== "native") throw new Error("home-io-surface expects source|native");
    
    const fixtureRoot = join(this.root, "../../🧪️tests/🧫️fixtures/🏠️home-io-surface");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as HomeIoSurfaceFixture;
    const manifest = readFileSync(join(this.root, "Cargo.toml"), "utf8");
    const { parse: parseToml } = await import("@iarna/toml");
    const firstParty = Bun.TOML.parse(manifest) as { package: { name: string }; lib: Record<string, unknown>; features?: Record<string, string[]>; dependencies: Record<string, unknown> };
    assert.deepEqual(firstParty, parseToml(manifest) as unknown);
    assert.equal(firstParty.package.name, fixture.libraryPackage);
    assert(!firstParty.features, "the Stdio library declares features, so it compiles once per consumer feature set");
    assert(!Object.hasOwn(firstParty.lib, "crate-type"), "the Stdio library declares crate types; a library is rlib-only and its cdylib is the component crate");
    const artifactPrefix = "semio-s-artifact-stdio-";
    const componentSource = readFileSync(join(this.root, COMPONENT_MANIFEST), "utf8"), component = Bun.TOML.parse(componentSource) as typeof firstParty;
    assert.deepEqual(component, parseToml(componentSource) as unknown);
    assert.equal(component.package.name, fixture.componentPackage);
    assert.deepEqual(component.lib["crate-type"], ["cdylib"]);
    assert.deepEqual(Object.keys(component.dependencies).sort(), [fixture.libraryPackage, "semio-framework-plugin"].sort(), "the Stdio component crate links anything but its library and the plugin entry macro");
    assert(!component.features, "the Stdio component crate declares features");
    assert.match(readFileSync(join(this.root, "../../🧩️component/🦀️.rs"), "utf8"), /^semio_framework_plugin::plugin_exports!\(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio::plugin, semio_hub_stdio::plugin::StdioApps\);$/mu);
    const selected:readonly string[] = [...fixture.directArtifacts, ...fixture.sharedCodecs].sort();
    const catalogManifestPath = join(this.root, "../../🔌️plugin/📇️catalog/📦️packages/🦀️rust/Cargo.toml");
    const catalogSource = readFileSync(catalogManifestPath, "utf8"), catalogCargo = Bun.TOML.parse(catalogSource) as any;
    assert.deepEqual(catalogCargo, parseToml(catalogSource) as any);
    assert.equal(catalogCargo.package.name, fixture.catalogPackage);
    assert(!catalogCargo.features, "the Stdio artifact catalog declares features, so its library compiles once per consumer feature set");
    assert.equal(Object.keys(catalogCargo.dependencies).filter((name) => name.startsWith(artifactPrefix) && name !== `${artifactPrefix}contract`).length, fixture.fullArtifactCount);
    assert(Object.hasOwn(firstParty.dependencies, fixture.catalogPackage), "the Stdio component no longer assembles through its artifact catalog crate");
    const root = readFileSync(join(this.root, "../../🦀️.rs"), "utf8");
    assert.match(root, /^#\[path = "🔌️plugin\/🦀️\.rs"\]\s+pub mod plugin;$/mu);
    assert(!root.includes("📇️catalog"), "Stdio composition remounted its artifact catalog instead of linking the catalog crate");
    assert(!root.includes("pub mod artifacts"), "Stdio composition remounted artifact implementations");
    const spaceManifestPath = resolve(this.repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/Cargo.toml");
    const spaceManifest = readFileSync(spaceManifestPath, "utf8");
    assert(!spaceManifest.includes("semio-hub-stdio"), "Space retains the Stdio composition dependency");
    assert.match(spaceManifest, /^semio-framework-os\s*=\s*\{[^\n}]*\}$/mu);
    assert(!/^semio-framework-os\s*=[^\n]*features/mu.test(spaceManifest), "Space selects an OS host feature set");
    const workspace = parseToml(readFileSync(join(this.repoRoot, "Cargo.toml"), "utf8")) as unknown as { workspace: { dependencies: Record<string, { package?: string } | string> } };
    const dependencyNames = (cargo: any): string[] => [cargo, ...Object.values(cargo.target ?? {})].flatMap((table: any) => Object.entries(table.dependencies ?? {}).map(([alias, value]: [string, any]) => {
      const declaration = value?.workspace === true ? workspace.workspace.dependencies[alias] : value;
      return declaration && typeof declaration === "object" && declaration.package ? declaration.package : alias;
    }));
    const artifactManifests = new Map<string, any>();
    for (const path of cargoTomlFiles(resolve(this.repoRoot, ARTIFACT_OWNER, "🗿️artifacts"))) {
      const cargo = parseToml(readFileSync(path, "utf8")) as any;
      if (cargo.package?.name?.startsWith(artifactPrefix)) artifactManifests.set(cargo.package.name, cargo);
    }
    for (const artifact of [...fixture.directArtifacts, ...fixture.sharedCodecs]) assert(!Object.hasOwn(artifactManifests.get(`${artifactPrefix}${artifact}`)?.features ?? {}, fixture.componentAppAssembly), `${artifact} keeps a per-consumer app feature, so its library compiles once per consumer feature set instead of once per profile`);
    const homeClosure = new Set<string>();
    const visitArtifact = (name: string): void => {
      if (name === `${artifactPrefix}contract`) return;
      if (!name.startsWith(artifactPrefix) || homeClosure.has(name)) return;
      homeClosure.add(name);
      const cargo = artifactManifests.get(name);
      assert(cargo, `Home I/O dependency has no artifact package ${name}`);
      for (const dependency of dependencyNames(cargo)) visitArtifact(dependency);
    };
    const spaceCargo = parseToml(spaceManifest) as any;
    const spaceDirect = dependencyNames(spaceCargo).filter((name) => name.startsWith(artifactPrefix));
    for (const name of spaceDirect) visitArtifact(name);
    assert.deepEqual([...homeClosure].map((name) => name.slice(artifactPrefix.length)).sort(), selected);
    for (const artifact of fixture.directArtifacts) assert(spaceDirect.includes(`${artifactPrefix}${artifact}`), `Space misses direct Home I/O artifact ${artifact}`);
    const consumers = cargoTomlFiles(this.repoRoot).filter((path) => path !== join(this.root, "Cargo.toml") && readFileSync(path, "utf8").includes("semio-hub-stdio = {"));
    assert(!consumers.includes(spaceManifestPath));
    const catalogConsumers = cargoTomlFiles(this.repoRoot).filter((path) => readFileSync(path, "utf8").includes(`${fixture.catalogPackage} = {`));
    const hubManifestPath = resolve(this.repoRoot, "🌎️hub/📦️packages/🦀️rust/Cargo.toml");
    const osHostManifestPath = resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml");
    assert(!consumers.includes(hubManifestPath), "native Hub links the Stdio guest component instead of its artifact catalog");
    assert(catalogConsumers.includes(hubManifestPath) && !catalogConsumers.includes(spaceManifestPath));
    assert(readFileSync(hubManifestPath, "utf8").includes(`"dep:${fixture.catalogPackage}"`), "native Hub provider must select the concrete Stdio artifact catalog");
    const neutralManifest = Bun.TOML.parse(readFileSync(resolve(this.repoRoot, ARTIFACT_OWNER, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as any;
    assert(!neutralManifest.package.metadata.component, "general Stdio assembly advertises a concrete component");
    assert(!dependencyNames(neutralManifest).some((name) => name.startsWith(artifactPrefix) && name !== `${artifactPrefix}contract`), "general Stdio assembly statically depends on concrete artifacts");
    assert(!Object.values(neutralManifest.features ?? {}).flat().some((activation: any) => String(activation).replace(/^dep:/u, "").startsWith(artifactPrefix)), "general Stdio assembly contains a concrete artifact roster");
    const osHostManifest = readFileSync(osHostManifestPath, "utf8");
    const osHostCargo = parseToml(osHostManifest) as any;
    assert(!osHostCargo.features, "OS host declares features, so hub components compile more than one OS host unit");
    assert(osHostCargo.target['cfg(not(all(target_arch = "wasm32", target_env = "p2")))'].dependencies["semio-framework-os-kernel"].features.includes("sync"), "OS host lost the native kernel sync transport");
    assert(!consumers.includes(osHostManifestPath), "OS host must not select domain artifact codecs");
    assert(!osHostManifest.includes("semio-hub-stdio"), "OS host depends on Stdio");
    const osHostRoot = readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs"), "utf8");
    for (const module of ["host", "backbone"]) assert.match(osHostRoot, new RegExp(`^pub mod ${module};`, "mu"));
    for (const [module, producer] of [["codec_abi", "codec_abi"], ["instance", "instance"], ["registry", "registry"], ["workflow", "workflow"], ["media_export_raster", "media_raster"]] as const) assert.match(osHostRoot, new RegExp(`^pub use semio_framework_os_host_${producer} as ${module};`, "mu"));
    const spaceRoot = resolve(this.repoRoot, "✏️s/🔌️plugins/🪐️space");
    const productionRefs = stdioWalkText(spaceRoot)
      .filter((path) => path.endsWith(".rs") && path.includes("/🗿️artifacts/🏠️home/") && !path.includes("/🧪️tests/"))
      .flatMap((path) => [...readFileSync(path, "utf8").matchAll(/semio_s_artifact_stdio_([a-z0-9_]+)/gu)].map((match) => match[1]!));
    assert(productionRefs.length > 0 && productionRefs.every((artifact) => selected.includes(artifact)), "Space Home I/O uses an artifact outside its selected Cargo closure");
    console.log(`stdio-home-io-surface-oracle: AJV=1 TOML=bun+iarna library=${fixture.libraryPackage} component=${fixture.componentPackage} direct=${fixture.directArtifacts.length} shared=${fixture.sharedCodecs.length} full=${fixture.fullArtifactCount} codecs=${fixture.nativeCodecCount} consumers=${consumers.length} catalog-consumers=${catalogConsumers.length}`);
    if (mode === "native") {
      const options = { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() };
      runCmd("cargo", ["check", "--manifest-path", catalogManifestPath, "-p", fixture.catalogPackage, "--lib", "--target", "wasm32-wasip2", "--message-format=short"], options);
      runCmd("cargo", ["check", "--manifest-path", resolve(this.repoRoot, "🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-hub-space", "--lib", "--target", "wasm32-wasip2", "--message-format=short"], options);
    }
  }
}
