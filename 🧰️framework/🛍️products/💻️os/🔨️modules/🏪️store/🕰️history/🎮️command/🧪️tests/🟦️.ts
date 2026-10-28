import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";
test("native history selection restores original order and preserves complete refusal fields", () => {
 const db = new Database(":memory:");
 db.exec("CREATE TABLE applied(id TEXT PRIMARY KEY, value REAL); CREATE TABLE redo(id TEXT PRIMARY KEY, value REAL)");
 db.query("INSERT INTO applied VALUES (?,?)").run("original-publication-edit",fixture.published);
 for (const step of fixture.steps) {
  db.transaction(() => {
   if (step.mode === "undo") { db.exec("INSERT INTO redo SELECT * FROM applied; DELETE FROM applied"); }
   else { db.exec("INSERT INTO applied SELECT * FROM redo; DELETE FROM redo"); }
  })();
  const applied = db.query("SELECT count(*) AS n, max(value) AS value FROM applied").get() as {n:number,value:number|null};
  const redo = db.query("SELECT count(*) AS n FROM redo").get() as {n:number};
  expect(applied.n).toBe(step.applied); expect(redo.n).toBe(step.redo);
  expect(applied.value ?? fixture.genesis).toBe(step.expected);
 }
 db.exec("CREATE TABLE refusal(payload TEXT)");
 db.query("INSERT INTO refusal VALUES (?)").run(JSON.stringify(fixture.failure));
 const row = db.query("SELECT json_extract(payload,'$.kind') AS kind, json_extract(payload,'$.original.messages[0].message') AS message, json_extract(payload,'$.original.messages[0].target[0]') AS target FROM refusal").get() as {kind:string,message:string,target:string};
 expect(row.kind).toBe("rejected");expect(row.message).toBe("original source λ");expect(row.target).toBe("zoom");
 db.close();
});
