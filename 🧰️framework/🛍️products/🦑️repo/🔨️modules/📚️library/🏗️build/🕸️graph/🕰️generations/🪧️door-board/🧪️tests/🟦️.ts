/** 🪧 Contract: the door board names who holds the door and how far it is (pid, command, start, code, phase and owners done of total, time without progress) in English and German; a holder that did not advance for the declared time or runs other code than the library is asked to hand over, one that did not answer the ask loses the door; progress means the phase changed or more owners are done, never elapsed time; every decision equals the independent Python model. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv2020 from "ajv/dist/2020";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../🔐️custody/🧬️schema/🔣️.json";
import {advanceProgressV1,askReasonV1,currentHolderV1,doorSuffixV1,holderSuffixV1,holderTextV1,idleMillisecondsV1,isDeposedV1,readAskV1,readDoorV1,readHolderV1,readProgressV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle/🐍️.py"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=schema.definitions.Policy.const as Record<string,number>,sha="a".repeat(64),other="b".repeat(64);
const holder=(overrides={})=>({pid:4242,command:"script.ts repo show projects",startedMs:Date.UTC(2026,10,17,16,0,0),digest:sha,...overrides});
const progress=(overrides={})=>({phase:"owners",done:120,total:482,advancedMs:1_000_000,...overrides});

test("the declared numbers, the documents and the texts are closed under the schema and speak both languages with the same placeholders",()=>{
 expect(policy.stallMilliseconds).toBeGreaterThan(policy.noticeMilliseconds!);expect(policy.askGraceMilliseconds).toBeGreaterThan(policy.pollMilliseconds!);expect(policy.progressWriteMilliseconds).toBeLessThan(policy.stallMilliseconds!);
 const ajv=new Ajv2020({strict:false}),root={...schema,$id:"semio:test/custody"};ajv.addSchema(root);
 const check=(name:string,value:unknown)=>ajv.validate({$ref:"semio:test/custody#/definitions/"+name},value);
 expect(check("Holder",holder())).toBe(true);expect(check("Progress",progress())).toBe(true);expect(check("Ask",{version:1,reason:"stalled",atMs:5,by:{pid:1,digest:""}})).toBe(true);
 for(const bad of[holder({pid:0}),holder({digest:"xyz"}),{...holder(),extra:1}])expect(check("Holder",bad)).toBe(false);
 for(const bad of[progress({phase:"Owners"}),progress({done:-1}),{...progress(),extra:1}])expect(check("Progress",bad)).toBe(false);
 expect(readHolderV1(holder())).toEqual(holder());expect(readProgressV1(progress())).toEqual(progress());
 for(const bad of[holder({pid:0}),holder({digest:"xyz"}),{...holder(),extra:1},null])expect(()=>readHolderV1(bad)).toThrow();
 for(const bad of[progress({phase:"Owners"}),progress({done:-1}),{...progress(),extra:1}])expect(()=>readProgressV1(bad)).toThrow();
 expect(()=>readAskV1({version:1,reason:"because",atMs:1,by:{pid:1,digest:""}})).toThrow();
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const key of["holder","holderSuffix","askedStalled","askedStale","deposed","handoverStalled","handoverStale","blocked","blockedAll"] as const)expect(placeholders(schema.definitions.Messages.const.de[key]),key).toEqual(placeholders(schema.definitions.Messages.const.en[key]));
});

test("a holder is described by pid, command, start, code, phase with owners done of total and the time without progress, in English and German",()=>{
 const claim={state:"held",holder:holder(),progress:progress()},now=1_000_000+12_400;
 const en=holderTextV1(claim,now,"en"),de=holderTextV1(claim,now,"de");
 for(const text of[en,de]){expect(text).toContain("pid 4242");expect(text).toContain("script.ts repo show projects");expect(text).toContain("2026-11-17 16:00:00");expect(text).toContain("aaaaaaaaaaaa");expect(text).toContain("120");expect(text).toContain("482");expect(text).toContain("owners");expect(text).toContain("12 s");}
 expect(en).toContain("120 of 482");expect(de).toContain("120 von 482");expect(de).toContain("gestartet");
 expect(holderSuffixV1(claim,now,"en").startsWith("; holder: ")).toBe(true);expect(holderSuffixV1(claim,now,"de").startsWith("; Halter: ")).toBe(true);
 expect(holderSuffixV1({state:"held"},now,"en")).toBe("");expect(holderTextV1({state:"held",holder:holder()},now,"en")).toContain("? s");
});

test("the command of this process is cut to the declared length and starts with the script by name",()=>{
 const own=currentHolderV1({digest:other,pid:7,argv:["bun","/very/long/path/to/script.ts","repo","show","projects"],uptimeSeconds:90,now:()=>100_000});
 expect(own).toEqual({pid:7,command:"script.ts repo show projects",startedMs:10_000,digest:other});
 expect(currentHolderV1({argv:["bun","s.ts","x".repeat(500)]}).command.length).toBe(policy.commandCharacters!);
});

test("progress advances when the phase changes or more owners are done, never with time alone",()=>{
 const first=advanceProgressV1(undefined,{phase:"start",done:0,total:0},1000);expect(first.advancedMs).toBe(1000);
 const same=advanceProgressV1(first,{phase:"start",done:0,total:0},9000);expect(same.advancedMs).toBe(1000);
 const grew=advanceProgressV1(same,{phase:"start",done:1,total:10},9500);expect(grew).toEqual({phase:"start",done:1,total:10,advancedMs:9500});
 expect(advanceProgressV1(grew,{phase:"start",done:1,total:10},99999).advancedMs).toBe(9500);
 expect(advanceProgressV1(grew,{phase:"take",done:0,total:10},20000).advancedMs).toBe(20000);
 expect(advanceProgressV1(grew,{phase:"Bad Phase",done:-3},30000)).toEqual({phase:"start",done:1,total:10,advancedMs:9500});
});

test("every decision to ask and to depose over seeded claims equals the Python model, with both outcomes and both reasons covered",()=>{
 let seed=20261117;const random=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:600},(_,at)=>{
  const now=2_000_000+Math.floor(random()*1_000_000),withProgress=random()<0.8,withHolder=random()<0.85,asked=random()<0.3;
  const claim:Record<string,unknown>={state:random()<0.8?"held":"waiting",...withHolder?{holder:holder({digest:random()<0.2?"":random()<0.5?sha:other})}:{},...withProgress?{progress:progress({advancedMs:now-Math.floor(random()*1_400_000)})}:{},...asked?{ask:{version:1,reason:"stalled",atMs:now-Math.floor(random()*700_000),by:{pid:1,digest:""}}}:{}};
  return{claim,now,current:random()<0.6?(random()<0.5?sha:other):undefined,id:String(at)};
 });
 const advances=Array.from({length:300},()=>{const previous=random()<0.9?progress({done:Math.floor(random()*5),phase:random()<0.5?"owners":"take",advancedMs:100}):undefined,phases=["owners","take","Bad",7],phase=phases[Math.floor(random()*4)];return{previous,event:{phase,done:random()<0.8?Math.floor(random()*7):-1,total:Math.floor(random()*9)},now:5000+Math.floor(random()*100)};});
 const ours=cases.map(row=>({reason:askReasonV1({claim:row.claim as never,now:row.now,current:row.current})??null,idleMs:idleMillisecondsV1(row.claim as never,row.now)??null,deposed:isDeposedV1(row.claim as never,row.now)}));
 const ourAdvances=advances.map(row=>advanceProgressV1(row.previous,row.event as never,row.now));
 expect(ours.filter(row=>row.reason==="stalled").length).toBeGreaterThan(20);expect(ours.filter(row=>row.reason==="stale-code").length).toBeGreaterThan(20);expect(ours.filter(row=>row.reason===null).length).toBeGreaterThan(20);expect(ours.filter(row=>row.deposed).length).toBeGreaterThan(10);
 if(python){const result=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify({stall:policy.stallMilliseconds,grace:policy.askGraceMilliseconds,cases:cases.map(row=>({claim:row.claim,now:row.now,current:row.current})),advances}))});expect(result.exitCode).toBe(0);const model=JSON.parse(result.stdout.toString());expect(model.decisions).toEqual(ours);expect(model.advances).toEqual(ourAdvances);}
});

test("the front door reads the claims on disk without the custody: only held claims of living processes, with progress and ask, named in the language asked for",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"door-board-")));roots.push(root);const directory=join(root,"s");mkdirSync(directory);
 const claim=(token:string,ticket:number,state:string,pid:number)=>writeFileSync(join(directory,token+".json"),JSON.stringify({version:1,token,ticket,owners:[],structural:true,state,queuedMs:1,holder:holder({pid})}));
 const a="a".repeat(24),b="b".repeat(24),c="c".repeat(24),d="d".repeat(24);
 claim(a,1,"held",process.pid);claim(b,2,"waiting",process.pid);claim(c,3,"held",999999991);writeFileSync(join(directory,d+".json"),"{ torn");
 writeFileSync(join(directory,a+".progress.json"),JSON.stringify(progress({advancedMs:Date.now()-5000})));writeFileSync(join(directory,a+".ask.json"),JSON.stringify({version:1,reason:"stalled",atMs:1,by:{pid:2,digest:""}}));
 const rows=readDoorV1(directory,{isAlive:pid=>pid===process.pid});
 expect(rows.map(row=>row.token)).toEqual([a]);expect(rows[0]!.progress!.done).toBe(120);expect(rows[0]!.ask!.reason).toBe("stalled");
 const en=doorSuffixV1(directory,{language:"en",isAlive:pid=>pid===process.pid}),de=doorSuffixV1(directory,{language:"de",isAlive:pid=>pid===process.pid});
 expect(en).toContain("holder: pid "+process.pid);expect(en).toContain("120 of 482");expect(de).toContain("Halter: pid "+process.pid);expect(de).toContain("120 von 482");
 expect(doorSuffixV1(join(root,"absent"),{language:"en"})).toBe("");expect(doorSuffixV1(undefined,{language:"en"})).toBe("");
});
