import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./fixture.json";

test("typed retirement moves custody only with the full quoted copy span", () => {
  const database = new Database(":memory:");
  try {
    const selected = database.query(
      "SELECT (? = 0) AS movesSource, CASE WHEN ? = 0 THEN 1 ELSE 0 END AS work",
    );
    for (const { withheldCopyBytes, movesSource, work } of fixture.cases) {
      expect(selected.get(withheldCopyBytes, withheldCopyBytes)).toEqual({
        movesSource: Number(movesSource),
        work,
      });
    }
  } finally {
    database.close();
  }
});
