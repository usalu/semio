import { Database } from "bun:sqlite"
import { expect, test } from "bun:test"
import Ajv from "ajv"
import law from "./flat.json"
import schema from "./flat.schema.json"

test("flat schema resolves original keys by name or ordinal and preserves ambiguity", () => {
  expect(new Ajv().compile(schema)(law)).toBe(true)
  const db = new Database(":memory:")
  try {
    db.run("create table field(rank integer primary key, name text, ordinal integer)")
    const insert = db.query("insert into field values (?, ?, ?)")
    law.fields.forEach((field, rank) => insert.run(rank, field.name, field.ordinal))
    const byName = db.query("select rank from field where name = ? order by rank limit 2")
    const byOrdinal = db.query("select rank from field where ordinal = ? order by rank limit 2")
    for (const key of law.cases) {
      const found = key.name !== undefined ? byName.all(key.name) : byOrdinal.all(key.ordinal)
      const kind = found.length === 0 ? "missing" : found.length === 1 ? "field" : "ambiguous"
      const rank = kind === "field" ? (found[0] as { rank: number }).rank : null
      expect({ kind, field: rank }).toEqual({ kind: key.kind, field: key.field })
    }
  } finally {
    db.close()
  }
})
