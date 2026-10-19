import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
/** 📚️ Original ingress selection and conflict offsets agree with independent SQLite relations. */
export function originalIngressOracle():void{
 assert(new Ajv({strict:true}).compile(schema)(corpus));const db=new Database(":memory:");
 try{db.exec("CREATE TABLE rows(origin INTEGER,ordinal INTEGER,id TEXT,rows INTEGER);CREATE TABLE requested(ordinal INTEGER,id TEXT)");for(const law of corpus.cases){db.exec("DELETE FROM rows;DELETE FROM requested");for(const [origin,source]of [law.ledger,law.batch].entries())source.forEach((row,ordinal)=>db.run("INSERT INTO rows VALUES(?,?,?,?)",origin,ordinal,row.id,row.rows));law.order.forEach((id,ordinal)=>db.run("INSERT INTO requested VALUES(?,?)",ordinal,id));const selected=db.query("WITH selected AS(SELECT q.ordinal,q.id,(SELECT rows FROM rows r WHERE r.id=q.id ORDER BY origin DESC,ordinal LIMIT 1) rows FROM requested q) SELECT id,coalesce(sum(rows) OVER(ORDER BY ordinal ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING),0) offset,rows FROM selected ORDER BY ordinal").all()as {id:string;offset:number;rows:number|null}[];assert.deepEqual(selected.some(row=>row.rows===null)?null:selected,law.selected,law.id);const committed=db.query("SELECT b.id FROM rows b WHERE origin=1 AND NOT EXISTS(SELECT 1 FROM rows a WHERE a.origin=0 AND a.id=b.id) ORDER BY ordinal").all()as {id:string}[];assert.deepEqual(committed.map(row=>row.id),law.committed,law.id);}}finally{db.close();}
 console.log("[DEBUG] original ingress strict Ajv/SQLite oracle cases="+corpus.cases.length);
}
