/** 🧪️ Laws tying the unit cache collector to the cache ledger: its shares fit the ledger budget and its watcher also maintains between the two water marks. */
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import budgetPolicy from "../../../../../💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {cacheAreaBudgetBytesV1,cacheLedgerV1} from "../../../../../💾️storage/🧾️ledger/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {watchUnitCachePressureV1} from "../../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.gc,GIB=1024**3,base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(base,{recursive:true});
const python=(program:string,input:unknown):unknown=>{const run=spawnSync(testPythonV1()??"python3",["-I","-c",program],{input:JSON.stringify(input),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the shares of the unit cache (objects, lanes and memos; the incremental share lies inside the lanes) add up to at most its ledger budget, so enforcing the shares enforces the budget",()=>{
 const ledger=cacheLedgerV1(),area=ledger.areas.find(row=>row.id==="unit-cache")!,budget=cacheAreaBudgetBytesV1(ledger,area)!,shares=[rules.casShare,rules.laneShare,rules.admissionShare];
 expect(shares.every(share=>share>0&&share<1)).toBe(true);
 const independent=python("import json,sys\nv=json.load(sys.stdin)\nprint(json.dumps({'bytes':int(sum(v['shares'])*v['capacity']+0.5),'fits':sum(v['shares'])*v['capacity']<=v['budget']}))",{shares,capacity:budgetPolicy.partitions.rust.capacityBytes,budget}) as {bytes:number;fits:boolean};
 expect(independent.fits,"shares "+shares.join("+")+" of "+budgetPolicy.partitions.rust.capacityBytes+" bytes against a budget of "+budget).toBe(true);
 expect(shares.reduce((sum,share)=>sum+share,0)*budgetPolicy.partitions.rust.capacityBytes).toBeLessThanOrEqual(budget);
 expect(rules.casProtectedShare).toBeLessThanOrEqual(rules.casShare);expect(rules.incrementalShare).toBeLessThanOrEqual(rules.laneShare);
});

test("between the low and the high water mark the watcher maintains once at its start and then every maintenance interval, below the low mark it only watches",async()=>{
 const root=mkdtempSync(join(base,"ucb-")),volume=48*GIB;mkdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.locks),{recursive:true});
 try{
  const run=async(freeBytes:number)=>{
   let time=1_000_000;const started=time,at:number[]=[],collect=(async(_root:string,_capacity:number,options:{nowMs:number})=>{at.push(options.nowMs-started);return{running:false as const,plan:{removeLanes:[],removeIncremental:[],removeObjects:[]},applied:{lanes:0,incremental:0,objects:0,sessions:0,units:0}};}) as never;
   const result=await watchUnitCachePressureV1(root,volume,{now:()=>time,sleep:async(milliseconds:number)=>{time+=milliseconds;},measure:()=>({volumeBytes:volume,freeBytes}),collect});
   return{result,at};
  };
  const band=await run(9*GIB);
  const expected=python("import json,sys\nv=json.load(sys.stdin)\nt=0;last=None;out=[]\nwhile True:\n    if last is None or t-last>=v['interval']:\n        out.append(t);last=t\n    if t>=v['lifetime']: break\n    t+=v['poll']\nprint(json.dumps(out))",{interval:rules.maintenanceIntervalMilliseconds,lifetime:rules.watchMaximumMilliseconds,poll:rules.watchPollMilliseconds}) as number[];
  expect(band.result).toMatchObject({state:"done",reason:"expired",collections:expected.length});
  expect(band.at).toEqual(expected);expect(expected[0]).toBe(0);expect(expected.length).toBeGreaterThan(2);
  expect((await run(30*GIB)).at,"below the low-water mark the periodic maintenance launch is responsible").toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
