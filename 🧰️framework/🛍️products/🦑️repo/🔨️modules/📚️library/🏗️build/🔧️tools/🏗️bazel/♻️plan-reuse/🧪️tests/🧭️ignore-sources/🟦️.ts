/** 🧭️ Contract and oracle: a recorded planner run may only be reused while git would list the same source universe. Git decides what is ignored from `.gitignore` files, `.git/info/exclude` and the file `core.excludesFile` names (default `$XDG_CONFIG_HOME/git/ignore`, found through any config file, include and environment); the witness must follow git's own answer, not a list of files. Every scenario changes one ignore source after a real planner run in a real checkout (HOME, XDG_CONFIG_HOME and the system config are private): `git ls-files -co --exclude-standard` and `git check-ignore --stdin` are the oracles of whether the universe changed, and the witness must differ exactly then. A scenario whose change is irrelevant to git must keep the witness valid. Run from the repository root. */
import {expect,test}from"bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join}from"node:path";
import corpus from"../../../🎬️scopes/🧪️tests/🧱️workspace/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1}from"../../../../../🔭️witness/🟨️.mjs";

process.env.SEMIO_LANGUAGE="en";
const repo=process.cwd(),child=join(import.meta.dirname,"../🧫️fixtures/🟦️.ts"),signal=new AbortController().signal;
const api=await import(join(import.meta.dirname,"../../../📜️script.ts")) as {bazelIgnoreSourcesDigestV1?:(root:string,signal:AbortSignal)=>Promise<string>};
const git=(root:string,...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","core.fsmonitor=false","-c","commit.gpgsign=false",...args],{cwd:root,env:process.env,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(args.join(" ")+": "+run.stderr.toString());return run.stdout.toString();};
const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const universe=(root:string)=>git(root,"ls-files","-co","--exclude-standard","-z");
const ignored=(root:string,paths:readonly string[])=>{const run=Bun.spawnSync(["git","check-ignore","--stdin","-z"],{cwd:root,env:process.env,stdin:Buffer.from(paths.join("\0")+"\0"),stdout:"pipe",stderr:"pipe"});return run.stdout.toString().split("\0").filter(Boolean).sort().join("|");};

interface WorldV1{readonly base:string;readonly root:string;readonly home:string;readonly xdg:string}
interface ScenarioV1{readonly name:string;readonly relevant:boolean;readonly probe:string;setup(world:WorldV1):void;change(world:WorldV1):void}
const tmpFile="course/notes.tmp";
const scenarios:ScenarioV1[]=[
 {name:"core.excludesFile target rewritten to un-ignore",relevant:true,probe:tmpFile,setup:w=>{write(w.base,"ignore-global","*.tmp\n");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,tmpFile,"n\n");},change:w=>write(w.base,"ignore-global","")},
 {name:"core.excludesFile target rewritten to hide a visible file",relevant:true,probe:tmpFile,setup:w=>{write(w.base,"ignore-global","");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,tmpFile,"n\n");},change:w=>write(w.base,"ignore-global","*.tmp\n")},
 {name:"core.excludesFile named with ~ in the global config",relevant:true,probe:tmpFile,setup:w=>{write(w.home,".gitignore_global","*.tmp\n");write(w.home,".gitconfig","[core]\n\texcludesFile = ~/.gitignore_global\n");write(w.root,tmpFile,"n\n");},change:w=>write(w.home,".gitignore_global","")},
 {name:"XDG default ignore file rewritten",relevant:true,probe:tmpFile,setup:w=>{write(w.xdg,"git/ignore","*.tmp\n");write(w.root,tmpFile,"n\n");},change:w=>write(w.xdg,"git/ignore","")},
 {name:"XDG default ignore file created after the plan",relevant:true,probe:tmpFile,setup:w=>{write(w.root,tmpFile,"n\n");},change:w=>write(w.xdg,"git/ignore","*.tmp\n")},
 {name:"core.excludesFile switched to another file in the global config",relevant:true,probe:tmpFile,setup:w=>{write(w.base,"ignore-a","*.tmp\n");write(w.base,"ignore-b","");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-a")}\n`);write(w.root,tmpFile,"n\n");},change:w=>write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-b")}\n`)},
 {name:"core.excludesFile set by an included config file",relevant:true,probe:tmpFile,setup:w=>{write(w.base,"ignore-a","*.tmp\n");write(w.base,"ignore-b","");write(w.base,"included.config",`[core]\n\texcludesFile = ${join(w.base,"ignore-a")}\n`);write(w.home,".gitconfig",`[include]\n\tpath = ${join(w.base,"included.config")}\n`);write(w.root,tmpFile,"n\n");},change:w=>write(w.base,"included.config",`[core]\n\texcludesFile = ${join(w.base,"ignore-b")}\n`)},
 {name:".git/config switches core.excludesFile",relevant:true,probe:tmpFile,setup:w=>{write(w.base,"ignore-a","*.tmp\n");write(w.base,"ignore-b","");git(w.root,"config","core.excludesFile",join(w.base,"ignore-a"));write(w.root,tmpFile,"n\n");},change:w=>git(w.root,"config","core.excludesFile",join(w.base,"ignore-b"))},
 {name:".git/info/exclude emptied",relevant:true,probe:tmpFile,setup:w=>{write(w.root,".git/info/exclude","*.tmp\n");write(w.root,tmpFile,"n\n");},change:w=>write(w.root,".git/info/exclude","")},
 {name:"nested .gitignore gains a negation",relevant:true,probe:"course/keep.tmp",setup:w=>{write(w.root,"course/.gitignore","*.tmp\n");write(w.root,"course/keep.tmp","k\n");},change:w=>write(w.root,"course/.gitignore","*.tmp\n!keep.tmp\n")},
 {name:"global directory rule hides a build directory and is removed",relevant:true,probe:"course/build/out.txt",setup:w=>{write(w.base,"ignore-global","build/\n");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,"course/build/out.txt","o\n");},change:w=>write(w.base,"ignore-global","")},
 {name:"global directory rule hides a dist directory and is added",relevant:true,probe:"course/dist/out.txt",setup:w=>{write(w.base,"ignore-global","");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,"course/dist/out.txt","o\n");},change:w=>write(w.base,"ignore-global","dist/\n")},
 {name:"unrelated key of the global config changes",relevant:false,probe:tmpFile,setup:w=>{write(w.base,"ignore-global","*.tmp\n");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,tmpFile,"n\n");},change:w=>writeFileSync(join(w.home,".gitconfig"),`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n[user]\n\tname = somebody\n`)},
 {name:"unrelated key of .git/config changes",relevant:false,probe:tmpFile,setup:w=>{write(w.root,".git/info/exclude","*.tmp\n");write(w.root,tmpFile,"n\n");},change:w=>git(w.root,"config","core.editor","vim")},
 {name:"core.excludesFile target touched without a change of content",relevant:false,probe:tmpFile,setup:w=>{write(w.base,"ignore-global","*.tmp\n");write(w.home,".gitconfig",`[core]\n\texcludesFile = ${join(w.base,"ignore-global")}\n`);write(w.root,tmpFile,"n\n");},change:w=>{const later=new Date(Date.now()+60000);utimesSync(join(w.base,"ignore-global"),later,later);}},
];

function checkout(world:(w:WorldV1)=>void){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"ignore-"))),root=join(base,"checkout"),home=join(base,"home"),xdg=join(base,"xdg");mkdirSync(root);mkdirSync(home);mkdirSync(xdg);
 const saved={HOME:process.env.HOME,XDG_CONFIG_HOME:process.env.XDG_CONFIG_HOME,GIT_CONFIG_NOSYSTEM:process.env.GIT_CONFIG_NOSYSTEM,GIT_CONFIG_GLOBAL:process.env.GIT_CONFIG_GLOBAL};
 process.env.HOME=home;process.env.XDG_CONFIG_HOME=xdg;process.env.GIT_CONFIG_NOSYSTEM="1";delete process.env.GIT_CONFIG_GLOBAL;
 for(const[path,text]of Object.entries(corpus.files))write(root,path,text);
 write(root,"other/x.txt","x\n");
 git(root,"init","-q");git(root,"add","-A");git(root,"commit","-q","-m","fixture");
 const w={base,root,home,xdg};world(w);
 return{...w,restore:()=>{for(const[name,value]of Object.entries(saved)){if(value===undefined)delete process.env[name];else process.env[name]=value;}rmSync(base,{recursive:true,force:true});}};
}
function plan(base:string,root:string){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
 const env={...process.env,SEMIO_LANGUAGE:"en",...worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace:join(base,"ws"),writable:[tmpdir()],racyMilliseconds:0})};
 const run=Bun.spawnSync([process.execPath,"--preload",join(base,"preload.mjs"),child,root,join(base,"ws"),"{}",repo],{cwd:repo,env,stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString()+run.stdout.toString());
 return readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8"));
}
const ports=(root:string)=>({
 git:async(subject:string,expected:string)=>{const specs=JSON.parse(subject) as string[];let listing:string;try{listing=git(root,"ls-files","-c","-z",...(specs.length?["--",...specs]:[]));}catch{return expected==="!failed";}return worldWitnessGitDigestV1(listing)===expected;},
 ...(api.bazelIgnoreSourcesDigestV1===undefined?{}:{ignore:async(_subject:string,expected:string)=>(await api.bazelIgnoreSourcesDigestV1!(root,signal))===expected}),
});

for(const scenario of scenarios)test(`[ignore-sources] ${scenario.name}: the witness differs exactly when git's universe changes`,async()=>{
 const world=checkout(scenario.setup);
 try{
  const witness=plan(world.base,world.root),before=universe(world.root),verdictBefore=ignored(world.root,[scenario.probe]);
  scenario.change(world);
  const after=universe(world.root),verdictAfter=ignored(world.root,[scenario.probe]);
  expect(after!==before,"oracle: git ls-files").toBe(scenario.relevant);
  expect(verdictAfter!==verdictBefore,"oracle: git check-ignore").toBe(scenario.relevant);
  const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(world.root)});
  console.log(`[ignore-sources] ${scenario.name}: universe changed ${after!==before}, witness ${JSON.stringify(verdict)}, tainted ${JSON.stringify(witness.tainted)}`);
  if(scenario.relevant)expect(witness.tainted.length>0||verdict.holds===false,"universe changed but the witness still holds").toBe(true);
  else{expect(witness.tainted).toEqual([]);expect(verdict.holds,"universe unchanged but the witness differs").toBe(true);}
 }finally{world.restore();}
},120000);
