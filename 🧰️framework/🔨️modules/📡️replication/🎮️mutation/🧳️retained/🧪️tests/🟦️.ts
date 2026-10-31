/** 🧪️ Portable retained outcomes agree with an independent transactional SQL projection. */
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv";
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fieldCorpus from "../🪡️field/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import fieldSchema from "../🪡️field/🧬️schema/🔣️.json" with { type: "json" };
import vectorCorpus from "../../../../🌱️value/♻️retirement/🏭️factory/📋️vector/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import vectorSchema from "../../../../🌱️value/♻️retirement/🏭️factory/📋️vector/🧬️schema/🔣️.json" with { type: "json" };
import keyCorpus from "../🔎️key/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import keySchema from "../🔎️key/🧬️schema/🔣️.json" with { type: "json" };
import patchCorpus from "../🩹️patch/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import patchSchema from "../🩹️patch/🧬️schema/🔣️.json" with { type: "json" };
import diagnosticCorpus from "../⚠️diagnostic/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import diagnosticSchema from "../⚠️diagnostic/🧬️schema/🔣️.json" with { type: "json" };
import rowCorpus from "../🧱️row/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import rowSchema from "../🧱️row/🧬️schema/🔣️.json" with { type: "json" };
import diffCorpus from "../🔺️diff/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import diffSchema from "../🔺️diff/🧬️schema/🔣️.json" with { type: "json" };

