import toml from "@iarna/toml";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { inspectMutationMetadataSource } from "../../../🔍️discovery/🧵️mutation-metadata-source/🟦️.ts";
import { inspectRustModuleGraph } from "../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { inspectRustMutationMetadataFacts } from "../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { projectCargoProviderManifest, resolveCargoProviderBinding } from "../../../🔍️discovery/🚚️cargo-provider/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

describe("registryCompilerImports", () => {
  test("validates the language-neutral vectors against Bun and TypeScript", async () => {
    const { registryStaticImports } = await import("../../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts");
    const ts = await import("typescript");
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../../🔍️discovery/🧪️tests/🧫️fixtures/compiler-imports/🔣️.json"), "utf8")) as { cases: { name: string; source: string; expected: string[] }[] };
    
    
    
    
    for (const vector of fixture.cases) {
      const source = ts.createSourceFile("fixture.tsx", vector.source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
      const oracle = [...new Set(source.statements.flatMap((statement) => {
        if (ts.isImportDeclaration(statement) && statement.importClause?.isTypeOnly || ts.isExportDeclaration(statement) && statement.isTypeOnly) return [];
        const specifier = ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement) ? statement.moduleSpecifier : undefined;
        return specifier && ts.isStringLiteral(specifier) ? [specifier.text] : [];
      }))].sort();
      expect(oracle).toEqual(vector.expected);
      expect(registryStaticImports(vector.source, "fixture.tsx")).toEqual(oracle);
    }
  });

  test("rejects malformed runtime compiler capabilities without trusting declarations", async () => {
    const { scanRegistryCompilerImports } = await import("../../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts");
    for (const platform of [null, {}, { Transpiler: 7 }, { Transpiler: () => ({}) }, { Transpiler: class {} }, { Transpiler: class { scanImports() { return {}; } } }, { Transpiler: class { scanImports() { return [{ path: 1, kind: "import-statement" }]; } } }, { Transpiler: class { scanImports() { return [{ path: "./a", kind: null }]; } } }]) {
      expect(() => scanRegistryCompilerImports("import './🟦️'", "tsx", platform)).toThrow();
    }
    const row = { path: "./a", kind: "import-statement", ignored: true };
    const imports = scanRegistryCompilerImports("", "tsx", { Transpiler: class { scanImports() { return [row]; } } });
    row.path = "./rebound";
    expect(imports).toEqual([{ path: "./a", kind: "import-statement" }]);
  });
});

const CARGO_PROVIDER_PROJECTION_FIXTURE = join(contractRoot, "../🧫️fixtures/📽️cargo-provider-projection/🔣️.json");

describe("cargo provider manifest projection", () => {
  test("matches independent TOML facts before preserving only bounded identity projections", () => {
    const fixture = JSON.parse(readFileSync(CARGO_PROVIDER_PROJECTION_FIXTURE, "utf8")) as {
      schemaVersion: number;
      accepted: readonly { id: string; locator: string; source: string; tomlOracle: unknown; expected: unknown }[];
      rejected: readonly { id: string; locator: string; source: string; reason: string; tomlOracle: unknown }[];
    };
    
    
    
    for (const vector of fixture.accepted) {
      expect(toml.parse(vector.source), vector.id).toEqual<typeof vector.tomlOracle>(vector.tomlOracle);
      expect(projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.id).toEqual<typeof vector.expected>(vector.expected);
    }
    for (const vector of fixture.rejected) {
      if (vector.tomlOracle === null) expect(() => toml.parse(vector.source), vector.id).toThrow();
      else expect(toml.parse(vector.source), vector.id).toEqual<typeof vector.tomlOracle>(vector.tomlOracle);
      expect(() => projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.reason).toThrow();
    }
  });
});

const CARGO_PROVIDER_BINDING_FIXTURE = join(contractRoot, "../🧫️fixtures/🪢️cargo-provider-binding/🔣️.json");

