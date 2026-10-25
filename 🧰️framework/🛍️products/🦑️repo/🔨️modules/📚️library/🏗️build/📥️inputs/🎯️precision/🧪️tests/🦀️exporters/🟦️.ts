/** 🦀️ Contract: the input precision rules of the three Cargo exporters bound each key to what the exporter reads on the real tree. The crate closure equals an independent walk of the Cargo manifests (read with @iarna/toml), the exporter test is inside its own key although it lives in a `🧪️tests` directory, a dependency contributes its library and never its test targets, and no TypeScript, CSS or other-product file is part of the derived reads. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import {deriveInputsV1,readInputPrecisionV1,selectRuleV1,type InputPrecisionHostV1} from "../../🟦️.ts";
import rules from "../../🔣️.json" with {type: "json"};
import cache from "../../../../../⚡️caching/🔣️policy.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

const toml = createRequire(import.meta.url)("@iarna/toml");
const inference = await import("../../../../🕸️graph/📥️inference/🟨️.mjs"), {cacheInternals, libraryBootstrap} = inference as any;
await libraryBootstrap;
const findRoot = (directory: string): string => (existsSync(join(directory, "nx.json")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? process.cwd() : findRoot(dirname(directory)));
const workspace = findRoot(import.meta.dir), document = readInputPrecisionV1(rules);
const slash = (path: string) => path.split("\\").join("/");
const manifest = (directory: string) => toml.parse(readFileSync(join(workspace, directory, "Cargo.toml"), "utf8"));

/** 🔬️ The independent walk: path dependencies, build dependencies and, for a named crate only, development dependencies, with workspace inheritance, renames and target tables. */
function closure(directories: readonly string[]): string[] {
  const seen = new Set<string>(), named = new Set(directories), pending = [...directories];
  while (pending.length) {
    const directory = pending.pop()!;
    if (seen.has(directory)) continue;
    seen.add(directory);
    const own = manifest(directory);
    let space = resolve(workspace, directory);
    space = own.package?.workspace ? resolve(space, own.package.workspace) : (() => { let at = space; while (at !== workspace && !toml.parse(readFileSync(join(at, "Cargo.toml"), "utf8")).workspace) at = dirname(at); return at; })();
    const inherited = toml.parse(readFileSync(join(space, "Cargo.toml"), "utf8")).workspace?.dependencies ?? {};
    for (const table of [own, ...Object.values<any>(own.target ?? {})]) for (const kind of ["dependencies", "build-dependencies", "dev-dependencies"]) {
      if (kind === "dev-dependencies" && !named.has(directory)) continue;
      for (const [alias, declaration] of Object.entries<any>(table[kind] ?? {})) {
        if (typeof declaration !== "object") continue;
        const target = declaration.workspace === true ? inherited[alias] : declaration;
        if (target?.path) pending.push(slash(relative(workspace, resolve(declaration.workspace === true ? space : join(workspace, directory), target.path))));
      }
    }
  }
  return [...seen].sort();
}

const host = (): InputPrecisionHostV1 => ({
  exists: (path) => existsSync(join(workspace, path)),
  readText: (path) => { try { return readFileSync(join(workspace, path), "utf8"); } catch { return undefined; } },
  listDirectory: () => [],
  kinds: {},
  importClosure: async () => [],
  cargoManifests: () => [],
  cargoRoots: (crate) => cacheInternals.nativeDependencyRoots(crate, workspace, true, new Map(), new Map()),
  cargoSources: (root, named) => cacheInternals.cargoSourceInputs(root, workspace, new Map(), named, !named),
  cargoToolchain: cache.toolchains.cargo.files,
});

test("the corpus is closed by its schema", () => { expect(new Ajv({strict: true}).validate(schema, corpus)).toBe(true); });

for (const row of corpus.cases) test(`${row.rule}: the key is the exporter, its crate closure and nothing of the TypeScript, CSS or product trees`, async () => {
  const identity = {project: row.project, target: row.target, outputs: [row.output]}, rule = selectRuleV1(document, identity);
  expect(rule?.id).toBe(row.rule);
  expect(rule!.replaces).toEqual(["default", "^default"]);
  const derived = await deriveInputsV1(document, rule!, identity, host()), files = derived.files, manifests = derived.patterns.filter((pattern) => pattern.endsWith("/Cargo.toml") && !pattern.startsWith("!")).map((pattern) => pattern.replace("{workspaceRoot}/", "").replace(/\/Cargo\.toml$/u, "")).filter((path) => path !== "").sort();
  const expected = closure(row.directories);
  expect(manifests.filter((path) => path !== "rustfmt.toml" && path !== "rust-toolchain.toml" && existsSync(join(workspace, path, "Cargo.toml")))).toEqual(expected);
  expect(files).toContain(row.exporter);
  expect(files.some((file) => file === row.output || file.startsWith(`${row.output}/`)), "the mirror the exporter writes is not one of its reads").toBe(false);
  const included = new Set(files.filter((file) => file.endsWith(".rs")).flatMap((file) => [...readFileSync(join(workspace, file), "utf8").matchAll(/include_(?:str|bytes)!\s*\([^;]*?"([^"]+)"\s*,?\s*\)/gu)].map((match) => match[1]!.split("/").pop()!)));
  for (const file of files.filter((candidate) => /\.(?:ts|tsx|css|scss|html)$/u.test(candidate))) expect(included.has(file.split("/").pop()!), `${file} is a TypeScript or CSS read only when a Rust include names it`).toBe(true);
  for (const directory of expected) if (!row.directories.includes(directory)) for (const file of files.filter((candidate) => candidate.startsWith(`${directory}/`))) expect(/(?:^|\/)tests\/[^/]+\.rs$/u.test(file.slice(directory.length + 1)), `${file}: a dependency contributes no test target`).toBe(false);
  expect(derived.groups).toEqual(["sharedGlobals", "{projectRoot}/📋️project.json"]);
});
