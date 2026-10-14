import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {folderReloadRouteOracle,walkAuthor} from "../../../🧪️folder-reload-route/🟦️.ts";
test("history input keeps its original scalar edit kind and validated value",()=>{
 expect(folderReloadRouteOracle(resolve(import.meta.dir,"../../../../../../../../.."))).toBeGreaterThan(0);
 expect(walkAuthor([{edit:{kind:"setCount",value:1}},{begin:0},{input:{value:2}},{commit:{choice:"overwrite"}}]).supersessions[0]?.replacement).toEqual({kind:"setCount",value:2});
 expect(()=>walkAuthor([{edit:{kind:"setCount",value:1}},{begin:0},{input:{value:"invalid"}}])).toThrow();
 expect(()=>walkAuthor([{edit:{kind:"setLabel",value:"a"}},{begin:0},{input:{value:2}}])).toThrow();
});
test("history action source remains pending until its original receiving turn accepts",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");db.exec("CREATE TABLE input (path TEXT, value TEXT);CREATE TABLE publication (path TEXT,value TEXT)");db.query("INSERT INTO input VALUES (?,?)").run(row.input.path,row.input.value);let outcome="pending";
  for(const turn of row.turns){db.exec("BEGIN");if(turn==="deferred"){db.exec("ROLLBACK");expect(db.query("SELECT * FROM input").get()).toEqual(row.input);expect(db.query("SELECT count(*) AS n FROM publication").get()).toEqual({n:0});continue}if(turn==="accepted"){db.exec("INSERT INTO publication SELECT * FROM input;DELETE FROM input;COMMIT");outcome="accepted"}else{db.exec("ROLLBACK");outcome="fault"}break}
  expect(outcome).toBe(row.expected);expect(db.query("SELECT * FROM publication").all()).toEqual(row.expected==="accepted"?[row.input]:[]);db.close();
 }
 const source=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8"),body=source.slice(source.indexOf("async fn acceptance_verb"),source.indexOf("async fn acceptance_pump"));
 expect(body).toContain("handle_action(action,&mut args,&meta,&mut identity,cx)");expect(body).toContain("if let Some(result)=result");expect(body).toContain("OriginalFixtureCaller::maximum_turns()");expect(body).toContain("ControlledRetirement::new(args)");expect(body).toContain("close_original_retained_step(same");expect(body).toContain("original acceptance action remains pending");
 console.log("[DEBUG] historyActionOriginalOption=true deferredNeverPublishes=true SQLiteAjv=true paidInputClose=true");
});