import persistentCorpus from "../../../../🌱️value/📋️list/🔗️persistent/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import persistentSchema from "../../../../🌱️value/📋️list/🔗️persistent/🧬️schema/🔣️.json" with { type: "json" };
import concatCorpus from "../../📨️messages/🔗️concat/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import concatSchema from "../../📨️messages/🔗️concat/🧬️schema/🔣️.json" with { type: "json" };
import spliceCorpus from "../../../../🌱️value/📋️list/🔗️persistent/🎮️splice/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import spliceSchema from "../../../../🌱️value/📋️list/🔗️persistent/🎮️splice/🧬️schema/🔣️.json" with { type: "json" };
import readCorpus from "../../../../🌱️value/📋️list/🔗️persistent/🧳️read/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import readSchema from "../../../../🌱️value/📋️list/🔗️persistent/🧳️read/🧬️schema/🔣️.json" with { type: "json" };
import constructionCorpus from "../../../../🌱️value/🛬️decode/🏭️construction/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import constructionSchema from "../../../../🌱️value/🛬️decode/🏭️construction/🧬️schema/🔣️.json" with { type: "json" };
import referenceCorpus from "../../❄️reference/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import referenceSchema from "../../❄️reference/🧬️schema/🔣️.json" with { type: "json" };
import receivingCorpus from "../../../../🌱️value/🛬️decode/🧳️owned/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import receivingSchema from "../../../../🌱️value/🛬️decode/🧳️owned/🧬️schema/🔣️.json" with { type: "json" };
import recordCorpus from "../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import recordSchema from "../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧬️schema/🔣️.json" with { type: "json" };
/** 🧾️ Verifies semantic refusal, exact inverse and independent grant axes against SQLite. */
export function testRetainedMutationOracle(): void {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  assert.equal(validate(corpus), true, JSON.stringify(validate.errors));
  assert.equal(new Ajv2020({ strict: true }).compile(fieldSchema)(fieldCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(vectorSchema)(vectorCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(keySchema)(keyCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(patchSchema)(patchCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(diagnosticSchema)(diagnosticCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(rowSchema)(rowCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(diffSchema)(diffCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(persistentSchema)(persistentCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(concatSchema)(concatCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(spliceSchema)(spliceCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(readSchema)(readCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(constructionSchema)(constructionCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(referenceSchema)(referenceCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(receivingSchema)(receivingCorpus), true);
  assert.equal(new Ajv2020({ strict: true }).compile(recordSchema)(recordCorpus), true);
  const db = new Database(":memory:");
  db.exec("CREATE TABLE received_field_spec(name TEXT,ordinal INTEGER); CREATE TABLE received_field_input(name TEXT,ordinal INTEGER,value)");
  recordCorpus.fields.forEach(field=>db.query("INSERT INTO received_field_spec VALUES(?,?)").run(field.name,field.ordinal));
  for(const lane of ["canonical","text"] as const){db.exec("DELETE FROM received_field_input");recordCorpus[lane].forEach(entry=>db.query("INSERT INTO received_field_input VALUES(?,?,?)").run(lane==="canonical"?entry[0]!:null,lane==="text"?entry[0]!:null,entry[1]!));const projection=Object.fromEntries(db.query("SELECT s.name,i.value FROM received_field_spec s JOIN received_field_input i ON s.name=i.name OR s.ordinal=i.ordinal ORDER BY s.ordinal").all().map(row=>[(row as {name:string}).name,(row as {value:string|number}).value]));assert.deepEqual(projection,recordCorpus.expected);}
  db.exec("CREATE TABLE native_received(id TEXT,name TEXT,value INTEGER)");
  db.query("INSERT INTO native_received VALUES(?,?,?)").run(receivingCorpus.before.id,receivingCorpus.before.name,receivingCorpus.before.value);
  assert.deepEqual(db.query("SELECT id,name,value FROM native_received").get(),receivingCorpus.expected);
  db.exec("CREATE TABLE refused_receive(key TEXT,value TEXT)");
  receivingCorpus.refusedEntries.forEach(entry=>db.query("INSERT INTO refused_receive VALUES(?,?)").run(entry[0]!,String(entry[1])));
  assert.equal((db.query("SELECT EXISTS(SELECT key FROM refused_receive GROUP BY key HAVING count(*)>1) AS refused").get() as {refused:number}).refused,1);
  db.exec("CREATE TABLE contract_order(lane TEXT,position INTEGER,phase TEXT)");
  for(const [lane,phases] of [["native",referenceCorpus.nativeOrder],["reference",referenceCorpus.coldWireOrder]] as const){phases.forEach((phase,position)=>db.query("INSERT INTO contract_order VALUES(?,?,?)").run(lane,position,phase));assert.deepEqual(db.query("SELECT phase FROM contract_order WHERE lane=? ORDER BY position").all(lane).map(row=>(row as {phase:string}).phase),phases);}
  assert.equal((db.query("SELECT count(*) AS count FROM contract_order WHERE lane='native' AND phase IN ('diff','apply')").get() as {count:number}).count,0);
  for(const vector of constructionCorpus){const row=db.query("SELECT CASE WHEN ? THEN 'output' ELSE 'input' END AS keeper, (? AND ?) AS success").get(Number(vector.admitted),Number(vector.admitted),Number(vector.receiptAccepted)) as {keeper:string,success:number};assert.equal(row.keeper,vector.keeper,vector.id);assert.equal(Boolean(row.success),vector.success,vector.id);}
  const affinity=vectorCorpus.issuerAlias;
  const issuerRow=db.query("SELECT (? AND ?) AS admitted, CASE WHEN ? AND ? THEN 1 ELSE 0 END AS providerCalls, ? AS pendingId").get(Number(affinity.dataEqual),Number(affinity.methodTableEqual),Number(affinity.dataEqual),Number(affinity.methodTableEqual),affinity.pendingId) as {admitted:number,providerCalls:number,pendingId:number};
  assert.equal(Boolean(issuerRow.admitted),affinity.admitted);assert.equal(issuerRow.providerCalls,affinity.providerCalls);assert.equal(issuerRow.pendingId,affinity.pendingId);
  try {
    db.exec("CREATE TABLE original_read(rank INTEGER PRIMARY KEY)");
    for(const vector of readCorpus.cases){db.exec("DELETE FROM original_read");for(let rank=0;rank<vector.length;rank++)db.query("INSERT INTO original_read VALUES(?)").run(rank);assert.deepEqual(db.query("SELECT rank FROM original_read ORDER BY rank").all().map(row=>(row as {rank:number}).rank),vector.expected,vector.id);}
    db.exec("CREATE TABLE original_batches(batch INTEGER NOT NULL,position INTEGER NOT NULL,body TEXT NOT NULL)");
    concatCorpus.batches.forEach((batch,i)=>batch.forEach((body,j)=>db.query("INSERT INTO original_batches VALUES(?,?,?)").run(i,j,body)));
    assert.deepEqual(db.query("SELECT body FROM original_batches ORDER BY batch,position").all().map(row=>(row as {body:string}).body),concatCorpus.expected);
        db.exec("CREATE TABLE ranked_rows(position INTEGER PRIMARY KEY,value INTEGER NOT NULL)");
    for(const vector of spliceCorpus){
      db.exec("DELETE FROM ranked_rows");vector.before.forEach((value,i)=>db.query("INSERT INTO ranked_rows VALUES(?,?)").run(i,value));
      db.exec("BEGIN");
      if(vector.kind==="insert"){
        const suffix=db.query("SELECT position,value FROM ranked_rows WHERE position>=? ORDER BY position DESC").all(vector.index) as {position:number,value:number}[];
        suffix.forEach(row=>db.query("UPDATE ranked_rows SET position=? WHERE position=?").run(row.position+1,row.position));
        db.query("INSERT INTO ranked_rows VALUES(?,?)").run(vector.index,vector.authored!);
      }else{
        db.query("DELETE FROM ranked_rows WHERE position=?").run(vector.index);
        const suffix=db.query("SELECT position,value FROM ranked_rows WHERE position>? ORDER BY position").all(vector.index) as {position:number,value:number}[];
        suffix.forEach(row=>db.query("UPDATE ranked_rows SET position=? WHERE position=?").run(row.position-1,row.position));
      }
      db.exec("COMMIT");assert.deepEqual(db.query("SELECT value FROM ranked_rows ORDER BY position").all().map(row=>(row as {value:number}).value),vector.expected,vector.id);
    }
    db.exec("CREATE TABLE projection(value INTEGER NOT NULL)");
    for (const vector of corpus.cases) {
      db.exec("DELETE FROM projection");
      db.query("INSERT INTO projection VALUES (?)").run(vector.base);
      const inverse: number[] = [];
      let applies = 0;
      db.transaction(() => {
        if (vector.refused) return;
        inverse.push((db.query("SELECT value FROM projection").get() as { value: number }).value);
        db.query("UPDATE projection SET value=?").run(vector.set);
        applies++;
      })();
      assert.equal((db.query("SELECT value FROM projection").get() as { value: number }).value, vector.expected);
      assert.equal(inverse.length, vector.inverseCalls);
      assert.equal(applies, vector.applyCalls);
      if (inverse.length) {
        db.query("UPDATE projection SET value=?").run(inverse[0]!);
        assert.equal((db.query("SELECT value FROM projection").get() as { value: number }).value, vector.base);
      }
    }
    for (const vector of corpus.grants) {
      const g = vector.grant, d = vector.demand;
      const row = db.query("SELECT (? > 0 AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ?) AS admitted").get(g.maximumItems, g.maximumCopyBytes, d.copyBytes, g.maximumCapacityBytes, d.capacityBytes, g.maximumReleaseBytes, d.releaseBytes, g.maximumDepth, d.depth) as { admitted: number };
      assert.equal(Boolean(row.admitted), vector.admitted, vector.id);
    }
    db.exec("CREATE TABLE persistent_page_rows(version INTEGER NOT NULL,position INTEGER NOT NULL,value INTEGER NOT NULL,PRIMARY KEY(version,position))");
    for (const vector of persistentCorpus.cases) {
      db.exec("DELETE FROM persistent_page_rows");
      db.query("WITH RECURSIVE rows(position) AS (VALUES(0) UNION ALL SELECT position+1 FROM rows WHERE position+1 < ?) INSERT INTO persistent_page_rows SELECT 0,position,position*2 FROM rows").run(vector.rowCount);
      db.transaction(() => {
        db.exec("INSERT INTO persistent_page_rows SELECT 1,position,value FROM persistent_page_rows WHERE version=0");
        db.query("UPDATE persistent_page_rows SET value=? WHERE version=1 AND position=?").run(vector.authored,vector.index);
      })();
      const read=(version:number)=>(db.query("SELECT value FROM persistent_page_rows WHERE version=? AND position=?").get(version,vector.index) as {value:number}).value;
      assert.equal(read(0),vector.unchangedReader,vector.id);
      assert.equal(read(1),vector.after,vector.id);
      const levels=db.query("WITH RECURSIVE path(depth,capacity) AS (VALUES(1,?) UNION ALL SELECT depth+1,capacity*? FROM path WHERE capacity < ?) SELECT MAX(depth) AS levels FROM path").get(persistentCorpus.fanout,persistentCorpus.fanout,vector.rowCount) as {levels:number};
      assert.equal(levels.levels,vector.changedPages,vector.id);
      const changed=db.query("SELECT COUNT(*) AS changed FROM persistent_page_rows old JOIN persistent_page_rows candidate USING(position) WHERE old.version=0 AND candidate.version=1 AND old.value<>candidate.value").get() as {changed:number};
      assert.equal(changed.changed,1,vector.id);
    }
    db.exec("CREATE TABLE owned_field(value TEXT NOT NULL)");
    for (const vector of fieldCorpus) {
      db.exec("DELETE FROM owned_field");
      db.query("INSERT INTO owned_field VALUES (?)").run(vector.before);
      db.transaction(() => db.query("UPDATE owned_field SET value=?").run(vector.authored))();
      assert.equal((db.query("SELECT value FROM owned_field").get() as { value: string }).value,vector.expected,vector.id);
    }
    db.exec("CREATE TABLE inverse_row(value INTEGER NOT NULL)");
    db.query("INSERT INTO inverse_row VALUES (?)").run(corpus.issuer.inverseSet);
    let attempts=0,issued=0;
    assert.throws(()=>db.transaction(()=>{attempts++;db.exec("DELETE FROM inverse_row");throw Error("original issuer refused");})());
    assert.equal((db.query("SELECT value FROM inverse_row").get() as {value:number}).value,corpus.issuer.inverseSet);
    db.transaction(()=>{attempts++;db.exec("DELETE FROM inverse_row");issued++;})();
    assert.equal(attempts,corpus.issuer.attempts);assert.equal(issued,corpus.issuer.issued);
    db.exec("CREATE TABLE original_rows(position INTEGER PRIMARY KEY,id INTEGER NOT NULL,text TEXT NOT NULL)");
    vectorCorpus.rows.forEach((row,position)=>db.query("INSERT INTO original_rows VALUES(?,?,?)").run(position,row.id,row.text));
    let vectorAttempts=0;
    const settled:number[]=[];
    if(vectorCorpus.refuseFirst){
      assert.throws(()=>db.transaction(()=>{vectorAttempts++;db.exec("DELETE FROM original_rows WHERE position=0");throw Error("exact vector issuer refused");})());
      assert.deepEqual(db.query("SELECT id,text FROM original_rows ORDER BY position").all(),vectorCorpus.rows);
    }
    while(true){
      const row=db.query("SELECT position,id FROM original_rows ORDER BY position LIMIT 1").get() as {position:number,id:number}|null;
      if(!row)break;
      db.transaction(()=>{vectorAttempts++;db.query("DELETE FROM original_rows WHERE position=?").run(row.position);settled.push(row.id);})();
    }
    assert.deepEqual(settled,vectorCorpus.expectedOrder);assert.equal(vectorAttempts,vectorCorpus.expectedAttempts);
    db.exec("CREATE TABLE keyed_rows(position INTEGER PRIMARY KEY,id TEXT NOT NULL)");
    for(const vector of keyCorpus.cases){
      db.exec("DELETE FROM keyed_rows");
      vector.keys.forEach((key,position)=>db.query("INSERT INTO keyed_rows VALUES(?,?)").run(position,key));
      const matches=db.query("SELECT position FROM keyed_rows WHERE id=? ORDER BY position LIMIT 2").all(vector.needle) as {position:number}[];
      assert.deepEqual(matches.map(row=>row.position),vector.expectedIndices,vector.id);
    }
    db.exec("CREATE TABLE sparse_row(name TEXT NOT NULL,value REAL NOT NULL)");
    for(const vector of patchCorpus){
      db.exec("DELETE FROM sparse_row");db.query("INSERT INTO sparse_row VALUES(?,?)").run(vector.before.name,vector.before.value);
      const inverse:Record<string,string|number>={};
      for(const field of ["name","value"] as const){
        if(!(field in vector.patch))continue;
        inverse[field]=(db.query(`SELECT ${field} FROM sparse_row`).get() as Record<string,string|number>)[field]!;
        db.query(`UPDATE sparse_row SET ${field}=?`).run((vector.patch as Record<string,string|number>)[field]!);
      }
      assert.deepEqual(inverse,vector.inverse,vector.id);assert.deepEqual(db.query("SELECT * FROM sparse_row").get(),vector.after,vector.id);
      const sibling=!("value" in vector.patch);if(sibling)db.exec("UPDATE sparse_row SET value=99");
      for(const field of ["name","value"] as const){if(field in inverse)db.query(`UPDATE sparse_row SET ${field}=?`).run(inverse[field]!);}
      assert.deepEqual(db.query("SELECT * FROM sparse_row").get(),{name:vector.before.name,value:sibling?99:vector.before.value},vector.id);
    }
    db.exec("CREATE TABLE semantic_diagnostic(level TEXT NOT NULL,code TEXT NOT NULL,message TEXT NOT NULL,target TEXT NOT NULL)");
    for(const vector of diagnosticCorpus){
      db.exec("DELETE FROM semantic_diagnostic");
      db.query("INSERT INTO semantic_diagnostic VALUES(?,?,?,?)").run(vector.level,vector.code,vector.message,JSON.stringify(vector.target));
      const row=db.query("SELECT * FROM semantic_diagnostic").get() as {level:string,code:string,message:string,target:string};
      assert.deepEqual({...row,target:JSON.parse(row.target)},{level:vector.level,code:vector.code,message:vector.message,target:vector.target},vector.id);
    }
    db.exec("CREATE TABLE owned_rows(position INTEGER PRIMARY KEY,id TEXT NOT NULL,name TEXT NOT NULL,value REAL NOT NULL)");
    for(const vector of rowCorpus.cases){
      db.exec("DELETE FROM owned_rows");vector.rows.forEach((row,index)=>db.query("INSERT INTO owned_rows VALUES(?,?,?,?)").run(index,row.id,row.name,row.value));
      const matches=db.query("SELECT position,name,value FROM owned_rows WHERE id=? ORDER BY position LIMIT 2").all(vector.target) as {position:number,name:string,value:number}[];
      const invalid="value" in vector.patch&&Number(vector.patch.value)<0;
      const admitted=matches.length===1&&!invalid;assert.equal(admitted,vector.admitted,vector.id);
      let inverses=0;
      if(admitted&&Object.keys(vector.patch).length){
        inverses++;
        for(const field of ["name","value"] as const){if(field in vector.patch)db.query(`UPDATE owned_rows SET ${field}=? WHERE position=?`).run((vector.patch as Record<string,string|number>)[field]!,matches[0]!.position);}
      }
      assert.equal(inverses,vector.inverseCount,vector.id);assert.deepEqual(db.query("SELECT id,name,value FROM owned_rows ORDER BY position").all(),vector.expected,vector.id);
    }
    db.exec("CREATE TABLE diff_projection(value INTEGER NOT NULL)");
    for(const vector of diffCorpus){
      db.exec("DELETE FROM diff_projection");db.query("INSERT INTO diff_projection VALUES(?)").run(vector.before);
      const original=JSON.stringify(vector.diff);
      if(!vector.diff.refused)db.query("UPDATE diff_projection SET value=?").run(vector.diff.set);
      assert.equal((db.query("SELECT value FROM diff_projection").get() as {value:number}).value,vector.expected,vector.id);
      assert.equal(!vector.diff.refused,vector.admitted,vector.id);assert.equal(JSON.stringify(vector.diff),original,vector.id);
    }
  } finally { db.close(); }
}
