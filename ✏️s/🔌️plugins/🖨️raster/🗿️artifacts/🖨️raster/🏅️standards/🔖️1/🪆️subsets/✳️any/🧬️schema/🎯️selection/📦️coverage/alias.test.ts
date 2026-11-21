import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "./fixture.json";

for (const scenario of fixture.alias) {
  test(`packed coverage alias retains ${scenario.count} ordered spans`, () => {
    using database = new Database(":memory:");
    database.run("CREATE TABLE root (owner TEXT PRIMARY KEY, backing INTEGER)");
    database.run("CREATE TABLE span (backing INTEGER, start INTEGER, PRIMARY KEY (backing, start))");
    database.run("INSERT INTO root VALUES ('original', CASE WHEN ? > 0 THEN 1 END)", [scenario.count]);
    database.run(`
      WITH RECURSIVE rank(start) AS (
        SELECT 0 WHERE ?1 > 0 UNION ALL SELECT start + 1 FROM rank WHERE start + 1 < ?1
      )
      INSERT INTO span SELECT 1, start FROM rank
    `, [scenario.count]);
    database.run("INSERT INTO root SELECT 'alias', backing FROM root WHERE owner = 'original'");

    expect(database.query("SELECT COUNT(*) AS count FROM root").get()).toEqual({ count: 2 });
    expect(database.query("SELECT COUNT(DISTINCT backing) AS count FROM root").get()).toEqual({
      count: Number(scenario.count > 0),
    });
    database.run("DELETE FROM root WHERE owner = 'original'");
    const spans = database.query("SELECT span.start FROM root JOIN span USING (backing) ORDER BY span.start").values();
    expect(spans).toEqual(Array.from({ length: scenario.count }, (_, start) => [start]));
    expect(database.query("SELECT COUNT(DISTINCT start / ?) AS pages FROM span").get(fixture.pageRows)).toEqual({
      pages: scenario.pages,
    });
    expect(database.query(`
      WITH operations AS (
        SELECT 'publish-header'
        UNION ALL SELECT 'increment-logical-count' FROM root WHERE backing IS NOT NULL
        UNION ALL SELECT 'duplicate-strong-reference' FROM root WHERE backing IS NOT NULL
        UNION ALL SELECT 'publish-reference' FROM root WHERE backing IS NOT NULL
      )
      SELECT COUNT(*) AS work FROM operations
    `).get()).toEqual({ work: scenario.work });
  });
}
