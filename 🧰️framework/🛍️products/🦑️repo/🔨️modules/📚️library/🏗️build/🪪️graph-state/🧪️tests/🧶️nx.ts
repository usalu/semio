import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {encodeGraphRecordsV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";
import budget from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json" with {type:"json"};
import {nxGenerationHoldsV1,nxGenerationIdentityV1,type NxGenerationClaimV1} from "../../../⚡️caching/🕸️graph/🔏️seal/📌️generation/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🌐️remote-cache/🖥️server/🟦️.ts";
import {GRAPH_STATE_POLICY_V1} from "../🟦️.ts";
import {graphEntryKeyV1} from "../🛰️exchange/🟦️.ts";
import {beginNxFleetV1,collectNxGraphV1,graphFleetSettingsV1,installNxGraphV1,nxGraphStateV1,readNxGraphCostV1,rootlessGraphDigestV1} from "../🧶️nx/🟦️.ts";

/** 🧶️ The Nx graph exchange: the portable graph digest is the same at every root, a graph computed on one machine is installed on another with that machine's root, and a second cold start finds the claim held and the graph stored. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`graph-nx-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

function portableGraph(root:string,edit?:string){
 const project=(name:string)=>({name,root:`libs/${name}`,targets:{build:{executor:"repo:script",options:{command:`repo ${root}/🧰️framework/${name}/📜️script.ts build`,cwd:`${root}/libs/${name}`,env:{SEMIO_PLUGIN_CATALOG:`${root}/a.json:${root}/b.json`}}}}});
 const names=["alpha","beta","gamma"],projects:Record<string,unknown>={};
 for(const name of names)projects[name]=project(name);
 if(edit)(projects.beta as {targets:{build:{options:{command:string}}}}).targets.build.options.command+=" "+edit;
 return {version:1,namedInputs:{default:["{projectRoot}/**/*"]},admission:{root},projects,dependencies:[{source:"alpha",target:"beta"}],externalNodes:{"npm:x":{name:"npm:x",data:{root:`${root}/node_modules/x`}}}};
}

async function snapshot(root:string,edit?:string){
 const directory=temp("snap"),encoded=await encodeGraphRecordsV1(portableGraph(root,edit),{policy:budget.records as never,maximumBytes:1<<26});
 writeFileSync(join(directory,encoded.digest+".json"),encoded.manifestBytes);
 for(const [digest,bytes] of encoded.records)writeFileSync(join(directory,digest+".gz"),bytes);
 return {manifest:join(directory,encoded.digest+".json"),digest:encoded.digest};
}

test("the portable graph digest equals at every root and differs when a project differs",async()=>{
 const rootA="/Users/Shared/semio",rootB="/srv/other-root-2",a=await snapshot(rootA),b=await snapshot(rootB),changed=await snapshot(rootA,"--extra");
 expect(a.digest).not.toBe(b.digest);
 const memo=join(temp("memo"),"records.json");
 const left=await rootlessGraphDigestV1(a.manifest,rootA,memo),right=await rootlessGraphDigestV1(b.manifest,rootB),other=await rootlessGraphDigestV1(changed.manifest,rootA,memo);
 expect(left).toMatch(/^[0-9a-f]{64}$/u);expect(left).toBe(right);expect(other).not.toBe(left);
 expect(await rootlessGraphDigestV1(a.manifest,rootA,memo)).toBe(left);
 expect(JSON.parse(readFileSync(memo,"utf8")).scope).toMatch(/^[0-9a-f]{16}$/u);
 expect(await rootlessGraphDigestV1(join(temp("none"),"missing.json"),rootA)).toBeUndefined();
});

function workspaceData(root:string,label:string){
 const directory=join(temp("data"),"ns");mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,"project-graph.json"),JSON.stringify({nodes:{alpha:{data:{cwd:`${root}/libs/alpha`,label}}},computedAt:1}));
 writeFileSync(join(directory,"file-map.json"),JSON.stringify({files:[`${root}/x`],label}));
 writeFileSync(join(directory,"source-maps.json"),JSON.stringify({maps:label}));
 return directory;
}

async function claimOf(root:string,graph:string):Promise<NxGenerationClaimV1>{return {graphDigest:graph,implementation:sha("implementation"),tooling:sha("tooling"),identity:await nxGenerationIdentityV1(root)};}

test("collect and install round trip across roots and refuse a foreign document set",async()=>{
 const rootA=temp("rootA"),rootB=temp("rootB"),source=workspaceData(rootA,"one"),documents=(await collectNxGraphV1(source,rootA))!;
 expect(documents.map(document=>document.name).sort()).toEqual(["nx/file-map.json","nx/project-graph.json","nx/source-maps.json"]);
 expect(Buffer.from(documents[0]!.bytes).toString()).not.toContain(rootA);
 const target=join(temp("install"),"ns");
 expect(await installNxGraphV1(target,documents,rootB)).toBeGreaterThan(50);
 expect(JSON.parse(readFileSync(join(target,"project-graph.json"),"utf8")).nodes.alpha.data.cwd).toBe(`${rootB}/libs/alpha`);
 expect(await installNxGraphV1(join(target,"x"),documents.slice(0,2),rootB)).toBeUndefined();
 expect(await installNxGraphV1(join(target,"y"),[...documents.slice(0,2),{name:"nx/evil.json",bytes:new Uint8Array(1)}],rootB)).toBeUndefined();
 expect(await collectNxGraphV1(join(temp("empty")),rootA)).toBeUndefined();
 expect(existsSync(join(target,"project-graph.json.fleet"))).toBe(false);
});

