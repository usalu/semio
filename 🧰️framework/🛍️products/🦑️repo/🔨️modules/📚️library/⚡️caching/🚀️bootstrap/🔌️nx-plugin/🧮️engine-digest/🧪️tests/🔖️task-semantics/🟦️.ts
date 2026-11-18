/** 🔖️ Contract: the task semantics revision of `../../🔣️.json` is bumped on purpose. The owner argument wire the executors hand a task (the encoded envelope of every fixture case, its round trip, the environment left after consumption and the refusals of the invalid envelopes) is digested and recorded per revision in `./🔣️.json`. The law fails when the wire changes without a new history row for the declared revision, when a revision is reused with other outputs, and when the history is not strictly increasing. Executor behaviour outside this wire stays a declared judgment of the revision, see `../../🔣️.json`. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import declared from "../../🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const fixture=JSON.parse(readFileSync(new URL("../../../📤️arguments/🧪️tests/🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
const observe=async()=>{
 const api=await import(new URL("../../../../../../🏗️build/📤️arguments/🟨️.mjs",import.meta.url).href),name="SEMIO_OWNER_ARGUMENTS";
 const encoded=fixture.cases.map((row:{id:string;arguments:string[]})=>{const wire=api.encodeOwnerArgumentsV1(row.arguments),consumed=api.consumeOwnerArgumentsV1({[name]:wire,KEEP:"1"});return{id:row.id,wire,arguments:consumed.arguments,environment:Object.keys(consumed.environment)};});
 const refused=fixture.invalid.map((row:unknown,index:number)=>{try{api.consumeOwnerArgumentsV1({[name]:JSON.stringify(row)});return{index,refused:false};}catch{return{index,refused:true};}});
 return{encoded,refused,empty:api.consumeOwnerArgumentsV1({KEEP:"1"})};
};
const digest=(value:unknown)=>createHash("sha256").update(JSON.stringify(value)).digest("hex");

test("the history is closed by its schema and strictly increasing",()=>{
 const ajv=new Ajv({strict:false});expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 const revisions=corpus.history.map(row=>row.revision);expect(revisions).toEqual([...revisions].sort((a,b)=>a-b));expect(new Set(revisions).size).toBe(revisions.length);
 expect(new Set(corpus.history.map(row=>row.outputs)).size,"a bump without a changed output is a plain bump and needs its own row text").toBeGreaterThan(0);
});

test("the declared revision is the newest history row and the owner argument wire equals the recorded outputs of that revision",async()=>{
 const outputs=digest(await observe()),last=corpus.history[corpus.history.length-1]!;
 expect(declared.semantics.revision,"the revision of the policy and the newest history row must be the same number").toBe(last.revision);
 expect(outputs,"the task wire changed: bump `semantics.revision` in ../../🔣️.json and append a history row with these outputs").toBe(last.outputs);
},60000);

test("the wire carries literal arguments in one environment key, refuses every invalid envelope and leaves the rest of the environment alone",async()=>{
 const wire=await observe();
 for(const row of wire.encoded){expect(row.arguments,row.id).toEqual(fixture.cases.find((entry:{id:string})=>entry.id===row.id).arguments);expect(row.environment,row.id).toEqual(["KEEP"]);}
 for(const row of wire.refused)expect(row.refused,String(row.index)).toBe(true);
 expect(wire.empty.arguments).toEqual([]);
});
