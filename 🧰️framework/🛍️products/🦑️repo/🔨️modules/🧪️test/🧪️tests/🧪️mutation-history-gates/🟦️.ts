import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Parser from "web-tree-sitter";
import { type MutationLabelSite, type MutationLeafEditability, mutationEditabilityReport, mutationHandwrittenReason, rustMutationLabelSites } from "../../🧬️schema/📋️orchestration/🟦️.ts";

type Site = Pick<MutationLabelSite, "trait" | "implementor" | "verdict" | "texts">;
type LabelCase = { readonly id: string; readonly rust: string; readonly sites: readonly Site[] };
type EditabilityCase = { readonly kind: string; readonly verdict: MutationLeafEditability["verdict"] };

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const labels = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧫️mutation-labels/🔣️.json"), "utf8")) as { readonly cases: readonly LabelCase[] };
const editability = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🧫️mutation-editability/🔣️.json"), "utf8")) as { readonly aggregate: string; readonly cases: readonly EditabilityCase[] };

await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repoRoot)), "out", "tree-sitter-rust.wasm")));

/** 🌳️ Every node of `node`'s subtree, `node` first. */
function descendants(node: Parser.SyntaxNode): Parser.SyntaxNode[] {
  return [node, ...node.namedChildren.flatMap(descendants)];
}

/** 🏷️ The last path segment of a tree-sitter type node (`a::B<C>` → `B`). */
function typeName(node: Parser.SyntaxNode | null): string {
  if (node === null) return "";
  if (node.type === "generic_type") return typeName(node.childForFieldName("type"));
  return node.type === "scoped_type_identifier" ? (node.childForFieldName("name")?.text ?? "") : node.text;
}

/** 🔤️ The text of the first string literal under `node`, quotes stripped. */
function firstLiteral(node: Parser.SyntaxNode): string | null {
  const literal = descendants(node).find((part) => part.type === "string_literal");
  return literal === undefined ? null : literal.text.slice(1, -1);
}

/** 📞️ The callee name of a call and the path it is scoped under (`a::B::c(…)` → `["c", "a::B"]`, `x.c(…)` → `["c", ""]`). */
function callee(call: Parser.SyntaxNode): readonly [string, string] {
  const target = call.childForFieldName("function");
  if (target?.type === "scoped_identifier") return [target.childForFieldName("name")?.text ?? "", target.childForFieldName("path")?.text ?? ""];
  if (target?.type === "field_expression") return [target.childForFieldName("field")?.text ?? "", ""];
  return [target?.text ?? "", ""];
}

/**
 * 🔮️ The independent oracle: the label verdict of one function body read from the tree-sitter-rust syntax tree — every
 * `LocalizedLabel::native` call takes two locales, neither the empty literal; `LocalizedLabel::data` and `print_op` are findings; a
 * body without `native` forwards to a `label` call or is an empty `match *self {}`.
 */
function oracleVerdict(body: Parser.SyntaxNode): Pick<Site, "verdict" | "texts"> {
  const found = new Set<string>();
  const texts: (readonly [string | null, string | null])[] = [];
  let forward = false;
  for (const node of descendants(body)) {
    if (node.type === "call_expression") {
      const [name, path] = callee(node);
      const localized = /(^|::)LocalizedLabel$/u.test(path);
      if (localized && name === "native") {
        const args = node.childForFieldName("arguments")?.namedChildren ?? [];
        if (args.length !== 2) found.add("labelUnresolved");
        else {
          texts.push([firstLiteral(args[0]!), firstLiteral(args[1]!)]);
          if (args.some((arg) => (arg.type === "reference_expression" ? arg.childForFieldName("value")! : arg).text === '""')) found.add("labelLocaleEmpty");
        }
      } else if (localized && name === "data") found.add("labelLocaleInvariant");
      else if (/(^|_)label$/u.test(name)) forward = true;
    }
    if ((node.type === "identifier" || node.type === "field_identifier") && node.text === "print_op") found.add("labelOpText");
  }
  const match = descendants(body).find((node) => node.type === "match_expression");
  const uninhabited = match !== undefined && match.childForFieldName("value")?.text === "*self" && (match.childForFieldName("body")?.namedChildCount ?? 1) === 0 && descendants(body).filter((node) => node.type === "call_expression").length === 0;
  const worst = ["labelOpText", "labelLocaleInvariant", "labelLocaleEmpty", "labelUnresolved"].find((verdict) => found.has(verdict)) as Site["verdict"] | undefined;
  return { verdict: worst ?? (texts.length > 0 ? "native" : forward || uninhabited ? "forward" : "labelUnresolved"), texts };
}

