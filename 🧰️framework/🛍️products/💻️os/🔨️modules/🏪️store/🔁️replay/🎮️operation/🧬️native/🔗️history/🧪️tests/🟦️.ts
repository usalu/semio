import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";
/** 🔗️ Independent SQLite selects the same recorded original operation order. */
export function originalHistoryInputOracle():void{
 assert(new Ajv({strict:true}).compile(schema)(corpus));const db=new Database(":memory:");
 try{db.exec("CREATE TABLE operations(position INTEGER PRIMARY KEY,value TEXT);CREATE TABLE requested(position INTEGER PRIMARY KEY,source INTEGER)");for(const law of corpus.cases){db.exec("DELETE FROM operations;DELETE FROM requested");law.operations.forEach((value,position)=>db.run("INSERT INTO operations VALUES(?,?)",[position,value]));law.readOrder.forEach((source,position)=>db.run("INSERT INTO requested VALUES(?,?)",[position,source]));const actual=db.query("SELECT o.value FROM requested r JOIN operations o ON o.position=r.source ORDER BY r.position").all()as {value:string}[];assert.deepEqual(actual.map(row=>row.value),law.expected,law.name);assert.deepEqual(law.readOrder.map(index=>law.operations[index]),law.expected,law.name);}}finally{db.close();}
}
