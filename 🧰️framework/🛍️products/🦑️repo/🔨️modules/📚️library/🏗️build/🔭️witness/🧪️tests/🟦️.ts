/** 🔭️ Contract: a computation that runs under the preload leaves a witness of everything it read; the witness holds exactly while the world is unchanged for those facts, ignores every unrelated edit, and names what it cannot see so that such a computation is never reused. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync,readFileSync,chmodSync}from"node:fs";
import {tmpdir}from"node:os";
import {join}from"node:path";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import {readWorldWitnessSwitchV1,readWorldWitnessV1,summarizeWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessPreloadV1}from"../🟨️.mjs";

const old=new Date(Date.now()-3600_000);
function world(){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"witness-"))),root=join(base,"root");
 mkdirSync(join(root,"src/deep"),{recursive:true});mkdirSync(join(root,".cache"),{recursive:true});mkdirSync(join(base,"ws"));mkdirSync(join(root,"unrelated"));
 for(const[path,text]of Object.entries({"src/a.txt":"one","src/deep/b.txt":"two","unrelated/c.txt":"three"})){writeFileSync(join(root,path),text);utimesSync(join(root,path),old,old);}
 symlinkSync("a.txt",join(root,"src/link"));
 return{base,root};
}
function run(base:string,root:string,script:string,env:Record<string,string>={},racyMilliseconds=0){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());writeFileSync(join(base,"scenario.mjs"),script);
 const result=Bun.spawnSync([process.execPath,"--preload",join(base,"preload.mjs"),join(base,"scenario.mjs"),root],{cwd:root,env:{...process.env,...worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace:join(base,"ws"),writable:[join(base,"ws")],racyMilliseconds}),...env},stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(result.stderr.toString());
 return readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8"));
}
const signal=new AbortController().signal,holds=async(witness:ReturnType<typeof readWorldWitnessV1>)=>(await verifyWorldWitnessV1(witness,{signal})).holds;
const scenario=`
import {lstat,readFile,readdir,stat,readlink,realpath,access,writeFile} from "node:fs/promises";
import {existsSync,readFileSync} from "node:fs";
const root=process.argv[2];
await lstat(root+"/src/a.txt");await stat(root+"/src/absent.txt").catch(()=>{});await readdir(root+"/src");await readFile(root+"/src/deep/b.txt","utf8");
await readlink(root+"/src/link");await realpath(root+"/src/link");await access(root+"/src/a.txt");existsSync(root+"/src/nothing");readFileSync(root+"/src/a.txt");
process.env.SEMIO_FIXTURE_VALUE;await writeFile(root+"/.cache/out.txt","x");await readFile(root+"/.cache/out.txt","utf8");
globalThis.__semioWorldWitnessV1.finish();
`;

test("a witness lists what was read, leaves out infrastructure and the computation's own output, and holds on an unchanged world",async()=>{
 const{base,root}=world();
 try{
  const witness=run(base,root,scenario,{SEMIO_FIXTURE_VALUE:"a"});
  expect(witness.tainted).toEqual([]);expect(witness.racy).toBeNull();expect(witness.unstable).toEqual([]);
  const kinds=summarizeWorldWitnessV1(witness);
  for(const kind of["l","s","d","f","r","p","a0","e"])expect(kinds[kind],kind).toBeGreaterThan(0);
  expect(witness.observations.some(row=>row[1].includes("/.cache/"))).toBe(false);
  process.env.SEMIO_FIXTURE_VALUE="a";
  expect(await holds(witness)).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("every edit of something the computation read breaks the witness and every unrelated edit leaves it holding",async()=>{
 const edits:Record<string,(root:string)=>void>={
  "content":root=>{writeFileSync(join(root,"src/deep/b.txt"),"TWO");},
  "touch":root=>{const now=new Date();utimesSync(join(root,"src/deep/b.txt"),now,now);},
  "mode":root=>{chmodSync(join(root,"src/a.txt"),0o600);},
  "appears":root=>{writeFileSync(join(root,"src/absent.txt"),"new");},
  "listing":root=>{writeFileSync(join(root,"src/extra.txt"),"new");},
  "link target":root=>{rmSync(join(root,"src/link"));symlinkSync("deep/b.txt",join(root,"src/link"));},
  "access":root=>{writeFileSync(join(root,"src/nothing"),"x");},
  "environment":()=>{process.env.SEMIO_FIXTURE_VALUE="b";},
 };
 const unrelated:Record<string,(root:string)=>void>={
  "other file":root=>{writeFileSync(join(root,"unrelated/c.txt"),"THREE");},
  "other directory entry":root=>{writeFileSync(join(root,"unrelated/new.txt"),"new");},
  "infrastructure":root=>{writeFileSync(join(root,".cache/noise.txt"),"noise");},
  "other variable":()=>{process.env.SEMIO_FIXTURE_OTHER="x";},
 };
 for(const[name,edit]of Object.entries(edits)){
  const{base,root}=world();
  try{const witness=run(base,root,scenario,{SEMIO_FIXTURE_VALUE:"a"});process.env.SEMIO_FIXTURE_VALUE="a";expect(await holds(witness),"before "+name).toBe(true);edit(root);expect(await holds(witness),name).toBe(false);}finally{rmSync(base,{recursive:true,force:true});delete process.env.SEMIO_FIXTURE_OTHER;}
 }
 for(const[name,edit]of Object.entries(unrelated)){
  const{base,root}=world();
  try{const witness=run(base,root,scenario,{SEMIO_FIXTURE_VALUE:"a"});process.env.SEMIO_FIXTURE_VALUE="a";edit(root);expect(await holds(witness),name).toBe(true);}finally{rmSync(base,{recursive:true,force:true});delete process.env.SEMIO_FIXTURE_OTHER;}
 }
});

test("an entry kind row holds through edits of bytes, times and mode, never makes the witness racy, and breaks when the entry vanishes or becomes another kind; without the witness the same reader answers directly",async()=>{
 const reader=`import {worldEntryKindV1} from ${JSON.stringify(join(import.meta.dirname,"../🟨️.mjs"))};const root=process.argv[2];const kinds=[await worldEntryKindV1(root+"/src/a.txt"),await worldEntryKindV1(root+"/src/link"),await worldEntryKindV1(root+"/src/link",true),await worldEntryKindV1(root+"/src/deep"),await worldEntryKindV1(root+"/src/absent.txt"),await worldEntryKindV1(root+"/.cache")];if(kinds.join()!=="file,link,file,directory,!ENOENT,directory")throw Error(kinds.join());globalThis.__semioWorldWitnessV1.finish();`;
 const holding:Record<string,(root:string)=>void>={
  "content":root=>{writeFileSync(join(root,"src/a.txt"),"ONE AND MORE");},
  "touch":root=>{const now=new Date();utimesSync(join(root,"src/a.txt"),now,now);},
  "mode":root=>{chmodSync(join(root,"src/a.txt"),0o755);},
  "link target content":root=>{writeFileSync(join(root,"src/a.txt"),"other");},
 };
 const breaking:Record<string,(root:string)=>void>={
  "vanished":root=>{rmSync(join(root,"src/a.txt"));},
  "file becomes a link":root=>{rmSync(join(root,"src/a.txt"));symlinkSync("deep/b.txt",join(root,"src/a.txt"));},
  "link becomes a file":root=>{rmSync(join(root,"src/link"));writeFileSync(join(root,"src/link"),"x");},
  "appears":root=>{writeFileSync(join(root,"src/absent.txt"),"new");},
  "directory becomes a file":root=>{rmSync(join(root,"src/deep"),{recursive:true});writeFileSync(join(root,"src/deep"),"x");},
 };
 for(const[group,edits]of[["holding",holding],["breaking",breaking]]as const)for(const[name,edit]of Object.entries(edits)){
  const{base,root}=world();
  try{
   writeFileSync(join(root,"src/a.txt"),"fresh");
   const witness=run(base,root,reader,{},3000);
   expect(witness.racy,name).toBeNull();expect(witness.tainted,name).toEqual([]);
   expect(witness.observations.filter(row=>row[0]==="k"||row[0]==="K").length,name).toBe(5);
   expect(witness.observations.some(row=>row[1].includes("/.cache")),name).toBe(false);
   expect(witness.observations.some(row=>row[0]!=="k"&&row[0]!=="K"&&row[0]!=="e"&&row[0]!=="E"),name).toBe(false);
   edit(root);expect(await holds(witness),group+": "+name).toBe(group==="holding");
  }finally{rmSync(base,{recursive:true,force:true});}
 }
 const{worldEntryKindV1}=await import("../🟨️.mjs"),{base,root}=world();
 try{expect([await worldEntryKindV1(join(root,"src/link")),await worldEntryKindV1(join(root,"src/link"),true),await worldEntryKindV1(join(root,"nothing"))]).toEqual(["link","file","!ENOENT"]);}finally{rmSync(base,{recursive:true,force:true});}
});

test("whatever the witness cannot see is named, and a file modified moments before it was read makes the witness racy",()=>{
 const{base,root}=world();
 try{
  const outside=join(base,"outside.txt"),database=join(base,"outside.db");
  const sneaky=`
   import {spawnSync} from "node:child_process";import {freemem} from "node:os";import {Database} from "bun:sqlite";import {writeFileSync} from "node:fs";
   const root=process.argv[2];spawnSync("true");freemem();await Bun.file(root+"/src/a.txt").text();Bun.spawnSync(["true"]);
   writeFileSync(${JSON.stringify(outside)},"x");new Database(${JSON.stringify(database)});
   const {...everything}=process.env;fetch("http://127.0.0.1:9").catch(()=>{});
   globalThis.__semioWorldWitnessV1.finish();`;
  const witness=run(base,root,sneaky);
  const text=witness.tainted.join("\n");
  for(const needle of["spawned spawnSync true","volatile host fact freemem","spawned Bun.spawnSync","wrote outside the declared places","opened a database outside","network fetch"])expect(text,needle).toContain(needle);
  writeFileSync(join(root,"src/fresh.txt"),"just now");
  const racy=run(base,root,`import {readFileSync} from "node:fs";readFileSync(process.argv[2]+"/src/fresh.txt");globalThis.__semioWorldWitnessV1.finish();`,{},3000);
  expect(racy.racy).toEndWith("/src/fresh.txt");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a file the witness skips (infrastructure, the computation's own output) never makes it racy, however fresh",()=>{
 const{base,root}=world();
 try{
  const own=run(base,root,`import {writeFileSync,statSync,lstatSync,mkdirSync} from "node:fs";import {stat} from "node:fs/promises";const root=process.argv[2];writeFileSync(root+"/.cache/fresh.txt","x");statSync(root+"/.cache/fresh.txt");lstatSync(root+"/.cache/fresh.txt");await stat(root+"/.cache/fresh.txt");mkdirSync(root+"/../ws",{recursive:true});writeFileSync(root+"/../ws/made.txt","y");statSync(root+"/../ws/made.txt");globalThis.__semioWorldWitnessV1.finish();`,{},3000);
  expect(own.tainted).toEqual([]);expect(own.racy).toBeNull();
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a name the policy ignores is never an input: reading it (also through a spread of the environment) leaves no row, any other read does",()=>{
 const{base,root}=world();
 try{
  const witness=run(base,root,`const copy={...process.env};process.env.SEMIO_BUILD_PARALLEL;process.env.SEMIO_FIXTURE_VALUE;globalThis.__semioWorldWitnessV1.finish();`,{SEMIO_BUILD_PARALLEL:"6",SEMIO_LEASE_PRIORITY:"batch",SEMIO_FIXTURE_VALUE:"a",SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust",SEMIO_BAZEL_CONTROL_URL:"tcp://127.0.0.1:1/x",SEMIO_HOST_CAPACITY_GRANT:"g"});
  const names=new Set(witness.observations.filter(row=>row[0]==="e").map(row=>row[1]));
  for(const name of["SEMIO_BUILD_PARALLEL","SEMIO_LEASE_PRIORITY","SEMIO_REPO_SEAL","SEMIO_REPO_IMPLEMENTATION","SEMIO_BAZEL_CONTROL_URL","SEMIO_HOST_CAPACITY_GRANT","PWD","SHLVL","_"])expect(names.has(name),name).toBe(false);
  expect(names.has("SEMIO_FIXTURE_VALUE")).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("an ambient host, agent or session variable and the identity of the agent are never an input: reading it (also through a spread) leaves no row and no enumeration entry, and its change never breaks the witness while another variable still does",async()=>{
 const{base,root}=world(),ambient={CLAUDE_FIXTURE_ID:"a",ANTHROPIC_FIXTURE_URL:"u",XPC_FIXTURE_FLAGS:"0x0",__CFFixture:"b",TERM_SESSION_ID:"w0t0p0",SECURITYSESSIONID:"q",LaunchInstanceID:"l"},identity={SEMIO_AGENT_SESSION:"claude/s1",AI_AGENT:"claude-code"},given={...ambient,...identity,SEMIO_FIXTURE_VALUE:"a"};
 try{
  const witness=run(base,root,`const copy={...process.env};for(const name of Object.keys(copy))process.env[name];globalThis.__semioWorldWitnessV1.finish();`,given);
  const names=new Set(witness.observations.filter(row=>row[0]==="e").map(row=>row[1])),enumeration=witness.observations.find(row=>row[0]==="E")![2];
  for(const name of [...Object.keys(ambient),...Object.keys(identity)]){expect(names.has(name),name).toBe(false);expect(enumeration.includes(name+"="),name).toBe(false);}
  expect(names.has("SEMIO_FIXTURE_VALUE")).toBe(true);expect(enumeration.includes("SEMIO_FIXTURE_VALUE=")).toBe(true);
  const now={...process.env,...given};
  expect((await verifyWorldWitnessV1(witness,{signal,environment:now})).holds).toBe(true);
  expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,CLAUDE_FIXTURE_ID:"b",XPC_FIXTURE_FLAGS:"0x1",TERM_SESSION_ID:"w0t1p0",ANTHROPIC_FIXTURE_URL:undefined,SEMIO_AGENT_SESSION:"codex/s9",AI_AGENT:undefined}})).holds).toBe(true);
  expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,SEMIO_FIXTURE_VALUE:"b"}})).holds).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("an allowed spawn and an explicit observation are recorded, a changed answer of a port breaks the witness and cancellation stops the check",async()=>{
 const{base,root}=world();
 try{
  const witness=run(base,root,`const w=globalThis.__semioWorldWitnessV1;import {spawnSync} from "node:child_process";w.allow("t",()=>spawnSync("true"));w.observe("x","command\\0prototype","d1");w.finish();`);
  expect(witness.tainted).toEqual([]);expect(witness.observations).toContainEqual(["x","command\0prototype","d1"]);expect(witness.observations.some(row=>row[0]==="E")).toBe(true);
  let answer="d1";const ports={runtime:async(_subject:string,expected:string)=>expected===answer};
  expect((await verifyWorldWitnessV1(witness,{signal,ports})).holds).toBe(true);
  answer="d2";expect(await verifyWorldWitnessV1(witness,{signal,ports})).toMatchObject({holds:false,kind:"x"});
  const abort=new AbortController();abort.abort(Error("cancelled"));
  await expect(verifyWorldWitnessV1(witness,{signal:abort.signal,ports})).rejects.toThrow("cancelled");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a closed table of read-only host probes does not taint the witness, any other spawn of the same programs still does, and the probe files stay out of the observations",()=>{
 const{base,root}=world();
 try{
  const probeFile=join(base,"boot-id");writeFileSync(probeFile,"first");
  const probes=`import {spawnSync,execFileSync} from "node:child_process";import {readFileSync} from "node:fs";
   const w=globalThis.__semioWorldWitnessV1,t=f=>{try{f();}catch{}};t(()=>Bun.spawnSync(["sysctl","-n","kern.boottime"]));t(()=>Bun.spawnSync({cmd:["/sbin/mount"]}));t(()=>spawnSync("ps",["-axo","pid=,ppid="]));t(()=>spawnSync("ps",["-o","etime=","-p","1"]));t(()=>spawnSync("ps",["-o","cputime=","-p","1,2,3"]));t(()=>execFileSync("hdiutil",["info","-plist"],{stdio:"ignore"}));
   readFileSync(process.argv[3]);w.finish();`;
  const quiet=(extra:Record<string,unknown>={})=>{
   writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());writeFileSync(join(base,"scenario.mjs"),probes);
   const result=Bun.spawnSync([process.execPath,"--preload",join(base,"preload.mjs"),join(base,"scenario.mjs"),root,probeFile],{cwd:root,env:{...process.env,...worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace:join(base,"ws"),writable:[join(base,"ws")],racyMilliseconds:0,hostProbeFiles:[probeFile],...extra})},stdout:"pipe",stderr:"pipe"});
   if(result.exitCode!==0&&!/hdiutil|ENOENT|not found/iu.test(result.stderr.toString()))throw Error(result.stderr.toString());
   return readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8"));
  };
  const witness=quiet();
  expect(witness.tainted).toEqual([]);expect(witness.observations.some(row=>row[1]===probeFile)).toBe(false);
  const closed=run(base,root,`import {spawnSync} from "node:child_process";const w=globalThis.__semioWorldWitnessV1,t=f=>{try{f();}catch{}};t(()=>Bun.spawnSync(["sysctl","-w","kern.boottime=0"]));t(()=>spawnSync("hdiutil",["attach","x.dmg"]));t(()=>Bun.spawnSync(["/nonexistent/ps","-axo","pid=,ppid=","--extra"]));t(()=>spawnSync("ps",["-axo","pid=,ppid=,command="]));t(()=>spawnSync("ps",["-ef"]));t(()=>spawnSync("ps",["-o","etime=","-p","1;x"]));t(()=>spawnSync("ps",["-o","cputime=","-p","1,,2"]));t(()=>spawnSync("ps",["-o","etime=","-p","1","2"]));w.finish();`);
  expect(closed.tainted.length).toBeGreaterThanOrEqual(3);expect(closed.tainted.some(reason=>/spawned \S+ ps -axo pid=,ppid=( @|$)/u.test(reason))).toBe(false);
  for(const vector of["ps -axo pid=,ppid=,command=","ps -ef","ps -o etime= -p 1;x","ps -o cputime= -p 1,,2","ps -o etime= -p 1 2"])expect(closed.tainted.some(reason=>reason.includes("spawned")&&reason.includes(vector)),vector+" must taint").toBe(true);expect(closed.tainted.join("\n")).toMatch(/sysctl/u);expect(closed.tainted.join("\n")).toMatch(/hdiutil/u);expect(closed.tainted.join("\n")).toMatch(/spawned Bun\.spawnSync sysctl -w kern\.boottime=0/u);expect(closed.tainted.join("\n")).toMatch(/\/nonexistent\/ps -axo pid=,ppid= --extra/u);
  const none=quiet({hostProbes:[],hostProbeFiles:[]});
  expect(none.tainted.join("\n")).toMatch(/spawned/u);expect(none.observations.some(row=>row[1]===probeFile)).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the host probe table names every boot-identity command the native storage declares, so the two cannot drift apart",()=>{
 const storage=join(import.meta.dirname,"../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"),declared=(file:string)=>JSON.parse(readFileSync(join(storage,file),"utf8")),found:Array<{program:string;arguments:string[]}>=[];
 const walk=(value:unknown)=>{if(Array.isArray(value))value.forEach(walk);else if(value!==null&&typeof value==="object"){const row=value as Record<string,unknown>;if(typeof row.program==="string"&&/sysctl$/u.test(row.program)&&Array.isArray(row.arguments))found.push({program:row.program,arguments:row.arguments as string[]});Object.values(row).forEach(walk);}};
 walk(declared("🧬️schema/🔣️.json"));walk(declared("🌐️platform/🥾️boot/🩹️recovery/🧬️schema/🔣️.json"));
 expect(found.length).toBeGreaterThanOrEqual(2);
 const table=schema.definitions.Policy.const.hostProbes.filter(row=>row.arguments!==null).map(row=>[row.program,...row.arguments!].join("\0"));
 for(const row of found)expect(table,row.program+" "+row.arguments.join(" ")).toContain(["sysctl",...row.arguments].join("\0"));
});

test("an observation of git's ignore sources is decided by the ignore port alone: without a port, with a refusing port and with a changed answer the witness breaks",async()=>{
 const{base,root}=world();
 try{
  const witness=run(base,root,`const w=globalThis.__semioWorldWitnessV1;w.observe("i","excludes","d1");w.finish();`);
  expect(witness.tainted).toEqual([]);expect(witness.observations).toContainEqual(["i","excludes","d1"]);
  expect(await verifyWorldWitnessV1(witness,{signal})).toMatchObject({holds:false,kind:"i"});
  let answer="d1";const ignore=async(_subject:string,expected:string)=>expected===answer;
  expect((await verifyWorldWitnessV1(witness,{signal,ports:{ignore}})).holds).toBe(true);
  answer="d2";expect(await verifyWorldWitnessV1(witness,{signal,ports:{ignore}})).toMatchObject({holds:false,kind:"i",subject:"excludes"});
  expect(Object.keys(schema.definitions.Policy.const.kinds)).toContain("i");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the switch reads on, off and verify and refuses anything else in both languages",()=>{
 expect(readWorldWitnessSwitchV1({})).toBe("on");expect(readWorldWitnessSwitchV1({SEMIO_REPO_PLAN_REUSE:"VERIFY"})).toBe("verify");expect(readWorldWitnessSwitchV1({SEMIO_REPO_PLAN_REUSE:"off"})).toBe("off");
 expect(()=>readWorldWitnessSwitchV1({SEMIO_REPO_PLAN_REUSE:"maybe"})).toThrow(/Erwartet SEMIO_REPO_PLAN_REUSE=on\|off\|verify/u);
});

test("the one environment door the witness cannot observe stays closed: no production module of the build library or the process module reads Bun.env",()=>{
 const roots=[join(import.meta.dirname,"../.."),join(import.meta.dirname,"../../../../../../../🔨️modules/🏃️process")],found:string[]=[];
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(entry.isDirectory()){if(!["🧪️tests","node_modules","🧫️fixtures","🔭️witness"].includes(entry.name))walk(join(directory,entry.name));continue;}
  if(!/\.(?:ts|mjs)$/u.test(entry.name))continue;
  if(readFileSync(join(directory,entry.name),"utf8").split("\n").some(line=>!/^\s*(?:\/\/|\*|\/\*)/u.test(line)&&/\bBun\.env\b|import\.meta\.env\b/u.test(line)))found.push(join(directory,entry.name));
 }};
 for(const root of roots)walk(root);
 expect(found).toEqual([]);
});
