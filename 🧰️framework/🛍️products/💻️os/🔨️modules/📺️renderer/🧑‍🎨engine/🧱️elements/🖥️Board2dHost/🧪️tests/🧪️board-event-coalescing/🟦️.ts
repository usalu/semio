// #region 🧲️Header
/** 🧫️ The shared board event coalescing corpus: Ajv validates it against its schema of record, then every case
 * replays through React's `coalesceBoard2dEvents` — the same corpus the wgpu twin `coalesce_owned_board_events`
 * replays in `⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import corpus from "../../🧫️fixtures/🧫️board-event-coalescing/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️board-event-coalescing/🔣️.json" with { type: "json" };
import { coalesceBoard2dEvents } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧫️Corpus
describe("board event coalescing corpus", () => {
  it("is valid against its schema of record, and the schema refuses a drag record without its offset", () => {
    const validate = new Ajv({ strict: true, allowUnionTypes: true }).compile(schema);
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    const broken = structuredClone(corpus) as { cases: { rows: { name: string; payload: Record<string, unknown> }[] }[] };
    const record = broken.cases.flatMap((entry) => entry.rows).find((row) => row.name === "gesture" && row.payload.kind === "drag");
    delete record?.payload.dx;
    expect(validate(broken)).toBe(false);
  });

  for (const entry of corpus.cases) {
    it(`coalesces: ${entry.name}`, () => {
      const { flushNow, eventsJson } = coalesceBoard2dEvents(entry.rows);
      expect(JSON.parse(eventsJson)).toEqual(entry.expect.events);
      expect(flushNow).toBe(entry.expect.flushNow);
    });
  }
});
//#endregion 🧫️Corpus
