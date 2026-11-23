/** 🏷️ Contract: a pass receipt is named by the run that observes it. A pooled lane (and so its build directory) is reused by later runs of the same pass, and the receipt records when and what that run built, so a name that ignored the run met an earlier run's bytes in the shared ledger and retention refused the lane. The oracle is the shared ledger after two runs of one pass in one reused lane, read back with an independent SHA-256 (`hashlib`).
 * @see ../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,join,resolve} from "node:path";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {nativeBuildPassObservationPathV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {retainLaneOutputsV1} from "../../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.retention,base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),python=requireTestToolV1("python");
mkdirSync(base,{recursive:true});
const pass={manifest:"/ws/Cargo.toml"},args=["build","--locked","--target","wasm32-wasip2"];
const sha=(path:string):string=>{const run=spawnSync(python,["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",path],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return run.stdout.trim();};
const oldName=(build:string):string=>"cargo-unit-provenance-release-"+createHash("sha256").update(JSON.stringify([pass.manifest,args,build])).digest("hex").slice(0,32)+".json";
const run=(root:string,lane:string,id:string,name:string|null,builtAtMs:number)=>{
 const generation={report:join(root,"runs",id,"r"),build:lane},file=name??basename(nativeBuildPassObservationPathV1(generation,pass,args)),ledger=join(lane,rules.ledger);
 mkdirSync(ledger,{recursive:true});writeFileSync(join(ledger,file),JSON.stringify({builtAtMs,pass:pass.manifest})+"\n");return{file,generation};
};

test("the same pass built twice in one reused lane keeps the receipt name of the first run only under the old naming, and both runs publish under the run-owned naming",()=>{
 const root=mkdtempSync(join(base,"on-")),lane=join(root,"u","l","ab12"),shared=join(root,"b","p"),sharedLedger=join(shared,rules.ledger);
 try{
  const first=run(root,lane,"w0CWEcg",oldName(lane),1000);
  expect(retainLaneOutputsV1({lane,shared,laneId:"ab12",language:"en"})).toMatchObject({published:1,conflicts:[]});
  const second=run(root,lane,"w1XyZab",oldName(lane),2000);
  expect(second.file).toBe(first.file);
  expect(retainLaneOutputsV1({lane,shared,laneId:"ab12",language:"en"}).conflicts,"the old name is the conflict this contract exists for").toEqual([first.file]);
  rmSync(join(lane,rules.ledger),{recursive:true,force:true});rmSync(sharedLedger,{recursive:true,force:true});
  const one=run(root,lane,"w0CWEcg",null,1000),two=run(root,lane,"w1XyZab",null,2000);
  expect(two.file).not.toBe(one.file);
  expect(one.file).toMatch(/^cargo-unit-provenance-release-[0-9a-f]{32}\.json$/u);
  const bytes=readdirSync(join(lane,rules.ledger)).map(name=>[name,readFileSync(join(lane,rules.ledger,name))] as const);
  expect(retainLaneOutputsV1({lane,shared,laneId:"ab12",language:"en"})).toMatchObject({published:2,conflicts:[],refused:0});
  for(const [name,content] of bytes){expect(readFileSync(join(sharedLedger,name)).equals(content)).toBe(true);expect(sha(join(sharedLedger,name))).toBe(createHash("sha256").update(content).digest("hex"));}
  expect(readdirSync(sharedLedger).sort()).toEqual([one.file,two.file].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("one run names the receipt of one pass the same way every time, and the name covers the manifest, the arguments, the lane and the run",()=>{
 const generation={report:"/r/a/r",build:"/b/lane"},name=(over:Partial<typeof generation>,manifest=pass.manifest,arguments_=args)=>nativeBuildPassObservationPathV1({...generation,...over},{manifest},arguments_);
 expect(name({})).toBe(name({}));
 for(const other of [name({report:"/r/b/r"}),name({build:"/b/other"}),name({},"/ws/Other.toml"),name({},pass.manifest,[...args,"--release"])])expect(other).not.toBe(name({}));
});
