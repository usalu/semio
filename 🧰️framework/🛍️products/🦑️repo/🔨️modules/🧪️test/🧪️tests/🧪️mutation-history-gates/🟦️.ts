import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Parser from "web-tree-sitter";
import { type MutationLabelSite, type MutationLeafEditability, mutationEditabilityReport, mutationHandwrittenReason, mutationLabelReport, mutationLeafUnpublishedReferences, mutationSchemaDocumentIndex, mutationSchemaSearchRoot, rustMutationLabelSites } from "../../🧬️schema/📋️orchestration/🟦️.ts";

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

/** 🏷️ The last path segment of a tree-sitter type node (`a::B<C>` → `B`, `B::<C>` → `B`). */
function typeName(node: Parser.SyntaxNode | null): string {
  if (node === null) return "";
  if (node.type === "generic_type" || node.type === "generic_type_with_turbofish") return typeName(node.childForFieldName("type"));
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
  for (const node of descendants(tree.rootNode)) {
    if (node.type === "call_expression") {
      const [name, path] = callee(node);
      const label = node.childForFieldName("arguments")?.namedChildren[1];
      if (name === "commit" && /(^|::)Emit(::<.*>)?$/su.test(path)) sites.push({ trait: "Emit", implementor: "commit", verdict: "labelHandwritten", texts: [[label === undefined ? null : firstLiteral(label), null]] });
    } else if (node.type === "struct_expression" && typeName(node.childForFieldName("name")) === "Emit") {
      for (const field of node.childForFieldName("body")?.namedChildren ?? []) {
        if (field.type === "shorthand_field_initializer" && field.text === "description") sites.push({ trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[null, null]] });
        const value = field.type === "field_initializer" && field.childForFieldName("name")?.text === "description" ? field.childForFieldName("value") : null;
        if (value !== null && value.text !== "None") sites.push({ trait: "Emit", implementor: "description", verdict: "labelHandwritten", texts: [[firstLiteral(value), null]] });
      }
    }
  }
  tree.delete();
  return sites;
}

const plain = (sites: readonly MutationLabelSite[]): Site[] => sites.map(({ trait, implementor, verdict, texts }) => ({ trait, implementor, verdict, texts }));