describe("cargo provider binding", () => {
  test("resolves one selected local normal dependency without inferring a provider", () => {
    const fixture = JSON.parse(readFileSync(CARGO_PROVIDER_BINDING_FIXTURE, "utf8")) as { schemaVersion: 1; accepted: readonly { id: string; consumerManifest: string; consumerSource: string; providerManifest: string; providerSource: string; librarySource: string; dependencyKey: string; workspaceSource: string; workspaceRoot?: string; workspaceRootSymlink?: true; workspaceAncestorSymlink?: true; virtualFilesystem?: true; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; omitProviderManifest?: true; omitLibrarySource?: true; expected: { externName: string; packageName: string; libraryName: string; workspaceInherited: boolean } }[]; rejected: readonly { id: string; consumerManifest: string; consumerSource: string; providerManifest: string; providerSource: string; librarySource: string; dependencyKey: string; workspaceSource: string; workspaceRoot?: string; workspaceRootSymlink?: true; workspaceAncestorSymlink?: true; virtualFilesystem?: true; extraFiles?: readonly { path: string; source: string }[]; links?: readonly { path: string; target: string }[]; omitProviderManifest?: true; omitLibrarySource?: true; expected: null }[] };
    
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cargo provider binding artifacts.");
    mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot, "semio-cargo-provider-binding-"));
    try {
      for (const vector of [...fixture.accepted, ...fixture.rejected]) {
        const caseDirectory = join(root, vector.id), sourceRoot = vector.workspaceRootSymlink ? join(caseDirectory, "source") : vector.workspaceAncestorSymlink ? join(caseDirectory, "source", "workspace") : vector.workspaceRoot ?? caseDirectory;
        const caseRoot = vector.workspaceRootSymlink ? join(caseDirectory, "workspace") : vector.workspaceAncestorSymlink ? join(caseDirectory, "alias", "workspace") : sourceRoot, librarySource = join(sourceRoot, "provider/src/lib.rs");
        if (!vector.virtualFilesystem) {
          mkdirSync(dirname(librarySource), { recursive: true });
          writeFileSync(join(sourceRoot, "Cargo.toml"), vector.workspaceSource);
          mkdirSync(dirname(join(sourceRoot, vector.consumerManifest)), { recursive: true });
          writeFileSync(join(sourceRoot, vector.consumerManifest), vector.consumerSource);
          mkdirSync(dirname(join(sourceRoot, vector.providerManifest)), { recursive: true });
          if (!vector.omitProviderManifest) writeFileSync(join(sourceRoot, vector.providerManifest), vector.providerSource);
          if (!vector.omitLibrarySource) writeFileSync(librarySource, vector.librarySource);
          for (const file of vector.extraFiles ?? []) {
            const path = join(sourceRoot, file.path);
            mkdirSync(dirname(path), { recursive: true });
            writeFileSync(path, file.source);
          }
          for (const link of vector.links ?? []) {
            const path = join(sourceRoot, link.path);
            mkdirSync(dirname(path), { recursive: true });
            symlinkSync(process.platform === "win32" ? join(dirname(path), link.target) : link.target, path, "file");
          }
          if (vector.workspaceRootSymlink) symlinkSync(process.platform === "win32" ? sourceRoot : "source", caseRoot, process.platform === "win32" ? "junction" : "dir");
          if (vector.workspaceAncestorSymlink) symlinkSync(process.platform === "win32" ? join(caseDirectory, "source") : "source", join(caseDirectory, "alias"), process.platform === "win32" ? "junction" : "dir");
        }
        expect(toml.parse(vector.consumerSource), vector.id).toBeDefined();
        expect(toml.parse(vector.providerSource), vector.id).toBeDefined();
        const input = { workspaceRoot: caseRoot, consumerManifestLocator: vector.consumerManifest, dependencyKey: vector.dependencyKey };
        if (vector.expected === null) expect(() => resolveCargoProviderBinding(input), vector.id).toThrow();
        else expect(resolveCargoProviderBinding(input), vector.id).toMatchObject(vector.expected);
      }
    } finally {}
  });
});

const MUTATION_METADATA_SOURCE_FIXTURE = join(contractRoot, "../🧫️fixtures/🏷️metadata-source-provider/🔣️.json");

