/** 🔎️ Closed declared-child vectors retain independent logical and addressed identities. */
import assert from "node:assert/strict";
import {existsSync,readFileSync} from "node:fs";
import {Database} from "bun:sqlite";



type Row={slot:string;childId:string;artifactId:string;kind:string;standard:string;subset:string};
type Fixture={contractId:string;cases:{id:string;rows:Row[]}[];overrideRefusal:{revision:number;code:string;message:string}};
export function testDeclaredChildProjectionOracle():void {
 const path=new URL("../../../🧩️composition/🔎️projection/🧫️fixtures/🔣️.json",import.meta.url);
 assert(existsSync(path),"closed declared projection fixture must be published");
 const fixture=JSON.parse(readFileSync(path,"utf8")) as Fixture;
 
 
 assert.deepEqual(fixture.cases.map(item=>item.id),["empty","distinct-literal"]);
 const database=new Database(":memory:");
 try {
  database.exec("CREATE TABLE declared_child(ordinal INTEGER PRIMARY KEY, slot TEXT NOT NULL, logical TEXT NOT NULL, target TEXT NOT NULL, kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL)");
  for(const item of fixture.cases) {
   database.exec("DELETE FROM declared_child");
   const insert=database.prepare("INSERT INTO declared_child VALUES(?,?,?,?,?,?,?)");
   item.rows.forEach((row,index)=>insert.run(index,row.slot,row.childId,row.artifactId,row.kind,row.standard,row.subset));
   const actual=database.prepare("SELECT slot,logical AS childId,target AS artifactId,kind,standard,subset FROM declared_child ORDER BY ordinal").all();
   assert.deepEqual(actual,item.rows);
   assert.equal(new Set(item.rows.map(row=>row.childId)).size,item.rows.length);
   assert.equal(new Set(item.rows.map(row=>row.artifactId)).size,item.rows.length);
  }
  const literal=fixture.cases[1]!.rows[0]!;
  assert.notEqual(literal.childId,literal.artifactId);
  assert(literal.childId.includes("\0")&&literal.artifactId.includes("\0"));
  database.prepare("UPDATE declared_child SET target=? WHERE ordinal=0").run("edited addressed\0引用😀");
  const edited=database.prepare("SELECT logical,target FROM declared_child WHERE ordinal=0").get() as {logical:string;target:string};
  assert.equal(edited.logical,literal.childId);
  assert.equal(edited.target,"edited addressed\0引用😀");
  assert.equal(fixture.overrideRefusal.revision,-7);
  assert.equal(fixture.overrideRefusal.code,"test.parent-projection");
  const unknown=structuredClone(fixture) as unknown as Record<string,unknown>;
  unknown["fallback"]=true;
  
 } finally {database.close();}
}
