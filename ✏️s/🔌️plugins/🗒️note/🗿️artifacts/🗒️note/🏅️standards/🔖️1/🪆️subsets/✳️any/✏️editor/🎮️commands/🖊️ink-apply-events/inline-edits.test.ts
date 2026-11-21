import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./inline-edits.json";

for (const entry of fixture.cases) {
  test(`replays and reverses ${entry.mutation} without altering other block fields`, () => {
    const db = new Database(":memory:");
    db.exec("CREATE TABLE block (id TEXT PRIMARY KEY, value TEXT)");
    const id = entry.before.id;
    const patch = entry.patch;
    const path = "row" in patch && "column" in patch ? `$.rows[${patch.row}][${patch.column}].content` : `$.${patch.field}`;
    db.query("INSERT INTO block VALUES (?, ?)").run(id, JSON.stringify(entry.before));
    const previous = db.query<{ previous: string }, [string, string]>("SELECT value -> ? AS previous FROM block WHERE id = ?").get(path, id)!.previous;
    const replace = db.query("UPDATE block SET value = json_set(value, ?, json(?)) WHERE id = ?");
    const read = db.query<{ value: string }, [string]>("SELECT value FROM block WHERE id = ?");
    replace.run(path, JSON.stringify(patch.value), id);
    expect(JSON.parse(read.get(id)!.value)).toEqual(entry.after);
    replace.run(path, previous, id);
    expect(JSON.parse(read.get(id)!.value)).toEqual(entry.before);
    db.close();
  });
}
