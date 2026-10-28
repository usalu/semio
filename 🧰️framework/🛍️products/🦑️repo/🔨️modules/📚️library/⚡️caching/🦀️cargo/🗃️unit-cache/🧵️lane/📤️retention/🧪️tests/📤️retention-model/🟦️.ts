/** 📤️ Contract: a lane compile loses no provenance when its lane is released, and a recorded compiler origin is judged by what it IS (the store's shared directories or one of its own lanes), not by string equality with the shared directory. The two pure rules (`planRetentionV1`, `compilerOriginOwnedV1`) are written twice, in the production module and in the independent Python oracle `./🐍️.py`, and both agree with the language-agnostic corpus `./🔣️.json` and with a third-party-library check (`lodash`) on 500 seeded cases.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {differenceBy,intersectionBy,sortBy} from "lodash";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {compilerOriginOwnedV1,laneBuildDirectoryOwnedV1,planRetentionV1,type RetainedEntryV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const python=requireTestToolV1("python"),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const ask=(input:unknown)=>{const run=spawnSync(python,["-I",oracle,"verdicts","/dev/stdin"],{input:JSON.stringify(input),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {plan:unknown[];origin:boolean[];lane:boolean[]};};
const posix=process.platform!=="win32";
const rules=UNIT_CACHE_POLICY_V1.retention;

test("the corpus is closed by its schema and speaks the authored policy",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(corpus.policy).toEqual({ledgerPattern:rules.ledgerPattern,maximumFiles:rules.maximumFiles,maximumFileBytes:rules.maximumFileBytes,layout:{lanes:UNIT_CACHE_POLICY_V1.layout.lanes,build:UNIT_CACHE_POLICY_V1.layout.build,target:UNIT_CACHE_POLICY_V1.layout.target}});
 for(const mutate of [(d:any)=>{d.extra=1;},(d:any)=>{d.version=2;},(d:any)=>{d.plan[0].kind="ledger";},(d:any)=>{d.plan[0].lane[0].sha256="xyz";},(d:any)=>{d.plan[0].expect[0].action="delete";},(d:any)=>{d.origin[0].origin.buildDirectory="relative/b";},(d:any)=>{delete d.policy.layout.lanes;},(d:any)=>{d.origin[0].expect="yes";}]){
  const copy=JSON.parse(JSON.stringify(corpus));mutate(copy);expect(ajv.validate(corpusSchema,copy),JSON.stringify(copy).slice(0,80)).toBe(false);
 }
});

test("production and the Python oracle reach the authored verdict of every corpus case",()=>{
 const verdicts=ask(corpus);
 for(const [index,item] of corpus.plan.entries()){
  expect(planRetentionV1(item.lane,item.shared),item.id+" production").toEqual(item.expect as never);
  expect(verdicts.plan[index],item.id+" oracle").toEqual(item.expect);
 }
 if(!posix)return;
 for(const [index,item] of corpus.origin.entries()){
  expect(compilerOriginOwnedV1({cacheRoot:item.cacheRoot,shared:item.shared,origin:item.origin}),item.id+" production").toBe(item.expect);
  expect(verdicts.origin[index],item.id+" oracle").toBe(item.expect);
  expect(laneBuildDirectoryOwnedV1(item.cacheRoot,item.origin.buildDirectory),item.id+" lane production").toBe(item.lane);
  expect(verdicts.lane[index],item.id+" lane oracle").toBe(item.lane);
 }
});

test("a lane over the file bound is cut at the bound in name order and every kept name is published",()=>{
 const lane:RetainedEntryV1[]=Array.from({length:rules.maximumFiles+3},(_,index)=>({name:"cargo-unit-provenance-"+String(rules.maximumFiles+2-index).padStart(5,"0")+".json",sha256:"a".repeat(64),bytes:1}));
 const plan=planRetentionV1(lane,[]),expected=sortBy(lane,"name").slice(0,rules.maximumFiles).map(row=>row.name);
 expect(plan.map(row=>row.name)).toEqual(expected);
 expect(plan.every(row=>row.action==="publish")).toBe(true);
 expect((ask({version:1,policy:corpus.policy,plan:[{id:"big",lane,shared:[],expect:[]}],origin:[]}).plan[0] as {name:string}[]).map(row=>row.name)).toEqual(expected);
});

const random=(seed:number)=>{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};};

test("on 500 seeded cases production equals the oracle, equals a lodash set computation, and planning twice publishes nothing",()=>{
 const next=random(20261026),names=["cargo-unit-provenance-a.json","cargo-unit-provenance-b.json","cargo-unit-provenance-c.json","cargo-unit-provenance-d.json","cargo-unit-provenance-e.json","x.tmp",".hidden","cargo-unit-provenance-.json","cargo-unit-provenance-é.json"];
 const digest=()=>["a","b","c"][Math.floor(next()*3)]!.repeat(64),pick=():RetainedEntryV1[]=>names.filter(()=>next()<0.5).map(name=>({name,sha256:digest(),bytes:next()<0.1?rules.maximumFileBytes+1:Math.floor(next()*1000)}));
 const cases=Array.from({length:500},(_,index)=>({id:"seed "+index,lane:pick(),shared:pick(),expect:[] as unknown[]}));
 const verdicts=ask({version:1,policy:corpus.policy,plan:cases,origin:[]});
 const pattern=new RegExp(rules.ledgerPattern,"u");
 for(const [index,item] of cases.entries()){
  const plan=planRetentionV1(item.lane,item.shared);
  expect(plan).toEqual(verdicts.plan[index] as never);
  const eligible=item.lane.filter(row=>pattern.test(row.name)),ok=eligible.filter(row=>row.bytes<=rules.maximumFileBytes);
  const fresh=differenceBy(ok,item.shared,"name").map(row=>row.name),known=intersectionBy(ok,item.shared,"name");
  expect(plan.filter(row=>row.action==="publish").map(row=>row.name)).toEqual(sortBy(fresh));
  expect(plan.filter(row=>row.action==="refuse").map(row=>row.name)).toEqual(sortBy(eligible.filter(row=>row.bytes>rules.maximumFileBytes).map(row=>row.name)));
  const sameDigest=known.filter(row=>item.shared.find(other=>other.name===row.name)!.sha256===row.sha256).map(row=>row.name);
  expect(plan.filter(row=>row.action==="same").map(row=>row.name)).toEqual(sortBy(sameDigest));
  expect(plan.filter(row=>row.action==="conflict").map(row=>row.name)).toEqual(sortBy(known.map(row=>row.name).filter(name=>!sameDigest.includes(name))));
  const published=new Set(plan.filter(row=>row.action==="publish").map(row=>row.name)),laneAfter=item.lane.filter(row=>!published.has(row.name)),sharedAfter=[...item.shared,...item.lane.filter(row=>published.has(row.name))];
  expect(planRetentionV1(laneAfter,sharedAfter).filter(row=>row.action==="publish")).toEqual([]);
 }
});

test("on 400 seeded origins production equals the oracle and the lane identifier alone decides",()=>{
 if(!posix)return;
 const next=random(7),ids=["ab12","a","abcdefghijkl","abcdefghijklm","AB12","a-b","", ".."],tails=["t","b","t/x","b/t/0123456789abcdef","b/../t","../t","c"];
 const cases=Array.from({length:400},(_,index)=>{
  const id=ids[Math.floor(next()*ids.length)]!,build=["/s/u/l/"+id+"/b","/s/u/l/"+id+"/bx","/s/u/lx/"+id+"/b","/s/m/b/prototype"][Math.floor(next()*4)]!,target=build.replace(/\/b(x)?$/u,"")+"/"+tails[Math.floor(next()*tails.length)]!;
  return{id:"seed "+index,cacheRoot:"/s/u",shared:{build:"/s/m/b/prototype",target:"/s/m/t/prototype"},origin:{buildDirectory:build,targetDirectory:next()<0.1?"/s/m/t/prototype":target},expect:false};
 });
 const verdicts=ask({version:1,policy:corpus.policy,plan:[],origin:cases});
 for(const [index,item] of cases.entries()){
  const production=compilerOriginOwnedV1({cacheRoot:item.cacheRoot,shared:item.shared,origin:item.origin});
  expect(production,JSON.stringify(item.origin)).toBe(verdicts.origin[index]!);
 }
});
