/** 🧪️ Third-party oracle of the HTML net-leaves corpus (`🧫️fixtures/🧫️net-leaves`): ajv validates the corpus against its
 * schema, and parse5 (an independent HTML5 parser) parses both texts — every expected leaf must address a node that exists
 * where it says, of the kind it edits, and that really differs; an unchanged text must mean no leaf at all. The Rust editor
 * laws replay the same corpus against `html_net_mutations`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { parse, serialize } from "parse5";
import corpus from "../../🧫️fixtures/🧫️net-leaves/🔣️.json";
import schema from "../../🧬️schema/🔣️net-leaves/🔣️.json";

type Node = { readonly nodeName: string; readonly tagName?: string; readonly value?: string; readonly data?: string; readonly attrs?: readonly { readonly name: string; readonly value: string }[]; readonly childNodes?: readonly Node[] };
type Leaf = { readonly kind: string; readonly at: readonly number[] };

/** 🌳️ The root element of `text` and its doctype name, as parse5 builds them. */
function documentOf(text: string): { readonly root: Node; readonly doctype: string | null } {
  const document = parse(text) as unknown as { readonly childNodes: readonly (Node & { readonly name?: string })[] };
  const doctype = document.childNodes.find((node) => node.nodeName === "#documentType");
  return { root: document.childNodes.find((node) => node.nodeName === "html")!, doctype: doctype?.name ?? null };
}

function nodeAt(root: Node, path: readonly number[]): Node | undefined {
  return path.reduce<Node | undefined>((node, index) => node?.childNodes?.[index], root);
}

const attributes = (node: Node | undefined) => JSON.stringify(node?.attrs ?? []);

describe("html net leaves (parse5 oracle)", () => {
  test("the corpus validates against its schema (ajv)", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: every leaf addresses the node it edits`, () => {
      const before = documentOf(row.before);
      const after = documentOf(row.after);
      const leaves = row.leaves as readonly Leaf[];
      expect(leaves.length === 0).toBe(serialize(parse(row.before)) === serialize(parse(row.after)));
      for (const leaf of leaves) {
        const [old, next] = [nodeAt(before.root, leaf.at), nodeAt(after.root, leaf.at)];
        switch (leaf.kind) {
          case "set-doctype":
            expect(before.doctype).not.toBe(after.doctype);
            break;
          case "set-text":
          case "set-raw-text":
            expect([old?.nodeName, next?.nodeName]).toEqual(["#text", "#text"]);
            expect(old?.value).not.toBe(next?.value);
            break;
          case "set-comment":
            expect([old?.nodeName, next?.nodeName]).toEqual(["#comment", "#comment"]);
            expect(old?.data).not.toBe(next?.data);
            break;
          case "set-attribute":
            expect(old?.tagName).toBe(next?.tagName);
            expect(attributes(old)).not.toBe(attributes(next));
            break;
          case "set-element-name":
            expect(old?.tagName).toBeDefined();
            expect(next?.tagName).toBeDefined();
            expect(old?.tagName).not.toBe(next?.tagName);
            break;
          case "remove-node":
            expect(old).toBeDefined();
            break;
          case "insert-node":
            expect(next).toBeDefined();
            break;
          default:
            throw new Error(`unexpected leaf kind ${leaf.kind}`);
        }
      }
    });
  }
});
