import{createHash}from"node:crypto";
import {gunzipSync} from "node:zlib";
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧵️shards/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));

test("neutral streamed ranking, continuation and eligibility agree with SQLite FTS5", () => {
 expect(new Ajv({ strict: false }).compile(schema.$defs.ShardedFixture)(fixture)).toBe(true);
 const database = new Database(":memory:");
 try {
  database.exec("CREATE VIRTUAL TABLE documents USING fts5(id UNINDEXED,text,kind UNINDEXED)");
  for (const [id, text] of Object.entries(fixture.small.documents)) database.query("INSERT INTO documents(id,text) VALUES(?1,?2)").run(id, text as string);
  const hits = database.query("SELECT id FROM documents WHERE documents MATCH ?1 ORDER BY id LIMIT ?2").all(fixture.small.query, fixture.small.size) as { id: string }[];
  const total = database.query("SELECT count(*) count FROM documents WHERE documents MATCH ?1").get(fixture.small.query) as { count: number };
  expect(hits.map(row => row.id)).toEqual(fixture.small.hits);
  expect(total.count).toBe(fixture.small.total);
  const next = database.query("SELECT id FROM documents WHERE documents MATCH ?1 AND id > ?2 ORDER BY id LIMIT ?3").all(fixture.small.query, hits.at(-1)!.id, fixture.small.size) as { id: string }[];
  expect(next.map(row => row.id)).toEqual(fixture.small.next);
  database.exec("DELETE FROM documents");
  for (const [id, document] of Object.entries(fixture.eligibility.documents) as [string, { text: string; kind: string }][]) database.query("INSERT INTO documents(id,text,kind) VALUES(?1,?2,?3)").run(id, document.text, document.kind);
  const eligible = database.query("SELECT id FROM documents WHERE documents MATCH ?1 AND kind = ?2 ORDER BY id LIMIT ?3").all(fixture.eligibility.query, fixture.eligibility.kind, fixture.eligibility.size) as { id: string }[];
  const eligibleTotal = database.query("SELECT count(*) count FROM documents WHERE documents MATCH ?1 AND kind = ?2").get(fixture.eligibility.query, fixture.eligibility.kind) as { count: number };
  expect(eligible.map(row => row.id)).toEqual(fixture.eligibility.hits);
  expect(eligibleTotal.count).toBe(fixture.eligibility.total);
  for (const scope of fixture.scopes) {
   database.exec("DELETE FROM documents");
   for (const [id, text] of Object.entries(scope.documents)) database.query("INSERT INTO documents(id,text) VALUES(?1,?2)").run(id, text as string);
   const scoped = database.query("SELECT id FROM documents WHERE documents MATCH ?1 ORDER BY id").all(fixture.small.query) as { id: string }[];
   expect(scoped.map(row => row.id)).toEqual(scope.hits);
  }
 } finally { database.close(); }
});

test("neutral 250001-document stream keeps both late matches against SQLite FTS5", () => {
 const database = new Database(":memory:");
 try {
  database.exec("CREATE VIRTUAL TABLE documents USING fts5(id UNINDEXED,text)");
  const insert = database.query("INSERT INTO documents(id,text) VALUES(?1,?2)");
  database.transaction(() => { for (let i = 0; i < fixture.large.documents; i++) insert.run("d-" + String(i).padStart(7, "0"), i === 0 || i === fixture.large.documents - 1 ? "needle" : "plain"); })();
  const count = database.query("SELECT count(*) count FROM documents").get() as { count: number };
  const total = database.query("SELECT count(*) count FROM documents WHERE documents MATCH ?1").get(fixture.large.query) as { count: number };
  const hits = database.query("SELECT id FROM documents WHERE documents MATCH ?1 ORDER BY id LIMIT ?2").all(fixture.large.query, fixture.large.size) as { id: string }[];
  const next = database.query("SELECT id FROM documents WHERE documents MATCH ?1 AND id > ?2 ORDER BY id LIMIT ?3").all(fixture.large.query, hits[0]!.id, fixture.large.size) as { id: string }[];
  expect(count.count).toBe(fixture.large.documents);
  expect(total.count).toBe(fixture.large.total);
  expect(hits.map(row => row.id)).toEqual(fixture.large.hits);
  expect(next.map(row => row.id)).toEqual([fixture.large.last]);
 } finally { database.close(); }
});

