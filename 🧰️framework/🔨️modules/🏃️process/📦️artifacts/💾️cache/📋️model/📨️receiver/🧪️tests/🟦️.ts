import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{NativeArtifactCacheReceiverLedgerV1}from"../🟦️.ts";
import{readNativeArtifactCachePolicyV1}from"../../../⚙️policy/🟦️.ts";
import{receiveScriptProcessInvocation}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import policyCorpus from"../../../🧪️tests/🔣️.json";
import corpus from"./🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("neutral cache receiver rejection stops new work before retained siblings settle",async()=>receiveScriptProcessInvocation(process.env,async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");
 try{for(const row of corpus.cases){const ledger=new NativeArtifactCacheReceiverLedgerV1(readNativeArtifactCachePolicyV1(policyCorpus.smallPolicy)),marker=row.failure==="undefined"?undefined:Error("actual cache receiver rejection");let release!:()=>void,newEffects=0,refusals=0,siblings=0,settled=false,caught=false,failure:unknown;
  if(row.kind!=="read"&&row.kind!=="replace")throw Error("Closed cache receiver kind required");const operation=ledger.run(row.kind,async()=>{throw marker;}),sibling=row.kind==="read"?ledger.run("read",async()=>{await new Promise<void>(done=>{release=done;});siblings++;}):Promise.resolve();
  try{try{await operation;}catch(error){caught=true;failure=error;}for(const kind of["read","replace"]as const){try{await ledger.run(kind,async()=>{newEffects++;});}catch{refusals++;}}
   const closing=ledger.close();void closing.then(()=>{settled=true;},()=>{settled=true;});await new Promise<void>(done=>setImmediate(done));expect(settled).toBe(row.kind!=="read");
  }finally{release?.();await sibling;await ledger.close().catch(error=>{caught=true;failure=error;});}
  const oracle=db.query("WITH events(seq,kind) AS (VALUES (1,'failure'),(2,'read'),(3,'replace')), attempts AS (SELECT EXISTS(SELECT 1 FROM events failed WHERE failed.kind='failure' AND failed.seq<current.seq) AS closed FROM events current WHERE current.kind IN ('read','replace')) SELECT sum(1-closed) AS effects, sum(closed) AS refused, CASE WHEN ? = 'read' THEN 1 ELSE 0 END AS completed FROM attempts").get(row.kind)as{effects:number;refused:number;completed:number};expect(newEffects).toBe(oracle.effects);expect(newEffects).toBe(row.expectedNewEffects);expect(refusals).toBe(oracle.refused);expect(refusals).toBe(row.expectedRefusals);expect(siblings).toBe(oracle.completed);expect(siblings).toBe(row.expectedSiblingCompletions);expect(caught&&Object.is(failure,marker)).toBe(true);
 }
 console.log("[DEBUG] Actual cache receiver ledger refused new read and replacement callbacks after Error or undefined while accepted siblings drained against Ajv and SQLite");
 }finally{db.close();}
}));
