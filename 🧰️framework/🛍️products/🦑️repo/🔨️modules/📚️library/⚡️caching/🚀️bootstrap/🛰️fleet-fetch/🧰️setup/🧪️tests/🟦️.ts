import {afterAll,expect,test} from "bun:test";
import {chmodSync,cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetFetchContextV1} from "../../🟦️.ts";
import {syncFleetIdentityV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {FLEET_CARGO_TOOLS_V1,FLEET_SETUP_FAMILY_V1,fleetCargoToolV1,fleetSetupOutputV1,placeSetupRootV1,rustChannelV1,setupClassV1,setupHostV1,setupKeyPartsV1,setupTextV1,type SetupHostV1,type SetupKindV1,type SetupOutputV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-setup-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
import platformClassSchema from "../../../../../🏗️build/🪪️graph-state/🌍️platform-class/🧬️schema/🔣️.json" with {type:"json"};
const here=import.meta.dirname,platformSchema=join(directory("platform"),"platform-class.json");
writeFileSync(platformSchema,JSON.stringify(platformClassSchema));
const python=async(...argv:string[])=>{const child=Bun.spawn(["python3","-I",...argv],{stdout:"pipe",stderr:"pipe"}),[out,code,err]=await Promise.all([new Response(child.stdout).text(),child.exited,new Response(child.stderr).text()]);if(code!==0)throw Error(err);return out.trim();};

const darwin:SetupHostV1={platform:"darwin",arch:"arm64"},linuxGnu:SetupHostV1={platform:"linux",arch:"x64",libc:"gnu"},linuxMusl:SetupHostV1={platform:"linux",arch:"x64",libc:"musl"};
const template=directory("identity");mkdirSync(join(template,".cache","f"),{recursive:true});writeFileSync(join(template,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:template,key:"k"}));
await syncFleetIdentityV1({root:template});
const machine=(store:string)=>{
 const root=directory("machine");
 mkdirSync(join(root,".cache","f"),{recursive:true});cpSync(join(template,".cache","f","identity"),join(root,".cache","f","identity"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-m01",groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins","g1-m01"),{recursive:true});cpSync(join(template,".cache","f","identity","registration.json"),join(store,"v1","origins","g1-m01","fleet-auth.json"));
 const home=join(root,"cargo-home");mkdirSync(join(home,"bin"),{recursive:true});
 return {root,home,context:fleetFetchContextV1({},root,{storeRoot:store,remote:undefined})};
};
const spaceOf=(store:string)=>join(store,"v1","fleet","setup");
const say=(lines:string[])=>(line:string)=>{lines.push(line);};

type Built={count:number};
const tool=(name:string,body:string)=>async(home:string,built:Built)=>{mkdirSync(join(home,"bin"),{recursive:true});writeFileSync(join(home,"bin",name),body);chmodSync(join(home,"bin",name),0o755);built.count++;};

test("the class of a machine equals an independent model written in Python from graph-state/platform-class",async()=>{
 const expected=JSON.parse(await python(join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"../🧬️schema/🔣️.json"),platformSchema)) as {classes:(string|null)[];keys:string[]};
 fixture.classes.forEach((row,index)=>{
  const klass=setupClassV1(row.kind as SetupKindV1,row.host as SetupHostV1);
  expect(klass?.tag??null,row.name).toBe(row.tag);
  expect(klass?.tag??null,row.name).toBe(expected.classes[index]!);
 });
 expect(setupHostV1("linux","x64",()=>["ld-musl-x86_64.so.1"]).libc).toBe("musl");
 expect(setupHostV1("linux","x64",()=>["ld-linux-x86-64.so.2"]).libc).toBe("gnu");
 expect(setupHostV1("darwin","arm64").libc).toBeUndefined();
});

test("a key is a function of the real inputs, the roots and the class and of nothing else; Python computes the same keys",async()=>{
 const expected=JSON.parse(await python(join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"../🧬️schema/🔣️.json"),platformSchema)) as {keys:string[]};
 fixture.keys.forEach((row,index)=>{
  const klass=setupClassV1(row.kind as SetupKindV1,row.host as SetupHostV1)!;
  expect(setupKeyPartsV1(row.task,row.inputs,row.kind as SetupKindV1,row.roots,klass).key,row.task).toBe(expected.keys[index]!);
 });
 const base=fixture.keys[0]!,klass=setupClassV1("host",darwin)!,key=(inputs:Record<string,string>,roots=base.roots,k=klass)=>setupKeyPartsV1(base.task,inputs,"host",roots,k).key;
 expect(key({...base.inputs})).toBe(key({locked:"true",version:"0.21.14",crate:"trunk",channel:"nightly-2026-07-20"}));
 expect(key({...base.inputs,version:"0.21.15"})).not.toBe(key(base.inputs));
 expect(key({...base.inputs,channel:"nightly-2026-07-21"})).not.toBe(key(base.inputs));
 expect(key(base.inputs,["bin/other"])).not.toBe(key(base.inputs));
 expect(key(base.inputs,base.roots,setupClassV1("host",linuxGnu)!)).not.toBe(key(base.inputs));
});

