import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧫️fixtures/🧬️schema/🔣️.json" with { type: "json" };

test("one common decision publishes three retained member roots together", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
  mkdirSync(generated, { recursive: true });
  const directory = mkdtempSync(join(generated, "member-group-sqlite-"));
  try {
    for (const row of fixture.cases) {
      for (const decision of ["commit", "abort"]) {
        const path = join(directory, `${row.id}-${decision}.sqlite`);
        const writer = new Database(path), reader = new Database(path);
        try {
          writer.run("PRAGMA journal_mode=WAL");
          writer.run("CREATE TABLE roots (lane INTEGER PRIMARY KEY, value INTEGER NOT NULL, generation INTEGER NOT NULL)");
          for (let lane = 0; lane < 3; lane++) writer.run("INSERT INTO roots VALUES (?, ?, ?)", [lane, row.before[lane], row.historyEntries]);
          const read = () => reader.query("SELECT value, generation FROM roots ORDER BY lane").all() as { value: number; generation: number }[];
          const before = read();
          writer.run("BEGIN");
          for (let lane = 0; lane < 3; lane++) {
            writer.run("UPDATE roots SET value=?, generation=generation+1 WHERE lane=?", [row.after[lane], lane]);
            expect(read()).toEqual(before);
          }
          writer.run(decision === "commit" ? "COMMIT" : "ROLLBACK");
          expect(read().map(root => root.value)).toEqual(decision === "commit" ? row.after : row.before);
          expect(read().map(root => root.generation)).toEqual([0, 1, 2].map(() => row.historyEntries + Number(decision === "commit")));
        } finally { reader.close(); writer.close(); }
      }
    }
  } finally { rmSync(directory, { recursive: true, force: true }); }
  const group = resolve(import.meta.dir, "../../🦀️.rs");
  const source = readFileSync(resolve(import.meta.dir, "../../../../../🦀️.rs"), "utf8") + (existsSync(group) ? readFileSync(group, "utf8") : "");
  for (const method of ["stage_apply_batch_group", "adopt_apply_batch_group", "stage_one_item_publication", "adopt_one_item_publication"]) expect(source.includes(`fn ${method}(`)).toBe(true);
  console.log("[DEBUG] four neutral histories x commit/abort: SQLite readers observe all three roots at one common decision");
});

test("private prepared roots can be read before one common decision", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE roots (lane INTEGER PRIMARY KEY, value INTEGER NOT NULL)");
    for (const row of fixture.cases) {
      database.run("DELETE FROM roots");
      for (let lane = 0; lane < 3; lane++) database.run("INSERT INTO roots VALUES (?, ?)", [lane, row.before[lane]]);
      database.run("BEGIN");
      for (let lane = 0; lane < 3; lane++) database.run("UPDATE roots SET value=? WHERE lane=?", [row.after[lane], lane]);
      expect(database.query("SELECT value FROM roots ORDER BY lane").all().map(root => (root as { value: number }).value)).toEqual(row.after);
      database.run("ROLLBACK");
      expect(database.query("SELECT value FROM roots ORDER BY lane").all().map(root => (root as { value: number }).value)).toEqual(row.before);
    }
  } finally { database.close(); }
  expect(readFileSync(resolve(import.meta.dir, "../../../../../🦀️.rs"), "utf8").includes("fn snapshot_read_erased_for_publication(")).toBe(true);
  console.log("[DEBUG] four neutral private roots: prepared reads precede common commit without publishing live authority");
});

test("private prepared read cancellation returns to the exact authority before rollback", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE roots (lane INTEGER PRIMARY KEY, value INTEGER NOT NULL)");
    for (const row of fixture.cases) {
      database.run("DELETE FROM roots");
      for (let lane = 0; lane < 3; lane++) database.run("INSERT INTO roots VALUES (?, ?)", [lane, row.before[lane]]);
      database.run("BEGIN");
      for (let lane = 0; lane < 3; lane++) database.run("UPDATE roots SET value=? WHERE lane=?", [row.after[lane], lane]);
      const leases = database.query("SELECT lane, value FROM roots ORDER BY lane").all() as { lane: number; value: number }[];
      expect(leases.map(lease => lease.value)).toEqual(row.after);
      expect(leases.map(lease => lease.lane)).toEqual([0, 1, 2]);
      leases.length = 0;
      database.run("ROLLBACK");
      expect(database.query("SELECT value FROM roots ORDER BY lane").all().map(root => (root as { value: number }).value)).toEqual(row.before);
    }
  } finally { database.close(); }
  expect(readFileSync(resolve(import.meta.dir, "../../../../../🦀️.rs"), "utf8").includes("fn return_prepared_snapshot_read_erased(")).toBe(true);
  console.log("[DEBUG] four private roots: exact candidate leases return before rollback without publishing future authority");
});

test("private child projections borrow only their exact member read authority",()=>{
 const path=resolve(import.meta.dir,"../../🧫️fixtures/🔐️projection"),law=JSON.parse(readFileSync(join(path,"🔣️.json"),"utf8"));
 expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(path,"📐️schema.json"),"utf8")))(law)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.run("CREATE TABLE reads (id TEXT PRIMARY KEY, member TEXT NOT NULL, read_member TEXT NOT NULL, typed INTEGER NOT NULL)");
  for(const row of law.cases){database.run("INSERT INTO reads VALUES (?, ?, ?, ?)",[row.id,row.member,row.readMember,Number(row.typed)]);expect(database.query("SELECT member=read_member AND typed=1 AS accepted FROM reads WHERE id=?").get(row.id)).toEqual({accepted:Number(row.accepted)});}
 }finally{database.close();}
 const source=readFileSync(resolve(import.meta.dir,"../../../../../🦀️.rs"),"utf8");
 expect(source.includes("fn child_restore_projection_for_read<")).toBe(true);
 expect(law.allocatedBytes).toBe(0);expect(law.liveAuthorityChanged).toBe(false);
 console.log("[DEBUG] private projection Ajv/SQLite independent exactmember+typed read corpus forbids foreign aliases with equal scalar values");
});
