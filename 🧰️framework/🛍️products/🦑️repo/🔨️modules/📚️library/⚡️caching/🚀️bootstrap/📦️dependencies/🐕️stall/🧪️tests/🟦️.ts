/** 🐕️ Tests for the installer stall rule. */
import {expect,test} from "bun:test";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {installerHttpFailureV1,installerProgressStampV1,installerStallDecisionV1,releaseInstallerBindingsV1,watchInstallerStallV1} from "../🟦️.ts";

const cases=[
 {id:"progress-keeps-alive",events:[{at:0,changed:true},{at:100000,changed:true},{at:250000,changed:true},{at:400000,changed:true}],stalledAtMilliseconds:null},
 {id:"silent-for-the-bound",events:[{at:0,changed:true},{at:100000,changed:false},{at:200000,changed:false},{at:300000,changed:false}],stalledAtMilliseconds:300000},
 {id:"change-resets-the-clock",events:[{at:0,changed:true},{at:200000,changed:false},{at:250000,changed:true},{at:540000,changed:false}],stalledAtMilliseconds:null},
 {id:"never-changing-from-start",events:[{at:0,changed:false},{at:299999,changed:false},{at:300000,changed:false}],stalledAtMilliseconds:300000},
];
const oracle=`import json,sys
rows=json.loads(sys.argv[1]);out={}
for r in rows:
    last=r["events"][0]["at"] if r["events"] else 0;res=None
    for e in r["events"]:
        if e["changed"]: last=e["at"]
        elif e["at"]-last>=300000:
            res=e["at"];break
    out[r["id"]]=res
print(json.dumps(out))`;
test("the stall decision equals an independent Python model on every case",()=>{
 for(const row of cases)expect(validateJsonSchemaSubset(schema.definitions.Case,row,schema),row.id).toEqual([]);
 const run=spawnSync("python3",["-I","-c",oracle,JSON.stringify(cases)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const independent=JSON.parse(run.stdout);
 for(const row of cases){expect(installerStallDecisionV1(row.events),row.id).toBe(row.stalledAtMilliseconds);expect(independent[row.id],row.id).toBe(row.stalledAtMilliseconds);}
});
test("the watcher kills a silent child once and leaves a progressing one alone",async()=>{
 let clock=0,stampValue=1,kills=0;
 const stalled=watchInstallerStallV1({probes:["x"],kill:()=>{kills++;},stallMilliseconds:50,pollMilliseconds:5,now:()=>clock,stamp:()=>stampValue});
 clock=40;await Bun.sleep(20);stampValue=2;await Bun.sleep(20);clock=80;await Bun.sleep(20);expect(kills).toBe(0);expect(stalled.stalled()).toBe(false);
 clock=200;await Bun.sleep(30);expect(kills).toBe(1);expect(stalled.stalled()).toBe(true);await Bun.sleep(20);expect(kills).toBe(1);stalled.stop();
});
test("the progress stamp follows real file changes and ignores missing paths",()=>{
 const root=mkdtempSync(join(tmpdir(),"stall-"));
 try{
  writeFileSync(join(root,"a"),"1");utimesSync(join(root,"a"),new Date(1000),new Date(1000));
  expect(installerProgressStampV1([join(root,"a"),join(root,"missing")])).toBe(1000);
  utimesSync(join(root,"a"),new Date(5000),new Date(5000));expect(installerProgressStampV1([join(root,"a")])).toBe(5000);
  expect(installerProgressStampV1([join(root,"missing")])).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});
const bun=process.execPath;
test("released bindings let a real Bun rerun finish instead of livelocking in mkdirat",()=>{
 const root=mkdtempSync(join(tmpdir(),"stall-bun-"));
 try{
  mkdirSync(join(root,"w","0"),{recursive:true});mkdirSync(join(root,"w","1"),{recursive:true});mkdirSync(join(root,"real"),{recursive:true});
  writeFileSync(join(root,"package.json"),JSON.stringify({name:"root",private:true,workspaces:["w/0","w/1"]}));
  writeFileSync(join(root,"w","0","package.json"),JSON.stringify({name:"@t/zero",version:"1.0.0",dependencies:{"@t/one":"workspace:*"}}));
  writeFileSync(join(root,"w","1","package.json"),JSON.stringify({name:"@t/one",version:"1.0.0"}));
  writeFileSync(join(root,"real","marker"),"x");
  const install=(timeout:number)=>spawnSync(bun,["install","--ignore-scripts","--linker=hoisted","--backend=copyfile","--no-save"],{cwd:root,encoding:"utf8",timeout,env:{...process.env,BUN_INSTALL_CACHE_DIR:join(root,".cache")}});
  expect(install(60000).status).toBe(0);
  expect(lstatSync(join(root,"node_modules","@t","one")).isSymbolicLink()).toBe(true);
  rmSync(join(root,"node_modules","@t","one"));symlinkSync(join(root,"real"),join(root,"node_modules","@t","one"));
  expect(releaseInstallerBindingsV1(root)).toBeGreaterThanOrEqual(1);
  expect(existsSync(join(root,"node_modules","@t","one"))).toBe(false);
  expect(existsSync(join(root,"real","marker"))).toBe(true);
  const rerun=install(60000);expect(rerun.status,String(rerun.stderr)).toBe(0);
  expect(lstatSync(join(root,"node_modules","@t","one")).isSymbolicLink()).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},180000);

const failures=[
 {id:"real bun 502 on a tarball",output:"bun install v1.4.2\nResolving dependencies\nResolved, downloaded and extracted [8]\nerror: GET http://192.168.178.70:6090/mirror/npm/react/-/react-18.3.1.tgz - 502\n",expected:"GET http://192.168.178.70:6090/mirror/npm/react/-/react-18.3.1.tgz - 502"},
 {id:"the last failure wins",output:"error: GET http://a.test/x - 503\nerror: GET http://a.test/y - 502\nerror: y@1.0.0 failed to resolve\n",expected:"GET http://a.test/y - 502"},
 {id:"credentials and query are dropped",output:"error: GET https://user:secret@registry.test/p?token=abc - 401",expected:"GET https://registry.test/p - 401"},
 {id:"no http failure",output:"error: lockfile had changes, but lockfile is frozen",expected:null},
 {id:"success codes are not failures",output:"GET http://a.test/x - 200",expected:null},
];
const failureOracle=`import json,re,sys
pattern=re.compile(sys.argv[2]);out={}
for row in json.loads(sys.argv[1]):
    found=None
    for m in pattern.finditer(row["output"]):
        found=m
    if found:
        url=re.sub(r"^(https?://)[^/@]*@",r"\\1",found.group(2))
        out[row["id"]]=found.group(1)+" "+url+" - "+found.group(3)
    else: out[row["id"]]=None
print(json.dumps(out))`;
test("the real HTTP error behind a stalled installer is named, as an independent Python reading names it",()=>{
 const run=spawnSync("python3",["-I","-c",failureOracle,JSON.stringify(failures),schema.definitions.Protocol.const.httpFailure],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const independent=JSON.parse(run.stdout);
 for(const row of failures){expect(installerHttpFailureV1(row.output)??null,row.id).toBe(row.expected);expect(independent[row.id],row.id+" (python)").toBe(row.expected);}
});
