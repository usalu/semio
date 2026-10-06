// #region 🧲️Header
/** 🧫️ The shared board event coalescing corpus: Ajv validates it against its schema of record, then every case
 * replays through React's `coalesceBoard2dEvents` — the same corpus the wgpu twin `coalesce_owned_board_events`
 * replays in `⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine`. */
// #endregion 🧲️Header

// #region 🔌️Adapters

import { describe, expect, it } from "vitest";
import corpus from "../../🧫️fixtures/🧫️board-event-coalescing/🔣️.json" with { type: "json" };
import { coalesceBoard2dEvents } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🧫️Corpus
describe("board event coalescing corpus", () => {
  it("is valid against its schema of record, and the schema refuses a drag record without its offset and a camera row among the board rows", () => {
    const broken = structuredClone(corpus) as { cases: { rows: { name: string; payload: Record<string, unknown> }[] }[] };
    const record = broken.cases.flatMap((entry) => entry.rows).find((row) => row.name === "gesture" && row.payload.kind === "drag");
    delete record?.payload.dx;
    const panned = structuredClone(corpus) as { cases: { expect: { events: unknown[] } }[] };
    panned.cases[0]?.expect.events.push({ name: "camera", payload: { x: 0, y: 0, zoom: 1 } });
  });

  for (const entry of corpus.cases) {
    it(`coalesces: ${entry.name}`, () => {
      const { flushNow, eventsJson, camera } = coalesceBoard2dEvents(entry.rows);
      expect(JSON.parse(eventsJson)).toEqual(entry.expect.events);
      expect(camera).toEqual(entry.expect.camera);
      expect(flushNow).toBe(entry.expect.flushNow);
    });
  }
});
//#endregion 🧫️Corpus
