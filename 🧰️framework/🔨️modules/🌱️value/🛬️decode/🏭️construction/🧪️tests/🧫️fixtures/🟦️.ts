import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

test("shared admission orders its allocation events before the receipt", () => {
  const fixture = JSON.parse(readFileSync(new URL("./effect-frame.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./effect-frame.schema.json", import.meta.url), "utf8"));
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const db = new Database(":memory:");
  for (const row of fixture.sharedAdmission) {
    const events = db.query("SELECT 'quote' AS event UNION ALL SELECT 'post' WHERE ?").all(row.born ? 1 : 0) as {event:string}[];
    expect(events.map(event => event.event)).toEqual(row.events);
    expect(Boolean(row.born)).toBe(row.receipt);
  }
  const recovery = fixture.workRecovery;
  const total = db.query("SELECT ? + ? AS reported").get(recovery.interrupted, recovery.settled) as {reported:number};
  expect(total.reported).toBe(recovery.child);
  const retirement = db.query("SELECT json_extract(?, '$.retirementWork') AS work").get(JSON.stringify(fixture)) as {work:string};
  expect(retirement.work).toBe(fixture.retirementWork);
  for (const row of fixture.slotCustody) {
    const actual = db.query("SELECT ? = ? AS accepted").get(row.owner,row.candidate) as {accepted:number};
    expect(Boolean(actual.accepted)).toBe(row.accepted);
  }
  db.close();
});
