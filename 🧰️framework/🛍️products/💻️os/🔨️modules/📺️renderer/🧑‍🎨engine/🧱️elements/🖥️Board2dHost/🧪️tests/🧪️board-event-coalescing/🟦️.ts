// #region 🧲️Header
/** 🧫️ The shared board event coalescing corpus: Ajv validates it against its schema of record, then every case
 * replays through React's `coalesceBoard2dEvents` — the same corpus the wgpu twin `coalesce_owned_board_events`
 * replays in `⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine`. */
// #endregion 🧲️Header

// #region 🔌️Adapters

import { describe, expect, it } from "vitest";
import corpus from "../🧫️fixtures/🧫️board-event-coalescing/🔣️.json" with { type: "json" };
import { bufferBoard2dEvents, coalesceBoard2dEvents } from "../../📡️board-events/🟦️.ts";
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
      const buffer: Parameters<typeof bufferBoard2dEvents>[0] = [];
      for (const row of entry.rows) bufferBoard2dEvents(buffer, [row]);
      expect(coalesceBoard2dEvents(buffer)).toEqual({ eventsJson, camera, flushNow });
    });
  }

  it("keeps a long preview and camera gesture buffer bounded", () => {
    const buffer: Parameters<typeof bufferBoard2dEvents>[0] = [];
    for (let frame = 0; frame < 2_000; frame++) {
      expect(bufferBoard2dEvents(buffer, [{ name: "nodeMove", payload: { id: "node", x: frame, y: 0 } }, { name: "hover", payload: { id: "node" } }])).toBe(false);
      bufferBoard2dEvents(buffer, [{ name: "camera", payload: { x: frame, y: 0, zoom: 1 } }]);
    }
    expect(buffer).toEqual([{ name: "camera", payload: { x: 1_999, y: 0, zoom: 1 } }]);
  });
});
//#endregion 🧫️Corpus
