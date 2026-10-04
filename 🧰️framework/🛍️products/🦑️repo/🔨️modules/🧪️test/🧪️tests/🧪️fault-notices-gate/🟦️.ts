/** 📢️ The `schema-fault-notice` gate (design §20.12) over `🧫️fixtures/🧫️fault-notices-gate/🔣️.json` (Ajv-validated against its schema):
 * every snippet's fault code sites read by the token gate — string literals, named `const`s, parameter and fixed-code refusal-helper
 * calls, none inside `#[cfg(test)]` items, each with its enclosing `fn` and trait-default flag — equal those of an independent
 * tree-sitter-rust oracle, and one planted plugin plus a planted plugin SDK — sources, committed descriptor and framework table written
 * to a scratch repository — yield exactly the listed findings and census row (the gate fails on them), and the `history-editing` scope
 * keeps exactly the refusals of the history-editing, document-load and tool flows among them. */
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Parser from "web-tree-sitter";
import { type FaultCodeSite, type FaultNoticeScope, faultNoticeReport, rustFaultCodeSites, rustFaultHelpers, rustStringConsts } from "../../🧬️schema/📋️orchestration/🟦️.ts";

type Site = Pick<FaultCodeSite, "code" | "via">;
type Located = Pick<FaultCodeSite, "code" | "via" | "within" | "defaulted">;
type Fixture = {
  readonly cases: readonly { readonly id: string; readonly rust: string; readonly sites: readonly Site[] }[];
  readonly plugin: { readonly files: Readonly<Record<string, string>>; readonly findings: readonly (readonly [string, string])[]; readonly census: readonly Readonly<{ owner: string; codes: number; labelled: number; declared: number; anonymous: number; findings: number; scoped: number }>[]; readonly scoped: { readonly scope: FaultNoticeScope; readonly findings: readonly (readonly [string, string])[] } };
};

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧫️fault-notices-gate/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️fault-notices-gate/🔣️.json"), "utf8"));

await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repoRoot)), "out", "tree-sitter-rust.wasm")));

/** 🌳️ Every node of `node`'s subtree, `node` first. */
function descendants(node: Parser.SyntaxNode): Parser.SyntaxNode[] {
  return [node, ...node.namedChildren.flatMap(descendants)];
}

/** 🔤️ A tree-sitter `string_literal`'s value, or `null` for any other node. */
function stringValue(node: Parser.SyntaxNode | undefined): string | null {
  return node?.type === "string_literal" ? node.text.slice(1, -1) : null;
}

/** 🔮️ The independent oracle: the fault code sites of one source read from the tree-sitter-rust syntax tree, in source order, each with
 * its innermost enclosing `fn` and whether that `fn` is a trait's default method — a code argument is a string literal or a (path to a)
 * `&str` const; a call of a free function whose body builds its fault from one of its parameters carries its code at that position, and
 * a call of a free `-> Fault` function building exactly one fault from a literal or const carries that fixed code; items gated by
 * `#[cfg(…test…)]` hold no site. */
