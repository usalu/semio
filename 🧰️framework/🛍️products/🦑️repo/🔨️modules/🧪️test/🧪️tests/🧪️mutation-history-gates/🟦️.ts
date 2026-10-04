import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, posix, resolve } from "node:path";
import Parser from "web-tree-sitter";
import { type MutationLabelSite, type MutationLeafEditability, mutationEditabilityReport, mutationHandwrittenReason, mutationLabelReport, mutationLeafCrateManifest, mutationLeafUnpublishedReferences, mutationManifestPathDependencies, mutationSchemaDocumentIndex, mutationSchemaSearchRoot, mutationSchemaSearchRoots, rustMutationLabelSites, wordOnlyFloatPointers, wordOnlyFloatTwinLines, composedLeafChildReads, composedSnapshot } from "../../🧬️schema/📋️orchestration/🟦️.ts";

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

describe("schema-mutation-input-ui wordOnlyFloat", () => {
  test("a planted artifact twin that reads a float through the strict word parser fails; transport reads and output re-validation pass (TypeScript compiler oracle)", async () => {
    const ts = (await import("typescript")).default;
    const source = [
      'import {parseBinary64,parseBinary32Transport,type Binary64} from "./ieee754/🟦️.ts";',
      "export function parsePoint(row:any){return{x:parseBinary64(row.x),y:parseBinary32Transport(row.y)};}",
      "export const out=(v:Binary64)=>({bits:parseBinary64(v).bits.toString(16)});",
      "export const wide=(v:unknown)=>parseBinary32(v);",
    ].join("\n");
    const oracle: number[] = [];
    const visit = (node: import("typescript").Node, file: import("typescript").SourceFile): void => {
      if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && /^parseBinary(64|32)$/.test(node.expression.text) && !(ts.isPropertyAccessExpression(node.parent) && node.parent.name.text === "bits")) oracle.push(file.getLineAndCharacterOfPosition(node.getStart(file)).line + 1);
      node.forEachChild((child) => visit(child, file));
    };
    const file = ts.createSourceFile("twin.ts", source, ts.ScriptTarget.Latest, true);
    visit(file, file);
    expect(wordOnlyFloatTwinLines(source)).toEqual([2, 4]);
    expect(oracle).toEqual([2, 4]);
    expect(wordOnlyFloatTwinLines(source.replaceAll("parseBinary64(row.x)", "parseBinary64Transport(row.x)").replaceAll("parseBinary32(v)", "parseBinary32Transport(v)"))).toEqual([]);
  });
  test("a planted word-only float fails wherever the payload carries it; the number form and a word beside a number pass", () => {
    const word = { type: "object", additionalProperties: false, required: ["bits"], properties: { bits: { type: "string", pattern: "^[0-9a-f]{16}$" } } };
    const shared = { $id: "https://json.schemas.assets.semio-tech.com/test/word-only/shared.json", $defs: { Binary64: word, Point: { type: "object", properties: { x: { $ref: "#/$defs/Binary64" }, y: { type: "number" } } } } };
    const leaf = {
      type: "object",
      properties: {
        dx: { $ref: "#/$defs/Word", "x-semio-ui": { widget: "stepper", label: { en: "Offset X", de: "Versatz X" } } },
        dy: { type: "number" },
        transport: { anyOf: [{ $ref: "#/$defs/Word" }, { type: "number" }] },
        nullable: { oneOf: [{ $ref: "#/$defs/Word" }, { type: "null" }] },
        points: { type: "array", items: { $ref: "https://json.schemas.assets.semio-tech.com/test/word-only/shared.json#/$defs/Point" } },
        pair: { type: "array", items: [{ type: "number" }, word] },
        tree: { $ref: "#/$defs/Tree", "x-semio-ui": { widget: "hidden" } },
        weights: { type: "object", additionalProperties: { $ref: "#/$defs/Word" } },
      },
      $defs: { Word: word, Tree: { type: "object", properties: { size: { $ref: "#/$defs/Word" }, children: { type: "array", items: { $ref: "#/$defs/Tree" } } } } },
    };
    const resolve = (id: string) => (id === shared.$id ? (shared as Record<string, unknown>) : undefined);
    expect(wordOnlyFloatPointers(leaf, resolve)).toEqual(["/dx", "/nullable", "/pair/1", "/points/-/x", "/tree/size", "/weights/*"]);
    const fixed = JSON.parse(JSON.stringify(leaf).replaceAll('"$ref":"#/$defs/Word"', '"anyOf":[{"$ref":"#/$defs/Word"},{"type":"number"}]')) as Record<string, unknown>;
    expect(wordOnlyFloatPointers(fixed, resolve)).toEqual(["/pair/1", "/points/-/x"]);
  });
});

