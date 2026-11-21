import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

test("SQLite retains the exact media control variant and unsigned boundary values", async () => {
  const fixture = await Bun.file(new URL("./media-control.json", import.meta.url)).text();
  using database = new Database(":memory:");
  const rows = database.query<{
    tag: string; variant: string; name: string | null; mediaType: string | null;
    owner: string; seq: string; extent: number; instance: number;
  }, [string]>(`
    WITH SOURCE(input) AS (VALUES (?1)), EMITTED(wire) AS (
      SELECT json_object('tag','original-media-control','val',
        json_set(json_extract(input,'$.common'),'$.control',json(value)))
      FROM SOURCE, json_each(input,'$.cases')
    )
    SELECT wire ->> '$.tag' AS tag, wire ->> '$.val.control.tag' AS variant,
      wire ->> '$.val.control.val.name' AS name, wire ->> '$.val.control.val.mediaType' AS mediaType,
      wire -> '$.val.owner' AS owner, wire -> '$.val.seq' AS seq,
      wire ->> '$.val.extent' AS extent, wire ->> '$.val.loan.instanceId' AS instance
    FROM EMITTED
  `).all(fixture);
  const common = { tag: "original-media-control", owner: "18446744073709551615",
    seq: "9007199254740993", extent: 4294967295, instance: 4294967295 };
  expect(rows).toEqual([
    { ...common, variant: "begin", name: "fixture.png", mediaType: "image/png" },
    { ...common, variant: "complete", name: null, mediaType: null },
    { ...common, variant: "cancel", name: null, mediaType: null },
  ]);
});