function oracleSites(source: string): Located[] {
  const sites: (Site & { readonly node: Parser.SyntaxNode })[] = [];
  const root = parser.parse(source).rootNode;
  const nodes = descendants(root);
  const testOnly = nodes
    .filter((node) => node.type === "attribute_item" && /^#\[cfg\(/u.test(node.text.replace(/\s/gu, "")) && /\btest\b/u.test(node.text) && !/not\(\s*test\s*\)/u.test(node.text))
    .map((attribute) => {
      let item = attribute.nextNamedSibling;
      while (item?.type === "attribute_item") item = item.nextNamedSibling;
      return item;
    })
    .filter((item): item is Parser.SyntaxNode => item !== null);
  const consts = new Map(nodes.filter((node) => node.type === "const_item" && /^&(?:'static)?str$/u.test(node.childForFieldName("type")?.text.replace(/\s/gu, "") ?? "") && node.childForFieldName("value")?.type === "string_literal").map((node) => [node.childForFieldName("name")!.text, stringValue(node.childForFieldName("value")!)!] as const));
  const value = (node: Parser.SyntaxNode | undefined): string | null => stringValue(node) ?? (node?.type === "identifier" ? (consts.get(node.text) ?? null) : node?.type === "scoped_identifier" ? (consts.get(node.childForFieldName("name")?.text ?? "") ?? null) : null);
  const callee = (node: Parser.SyntaxNode): readonly [string, string, readonly Parser.SyntaxNode[]] => {
    const target = node.childForFieldName("function");
    const scoped = target?.type === "scoped_identifier";
    return [scoped ? (target.childForFieldName("path")?.text.split("::").at(-1) ?? "") : "", scoped ? (target.childForFieldName("name")?.text ?? "") : (target?.text ?? ""), node.childForFieldName("arguments")?.namedChildren ?? []];
  };
  const codeArgument = (call: Parser.SyntaxNode): Parser.SyntaxNode | undefined => {
    const [owner, name, args] = callee(call);
    const arg = owner === "FaultCode" && (name === "new" || name === "from") ? args[0] : owner === "Fault" && name === "new" ? args[1] : undefined;
    return arg !== undefined && arg.text.includes("FaultCode") ? undefined : arg;
  };
  const helpers = new Map<string, number | string>();
  for (const fn of nodes.filter((node) => node.type === "function_item" && node.childForFieldName("body") !== null)) {
    const parameters = fn.childForFieldName("parameters")?.namedChildren ?? [];
    if (parameters.some((param) => param.type === "self_parameter")) continue;
    const params = parameters.map((param) => (param.type === "parameter" ? (param.childForFieldName("pattern")?.text ?? null) : null));
    const codeArgs = descendants(fn.childForFieldName("body")!).filter((node) => node.type === "call_expression").flatMap((call) => codeArgument(call) ?? []);
    const position = codeArgs.map((arg) => (arg.type === "identifier" ? params.indexOf(arg.text) : -1)).find((at) => at >= 0);
    const returnsFault = /^(?:\w+::)*Fault$/u.test(fn.childForFieldName("return_type")?.text.replace(/\s/gu, "") ?? "");
    const fixed = codeArgs.length === 1 ? (codeArgs[0]!.type === "string_literal" ? stringValue(codeArgs[0]) : codeArgs[0]!.type === "identifier" ? (consts.get(codeArgs[0]!.text) ?? null) : null) : null;
    if (position !== undefined) helpers.set(fn.childForFieldName("name")!.text, position);
    else if (returnsFault && fixed !== null && /^[A-Za-z][\w-]*(?:\.[A-Za-z0-9][\w-]*)+$/u.test(fixed)) helpers.set(fn.childForFieldName("name")!.text, fixed);
  }
  for (const node of nodes) {
    if (node.type === "call_expression") {
      const [owner, name, args] = callee(node);
      if (owner === "FaultCode" && (name === "new" || name === "from") && value(args[0]) !== null) sites.push({ code: value(args[0]), via: "faultCode", node });
      if (owner === "Fault" && name === "new" && value(args[1]) !== null) sites.push({ code: value(args[1]), via: "faultNew", node });
      if (owner === "Fault" && name === "from" && args[0] !== undefined && (value(args[0]) !== null || args[0].type === "macro_invocation" || /\.(to_string|to_owned)\(\)$/u.test(args[0].text))) sites.push({ code: null, via: "faultFrom", node });
      const helper = owner === "" ? helpers.get(name) : undefined;
      const code = typeof helper === "string" ? helper : helper === undefined ? null : value(args[helper]);
      if (code !== null) sites.push({ code, via: "faultHelper", node });
    }
    if (node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "fault_from_error") {
      const literals = descendants(node).filter((part) => part.type === "string_literal");
      if (literals.length > 0) sites.push({ code: stringValue(literals[0]), via: "faultMacro", node });
    }
    if (node.type === "function_item" && ["code", "fault_code"].includes(node.childForFieldName("name")?.text ?? "")) {
      for (const part of descendants(node.childForFieldName("body")!).filter((part) => part.type === "string_literal" || part.type === "identifier")) {
        const code = value(part);
        if (code !== null && /^[A-Za-z][\w-]*(?:\.[A-Za-z0-9][\w-]*)+$/u.test(code)) sites.push({ code, via: "codeFn", node: part });
      }
    }
  }
  const enclosing = (node: Parser.SyntaxNode): Parser.SyntaxNode | null => {
    let cursor = node.parent;
    while (cursor !== null && cursor.type !== "function_item") cursor = cursor.parent;
    return cursor;
  };
  return sites
    .filter(({ node }) => !testOnly.some((item) => item.startIndex <= node.startIndex && node.endIndex <= item.endIndex))
    .sort((left, right) => left.node.startIndex - right.node.startIndex)
    .map(({ code, via, node }) => {
      const fn = enclosing(node);
      return { code, via, within: fn?.childForFieldName("name")?.text ?? null, defaulted: fn?.parent?.type === "declaration_list" && fn.parent.parent?.type === "trait_item" };
    });
}

describe("📢️ the schema-fault-notice gate", () => {
  test("the fixture satisfies its schema", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("every snippet's sites are what the token gate and the tree-sitter oracle both read", () => {
    for (const row of fixture.cases) {
      const consts = rustStringConsts(row.rust);
      const gate = rustFaultCodeSites("✏️s/🔌️plugins/🧪️demo/🦀️.rs", row.rust, consts, rustFaultHelpers(row.rust, consts)).map(({ code, via, within, defaulted }) => ({ code, via, within, defaulted }));
      expect(gate.map(({ code, via }) => ({ code, via })), `${row.id} (gate)`).toEqual([...row.sites]);
      expect(oracleSites(row.rust), `${row.id} (oracle)`).toEqual(gate);
    }
  });

  test("the planted plugin and SDK yield exactly their findings and census rows, overall and in scope, so the gate fails on them", () => {
    const root = mkdtempSync(join(tmpdir(), "s3-notices-gate-"));
    try {
      for (const [path, text] of Object.entries(fixture.plugin.files)) {
        mkdirSync(dirname(join(root, path)), { recursive: true });
        writeFileSync(join(root, path), text);
      }
      const report = faultNoticeReport(root);
      expect(report.diagnostics.map((entry) => [entry.detail.slice(0, entry.detail.indexOf(":")), entry.detail.slice(entry.detail.indexOf(":") + 2)])).toEqual(fixture.plugin.findings.map((finding) => [...finding]));
      expect(report.diagnostics.every((entry) => entry.code === "schema-fault-notice")).toBe(true);
      const scoped = faultNoticeReport(root, "", fixture.plugin.scoped.scope);
      expect(scoped.diagnostics.map((entry) => [entry.detail.slice(0, entry.detail.indexOf(":")), entry.detail.slice(entry.detail.indexOf(":") + 2)])).toEqual(fixture.plugin.scoped.findings.map((finding) => [...finding]));
      expect(report.census.map(({ owner, codes, labelled, declared, anonymous, findings }) => ({ owner, codes, labelled, declared, anonymous, findings, scoped: scoped.census.find((row) => row.owner === owner)!.findings }))).toEqual(fixture.plugin.census.map((row) => ({ ...row })));
      expect(fixture.plugin.census.reduce((sum, row) => sum + row.findings, 0)).toBe(fixture.plugin.findings.length);
      expect(fixture.plugin.census.reduce((sum, row) => sum + row.scoped, 0)).toBe(fixture.plugin.scoped.findings.length);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
