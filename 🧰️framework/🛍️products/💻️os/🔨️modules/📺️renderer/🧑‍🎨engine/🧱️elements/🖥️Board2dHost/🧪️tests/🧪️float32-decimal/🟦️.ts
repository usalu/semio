// #region 🧲️Header
/** 🧫️ The shared f32-decimal corpus: Ajv validates it against its schema of record, then every case replays through
 * React's `board2dFloat32Decimal` — the same corpus the board engine's `board_pointer_offset` replays in
 * `⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine`. */
// #endregion 🧲️Header

// #region 🔌️Adapters

import { describe, expect, it } from "vitest";
import corpus from "../../🧫️fixtures/🧫️float32-decimal/🔣️.json" with { type: "json" };
import { board2dFloat32Decimal } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧫️Corpus
describe("board f32 decimal corpus", () => {
  

  for (const entry of corpus.cases) {
    it(`canonicalizes: ${entry.name}`, () => {
      expect(board2dFloat32Decimal(entry.value)).toBe(entry.expect);
    });
  }

  it("keeps a non-finite reading as it is", () => {
    expect(Number.isNaN(board2dFloat32Decimal(Number.NaN))).toBe(true);
  });
});
//#endregion 🧫️Corpus