test("the second machine of the same class with equal inputs performs 0 builds and restores every output; another class builds and uploads under its own class",async()=>{
 const store=directory("store"),one=machine(store),two=machine(store),three=machine(store),four=machine(store),five=machine(store),six=machine(store);
 const channel="nightly-2026-07-20",trunk={crate:"trunk",version:"0.21.14",channel,home:""},bindgen={crate:"wasm-bindgen-cli",version:"0.2.129",channel,home:""};
 const run=async(m:ReturnType<typeof machine>,host:SetupHostV1,built:Built,lines:string[],crate:typeof trunk=trunk)=>{
  const names=FLEET_CARGO_TOOLS_V1[crate.crate]!;
  return fleetCargoToolV1(m.context,{...crate,home:m.home,windows:false},async()=>{for(const name of names)await tool(name,`#!/bin/sh\necho ${crate.crate}-${crate.version}-${host.platform}-${host.arch}\n`)(m.home,built);},{host,say:say(lines),environment:{SEMIO_LANGUAGE:"en"}});
 };
 const a:Built={count:0},linesA:string[]=[];
 const first=await run(one,darwin,a,linesA),firstBindgen=await run(one,darwin,a,linesA,bindgen);
 expect([first.source,first.published,firstBindgen.source,firstBindgen.published]).toEqual(["built",true,"built",true]);
 expect(a.count).toBe(1+3);
 expect(linesA.some(line=>line.includes("cargo-tool:trunk")&&line.includes("published to the fleet: true")&&line.includes("class darwin-arm64"))).toBe(true);
 const b:Built={count:0},linesB:string[]=[];
 const second=await run(two,darwin,b,linesB),secondBindgen=await run(two,darwin,b,linesB,bindgen);
 expect(b.count,"the second machine of the class installs nothing").toBe(0);
 expect([second.source,secondBindgen.source]).toEqual(["restored","restored"]);
 expect(second.files+secondBindgen.files).toBe(4);
 for(const name of ["trunk","wasm-bindgen","wasm-bindgen-test-runner","wasm2es6js"]){
  expect(readFileSync(join(two.home,"bin",name),"utf8")).toBe(readFileSync(join(one.home,"bin",name),"utf8"));
  expect(statSync(join(two.home,"bin",name)).mode&0o111,`${name} keeps its execute bit`).not.toBe(0);
 }
 expect(linesB.filter(line=>line.includes("restored from the fleet")).length).toBe(2);
 expect(linesB.every(line=>!line.includes("built here"))).toBe(true);
 const c:Built={count:0},linesC:string[]=[];
 const other=await run(three,linuxGnu,c,linesC);
 expect([other.source,other.published,other.class],"a class that nobody has built builds and uploads under its own class").toEqual(["built",true,"linux-x64-gnu"]);
 expect(c.count).toBe(1);
 const d:Built={count:0},second2=await run(four,linuxGnu,d,[]);
 expect([second2.source,d.count]).toEqual(["restored",0]);
 const e:Built={count:0},musl=await run(five,linuxMusl,e,[]);
 expect([musl.source,musl.class,e.count],"glibc and musl are two classes").toEqual(["built","linux-x64-musl",1]);
 const f:Built={count:0},newer=await run(six,darwin,f,[],{...trunk,version:"0.21.15"});
 expect([newer.source,f.count],"a pin change is a miss").toEqual(["built",1]);
 expect(readdirSync(join(spaceOf(store))).filter(name=>name.endsWith(".json")).length).toBe(5);
});

