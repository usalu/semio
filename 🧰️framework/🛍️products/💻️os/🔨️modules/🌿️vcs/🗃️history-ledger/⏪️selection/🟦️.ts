import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";
test("original selected history order agrees with SQL insertion and exclusion", () => {
  const db = new Database(":memory:");
  db.run("CREATE TABLE edits(position INTEGER PRIMARY KEY, id INTEGER)");
  for (const row of fixture.cases) {
    db.run("DELETE FROM edits");
    row.original.forEach((id, position) => db.run("INSERT INTO edits VALUES (?, ?)", [position * 2, id]));
    if (row.restoring) db.run("INSERT INTO edits VALUES (?, ?)", [row.position * 2 - 1, 1]);
    else db.run("DELETE FROM edits WHERE position=?", [row.position * 2]);
    const selected = db.query("SELECT id FROM edits ORDER BY position").all() as { id: number }[];
    expect(selected.map(({ id }) => id)).toEqual(row.selected);
  }
  db.close();
});
