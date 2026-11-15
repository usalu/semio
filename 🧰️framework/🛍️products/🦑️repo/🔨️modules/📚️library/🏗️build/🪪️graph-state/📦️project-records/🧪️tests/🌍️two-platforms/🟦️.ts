import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{expandRootV1,neutralizeRootV1}from"../../../🛰️exchange/🟦️.ts";
import{observationRowsHoldV1,withObservedWorldV1}from"../../../../👁️observation/🟨️.mjs";
import{createProjectFleetPortV1,projectRecordStateV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{dependencyGenerationV1,hostOfRootV1,localRowsV1,localizeRowsV1,neutralRowsV1,portableRowsV1,type ObservationRowV1}from"../../🧳️portable/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";

/** 🌍️ Contract: a project record produced on one platform and checkout is taken by a machine of any other platform at another checkout (macOS, Linux, Windows), with that machine's paths, separators, dependency generation, analysis providers and closure keys, and without the facts that name the producer's machine; a project that is bound to its host is never taken across hosts. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),`records-two-${label}-`)));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLAIN=JSON.stringify(["rev",null,[["authority"],"libs/alpha",{name:"alpha",targets:{build:{}}}]]),BOUND=JSON.stringify(["rev",null,[["authority"],"libs/emitter",{name:"emitter",targets:{build:{metadata:{semio:{hostPlatformOutputsV1:{version:1}}}}}}]]);
const GENERATION_A="86415d695aa883fffc22",GENERATION_B="8714ffcdb7fdf62c3f57";

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
function checkout(label:string,generation?:string){
 const root=temp(label);
 if(generation!==undefined){const modules=join(root,".cache","b","d","m","j",generation,"node_modules");mkdirSync(modules,{recursive:true});symlinkSync(modules,join(root,"node_modules"));}
 return root;
}
const above=(root:string)=>{const rows:{kind:string;path:string;fact:string}[]=[];for(let at=dirname(root);;at=dirname(at)){rows.push({kind:"lstat",path:at,fact:"directory"});if(dirname(at)===at)break;}return rows;};
const recordOf=(root:string,generation:string)=>({digest:"d".repeat(64),authority:[],observations:[...above(root),{kind:"lstat",path:root,fact:"directory"},{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}},{kind:"lstat",path:`${root}/.cache/b/d/m/j/${generation}/node_modules/typescript/lib/typescript.js`,fact:"file"},{kind:"field",path:`${root}/.cache/b/d/m/j/${generation}/node_modules/typescript/lib/typescript.js`,fact:{digest:"f".repeat(64),format:"package-identity",pointer:""}}],image:JSON.stringify({name:"alpha",root:"libs/alpha"})});
const infoOf=(root:string,keyText:string,store:{record?:unknown;imported?:unknown[]}):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText,exportRecord:()=>store.record,async import(portable){(store.imported??=[]).push(portable);return true;}});
type Row={kind:string;path:string;fact:unknown};

test("the state is the same for every platform and a host-bound project is keyed by the exact platform",()=>{
 const state=(keyText:string,platform:string)=>projectRecordStateV1({root:"/r",fleet:"libs/alpha",keyText,platform});
 expect(state(PLAIN,"darwin-arm64").digest).toBe(state(PLAIN,"linux-x64").digest);
 expect(state(PLAIN,"darwin-arm64").digest).toBe(state(PLAIN,"win32-x64").digest);
 expect(state(PLAIN,"darwin-arm64").frame.platform).toBe("all");
 expect(state(BOUND,"darwin-arm64").digest).not.toBe(state(BOUND,"linux-x64").digest);expect(state(BOUND,"linux-x64").frame.platform).toBe("linux-x64");expect(state(BOUND,"win32-x64").frame.platform).toBe("win32-x64");
 expect(projectRecordStateV1({root:"C:\\semio",fleet:"libs/alpha",keyText:PLAIN,platform:"win32-x64"}).digest).toBe(state(PLAIN,"linux-x64").digest);
});

test("a macOS record is taken by a Linux machine at another checkout: its paths, its own dependency generation, none of the producer's location rows; the original stays whole",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac",GENERATION_A),rootB=checkout("linux",GENERATION_B),store:{record?:unknown;imported?:unknown[]}={record:recordOf(rootA,GENERATION_A)},other:{record?:unknown;imported?:unknown[]}={};
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!;
  const gate=await mac.gate(infoOf(rootA,PLAIN,store));expect(gate.seeded).toBe(false);gate.publish();await mac.flush();
  const taken=await linux.gate(infoOf(rootB,PLAIN,other));expect(taken.seeded).toBe(true);expect(linux.stats().seeded).toBe(1);
  const record=other.imported![0] as {observations:Row[];image:string};
  const rows=record.observations,text=JSON.stringify(rows);
  expect(text).not.toContain(rootA);expect(text).not.toContain(GENERATION_A);
  expect(rows.some(row=>row.kind==="lstat"&&above(rootA).some(ancestor=>ancestor.path===row.path))).toBe(false);
  expect(rows).toContainEqual({kind:"lstat",path:rootB,fact:"directory"});
  expect(rows).toContainEqual({kind:"file",path:`${rootB}/libs/alpha/src/a.ts`,fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}});
  expect(rows).toContainEqual({kind:"lstat",path:`${rootB}/.cache/b/d/m/j/${GENERATION_B}/node_modules/typescript/lib/typescript.js`,fact:"file"});
  expect(rows.filter(row=>row.kind==="field").map(row=>row.path)).toEqual([`${rootB}/.cache/b/d/m/j/${GENERATION_B}/node_modules/typescript/lib/typescript.js`]);
  expect(record.image).toBe(JSON.stringify({name:"alpha",root:"libs/alpha"}));
  expect(JSON.stringify(store.record)).toContain(GENERATION_A);expect((store.record as {observations:Row[]}).observations.some(row=>row.path==="/")).toBe(true);
 }finally{await server.stop();}
},60000);

test("a machine without a dependency generation takes the record with rows that cannot hold, so it infers the project itself",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac2",GENERATION_A),rootB=checkout("bare"),store={record:recordOf(rootA,GENERATION_A)},other:{imported?:unknown[]}={};
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!;
  (await mac.gate(infoOf(rootA,PLAIN,store))).publish();await mac.flush();
  expect((await linux.gate(infoOf(rootB,PLAIN,other))).seeded).toBe(true);
  expect(JSON.stringify(other.imported)).toContain("generation-unavailable");
 }finally{await server.stop();}
},60000);

test("a host-bound project is never taken across hosts, a neutral one is taken by every platform",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=checkout("mac3",GENERATION_A),rootB=checkout("linux3",GENERATION_B),rootW=checkout("win3",GENERATION_B);
  const mac=createProjectFleetPortV1({root:rootA,environment,owner:"mac:1",platform:"darwin-arm64"})!,linux=createProjectFleetPortV1({root:rootB,environment,owner:"linux:1",platform:"linux-x64"})!,windows=createProjectFleetPortV1({root:rootW,environment,owner:"win:1",platform:"win32-x64"})!;
  const store={record:recordOf(rootA,GENERATION_A)};
  (await mac.gate(infoOf(rootA,BOUND,store))).publish();(await mac.gate(infoOf(rootA,PLAIN,store))).publish();await mac.flush();
  const bound=await linux.gate(infoOf(rootB,BOUND,{}));expect(bound.seeded).toBe(false);await bound.abandon();
  const boundWin=await windows.gate(infoOf(rootW,BOUND,{}));expect(boundWin.seeded).toBe(false);await boundWin.abandon();
  expect((await windows.gate(infoOf(rootW,PLAIN,{}))).seeded).toBe(true);
  expect((await linux.gate(infoOf(rootB,PLAIN,{}))).seeded).toBe(true);
 }finally{await server.stop();}
},60000);

test("the real replay of the observations holds in the tree of the taker with the portable rows and fails without the rewrite: the rows of a machine whose checkout lies elsewhere",()=>{
 const rootB=checkout("hold",GENERATION_B),PRODUCER="/zz-p14-producer/Users/Shared/semio",G=GENERATION_A,sha=(text:string)=>createHash("sha256").update(text).digest("hex");
 mkdirSync(join(rootB,"libs","alpha","src"),{recursive:true});writeFileSync(join(rootB,"libs","alpha","src","a.ts"),"abc");
 const lib=join(rootB,".cache","b","d","m","j",GENERATION_B,"node_modules","typescript","lib");mkdirSync(lib,{recursive:true});writeFileSync(join(lib,"typescript.js"),"x");
 const rows:ObservationRowV1[]=[{kind:"lstat",path:"/",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer/Users",fact:"directory"},{kind:"lstat",path:"/zz-p14-producer/Users/Shared",fact:"directory"},{kind:"lstat",path:PRODUCER,fact:"directory"},
  {kind:"file",path:`${PRODUCER}/libs/alpha/src/a.ts`,fact:{size:3,mtimeMs:1,ctimeMs:1,ino:1,sha256:sha("abc")}},{kind:"lstat",path:`${PRODUCER}/.cache/b/d/m/j/${G}/node_modules/typescript/lib/typescript.js`,fact:"file"},{kind:"fileExists",path:`${PRODUCER}/.cache/b/d/m/j/${G}/node_modules/typescript/lib/typescript.js`,fact:true}];
 const moved=(list:readonly ObservationRowV1[])=>JSON.parse(Buffer.from(expandRootV1(neutralizeRootV1(Buffer.from(JSON.stringify(list)),PRODUCER),rootB)).toString("utf8")) as ObservationRowV1[];
 const holds=(list:readonly ObservationRowV1[])=>withObservedWorldV1(undefined as never,()=>observationRowsHoldV1(list as never));
 expect(holds(moved(rows))).toBe(false);
 const shared=localRowsV1(moved(portableRowsV1(rows,PRODUCER)),dependencyGenerationV1(rootB));
 expect(holds(shared)).toBe(true);
 expect(holds(localRowsV1(moved(portableRowsV1(rows,PRODUCER)),undefined))).toBe(false);
});

const WINDOWS="C:\\semio",CLOSURE_RECIPE=(slash:string)=>({fn:"native-target",target:{options:{command:`repo "tool/script.ts" -- "${slash}/libs/alpha/script.ts"`}},projectRoot:"libs/alpha"});
const sortedJson=(value:unknown):unknown=>Array.isArray(value)?value.map(sortedJson):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sortedJson((value as Record<string,unknown>)[key])])):value;
const keyOf=(recipe:unknown)=>createHash("sha256").update(JSON.stringify(sortedJson(recipe))).digest("hex");
const PROVIDER="build:rust-references:v1:";
function recordAt(root:string,windows:boolean,generation:string,owner:string){
 const sep=windows?"\\":"/",slash=root.replaceAll("\\","/"),at=(...parts:string[])=>[root,...parts].join(sep),recipe=CLOSURE_RECIPE(slash),key=keyOf(recipe),module=at(".cache","b","d","m","j",generation,"node_modules","typescript","lib","typescript.js");
 return{digest:"d".repeat(64),authority:[],observations:[
  {kind:"lstat",path:windows?"C:\\":"/",fact:"directory"},{kind:"lstat",path:root,fact:"directory"},
  {kind:"file",path:at("libs","alpha","src","a.ts"),fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}},
  {kind:"exists",path:at("libs","alpha","Cargo.toml"),fact:true},
  {kind:"resolve",path:JSON.stringify([at("tool","x.mjs"),"typescript"]),fact:at(".cache","b","d","m","j","*","node_modules","typescript","lib","typescript.js")},
  {kind:"field",path:module,fact:{digest:"f".repeat(64),format:"package-identity",pointer:""}},
  {kind:"closure",path:JSON.stringify([root,key,["digest"]]),fact:"\""+"1".repeat(64)+"\""},
  {kind:"analysis",path:at("libs","alpha","src","lib.rs"),fact:{provider:PROVIDER+owner,revision:"r".repeat(64),resultSha256:"9".repeat(64),source:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1},nestedObservations:[]}},
 ],image:JSON.stringify({name:"alpha",root:"libs/alpha",targets:{build:{options:{command:`repo -- "${slash}/libs/alpha/script.ts"`},inputs:[{closureRef:key}]}},closureRecipes:{[key]:recipe}})};
}
type Imported={observations:Row[];image:string};
const portHost=(environment:Record<string,string>,root:string,owner:string,platform:string,generation:string|undefined,providers:string[],label:string)=>{
 const settings=graphFleetSettingsV1(temp(label),environment)!;
 return createProjectFleetPortV1({root,environment,client:graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:temp(label+"f")})}),owner,platform,generation:()=>generation,providers:async()=>providers})!;
};

test("a Windows record is taken by a macOS machine: POSIX paths and separators, the machine's generation, provider and closure keys, no drive and no backslash",async()=>{
 const {server,environment}=await world();
 try{
  const rootM=checkout("mac4",GENERATION_B),owner="a".repeat(64),takerOwner="b".repeat(64),store={record:recordAt(WINDOWS,true,GENERATION_A,owner)},got:{imported?:unknown[]}={};
  const windows=portHost(environment,WINDOWS,"win:1","win32-x64",GENERATION_A,[],"win"),mac=portHost(environment,rootM,"mac:1","darwin-arm64",GENERATION_B,[PROVIDER+takerOwner],"mac");
  (await windows.gate(infoOf(WINDOWS,PLAIN,store))).publish();await windows.flush();
  expect((await mac.gate(infoOf(rootM,PLAIN,got))).seeded).toBe(true);
  const record=got.imported![0] as Imported,text=JSON.stringify(record);
  expect(text).not.toContain("C:");expect(text).not.toContain(GENERATION_A);expect(record.observations.some(row=>row.path.includes("\\"))).toBe(false);
  const rows=record.observations;
  expect(rows).toContainEqual({kind:"lstat",path:rootM,fact:"directory"});
  expect(rows).toContainEqual({kind:"file",path:`${rootM}/libs/alpha/src/a.ts`,fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}});
  expect(rows.find(row=>row.kind==="resolve")).toEqual({kind:"resolve",path:JSON.stringify([`${rootM}/tool/x.mjs`,"typescript"]),fact:`${rootM}/.cache/b/d/m/j/*/node_modules/typescript/lib/typescript.js`});
  expect(rows.find(row=>row.kind==="field")!.path).toBe(`${rootM}/.cache/b/d/m/j/${GENERATION_B}/node_modules/typescript/lib/typescript.js`);
  expect((rows.find(row=>row.kind==="analysis")!.fact as{provider:string}).provider).toBe(PROVIDER+takerOwner);
  const image=JSON.parse(record.image) as {closureRecipes:Record<string,unknown>;targets:{build:{inputs:{closureRef:string}[]}}},[key]=Object.keys(image.closureRecipes);
  expect(key).toBe(keyOf(CLOSURE_RECIPE(rootM)));expect(image.targets.build.inputs).toEqual([{closureRef:key}]);
  expect(JSON.parse(rows.find(row=>row.kind==="closure")!.path)).toEqual([rootM,key,["digest"]]);
  expect(mac.stats().seeded).toBe(1);
 }finally{await server.stop();}
},60000);

test("a macOS record is taken by a Windows machine: its root, backslashes in row paths and forward slashes in the texts, its own provider and closure keys; a taker without a provider takes rows that cannot hold",async()=>{
 const {server,environment}=await world();
 try{
  const rootM=checkout("mac5",GENERATION_A),taker="D:\\work\\semio",store={record:recordAt(rootM,false,GENERATION_A,"a".repeat(64))},got:{imported?:unknown[]}={},bare:{imported?:unknown[]}={};
  const mac=portHost(environment,rootM,"mac:1","darwin-arm64",GENERATION_A,[],"mac"),windows=portHost(environment,taker,"win:1","win32-x64","c6d139b55ee3b931e7e4",[PROVIDER+"c".repeat(64)],"win"),starved=portHost(environment,"E:\\semio","win:2","win32-x64",undefined,[],"win2");
  (await mac.gate(infoOf(rootM,PLAIN,store))).publish();await mac.flush();
  expect((await windows.gate(infoOf(taker,PLAIN,got))).seeded).toBe(true);
  const record=got.imported![0] as Imported,rows=record.observations;
  expect(rows).toContainEqual({kind:"file",path:"D:\\work\\semio\\libs\\alpha\\src\\a.ts",fact:{sha256:"e".repeat(64),size:3,mtimeMs:1,ctimeMs:1,ino:1}});
  expect(rows.find(row=>row.kind==="field")!.path).toBe("D:\\work\\semio\\.cache\\b\\d\\m\\j\\c6d139b55ee3b931e7e4\\node_modules\\typescript\\lib\\typescript.js");
  expect(JSON.parse(rows.find(row=>row.kind==="resolve")!.path)).toEqual(["D:\\work\\semio\\tool\\x.mjs","typescript"]);
  expect(rows.some(row=>row.kind==="lstat"&&row.path==="/")).toBe(false);
  expect(record.image).toContain("D:/work/semio/libs/alpha/script.ts");expect(record.image).not.toContain(rootM);
  expect((rows.find(row=>row.kind==="analysis")!.fact as{provider:string}).provider).toBe(PROVIDER+"c".repeat(64));
  expect((await starved.gate(infoOf("E:\\semio",PLAIN,bare))).seeded).toBe(true);
  expect(JSON.stringify(bare.imported)).toContain("generation-unavailable");expect(JSON.stringify(bare.imported)).toContain("analysis-owner");
 }finally{await server.stop();}
},60000);

test("a record that names the root in a spelling that cannot be mapped is not shared and said so, and a taken record that did not hold is counted as inferred again",async()=>{
 const {server,environment}=await world();
 try{
  const owner="a".repeat(64),broken=recordAt(WINDOWS,true,GENERATION_A,owner);
  const image=JSON.parse(broken.image) as {targets:{build:{options:{command:string}}}};image.targets.build.options.command="repo -- C:\\semio\\libs";(broken as {image:string}).image=JSON.stringify(image);
  const windows=portHost(environment,WINDOWS,"win:1","win32-x64",GENERATION_A,[],"win");
  (await windows.gate(infoOf(WINDOWS,PLAIN,{record:broken}))).publish();await windows.flush();
  expect(windows.stats().refused).toBe(1);expect(windows.stats().published).toBe(0);
  (await windows.gate(infoOf(WINDOWS,PLAIN,{record:recordAt(WINDOWS,true,GENERATION_A,owner)}))).publish();await windows.flush();expect(windows.stats().published).toBe(1);
  const rootM=checkout("mac6",GENERATION_B),mac=portHost(environment,rootM,"mac:1","darwin-arm64",GENERATION_B,[],"mac"),gate=await mac.gate(infoOf(rootM,PLAIN,{}));
  expect(gate.seeded).toBe(true);gate.publish();await mac.flush();expect(mac.stats().reinferred).toBe(1);
  expect((await mac.flush())).toContain("1 did not hold here");
 }finally{await server.stop();}
},60000);

test("thirty neutral projects of a macOS machine are taken by a Windows machine with equal digests, the bound ones are refused",async()=>{
 const {server,environment}=await world();
 try{
  const rootM=checkout("mac7",GENERATION_A),taker="C:\\semio",mac=portHost(environment,rootM,"mac:1","darwin-arm64",GENERATION_A,[],"mac"),windows=portHost(environment,taker,"win:1","win32-x64","c6d139b55ee3b931e7e4",[],"win"),digests:string[]=[];
  const key=(index:number,bound:boolean)=>JSON.stringify(["rev",null,[["authority"],`libs/p${index}`,{name:`p${index}`,targets:{build:bound?{metadata:{semio:{hostPlatformOutputsV1:{version:1}}}}:{}}}]]);
  for(let index=0;index<30;index++){
   const info={...infoOf(rootM,key(index,index%10===9),{record:recordAt(rootM,false,GENERATION_A,"a".repeat(64))}),fleet:`libs/p${index}`};
   (await mac.gate(info)).publish();
  }
  await mac.flush();
  let taken=0,refused=0;
  for(let index=0;index<30;index++){
   const got:{imported?:unknown[]}={},gate=await windows.gate({...infoOf(taker,key(index,index%10===9),got),fleet:`libs/p${index}`});
   if(gate.seeded){taken++;digests.push((got.imported![0] as{digest:string}).digest);}else{refused++;await gate.abandon();}
  }
  expect(taken).toBe(27);expect(refused).toBe(3);expect(new Set(digests)).toEqual(new Set(["d".repeat(64)]));
 }finally{await server.stop();}
},120000);

test("the rows of a Windows machine, spelled for a macOS tree, hold in that tree through the real replay of the observations",()=>{
 const rootM=checkout("hold2",GENERATION_B),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
 mkdirSync(join(rootM,"libs","alpha","src"),{recursive:true});writeFileSync(join(rootM,"libs","alpha","src","a.ts"),"abc");
 const lib=join(rootM,".cache","b","d","m","j",GENERATION_B,"node_modules","typescript","lib");mkdirSync(lib,{recursive:true});writeFileSync(join(lib,"typescript.js"),"x");
 const rows:ObservationRowV1[]=[{kind:"lstat",path:"C:\\",fact:"directory"},{kind:"lstat",path:WINDOWS,fact:"directory"},{kind:"file",path:"C:\\semio\\libs\\alpha\\src\\a.ts",fact:{size:3,mtimeMs:1,ctimeMs:1,ino:1,sha256:sha("abc")}},{kind:"exists",path:"c:\\SEMIO\\libs\\alpha\\src\\a.ts",fact:true},{kind:"fileExists",path:`C:\\semio\\.cache\\b\\d\\m\\j\\${GENERATION_A}\\node_modules\\typescript\\lib\\typescript.js`,fact:true},{kind:"exists",path:"C:\\semio\\libs\\alpha\\absent.txt",fact:false}];
 const holds=(list:readonly ObservationRowV1[])=>withObservedWorldV1(undefined as never,()=>observationRowsHoldV1(list as never));
 expect(holds(rows)).toBe(false);
 const neutral=neutralRowsV1(rows,hostOfRootV1(WINDOWS)),taken=localizeRowsV1(neutral,hostOfRootV1(rootM),{generation:dependencyGenerationV1(rootM)});
 expect(JSON.stringify(taken)).not.toContain("C:");
 expect(holds(taken)).toBe(true);
 expect(holds(localizeRowsV1(neutral,hostOfRootV1(rootM),{}))).toBe(false);
});