test("kinds: neutral outputs are shared by every host, posix outputs by macOS and Linux, host outputs by their exact class",async()=>{
 const store=directory("store");
 const output=(m:ReturnType<typeof machine>,kind:SetupKindV1):SetupOutputV1=>({task:`tree-${kind}`,inputs:{lock:"x"},kind,base:m.home,roots:["tree"]});
 const buildTree=(m:ReturnType<typeof machine>,built:Built)=>async()=>{
  mkdirSync(join(m.home,"tree","nested","deep"),{recursive:true});writeFileSync(join(m.home,"tree","nested","deep","a.txt"),"alpha\n".repeat(50));writeFileSync(join(m.home,"tree","b.bin"),Buffer.from([0,1,2,3,255]));
  writeFileSync(join(m.home,"tree","run.sh"),"#!/bin/sh\n");chmodSync(join(m.home,"tree","run.sh"),0o755);symlinkSync("nested/deep/a.txt",join(m.home,"tree","relative-link"));symlinkSync(join(m.home,"tree","b.bin"),join(m.home,"tree","absolute-link"));
  mkdirSync(join(m.home,"tree","empty"),{recursive:true});built.count++;
 };
 const results:Record<string,string[]>={};
 for(const kind of ["neutral","posix","host"] as SetupKindV1[]){
  const sequence:[SetupHostV1,string][]=[[darwin,"built"],[linuxGnu,kind==="host"?"built":"restored"],[{platform:"win32",arch:"x64"},kind==="neutral"?"restored":"built"],[darwin,"restored"]];
  results[kind]=[];
  for(const [host,expected] of sequence){
   const m=machine(store),built:Built={count:0},outcome=await fleetSetupOutputV1(m.context,output(m,kind),buildTree(m,built),{host,say:()=>{}});
   expect(outcome.source,`${kind} on ${host.platform}`).toBe(expected);
   expect(built.count).toBe(expected==="built"?1:0);
   results[kind]!.push(outcome.source);
   expect(readFileSync(join(m.home,"tree","nested","deep","a.txt"),"utf8")).toBe("alpha\n".repeat(50));
   expect(readlinkSync(join(m.home,"tree","relative-link"))).toBe("nested/deep/a.txt");
   expect(readlinkSync(join(m.home,"tree","absolute-link")),"an absolute link inside the base follows the new base").toBe(join(m.home,"tree","b.bin"));
   expect(existsSync(join(m.home,"tree","empty"))).toBe(true);
   expect(statSync(join(m.home,"tree","run.sh")).mode&0o111).not.toBe(0);
  }
 }
 expect(results).toEqual({neutral:["built","restored","restored","restored"],posix:["built","restored","built","restored"],host:["built","built","built","restored"]});
});

test("a restore is complete or a miss: a damaged block and a failing check never leave a partial output",async()=>{
 const store=directory("store"),one=machine(store);
 const spec=(m:ReturnType<typeof machine>):SetupOutputV1=>({task:"tree",inputs:{lock:"y"},kind:"host",base:m.home,roots:["tree/a","tree/b"]});
 const make=(m:ReturnType<typeof machine>,built:Built)=>async()=>{mkdirSync(join(m.home,"tree"),{recursive:true});writeFileSync(join(m.home,"tree","a"),"A".repeat(5000));writeFileSync(join(m.home,"tree","b"),"B".repeat(5000));built.count++;};
 const lines:string[]=[];
 expect((await fleetSetupOutputV1(one.context,spec(one),make(one,{count:0}),{host:darwin,say:say(lines)})).published).toBe(true);
 const two=machine(store),seen:string[]=[];
 const verifyFails=await fleetSetupOutputV1(two.context,spec(two),async()=>{seen.push(existsSync(join(two.home,"tree","a"))?"present":"absent");mkdirSync(join(two.home,"tree"),{recursive:true});writeFileSync(join(two.home,"tree","a"),"A".repeat(5000));writeFileSync(join(two.home,"tree","b"),"B".repeat(5000));},{host:darwin,say:say(lines),verify:async()=>false,environment:{SEMIO_LANGUAGE:"en"}});
 expect([verifyFails.source,verifyFails.reason],"a restore that fails its check is rebuilt here").toEqual(["built",undefined]);
 expect(seen,"the failed restore left nothing behind when the build started").toEqual(["absent"]);
 expect(lines.some(line=>line.includes("did not pass its check"))).toBe(true);
 const blobs:string[]=[];const walk=(path:string)=>{for(const name of readdirSync(path)){const child=join(path,name);if(lstatSync(child).isDirectory())walk(child);else blobs.push(child);}};walk(join(spaceOf(store),"blobs"));
 expect(blobs.length).toBeGreaterThanOrEqual(2);
 const three=machine(store);
 for(const blob of blobs){writeFileSync(blob,Buffer.concat([readFileSync(blob),Buffer.from("tail")]));}
 const damaged:Built={count:0},outcome=await fleetSetupOutputV1(three.context,spec(three),make(three,damaged),{host:darwin,say:()=>{}});
 expect(outcome.source,"a damaged block is a miss, the output is built here").toBe("built");
 expect(damaged.count).toBe(1);
 expect(readFileSync(join(three.home,"tree","a"),"utf8")).toBe("A".repeat(5000));
 expect(readdirSync(join(three.root,".cache","f")).includes("setup-tmp")?readdirSync(join(three.root,".cache","f","setup-tmp")):[],"no staging directory is left").toEqual([]);
});

