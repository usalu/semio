/** 🧪️ Tests for MVCC revisions: frozen content under concurrent edits, Merkle digests equal to an independent Python tarball oracle, cross-workspace result sharing, pins as GC roots. */
import {spawnSync} from "node:child_process";
import * as nodeFs from "node:fs";
import {chmodSync,mkdirSync,readdirSync,mkdtempSync,readFileSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {importObservedModuleV1,observedFs} from "../../👁️observation/🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {requireTestToolV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {casUsageV1,createCasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {readCasPinsV1} from "../../🗄️cas/📌️pins/🟦️.ts";
import {actionKeyV1,buildTreeV1,compareCodePointsV1,flattenTreeV1,loadManifestV1,materializeRevisionV1,createPinnedFsV1,pinModuleV1,withCapturedPlanV1,openPinnedWorldSyncV1,openPinnedWorldV1,pinRevisionV1,releasePinV1,renewPinV1,revisionDigestV1,revisionErrorV1,revisionTextV1,stageRevisionV1,withPinnedRevisionV1,REVISION_POLICY_V1,type ManifestEntryV1} from "../🟦️.ts";

const directories:string[]=[];
const scratch=(name:string):string=>{const path=mkdtempSync(join(tmpdir(),`rev-${name}-`));directories.push(path);return path;};
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const python=requireTestToolV1("python"),oracle=join(import.meta.dir,"🐍️oracle.py");
const ask=(request:Record<string,unknown>):any=>{const run=spawnSync(python,["-I",oracle],{input:JSON.stringify(request),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout||"{}");};
const ajv=new Ajv({strict:false,allowUnionTypes:true}),validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const store=(name:string)=>createCasStoreV1({root:scratch(name)});
type Fixture={name:string;files:Record<string,{text:string;executable?:boolean}>;paths:string[];absent?:string[]};
const write=(root:string,files:Fixture["files"]):void=>{for(const [path,file] of Object.entries(files)){const full=join(root,...path.split("/"));mkdirSync(dirname(full),{recursive:true});writeFileSync(full,file.text);chmodSync(full,file.executable?0o755:0o644);}};
const workspaceOf=(fixture:Fixture,name:string):string=>{const root=scratch(name);write(root,fixture.files);return root;};
const options=(cas:ReturnType<typeof store>,root:string,paths:string[],extra:Record<string,unknown>={})=>({cas,root,workspace:"w1",paths,holder:"agent-a",journalCursor:"c1",...extra}) as Parameters<typeof pinRevisionV1>[0];
const bytesOf=(value:Uint8Array|undefined):string=>Buffer.from(value!).toString("utf8");

test("the policy, every fixture, a pin and the messages satisfy the closed schema (Ajv)",async()=>{
 expect(validator("Policy")(REVISION_POLICY_V1)).toBe(true);
 const cas=store("schema"),fixture=fixtures.fixtures[0] as Fixture,root=workspaceOf(fixture,"schema-ws");
 for(const row of fixtures.fixtures)expect(validator("Fixture")(row),JSON.stringify(validator("Fixture").errors)).toBe(true);
 const pinned=await pinRevisionV1(options(cas,root,fixture.paths));
 expect(validator("Pin")(pinned.pin)).toBe(true);
 expect(validator("Revision")({journalCursor:pinned.revision.journalCursor,roots:pinned.revision.roots,workspace:pinned.revision.workspace})).toBe(true);
 expect(validator("InputManifest")(pinned.manifest),JSON.stringify(validator("InputManifest").errors)).toBe(true);
 for(const bad of ["../x","/abs","a//b","a\\b","a/./b",""])expect(validator("Path")(bad),bad).toBe(false);
 for(const key of Object.keys(schema.definitions.Messages.const))expect(Object.keys((schema.definitions.Messages.const as any)[key]).sort()).toEqual(["de","en"]);
 expect(revisionTextV1("pinned","de",{revision:"r",files:1,reused:0,staged:1,torn:0})).toContain("gepinnt");
});

test("manifest, tree digest and action key equal the independent Python tarball oracle for every fixture",async()=>{
 for(const fixture of fixtures.fixtures as Fixture[]){
  const root=workspaceOf(fixture,"oracle"),tar=join(scratch("tar"),"snapshot.tar"),cas=store("oracle-cas");
  ask({op:"snapshot",root,paths:fixture.paths,tar});
  const pinned=await pinRevisionV1(options(cas,root,fixture.paths)),expected=ask({op:"digest",tar});
  expect(pinned.manifest.digest,fixture.name).toBe(expected.digest);
  expect([...pinned.manifest.entries],fixture.name).toEqual(expected.entries);
  expect([...pinned.absent]).toEqual(fixture.absent??[]);
  const spec={tool:"tsc",command:["tsc","-p","."],environment:{B:"2",A:"é"},manifest:expected.digest};
  expect(actionKeyV1(spec)).toBe(ask({op:"action",...spec}).key);
 }
});

test("symbolic links are inputs whose digest is their target text, equal to the oracle",async()=>{
 if(process.platform==="win32")return;
 const root=scratch("link"),cas=store("link-cas");
 write(root,{"real.txt":{text:"r\n"}});symlinkSync("real.txt",join(root,"alias.txt"));
 const tar=join(scratch("link-tar"),"s.tar");ask({op:"snapshot",root,paths:["real.txt","alias.txt"],tar});
 const pinned=await pinRevisionV1(options(cas,root,["real.txt","alias.txt"]));
 expect(pinned.manifest.digest).toBe(ask({op:"digest",tar}).digest);
 const target=scratch("link-out");await materializeRevisionV1(cas,pinned.revision.id,target);
 expect(readFileSync(join(target,"alias.txt"),"utf8")).toBe("r\n");
});

test("code point order differs from UTF-16 order for astral names and the tree sorts by code point",()=>{
 expect(["ｚ","𠀀"].sort()).toEqual(["𠀀","ｚ"]);
 expect(["ｚ","𠀀"].sort(compareCodePointsV1)).toEqual(["ｚ","𠀀"]);
 expect(compareCodePointsV1("a","a")).toBe(0);expect(compareCodePointsV1("ab","a")).toBe(1);expect(compareCodePointsV1("a","ab")).toBe(-1);
});

test("an edit after the pin never changes the pinned content, manifest, materialised files or action key",async()=>{
 const fixture=fixtures.fixtures[0] as Fixture,root=workspaceOf(fixture,"frozen"),cas=store("frozen-cas");
 const tar=join(scratch("frozen-tar"),"r.tar");ask({op:"snapshot",root,paths:fixture.paths,tar});
 const pinned=await pinRevisionV1(options(cas,root,fixture.paths)),key=actionKeyV1({tool:"t",command:["c"],manifest:pinned.manifest.digest});
 writeFileSync(join(root,"src/main.ts"),"export const a=999;\n");rmSync(join(root,"package.json"));writeFileSync(join(root,"new.ts"),"n\n");
 const world=await openPinnedWorldV1(cas,pinned.revision.id,{verify:true});
 expect(bytesOf(await world.read("src/main.ts"))).toBe("export const a=1;\n");
 expect(bytesOf(await world.read("package.json"))).toBe("{\"name\":\"a\"}\n");
 expect(await world.read("new.ts")).toBeUndefined();
 expect(await world.files("src")).toEqual(["src/main.ts","src/𠀀.ts","src/ｚ.ts","src/🧊️/x.ts"].sort(compareCodePointsV1));
 const out=scratch("frozen-out"),placed=await materializeRevisionV1(cas,pinned.revision.id,out);
 expect(placed).toBe(pinned.manifest.entries.length);
 expect(readFileSync(join(out,"src/main.ts"),"utf8")).toBe("export const a=1;\n");
 if(process.platform!=="win32")expect(statSync(join(out,"bin/run.sh")).mode&0o111).not.toBe(0);
 expect(ask({op:"digest",tar}).digest).toBe(pinned.manifest.digest);
 const reloaded=await loadManifestV1(cas,pinned.revision.id);
 expect(reloaded.manifest).toEqual(pinned.manifest);
 expect(actionKeyV1({tool:"t",command:["c"],manifest:reloaded.manifest.digest})).toBe(key);
 const next=await pinRevisionV1(options(cas,root,[...fixture.paths,"new.ts"],{parent:pinned,dirty:{paths:["src/main.ts","package.json","new.ts"]},journalCursor:"c2"}));
 expect(next.manifest.digest).not.toBe(pinned.manifest.digest);
 expect(next.revision.parent).toBe(pinned.revision.id);
 expect(next.reused).toBe(fixture.paths.length-1-3+1);
});

test("an edit at each phase of staging yields a valid revision equal to some frozen snapshot of that file",async()=>{
 const phases=["beforeRead","afterRead"] as const;
 for(const phase of phases){
  const root=scratch(`phase-${phase}`),cas=store(`phase-${phase}-cas`);
  write(root,{"a.txt":{text:"old\n"},"b.txt":{text:"stable\n"}});
  let fired=false;
  const hook=(path:string):void=>{if(path==="a.txt"&&!fired){fired=true;writeFileSync(join(root,"a.txt"),"edited-longer\n");}};
  const pinned=await pinRevisionV1(options(cas,root,["a.txt","b.txt"],{io:{[phase]:hook}}));
  const world=await openPinnedWorldV1(cas,pinned.revision.id,{verify:true}),seen=bytesOf(await world.read("a.txt"));
  expect(["old\n","edited-longer\n"]).toContain(seen);
  expect(pinned.manifest.entries.find(entry=>entry.path==="a.txt")!.digest).toBe(revisionDigestV1(seen));
  expect(pinned.torn).toEqual([]);
  if(phase==="afterRead")expect(seen).toBe("edited-longer\n");
 }
});

test("a file that keeps changing is recorded with the content actually read, after bounded attempts",async()=>{
 const root=scratch("torn"),cas=store("torn-cas");write(root,{"a.txt":{text:"0\n"}});
 let attempts=0,counter=0;
 const pinned=await pinRevisionV1(options(cas,root,["a.txt"],{io:{afterRead:()=>{attempts++;counter++;writeFileSync(join(root,"a.txt"),`${counter}-${"x".repeat(counter)}\n`);}}}));
 expect(attempts).toBe(REVISION_POLICY_V1.maximumAttempts);
 expect([...pinned.torn]).toEqual(["a.txt"]);
 const world=await openPinnedWorldV1(cas,pinned.revision.id,{verify:true}),seen=bytesOf(await world.read("a.txt"));
 expect(seen).toBe(`${counter-1}-${"x".repeat(counter-1)}\n`);
 expect(pinned.manifest.entries[0]!.digest).toBe(revisionDigestV1(seen));
 expect(revisionTextV1("torn","en",{path:"a.txt"})).toContain("a.txt");
});

test("two workspaces with equal inputs share manifest digest, revision content address of the tree, action keys and one stored copy",async()=>{
 const fixture=fixtures.fixtures[0] as Fixture,cas=store("share-cas"),first=workspaceOf(fixture,"ws-one"),second=workspaceOf(fixture,"ws-two");
 const one=await pinRevisionV1(options(cas,first,fixture.paths,{workspace:"w1"})),afterOne=await casUsageV1(cas.root);
 const two=await pinRevisionV1(options(cas,second,fixture.paths,{workspace:"w2",holder:"agent-b"})),afterTwo=await casUsageV1(cas.root);
 expect(two.manifest.digest).toBe(one.manifest.digest);
 expect(two.revision.id).not.toBe(one.revision.id);
 expect(actionKeyV1({tool:"t",command:["c"],manifest:two.manifest.digest})).toBe(actionKeyV1({tool:"t",command:["c"],manifest:one.manifest.digest}));
 expect(afterTwo.bytes-afterOne.bytes).toBeLessThan(400);
 expect(afterTwo.objects-afterOne.objects).toBe(1);
 writeFileSync(join(second,"src/main.ts"),"different\n");
 const three=await pinRevisionV1(options(cas,second,fixture.paths,{workspace:"w2",holder:"agent-b"}));
 expect(three.manifest.digest).not.toBe(one.manifest.digest);
});

test("property: equal contents in any directory layout order give equal digests; any single change changes the digest",async()=>{
 let state=7;const random=()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};
 for(let round=0;round<25;round++){
  const files:Record<string,{text:string}>={};
  for(let index=0;index<1+Math.floor(random()*12);index++)files[`${["a","b","ü","𠀀","🧊️"][Math.floor(random()*5)]}/${["x","y","z"][Math.floor(random()*3)]}${index%3===0?"/deep":""}/f${index}.txt`]={text:`c${Math.floor(random()*5)}`};
  const paths=Object.keys(files),reversed=[...paths].reverse(),cas=store("prop-cas"),root=scratch("prop-ws");write(root,files);
  const a=await pinRevisionV1(options(cas,root,paths)),b=await pinRevisionV1(options(cas,root,reversed));
  expect(b.manifest.digest).toBe(a.manifest.digest);
  const tar=join(scratch("prop-tar"),"s.tar");ask({op:"snapshot",root,paths,tar});
  expect(ask({op:"digest",tar}).digest).toBe(a.manifest.digest);
  const victim=paths[Math.floor(random()*paths.length)]!;writeFileSync(join(root,...victim.split("/")),"changed-content");
  expect((await pinRevisionV1(options(cas,root,paths))).manifest.digest).not.toBe(a.manifest.digest);
 }
});

test("a parent revision with an exact dirty set stages only the dirty files and equals a full restage",async()=>{
 const fixture=fixtures.fixtures[0] as Fixture,root=workspaceOf(fixture,"inc"),cas=store("inc-cas");
 const parent=await pinRevisionV1(options(cas,root,fixture.paths));
 expect(parent.staged).toBe(fixture.paths.length-1);
 writeFileSync(join(root,"src/main.ts"),"export const a=2;\n");
 const reads:string[]=[],child=await pinRevisionV1(options(cas,root,fixture.paths,{parent,dirty:{paths:["src/main.ts"]},io:{beforeRead:(path:string)=>{reads.push(path);}}}));
 expect(reads).toEqual(["gone/missing.ts","src/main.ts"].sort(compareCodePointsV1).filter(path=>reads.includes(path)));
 expect(reads).toContain("src/main.ts");expect(reads).not.toContain("package.json");
 expect(child.reused).toBe(fixture.paths.length-1-1);
 const full=await pinRevisionV1(options(cas,root,fixture.paths));
 expect(child.manifest.digest).toBe(full.manifest.digest);
 const overflow:string[]=[];
 await pinRevisionV1(options(cas,root,fixture.paths,{parent,dirty:{paths:[],overflow:true},io:{beforeRead:(path:string)=>{overflow.push(path);}}}));
 expect(overflow.length).toBe(fixture.paths.length);
});

test("a reused parent entry whose object left the store is staged again",async()=>{
 const root=scratch("lost"),cas=store("lost-cas");write(root,{"a.txt":{text:"a\n"}});
 const parent=await pinRevisionV1(options(cas,root,["a.txt"]));
 await releasePinV1(cas,parent.pin);rmSync(cas.objectPath(parent.manifest.entries[0]!.digest));
 const child=await pinRevisionV1(options(cas,root,["a.txt"],{parent,dirty:{paths:[]}}));
 expect(child.reused).toBe(0);expect(child.staged).toBe(1);
});

test("pins are refcounted GC roots: every object is protected while any holder pins the revision and free after the last release",async()=>{
 const root=scratch("pins"),cas=store("pins-cas");write(root,{"a.txt":{text:"a\n"},"d/b.txt":{text:"b\n"}});
 const first=await pinRevisionV1(options(cas,root,["a.txt","d/b.txt"],{holder:"agent-a"})),second=await pinRevisionV1(options(cas,root,["a.txt","d/b.txt"],{holder:"agent-b"}));
 expect(first.pin.holder).not.toBe(second.pin.holder);
 const {digests}=await loadManifestV1(cas,first.revision.id);
 expect(digests.length).toBe(1+2+2);
 const roots=async()=>readCasPinsV1(cas.root,Date.now());
 expect([...(await roots()).keys()].sort()).toEqual([...digests].sort());
 await releasePinV1(cas,first.pin);
 expect((await roots()).size).toBe(digests.length);
 await releasePinV1(cas,second.pin);
 expect((await roots()).size).toBe(0);
});

test("a pin lapses at its lease end and renewal restores it only while the objects exist",async()=>{
 const root=scratch("lease"),cas=store("lease-cas");write(root,{"a.txt":{text:"a\n"}});
 const pinned=await pinRevisionV1(options(cas,root,["a.txt"],{ttlMs:1}));
 await Bun.sleep(10);
 expect((await readCasPinsV1(cas.root,Date.now())).size).toBe(0);
 const renewed=await renewPinV1(cas,pinned.pin,{ttlMs:60000});
 expect(renewed.expiresMs).toBeGreaterThan(pinned.pin.expiresMs);
 expect((await readCasPinsV1(cas.root,Date.now())).size).toBeGreaterThan(0);
 await releasePinV1(cas,renewed);
 rmSync(cas.objectPath(pinned.manifest.entries[0]!.digest));
 await expect(renewPinV1(cas,renewed)).rejects.toMatchObject({code:"unknown-revision"});
});

test("withPinnedRevision releases the pin after success, failure and cancellation",async()=>{
 const root=scratch("with"),cas=store("with-cas");write(root,{"a.txt":{text:"a\n"}});
 const live=async()=>(await readCasPinsV1(cas.root,Date.now())).size;
 expect(await withPinnedRevisionV1(options(cas,root,["a.txt"]),async pinned=>{expect(await live()).toBeGreaterThan(0);return pinned.manifest.entries.length;})).toBe(1);
 expect(await live()).toBe(0);
 await expect(withPinnedRevisionV1(options(cas,root,["a.txt"]),async()=>{throw Error("boom");})).rejects.toThrow("boom");
 expect(await live()).toBe(0);
 const controller=new AbortController();
 await expect(withPinnedRevisionV1(options(cas,root,["a.txt"],{signal:controller.signal}),async()=>{controller.abort();controller.signal.throwIfAborted();})).rejects.toThrow();
 expect(await live()).toBe(0);
});

test("staging reports progress, honours cancellation and bounds the path count",async()=>{
 const root=scratch("prog"),cas=store("prog-cas"),files:Record<string,{text:string}>={};
 for(let index=0;index<40;index++)files[`f/${index}.txt`]={text:`${index}`};write(root,files);
 const events:{done:number;total:number}[]=[];
 await pinRevisionV1(options(cas,root,Object.keys(files),{progress:(event:{done:number;total:number})=>events.push(event),policy:{...REVISION_POLICY_V1,progressMs:0}}));
 expect(events.at(-1)).toEqual({phase:"stage",done:40,total:40} as any);
 expect(events.length).toBeGreaterThan(2);
 const controller=new AbortController();controller.abort();
 await expect(pinRevisionV1(options(cas,root,Object.keys(files),{signal:controller.signal}))).rejects.toThrow();
 await expect(pinRevisionV1(options(cas,root,["a","b"],{policy:{...REVISION_POLICY_V1,maximumPaths:1}}))).rejects.toMatchObject({code:"too-many-paths"});
 await expect(pinRevisionV1(options(cas,root,["../escape"]))).rejects.toMatchObject({code:"unsafe-path"});
 expect(revisionErrorV1("unsafe-path",{path:"x"}).message).toContain("Pfad");
});

test("stage requests return store addresses and restage evicted content only when the workspace still holds it",async()=>{
 const root=scratch("stage"),cas=store("stage-cas");write(root,{"a.txt":{text:"a\n"},"b.txt":{text:"b\n"}});
 const pinned=await pinRevisionV1(options(cas,root,["a.txt","b.txt"])),request={revision:pinned.revision.id,paths:["b.txt"]};
 const reply=await stageRevisionV1(cas,request,{root});
 expect(validator("StageReply")(reply)).toBe(true);expect(validator("StageRequest")(request)).toBe(true);
 expect(reply.refs).toEqual([{hash:revisionDigestV1("b\n"),size:2}]);
 rmSync(cas.objectPath(revisionDigestV1("b\n")));
 expect((await stageRevisionV1(cas,request,{root})).refs[0]!.hash).toBe(revisionDigestV1("b\n"));
 rmSync(cas.objectPath(revisionDigestV1("b\n")));writeFileSync(join(root,"b.txt"),"changed\n");
 await expect(stageRevisionV1(cas,request,{root})).rejects.toMatchObject({code:"content-evicted"});
});

test("an unknown or tampered revision is refused",async()=>{
 const cas=store("bad-cas"),fake=await cas.put(new TextEncoder().encode("{\"x\":1}"));
 await expect(loadManifestV1(cas,"0".repeat(64))).rejects.toMatchObject({code:"unknown-revision"});
 await expect(loadManifestV1(cas,fake.hash)).rejects.toMatchObject({code:"invalid-revision"});
 const tree=buildTreeV1([{path:"a/b.txt",mode:"file",digest:"1".repeat(64),size:3}]);
 const flat=await flattenTreeV1(tree.root,async digest=>tree.objects.get(digest));
 expect(flat).toEqual([{path:"a/b.txt",mode:"file",digest:"1".repeat(64),size:3}] as ManifestEntryV1[]);
 expect(()=>buildTreeV1([{path:"a",mode:"file",digest:"1".repeat(64),size:1},{path:"a/b",mode:"file",digest:"1".repeat(64),size:1}])).toThrow();
});

test("utimes-only touches do not change the digest",async()=>{
 const root=scratch("touch"),cas=store("touch-cas");write(root,{"a.txt":{text:"a\n"}});
 const one=await pinRevisionV1(options(cas,root,["a.txt"]));utimesSync(join(root,"a.txt"),new Date(1e12),new Date(1e12));
 expect((await pinRevisionV1(options(cas,root,["a.txt"]))).manifest.digest).toBe(one.manifest.digest);
});

test("the synchronous view equals the asynchronous world and the live directory listing it was staged from",async()=>{
 const fixture=fixtures.fixtures[0] as Fixture,root=workspaceOf(fixture,"sync"),cas=store("sync-cas");
 const pinned=await pinRevisionV1(options(cas,root,fixture.paths)),asyncWorld=await openPinnedWorldV1(cas,pinned.revision.id,{verify:true}),syncWorld=openPinnedWorldSyncV1(cas,pinned.manifest,{verify:true});
 writeFileSync(join(root,"src/main.ts"),"changed\n");
 for(const path of fixture.paths){const left=syncWorld.readSync(path),right=await asyncWorld.read(path);expect(left===undefined).toBe(right===undefined);if(left&&right)expect(bytesOf(left)).toBe(bytesOf(right));}
 expect(syncWorld.readSync("absent.txt")).toBeUndefined();
 expect(syncWorld.filesSync("src")).toEqual(await asyncWorld.files("src"));
 expect(syncWorld.kindSync("src")).toBe("directory");expect(syncWorld.kindSync("src/main.ts")).toBe("file");expect(syncWorld.kindSync("nope")).toBeUndefined();
 const names=(directory:string)=>syncWorld.listSync(directory)!.map(row=>row.name).sort(compareCodePointsV1);
 const live=readdirSync(join(root,"src")).filter(name=>fixture.paths.includes("src/"+name)||fixture.paths.some(path=>path.startsWith("src/"+name+"/"))).sort(compareCodePointsV1);
 expect(names("src")).toEqual(live);
 expect(syncWorld.listSync("src")!.find(row=>row.name==="🧊️")!.directory).toBe(true);
 expect(syncWorld.listSync("missing")).toBeUndefined();
});

test("the pinned fs port answers stat, lstat, readdir, readFile, exists and realpath like node:fs does on the live tree it was staged from",async()=>{
 const root=scratch("fsport");
 write(root,{"a/b/c.txt":{text:"c\n"},"a/run.sh":{text:"#!/bin/sh\n",executable:true},"a/𠀀.ts":{text:"x\n"},"a/ｚ.ts":{text:"z\n"},"top.txt":{text:"t\n"}});
 symlinkSync("b/c.txt",join(root,"a/link-file"));symlinkSync("b",join(root,"a/link-dir"));
 const paths=["a/b/c.txt","a/run.sh","a/𠀀.ts","a/ｚ.ts","top.txt","a/link-file","a/link-dir"],cas=store("fsport-cas"),pinned=await pinRevisionV1(options(cas,root,paths));
 const port=createPinnedFsV1({world:openPinnedWorldSyncV1(cas,pinned.manifest,{verify:true}),workspaceRoot:root,fallback:nodeFs});
 const live=nodeFs;
 const kind=(s:nodeFs.Stats)=>s.isDirectory()?"d":s.isSymbolicLink()?"l":(s.mode&0o111)!==0?"x":"f";
 const probe=(fs:typeof nodeFs,path:string)=>{
  const out:Record<string,unknown>={};
  for(const [name,call] of Object.entries({
   stat:()=>{const s=fs.statSync(path);return [kind(s),s.isFile()?s.size:0];},
   lstat:()=>{const s=fs.lstatSync(path);return [kind(s),s.isSymbolicLink()||s.isFile()?s.size:0];},
   exists:()=>fs.existsSync(path),
   read:()=>fs.readFileSync(path,"utf8"),
   readdir:()=>fs.readdirSync(path).sort(compareCodePointsV1),
   dirents:()=>fs.readdirSync(path,{withFileTypes:true}).map(row=>[row.name,row.isDirectory(),row.isFile(),row.isSymbolicLink()] as const).sort((left,right)=>compareCodePointsV1(left[0],right[0])),
   real:()=>fs.realpathSync(path),
  })){try{out[name]=call();}catch(error){out[name]="!"+(error as {code?:string}).code;}}
  return out;
 };
 const names=[".","a","a/b","a/b/c.txt","a/run.sh","a/𠀀.ts","a/ｚ.ts","top.txt","a/link-file","a/link-dir","a/link-dir/c.txt","a/missing","missing/deeper"];
 for(const name of names){
  const full=join(root,name),expected=probe(live as typeof nodeFs,full),actual=probe(port as typeof nodeFs,full);
  expect(actual,name).toEqual(expected);
 }
 expect(probe(port as typeof nodeFs,join(scratch("outside"),"nope")).stat).toBe("!ENOENT");
 writeFileSync(join(root,"top.txt"),"edited\n");
 expect(port.readFileSync(join(root,"top.txt"),"utf8")).toBe("t\n");
});

test("a captured plan sees one snapshot while files change, its pin equals what it read and the independent tarball oracle agrees",async()=>{
 const root=scratch("capture"),cas=store("capture-cas"),files={"src/a.ts":{text:"a1\n"},"src/b.ts":{text:"b1\n"},"bin/run.sh":{text:"#!/bin/sh\n",executable:true},"unread.txt":{text:"never\n"},"node_modules/dep/index.js":{text:"dep1\n"}};
 write(root,files);
 const full=(path:string)=>join(root,path);
 const outcome=await withCapturedPlanV1({cas,root,workspace:"w1",holder:"plan-a",journalCursor:"c1"},async()=>{
  const first=bytesOf(observedFs.readFileSync(full("src/a.ts"))),list=observedFs.readdirSync(full("src")).sort(),dep=bytesOf(observedFs.readFileSync(full("node_modules/dep/index.js")));
  const hasMissing=observedFs.existsSync(full("src/new.ts")),run=bytesOf(observedFs.readFileSync(full("bin/run.sh")));
  writeFileSync(full("src/a.ts"),"a2\n");writeFileSync(full("src/new.ts"),"n\n");writeFileSync(full("src/b.ts"),"b2\n");writeFileSync(full("node_modules/dep/index.js"),"dep2\n");
  const again=bytesOf(observedFs.readFileSync(full("src/a.ts"))),later=bytesOf(observedFs.readFileSync(full("src/b.ts")));
  return{first,again,list,hasMissing,missingAfter:observedFs.existsSync(full("src/new.ts")),dep,depAfter:bytesOf(observedFs.readFileSync(full("node_modules/dep/index.js"))),later,run,listAfter:observedFs.readdirSync(full("src")).sort()};
 });
 const seen=outcome.result;
 expect(seen.first).toBe("a1\n");expect(seen.again,"an edit mid-plan does not change the plan").toBe("a1\n");
 expect(seen.later,"a file first read after the edit is the edited file").toBe("b2\n");
 expect(seen.list).toEqual(["a.ts","b.ts"]);expect(seen.listAfter,"a listing keeps its first answer").toEqual(["a.ts","b.ts"]);
 expect(seen.hasMissing).toBe(false);expect(seen.missingAfter,"a missing path stays missing").toBe(false);
 expect(seen.dep).toBe("dep1\n");expect(seen.depAfter,"dependency trees are read live").toBe("dep2\n");
 const paths=outcome.pinned.manifest.entries.map(entry=>entry.path);
 expect(paths).toEqual(["bin/run.sh","src/a.ts","src/b.ts"]);
 expect(outcome.pinned.manifest.entries.find(entry=>entry.path==="bin/run.sh")!.mode).toBe(process.platform==="win32"?"file":"executable");
 const frozen=openPinnedWorldSyncV1(cas,outcome.pinned.manifest,{verify:true});
 expect(bytesOf(frozen.readSync("src/a.ts"))).toBe("a1\n");expect(frozen.readSync("unread.txt")).toBeUndefined();
 const reference=scratch("capture-ref");write(reference,{"src/a.ts":{text:"a1\n"},"src/b.ts":{text:"b2\n"},"bin/run.sh":{text:"#!/bin/sh\n",executable:true}});
 const tar=join(scratch("capture-tar"),"r.tar");ask({op:"snapshot",root:reference,paths,tar});
 expect(outcome.pinned.manifest.digest,"the tarball oracle agrees").toBe(ask({op:"digest",tar}).digest);
 expect(readCasPinsV1(cas.root)).toBeDefined();
});

test("a plan that reads nothing pins the empty revision and a failing plan seals nothing",async()=>{
 const root=scratch("capture-empty"),cas=store("capture-empty-cas");write(root,{"a.txt":{text:"x\n"}});
 const empty=await withCapturedPlanV1({cas,root,workspace:"w",holder:"p",journalCursor:"c"},async()=>1);
 expect(empty.pinned.manifest.entries).toEqual([]);expect(empty.result).toBe(1);
 await expect(withCapturedPlanV1({cas,root,workspace:"w",holder:"p",journalCursor:"c"},async()=>{throw Error("plan failed");})).rejects.toThrow("plan failed");
});

test("providers load through the plan's capture: an edit of the provider or its relative import mid-plan does not change what loads, the pin lists what the loader read, bare imports resolve live",async()=>{
 const root=scratch("loader"),cas=store("loader-cas"),full=(path:string)=>join(root,path);
 write(root,{"plugin/provider.ts":{text:'import {dep} from "./dep.ts";\nimport {basename} from "node:path";\nimport {bare} from "bare-pkg";\nexport const value:string=dep+":"+basename("/x/y")+":"+bare;\nexport const here:string=import.meta.dir;\n'},"plugin/dep.ts":{text:'export const dep:string="dep1";\n'},"plugin/other.ts":{text:'import {dep} from "./dep";\nexport const other:string="other:"+dep;\n'},"node_modules/bare-pkg/package.json":{text:'{"name":"bare-pkg","main":"index.js"}\n'},"node_modules/bare-pkg/index.js":{text:'exports.bare="bare1";\n'}});
 const first=await withCapturedPlanV1({cas,root,workspace:"w",holder:"p1",journalCursor:"c1"},async()=>{
  const provider=await importObservedModuleV1(full("plugin/provider.ts")) as {value:string;here:string};
  writeFileSync(full("plugin/dep.ts"),'export const dep:string="dep2";\n');writeFileSync(full("plugin/provider.ts"),'export const value:string="rewritten";\n');
  const other=await importObservedModuleV1(full("plugin/other.ts")) as {other:string};
  const again=await importObservedModuleV1(full("plugin/provider.ts")) as {value:string};
  return{value:provider.value,here:provider.here,other:other.other,again:again.value};
 });
 expect(first.result.value).toBe("dep1:y:bare1");expect(first.result.here,"import.meta.dir names the real directory").toBe(full("plugin"));
 expect(first.result.other,"an import of the edited file still sees the captured bytes").toBe("other:dep1");expect(first.result.again).toBe("dep1:y:bare1");
 expect(first.pinned.manifest.entries.map(entry=>entry.path)).toEqual(["plugin/dep.ts","plugin/other.ts","plugin/provider.ts"]);
 const frozen=openPinnedWorldSyncV1(cas,first.pinned.manifest,{verify:true});
 expect(bytesOf(frozen.readSync("plugin/dep.ts"))).toContain("dep1");expect(bytesOf(frozen.readSync("plugin/provider.ts"))).toContain("import {dep}");
 const second=await withCapturedPlanV1({cas,root,workspace:"w",holder:"p2",journalCursor:"c2"},async()=>(await importObservedModuleV1(full("plugin/provider.ts")) as {value:string}).value);
 expect(second.result,"the next plan loads the edited provider").toBe("rewritten");
 const third=await withCapturedPlanV1({cas,root,workspace:"w",holder:"p3",journalCursor:"c3"},async()=>(await importObservedModuleV1(full("plugin/provider.ts")) as {value:string}).value);
 expect(third.result).toBe("rewritten");expect(third.pinned.manifest.entries.map(entry=>entry.path),"a reused module still lists its files in the pin").toEqual(["plugin/provider.ts"]);
 expect(importObservedModuleV1(full("plugin/provider.ts")),"outside a plan nothing is intercepted").toBeUndefined();
});

test("the module rewrite is syntax aware: specifiers and import.meta are rewritten, strings, templates and comments never",async()=>{
 const source=[
  '// import.meta.url and "./dep" in a comment',
  '/* import("./dep") */',
  'import {dep} from "./dep";',
  'export {dep as again} from \'./dep\';',
  'export const text:string="import.meta.url";',
  'export const quoted:string=\'from "./dep"\';',
  'export const template:string=`${import.meta.dir} import.meta.url "./dep"`;',
  'export const here:string=import.meta.url;',
  'export const later=async()=>(await import("./dep")).dep+require("./dep").dep;',
  'export const computed:string="require(\\"./dep\\")"+String(import.meta.main);',
 ].join("\n");
 const output=pinModuleV1(source,"/w/plugin/p.ts","ts",specifier=>specifier==="./dep"?"semio-capture/id|/w/plugin/dep.ts":undefined);
 const lines=output.split("\n");
 expect(lines[0]).toBe('// import.meta.url and "./dep" in a comment');expect(lines[1]).toBe('/* import("./dep") */');
 expect(lines[2]).toBe('import {dep} from "semio-capture/id|/w/plugin/dep.ts";');expect(lines[3]).toBe('export {dep as again} from "semio-capture/id|/w/plugin/dep.ts";');
 expect(lines[4]).toBe('export const text:string="import.meta.url";');expect(lines[5]).toBe('export const quoted:string=\'from "./dep"\';');
 expect(lines[6]).toBe('export const template:string=`${"/w/plugin"} import.meta.url "./dep"`;');
 expect(lines[7]).toBe('export const here:string="file:///w/plugin/p.ts";');
 expect(lines[8]).toBe('export const later=async()=>(await import("semio-capture/id|/w/plugin/dep.ts")).dep+require("semio-capture/id|/w/plugin/dep.ts").dep;');
 expect(lines[9]).toBe(source.split("\n")[9]!.replace("import.meta.main","import.meta.main"));
 expect(pinModuleV1('{"a":"import.meta.url"}',"/w/x.json","json",()=>"x")).toBe('{"a":"import.meta.url"}');
});
