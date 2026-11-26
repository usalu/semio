import {expect, test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import law from "./🧫️fixtures/⏱️export-cancellation-law.json" with {type: "json"};
import schema from "./🧫️fixtures/⏱️export-cancellation.schema.json" with {type: "json"};

test("a cancelled export never releases a result or changes its persisted pair", () => {
 const validate = new Ajv({strict:true}).compile(schema);
 expect(validate(law), JSON.stringify(validate.errors)).toBe(true);
 const database = new Database(":memory:");
 try {
  database.exec(`
   create table document(pack blob not null, spr blob not null);
   insert into document values (X'0102', X'0304');
   create table phases(position integer primary key, name text not null);
   insert into phases values (0,'opening-guest'), (1,'loading-document'), (2,'reading-document');
   create table jobs(id text primary key, stopped integer not null, settled integer not null default 0);
   create table exports(pack blob not null, spr blob not null, port text not null);
  `);
  const publish = database.query(`
   insert into exports select pack, spr, ?2 from document
   where exists(select 1 from jobs where id=?1 and settled=1 and stopped=0)
  `);
  const admit = database.query("insert into jobs(id,stopped) select ?1,exists(select 1 from phases where name=?2 and ?3 >= 0)");
  const observe = database.query(`
   select case when stopped=1 then 'CANCELLED' when settled=1 then 'EXPORTED' end request,
   case when settled=0 then 'RUNNING' when stopped=1 then 'CANCELLED' else 'SUCCEEDED' end job
   from jobs where id=?
  `);
  for (const row of law.cases) {
   admit.run(row.id, row.cancelAt, row.after);
   expect(observe.get(row.id), row.id).toEqual({request: row.cancelAt === null ? null : "CANCELLED", job:"RUNNING"});
   database.query("update jobs set settled=1 where id=?").run(row.id);
   const exported = publish.run(row.id, law.port).changes === 1;
   expect(exported ? "EXPORTED" : "CANCELLED", row.id).toBe(row.expected);
   expect(observe.get(row.id), row.id).toEqual({request:row.expected, job:exported ? "SUCCEEDED" : "CANCELLED"});
   expect(database.query("select hex(pack) pack, hex(spr) spr from document").get(), row.id).toEqual({pack: "0102", spr: "0304"});
   expect(database.query("select hex(pack) pack, hex(spr) spr, port from exports").all(), row.id)
    .toEqual(exported ? [{pack: "0102", spr: "0304", port: law.port}] : []);
  }
 } finally {
  database.close();
 }
});
