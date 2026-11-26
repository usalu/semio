import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import law from "./actor-clock.json" with { type: "json" };

test("the strict lifecycle verdict counts time only inside the actor's execution", () => {
  const db = new Database(":memory:");
  try {
    const verdict = db.query<{ active: number; expired: number }, [number, number, number, number]>(`
      WITH intervals(running, elapsed) AS (VALUES (1,?), (0,?), (1,?))
      SELECT sum(running*elapsed) active, sum(running*elapsed)>=? expired FROM intervals
    `);
    for (const probe of law.cases) {
      const [first, second] = probe.activeUs;
      expect(verdict.get(first!,probe.parkedUs,second!,law.strictCeilingUs),probe.name)
        .toEqual({active:probe.expectedActiveUs,expired:Number(probe.refused)});
    }
  } finally { db.close(); }
});
