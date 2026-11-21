import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./🧫️fixtures/interning.json" with { type: "json" };

test("symbol ordinals follow first exact UTF-8 occurrence", () => {
  const database = new Database(":memory:");
  try {
    const results = database.query<{ ordinal: number }, [string]>(`
      with inputs as (select key, value from json_each(?1)),
           distinct_values as (select value, min(key) as first_seen from inputs group by value collate binary),
           ordinals as (select value, row_number() over (order by first_seen) - 1 as ordinal from distinct_values)
      select ordinal from inputs join ordinals on inputs.value = ordinals.value collate binary order by inputs.key
    `).all(JSON.stringify(fixture.inputs));
    expect(results.map(row => row.ordinal)).toEqual(fixture.ordinals);
  } finally {
    database.close();
  }
});
