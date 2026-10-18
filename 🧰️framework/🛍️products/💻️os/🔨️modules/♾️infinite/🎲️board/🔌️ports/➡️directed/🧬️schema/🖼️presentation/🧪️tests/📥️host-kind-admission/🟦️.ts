import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import Ajv from "ajv";

/** 📥️ Independent schema admission for every authored host kind under caller cancellation. */
export function runHostKindAdmissionChecks(signal?:AbortSignal):number {
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
 const corpus=read("../../../../🧪️tests/🧫️fixtures/🖼️presentation/📥️host-kind-admission/🔣️.json"),schema=read("../../🔣️.json");
 assert.equal(corpus.schema,"semio.dag.host-kind-admission/v1");
 assert.equal(new Set(corpus.cases.map((row:{id:string})=>row.id)).size,15);
 const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/DagHostSnapshot"});
 for(const row of corpus.cases){if(signal?.aborted)throw Error("host-kind admission cancelled");assert.equal(validate(row.snapshot),row.accepted,row.id);}
 return corpus.cases.length;
}
