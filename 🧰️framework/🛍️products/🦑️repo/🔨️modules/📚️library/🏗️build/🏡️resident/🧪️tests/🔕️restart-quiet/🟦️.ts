/** 🔕️ Contract: a slice worker that ended because the code of the build tool itself changed under it is started again only after that code has been unchanged for the quiet period (bounded by the maximum wait, cancellable, announced in English and German), so a burst of edits restarts a load once; a worker that ended for any other reason is restarted as before. The quiet verdict matches an independent SQLite oracle on the shared case table.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🔕️quiet/🟨️.mjs
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import cases from "./🔣️.json" with {type:"json"};
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {implementationDriftPathsV1,implementationPathShownV1,implementationQuietV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🔕️quiet/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const drift="Stale source facts that a repeated inference cannot refresh; restart required: file:/w/🏗️build/x.ts / Veraltete Quellfakten; Neustart erforderlich: file:/w/🏗️build/x.ts";
const harness=(exits:{code:number|"done";tail?:string}[],quiet:{milliseconds:number;maximumMilliseconds:number},stamps:Map<string,number|undefined>)=>{
 const starts:number[]=[],exitsAt:number[]=[],progress:any[]=[];let index=0;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,heartbeatMilliseconds:10,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  restartPolicy:{backoffMilliseconds:[1],settleMilliseconds:60000,progressMilliseconds:60000,totalMaximum:10,sameCauseMaximum:10},quietPolicy:quiet,implementationStamps:(paths:Iterable<string>)=>new Map([...paths].map(path=>[path,stamps.get(path)])),
  reaper:{reap:async()=>0,killNow:()=>undefined},
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:()=>{const outcome=exits[index++]!,listeners:Record<string,(value:unknown)=>void>={};starts.push(Date.now());queueMicrotask(()=>{if(outcome.code==="done")listeners.message?.({type:"done",graph:{}});else{exitsAt.push(Date.now());listeners.exit?.(outcome.code);}});return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0,stderrTail:()=>outcome.tail??""};}} as never);
 return{pool,starts,exitsAt,progress};
};

test("the quiet verdict of every case of the table equals the table and the SQLite oracle",()=>{
 const db=new Database(":memory:");
 try{
  for(const row of cases.quiet){
   db.run("CREATE TABLE stamps(path TEXT,at REAL)");
   for(const [path,at] of row.stamps)db.run("INSERT INTO stamps VALUES(?,?)",[path as string,at as number|null]);
   const newest=db.query<{path:string;at:number},[]>("SELECT path,at FROM stamps WHERE at IS NOT NULL ORDER BY at DESC,path ASC LIMIT 1").get();
   const oracle=db.query<{quietFor:number},[number,number|null,number]>("SELECT CASE WHEN ?2 IS NULL THEN ?3 ELSE MAX(0,?1-?2) END AS quietFor").get(row.nowMs,newest?.at??null,row.quietMilliseconds)!.quietFor;
   db.run("DROP TABLE stamps");
   const verdict=implementationQuietV1({stamps:row.stamps.map(([path,at])=>[path as string,at===null?undefined:at as number] as const),nowMs:row.nowMs,quietMilliseconds:row.quietMilliseconds});
   expect(verdict.quietForMilliseconds,row.name).toBe(row.quietForMilliseconds);expect(verdict.quietForMilliseconds,row.name).toBe(oracle);
   expect(verdict.quiet,row.name).toBe(row.quiet);expect(verdict.quiet,row.name).toBe(oracle>=row.quietMilliseconds);
   expect(verdict.newestPath,row.name).toBe((row as {newestPath?:string}).newestPath);expect(verdict.newestPath,row.name).toBe(newest?.path);
  }
 }finally{db.close();}
});

test("the changed implementation a restart cause names is read from both message forms, without repetition, and nothing else is",()=>{
 for(const row of cases.drift)expect(implementationDriftPathsV1(row.cause),row.name).toEqual(row.paths);
 expect(implementationPathShownV1("/w","/w/🏗️build/x.ts")).toBe("🏗️build/x.ts");expect(implementationPathShownV1("/w","/elsewhere/x.ts")).toBe("/elsewhere/x.ts");expect(implementationPathShownV1("/w",undefined)).toBe("");
});

test("a burst of edits of the tool code restarts the worker once, after the code has been quiet for the quiet period, and says so in both languages",async()=>{
 const stamps=new Map<string,number|undefined>([["/w/🏗️build/x.ts",Date.now()]]),edits=[40,80,120];
 const h=harness([{code:75,tail:"[graph] reading\n"+drift},{code:"done"}],{milliseconds:150,maximumMilliseconds:5000},stamps);
 const timers=edits.map(delay=>setTimeout(()=>stamps.set("/w/🏗️build/x.ts",Date.now()),delay));
 const began=Date.now();
 await h.pool.run({root:"/w"},{onProgress:(_ms:number,detail:any)=>h.progress.push(detail)});await h.pool.close();timers.forEach(clearTimeout);
 expect(h.starts.length,"one restart for the whole burst").toBe(2);
 expect(h.starts[1]!-(began+edits.at(-1)!),"started only after the last edit was quiet").toBeGreaterThanOrEqual(140);
 const settling=h.progress.filter(detail=>detail?.decision?.key==="settling").map(detail=>detail.decision);
 expect(settling.length).toBeGreaterThan(0);expect(settling[0]).toMatchObject({key:"settling",budget:0,kind:"75",limit:5,cause:"🏗️build/x.ts"});
 const text=renderTextV1("host-slice-owners-settling",{seconds:9,phase:"reservation",done:0,total:0,owners:"",used:12,budget:60,limit:600,kind:"75",cause:"🏗️build/x.ts"},["en","de"])!;
 for(const language of["en","de"] as const){expect(text[language]).toContain("🏗️build/x.ts");expect(text[language]).toContain("60");expect(text[language]).toContain("600");expect(text[language]).not.toMatch(/\{[a-z]+\}/u);}
 expect(text.en).toContain("unchanged");expect(text.de).toContain("unverändert");
});

test("edits that never stop delay the restart by the maximum wait only",async()=>{
 const stamps=new Map<string,number|undefined>([["/w/🏗️build/x.ts",Date.now()]]);
 const churn=setInterval(()=>stamps.set("/w/🏗️build/x.ts",Date.now()),5);
 const h=harness([{code:75,tail:drift},{code:"done"}],{milliseconds:10000,maximumMilliseconds:250},stamps);
 try{await h.pool.run({root:"/w"});await h.pool.close();}finally{clearInterval(churn);}
 expect(h.starts.length).toBe(2);const delay=h.starts[1]!-h.exitsAt[0]!;
 expect(delay).toBeGreaterThanOrEqual(240);expect(delay).toBeLessThan(5000);
});

test("a worker that ended for another reason, or whose named code is already quiet, is restarted without waiting for quiet",async()=>{
 const stamps=new Map<string,number|undefined>([["/w/🏗️build/x.ts",Date.now()-60000]]);
 for(const tail of["TypeError: boom",drift]){
  const h=harness([{code:75,tail},{code:"done"}],{milliseconds:30000,maximumMilliseconds:60000},stamps);
  await h.pool.run({root:"/w"},{onProgress:(_ms:number,detail:any)=>h.progress.push(detail)});await h.pool.close();
  expect(h.starts.length,tail).toBe(2);expect(h.starts[1]!-h.exitsAt[0]!,tail).toBeLessThan(1000);
  expect(h.progress.some(detail=>detail?.decision?.key==="settling"),tail).toBe(false);
 }
});

test("a plan cancelled while the code settles ends at once and starts no worker",async()=>{
 const stamps=new Map<string,number|undefined>([["/w/🏗️build/x.ts",Date.now()]]),controller=new AbortController();
 const h=harness([{code:75,tail:drift},{code:"done"}],{milliseconds:60000,maximumMilliseconds:600000},stamps);
 const running=h.pool.run({root:"/w"},{signal:controller.signal,onProgress:(_ms:number,detail:any)=>{if(detail?.decision?.key==="settling")controller.abort(new Error("cancelled"));}});
 const began=Date.now(),error=await running.then(()=>undefined,(failure:unknown)=>failure);await h.pool.close();
 expect(String((error as Error)?.message)).toContain("cancelled");expect(Date.now()-began).toBeLessThan(5000);expect(h.starts.length).toBe(1);
});