describe("mutation metadata source provider", () => {
  test("proves independent canonical derive and lower contract routes from one consumer declaration", () => {
    const fixture = JSON.parse(readFileSync(MUTATION_METADATA_SOURCE_FIXTURE, "utf8")) as { schemaVersion: 1; cases: readonly { id: string; derive: string; contract: string; facadeSource: string; manualImpl: string; binding?: "direct" | "inherited"; consumerPrefix?: string; consumerSuffix?: string; originModulePath?: readonly string[]; accepted: boolean }[] };
    
    const artifactDirectory = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactDirectory) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for metadata source provider artifacts.");
    const artifactRoot = resolve(getWorkspaceRoot(), artifactDirectory);
    mkdirSync(artifactRoot, { recursive: true });
    const root = mkdtempSync(join(artifactRoot, "semio-metadata-source-provider-"));
    try {
      const lower = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust", facade = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", derive = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust", consumer = "consumer";
      const manifest = (path: string): string => `${path}/Cargo.toml`, source = (path: string): string => `${path}/🦀️.rs`, relativePath = (from: string, to: string): string => relative(dirname(join(root, from)), join(root, to)).split(sep).join("/");
      for (const vector of fixture.cases) {
        const files = new Map<string, string>(), add = (path: string, text: string): void => { files.set(path, text); const absolute = join(root, path); mkdirSync(dirname(absolute), { recursive: true }); writeFileSync(absolute, text); };
        const dependencyRows = (from: string): string => `derive = { package = 'semio-framework-os-kernel-dsl-derive', path = '${relativePath(from, derive)}' }\nfacade = { package = 'semio-framework-os-kernel', path = '${relativePath(from, facade)}' }\nlower = { package = 'semio-framework-replication', path = '${relativePath(from, lower)}' }`;
        add("Cargo.toml", `[workspace]\n${vector.binding === "inherited" ? `\n[workspace.dependencies]\n${dependencyRows("Cargo.toml")}\n` : ""}`);
        add(manifest(lower), "[package]\nname = 'semio-framework-replication'\n\n[lib]\nname = 'protocol'\npath = '🦀️.rs'\n");
        add(source(lower), "pub trait MutationLeaf {}\n");
        add(manifest(derive), "[package]\nname = 'semio-framework-os-kernel-dsl-derive'\n\n[lib]\nname = 'dsl_derive'\npath = '🦀️.rs'\nproc-macro = true\n");
        add(source(derive), "pub fn marker() {}\n");
        add(manifest(facade), `[package]\nname = 'semio-framework-os-kernel'\n\n[lib]\nname = 'semio_framework_os_kernel'\npath = '🦀️.rs'\n\n[dependencies]\nlower = { package = 'semio-framework-replication', path = '${relativePath(manifest(facade), lower)}' }\nderive = { package = 'semio-framework-os-kernel-dsl-derive', path = '${relativePath(manifest(facade), derive)}' }\n`);
        add(source(facade), vector.facadeSource);
        add(manifest(consumer), `[package]\nname = 'consumer'\n\n[lib]\nname = 'consumer'\npath = '🦀️.rs'\n\n[dependencies]\n${vector.binding === "inherited" ? "derive = { workspace = true }\nfacade = { workspace = true }\nlower = { workspace = true }" : dependencyRows(manifest(consumer))}\n`);
        add(source(consumer), `extern crate derive;\n${vector.consumerPrefix ?? ""}${vector.manualImpl}#[derive(${vector.derive})]\n#[mutation_leaf(contract = ${vector.contract})]\npub struct Payload;\n${vector.consumerSuffix ?? ""}`);
        const proof = inspectMutationMetadataSource({ repositoryRoot: root, consumerManifestLocator: manifest(consumer), origin: { sourcePath: source(consumer), declarationName: "Payload", modulePath: vector.originModulePath ?? [] }, files: [...files.keys()], readSource: (path) => files.get(path) });
        expect(proof.accepted, `${vector.id}: ${proof.diagnostics.join("; ")}`).toBe(vector.accepted);
        if (vector.accepted) expect(proof).toMatchObject({ deriveProvider: { role: "metadata-derive" }, contractProvider: { role: "lower-contract" }, facadeProvider: { role: "os-facade" } });
      }
      const compiler = join(root, "🧪️rustc"), lowerLibrary = join(compiler, "liblower.rlib"), facadeLibrary = join(compiler, "libfacade.rlib"), deriveLibrary = join(compiler, process.platform === "darwin" ? "libderive.dylib" : process.platform === "win32" ? "derive.dll" : "libderive.so"), consumerLibrary = join(compiler, "libconsumer.rlib");
      mkdirSync(compiler, { recursive: true });
      const compilerSource = (name: string, text: string): string => { const path = join(compiler, name); writeFileSync(path, text); return path; };
      const rustc = (args: string[]): void => { const result = spawnSync("rustc", args, { encoding: "utf8" }); expect({ status: result.status, signal: result.signal, stderr: result.stderr }).toMatchObject({ status: 0, signal: null }); };
      rustc(["--edition=2021", "--crate-name", "lower", "--crate-type", "rlib", compilerSource("lower.rs", "pub trait MutationLeaf {}\n"), "-o", lowerLibrary]);
      rustc(["--edition=2021", "--crate-name", "facade", "--crate-type", "rlib", compilerSource("facade.rs", "extern crate lower; pub use lower::MutationLeaf;\n"), "--extern", `lower=${lowerLibrary}`, "-L", `dependency=${compiler}`, "-o", facadeLibrary]);
      rustc(["--edition=2021", "--crate-name", "derive", "--crate-type", "proc-macro", compilerSource("derive.rs", "extern crate proc_macro; use proc_macro::TokenStream; #[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))] pub fn mutation_leaf(_: TokenStream) -> TokenStream { TokenStream::new() }\n"), "-o", deriveLibrary]);
      rustc(["--edition=2021", "--crate-name", "consumer", "--crate-type", "rlib", compilerSource("consumer.rs", "extern crate derive; extern crate facade; use self::facade::MutationLeaf as SelfContract; #[derive(derive::MutationLeaf)] #[mutation_leaf(contract = ::SelfContract)] pub struct SelfPayload; mod nested { use super::facade::MutationLeaf as SuperContract; #[derive(derive::MutationLeaf)] #[mutation_leaf(contract = ::SuperContract)] pub struct SuperPayload; }\n"), "--extern", `derive=${deriveLibrary}`, "--extern", `facade=${facadeLibrary}`, "--extern", `lower=${lowerLibrary}`, "-L", `dependency=${compiler}`, "-o", consumerLibrary]);
    } finally {}
  });

  test("proves current STDIO TXT and GLTF wrapped origins through kernel aliases", () => {
    const repositoryRoot = getWorkspaceRoot();
    const facadeManifest = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml", facadeLibrary = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs", dslSource = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs", sprSource = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs", commandSource = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs";
    const leaves = [
      { consumerManifest: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml", consumerLibrary: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs", root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs", sourcePath: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🦀️.rs" },
      { consumerManifest: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/Cargo.toml", consumerLibrary: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs", root: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦀️.rs", sourcePath: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🦀️.rs" },
    ] as const;
    const origins = leaves.map(({ consumerManifest, sourcePath }) => {
      const declarations = inspectRustMutationMetadataFacts(readFileSync(join(repositoryRoot, sourcePath), "utf8")).declarations.filter((item) => item.visibility === "pub" && !item.conditional && item.mutationLeaf.state === "valid");
      expect(declarations, sourcePath).toHaveLength(1);
      return { consumerManifest, origin: { sourcePath, declarationName: declarations[0]!.name, modulePath: declarations[0]!.modulePath } };
    });
    const files = [facadeManifest, facadeLibrary, dslSource, sprSource, commandSource, ...leaves.flatMap(({ consumerManifest, consumerLibrary, root, sourcePath }) => [consumerManifest, consumerLibrary, root, sourcePath])];
    expect(files.every((path) => !path.split("/").some((segment) => segment.toLocaleLowerCase("en-US") === "compose") && !lstatSync(join(repositoryRoot, path)).isSymbolicLink())).toBe(true);
    const source = new Map(files.map((path) => [path, readFileSync(join(repositoryRoot, path), "utf8")]));
    const graph = inspectRustModuleGraph([...source.keys()], (path) => source.get(path), { strictManifests: true });
    expect((graph.contexts.get(facadeLibrary) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.length === 0)).toHaveLength(1);
    expect(graph.targets.get(`${facadeLibrary}\0os_dsl`)).toBe(facadeLibrary);
    expect(graph.targets.get(`${facadeLibrary}\0os_dsl::component`)).toBe(dslSource);
    expect(graph.targets.get(`${facadeLibrary}\0os_spr::component`)).toBe(sprSource);
    expect(graph.targets.get(`${facadeLibrary}\0os_spr::command`)).toBe(commandSource);
    expect((graph.contexts.get(facadeLibrary) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.join("\0") === "os_dsl")).toHaveLength(1);
    expect((graph.contexts.get(dslSource) ?? []).filter((context) => context.manifestPath === facadeManifest && context.modulePath.join("\0") === "os_dsl\0component")).toHaveLength(1);
    expect(resolveCargoProviderBinding({ workspaceRoot: repositoryRoot, consumerManifestLocator: facadeManifest, dependencyKey: "dsl_derive" })).toMatchObject({ providerManifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", libraryName: "dsl_derive", procMacro: true });
    for (const { consumerManifest, origin } of origins) {
      expect((graph.contexts.get(origin.sourcePath) ?? []).filter((context) => context.manifestPath === consumerManifest && context.sourceScope.join("\0") === origin.modulePath.join("\0"))).toHaveLength(1);
      expect(inspectMutationMetadataSource({ repositoryRoot, consumerManifestLocator: consumerManifest, origin, files: [...source.keys()], readSource: (path) => source.get(path) })).toMatchObject({ accepted: true, deriveProvider: { role: "metadata-derive", manifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", libraryName: "dsl_derive", procMacro: true }, contractProvider: { role: "lower-contract", manifestLocator: "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml", libraryName: "protocol", procMacro: false }, facadeProvider: { role: "os-facade", manifestLocator: facadeManifest, libraryName: "semio_framework_os_kernel", procMacro: false } });
    }
  }, 30_000);
});
