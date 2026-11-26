import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./lifecycle-retry.json";

test("only a single retained lifecycle event can replay a retryable framework deadline", () => {
  const db = new Database(":memory:");
  try {
    const authorized = db.query<{ retry: number }, [string, number, string, string]>(
      "SELECT (?1='framework' AND ?2=1 AND ?3='plugin.reactor-turn-deadline' AND ?4 IN ('empty','open','close','ack')) AS retry"
    );
    for (const row of fixture.cases) {
      expect(Boolean(authorized.get("framework",1,fixture.code,row.event)?.retry)).toBe(row.retry);
      expect(Boolean(authorized.get("framework",0,fixture.code,row.event)?.retry)).toBe(false);
      expect(Boolean(authorized.get("plugin",1,fixture.code,row.event)?.retry)).toBe(false);
    }
    const bytes = Buffer.from(fixture.packHex,"hex");
    expect(bytes.length).toBe(395);
    expect(bytes.includes(Buffer.from(fixture.message))).toBe(true);
  } finally {
    db.close();
  }
});
