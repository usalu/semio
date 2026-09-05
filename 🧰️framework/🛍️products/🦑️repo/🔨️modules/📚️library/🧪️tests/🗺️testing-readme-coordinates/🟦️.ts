import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readFileSync } from "node:fs";
import { join, posix, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJson, type ParseError } from "jsonc-parser";
import { fromMarkdown } from "mdast-util-from-markdown";
import ts from "typescript";

const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");
const sha = (value: Uint8Array): string => createHash("sha256").update(value).digest("hex");
const tick = String.fromCharCode(96);

/** 📖️ Reads only exact declared regular files through no-follow ancestry. */
function readOwned(path: string): Buffer {
  if (posix.isAbsolute(path) || /\\|^[A-Za-z]:/u.test(path) || path.split("/").some((part) => part === "" || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe documentation input");
  let current = root;
  const parts = path.split("/");
  for (const [index, part] of parts.entries()) {
    current = join(current, part);
    const node = lstatSync(current);
    if (node.isSymbolicLink() || (index < parts.length - 1 ? !node.isDirectory() : !node.isFile())) throw new Error("Nonregular documentation input");
  }
  const before = lstatSync(current), fd = openSync(current, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const node = fstatSync(fd);
    if (node.dev !== before.dev || node.ino !== before.ino || !node.isFile()) throw new Error("Documentation node changed");
    const bytes = readFileSync(fd), after = fstatSync(fd);
    if (after.size !== node.size || after.mtimeMs !== node.mtimeMs || after.mode !== node.mode) throw new Error("Documentation bytes changed");
    return bytes;
  } finally {
    closeSync(fd);
  }
}

const ownerRelative = relative(root, import.meta.dir).split("\\").join("/");
const vector = JSON.parse(readOwned(ownerRelative + "../🗺️testing-readme-coordinates/🔣️.json").toString("utf8"));
const schema = JSON.parse(readOwned(ownerRelative + "../🗺️testing-readme-coordinates/🧬️schema/🔣️.json").toString("utf8"));
const content = (): string => readOwned(vector.documents.readme).toString("utf8");
const squash = (value: string): string => value.replace(/\s+/gu, " ").trim();
const nodeText = (node: any): string => typeof node.value === "string" ? node.value : (node.children ?? []).map(nodeText).join("");

/** 📜️ Uses an independent CommonMark AST for visible prose and inline-code ownership. */
function markdownFacts(source: string) {
  const tree = fromMarkdown(source), inline: string[] = [], paragraphs: { section: string; text: string }[] = [], code: string[] = [];
  let section = "";
  const visit = (node: any): void => {
    if (node.type === "heading" && node.depth === 2) section = nodeText(node);
    if (node.type === "inlineCode") inline.push(node.value);
    if (node.type === "paragraph") paragraphs.push({ section, text: squash(nodeText(node)) });
    if (node.type === "code") code.push(node.value);
    for (const child of node.children ?? []) visit(child);
  };
  visit(tree);
  return { inline, paragraphs, code };
}

test("authored coordinate and prose vectors satisfy their neutral schema and independent JSON parser", () => {
  const validate = new Ajv({ allErrors: true }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const errors: ParseError[] = [];
  expect(parseJson(readOwned(ownerRelative + "../🗺️testing-readme-coordinates/🔣️.json").toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  for (const rows of [vector.kindLeaves, vector.inlineCoordinates, vector.paragraphs, vector.parserCases]) {
    const keys = rows.map((row: any) => row.id ?? row.kindId);
    expect(new Set(keys).size).toBe(keys.length);
  }
});

test("independent Markdown oracle excludes commented and fenced decoys", () => {
  for (const row of vector.parserCases) {
    const actual = markdownFacts(row.markdown);
    expect(actual.inline, row.id).toEqual(row.inline);
    expect(actual.paragraphs.map((entry) => entry.text), row.id).toEqual(row.paragraphs);
  }
});

test("README canonical owner tree matches raw source and independent Markdown structure", () => {
  const source = content(), facts = markdownFacts(source);
  expect(source.includes(tick.repeat(3) + "\n" + vector.canonicalTree + "\n" + tick.repeat(3))).toBe(true);
  expect(facts.code.filter((value) => value.startsWith("<owner>/"))).toEqual([vector.canonicalTree]);
  const taxonomy = JSON.parse(readOwned(vector.documents.taxonomy).toString("utf8"));
  for (const row of vector.kindLeaves) {
    const kind = taxonomy.fileKinds[row.kindId];
    expect(kind.emoji + kind.extensionChains[0], row.kindId).toBe(row.filename);
    expect(vector.canonicalTree).toContain(row.filename);
  }
});

test("README exact inline coordinates resolve their declared current owners", () => {
  const source = content(), facts = markdownFacts(source);
  for (const row of vector.inlineCoordinates) {
    expect(source.split(tick + row.value + tick).length - 1, row.id).toBe(row.count);
    expect(facts.inline.filter((value) => value === row.value).length, row.id).toBe(row.count);
    if (row.target !== null) expect(readOwned(row.target).length, row.id).toBeGreaterThan(0);
  }
  expect(source).toContain("repository-root\n    " + tick + "🚚️migration.json" + tick);
});

for (const row of vector.paragraphs) test("README reviewed requirement and routing prose: " + row.id, () => {
  const source = content(), facts = markdownFacts(source);
  expect(source.includes(row.markdown), row.id).toBe(true);
  expect(facts.paragraphs.filter((entry) => entry.section === row.section && entry.text === row.plainText), row.id).toHaveLength(1);
});

test("README no longer advertises obsolete coordinates or uniform runtime guarantees", () => {
  const source = content();
  for (const obsolete of vector.rejectedMarkdown) expect(source).not.toContain(obsolete);
});

test("README retains the concurrently authored protocol-v2 ownership and safety requirements", () => {
  const source = content();
  for (const preserved of vector.preservedMarkdown) expect(source).toContain(preserved);
});

test("registry JSON parsers agree and its schema link names the current regular schema", () => {
  const bytes = readOwned(vector.documents.registry), text = bytes.toString("utf8"), errors: ParseError[] = [];
  const own = JSON.parse(text), independent = parseJson(text, errors, { disallowComments: true, allowTrailingComma: false });
  expect(errors).toEqual([]);
  expect(independent).toEqual(own);
  expect(own.$schema).toBe(vector.registrySchema);
  const target = posix.normalize(posix.join(posix.dirname(vector.documents.registry), own.$schema));
  expect(target).toBe(vector.documents.protocolSchema);
  const schemaBytes = readOwned(target), schemaErrors: ParseError[] = [];
  expect(parseJson(schemaBytes.toString("utf8"), schemaErrors, { disallowComments: true, allowTrailingComma: false })).toEqual(JSON.parse(schemaBytes.toString("utf8")));
  expect(schemaErrors).toEqual([]);
  expect(JSON.parse(schemaBytes.toString("utf8")).$id).toBe("https://semio-tech.com/schema/repo/test/v2");
});

test("documentation correction leaves the original forty-row authority and baseline preimage immutable", () => {
  const bytes = readOwned(vector.frozenAuthority.path), catalog = JSON.parse(bytes.toString("utf8"));
  expect(sha(bytes)).toBe(vector.frozenAuthority.sha256);
  expect(catalog.cases).toHaveLength(40);
  const row = catalog.cases[vector.frozenAuthority.row];
  expect(row.referenceOwnerIds).toEqual(["markdown-relative-reference-adapter"]);
  expect(row.preimage).toEqual(vector.frozenAuthority.preimage);
  expect(row.disposition).toBe("owner-documentation-relocate");
  expect(row.generatorOwnerId).toBeNull();
  expect(sha(readOwned(vector.documents.readme))).not.toBe(row.preimage.sha256);
});

test("documentation gate registration matches the declared Nx route and both launch catalogs", () => {
  const expected = vector.execution, packagePath = relative(root, libraryRoot).split("\\").join("/") + "/📦️packages/🟦️typescript";
  const projectSource = readOwned(packagePath + "/📋️project.json").toString("utf8"), project = JSON.parse(projectSource), errors: ParseError[] = [];
  expect(parseJson(projectSource, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(project);
  expect(errors).toEqual([]);
  expect(project.targets[expected.target]).toEqual({ executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } });
  const router = readOwned(packagePath + "/📜️script.ts").toString("utf8"), tree = ts.createSourceFile("router.ts", router, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(tree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(tree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
  expect(branches[0]!.thenStatement.getText(tree)).toContain('await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });');
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const errors: ParseError[] = [], configurations = parseJson(readOwned(path).toString("utf8"), errors).configurations;
    expect(errors, path).toEqual([]);
    expect(configurations.filter((row: any) => row.name === expected.launchName), path).toEqual([{ name: expected.launchName, type: "node-terminal", request: "launch", command: expected.launchCommand, cwd: "${workspaceFolder}", presentation: { group: expected.launchGroup, order: expected.launchOrder } }]);
    expect(configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder), path).toHaveLength(1);
  }
});
