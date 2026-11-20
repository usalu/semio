import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { compactAcknowledgedTurns } from "../🟦️.ts";
import type { WireTurnResult } from "../../../🪝️wire-codec/🟦️.ts";
import fixture from "./🔣️.json";

test("settlement retains outputs in order and the latest status across quiet command turns", () => {
  const oracle = new Database(":memory:");
  try {
    oracle.exec("CREATE TABLE turns(id INTEGER PRIMARY KEY, output INTEGER)");
    const insert = oracle.prepare("INSERT INTO turns VALUES (?, ?)");
    const turns: WireTurnResult[] = [];
    let folded = 0;
    oracle.transaction(() => {
      for (let id = 0; id < fixture.turns; id++) {
        const patch = fixture.patches.includes(id), effect = fixture.effects.includes(id);
        const presence = fixture.presence.includes(id), wake = fixture.wake.includes(id);
        insert.run(id, Number(patch || effect || presence || wake));
        turns.push({ status: id, uiPatches: patch ? [{}] : [],
          effects: effect ? [{ tag: "effect" }] : [], presence: presence ? [{update:new Uint8Array()}] : [],
          nextWake: wake ? 0 : null, original: {}, lifecycleReceipt: new Uint8Array() });
        if ((id + 1) % fixture.batchSize === 0 || id + 1 === fixture.turns) {
          compactAcknowledgedTurns(turns, folded);
          folded = turns.length;
          expect(folded).toBeLessThanOrEqual(fixture.expected.length);
        }
      }
    })();
    const rows = oracle.query("SELECT id FROM turns WHERE output = 1 OR id = (SELECT max(id) FROM turns) ORDER BY id").values();
    expect(rows.flat()).toEqual(fixture.expected);
    expect(turns.map(turn => turn.status)).toEqual(fixture.expected);
  } finally { oracle.close(); }
});
