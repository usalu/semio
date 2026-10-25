import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
/** 🛬️ Independent SQLite BLOB and Buffer byte counts validate cumulative original receiving across turns. */
export function originalJobReceivingOracle():void{assert(new Ajv({strict:true}).compile(schema)(corpus));const db=new Database(":memory:");try{const law=corpus.receiptLaw;const receipt=db.query("SELECT ? + ? copied,? capacity").get(law.childCopy,law.parentCopy,law.childCapacity)as {copied:number,capacity:number};assert.equal(receipt.copied,law.expectedCopy);assert.equal(receipt.capacity,law.expectedCapacity);for(const law of corpus.cases){const length=(db.query("SELECT length(?) bytes").get(Buffer.from(law.text,"utf8"))as {bytes:number}).bytes;assert.equal(length,Buffer.byteLength(law.text,"utf8"));const owned=law.cancelled?0:law.turnCapacity.reduce((sum,capacity)=>{assert(length<=capacity);return sum+length;},0);assert.equal(owned,law.expectedOwned);}for(const law of corpus.retirementCases){const permitted=(db.query("SELECT ? AND ? > 0 permitted").get(Number(law.deadlineOpen),law.fuel) as {permitted:number}).permitted;assert.equal(Boolean(permitted),law.visited);assert.equal(law.deadlineOpen&&law.fuel>0,law.visited);}}finally{db.close();}}