test("a record that is not signed by a registered origin is refused and the output is built here",async()=>{
 const store=directory("store"),one=machine(store),two=machine(store);
 const spec=(m:ReturnType<typeof machine>):SetupOutputV1=>({task:"signed",inputs:{lock:"z"},kind:"host",base:m.home,roots:["bin/tool"]});
 const make=(m:ReturnType<typeof machine>,built:Built)=>tool("tool","tool-body\n")(m.home,built);
 const first:Built={count:0};
 expect((await fleetSetupOutputV1(one.context,spec(one),()=>make(one,first),{host:darwin,say:()=>{}})).published).toBe(true);
 const manifestName=readdirSync(spaceOf(store)).find(name=>name.endsWith(".json"))!,path=join(spaceOf(store),manifestName),manifest=JSON.parse(readFileSync(path,"utf8")) as {publisher:{signature:string}};
 manifest.publisher.signature=manifest.publisher.signature.split("").reverse().join("");writeFileSync(path,JSON.stringify(manifest)+"\n");
 const second:Built={count:0},outcome=await fleetSetupOutputV1(two.context,spec(two),()=>make(two,second),{host:darwin,say:()=>{}});
 expect([outcome.source,second.count],"a forged record is never restored").toEqual(["built",1]);
});

test("a machine without a store or a platform without a class builds locally and says why; the build is never skipped",async()=>{
 const none=fleetFetchContextV1({},undefined,{storeRoot:undefined,remote:undefined}),home=directory("home"),lines:string[]=[];
 const built:Built={count:0};
 const outcome=await fleetSetupOutputV1(none,{task:"t",inputs:{},kind:"host",base:home,roots:["x"]},async()=>{built.count++;},{host:darwin,say:say(lines),environment:{SEMIO_LANGUAGE:"de"}});
 expect([outcome.source,outcome.reason,built.count]).toEqual(["local","no fleet store on this machine",1]);
 expect(lines[0]).toContain("hier gebaut und lokal behalten");
 const store=directory("store"),m=machine(store);
 const odd=await fleetSetupOutputV1(m.context,{task:"t",inputs:{},kind:"host",base:m.home,roots:["x"]},async()=>{built.count++;},{host:{platform:"freebsd",arch:"x64"},say:()=>{}});
 expect([odd.source,odd.reason,built.count]).toEqual(["local","this platform has no shareable class",2]);
 const ready=await fleetSetupOutputV1(m.context,{task:"t",inputs:{},kind:"host",base:m.home,roots:["x"]},async()=>{built.count++;},{host:darwin,say:()=>{},ready:async()=>true});
 expect([ready.source,built.count]).toEqual(["present",2]);
 const empty=await fleetSetupOutputV1(m.context,{task:"u",inputs:{},kind:"host",base:m.home,roots:["never"]},async()=>{},{host:darwin,say:()=>{},environment:{SEMIO_LANGUAGE:"en"}});
 expect([empty.source,empty.published,empty.reason?.startsWith("the build left no output")]).toEqual(["local",false,true]);
 expect(existsSync(spaceOf(store)),"an output that was not built is never published").toBe(false);
});

test("placing a root is atomic: a reader polling while a root is replaced repeatedly never finds it absent",async()=>{
 const source=directory("source"),target=join(directory("target"),"bin","tool");
 writeFileSync(join(source,"tool"),"x".repeat(2_000_000));
 await placeSetupRootV1(join(source,"tool"),target);
 let absent=0,stop=false;
 const reader=(async()=>{while(!stop){if(!existsSync(target))absent++;await new Promise(resolve=>setImmediate(resolve));}})();
 for(let round=0;round<25;round++){writeFileSync(join(source,"tool"),String(round%10).repeat(1_000_000));await placeSetupRootV1(join(source,"tool"),target);}
 stop=true;await reader;
 expect(absent).toBe(0);expect(readFileSync(target,"utf8").length).toBe(1_000_000);
 expect(readdirSync(join(target,"..")).filter(name=>name!=="tool"),"no side file is left").toEqual([]);
});

