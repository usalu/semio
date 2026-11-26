import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{dependencyContainerChangeV1,dependencyGenerationNamesV1,dependencyNameKindV1,settleDependencyForeignNamesV1}from"../🟦️.ts";
import install from"../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json"with{type:"json"};
import{requireTestToolV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";

/** 🧹️ Contract: the roster of the generation folder stays exact by itself: OS metadata is removed, a foreign name is quarantined after the grace, the quarantine is bounded and collected, generations, capsules, tombstones and links are never touched or followed. */
const policy=install.$defs.SelectedPlanProtocol.const.foreignNames,NOW=1_800_000_000_000,GEN="0123456789abcdef0123",CAPSULE=".pdeadbeef";
const temp=():string=>mkdtempSync(join(realpath(),"foreign-"));
const realpath=():string=>tmpdir();
const age=(path:string,ms:number)=>{const at=(NOW-ms)/1000;utimesSync(path,at,at);};

test("the declared policy classifies every name, equal to an independent Python re implementation",()=>{
 const names=[GEN,CAPSULE,".x"+GEN,".x"+CAPSULE,".q",".DS_Store","._"+GEN,"Thumbs.db","desktop.ini",".Spotlight-V100",".fseventsd",".Trashes","stray","0123456789abcdef012","0123456789abcdef01234",".pdeadbee",".X"+GEN,".q2"];
 const run=spawnSync(requireTestToolV1("python"),["-c",`import json,re,sys
p=json.loads(sys.argv[1]);names=json.loads(sys.argv[2])
def kind(n):
    if re.search(p["generation"],n):return "generation"
    if re.search(p["tombstone"],n):return "tombstone"
    if n==p["quarantine"]:return "quarantine"
    return "ignorable" if any(re.search(x,n) for x in p["ignorable"]) else "unknown"
print(json.dumps([kind(n) for n in names]))`,JSON.stringify(policy),JSON.stringify(names)],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as string[];
 expect<unknown>(names.map(name=>dependencyNameKindV1(name,policy))).toEqual(expected);
 expect(expected.filter(kind=>kind==="ignorable").length).toBe(7);
 expect(expected.filter(kind=>kind==="unknown").length).toBeGreaterThanOrEqual(6);
});

test("OS metadata goes, an old foreign name is quarantined, a young one waits, known names stay",()=>{
 const directory=temp();
 try{
  for(const name of[GEN,CAPSULE,".x"+GEN])mkdirSync(join(directory,name));
  for(const name of[".DS_Store","._"+GEN,"Thumbs.db","desktop.ini",".Trashes"])writeFileSync(join(directory,name),"x");
  mkdirSync(join(directory,".Spotlight-V100"));writeFileSync(join(directory,".Spotlight-V100","a"),"x");
  writeFileSync(join(directory,"stray-old"),"old");age(join(directory,"stray-old"),policy.graceMilliseconds+1000);
  writeFileSync(join(directory,"stray-young"),"young");age(join(directory,"stray-young"),1000);
  const done=settleDependencyForeignNamesV1({directory,nowMs:NOW,policy});
  expect([...done.removed].sort()).toEqual([".DS_Store",".Spotlight-V100",".Trashes","Thumbs.db","desktop.ini","._"+GEN].sort());
  expect(done.quarantined).toEqual(["stray-old"]);
  expect(readdirSync(directory).sort()).toEqual([".q",".x"+GEN,CAPSULE,GEN,"stray-young"].sort());
  expect(readdirSync(join(directory,".q"))).toEqual([`${NOW}-stray-old`]);
  expect(settleDependencyForeignNamesV1({directory,nowMs:NOW,policy}).quarantined,"idempotent").toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the quarantine is bounded and collected, oldest first, and a link is moved, never followed",()=>{
 const directory=temp(),outside=temp();
 try{
  writeFileSync(join(outside,"precious"),"keep");symlinkSync(outside,join(directory,"link"));
  const quarantine=join(directory,".q");mkdirSync(quarantine);
  for(let index=0;index<policy.maximum+5;index++)mkdirSync(join(quarantine,`${NOW-1000-index}-old${index}`));
  mkdirSync(join(quarantine,`${NOW-policy.retentionMilliseconds-5}-expired`));
  const done=settleDependencyForeignNamesV1({directory,nowMs:NOW,policy});
  expect(done.quarantined).toEqual(["link"]);
  expect(readdirSync(quarantine).length).toBe(policy.maximum);
  expect(done.collected).toBe(7);
  expect(existsSync(join(outside,"precious"))).toBe(true);
  expect(lstatSync(join(quarantine,`${NOW}-link`)).isSymbolicLink()).toBe(true);
  expect(readdirSync(quarantine).includes(`${NOW-policy.retentionMilliseconds-5}-expired`)).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("the generation names of a roster leave out tombstones, the quarantine, OS metadata and strays, equal to an independent Python re implementation",()=>{
 const names=[GEN,CAPSULE,".x"+GEN,".x"+CAPSULE,".q",".DS_Store","._"+GEN,"stray","0123456789abcdef012"];
 const run=spawnSync(requireTestToolV1("python"),["-c",`import json,re,sys
p=json.loads(sys.argv[1]);names=json.loads(sys.argv[2])
print(json.dumps([n for n in names if re.search(p["generation"],n)]))`,JSON.stringify(policy),JSON.stringify(names)],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(dependencyGenerationNamesV1(names,policy)).toEqual(JSON.parse(run.stdout));
 expect(dependencyGenerationNamesV1(names,policy)).toEqual([GEN,CAPSULE]);
 expect(dependencyGenerationNamesV1([],policy)).toEqual([]);
});

/** 🧭️ Contract: the container check of a retirement compares generation and capsule names and the identity of the folder, so tombstones, the quarantine folder and OS metadata (and a changed size or time of the folder) are no change, while a new or vanished generation, a swapped directory and a returned name are. The Python oracle recomputes the roster difference. */
test("the container check ignores foreign names and times, and names a changed roster",()=>{
 const plan={names:[GEN,CAPSULE].sort(),stamp:JSON.stringify(["1","2","16877","5","160","100","200"])},stamp=(mtime:string,ino="2")=>JSON.stringify(["1",ino,"16877","9","512",mtime,"300"]);
 expect(dependencyContainerChangeV1([GEN,CAPSULE,".q",".x"+GEN,".DS_Store","stray"].sort(),stamp("999"),plan,policy)).toBeUndefined();
 const grown=dependencyContainerChangeV1([GEN,CAPSULE,"fedcba9876543210fedc"].sort(),stamp("100"),plan,policy);
 expect(grown).toEqual({kind:"roster",added:["fedcba9876543210fedc"],gone:[]});
 const lost=dependencyContainerChangeV1([GEN],stamp("100"),plan,policy);
 expect(lost).toEqual({kind:"roster",added:[],gone:[CAPSULE]});
 expect(dependencyContainerChangeV1([GEN,CAPSULE].sort(),stamp("100","7"),plan,policy)).toEqual({kind:"identity"});
 const oracle=spawnSync(requireTestToolV1("python"),["-c","import json,sys;a=set(json.loads(sys.argv[1]));b=set(json.loads(sys.argv[2]));print(json.dumps([sorted(b-a),sorted(a-b)]))",JSON.stringify(plan.names),JSON.stringify([GEN,CAPSULE,"fedcba9876543210fedc"])],{encoding:"utf8"});
 expect(JSON.parse(oracle.stdout)).toEqual([grown?.kind==="roster"?grown.added:undefined,[]]);
});
