import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";
/** 🧮️ The original slot can return only after every producer reference to it has returned. */
function verifyCanceledHistorySlot(): void {
 const law = corpus.cancellation;
 const original = law.original.repeat(law.repeat);
 const db = new Database(':memory:');
 try {
  db.exec('CREATE TABLE ledger (slot INTEGER PRIMARY KEY, original TEXT); CREATE TABLE live (slot INTEGER, owner TEXT, PRIMARY KEY (slot, owner));');
  const giveBack = db.query(`DELETE FROM live WHERE owner = 'source' AND NOT EXISTS (SELECT 1 FROM live WHERE owner = 'producer')`);
  const custody = db.query(`SELECT
     NOT EXISTS (SELECT 1 FROM live WHERE owner = 'source') AS sourceEmpty,
     EXISTS (SELECT 1 FROM ledger WHERE slot = ? AND original = ?) AS originalRestored,
     NOT EXISTS (SELECT 1 FROM live WHERE owner = 'producer') AS producerReturned`);
  for (let frontier = 0; frontier < law.fundedFrontiers; frontier++) {
   for (const producer of [false, true]) {
    db.exec('DELETE FROM live; DELETE FROM ledger;');
    db.run('INSERT INTO ledger VALUES (?, ?)', [frontier, original]);
    db.run("INSERT INTO live VALUES (?, 'source')", [frontier]);
    if (producer) db.run("INSERT INTO live VALUES (?, 'producer')", [frontier]);
    giveBack.run();
    if (producer) {
     assert.equal((custody.get(frontier, original) as {sourceEmpty: number}).sourceEmpty, 0);
     db.run("DELETE FROM live WHERE owner = 'producer'");
     giveBack.run();
    }
    const result = custody.get(frontier, original) as Record<string, number>;
    assert.deepEqual(Object.fromEntries(Object.entries(result).map(([field, value]) => [field, Boolean(value)])), law.expected);
   }
  }
 } finally { db.close(); }
}

/** 🔗️ Independent SQLite selects the same recorded original operation order. */
export function originalHistoryInputOracle():void{
 verifyCanceledHistorySlot();
 assert(new Ajv({strict:true}).compile(schema)(corpus));const db=new Database(":memory:");
 try{db.exec("CREATE TABLE operations(position INTEGER PRIMARY KEY,value TEXT);CREATE TABLE requested(position INTEGER PRIMARY KEY,source INTEGER)");for(const law of corpus.cases){db.exec("DELETE FROM operations;DELETE FROM requested");law.operations.forEach((value,position)=>db.run("INSERT INTO operations VALUES(?,?)",[position,value]));law.readOrder.forEach((source,position)=>db.run("INSERT INTO requested VALUES(?,?)",[position,source]));const actual=db.query("SELECT o.value FROM requested r JOIN operations o ON o.position=r.source ORDER BY r.position").all()as {value:string}[];assert.deepEqual(actual.map(row=>row.value),law.expected,law.name);assert.deepEqual(law.readOrder.map(index=>law.operations[index]),law.expected,law.name);}}finally{db.close();}
}
