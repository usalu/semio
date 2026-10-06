/** 🧪️ Third-party oracle of the markdown net-leaves corpus (`🧫️fixtures/🧫️net-leaves`): ajv validates individual mutation records, markdown-it (an independent CommonMark parser) splits both texts into top-level blocks, and jsdiff's array diff finds
 * the blocks an Apply changed — every expected leaf must address a changed block of the right kind, and an unchanged text must
 * mean no leaf at all. The Rust editor laws replay the same corpus against `md_net_mutations`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { diffArrays } from "diff";
import MarkdownIt from "markdown-it";
import corpus from "../../🧫️fixtures/🧫️net-leaves/🔣️.json";
import schema from "../../🧬️schema/🔣️net-leaves/🔣️.json";

type Leaf = { readonly kind: string; readonly path: readonly string[]; readonly index: number };
type Block = { readonly type: string; readonly tag: string; readonly source: string };

const markdown = new MarkdownIt("commonmark");

/** 🧱️ The top-level blocks of `text` as markdown-it parses them: opening token type, tag and the source lines it spans. */
function topLevelBlocks(text: string): Block[] {
  const lines = text.split("\n");
  return markdown
    .parse(text, {})
    .filter((token) => token.level === 0 && token.nesting >= 0 && token.map !== null)
    .map((token) => ({ type: token.type, tag: token.tag, source: lines.slice(token.map![0], token.map![1]).join("\n") }));
}

/** ✂️ How many blocks the two texts share at their start and at their end, by jsdiff's own array diff. */
function unchangedEnds(before: readonly Block[], after: readonly Block[]): { readonly prefix: number; readonly suffix: number } {
  const chunks = diffArrays(before as Block[], after as Block[], { comparator: (left: Block, right: Block) => left.source === right.source });
  const kept = (chunk: (typeof chunks)[number] | undefined) => (chunk !== undefined && !chunk.added && !chunk.removed ? chunk.count ?? 0 : 0);
  const prefix = kept(chunks[0]);
  return { prefix, suffix: chunks.length > 1 ? kept(chunks.at(-1)) : 0 };
}

describe("markdown net leaves (markdown-it + jsdiff oracle)", () => {
  test("actual mutation records conform to their domain schema (ajv)", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    for (const row of corpus.cases) for (const leaf of row.leaves ?? []) expect(validate(leaf), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: every leaf addresses a changed block of its kind`, () => {
      const before = topLevelBlocks(row.before);
      const after = topLevelBlocks(row.after);
      const { prefix, suffix } = unchangedEnds(before, after);
      const leaves = row.leaves as readonly Leaf[];
      expect(leaves.length === 0).toBe(before.length === after.length && before.every((block, index) => block.source === after[index]!.source));
      for (const leaf of leaves) {
        const top = leaf.path.length === 0 ? leaf.index : Number(leaf.path[0]!.split(":")[1]);
        const inserted = leaf.kind === "insert-block" && leaf.path.length === 0;
        expect(top).toBeGreaterThanOrEqual(prefix);
        expect(top).toBeLessThan((inserted ? after.length : before.length) - suffix);
        if (leaf.path.length > 0) {
          const container = leaf.path[0]!.startsWith("quote:") ? "blockquote_open" : "bullet_list_open";
          expect([before[top]!.type, after[top]!.type]).toEqual([container, container]);
        } else if (leaf.kind === "set-inlines") {
          expect(before[top]!.type).toBe(after[top]!.type);
          expect(before[top]!.tag).toBe(after[top]!.tag);
          expect(["paragraph_open", "heading_open"]).toContain(after[top]!.type);
        } else if (leaf.kind === "replace-block") {
          expect(before[top]!.source).not.toBe(after[top]!.source);
        }
      }
    });
  }
});