describe("schema-mutation-label corpus", () => {
  test("the corpus covers every verdict the gate reaches", () => {
    const verdicts = new Set(labels.cases.flatMap((entry) => entry.sites.map((site) => site.verdict)));
    expect([...verdicts].sort()).toEqual(["forward", "labelHandwritten", "labelLocaleEmpty", "labelLocaleInvariant", "labelOpText", "labelOverride", "labelUnresolved", "native"]);
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

describe("schema-mutation-label gate", () => {
  test("a planted hand-written commit label fails the gate; the same emission labelled by its leaf, and test sources, pass", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-label-gate-"));
    try {
      const editor = "✏️s/🔌️plugins/planted/🗿️artifacts/planted/✏️editor";
      const leaf = "impl MutationKind<Snap, Op> for Delete {\n    fn label(&self) -> LocalizedLabel {\n        LocalizedLabel::native(\"Delete\", \"Löschen\")\n    }\n}\n";
      const planted = "fn delete(mutations: Vec<Op>) -> Result<Emit<Op, Cfg>, Fault> {\n    Ok(Emit::commit(mutations, \"Delete objects\"))\n}\nfn fill(op: Op) -> Emit<Op, Cfg> {\n    Emit { artifact_mutations: vec![op], description: Some(\"Fill\".into()), ..Default::default() }\n}\n";
      mkdirSync(join(root, editor, "🧪️tests"), { recursive: true });
      writeFileSync(join(root, editor, "🦀️.rs"), leaf + planted);
      writeFileSync(join(root, editor, "🧪️tests", "🦀️.rs"), planted);
      const failing = mutationLabelReport(root);
      expect(failing.diagnostics.map((entry) => [entry.path, entry.detail])).toEqual([
        [`${editor}/🦀️.rs`, 'labelHandwritten: Emit::commit(mutations, label) [["Delete objects",null]]'],
        [`${editor}/🦀️.rs`, 'labelHandwritten: Emit { description: … } [["Fill",null]]'],
      ]);
      expect(failing.census.reduce((sum, row) => sum + row.labels, 0)).toBe(1);
      writeFileSync(join(root, editor, "🦀️.rs"), leaf + planted.replace(', "Delete objects"', "").replace("Emit::commit(", "Emit::mutations(").replace(', description: Some("Fill".into())', ""));
      expect(mutationLabelReport(root).diagnostics).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("schema-mutation-editability", () => {
  test("every counter fixture leaf reaches the verdict the Rust law reaches at runtime", () => {
    const report = mutationEditabilityReport(repoRoot, editability.aggregate, [editability.aggregate]);
    const verdicts = Object.fromEntries(report.leaves.filter((leaf) => leaf.aggregate === "CounterMutation").map((leaf) => [leaf.kind, leaf.verdict]));
    expect(verdicts).toEqual(Object.fromEntries(editability.cases.map((entry) => [entry.kind, entry.verdict])));
    expect(report.diagnostics).toEqual([]);
  }, 60_000);
  test("a hand-written aggregate declares why it is not edited, or is a finding", () => {
    const body = (source: string) => [...source.matchAll(/[\p{L}_][\p{L}\p{N}_]*/gu)].map(([text]) => ({ kind: "ident" as const, text }));
    const forwarding = "impl Mutation<PlaySnapshot> for M { const INPUT_SCHEMAS = X; fn input_schema() {} fn payload_value() {} fn with_payload_value() {} fn from_payload_value() {} }";
    expect(mutationHandwrittenReason("✏️s/🔌️plugins/p/🧬️schema/🧬️mutations/🦀️.rs", "M", "PlaySnapshot", body(forwarding), forwarding)).toBe("forwarding");
    expect(mutationHandwrittenReason("✏️s/🔌️plugins/p/✏️editor/🎚️config/🦀️.rs", "PConfigMutation", "PConfig", body("impl"), "impl")).toBe("lane");
    expect(mutationHandwrittenReason("🧰️framework/x/🧪️tests/🔬️unit/🦀️.rs", "DemoMutation", "DemoSnapshot", body("impl"), "impl")).toBe("fixture");
    expect(mutationHandwrittenReason("🧰️framework/x/🦀️.rs", "NoMutation", "NoState", body("impl"), "pub enum NoMutation {}")).toBe("empty");
    expect(mutationHandwrittenReason("🧰️framework/x/🦀️.rs", "SpaceMutation", "SpaceSnapshot", body("impl"), "pub enum SpaceMutation { A }")).toBe("aggregateHandwritten");
  });
  test("a leaf's references resolve through its plugin tree, the documents the runtime publishes beside it", () => {
    const forms = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
    const framework = new Map(["🧰️framework", "🌎️hub"].flatMap((root) => [...mutationSchemaDocumentIndex(repoRoot, root)]));
    for (const leaf of ["🔁replace-block", "➕create-block", "🌱create-step"]) {
      const schema = `${forms}/${leaf}/🧬️schema/🔣️.json`;
      expect(mutationSchemaSearchRoot(repoRoot, schema)).toBe("✏️s/🔌️plugins/📋️forms");
      expect(mutationLeafUnpublishedReferences(repoRoot, schema, mutationSchemaDocumentIndex(repoRoot, "✏️s/🔌️plugins/📋️forms"), framework)).toEqual([]);
      expect(mutationLeafUnpublishedReferences(repoRoot, schema, new Map(), new Map())).toEqual(["https://json.schemas.assets.semio-tech.com/s/forms/forms/definition.json"]);
    }
  });
});
