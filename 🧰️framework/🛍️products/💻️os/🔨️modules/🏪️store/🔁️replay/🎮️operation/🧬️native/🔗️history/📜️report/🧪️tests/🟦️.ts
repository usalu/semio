import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv";
import {readFileSync} from "node:fs";
const law=JSON.parse(readFileSync(new URL("🔣️.json",import.meta.url),"utf8"));
test("original Report supersession and withdrawal agree with independent event SQL", () => {
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv2020({ strict: true }).compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  try {
    db.exec(`
      CREATE TABLE history (ordinal INTEGER PRIMARY KEY, edit_id TEXT, operation TEXT, zoom REAL);
      CREATE TABLE replacements (operation TEXT PRIMARY KEY, zoom REAL, withdrawn INTEGER)
    `);
    db.query("INSERT INTO history VALUES (0, 'original-report-edit', 'original-report-op', ?)").run(law.authoredZoom);
    db.query("INSERT INTO replacements VALUES ('original-report-op', ?, 0)").run(law.replacementZoom);
    const select = db.query(`
      SELECT CASE WHEN replacement.withdrawn = 1 THEN ? ELSE COALESCE(replacement.zoom, history.zoom) END AS zoom
      FROM history LEFT JOIN replacements replacement ON replacement.operation = history.operation
      ORDER BY history.ordinal
    `);
    expect(select.get(law.genesisZoom)).toEqual({ zoom: law.replacementZoom });
    db.exec("UPDATE replacements SET withdrawn = 1");
    expect(select.get(law.genesisZoom)).toEqual({ zoom: law.genesisZoom });
    const work = db.query(`
      SELECT COUNT(DISTINCT edit_id) AS plannedEdits, COUNT(*) AS foldedOperations,
        COUNT(DISTINCT edit_id) AS closedEdits, (SELECT COUNT(*) FROM replacements) AS plannedDrafts
      FROM history
    `).get() as Record<string, number>;
    expect(Object.values(work).reduce((total, count) => total + count, 0)).toBe(law.completedWork);
  } finally {
    db.close();
  }
});
