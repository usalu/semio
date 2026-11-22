import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {join} from "node:path";
import {classifySlotsV1,isSlotIdV1,requireSlotShapeV1,slotIdV1,slotTextV1,type SlotRowV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle","🐍️.py");
const ask=(requests:unknown[])=>{const run=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString()) as unknown[];};
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

test.skipIf(!python)("slot ids equal an independent Python SHA-256 of the canonical array, unicode included",()=>{
 const cases:[("rust-unit"|"nx-task"),string[]][]=[["rust-unit",["rustc 1.93.0","aarch64-apple-darwin","dev","serde 1.0.0","serde","lib","default,std"]],["nx-task",["@semio-tech/framework","build","","darwin-arm64"]],["nx-task",["ünï","cödé","\"q\"",""]],["rust-unit",[]]];
 const model=ask(cases.map(([domain,parts])=>({op:"id",domain,parts})));
 cases.forEach(([domain,parts],index)=>{expect(slotIdV1(domain,parts)).toBe(model[index] as string);expect(isSlotIdV1(model[index])).toBe(true);});
 expect(slotIdV1("nx-task",["a","b"]),"domain and order matter").not.toBe(slotIdV1("rust-unit",["a","b"]));
 expect(slotIdV1("nx-task",["a","b"])).not.toBe(slotIdV1("nx-task",["b","a"]));
 expect(slotIdV1("nx-task",["ab",""]),"parts are delimited").not.toBe(slotIdV1("nx-task",["a","b"]));
});

test.skipIf(!python)("classification equals an independent Python model on 400 seeded caches: ties, reverts, grace edges, unslotted rows",()=>{
 const next=seeded(20261113),requests:{op:string;rows:SlotRowV1[];now:number;grace:number}[]=[];
 for(let round=0;round<400;round++){
  const count=Math.floor(next()*30),slots=1+Math.floor(next()*5),rows:SlotRowV1[]=[];
  for(let index=0;index<count;index++)rows.push({key:`k${String(index).padStart(3,"0")}`,slot:next()<0.15?null:`s${Math.floor(next()*slots)}`.padEnd(64,"0"),storedAt:1000+Math.floor(next()*(round%3===0?4:200)),accessed:Math.floor(next()*500),size:1+Math.floor(next()*100)});
  requests.push({op:"classify",rows,now:1000+Math.floor(next()*300),grace:round%5===0?0:Math.floor(next()*120)});
 }
 const model=ask(requests) as Record<string,[string,number|null,number|null]>[];let states=new Set<string>();
 requests.forEach((request,index)=>{
  const ours=classifySlotsV1(request.rows,{now:request.now,graceMilliseconds:request.grace}),expected=model[index]!;
  expect(ours.size,`round ${index}`).toBe(Object.keys(expected).length);
  for(const[key,[state,superseded,reclaim]]of Object.entries(expected)){const verdict=ours.get(key)!;states.add(state);expect(verdict,`round ${index} ${key}`).toEqual({state,supersededAt:superseded,reclaimableAt:reclaim});}
  const bySlot=new Map<string,number>();for(const row of request.rows)if(row.slot!==null&&ours.get(row.key)!.state==="current")bySlot.set(row.slot,(bySlot.get(row.slot)??0)+1);
  for(const row of request.rows)if(row.slot!==null)expect(bySlot.get(row.slot),"exactly one current key per slot").toBe(1);
 });
 expect([...states].sort()).toEqual(["current","expired","retained","unslotted"]);
});

