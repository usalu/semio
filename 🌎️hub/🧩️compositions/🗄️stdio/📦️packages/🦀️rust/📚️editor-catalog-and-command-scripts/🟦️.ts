/** 📚️ The editor catalog contract, the stdio package manifests and the mutation inventory, composition, test, editor component check and bench scripts of the hub stdio package. */
import { buildRepositoryCargoArtifacts } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { withCapturedCargoArtifactsTaskV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { completeCargoPreparationObservationV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import assert from "node:assert/strict";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { devToolingEnv } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import { resolveWorkspaceBin, runCmd } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { BundleScript } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { cargoTargetDirectory, cargoBuildDirectory } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { prepareStdioComposition } from "../../../🧩️composition/🟦️.ts";
import { runCompositionContributionChecks } from "../../../🧩️composition/📋️contract/🧪️tests/🟦️.ts";
import { ARTIFACT_OWNER, CATALOG_DEADLINE_MS, COMPONENT_PROFILE, PACKAGE_NAME } from "../🧮️component-constants/🟦️.ts";
import { assertComponentizableCore, assertContainedBounded, assertControlled, catalogControlV1, copyCatalogArtifact, pathIsWithin, prepareReleaseComponent, runControlled } from "../🧾️catalog-artifact-io/🟦️.ts";
import { runCatalogRootContractTests } from "../🔬️catalog-contract-and-flow-decode/🟦️.ts";

/** 🧪️ Checks the editor catalogue with independent native and manifest oracles: every stdio editor ships in
 * exactly one stdio package (the stdio component or one `🧩️extensions` family), the packages together ship every catalogue
 * format, and every editor owns exactly one launchable playground row across them. */
async function testEditorCatalogContract(packageRoot: string): Promise<void> {
  const root = resolve(packageRoot, "../..");
  prepareStdioComposition(resolve(root, "../../.."), root);
  const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🧫️fixtures/✏️editor-catalog/🔣️.json"), "utf8")) as { editorCount: number; formatCount: number; editorApps: string[]; actions: { id: string }[]; deployedComponents: { id: string; package: string; path: string; role: string; componentKind: string }[] };
  const { parse: parseToml } = await import("@iarna/toml");
  for (const row of fixture.deployedComponents) {
    const path = resolve(root, "../../..", row.path);
    const text = readFileSync(path, "utf8");
    const actual = Bun.TOML.parse(text) as any;
    if (!isDeepStrictEqual(actual, parseToml(text))) throw new Error(`${row.id}: independent manifest oracle differs`);
    if (actual.package.name !== row.package || actual.package.metadata.component.package !== `semio:${row.id}` || actual.package.metadata.semio.role !== row.role || actual.package.metadata.semio["component-kind"] !== row.componentKind) throw new Error(`${row.id}: deployed component ownership differs`);
    if (row.id !== "stdio" && existsSync(resolve(root, "../../..", row.path.replace("🌎️hub/🧩️compositions/🗄️stdio", ARTIFACT_OWNER)))) throw new Error(`${row.id}: prior concrete deployment owner remains`);
  }
  if (new Set(fixture.actions.map((action) => action.id)).size !== fixture.actions.length) throw new Error("editor catalogue repeats an edit action");
  const rust = readFileSync(join(root, "🤖️generated/🧪️editor-laws/🦀️.rs"), "utf8");
  const roots = [...rust.matchAll(/^    \([a-z0-9_]+, [a-z0-9_]+, semio_s_artifact_stdio_/gm)].length;
  if (roots !== fixture.editorCount) throw new Error(`editor catalogue expects ${fixture.editorCount} editors; native acceptance covers ${roots}`);
  const formats = new Map<string, string>();
  const apps = new Map<string, string>();
  for (const { id, assembly, manifest } of stdioPackageManifests(root)) {
    for (const format of stdioPackageEditorFormats(readFileSync(assembly, "utf8"))) {
      if (formats.has(format)) throw new Error(`${format} editors ship in both ${formats.get(format)} and ${id}`);
      formats.set(format, id);
    }
    for (const row of manifest.package.metadata.semio.playground ?? []) {
      if (apps.has(row.app)) throw new Error(`${row.app} has playground rows in both ${apps.get(row.app)} and ${id}`);
      apps.set(row.app, id);
    }
  }
  if (formats.size !== fixture.formatCount) throw new Error(`the stdio packages ship editors for ${formats.size}/${fixture.formatCount} formats`);
  if (fixture.editorApps.length !== fixture.editorCount) throw new Error("editor fixture count differs from its app identities");
  if (!isDeepStrictEqual([...apps.keys()].sort(), [...fixture.editorApps].sort())) throw new Error("each editor needs exactly one launchable playground across the stdio packages");
  console.log(`🧾️ editor catalogue fixture validated: ${roots} editors in ${new Set(apps.values()).size} packages, ${fixture.actions.length} edit operations`);
}

/** 📦️ Every stdio package manifest, parsed by Bun's TOML reader independently of the Rust census law's row scan, with its
 * generated plugin assembly source: the stdio component's own and one per family component under `🧩️extensions`. */
function stdioPackageManifests(root: string): { readonly id: string; readonly path: string; readonly assembly: string; readonly manifest: StdioPackageManifest }[] {
  const families = readdirSync(resolve(root, "🧩️extensions"), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => resolve(root, "🧩️extensions", entry.name, "🧩️component/📦️packages/🦀️rust/Cargo.toml"));
  return [join(root, "🧩️component/📦️packages/🦀️rust/Cargo.toml"), ...families.sort()].map((path) => {
    const manifest = Bun.TOML.parse(readFileSync(path, "utf8")) as StdioPackageManifest;
    return { id: manifest.package.metadata.component.package.slice("semio:".length), path, assembly: path === join(root, "🧩️component/📦️packages/🦀️rust/Cargo.toml") ? join(root, "🔌️plugin/🦀️.rs") : resolve(dirname(path), "../../../🦀️.rs"), manifest };
  });
}

/** 🧩️ The stdio formats whose editors one package's generated plugin assembly registers (`builder.editor::<semio_s_artifact_stdio_<format>::…>`). */
function stdioPackageEditorFormats(assembly: string): readonly string[] {
  return [...new Set([...assembly.matchAll(/builder\.editor::<semio_s_artifact_stdio_([a-z0-9]+)::/gu)].map((match) => match[1]!))];
}

/** 📜️ The Cargo manifest fields the editor catalogue contract reads from a stdio package. */
type StdioPackageManifest = {
  readonly features?: Record<string, string[]>;
  readonly dependencies?: Record<string, string | { readonly features?: readonly string[] }>;
  readonly package: { readonly name: string; readonly metadata: { readonly component: { readonly package: string }; readonly semio: { readonly playground?: readonly { readonly app: string }[] } } };
};

export class MutationInventoryScript extends BundleScript {
  run(args: string[]): void {
    runCmd(process.execPath, [join(this.root, "../../🏭️bridge/📜️script.ts"), ...args], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

export class CompositionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "prepare") {
      const result = prepareStdioComposition(this.repoRoot, resolve(this.root, "../.."));
      console.log(`stdio composition: contributions=${result.contributions} apps=${result.apps} receipts=${result.receipts}`);
      completeCargoPreparationObservationV1();
      return;
    }
    if (segments.length === 1 && segments[0] === "contribution-check") {
      console.log(`stdio composition contribution contract: checks=${runCompositionContributionChecks()}`);
      return;
    }
    if (segments.length !== 2 || !["removal-check", "removal-test-check"].includes(segments[0]!) || !isAbsolute(segments[1]!)) throw new Error("composition prepare|contribution-check|removal-check|removal-test-check <absolute-snapshot>");
    const tests = segments[0] === "removal-test-check";
    const snapshot = realpathSync(segments[1]!), artifact = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi";
    const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!output || snapshot === realpathSync(this.repoRoot) || !pathIsWithin(realpathSync(resolve(this.repoRoot, output)), snapshot)) throw new Error("removal-check requires a copied workspace inside SEMIO_TEST_ARTIFACT_DIR");
    const manifest = join(snapshot, relative(this.repoRoot, join(this.root, "Cargo.toml")));
    assertContainedBounded(snapshot, manifest, "retained Stdio parent manifest");
    if (existsSync(join(snapshot, artifact))) throw new Error("removal-check requires AVI to be physically absent");
    const authorityFiles = ["📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔣️.json", "📜️artifact-definition.json"];
    const scan = (root: string, expected: readonly string[] = []) => {
      let directory = root;
      for (const part of "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts".split("/")) {
        directory = join(directory, part);
        const info = lstatSync(directory);
        if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("Parent removal artifact ancestry is not a regular directory");
      }
      return readdirSync(directory, { withFileTypes: true }).filter(entry => entry.isSymbolicLink() || expected.includes(entry.name) || authorityFiles.some(path => {
      try { lstatSync(join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts", entry.name, path)); return true; } catch { return false; }
    })).map(entry => {
      const owner = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts", entry.name);
      const complete = authorityFiles.every(path => {
        let current = owner;
        try {
          for (const part of path.split("/")) { current = join(current, part); if (lstatSync(current).isSymbolicLink()) return false; }
          assertContainedBounded(root, current, "retained artifact authority");
          return true;
        } catch { return false; }
      });
      return { identity: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : "file", complete };
      });
    };
    const reference = scan(this.repoRoot);
    if (reference.some(owner => owner.kind !== "directory" || !owner.complete)) throw new Error("Live Stdio owner authority is partial or follows a symlink");
    const identities = reference.map(owner => owner.identity);
    const present = scan(snapshot, identities);
    if (present.some(owner => owner.kind !== "directory" || !owner.complete)) throw new Error("Removed-parent snapshot owner authority is partial or follows a symlink");
    const retained = present.map(owner => owner.identity).sort();
    assert.deepEqual(retained, identities.filter(identity => identity !== "📼️avi").sort(), "Removed-parent snapshot retains every actual sibling owner exactly once");
    const owners = retained.length;
    const phase=scriptCompilerAdmissionControlV1(this.invocation,"stdio:parent-removal",buildBudgetMs()||CATALOG_DEADLINE_MS),execution={invocation:this.invocation,phase},control=catalogControlV1(execution);
    let compiled=false,testsCompiled=false;const runtimeLaws:string[]=[];
    const env = devToolingEnv({ SEMIO_WORKSPACE_ROOT: snapshot, CARGO_TARGET_DIR: join(snapshot, "target"), CARGO_BUILD_BUILD_DIR: join(snapshot, "target", "intermediate") });
    try {
      await runControlled(execution,"cargo", [tests ? "test" : "check", "--offline", "--message-format=json-render-diagnostics", "--manifest-path", manifest, "-p", PACKAGE_NAME, ...(tests ? ["--tests", "--no-run"] : ["--lib"])], snapshot, env, control);
      compiled = true;
      if (tests) {
        testsCompiled = true;
        for (const [target, law] of [["native_openable_provider", "artifact_owned_native_codec_receipts_form_one_complete_static_bijection"], ["editor_catalog", "neutral_catalog_requires_each_edit_operation_once"]]) {
          await runControlled(execution,"cargo", ["test", "--offline", "--manifest-path", manifest, "-p", PACKAGE_NAME, "--test", target!, law!, "--", "--exact", "--nocapture"], snapshot, env, control);
          runtimeLaws.push(law!);
        }
      }
      console.log(`stdio parent removal: AVI absent; ${owners} artifact owners retained; default parent compiled`);
    } finally {
      writeFileSync(join(snapshot, tests ? "stdio-parent-removal-test-proof.json" : "stdio-parent-removal-proof.json"), JSON.stringify({ schema: "semio.stdio.parent-removal-proof/v1", absent: artifact, parent: PACKAGE_NAME, defaultFeatures: true, retainedOwnerIdentities: retained, compiled, testsCompiled, runtimeLaws }, null, 2) + "\n");
    }
  }
}

export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await testEditorCatalogContract(this.root);
    if (rest[0] === "editor-catalog-contract") return;
    await runCatalogRootContractTests(this.root,this.repoRoot,{invocation:this.invocation,phase:scriptCompilerAdmissionControlV1(this.invocation,"stdio:catalog-contract",buildBudgetMs()||CATALOG_DEADLINE_MS)});
    if (rest[0] === "catalog-root-contract") return;
    await runRepositoryCargoTests([PACKAGE_NAME], this.repoRoot, this.invocation, rest);
  }
}

/** 🧩️ Links every stdio package's own component (stdio and each `🧩️extensions` family) with the release component
 * profile, extracts its core module with JCO and validates it against the native WebAssembly parser and the component
 * function ceiling — the measured bound each family's bounded fleet must stay under. Each package's link runs within its
 * original finite caller budget, retaining compiler admission until its private artifact capture finishes. */
export class EditorComponentCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await testEditorCatalogContract(this.root);
    const outputRoot = segments[0] ?? join(cargoTargetDirectory(this.repoRoot), "stdio-editor-components");
    if (segments.length > 1 || !isAbsolute(outputRoot)) throw new Error("editor-component-check accepts one absolute output directory");
    mkdirSync(outputRoot, { recursive: true });
    const admission=scriptCompilerAdmissionControlV1(this.invocation,"stdio:editor-components",buildBudgetMs()||CATALOG_DEADLINE_MS);
    const execution={invocation:this.invocation,phase:admission},control=catalogControlV1(execution);
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
    if (!jco) throw new Error("missing workspace component tooling");
    const packages = stdioPackageManifests(resolve(this.root, "../.."));
    for (const [index, { id, path, manifest }] of packages.entries()) {
      assertControlled(control);
      const lib = manifest.package.name.replaceAll("-", "_"),stage=mkdtempSync(join(outputRoot,".stdio-editor-")),capture=join(stage,"raw.wasm");
      const env=devToolingEnv({CARGO_TARGET_DIR:join(stage,"target"),CARGO_BUILD_BUILD_DIR:cargoBuildDirectory(this.repoRoot)});
      const args = ["build", "--manifest-path", path, "-p", manifest.package.name, "--profile", COMPONENT_PROFILE, "--lib", "--target", "wasm32-wasip2"];
      try {
        await buildRepositoryCargoArtifacts(path,args.slice(1).filter((arg,index,array)=>arg!=="--manifest-path"&&array[index-1]!=="--manifest-path"),this.repoRoot,{producer:"artifact",invocation:this.invocation,control:admission,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),deliverables:[".wasm"],consume:token=>withCapturedCargoArtifactsTaskV1(token,async view=>{
          const wasm=view.files.get(`${lib}.wasm`);if(!wasm)throw Error("Actual selected Stdio compiler wasm required");await view.recheck();copyCatalogArtifact(wasm,capture,"original compiler component",catalogControlV1({invocation:this.invocation,phase:view.control}));await view.recheck();
        })});
        const out = join(outputRoot, id);
        mkdirSync(out, { recursive: true });
        const release = join(out, `${lib}.wasm`);
        await prepareReleaseComponent(capture, release, this.repoRoot, env, control,execution);
        await runControlled(execution,"node", [jco, "transpile", release, "-o", out, "--name", lib, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], this.repoRoot, env, control);
        const core = readFileSync(join(out, `${lib}.core.wasm`));
        const structure = assertComponentizableCore(core);
        console.log(`🧩️ ${index + 1}/${packages.length} ${id} component validated: ${JSON.stringify({ ...structure, coreBytes: core.byteLength })}`);
      } finally {rmSync(stage,{recursive:true,force:true});}
    }
  }
}

/** 📈️ Runs the owned deterministic-iteration `Brep` kernel benchmark suite (`🏃️benches/⏱️brep-kernel.rs`) — moved here
 * from `semio-framework-3d` in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
 * ARTIFACTS wave G5, alongside the `Brep` kernel itself. */
export class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "--manifest-path", join(this.root, "Cargo.toml"), "-p", "semio-hub-stdio"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}