test("the family table classifies every node of the closure of `setup`, as an independent Python walk of the root project finds it, and agrees with its own schema",async()=>{
 const root=[process.env.R11_LIVE??"",join(here,"../../../../../../../../../../")].find(path=>path!==""&&existsSync(join(path,"📋️project.json")));
 if(root===undefined)return;
 const rootProject=join(root,"📋️project.json");
 const code="import json,sys\np=json.load(open(sys.argv[1],encoding='utf8'))['targets'];seen=[]\ndef walk(n):\n    if n in seen:return\n    seen.append(n)\n    for d in (p.get(n,{}).get('dependsOn') or []):\n        if isinstance(d,str) and not d.startswith('^'):walk(d)\nwalk('setup')\nprint(json.dumps(seen))";
 const closure=JSON.parse(await python("-c",code,rootProject)) as string[];
 const classified=new Set(FLEET_SETUP_FAMILY_V1.map(row=>row.task));
 const missing=closure.filter(task=>!classified.has(task));
 expect(missing,"classify the new node in 🧰️setup/🧬️schema/🔣️.json: cacheable, lookup, upload, class, bound").toEqual([]);
 const stale=[...classified].filter(task=>!closure.includes(task));
 expect(stale,"a classified node that setup no longer depends on").toEqual([]);
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Family"},FLEET_SETUP_FAMILY_V1,schema as never);
 expect(errors).toEqual([]);
 for(const row of FLEET_SETUP_FAMILY_V1){
  if(row.status==="shared"||row.status==="wired")expect(row.lookup!=="none"&&row.upload!=="none",row.task).toBe(true);
  if(row.status==="local"||row.status==="no-output")expect(row.bound.length,`${row.task} says why it stays local`).toBeGreaterThan(0);
 }
});

test("texts exist in English and German with the same placeholders; the pinned channel is read from rust-toolchain.toml",()=>{
 for(const [name,row] of Object.entries(schema.definitions.Texts.const as Record<string,{en:string;de:string}>)){
  const holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(row.de),name).toEqual(holes(row.en));
 }
 expect(setupTextV1("present",{task:"x"},{SEMIO_LANGUAGE:"de"})).toBe("[setup] x: bereits vorhanden, nichts zu tun");
 expect(setupTextV1("present",{task:"x"},{})).toContain(" / ");
 expect(rustChannelV1('[toolchain]\nchannel = "nightly-2026-07-20"\ncomponents = ["rustfmt"]\n')).toBe("nightly-2026-07-20");
 const live=join(process.env.R11_LIVE??join(here,"../../../../../../../../../../"),"rust-toolchain.toml");
 if(existsSync(live))expect(rustChannelV1(readFileSync(live,"utf8"))).toMatch(/^[a-z0-9.-]+$/u);
});

test("the three generate tasks of the closure are uncacheable on purpose and say why: no cache flag with undeclared outputs is left, and the family table agrees with their project files",async()=>{
 const root=[process.env.R11_LIVE??"",join(here,"../../../../../../../../../../")].find(path=>path!==""&&existsSync(join(path,"📋️project.json")));
 if(root===undefined)return;
 const files={"@semio-tech/framework-schema:generate":"🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json","@semio-tech/ui-styling-tokens:generate":"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json","@semio-tech/framework-graph:generate":"🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📋️project.json"};
 const code="import json,sys\nt=json.load(open(sys.argv[1],encoding='utf8'))['targets']['generate']\nprint(json.dumps([t.get('cache'),t.get('outputs'),(t.get('metadata',{}).get('semio',{}).get('cache') or {})]))";
 for(const [task,file] of Object.entries(files)){
  const [cache,outputs,decision]=JSON.parse(await python("-c",code,join(root,file))) as [unknown,unknown,{class?:string;reason?:string}];
  expect(cache,task).toBe(false);expect(outputs,task).toEqual([]);expect(decision,task).toEqual({version:1,class:"side-effecting",reason:"mutates-workspace"});
  const row=FLEET_SETUP_FAMILY_V1.find(item=>item.task===task)!;
  expect(row.cacheable,task).toBe("no");expect(row.lookup,task).toBe("none");expect(row.bound,task).toContain("mutates-workspace");
 }
});