test("a revert makes the older key current again and the newer one superseded; reads change nothing",()=>{
 const row=(key:string,storedAt:number,accessed=0):SlotRowV1=>({key,slot:"a".repeat(64),storedAt,accessed,size:1});
 const before=classifySlotsV1([row("old",100),row("new",200)],{now:250,graceMilliseconds:100});
 expect(before.get("new")!.state).toBe("current");expect(before.get("old")).toEqual({state:"retained",supersededAt:200,reclaimableAt:300});
 expect(classifySlotsV1([row("old",100,9999),row("new",200)],{now:250,graceMilliseconds:100}).get("old")!.state,"a read of a superseded entry does not resurrect it").toBe("retained");
 expect(classifySlotsV1([row("old",100),row("new",200)],{now:301,graceMilliseconds:100}).get("old")!.state).toBe("expired");
 const reverted=classifySlotsV1([row("old",400),row("new",200)],{now:450,graceMilliseconds:100});
 expect(reverted.get("old")!.state,"storing or claiming the old key again makes it current").toBe("current");expect(reverted.get("new")).toEqual({state:"retained",supersededAt:400,reclaimableAt:500});
 expect(classifySlotsV1([row("a",100),row("b",100)],{now:1000,graceMilliseconds:0}).get("a")!.state,"ties go to the smaller key").toBe("current");
 const young=classifySlotsV1([row("old",290),row("new",300)],{now:310,graceMilliseconds:100});expect(young.get("old")!.state,"stored within the grace is never reclaimed").toBe("retained");
});

test("documents are checked against the schema and texts exist in both languages",()=>{
 const slot="b".repeat(64);
 requireSlotShapeV1("Meta",{version:1,slot,storedAt:1,refs:[slot]},"meta");
 requireSlotShapeV1("Claim",{entries:[{kind:"nx",key:"abc",slot}]},"claim");
 requireSlotShapeV1("Retention",{graceSeconds:600},"retention");
 requireSlotShapeV1("AuditReport",{version:1,cache:"x",current:0,grace:0,stray:[],unslotted:0},"audit");
 const declaration={version:1,kind:"rust-unit",toolchain:slot,sources:[{path:"a/lib.rs",sha256:slot}],externs:[slot]};
 requireSlotShapeV1("Declaration",declaration,"declaration");requireSlotShapeV1("Meta",{version:1,slot,storedAt:1,refs:[],declaration},"meta");
 requireSlotShapeV1("Staleness",{mode:"dry-run",recheckSeconds:3,manifestNames:["unit.json"],generatedRoots:[".cache"],examples:8},"staleness");
 requireSlotShapeV1("StalenessReport",{version:1,mode:"dry-run",sweptAt:1,entries:2,declared:1,live:1,grace:0,stale:{edited:1},unreached:0,retired:0,refused:{},examples:["edited k (a/lib.rs)"]},"report");
 expect(()=>requireSlotShapeV1("Declaration",{...declaration,kind:"other"},"declaration")).toThrow();
 expect(()=>requireSlotShapeV1("Declaration",{...declaration,sources:[{path:"a",sha256:"nope"}]},"declaration")).toThrow();
 expect(()=>requireSlotShapeV1("Staleness",{mode:"sometimes",recheckSeconds:3,manifestNames:[],generatedRoots:[],examples:1},"staleness")).toThrow();
 expect(()=>requireSlotShapeV1("Meta",{version:1,slot:"nope",storedAt:1,refs:[]},"meta")).toThrow();
 expect(()=>requireSlotShapeV1("Claim",{entries:[{kind:"cas",key:"abc",slot}]},"claim")).toThrow();
 expect(slotTextV1("audit.title","en",{cache:"x",current:1,grace:2,stray:3,unslotted:4})).toContain("3 stray");
 expect(slotTextV1("audit.title","de",{cache:"x",current:1,grace:2,stray:3,unslotted:4})).toContain("3 verwaist");
 expect(slotTextV1("staleness.title","en",{mode:"dry-run",declared:5,live:3,grace:1,stale:2,unreached:1,retired:0})).toContain("2 stale");
 expect(slotTextV1("staleness.title","de",{mode:"dry-run",declared:5,live:3,grace:1,stale:2,unreached:1,retired:0})).toContain("2 veraltet");
});