test("snapshot fixture agrees with Node gzip and relational cache-generation decisions",()=>{
 const snapshot=JSON.parse(readFileSync(new URL("../🧫️fixtures/🌲️snapshot/🔣️.json",import.meta.url),"utf8"));
 expect(new Ajv({strict:false}).compile(schema.$defs.SnapshotFixture)(snapshot)).toBe(true);
 expect(JSON.parse(gunzipSync(Buffer.from(snapshot.nodeGzip,"base64")).toString("utf8"))).toEqual(snapshot.value);
 const database=new Database(":memory:");
 try{database.exec("CREATE TABLE requests(sequence INTEGER PRIMARY KEY,fingerprint TEXT)");
  for(const[index,row]of snapshot.cases.entries()){
   database.query("INSERT INTO requests VALUES(?1,?2)").run(index,row.fingerprint);
   const actual=database.query("SELECT sum(changed) count FROM (SELECT fingerprint IS NOT lag(fingerprint) OVER (ORDER BY sequence) changed FROM requests)").get()as{count:number};
   expect(actual.count).toBe(row.builds);
  }
 }finally{database.close();}
});

test("authored source fingerprint fixture agrees with Node SHA256",()=>{
 const snapshot=JSON.parse(readFileSync(new URL("../🧫️fixtures/🌲️snapshot/🔣️.json",import.meta.url),"utf8"));
 for(const row of snapshot.sourceCases){const digest=createHash("sha256");for(const path of Object.keys(row.files).sort()){const body=Buffer.from(row.files[path]);digest.update(Buffer.byteLength(path)+":"+path+":file\n"+body.length+":");digest.update(body);}expect(digest.digest("hex")).toBe(row.sha256);}
});

test("fresh domain projection oracle selects the latest record while codebase remains cached",()=>{
 const snapshot=JSON.parse(readFileSync(new URL("../🧫️fixtures/🌲️snapshot/🔣️.json",import.meta.url),"utf8"));
 const database=new Database(":memory:");try{database.exec("CREATE TABLE records(sequence INTEGER PRIMARY KEY,title TEXT)");
 for(const[index,title]of snapshot.domainRefresh.titles.entries()){database.query("INSERT INTO records VALUES(?1,?2)").run(index,title);const latest=database.query("SELECT title FROM records ORDER BY sequence DESC LIMIT 1").get()as{title:string};expect(latest.title).toBe(title);}
 expect(snapshot.domainRefresh.cachedProjection).toBe("authored-codebase");expect(snapshot.domainRefresh.domainProjection).toBe("fresh-per-request");
 }finally{database.close();}
});

test("canonical indexed fields agree with SQLite JSON ordering across shuffled maps",()=>{
 const snapshot=JSON.parse(readFileSync(new URL("../🧫️fixtures/🌲️snapshot/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:false}).compile(schema.$defs.SnapshotFixture)(snapshot)).toBe(true);const database=new Database(":memory:");try{const query=database.query(`SELECT group_concat(term,' ') text FROM (SELECT COALESCE(CAST(member.value AS TEXT),'') term FROM json_each(?1) field JOIN json_each(CASE WHEN field.type='array' THEN field.value ELSE json_array(field.value) END) member ORDER BY field.key COLLATE BINARY,member.key)`);for(const data of [snapshot.projectionText.data,Object.fromEntries(Object.entries(snapshot.projectionText.data).reverse())])expect((query.get(JSON.stringify(data)) as {text:string}).text).toBe(snapshot.projectionText.expected);}finally{database.close();}
});
