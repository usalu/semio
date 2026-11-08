import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../🔣️key-query.json";
import schema from "../🧬️key-query.json";

test("a recorded key reply retains its original Source and is not replayed", () => {
  expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  try {
    const row = db.query("SELECT json_array_length(?1) AS operations, ?2 + ?3 AS callbacks, ?4 AS foreignSource, ?5 AS seedParts, ?6 AS maxPartCopy, json_array_length(?7) AS cancelCuts").get(JSON.stringify(law.operations), law.sourceCalls, law.recoverCalls, Number(law.foreignSourceAccepted), law.seedParts, law.maxPartCopy, JSON.stringify(law.seedCancelCuts));
    expect(row).toEqual({ operations: 6, callbacks: 1, foreignSource: 0, seedParts: 90, maxPartCopy: 4096, cancelCuts: 93 });
    const posts = db.query("SELECT count(*) AS accepted FROM json_each(?1) WHERE value = ?2").get(JSON.stringify(law.acknowledgementPhases), "acceptPost");
    expect(posts).toEqual({ accepted: 4 });
    expect(law.semanticFailureAcknowledged).toBe(false);
    const checkpoint = db.query("SELECT ?1 AS replay, ?2 AS retained, ?3 AS closeAllowed").get(Number(law.checkpointUnwind.replay), Number(law.checkpointUnwind.requestRetained), Number(law.checkpointUnwind.closeAllowed));
    expect(checkpoint).toEqual({ replay: 0, retained: 1, closeAllowed: 0 });
    const slotSource = db.query("SELECT ?1 AS genuine, ?2 AS foreignAccepted").get(Number(law.slotSource.genuineWitness), Number(law.slotSource.foreignAccepted));
    expect(slotSource).toEqual({ genuine: 1, foreignAccepted: 0 });
  } finally {
    db.close();
  }
});
