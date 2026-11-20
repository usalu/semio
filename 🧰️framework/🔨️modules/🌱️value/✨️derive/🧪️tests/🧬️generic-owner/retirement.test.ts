import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import schema from "../🧫️fixtures/🧬️generic-owner/🧬️.json";
import law from "../🧫️fixtures/🧬️generic-owner/🔣️.json";

test("wide record retirement releases the field allocations in order", () => {
  expect(new Ajv({ strict: true, strictTuples: false }).compile(schema)(law)).toBe(true);
  const { fields, firstRelease, lastRelease, totalReleased } = law.wideRetirement;
  const db = new Database(":memory:");
  try {
    const rows = db.query(`
      with recursive field(ordinal, bytes) as (
        select 1, ?1 union all select ordinal + 1, bytes + 1 from field where ordinal < ?2
      )
      select bytes from field order by ordinal
    `).all(firstRelease, fields) as { bytes: number }[];
    expect(rows.map(({ bytes }) => bytes)).toEqual(
      Array.from({ length: lastRelease - firstRelease + 1 }, (_, ordinal) => firstRelease + ordinal),
    );
    expect(rows.reduce((total, { bytes }) => total + bytes, 0)).toBe(totalReleased);
  } finally {
    db.close();
  }
});
