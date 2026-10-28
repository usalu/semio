/** 🛤️ Contract: a caller on a compiler lane compiles into the lane's directories through the unit cache chain (the focus is its crate set, a miss runs the compiler itself), a caller that keeps its own target directory switches the cache off, and the pool's byte quota is the lane share of the compiler store's quota in the machine disk budget, which an independent exact-rational computation reproduces.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../../🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {unitCacheShimPathV1} from "../../../../../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {cacheLedgerV1,cacheRootDirectoryV1} from "../../../../🧾️ledger/🟦️.ts";
import machinePolicy from "../../../../🧾️ledger/🎚️machine/🔣️policy.json" with {type:"json"};
import {compilerLaneEnvironmentV1,compilerLaneQuotaBytesV1} from "../🟦️.ts";

const GIB=1024**3,base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const python=requireTestToolV1("python");
const lane={id:"p1a2b3c",base:"/cache/u/l/p1a2b3c",target:"/cache/u/l/p1a2b3c/t",build:"/cache/u/l/p1a2b3c/b"};

test("a lane view names the lane's directories and the unit cache chain with the crate set as focus",()=>{
 const cacheRoot=mkdtempSync(join(base,"lv-"));
 try{
  const environment={RUSTC:"/ingress/rustc.ts",CARGO_TARGET_DIR:"/shared/t",CARGO_BUILD_BUILD_DIR:"/shared/b",RUSTC_WRAPPER:"",PATH:"/bin"},view=compilerLaneEnvironmentV1({environment,cacheRoot,lane,toolchain:"tc-digest",crates:["b","a"],unitCache:true});
  expect(view).toMatchObject({CARGO_TARGET_DIR:lane.target,CARGO_BUILD_BUILD_DIR:lane.build,RUSTC:"/ingress/rustc.ts",PATH:"/bin",RUSTC_WRAPPER:unitCacheShimPathV1(cacheRoot),SEMIO_UNIT_CACHE_ROOT:cacheRoot,SEMIO_UNIT_CACHE_LANE:lane.id,SEMIO_UNIT_CACHE_LANE_ROOT:lane.base,SEMIO_UNIT_CACHE_TOOLCHAIN:"tc-digest",SEMIO_UNIT_CACHE_FOCUS:"b,a",SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"});
  expect(JSON.parse(view.SEMIO_UNIT_CACHE_INNER!)).toEqual(["/ingress/rustc.ts"]);
  expect(Object.isFrozen(view)).toBe(true);
  expect(environment.CARGO_TARGET_DIR).toBe("/shared/t");
 }finally{rmSync(cacheRoot,{recursive:true,force:true});}
});

test("a caller with a private target directory, or a switched-off cache, only gets the lane's directories",()=>{
 const environment={RUSTC:"r",RUSTC_WRAPPER:""};
 for(const input of [{environment,unitCache:false},{environment:{...environment,SEMIO_UNIT_CACHE:"off"},unitCache:true}]){
  const view=compilerLaneEnvironmentV1({...input,cacheRoot:"/cache/u",lane,toolchain:"t",crates:["x"]});
  expect(view.CARGO_TARGET_DIR).toBe(lane.target);expect(view.CARGO_BUILD_BUILD_DIR).toBe(lane.build);
  expect(view.RUSTC_WRAPPER).toBe("");expect(view.SEMIO_UNIT_CACHE_ROOT).toBeUndefined();
 }
});

const exactQuota=(totalBytes:number,capacityBytes:number):number=>{
 const code=`import json,sys\nfrom fractions import Fraction\nd=json.load(sys.stdin)\nstores=d['policy']['stores']\nfloors=sum(s['floorBytes'] for s in stores)\nrust=[s for s in stores if s['id']==d['store']][0]\nweights=sum(s['weightBasisPoints'] for s in stores)\nshare=Fraction(d['total']-floors)*rust['weightBasisPoints']/weights\nquota=rust['floorBytes']+share\nprint(int(min(Fraction(d['capacity']),quota)*Fraction(d['numerator'],d['denominator'])//1))`;
 const run=spawnSync(python,["-I","-c",code],{input:JSON.stringify({policy:machinePolicy,store:UNIT_CACHE_POLICY_V1.pool.machineStore,total:totalBytes,capacity:capacityBytes,numerator:1,denominator:5}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return Number(run.stdout.trim());
};

test("the pool quota is the lane share of the store's quota of the machine budget, from the environment, the machine file and the default",async()=>{
 const host=mkdtempSync(join(base,"lq-"));
 try{
  expect(UNIT_CACHE_POLICY_V1.gc.laneShare).toBe(0.2);
  const capacity=40*GIB;
  expect(await compilerLaneQuotaBytesV1(host,{},capacity)).toBe(exactQuota(machinePolicy.defaultTotalBytes,capacity));
  expect(await compilerLaneQuotaBytesV1(host,{SEMIO_CACHE_BUDGET_BYTES:String(20*GIB)},capacity)).toBe(exactQuota(20*GIB,capacity));
  expect(await compilerLaneQuotaBytesV1(host,{SEMIO_CACHE_BUDGET_BYTES:"100GiB"},30*GIB)).toBe(exactQuota(100*GIB,30*GIB));
  const directory=cacheRootDirectoryV1(host,cacheLedgerV1(),"repository");mkdirSync(directory,{recursive:true});
  writeFileSync(join(directory,machinePolicy.file),JSON.stringify({version:1,totalBytes:"60GiB"}));
  expect(await compilerLaneQuotaBytesV1(host,{},capacity)).toBe(exactQuota(60*GIB,capacity));
  expect(await compilerLaneQuotaBytesV1(host,{SEMIO_CACHE_BUDGET_BYTES:String(20*GIB)},capacity)).toBe(exactQuota(20*GIB,capacity));
  expect(await compilerLaneQuotaBytesV1(host,{SEMIO_CACHE_BUDGET_BYTES:"1KiB"},capacity)).toBe(Math.floor(capacity*0.2));
 }finally{rmSync(host,{recursive:true,force:true});}
});
