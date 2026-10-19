import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { foldHistory } from "../../../🧪️tests/🗄️durable-collaborative-redo/🟦️.ts";

type Edit = { id: string; actor: string; logicalMs: number; mutationIds: string[] };
type Transition = Edit & { kind: "revert" | "reinstate" };
type Outcome = { outcome: "malformed"; detail: string } | { outcome: "accepted"; applied: string[]; redo: string[]; refused: string[] };
type Case = { id: string; edits: Edit[]; transitions: Transition[]; expected: Outcome };

/** ⚖️ Pins unique operation ownership and binary event order to independent SQLite constraints and collation. */
export function testHistoryInvariants(): void {
  const owner = join(import.meta.dir, "../..");
  const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🪪️history-invariants/🔣️.json"), "utf8")) as { schema: string; cases: Case[] };
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🪪️history-invariants/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE operations (id TEXT PRIMARY KEY, owner TEXT NOT NULL); CREATE TABLE events (id TEXT PRIMARY KEY, stamp INTEGER NOT NULL, kind TEXT NOT NULL, actor TEXT NOT NULL); CREATE TABLE active (id TEXT PRIMARY KEY); CREATE TABLE redo (id TEXT PRIMARY KEY, serial INTEGER NOT NULL); CREATE TABLE refused (id TEXT PRIMARY KEY, serial INTEGER NOT NULL)");
    for (const row of fixture.cases) {
      database.exec("DELETE FROM operations; DELETE FROM events; DELETE FROM active; DELETE FROM redo; DELETE FROM refused; BEGIN");
      let duplicate: string | undefined;
      for (const edit of row.edits) for (const id of edit.mutationIds) {
        try { database.query("INSERT INTO operations VALUES (?, ?)").run(id, edit.id); } catch { duplicate ??= id; }
      }
      if (row.expected.outcome === "malformed") {
        const detail = row.expected.detail;
        assert(duplicate, row.id);
        assert.equal(`history repeats operation ${duplicate}`, detail, row.id);
        assert.throws(() => foldHistory(row.edits, row.transitions), error => error instanceof Error && error.message === detail, row.id);
        database.exec("ROLLBACK");
        continue;
      }
      assert.equal(duplicate, undefined, row.id);
      database.exec("COMMIT");
      for (const event of [...row.edits.map(edit => ({ ...edit, kind: "edit" })), ...row.transitions]) database.query("INSERT INTO events VALUES (?, ?, ?, ?)").run(event.id, event.logicalMs, event.kind, event.actor);
      const ordered = database.query("SELECT id FROM events ORDER BY stamp, id COLLATE BINARY").all() as { id: string }[];
      const byteOrdered = [...row.edits, ...row.transitions].sort((a, b) => a.logicalMs - b.logicalMs || Buffer.compare(Buffer.from(a.id), Buffer.from(b.id)));
      assert.deepEqual(ordered.map(event => event.id), byteOrdered.map(event => event.id), row.id);
      const expected = { applied: row.expected.applied, redo: row.expected.redo, refused: row.expected.refused };
      let serial = 0;
      for (const event of ordered) {
        const edit = row.edits.find(edit => edit.id === event.id);
        if (edit) {
          database.query("INSERT INTO active VALUES (?)").run(edit.id);
          database.query("DELETE FROM redo WHERE id IN (SELECT id FROM events WHERE actor = ?)").run(edit.actor);
          continue;
        }
        const transition = row.transitions.find(transition => transition.id === event.id)!;
        const owners = [...new Set(transition.mutationIds.map(id => (database.query("SELECT owner FROM operations WHERE id = ?").get(id) as { owner: string }).owner))];
        const foreign = owners.some(owner => (database.query("SELECT actor FROM events WHERE id = ?").get(owner) as { actor: string }).actor !== transition.actor);
        if (foreign) { database.query("INSERT INTO refused VALUES (?, ?)").run(transition.id, serial++); continue; }
        for (const owner of owners) {
          if (transition.kind === "revert" && database.query("DELETE FROM active WHERE id = ?").run(owner).changes) database.query("INSERT INTO redo VALUES (?, ?)").run(owner, serial++);
          if (transition.kind === "reinstate" && database.query("DELETE FROM redo WHERE id = ?").run(owner).changes) database.query("INSERT INTO active VALUES (?)").run(owner);
        }
      }
      const ids = (sql: string) => (database.query(sql).all() as { id: string }[]).map(row => row.id);
      assert.deepEqual({ applied: ids("SELECT events.id FROM active JOIN events USING (id) ORDER BY stamp, events.id COLLATE BINARY"), redo: ids("SELECT id FROM redo ORDER BY serial"), refused: ids("SELECT id FROM refused ORDER BY serial") }, expected, row.id);
      assert.deepEqual(foldHistory(row.edits, row.transitions), expected, row.id);
      assert.deepEqual(foldHistory([...row.edits].reverse(), [...row.transitions].reverse()), expected, row.id);
    }
  } finally { database.close(); }
  console.log(`[DEBUG] ${fixture.cases.length} shared history vectors agree with Ajv and SQLite unique keys/binary collation`);
}
