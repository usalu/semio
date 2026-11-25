/** 🛎️ Contract: a `global` fact whose resolver needs asynchronous work registers a `prepare` beside it (`registerGlobalFactV1(name, holds, prepare)`); every judge of a record's rows (`projectRowsHoldV1`, `settleClosureRowsV1`) and `prepareGlobalFactsV1` itself hand each preparer exactly its own `global` rows once before the synchronous verdict, a resolver without a preparer is judged as before, rows without such a fact prepare nothing, and the verdict is the resolver's own. */
import{expect,test}from"bun:test";
import{observationRowFreshV1,prepareGlobalFactsV1,projectRowsHoldV1,registerGlobalFactV1,settleClosureRowsV1}from"../../🟨️.mjs";

const row=(name:string,segment:string,fact:unknown)=>({kind:"global",path:JSON.stringify([name,"/workspace",[segment]]),fact:JSON.stringify(fact)});

test("each preparer gets its own global rows before the verdict, once per judge, and the resolver decides",async()=>{
 const prepared:string[][]=[],ready=new Set<string>();
 registerGlobalFactV1("gr22-prepared",(_root,segments,fact)=>ready.has((segments as string[])[0]!)&&fact===1,async rows=>{prepared.push(rows.map(item=>JSON.parse(item.path)[2][0]));await Promise.resolve();for(const item of rows)ready.add(JSON.parse(item.path)[2][0]);});
 registerGlobalFactV1("gr22-plain",(_root,segments)=>(segments as string[])[0]==="yes");
 const rows=[row("gr22-prepared","a",1),row("gr22-plain","yes",0),row("gr22-prepared","b",1),{kind:"file",path:"/nowhere",fact:{size:0}}];
 expect(await projectRowsHoldV1(rows.slice(0,3),undefined,rest=>rest.every(item=>observationRowFreshV1(item)))).toBe(true);
 expect(prepared).toEqual([["a","b"]]);
 ready.clear();prepared.length=0;
 await settleClosureRowsV1(rows.slice(0,3),undefined);
 expect(prepared).toEqual([["a","b"]]);expect([...ready].sort()).toEqual(["a","b"]);
 prepared.length=0;await prepareGlobalFactsV1([row("gr22-plain","yes",0),{kind:"file",path:"/x",fact:{}}]);expect(prepared).toEqual([]);
 ready.clear();expect(await projectRowsHoldV1([row("gr22-prepared","c",2)],undefined,rest=>rest.every(item=>observationRowFreshV1(item)))).toBe(false);
 expect(prepared).toEqual([["c"]]);
 registerGlobalFactV1("gr22-prepared",()=>true);prepared.length=0;await prepareGlobalFactsV1([row("gr22-prepared","d",1)]);expect(prepared).toEqual([]);
});
