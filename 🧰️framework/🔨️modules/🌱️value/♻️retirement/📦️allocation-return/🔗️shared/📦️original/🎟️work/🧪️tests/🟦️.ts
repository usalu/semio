import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import law from "../📐️.json";
import schema from "../🔣️.json";

test("shared original slots reserve allocator and typed placement before transfer", () => {
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  const db = new Database(":memory:");
  try {
    expect(db.query("select 0 deniedWork, 1 nullWork, 1+3 admitWork, 1+1+1+1 slotWork, 1+3 releaseWork, 3 electionWork, 1 payloadWork, 0 cachedWork").get()).toEqual({
      deniedWork: law.deniedWork, nullWork: law.nullWork, admitWork: law.admitWork,
      slotWork: law.slotWork, releaseWork: law.releaseWork, electionWork:law.electionWork,payloadWork:law.payloadWork,cachedWork: law.cachedWork,
    });
    for (const denied of law.slotDenials) {
      expect(db.query("select case when ? in ('items','copy','capacity','depth') then 1 else 0 end originalRetained, 0 writes, 0 heap").get(denied)).toEqual({ originalRetained: Number(law.slotPreservesOriginal), writes: 0, heap: 0 });
    }
  } finally { db.close(); }
});