describe("schema-mutation-editability parentLeafReadsChild", () => {
  test("a planted parent-lane leaf that reads its composed child fails; coordinate-only, composed-on-read and test code pass", () => {
    const sources = [
      { path: "p/🧬️schema/📸️snapshot/🦀️.rs", source: 'pub struct PlantedSnapshot {\n    #[state(artifact)]\n    #[child(kind = "s.stdio.semio")]\n    pub content: PlantedChild,\n}\npub struct FlatSnapshot {\n    pub nodes: Vec<Node>,\n}\n' },
      {
        path: "p/🦀️.rs",
        source: [
          "pub fn planted_scene(snapshot: &PlantedSnapshot) -> Scene { snapshot.content.local_owner::<Scene>().unwrap().as_ref().clone() }",
          "pub fn planted_nodes(snapshot: &PlantedSnapshot) -> usize { planted_scene(snapshot).nodes.len() }",
          'pub fn planted_child(snapshot: &PlantedSnapshot, children: &ChildContentView) -> Scene { children.typed_read::<Scene>("content", &snapshot.content.child_id).unwrap() }',
          "impl MutationKind<PlantedSnapshot, PlantedMutation> for Other { fn inverse(&self, base: &PlantedSnapshot) -> Vec<PlantedMutation> { let _ = base.content.local_owner::<Scene>(); Vec::new() } }",
        ].join("\n"),
      },
      { path: "p/🧬️mutations/🚚️move-nodes/🦀️.rs", source: "impl MutationKind<PlantedSnapshot, PlantedMutation> for MoveNodes { fn diff(&self, base: &PlantedSnapshot) -> Outcome { Outcome::from(planted_nodes(base)) } }\n" },
      { path: "p/🧬️mutations/🏷️rename/🦀️.rs", source: "impl MutationKind<PlantedSnapshot, PlantedMutation> for Rename { fn diff(&self, base: &PlantedSnapshot) -> Outcome { Outcome::from(self.inverse(base)) } }\n" },
      { path: "p/🧬️mutations/🧭️retarget/🦀️.rs", source: "impl MutationKind<PlantedSnapshot, PlantedMutation> for Retarget { fn diff(&self, base: &PlantedSnapshot) -> Outcome { Outcome::from(base.content.child_id.clone()) } }\n" },
      { path: "p/🧬️mutations/🚚️move-nodes/🧪️tests/🦀️.rs", source: "fn law(base: &PlantedSnapshot) { base.content.local_owner::<Scene>(); }\n" },
    ];
    expect([composedSnapshot(sources, "PlantedSnapshot"), composedSnapshot(sources, "FlatSnapshot")]).toEqual([true, false]);
    expect(composedLeafChildReads(sources, ["p/🧬️mutations/🚚️move-nodes", "p/🧬️mutations/🏷️rename", "p/🧬️mutations/🧭️retarget"])).toEqual([{ directory: "p/🧬️mutations/🚚️move-nodes", call: "planted_nodes" }]);
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
  }, 60_000);
  test("a leaf's references resolve through the plugins its crate depends on, never a local copy", () => {
    const layout = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout";
    const schema = `${layout}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🧬️schema/🔣️.json`;
    const framework = new Map(["🧰️framework", "🌎️hub"].flatMap((root) => [...mutationSchemaDocumentIndex(repoRoot, root)]));
    expect(mutationLeafCrateManifest(repoRoot, schema)).toBe(`${layout}/📦️packages/🦀️rust/Cargo.toml`);
    expect(mutationSchemaSearchRoots(repoRoot, schema)).toEqual(["✏️s/🔌️plugins/📏️layout", "✏️s/🔌️plugins/📋️forms"]);
    const union = new Map(mutationSchemaSearchRoots(repoRoot, schema).flatMap((root) => [...mutationSchemaDocumentIndex(repoRoot, root)]).reverse());
    expect(mutationLeafUnpublishedReferences(repoRoot, schema, union, framework)).toEqual([]);
    expect(mutationLeafUnpublishedReferences(repoRoot, schema, mutationSchemaDocumentIndex(repoRoot, "✏️s/🔌️plugins/📏️layout"), framework)).toEqual(["https://json.schemas.assets.semio-tech.com/s/forms/forms/dictionary.json"]);
    expect(mutationSchemaSearchRoots(repoRoot, "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🧬️schema/🔣️.json")).toEqual(["✏️s/🔌️plugins/📋️forms"]);
  }, 60_000);
  test("the manifest dependency reader agrees with Bun's TOML parser on every artifact crate", () => {
    const manifests = [...new Bun.Glob("✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml").scanSync({ cwd: repoRoot })].sort();
    expect(manifests.length).toBeGreaterThan(40);
    for (const manifest of manifests) {
      const text = readFileSync(join(repoRoot, manifest), "utf8");
      const oracle = Object.values(((Bun.TOML.parse(text) as { dependencies?: Record<string, unknown> }).dependencies ?? {}) as Record<string, { path?: string }>).flatMap((entry) => (typeof entry === "object" && typeof entry.path === "string" ? [posix.normalize(posix.join(posix.dirname(manifest), entry.path))] : []));
      expect({ manifest, paths: mutationManifestPathDependencies(manifest, text).sort() }).toEqual({ manifest, paths: oracle.sort() });
    }
  }, 60_000);
  test("the manifest dependency reader reads the derive's language-agnostic cases like Bun's TOML parser", () => {
    const vectors = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🧫️manifest-path-dependencies/🔣️.json"), "utf8")) as { readonly cases: readonly { readonly name: string; readonly manifest: string; readonly text: string; readonly paths: readonly string[] }[] };
    expect(vectors.cases.length).toBeGreaterThan(3);
    for (const vector of vectors.cases) {
      const oracle = Object.values(((Bun.TOML.parse(vector.text) as { dependencies?: Record<string, unknown> }).dependencies ?? {}) as Record<string, { path?: string }>).flatMap((entry) => (typeof entry.path === "string" ? [posix.normalize(posix.join(posix.dirname(vector.manifest), entry.path))] : []));
      expect({ name: vector.name, paths: mutationManifestPathDependencies(vector.manifest, vector.text) }).toEqual({ name: vector.name, paths: [...vector.paths] });
      expect({ name: vector.name, paths: oracle }).toEqual({ name: vector.name, paths: [...vector.paths] });
    }
  }, 60_000);
});
