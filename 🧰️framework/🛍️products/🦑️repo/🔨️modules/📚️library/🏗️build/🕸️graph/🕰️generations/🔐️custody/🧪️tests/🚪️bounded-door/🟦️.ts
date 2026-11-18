/** 🚪️ Contract: a waiter at the door is told who holds it (pid, command, start, code, phase and owners done of total, time without progress) in its language and again at every notice; a holder that does not advance for the declared time, or runs other code than the library while someone waits, is asked to hand over at its next unit, and one that does not answer within the grace loses the door; progress (phase or owners done) decides, never elapsed time; a holder that finished owners gives them back so a read of exactly those owners is not kept waiting; a structural claim stays exclusive. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync,mkdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../../🧩️claims/🟨️.mjs";
import {CUSTODY_POLICY_V1,createSliceCustodyV1,readCustodyClaimV1,renderCustodyMessageV1} from "../../🟨️.mjs";

const roots:string[]=[],sha="a".repeat(64),other="b".repeat(64);
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const holder=(pid:number,digest=sha)=>({pid,command:"script.ts repo show projects",startedMs:Date.UTC(2026,10,17,16,0,0),digest});
const world=()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"graph-door-")));roots.push(root);const clock={t:1_000_000},said:{key:string;fields:Record<string,string|number>}[]=[];
 return{root,clock,said,say:(key:string,fields:Record<string,string|number>)=>{said.push({key,fields});},directory:join(root,"generations",CUSTODY_POLICY_V1.directory),custody:createSliceCustodyV1({root:join(root,"generations"),fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(root,"leases")),now:()=>clock.t})};
};
const until=async(check:()=>boolean|Promise<boolean>,ms=6000)=>{const end=Date.now()+ms;while(Date.now()<end){if(await check())return true;await sleep(50);}return false;};
const settled=async(promise:Promise<unknown>,ms=1500)=>Promise.race([promise.then(()=>true),sleep(ms).then(()=>false)]);

test("a waiter is told who holds the door and how far it is, in its language, and told again at the next notice with the new counters",async()=>{
 const{custody,clock,said,say}=world(),a=await custody.hold({structural:true,holder:holder(4711)});
 await a.progress({phase:"owners",done:120,total:482});
 const b=custody.hold({owners:["p1"],say,language:"en",holder:holder(4712)}),c=custody.hold({owners:["p9"],say:(key,fields)=>said.push({key:"de:"+key,fields}),language:"de",holder:holder(4713)});
 expect(await until(()=>said.some(row=>row.key==="blockedAll")&&said.some(row=>row.key==="de:blockedAll"))).toBe(true);
 const english=renderCustodyMessageV1("blockedAll",said.find(row=>row.key==="blockedAll")!.fields as never,"en"),german=renderCustodyMessageV1("blockedAll",said.find(row=>row.key==="de:blockedAll")!.fields as never,"de");
 expect(english).toContain("holder: pid 4711 (script.ts repo show projects)");expect(english).toContain("120 of 482");expect(english).toContain("aaaaaaaaaaaa");
 expect(german).toContain("Halter: pid 4711");expect(german).toContain("120 von 482");
 const before=said.length;clock.t+=CUSTODY_POLICY_V1.noticeMilliseconds+1000;await a.progress({phase:"owners",done:130,total:482});
 expect(await until(()=>said.length>before+1)).toBe(true);
 expect(renderCustodyMessageV1("blockedAll",said.filter(row=>row.key==="blockedAll").at(-1)!.fields as never,"en")).toContain("130 of 482");
 await a.release();await (await b).release();await (await c).release();
});

test("a holder that does not advance for the declared time is asked once and loses the door when it does not answer within the grace; its release afterwards is harmless",async()=>{
 const{custody,clock,said,say,directory}=world(),a=await custody.hold({structural:true,holder:holder(4711)});
 await a.progress({phase:"owners",done:5,total:10});
 const b=custody.hold({structural:true,say,holder:holder(4712)});
 await sleep(1300);expect(said.some(row=>row.key==="askedStalled")).toBe(false);expect(await a.handover()).toBeUndefined();
 clock.t+=CUSTODY_POLICY_V1.stallMilliseconds+1;
 expect(await until(()=>said.some(row=>row.key==="askedStalled"))).toBe(true);
 expect(said.filter(row=>row.key==="askedStalled").length).toBe(1);expect(said.find(row=>row.key==="askedStalled")!.fields.pid).toBe(4711);
 expect(await a.handover()).toMatchObject({reason:"stalled"});expect(await settled(b,300)).toBe(false);
 clock.t+=CUSTODY_POLICY_V1.askGraceMilliseconds+1;
 expect(await settled(b,4000)).toBe(true);expect(said.some(row=>row.key==="deposed")).toBe(true);
 const held=await b;expect((await a.extend({owners:["p1"]})).held).toBe(false);
 await a.release();expect(readdirSync(directory).filter(name=>name.startsWith(a.token))).toEqual([]);
 await held.release();
});

test("progress, not elapsed time, decides: a holder that finishes an owner every few minutes is never asked however long it runs",async()=>{
 const{custody,clock,said,say}=world(),a=await custody.hold({structural:true,holder:holder(4711)});
 const b=custody.hold({structural:true,say,holder:holder(4712)});
 for(let step=1;step<=4;step++){clock.t+=CUSTODY_POLICY_V1.stallMilliseconds/2;await a.progress({phase:"owners",done:step,total:10});await sleep(1200);}
 expect(said.some(row=>row.key==="askedStalled")).toBe(false);expect(await a.handover()).toBeUndefined();
 clock.t+=CUSTODY_POLICY_V1.stallMilliseconds+1;
 expect(await until(()=>said.some(row=>row.key==="askedStalled"))).toBe(true);
 await a.release();await (await b).release();
});

test("a holder on other code than the library is asked as soon as someone waits, and hands over on its own check; without a waiter it runs on",async()=>{
 const{custody,said,say}=world(),a=await custody.hold({structural:true,holder:holder(4711,sha),currentDigest:async()=>other});
 expect(await a.handover()).toBeUndefined();
 const b=custody.hold({structural:true,say,holder:holder(4712,other),currentDigest:async()=>other});
 expect(await until(()=>said.some(row=>row.key==="askedStale"))).toBe(true);
 const text=renderCustodyMessageV1("askedStale",said.find(row=>row.key==="askedStale")!.fields as never,"en");expect(text).toContain("pid 4711");expect(text).toContain("aaaaaaaaaaaa");expect(text).toContain("bbbbbbbbbbbb");
 expect(await a.handover()).toMatchObject({reason:"stale-code"});
 await a.release();await (await b).release();
 const c=await custody.hold({structural:true,holder:holder(4713,sha),currentDigest:async()=>other}),d=custody.hold({structural:true,holder:holder(4714,other)});
 expect(await until(async()=>(await c.handover())?.reason==="stale-code")).toBe(true);
 await c.release();await (await d).release();
});

test("a hand-over error restarts the command on the current code and says what is kept, in both languages",async()=>{
 const{custody}=world(),a=await custody.hold({structural:true,holder:holder(4711)});
 const error=a.handoverError({reason:"stalled"},{done:120,total:482}) as Error&{code:string;handover:{reason:string}};
 expect(error.code).toBe("SEMIO_STALE_IMPLEMENTATION");expect(error.handover.reason).toBe("stalled");
 expect(error.message).toContain("120 of 482");expect(error.message).toContain("120 von 482");
 const stale=a.handoverError({reason:"stale-code",current:other},{done:3,total:9});expect(stale.message).toContain("bbbbbbbbbbbb");
 await a.release();
});

test("a holder that finished owners gives them back: a read of exactly those owners holds at once while the rest stays claimed; a structural claim stays exclusive",async()=>{
 const{custody}=world(),a=await custody.hold({owners:["p1","p2","p3"],holder:holder(4711)});
 const b=custody.hold({owners:["p2"],holder:holder(4712)}),c=custody.hold({owners:["p3","p9"],holder:holder(4713)});
 expect(await settled(b,300)).toBe(false);
 expect(await a.settle(["p2"])).toBe(true);expect(a.owners).toEqual(["p1","p3"]);
 expect(await settled(b,4000)).toBe(true);expect(await settled(c,300)).toBe(false);
 await (await b).release();await a.release();await (await c).release();
 const s=await custody.hold({structural:true,holder:holder(4714)}),d=custody.hold({owners:["p2"],holder:holder(4715)});
 expect(await s.settle(["p2"])).toBe(false);expect(s.structural).toBe(true);expect(await settled(d,1300)).toBe(false);
 await s.release();await (await d).release();
});

test("reads of disjoint owner sets hold at the same time and a structural claim waits for every one of them",async()=>{
 const{custody}=world(),a=await custody.hold({owners:["p1"]}),b=await custody.hold({owners:["p2"]});
 const s=custody.hold({structural:true});expect(await settled(s,300)).toBe(false);
 await a.release();expect(await settled(s,1500)).toBe(false);await b.release();expect(await settled(s,4000)).toBe(true);await (await s).release();
});

test("sidecars of a dead claim are removed with it and the claim document stays closed",async()=>{
 const{custody,directory}=world(),token="c".repeat(24);mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,token+".json"),JSON.stringify({version:1,token,ticket:1,owners:[],structural:true,state:"held",queuedMs:1,holder:holder(1)}));
 writeFileSync(join(directory,token+".progress.json"),JSON.stringify({phase:"owners",done:1,total:2,advancedMs:1}));writeFileSync(join(directory,token+".ask.json"),JSON.stringify({version:1,reason:"stalled",atMs:1,by:{pid:2,digest:""}}));
 expect(await custody.list()).toEqual([]);expect(readdirSync(directory)).toEqual([]);
 expect(()=>readCustodyClaimV1({version:1,token,ticket:1,owners:[],structural:false,state:"held",queuedMs:1,holder:{...holder(1),pid:0}})).toThrow();
 expect(readCustodyClaimV1({version:1,token,ticket:1,owners:[],structural:false,state:"held",queuedMs:1,holder:holder(1)}).holder).toEqual(holder(1));
 expect(existsSync(join(directory,token+".ask.json"))).toBe(false);
});
