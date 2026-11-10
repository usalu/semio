/** 🫧️ Contract: the repository walk an owner starts (the crate index of all Cargo manifests) gives the event loop a turn at least every `maximumTurnMilliseconds` and stops within `cancelWithinMilliseconds` of its cancellation, on a generated workspace with thousands of directories; its answer equals the synchronous walk of the same tree. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,engine=await import("../../../../📥️inputs/🎯️precision/🟦️.ts") as any;
const roots:string[]=[],skip={paths:[],names:["node_modules","target"]};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

/** 🏗️ The generated tree, built once for the file: branches of nested directories, a few crates per branch. */
let tree:ReturnType<typeof build>|undefined;
const generate=()=>tree??=build();
function build():{root:string;crates:string[];directories:string[]}{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"cargo-yield-"))),crates:string[]=[],directories=new Set<string>([root]);roots.push(root);
 for(let branch=0;branch<fixture.branches;branch++){
  for(let at=0;at<fixture.directoriesPerBranch;at++){
   const directory="b"+branch+"/d"+(at%20)+"/e"+at;mkdirSync(join(root,directory),{recursive:true});for(const part of["b"+branch,"b"+branch+"/d"+(at%20),directory])directories.add(join(root,part));
   if(at<fixture.cratesPerBranch){const name="crate-"+branch+"-"+at;writeFileSync(join(root,directory,"Cargo.toml"),'[package]\nname = "'+name+'"\nversion = "0.1.0"\n');crates.push(name);}
  }
 }
 return{root,crates:crates.sort(),directories:[...directories]};
}

/** 🔬️ The production synchronous walk on the plain file system. */
function synchronous(root:string):string[]{
 const list=(directory:string)=>{try{return readdirSync(join(root,directory),{withFileTypes:true}).map(entry=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile(),link:entry.isSymbolicLink()}));}catch{return[];}};
 return[...engine.crateNamesV1(engine.cargoManifestDirectoriesV1(list,skip),(path:string)=>{try{return readFileSync(join(root,path),"utf8");}catch{return undefined;}}).keys()].sort();
}

const ask=(root:string,signal?:AbortSignal)=>{
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:false,...signal?{phaseControl:{signal}}:{}}),api=scriptInputs.closureApiV1(scripts,root);
 return observation.observeInferenceSnapshotV1(()=>api.derive({fn:"crate-index",skip}));
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the crate walk of an owner gives the event loop a turn within the declared time and equals the synchronous walk",async()=>{
 const{root,crates}=generate(),warm=realpathSync(mkdtempSync(join(tmpdir(),"cargo-yield-warm-")));roots.push(warm);writeFileSync(join(warm,"Cargo.toml"),'[package]\nname = "warm"\nversion = "0.1.0"\n');await ask(warm);
 let last=performance.now(),longest=0;
 const timer=setInterval(()=>{const now=performance.now();longest=Math.max(longest,now-last);last=now;},20);
 try{
  last=performance.now();
  const answer=await ask(root);
  expect([...answer].map((entry:[string,string])=>entry[0]).sort()).toEqual(crates);console.log("[DATA] cargo walk "+JSON.stringify({directories:fixture.branches*fixture.directoriesPerBranch,turnMs:Math.round(longest)}));expect(synchronous(root)).toEqual(crates);
 }finally{clearInterval(timer);}
 expect(longest,"the longest turn of the event loop in milliseconds").toBeLessThan(fixture.maximumTurnMilliseconds);
});

test("the crate walk stops within the declared time of its cancellation",async()=>{
 const{root}=generate(),controller=new AbortController();
 const asked=ask(root,controller.signal).then(()=>"finished",(error:Error)=>error);
 await new Promise(resolve=>setTimeout(resolve,fixture.cancelAfterMilliseconds));
 const cancelledAt=performance.now();controller.abort(Error("stopped by the test"));
 const outcome=await asked,took=performance.now()-cancelledAt;
 expect(outcome).not.toBe("finished");expect(String((outcome as Error).message)).toContain("stopped by the test");
 expect(took).toBeLessThan(fixture.cancelWithinMilliseconds);
});

const owner=(root:string,late:boolean)=>{
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:late}),api=scriptInputs.closureApiV1(scripts,root);
 return observation.observeInferenceSnapshotV1(()=>api.derive({fn:"crate-index",skip}));
};

test("ten cargo owners of one pass list every directory at most once and a clean second pass with stable directories lists none",async()=>{
 const{root,crates,directories}=generate();scriptInputs.resetClosureEntriesV1();
 const count=()=>observation.observationProbeCountsV1().readdir;
 const first=count();
 await scriptInputs.withClosurePassV1(async()=>{for(let at=0;at<fixture.owners;at++){const answer=await owner(root,true);expect([...answer].map((entry:[string,string])=>entry[0]).sort()).toEqual(crates);}});
 const listed=count()-first;
 expect(listed,"directory listings of ten owners in one pass").toBeLessThanOrEqual(directories.length);
 expect(scriptInputs.closureEntryCountsV1().derived,"the crate index is derived once").toBe(1);
 expect(scriptInputs.closureEntryCountsV1().checked,"the entry the pass derived needs no further verification").toBe(0);
 observation.useStableDirectoriesV1(new Set(directories));
 const before=count(),derived=scriptInputs.closureEntryCountsV1().derived;
 try{await scriptInputs.withClosurePassV1(async()=>{for(let at=0;at<fixture.owners;at++)await owner(root,true);});}finally{observation.useStableDirectoriesV1(new Set());}
 expect(count()-before,"directory listings of a clean second pass").toBe(0);
 expect(scriptInputs.closureEntryCountsV1().derived,"the second pass derives nothing").toBe(derived);
 expect(scriptInputs.closureEntryCountsV1().checked,"N owners of the second pass verify the entry once").toBe(1);
});

test("without stable directories the second pass verifies the shared entry once for all owners and lists each recorded directory once",async()=>{
 const{root,directories}=generate();scriptInputs.resetClosureEntriesV1();
 await scriptInputs.withClosurePassV1(()=>owner(root,true));
 const before=observation.observationProbeCountsV1().readdir,checked=scriptInputs.closureEntryCountsV1().checked;
 await scriptInputs.withClosurePassV1(async()=>{for(let at=0;at<fixture.owners;at++)await owner(root,true);});
 expect(scriptInputs.closureEntryCountsV1().checked-checked,"one verification for ten owners").toBe(1);
 expect(observation.observationProbeCountsV1().readdir-before).toBeLessThanOrEqual(directories.length);
});