test("the first machine computes and publishes, the second installs the graph with its own root and a valid generation receipt",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const rootA=temp("m1"),rootB=temp("m2"),snapA=await snapshot(rootA),snapB=await snapshot(rootB),environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  expect(graphFleetSettingsV1(rootA,{})).toBeUndefined();
  expect(graphFleetSettingsV1(rootA,{...environment,SEMIO_FLEET_GRAPH:"off"})).toBeUndefined();
  const settings=graphFleetSettingsV1(rootA,environment)!;
  expect(settings.upload).toBe(true);expect(settings.token).toBe("w");
  const dataA=workspaceData(rootA,"computed on one"),claimA=await claimOf(rootA,snapA.digest),messages:string[]=[];
  const first=await beginNxFleetV1({root:rootA,environment,claim:claimA,directory:dataA,manifestFile:snapA.manifest,settings,owner:"m1:1",say:(key,fields)=>messages.push(key+":"+fields.kind)});
  expect(first.installed).toBe(false);expect(first.state).toBeDefined();
  const rootless=(await rootlessGraphDigestV1(snapA.manifest,rootA))!,state=nxGraphStateV1(claimA,rootless);
  expect(first.state!.digest).toBe(state.digest);
  const dataB=join(temp("dataB"),"ns"),claimB=await claimOf(rootB,snapB.digest),secondSettings=graphFleetSettingsV1(rootB,environment)!;
  const racing=beginNxFleetV1({root:rootB,environment,claim:claimB,directory:dataB,manifestFile:snapB.manifest,settings:secondSettings,waitLimitMilliseconds:20000,owner:"m2:2",say:(key,fields)=>messages.push("B "+key+":"+JSON.stringify(fields))});
  await Bun.sleep(400);
  await first.finish(true);
  const second=await racing;
  expect(await readNxGraphCostV1(rootA)).toBeGreaterThan(300);expect(await readNxGraphCostV1(rootB)).toBeUndefined();
  expect(messages).toContain("published:nx-graph");expect(messages.some(message=>message.startsWith("B waited"))).toBe(true);expect(messages.some(message=>message.startsWith("B hit"))).toBe(true);
  expect(second.installed).toBe(true);expect(second.state!.digest).toBe(state.digest);
  expect(JSON.parse(readFileSync(join(dataB,"project-graph.json"),"utf8")).nodes.alpha.data.cwd).toBe(`${rootB}/libs/alpha`);
  expect(readFileSync(join(dataB,"file-map.json"),"utf8")).toContain(`${rootB}/x`);
  expect(await nxGenerationHoldsV1(rootB,dataB,claimB)).toBe(true);
  const third=await beginNxFleetV1({root:rootB,environment,claim:claimB,directory:join(temp("dataC"),"ns"),manifestFile:snapB.manifest,settings:secondSettings});
  expect(third.installed).toBe(true);
  const changed=await snapshot(rootB,"--extra"),fourth=await beginNxFleetV1({root:rootB,environment,claim:await claimOf(rootB,changed.digest),directory:join(temp("dataD"),"ns"),manifestFile:changed.manifest,settings:secondSettings,waitLimitMilliseconds:50});
  expect(fourth.installed).toBe(false);await fourth.finish(false);
  expect(graphEntryKeyV1(state)).toMatch(/^[0-9a-f]{64}$/u);expect(GRAPH_STATE_POLICY_V1.slotDomain).toBe("graph-state");
 }finally{await server.stop();}
},90000);

test("a dead cache or a missing snapshot leaves the run to compute as before",async()=>{
 const root=temp("solo"),snap=await snapshot(root),claim=await claimOf(root,snap.digest),dead={endpoint:"http://127.0.0.1:9",instance:"main",token:"w",upload:true,stateDirectory:temp("dead")};
 const first=await beginNxFleetV1({root,environment:{},claim,directory:join(temp("d"),"ns"),manifestFile:snap.manifest,settings:dead});
 expect(first.installed).toBe(false);await first.finish(true);
 const second=await beginNxFleetV1({root,environment:{},claim,directory:join(temp("d"),"ns"),manifestFile:undefined,settings:dead});
 expect(second.installed).toBe(false);expect(second.state).toBeUndefined();
 const third=await beginNxFleetV1({root,environment:{},claim,directory:join(temp("d"),"ns"),manifestFile:snap.manifest});
 expect(third.installed).toBe(false);
});

