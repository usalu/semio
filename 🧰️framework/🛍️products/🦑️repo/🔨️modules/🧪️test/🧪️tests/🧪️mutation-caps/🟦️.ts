/** 🐘️ The structural cap rule (audit F3) over `🧫️fixtures/🧫️mutation-caps/🔣️.json` every
 * planted case yields exactly the listed `capLawMissing` directories, and an independent oracle reaches the same — Ajv tells a
 * bounded leaf by the schema's `boundedLeaf` definition, and tree-sitter-rust reads every enum that derives `Mutations`, its
 * variants and whether it declares type parameters. */
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Parser from "web-tree-sitter";
import { mutationAggregateGeneric, mutationCapFindings, mutationLeafBound, rustMutationAggregates } from "../../🧬️schema/📋️orchestration/🟦️.ts";

type Case = { readonly id: string; readonly leaves: readonly { readonly directory: string; readonly variant: string; readonly input: { readonly schema: Record<string, unknown> } }[]; readonly aggregates: readonly { readonly path: string; readonly rust: string }[]; readonly findings: readonly string[] };

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const read = (path: string): Record<string, unknown> => JSON.parse(readFileSync(resolve(import.meta.dir, path), "utf8")) as Record<string, unknown>;
const fixture = read("../../🧫️fixtures/🧫️mutation-caps/🔣️.json") as unknown as { readonly cases: readonly Case[] };

const schema = read("../../🧬️schema/🔣️mutation-caps/🔣️.json") as Record<string, unknown> & { readonly $id: string };
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);

await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repoRoot)), "out", "tree-sitter-rust.wasm")));

/** 🌳️ Every node of `node`'s subtree, `node` first. */
function descendants(node: Parser.SyntaxNode): Parser.SyntaxNode[] {
  return [node, ...node.namedChildren.flatMap(descendants)];
}

/** 🗿️ The artifact tree of a path (`…/🗿️artifacts/<artifact>`), else `null`. */
function artifact(path: string): string | null {
  const segments = path.split("/");
  const at = segments.indexOf("🗿️artifacts");
  return at >= 0 && at + 1 < segments.length ? segments.slice(0, at + 2).join("/") : null;
}

/** 🔮️ The independent oracle: the directories of the bounded leaves (Ajv, `boundedLeaf`) that no non-generic enum deriving
 * `Mutations` (tree-sitter-rust: its attribute items, variants and type parameters) of the same artifact names as a variant. */
function oracle(entry: Case): string[] {
  const bounded = ajv.getSchema(`${schema.$id}#/definitions/boundedLeaf`)!;
  const derived = entry.aggregates.flatMap(({ path, rust }) =>
    descendants(parser.parse(rust).rootNode)
      .filter((node) => node.type === "enum_item")
      .filter((node) => {
        const attributes: string[] = [];
        for (let sibling = node.previousNamedSibling; sibling?.type === "attribute_item"; sibling = sibling.previousNamedSibling) attributes.push(sibling.text);
        return attributes.some((attribute) => /^#\[derive\(/u.test(attribute) && /\bMutations\b/u.test(attribute));
      })
      .map((node) => ({ path, generic: node.childForFieldName("type_parameters") !== null, variants: descendants(node.childForFieldName("body")!).filter((child) => child.type === "enum_variant").map((child) => child.childForFieldName("name")!.text) })),
  );
  return entry.leaves.filter((leaf) => bounded(leaf.input.schema)).filter((leaf) => !derived.some((aggregate) => !aggregate.generic && artifact(aggregate.path) === artifact(leaf.directory) && aggregate.variants.includes(leaf.variant))).map((leaf) => leaf.directory);
}

describe("🐘️ the structural cap rule", () => {
  test("the examples plant a passing and a failing leaf", () => {
    expect(fixture.cases.some((entry) => entry.findings.length === 0)).toBe(true);
    expect(fixture.cases.filter((entry) => entry.findings.length > 0).length).toBeGreaterThanOrEqual(3);
  });
  for (const entry of fixture.cases) {
    test(`${entry.id}: the gate and the oracle refuse the same leaves`, () => {
      const leaves = entry.leaves.flatMap((leaf) => {
        const cap = mutationLeafBound(leaf.input.schema);
        return cap === null ? [] : [{ directory: leaf.directory, variant: leaf.variant, bounded: cap }];
      });
      const aggregates = entry.aggregates.flatMap(({ path, rust }) => rustMutationAggregates(path, rust).map((aggregate) => ({ path, name: aggregate.name, variants: [...aggregate.variants.keys()], generic: mutationAggregateGeneric(rust, aggregate.name) })));
      const found = mutationCapFindings(leaves, aggregates);
      expect(found.map((finding) => finding.directory)).toEqual([...entry.findings]);
      expect(found.every((finding) => finding.code === "capLawMissing")).toBe(true);
      expect(oracle(entry)).toEqual([...entry.findings]);
    });
  }
});
