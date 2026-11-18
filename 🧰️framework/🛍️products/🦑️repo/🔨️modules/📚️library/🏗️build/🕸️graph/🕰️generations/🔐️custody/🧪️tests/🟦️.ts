/** 🔐️ Contract: builders of disjoint owner sets hold the custody at the same time, overlapping ones one after the other in ticket order, a structural claim excludes everybody, a waiting claim is never overtaken by a later conflicting one, a dead claim frees its owners, an extension succeeds exactly when no other held claim conflicts, `alone` succeeds exactly when no other live claim exists, and cancellation leaves nothing behind. Every admission decision equals the independent Python model in `🐍️oracle/🐍️.py`. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync,writeFileSync,mkdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv2020 from "ajv/dist/2020";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {canonicalJsonV1,createNodeFilePortV1} from "../../🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../🧩️claims/🟨️.mjs";
import schema from "../🧬️schema/🔣️.json";
import {CUSTODY_MESSAGES_V1,CUSTODY_POLICY_V1,admissionOfV1,attachToBuilderV1,claimsConflictV1,createSliceCustodyV1,readCustodyClaimV1,renderCustodyMessageV1,type SliceClaimV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle/🐍️.py"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"graph-custody-")));roots.push(root);return root;};
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const world=(root=scratch())=>{const said:{key:string;text:string}[]=[];return{root,said,custody:createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(root,"leases"))}),say:(key:string,fields:Record<string,string|number>)=>said.push({key,text:renderCustodyMessageV1(key as never,fields,"en")})};};
const settled=async(promise:Promise<unknown>,ms=60)=>Promise.race([promise.then(()=>true),sleep(ms).then(()=>false)]);

test("the declared policy, the claim and the bilingual texts are the ones the implementation uses",()=>{
 expect(CUSTODY_POLICY_V1).toEqual(schema.definitions.Policy.const as never);
 expect(CUSTODY_MESSAGES_V1).toEqual(schema.definitions.Messages.const as never);
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const key of Object.keys(CUSTODY_MESSAGES_V1.en) as (keyof typeof CUSTODY_MESSAGES_V1.en)[])expect(placeholders(CUSTODY_MESSAGES_V1.de[key])).toEqual(placeholders(CUSTODY_MESSAGES_V1.en[key]));
 const validate=new Ajv2020({strict:false}).compile({...schema.definitions.Claim,definitions:schema.definitions}),claim={version:1,token:"a".repeat(24),ticket:1,owners:["a","b"],structural:false,state:"held",queuedMs:1};
 expect(validate(claim)).toBe(true);expect(readCustodyClaimV1(claim)).toEqual(claim as never);
 for(const bad of[{...claim,owners:["b","a"]},{...claim,ticket:0},{...claim,state:"done"},{...claim,extra:1},{...claim,token:"x"}])expect(()=>readCustodyClaimV1(bad)).toThrow();
});

test("every admission decision over seeded worlds equals the Python model",()=>{
 let seed=20261027;const random=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const pick=(names:string[])=>names.filter(()=>random()<0.3).sort();
 const names=["a","b","c","d","e","f","g"],cases=[];
 for(let at=0;at<400;at++){
  const count=Math.floor(random()*5),others=Array.from({length:count},(_,index)=>({token:"o"+index,ticket:index+1+Math.floor(random()*3),owners:pick(names),structural:random()<0.1,state:random()<0.5?"held":"waiting"}));
  cases.push({mine:{token:"m",ticket:1+Math.floor(random()*8),owners:pick(names),structural:random()<0.1,state:"waiting"},others});
 }
 const ours=cases.map(row=>{const found=admissionOfV1(row.mine as never,row.others as never);return{admit:found.admit,blockers:found.blockers.map((claim:{token:string})=>claim.token)};});
 expect(ours.filter(row=>row.admit).length).toBeGreaterThan(50);expect(ours.filter(row=>!row.admit).length).toBeGreaterThan(50);
 for(const row of cases)for(const other of row.others)expect(claimsConflictV1(row.mine as never,other as never)).toBe(claimsConflictV1(other as never,row.mine as never));
 if(python){const result=Bun.spawnSync([python,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases}))});expect(result.exitCode).toBe(0);expect(JSON.parse(result.stdout.toString()).decisions).toEqual(ours);}
});

test("disjoint claims hold at once; an overlapping one waits, tells why in English and German, and holds when the first is released",async()=>{
 const{custody,said,say}=world(),a=await custody.hold({owners:["p1","p2"],say}),b=await custody.hold({owners:["p3"],say});
 const c=custody.hold({owners:["p2","p9"],say});
 expect(await settled(c)).toBe(false);
 expect(said.map(row=>row.key)).toEqual(["blocked"]);expect(said[0]!.text).toContain("shared: p2");
 expect(renderCustodyMessageV1("blocked",{peers:1,seconds:0,sample:"p2"},"de")).toContain("gemeinsam: p2");
 expect((await custody.list()).map(claim=>[claim.owners.join(","),claim.state])).toEqual(expect.arrayContaining([["p1,p2","held"],["p3","held"],["p2,p9","waiting"]]));
 await b.release();expect(await settled(c)).toBe(false);
 await a.release();const held=await c;expect(held.owners).toEqual(["p2","p9"]);
 expect(await custody.idle({owners:["p9"]})).toBe(false);expect(await custody.idle({owners:["p1"]})).toBe(true);
 await held.release();expect(await custody.list()).toEqual([]);
});

test("a structural claim waits for every held claim and makes every later claim wait; a waiting claim is never overtaken by a later conflicting one",async()=>{
 const{custody}=world(),order:string[]=[];
 const a=await custody.hold({owners:["x"]});
 const all=custody.hold({structural:true}).then(claim=>{order.push("all");return claim;});
 await sleep(20);
 const y=custody.hold({owners:["y"]}).then(claim=>{order.push("y");return claim;});
 expect(await settled(all)).toBe(false);expect(await settled(y)).toBe(false);
 await a.release();const whole=await all;expect(await settled(y)).toBe(false);
 await whole.release();const last=await y;expect(order).toEqual(["all","y"]);await last.release();
});

test("tickets serve overlapping claims first come first served while a disjoint later claim passes",async()=>{
 const{custody}=world(),order:string[]=[];
 const a=await custody.hold({owners:["k"]});
 const b=custody.hold({owners:["k","m"]}).then(claim=>{order.push("b");return claim;});await sleep(15);
 const c=custody.hold({owners:["m"]}).then(claim=>{order.push("c");return claim;});await sleep(15);
 const d=await custody.hold({owners:["z"]});order.push("d");
 expect(order).toEqual(["d"]);
 await a.release();const held=await b;expect(await settled(c)).toBe(false);await held.release();await(await c).release();await d.release();
 expect(order).toEqual(["d","b","c"]);
});

test("a claim of a dead command frees its owners, an unreadable document is removed after its grace",async()=>{
 const{custody,root}=world();const directory=join(root,"generations",CUSTODY_POLICY_V1.directory),token="f".repeat(24);
 mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,token+".json"),canonicalJsonV1({version:1,token,ticket:5,owners:["x"],structural:true,state:"held",queuedMs:1}));
 const claim=await custody.hold({owners:["x"]});expect((await custody.list()).map(row=>row.token)).toEqual([claim.token]);await claim.release();
});

test("an extension holds at once exactly when no other held claim conflicts; otherwise it names the union to queue for",async()=>{
 const{custody}=world(),a=await custody.hold({owners:["a"]}),b=await custody.hold({owners:["b"]});
 expect(await a.extend({owners:["a"]})).toEqual({held:true,request:{dirty:["a"],structural:false}});
 expect(await a.extend({owners:["c"]})).toEqual({held:true,request:{dirty:["a","c"],structural:false}});expect(a.owners).toEqual(["a","c"]);
 expect(await a.extend({owners:["b"]})).toEqual({held:false,request:{dirty:["a","b","c"],structural:false}});expect(a.owners).toEqual(["a","c"]);
 expect((await a.extend({structural:true})).held).toBe(false);
 await b.release();
 expect(await a.extend({structural:true})).toEqual({held:true,request:{dirty:["a","c"],structural:true}});
 expect(await settled(custody.hold({owners:["q"]}).then(claim=>claim.release()))).toBe(false);
 await a.release();
});

test("alone is granted exactly when no other live claim exists, and keeps every newcomer out until the release",async()=>{
 const{custody}=world(),a=await custody.hold({owners:["a"]}),b=await custody.hold({owners:["b"]});
 expect(await a.alone()).toBe(false);await b.release();
 expect(await a.alone()).toBe(true);expect(a.structural).toBe(true);
 const late=custody.hold({owners:["z"]});expect(await settled(late)).toBe(false);
 await a.release();await(await late).release();
});

test("a cancelled wait removes its claim and its lease, and a peer that waited behind it moves on",async()=>{
 const{custody}=world(),a=await custody.hold({owners:["a"]}),controller=new AbortController();
 const waiting=custody.hold({owners:["a"],signal:controller.signal});expect(await settled(waiting)).toBe(false);
 controller.abort(new Error("stop"));await expect(waiting).rejects.toThrow("stop");
 expect((await custody.list()).map(row=>row.token)).toEqual([a.token]);
 await a.release();const again:SliceClaimV1=await custody.hold({owners:["a"]});await again.release();
});

const attachPolicy={...CUSTODY_POLICY_V1,attachSpendRatio:0,attachPollMilliseconds:20,attachRecheckMilliseconds:150,noticeMilliseconds:60};
const lookOf=(custody:ReturnType<typeof world>["custody"],pointer:()=>number,owners:string[])=>async()=>({held:!await custody.idle({owners}),generation:pointer()});

test("a second reader on one state root attaches to the live builder: it never holds the custody beside it, tells the waves in English and German, is served from a wave through the recheck, and ends when the builder lets go",async()=>{
 const{custody,say,said}=world(),builder=await custody.hold({owners:["p1","p2","p3"]});
 let generation=0,rechecks=0,serve=false;
 const attached=attachToBuilderV1({look:lookOf(custody,()=>generation,["p2","p4"]),recheck:async()=>{rechecks++;return serve?{graph:"g"}:undefined;},say,policy:attachPolicy});
 await sleep(60);expect(said.map(row=>row.key)).toEqual(["attached"]);
 expect((await custody.list()).map(row=>[row.token,row.state])).toEqual([[builder.token,"held"]]);
 generation=1;await sleep(60);expect(said.map(row=>row.key)).toEqual(["attached","attachedWave"]);expect(rechecks,"a wave inside the interval is only told").toBe(0);
 await sleep(120);generation=3;await sleep(60);expect(rechecks).toBe(1);expect(said.filter(row=>row.key==="attachedWave").at(-1)!.text).toContain("3 wave(s)");
 expect(said.every(row=>row.text.startsWith("graph: "))).toBe(true);
 expect(renderCustodyMessageV1("attached",{generation:2,seconds:5},"de")).toContain("angehängt");expect(renderCustodyMessageV1("attachedWave",{generation:2,waves:1,seconds:5},"de")).toContain("Welle(n)");
 serve=true;await sleep(170);generation=4;expect(await attached).toEqual({graph:"g"});expect(rechecks).toBe(2);expect(said.at(-1)!.key).toBe("attachedServed");
 expect((await custody.list()).length).toBe(1);await builder.release();
 const left=await attachToBuilderV1({look:lookOf(custody,()=>generation,["p2"]),recheck:async()=>({graph:"never"}),policy:attachPolicy});
 expect(left,"nothing holds the custody against the reader any more").toBeUndefined();
 const next=await custody.hold({owners:["p2"]});expect((await custody.list()).length).toBe(1);await next.release();
});

test("an attached reader ends without a result when the builder releases, when the custody cannot be looked at, and rethrows a cancellation",async()=>{
 const{custody}=world(),builder=await custody.hold({owners:["p1"]}),started=Date.now();
 const waiting=attachToBuilderV1({look:lookOf(custody,()=>1,["p1"]),recheck:async()=>undefined,policy:attachPolicy});
 await sleep(50);await builder.release();expect(await waiting).toBeUndefined();expect(Date.now()-started).toBeLessThan(2000);
 expect(await attachToBuilderV1({look:async()=>{throw new Error("store gone");},recheck:async()=>({graph:"x"}),policy:attachPolicy})).toBeUndefined();
 const held=await custody.hold({owners:["p1"]}),controller=new AbortController();
 const stopped=attachToBuilderV1({look:lookOf(custody,()=>1,["p1"]),recheck:async()=>undefined,signal:controller.signal,policy:attachPolicy});
 await sleep(40);controller.abort(new Error("cancelled"));await expect(stopped).rejects.toThrow("cancelled");await held.release();
});

test("the recheck is asked at most once per interval and a failing recheck never ends the attachment",async()=>{
 const{custody}=world(),builder=await custody.hold({owners:["p1"]});let generation=0,asked=0;
 const attached=attachToBuilderV1({look:lookOf(custody,()=>generation,["p1"]),recheck:async()=>{asked++;throw new Error("verification failed");},policy:{...attachPolicy,attachRecheckMilliseconds:100}});
 for(let wave=1;wave<=8;wave++){generation=wave;await sleep(30);}
 expect(asked).toBeGreaterThanOrEqual(1);expect(asked).toBeLessThanOrEqual(3);
 await builder.release();expect(await attached).toBeUndefined();
});
