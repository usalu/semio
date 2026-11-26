import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import descriptor from "../../🧩️component/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };
import pin from "../../../../../✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json" with { type: "json" };

test("the published WFC roster names exactly the five ActionBus factory payloads", () => {
  using database = new Database(":memory:");
  database.exec("create table routes(owner text, kind text, service text primary key, payload text)");
  const insert = database.query("insert into routes values(?, ?, ?, ?)");
  for (const route of descriptor.contributions.inferenceServices) {
    insert.run(route.owner, route.artifactKind, route.inferenceSchema, route.payload.payloadSchemaId);
  }
  const routes = database.query("select kind, service, payload from routes where owner=? order by kind");
  expect(routes.all(fixture.owner)).toEqual(fixture.routes);
  expect(database.query("select count(*) as count from routes").get()).toEqual({ count: fixture.routes.length });
});

test("the pinned bitmap solve has one periodic input pattern and one output assignment", () => {
  const { solve, result } = pin;
  expect(solve.schema).toBe(fixture.retainedColdKind);
  expect([solve.input.width, solve.input.height, solve.output.width, solve.output.height]).toEqual([1, 1, 1, 1]);
  expect(solve.model.periodicInput).toBe(true);
  const source = Buffer.from(solve.input.pixels, "base64");
  using database = new Database(":memory:");
  const assign = database.query(`with recursive window(position, width, pattern) as (
    select 1, ?, hex(?) union all select position + 1, width, pattern || hex(?) from window where position < width * width
  ) select substr(pattern, 1, 2) as pixel, length(pattern) / 2 as cells from window where position = width * width`);
  const row = assign.get(solve.model.patternSize, source, source) as { pixel: string; cells: number };
  expect(row.cells).toBe(4);
  expect({ pixels: Buffer.from(row.pixel, "hex").toString("base64"), contradiction: false, entropy: [Math.log2(1)] }).toEqual(result);
});
