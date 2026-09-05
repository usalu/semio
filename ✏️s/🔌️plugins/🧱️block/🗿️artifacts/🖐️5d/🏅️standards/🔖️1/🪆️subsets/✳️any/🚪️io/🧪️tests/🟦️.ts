/** 🧪️ block5d io — cross-language parity for the `🔣️json` and `🔤️txt` leaves.
 *
 * `🧫️fixtures/*.json` is the single shared oracle: the Rust `🚪️io/🦀️.rs` test
 * `json_matches_the_typescript_parity_fixture` asserts the same files from `Block5dIntoJson`, so a
 * disagreement between the two implementations fails on both sides instead of drifting silently.
 */

import { describe, expect, test } from "bun:test";
import { block5dToJsonText } from "../📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block5dCanonicalJsonText } from "../📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️";
import { block5dFromDslText } from "../📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️";

const FIXTURES = [
  { asset: "hexagonal-cut-concrete-forest-left", fixture: "⬅️hexagonal-cut-concrete-forest-left.json", example: "🌲️hexagonal-cut-concrete-forest-left" },
  { asset: "nakagin-capsule", fixture: "🏢️nakagin-capsule.json", example: "🏢️nakagin-capsule" },
] as const;

async function read(path: string): Promise<string> {
  return Bun.file(new URL(path, import.meta.url)).text();
}

describe("block5d io", () => {
  for (const { asset, example, fixture } of FIXTURES) {
    test(`${asset}: the TypeScript json writer is a fixed point on the Rust bytes`, async () => {
      const expected = await read(`./🧫️fixtures/${fixture}`);
      expect(block5dCanonicalJsonText(expected)).toBe(expected);
    });

    test(`${asset}: the TypeScript dsl reader + json writer reproduce the Rust bytes`, async () => {
      const expected = await read(`./🧫️fixtures/${fixture}`);
      const dsl = await read(`../../📚️examples/${example}/🖼️assets/${example}/🗣️.dsl.semio`);
      expect(block5dToJsonText(block5dFromDslText(dsl))).toBe(expected);
    });
  }
});
