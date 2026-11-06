import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import law from "./fixture.json";
import schema from "./schema.json";

test("all refusal owners remain independent after one work adoption", () => {
  expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
  const db=new Database(":memory:");
  try {
    db.run("create table causes(slot text primary key, value text not null)");
    const insert=db.query("insert into causes values (?,?)");
    for(const key of ["primary","quote","allocation","receipt","context","work","secondWork"] as const) insert.run(key,law[key]);
    expect((db.query("select count(distinct value) n from causes").get() as {n:number}).n).toBe(7);
    expect(law.expectedAdoptions).toBe(1);
    expect(law.effectItems).toBe(3);
  } finally { db.close(); }
});
