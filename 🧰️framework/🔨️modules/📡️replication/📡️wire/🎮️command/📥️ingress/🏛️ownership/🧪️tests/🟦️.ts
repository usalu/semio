import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import Parser from "web-tree-sitter";

type Reader = (path: string) => string;
type ClosureRow = { package: string; manifest: string; edges: { scope: string; name: string; manifest: string }[] };
type Fixture = {
  version: number;
  owner: string;
  exports: { types: string[]; constants: string[] };
  bounds: Record<string, string>;
  sourceCases: { id: string; accepted: boolean }[];
  laws: string[];
  fixture: { path: string; digest: string };
  closure: ClosureRow[];
  paths: { entry: string; manifest: string; neutralTests: string };
};

const root = resolve(import.meta.dir, "../../../../../../../.."), owner = "🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress", digest = (value: string) => createHash("sha256").update(value).digest("hex");

/** 🏛️ Admits public neutral ownership, declared bounds, dependency authority and native law mounts. */
export async function inspectPagedCommandIngressOwnership(repository: string, read: Reader = path => readFileSync(join(repository, path), "utf8")): Promise<{ types: number; constants: number; laws: number; closure: number }> {
  const fixture = JSON.parse(read(`${owner}/🧫️fixtures/🔣️.json`)) as Fixture, schema = JSON.parse(read(`${owner}/🧬️schema/🔣️.json`)), validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repository)), "out/tree-sitter-rust.wasm")));
  const trees: Parser.Tree[] = [];
  const ast = (path: string) => { const tree = parser.parse(read(path)); trees.push(tree); return tree.rootNode; };
  const nodes = (node: Parser.SyntaxNode): Parser.SyntaxNode[] => [node, ...node.namedChildren.flatMap(nodes)];
  const publicNames = (node: Parser.SyntaxNode, kinds: string[]) => node.namedChildren.filter(child => kinds.includes(child.type) && child.namedChildren.some(part => part.type === "visibility_modifier" && part.text === "pub")).map(child => child.childForFieldName("name")!.text);
  try {
    const ownerAst = ast(`${owner}/🦀️.rs`);
    expect(ownerAst.hasError()).toBe(false);
    expect(publicNames(ownerAst, ["struct_item", "enum_item", "type_item"])).toEqual(fixture.exports.types);
    expect(publicNames(ownerAst, ["const_item"])).toEqual(fixture.exports.constants);
    const tokens = (node: Parser.SyntaxNode): string => node.type.endsWith("comment") ? "" : node.children.length ? node.children.map(tokens).join("") : node.text;
    const attributes = (node: Parser.SyntaxNode): Parser.SyntaxNode[] => {
      const result: Parser.SyntaxNode[] = [];
      for (let previous = node.previousNamedSibling; previous; previous = previous.previousNamedSibling) {
        if (previous.type.endsWith("comment")) continue;
        if (previous.type !== "attribute_item") break;
        result.unshift(previous.namedChildren[0]!);
      }
      return result;
    };
    const modulePath = (node: Parser.SyntaxNode) => {
      const values = attributes(node).filter(attribute => attribute.namedChildren[0]?.text === "path").map(attribute => JSON.parse(attribute.childForFieldName("value")!.text));
      expect(values).toHaveLength(1);
      return values[0] as string;
    };
    const executable = (node: Parser.SyntaxNode) => {
      for (let current: Parser.SyntaxNode | null = node; current; current = current.parent) if (current === node || current.type === "mod_item") expect(attributes(current).filter(attribute => ["cfg", "cfg_attr"].includes(attribute.namedChildren[0]?.text ?? ""))).toEqual([]);
    };
    for (const declaration of ownerAst.namedChildren.filter(node => ["struct_item", "enum_item", "type_item", "const_item"].includes(node.type) && node.namedChildren.some(part => part.type === "visibility_modifier" && part.text === "pub"))) executable(declaration);
    for (const constant of ownerAst.namedChildren.filter(node => node.type === "const_item" && fixture.exports.constants.includes(node.childForFieldName("name")!.text))) expect(tokens(constant.childForFieldName("value")!)).toBe(fixture.bounds[constant.childForFieldName("name")!.text]);
    const entryAst = ast(fixture.paths.entry);
    expect(entryAst.hasError()).toBe(false);
    const mounts = nodes(entryAst).filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "command_ingress");
    expect(mounts).toHaveLength(1);
    executable(mounts[0]!);
    expect(mounts[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    expect(mounts[0]!.parent?.parent?.childForFieldName("name")?.text).toBe("wire");
    expect(attributes(mounts[0]!).map(attribute => attribute.namedChildren[0]!.text)).toEqual(["path"]);
    expect(modulePath(mounts[0]!)).toBe("../../📡️wire/🎮️command/📥️ingress/🦀️.rs");
    const facades = nodes(entryAst).filter(node => node.type === "use_declaration" && tokens(node.childForFieldName("argument")!.childForFieldName("path") ?? node) === "wire::command_ingress");
    expect(facades).toHaveLength(1);
    executable(facades[0]!);
    expect(facades[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    expect(attributes(facades[0]!)).toEqual([]);
    expect(facades[0]!.childForFieldName("argument")!.childForFieldName("list")!.namedChildren.map(node => node.text)).toEqual([...fixture.exports.types, ...fixture.exports.constants]);
    const testMounts = ownerAst.namedChildren.filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "unit_tests");
    expect(testMounts).toHaveLength(1);
    expect(modulePath(testMounts[0]!)).toBe("🧪️tests/🔬️unit/🦀️.rs");
    expect(attributes(testMounts[0]!).map(attribute => attribute.namedChildren[0]!.text).sort()).toEqual(["cfg", "path"]);
    expect(attributes(testMounts[0]!).map(tokens)).toContain("cfg(test)");
    const workspace = parseToml(read("Cargo.toml")) as any, closure = new Map<string, ClosureRow>();
    const manifest = (path: string): any => { const source = read(path), first = parseToml(source), second = Bun.TOML.parse(source); expect<object>(first).toEqual(second); return first; };
    const visit = (path: string) => {
      if (closure.has(path)) return;
      const data = manifest(path), row: ClosureRow = { package: data.package.name, manifest: path, edges: [] }; closure.set(path, row);
      const tables: [string, Record<string, any>][] = [["normal", data.dependencies ?? {}], ...Object.entries(data.target ?? {}).map(([target, value]) => [target, (value as any).dependencies ?? {}] as [string, Record<string, any>])];
      for (const [scope, table] of tables) for (const [name, declaration] of Object.entries(table)) {
        const inherited = typeof declaration === "object" && declaration.workspace === true, edge = inherited ? workspace.workspace.dependencies[name] : declaration;
        expect(edge, `unresolved dependency ${path}:${name}`).toBeDefined();
        if (typeof edge !== "object" || !edge.path) { expect((edge.package ?? name).startsWith("semio-")).toBe(false); continue; }
        const target = relative(repository, resolve(repository, inherited ? "" : dirname(path), edge.path, "Cargo.toml")).split("\\").join("/");
        row.edges.push({ scope, name, manifest: target }); visit(target);
      }
      row.edges.sort((a, b) => `${a.scope}:${a.name}`.localeCompare(`${b.scope}:${b.name}`));
    };
    visit(fixture.paths.manifest);
    expect([...closure.values()].sort((a, b) => a.manifest.localeCompare(b.manifest))).toEqual(fixture.closure);
    expect([...closure.values()].filter(row => !row.manifest.startsWith("🧰️framework/🔨️modules/"))).toEqual([]);
    expect(workspace.workspace.members).toContain(dirname(fixture.paths.manifest));
    expect(manifest(fixture.paths.manifest).lib.name).toBe("protocol");
    const lawAst = ast(fixture.paths.neutralTests);
    expect(lawAst.hasError()).toBe(false);
    const functions = nodes(lawAst).filter(node => node.type === "function_item"), laws = functions.filter(node => fixture.laws.includes(node.childForFieldName("name")?.text ?? ""));
    expect(laws.map(node => node.childForFieldName("name")!.text)).toEqual(fixture.laws);
    expect(functions.filter(node => attributes(node).some(attribute => tokens(attribute) === "test")).map(node => node.childForFieldName("name")!.text)).toEqual(fixture.laws);
    for (const law of laws) {
      expect(law.parent?.type).toBe("source_file");
      expect(attributes(law).map(tokens)).toEqual(["test"]);
      expect(nodes(law).some(node => node.type === "macro_invocation" && ["assert", "assert_eq"].includes(node.childForFieldName("macro")?.text ?? ""))).toBe(true);
    }
    const corpusLaws = laws.slice(1);
    for (const law of corpusLaws) expect(nodes(law).some(node => node.type === "call_expression" && node.childForFieldName("function")?.text === "command_ingress_pages_fixture")).toBe(true);
    const includes = nodes(lawAst).filter(node => node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "include_str");
    expect(includes).toHaveLength(1);
    const literal = includes[0]!.namedChildren.find(node => node.type === "token_tree")!.namedChildren[0]!.text;
    expect(relative(repository, resolve(repository, dirname(fixture.paths.neutralTests), JSON.parse(literal))).split("\\").join("/")).toBe(fixture.fixture.path);
    expect(digest(read(fixture.fixture.path))).toBe(fixture.fixture.digest);
    return { types: fixture.exports.types.length, constants: fixture.exports.constants.length, laws: fixture.laws.length, closure: closure.size };
  } finally { for (const tree of trees) tree.delete(); parser.delete(); }
}

test("the complete paged command owner and original native laws admit only their canonical neutral authority", async () => {
  const result = await inspectPagedCommandIngressOwnership(root);
  expect(result).toEqual({ types: 14, constants: 5, laws: 4, closure: JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")).closure.length });
});

test("closed ownership corpus rejects duplicate, omitted and hostile source authority", () => {
  const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")), validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(root, owner, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(fixture)).toBe(true);
  const changes = [(value: any) => value.exports.types.push(value.exports.types[0]), (value: any) => value.exports.types.pop(), (value: any) => value.exports.constants.reverse(), (value: any) => value.laws.pop(), (value: any) => value.owner = "os/channel", (value: any) => value.paths.neutralTests = value.owner, (value: any) => value.extra = true];
  for (const change of changes) { const hostile = structuredClone(fixture); change(hostile); expect(validate(hostile)).toBe(false); }
});

test("canonical package membership and executable routes retain the complete native cohort", () => {
  const pkg = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust", workspace = JSON.parse(readFileSync(join(root, "package.json"), "utf8")), project = JSON.parse(readFileSync(join(root, pkg, "📋️project.json"), "utf8")), registration = JSON.parse(readFileSync(join(root, pkg, "package.json"), "utf8")), script = readFileSync(join(root, pkg, "📜️script.ts"), "utf8");
  expect(workspace.workspaces.some((pattern: string) => !pattern.startsWith("!") && new Bun.Glob(pattern).match(pkg))).toBe(true);
  expect(workspace.workspaces.some((pattern: string) => pattern.startsWith("!") && new Bun.Glob(pattern.slice(1)).match(pkg))).toBe(false);
  expect(registration.name).toBe(project.name);
  expect(registration.nx.includedScripts).toEqual([]);
  for (const gate of ["test-command-ingress-ownership", "test-command-ingress-native"]) {
    expect(project.targets[gate].options.command).toBe(`bun ./📜️script.ts ${gate}`);
    expect(registration.scripts[gate]).toBe(`nx run ${project.name}:${gate}`);
    expect(project.targets[gate].cache).toBe(false);
    const inputs = project.targets[gate].inputs as string[];
    expect(inputs).toContain(`{workspaceRoot}/${owner}/**/*`);
    expect(inputs).toContain("{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🧫️fixtures/📄️pages/🔣️.json");
    expect(inputs).toContain("{workspaceRoot}/Cargo.toml");
    expect(inputs).toContain("{workspaceRoot}/package.json");
    expect(inputs.filter(value => value.includes("🛍️products/") || value.includes("🔨️modules/🎠️kernel/") || value.includes("🧰️framework/📦️packages/"))).toEqual([]);
  }
  expect(script).toContain('extraArgs:["--lib","--no-fail-fast"]');
  expect(script).toContain('packages:["semio-framework-replication"]');
  expect(script).toContain('if(rest.length)throw Error("test-command-ingress-native accepts only an execution level")');
});

test("schema-authored source witnesses admit private evolution and reject public ownership failures", async () => {
  const fixture = JSON.parse(readFileSync(join(root, owner, "🧫️fixtures/🔣️.json"), "utf8")) as Fixture;
  for (const row of fixture.sourceCases) {
    const changes = new Map<string, string>(), componentPath = `${owner}/🦀️.rs`, component = readFileSync(join(root, componentPath), "utf8");
    if (row.id === "private-extension") changes.set(componentPath, component + "\nfn ownership_private_extension(value: usize) -> usize { value }\n");
    else if (row.id === "formatting") {
      for (const path of [componentPath, fixture.paths.entry, fixture.paths.neutralTests]) changes.set(path, readFileSync(join(root, path), "utf8").replaceAll("#[path =", "#[ path  =").replaceAll("#[test]", "#[ test ]").replaceAll("pub mod command_ingress;", "pub  mod\n command_ingress ;").replaceAll("mod unit_tests;", "mod\n unit_tests ;"));
    } else if (row.id === "public-owner") changes.set(fixture.paths.entry, readFileSync(join(root, fixture.paths.entry), "utf8").replace("../../📡️wire/🎮️command/📥️ingress/🦀️.rs", "../../../../🛍️products/💻️os/🦀️.rs"));
    else if (row.id === "public-type") changes.set(componentPath, component.replace("pub struct FixedCommandPage", "pub struct UnownedCommandPage"));
    else if (row.id === "constant") changes.set(componentPath, component.replace("usize = 4_096", "usize = 8_192"));
    else if (row.id === "missing-law") changes.set(fixture.paths.neutralTests, readFileSync(join(root, fixture.paths.neutralTests), "utf8").replace("fn " + fixture.laws[0] + "(", "fn absent_native_ownership_law("));
    else if (row.id === "disabled-law") changes.set(fixture.paths.neutralTests, readFileSync(join(root, fixture.paths.neutralTests), "utf8").replace("#[test]", "#[test]\n#[cfg(any())]"));
    else if (row.id === "disabled-test-mount") changes.set(componentPath, component.replace("#[cfg(test)]", "#[cfg(test)]\n#[cfg(any())]"));
    else if (row.id === "disabled-public-owner") changes.set(fixture.paths.entry, readFileSync(join(root, fixture.paths.entry), "utf8").replace("pub mod command_ingress;", "#[cfg(any())]\npub mod command_ingress;"));
    else if (row.id === "disabled-public-facade") changes.set(fixture.paths.entry, readFileSync(join(root, fixture.paths.entry), "utf8").replace("pub use wire::command_ingress::{", "#[cfg(any())]\npub use wire::command_ingress::{"));
    else if (row.id === "disabled-public-type") changes.set(componentPath, component.replace("pub struct FixedCommandPage", "#[cfg(any())]\npub struct FixedCommandPage"));
    else if (row.id === "disabled-constant") changes.set(componentPath, component.replace("pub const COMMAND_PAGE_MAXIMUM_BYTES", "#[cfg(any())]\npub const COMMAND_PAGE_MAXIMUM_BYTES"));
    else if (row.id === "disabled-parent-wire") changes.set(fixture.paths.entry, readFileSync(join(root, fixture.paths.entry), "utf8").replace("pub mod wire {", "#[cfg(any())]\npub mod wire {"));
    else if (row.id === "disabled-law-container") changes.set(fixture.paths.neutralTests, "#[cfg(any())]\nmod disabled_laws {\n" + readFileSync(join(root, fixture.paths.neutralTests), "utf8") + "\n}\n");
    else if (row.id === "hostile-dependency") changes.set(fixture.paths.manifest, readFileSync(join(root, fixture.paths.manifest), "utf8").replace("[dependencies]", '[dependencies]\nsemio-foreign-owner = "1"'));
    else throw Error("Unknown closed ownership source case: " + row.id);
    const read: Reader = path => changes.get(path) ?? readFileSync(join(root, path), "utf8");
    if (row.accepted) expect(await inspectPagedCommandIngressOwnership(root, read)).toEqual({ types: 14, constants: 5, laws: 4, closure: fixture.closure.length });
    else await expect(inspectPagedCommandIngressOwnership(root, read)).rejects.toThrow();
    console.log(`[DEBUG] command ingress source witness ${row.id}: ${row.accepted ? "admitted" : "refused"}`);
  }
});