/** 🔮️ Every label site of one Rust source, by the tree-sitter oracle, in the token gate's order: impl sites, then data sites. */
function oracleSites(source: string): Site[] {
  const tree = parser.parse(source);
  const sites: Site[] = [];
  for (const impl of descendants(tree.rootNode).filter((node) => node.type === "impl_item")) {
    const trait = typeName(impl.childForFieldName("trait"));
    const implementor = typeName(impl.childForFieldName("type"));
    for (const fn of impl.childForFieldName("body")?.namedChildren.filter((node) => node.type === "function_item") ?? []) {
      const name = fn.childForFieldName("name")?.text;
      const body = fn.childForFieldName("body");
      if (body === null) continue;
      if (["MutationKind", "CompositeMutationKind", "SemanticMutation"].includes(trait) && name === "label") sites.push({ trait, implementor, ...oracleVerdict(body) });
      if (["ArtifactApp", "ArtifactEditor", "ArtifactViewer"].includes(trait) && name === "mutation_label") sites.push({ trait, implementor, verdict: "labelOverride", texts: [] });
    }
  }
  for (const call of descendants(tree.rootNode).filter((node) => node.type === "call_expression")) {
    const [name, path] = callee(call);
    if (name === "data" && /(^|::)LocalizedLabel$/u.test(path) && descendants(call.childForFieldName("arguments")!).some((node) => node.text === "print_op" && (node.type === "identifier" || node.type === "field_identifier"))) sites.push({ trait: "data", implementor: "", verdict: "labelOpText", texts: [] });
  }
  tree.delete();
  return sites;
}

const plain = (sites: readonly MutationLabelSite[]): Site[] => sites.map(({ trait, implementor, verdict, texts }) => ({ trait, implementor, verdict, texts }));

describe("schema-mutation-label corpus", () => {
  test("the corpus covers every verdict the gate reaches", () => {
    const verdicts = new Set(labels.cases.flatMap((entry) => entry.sites.map((site) => site.verdict)));
    expect([...verdicts].sort()).toEqual(["forward", "labelLocaleEmpty", "labelLocaleInvariant", "labelOpText", "labelOverride", "labelUnresolved", "native"]);
  });
  for (const entry of labels.cases) {
    test(`${entry.id}: the token gate reaches the corpus verdict`, () => {
      expect(plain(rustMutationLabelSites(`${entry.id}.rs`, entry.rust))).toEqual([...entry.sites]);
    });
    test(`${entry.id}: the tree-sitter-rust oracle reaches the same verdict`, () => {
      expect(oracleSites(entry.rust)).toEqual([...entry.sites]);
    });
  }
});

describe("schema-mutation-editability", () => {
  test("every counter fixture leaf reaches the verdict the Rust law reaches at runtime", () => {
    const report = mutationEditabilityReport(repoRoot, editability.aggregate, [editability.aggregate]);
    const verdicts = Object.fromEntries(report.leaves.filter((leaf) => leaf.aggregate === "CounterMutation").map((leaf) => [leaf.kind, leaf.verdict]));
    expect(verdicts).toEqual(Object.fromEntries(editability.cases.map((entry) => [entry.kind, entry.verdict])));
    expect(report.diagnostics).toEqual([]);
  });
  test("a hand-written aggregate declares why it is not edited, or is a finding", () => {
    const body = (source: string) => [...source.matchAll(/[\p{L}_][\p{L}\p{N}_]*/gu)].map(([text]) => ({ kind: "ident" as const, text }));
    const forwarding = "impl Mutation<PlaySnapshot> for M { const INPUT_SCHEMAS = X; fn input_schema() {} fn payload_value() {} fn with_payload_value() {} fn from_payload_value() {} }";
    expect(mutationHandwrittenReason("✏️s/🔌️plugins/p/🧬️schema/🧬️mutations/🦀️.rs", "M", "PlaySnapshot", body(forwarding), forwarding)).toBe("forwarding");
    expect(mutationHandwrittenReason("✏️s/🔌️plugins/p/✏️editor/🎚️config/🦀️.rs", "PConfigMutation", "PConfig", body("impl"), "impl")).toBe("lane");
    expect(mutationHandwrittenReason("🧰️framework/x/🧪️tests/🔬️unit/🦀️.rs", "DemoMutation", "DemoSnapshot", body("impl"), "impl")).toBe("fixture");
    expect(mutationHandwrittenReason("🧰️framework/x/🦀️.rs", "NoMutation", "NoState", body("impl"), "pub enum NoMutation {}")).toBe("empty");
    expect(mutationHandwrittenReason("🧰️framework/x/🦀️.rs", "SpaceMutation", "SpaceSnapshot", body("impl"), "pub enum SpaceMutation { A }")).toBe("aggregateHandwritten");
  });
});
