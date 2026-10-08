/** 🧪️ Third-party and independent oracle of the markdown splice corpus (`🧫️fixtures/✂️splice-source`): ajv validates every payload against the leaf's JSON schema, an independent scalar splice rebuilds the source each row means, and markdown-it (an independent CommonMark parser) proves that every top-level block the ranges do not touch survives in the edited source. The Rust editor laws replay the same corpus through the leaf's sparse diff. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import MarkdownIt from "markdown-it";
import corpus from "../../🧫️fixtures/✂️splice-source/🔣️.json";
import schema from "../../🧬️schema/🧬️mutations/✂️splice-source/🧬️schema/🔣️.json";

type Splice = { readonly offset: number; readonly delete: number; readonly insert: string };
type Block = { readonly source: string; readonly start: number; readonly end: number };

const markdown = new MarkdownIt("commonmark");

/** ✂️ The source the ranges mean, or `null` when a range leaves the text or overlaps the one before. */
function spliced(before: string, splices: readonly Splice[]): string | null {
  const scalars = Array.from(before);
  const parts: string[] = [];
  let position = 0;
  for (const splice of splices) {
    if (splice.offset < position || splice.offset + splice.delete > scalars.length) return null;
    parts.push(scalars.slice(position, splice.offset).join(""), splice.insert);
    position = splice.offset + splice.delete;
  }
  parts.push(scalars.slice(position).join(""));
  return parts.join("");
}

/** 🧱️ The top-level blocks of `text` as markdown-it parses them: the source lines each spans and its scalar range. */
function topLevelBlocks(text: string): Block[] {
  const lines = text.split("\n");
  const starts = lines.reduce<number[]>((all, line, index) => [...all, index === 0 ? 0 : all[index - 1]! + Array.from(lines[index - 1]!).length + 1], []);
  return markdown
    .parse(text, {})
    .filter((token) => token.level === 0 && token.nesting >= 0 && token.map !== null)
    .map((token) => {
      const [first, last] = token.map!;
      return { source: lines.slice(first, last).join("\n"), start: starts[first]!, end: (starts[last] ?? Array.from(text).length + 1) - 1 };
    });
}

describe("markdown splices (ajv + markdown-it oracle)", () => {
  test("every payload conforms to the leaf schema (ajv)", () => {
    const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
    for (const row of corpus.cases) expect(validate({ mutation: "spliceSource", splices: row.splices }), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: the ranges mean the corpus source and untouched blocks survive`, () => {
      expect(spliced(row.before, row.splices)).toBe(row.after);
      const kept = topLevelBlocks(row.after).map((block) => block.source);
      const touched = (block: Block) => row.splices.some((splice) => splice.offset <= block.end && splice.offset + splice.delete >= block.start);
      for (const block of topLevelBlocks(row.before).filter((block) => !touched(block))) expect(kept, `${row.id}: ${block.source}`).toContain(block.source);
    });
  }
});
