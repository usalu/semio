/** 🎟️ Independent schemas and SQLite compare an inference caller's full source against its distinct loan. */
import Ajv from "ajv";
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";

test("original MCP inference caller keeps finite work and a bounded physical policy", async () => {
 const source = await Bun.file(new URL("../🧫️fixtures/original-caller.json", import.meta.url)).json();
 const component = await Bun.file(new URL("../../../🧬️schema/🔣️.json", import.meta.url)).json();
 const ajv = new Ajv({strict:false,allErrors:true});
 ajv.addSchema(component,"original-mcp");
 const turn = ajv.compile({$ref:"original-mcp#/$defs/InferenceRetainedTurnInput"});
 const policy = ajv.compile({$ref:"original-mcp#/$defs/InferenceRetainedAuthority"});
 expect(turn(source.turn)).toBe(true);expect(policy(source.retainedPolicy)).toBe(true);
 for(const key of ["operation","generation","epoch","grant","work"]) {
  const missing = {...source.turn};delete missing[key];expect(turn(missing)).toBe(false);
 }
 for(const key of ["clockBinding","absoluteDeadlineUs","fuel"]) {
  const work = {...source.turn.work};delete work[key];expect(turn({...source.turn,work})).toBe(false);
 }
 expect(turn({...source.turn,work:{...source.turn.work,extra:0}})).toBe(false);
 const db = new Database(":memory:");
 try {
  db.exec("CREATE TABLE source(axis TEXT PRIMARY KEY,total INTEGER CHECK(total>=0),loan INTEGER CHECK(loan>=0))");
  for(const key of Object.keys(source.retainedPolicy))db.query("INSERT INTO source VALUES(?,?,?)").run(key,source.turn.grant[key],source.retainedPolicy[key]);
  expect(db.query("SELECT count(*) AS count FROM source WHERE loan<=total").get()).toEqual({count:5});
  expect(db.query("SELECT total,loan FROM source WHERE axis='maximumItems'").get()).toEqual({total:64,loan:1});
  expect(db.query("SELECT total,loan FROM source WHERE axis='maximumCopyBytes'").get()).toEqual({total:4096,loan:3});
  expect(db.query("SELECT (?1<=?2) AS permitted,?2-?1 AS unused").get(source.workUnits,source.turn.work.fuel)).toEqual({permitted:1,unused:53});
  expect(db.query("SELECT (?1<=?2) AS permitted").get(source.turn.work.fuel+1,source.turn.work.fuel)).toEqual({permitted:0});
 }finally{db.close()}
});