test("the digest of generated, ignored files equals a Python model, ignores the root and the times, and sees every change",async()=>{
 const {generatedInputsDigestV1}=await import("../🧾️generated/🟦️.ts"),python=(await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts")).testPythonV1();
 const make=(label:string,extra?:string)=>{
  const root=temp(label);
  for(const [path,text] of Object.entries({"a/🤖️generated/🔌️plugins.json":"{\"p\":1}","a/🤖️generated/sub/🎠️x.json":"[1,2]","a/🤖️generated/zeta.txt":"z","b/other.txt":"not inside"}))mkdirSync(join(root,path,".."),{recursive:true}),writeFileSync(join(root,path),text);
  if(extra)writeFileSync(join(root,"a/🤖️generated/zeta.txt"),extra);
  return root;
 };
 const rootA=make("ga"),rootB=make("gb"),memo=join(temp("gm"),"memo.json");
 const first=await generatedInputsDigestV1(rootA,["a/🤖️generated"],memo),again=await generatedInputsDigestV1(rootA,["a/🤖️generated/"],memo);
 expect(first.files).toBe(3);expect(first.read).toBe(3);expect(again.digest).toBe(first.digest);expect(again.memoised).toBe(3);expect(again.read).toBe(0);
 expect((await generatedInputsDigestV1(rootB,["a/🤖️generated"])).digest).toBe(first.digest);
 const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
 const run=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️oracle/🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify({generated:[[["a/🤖️generated/sub/🎠️x.json",sha("[1,2]")],["a/🤖️generated/zeta.txt",sha("z")],["a/🤖️generated/🔌️plugins.json",sha("{\"p\":1}")]]]})),stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);expect(JSON.parse(run.stdout.toString())[0]).toBe(first.digest);
 expect((await generatedInputsDigestV1(make("gc","changed"),["a/🤖️generated"])).digest).not.toBe(first.digest);
 expect((await generatedInputsDigestV1(rootA,["a/missing"])).files).toBe(0);
 writeFileSync(join(rootA,"a/🤖️generated/new.json"),"{}");
 expect((await generatedInputsDigestV1(rootA,["a/🤖️generated"],memo)).digest).not.toBe(first.digest);
});

const nxModules=process.env.SEMIO_TEST_NX_NODE_MODULES;

test.skipIf(!nxModules||process.platform==="win32")("real Nx: the graph files computed at two roots are equal after neutralizing, and Nx reuses graph files installed from the other root instead of recomputing",async()=>{
 const {symlinkSync}=await import("node:fs"),base=temp("realnx");
 const workspace=(directory:string)=>{
  mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"package.json"),"{\"name\":\"ws\",\"private\":true}");writeFileSync(join(directory,"nx.json"),"{\"analytics\":false}");
  for(const name of ["a","b","c"]){mkdirSync(join(directory,"libs",name),{recursive:true});writeFileSync(join(directory,"libs",name,"project.json"),JSON.stringify({name:"proj-"+name,root:"libs/"+name,targets:{build:{command:"echo libs/"+name,options:{cwd:"{workspaceRoot}/libs/"+name}}}}));writeFileSync(join(directory,"libs",name,"f.txt"),"x");}
  symlinkSync(nxModules!,join(directory,"node_modules"));Bun.spawnSync(["git","init","-q"],{cwd:directory});
 };
 const rootA=join(base,"A"),rootB=join(base,"B");workspace(rootA);workspace(rootB);
 const nx=async(cwd:string,data:string,reuse:boolean)=>{
  const child=Bun.spawn(["node",join(nxModules!,"nx/dist/bin/nx.js"),"show","projects"],{cwd,env:{...process.env,NX_DAEMON:"false",NX_WORKSPACE_DATA_DIRECTORY:data,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",...(reuse?{NX_FORCE_REUSE_CACHED_GRAPH:"true"}:{})},stdout:"pipe",stderr:"pipe"});
  const out=await new Response(child.stdout).text();await child.exited;return JSON.parse(out) as string[];
 };
 expect(await nx(rootA,join(base,"dataA"),false)).toEqual(["proj-a","proj-b","proj-c"]);expect(await nx(rootB,join(base,"dataB"),false)).toEqual(["proj-a","proj-b","proj-c"]);
 const left=(await collectNxGraphV1(join(base,"dataA"),rootA))!,right=(await collectNxGraphV1(join(base,"dataB"),rootB))!,strip=(text:string)=>text.replace(/"computedAt": \d+/gu,"");
 for(const name of ["nx/project-graph.json","nx/source-maps.json"])expect(strip(Buffer.from(left.find(d=>d.name===name)!.bytes).toString())).toBe(strip(Buffer.from(right.find(d=>d.name===name)!.bytes).toString()));
 writeFileSync(join(rootB,"libs/a/project.json"),JSON.stringify({name:"proj-a-changed",root:"libs/a",targets:{}}));
 await installNxGraphV1(join(base,"installed"),left,rootB);
 expect(await nx(rootB,join(base,"installed"),true)).toEqual(["proj-a","proj-b","proj-c"]);
 expect(await nx(rootB,join(base,"installed"),false)).toEqual(["proj-a-changed","proj-b","proj-c"]);
},120000);
