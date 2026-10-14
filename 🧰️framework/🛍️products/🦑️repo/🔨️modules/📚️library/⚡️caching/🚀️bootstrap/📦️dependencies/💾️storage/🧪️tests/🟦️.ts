import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {resolve,sep,join} from "node:path";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync} from "node:fs";
import glob from "fast-glob";
import {discoverCurrentBunWorkspaceScopesV1,withPublishedBunWorkspaceOwnershipV1,type BunWorkspaceInstallationCustodyV1} from "../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import bindingsSchema from "../🧬️schema/📥️bindings.json";
import corpus from "./🔣️.json";
import {javascriptDependencyBindingsV1,javascriptDependencyInstallManifestV1} from "../🟦️.ts";
import installSchema from "../🧬️schema/📥️install.json";
import installCorpus from "./📥️install.json";

test("closed dependency scopes agree with independent SQLite identities and schema",()=>{const validate=new Ajv({strict:true}).compile(schema),oracle=new Database(":memory:");try{for(const row of corpus.cases){if(row.accepted){expect(validate({version:1,scope:row.scope,members:row.members,generation:"a".repeat(64)})).toBe(true);const actual=javascriptDependencyBindingsV1("/r","/d",row.scope,row.members,"a".repeat(64));expect(actual.length).toBe(row.count!);for(const [index,binding]of actual.entries()){const identity="a".repeat(20),expected=oracle.query("SELECT ?1 || ?3 || 'j' || ?3 || ?2 || ?3 || ?4 AS path").get(resolve("/d"),identity,sep,index===0?"node_modules":"w"+sep+(index-1)+sep+"node_modules") as {path:string};expect(binding.targetDirectory).toBe(expected.path);}}else expect(()=>javascriptDependencyBindingsV1("/r","/d",row.scope,row.members,"a".repeat(64))).toThrow();}}finally{oracle.close();}});

test("closed binding receipts agree with Ajv and independent SQLite projection",()=>{const validate=new Ajv({strict:true}).compile(bindingsSchema),receipt={version:1,generation:"a".repeat(64),bindings:javascriptDependencyBindingsV1("/r","/d",".",corpus.bindingPublication.members,"a".repeat(64))},database=new Database(":memory:");try{expect(validate(receipt)).toBe(true);expect((database.query("SELECT json_array_length(?, '$.bindings') AS count").get(JSON.stringify(receipt)) as {count:number}).count).toBe(corpus.bindingPublication.resumedBindings);for(const value of [{...receipt,extra:true},{...receipt,generation:"wrong"},{...receipt,bindings:[]},{...receipt,bindings:[{sourceDirectory:"/r"}]}])expect(validate(value)).toBe(false);}finally{database.close();}});

test("portable install material excludes only top-level task metadata against independent SQLite and Ajv",()=>{
 const ajv=new Ajv({strict:true}),manifest=ajv.compile({$schema:installSchema.$schema,$defs:installSchema.$defs,$ref:"#/$defs/InstallManifest"}),mutation=ajv.compile({$schema:installSchema.$schema,$defs:installSchema.$defs,$ref:"#/$defs/Mutation"}),database=new Database(":memory:");
 try{expect(ajv.validate(installSchema,installCorpus.protocol)).toBe(true);for(const row of installCorpus.mutations){expect(mutation(row)).toBe(true);const actual=[row.before,row.after].map(value=>javascriptDependencyInstallManifestV1(value));for(const[index,value]of[row.before,row.after].entries()){const oracle=database.query("SELECT json_remove(?, '$.semio') AS document").get(JSON.stringify(value)) as {document:string};expect(JSON.parse(actual[index]!)).toEqual(JSON.parse(oracle.document));expect(manifest(JSON.parse(actual[index]!))).toBe(true);}expect(actual[0]===actual[1]).toBe(row.sameInstallMaterial);expect(createHash("sha256").update(actual[0]!).digest("hex")===createHash("sha256").update(actual[1]!).digest("hex")).toBe(row.sameInstallMaterial);}for(const value of[null,[],true,"manifest"])expect(()=>javascriptDependencyInstallManifestV1(value)).toThrow("Original Bun manifest object required");expect(javascriptDependencyInstallManifestV1({z:{z:1,a:2},a:[2,1],semio:{ignored:true}})).toBe('{"a":[2,1],"z":{"a":2,"z":1}}');}finally{database.close();}
});

test("an actual recheck delayed in original control refuses settlement after callback release against SQLite and Ajv",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original delayed custody artifact root required");mkdirSync(artifacts,{recursive:true});const root=mkdtempSync(join(artifacts,"jl-")),input=installCorpus.delayedCustody,events:string[]=[],database=new Database(":memory:");let inside=false,paused=false,release!:()=>void,entered!:()=>void,pending:Promise<{refusal:string}|{ok:true}>|undefined;
 const gate=new Promise<void>(accept=>{release=accept;}),held=new Promise<void>(accept=>{entered=accept;}),control={signal:new AbortController().signal,advance:async()=>{if(inside&&!paused){paused=true;events.push("recheck-paused");entered();await gate;}}};
 writeFileSync(join(root,"package.json"),JSON.stringify({name:"delayed-custody",private:true,semio:{workspace:{schemaVersion:1,members:["packages/**"],owners:[]}},workspaces:[]}));
 try{
  expect(new Ajv({strict:true}).validate({$schema:installSchema.$schema,$defs:installSchema.$defs,$ref:"#/$defs/DelayedCustody"},input)).toBe(true);
  await withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{inside=true;events.push("body-entered");pending=custody.recheck().then(()=>({ok:true as const}),error=>({refusal:(error as Error).message}));await held;events.push("body-returned");});
  events.push("custody-released");events.push("recheck-resumed");release();if(!pending)throw Error("Original delayed recheck missing");const result=await pending;expect(events).toEqual(input.sequence);
  const oracle=database.query("SELECT CASE WHEN (SELECT key FROM json_each(?1) WHERE value='custody-released') < (SELECT key FROM json_each(?1) WHERE value='recheck-resumed') THEN ?2 ELSE 'accepted' END AS refusal").get(JSON.stringify(events),input.result.refusal) as {refusal:string};expect(result).toEqual({refusal:oracle.refusal});expect(result).toEqual(input.result);
 }finally{release();database.close();rmSync(root,{recursive:true,force:true});}
});

test("live canonical membership custody refreshes deletions and refuses metadata or topology drift without a second package scan",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original membership artifact root required");mkdirSync(artifacts,{recursive:true});const root=mkdtempSync(join(artifacts,"ji-")),input=installCorpus.membership,visits:string[]=[],selections:string[]=[],control={signal:new AbortController().signal,advance:async(progress:{path:string;operation:string})=>{visits.push(progress.path);if(progress.operation==="selection")selections.push(progress.path);}},put=(path:string,value:unknown)=>{mkdirSync(resolve(root,path,".."),{recursive:true});writeFileSync(join(root,path),JSON.stringify(value));};
 put("package.json",{name:"install-custody",private:true,semio:{workspace:{schemaVersion:1,members:input.members,owners:input.owners}},workspaces:[]});put(input.deleted,{name:"member-a",version:"1.0.0",...input.metadataBefore});
 try{
  const scopes=await discoverCurrentBunWorkspaceScopesV1(root,{kind:"all"},control);expect(scopes.map(scope=>scope.manifest)).toEqual(["package.json"]);expect(visits.some(path=>path.startsWith(join(root,"packages")))).toBe(false);let retained:BunWorkspaceInstallationCustodyV1|undefined;
  await withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{retained=custody;expect(custody.scopes[0]!.packages).toEqual(input.initial);const scans=selections.length;await custody.recheck();expect(selections.length).toBe(scans);expect(glob.sync(input.members.map(member=>member+"/package.json"),{cwd:root,onlyFiles:true,followSymbolicLinks:false}).map(path=>path.replace(/\/package.json$/u,""))).toEqual([...custody.scopes[0]!.packages]);});
  if(!retained)throw Error("Original custody callback missing");await expect(retained.recheck()).rejects.toThrow("callback released");
  await expect(withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{put(input.deleted,{name:"member-a",version:"1.0.0",...input.metadataAfter});await custody.recheck();})).rejects.toThrow("Bun current manifest changed");
  await expect(withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{rmSync(join(root,input.deleted));await custody.recheck();})).rejects.toThrow("Bun current ownership changed");
  await withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{expect(custody.scopes[0]!.packages).toEqual(input.expectedAfterDeletion);expect(JSON.parse(readFileSync(join(root,"package.json"),"utf8")).workspaces).toEqual(input.expectedAfterDeletion);});
  await expect(withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{put(input.addition,{name:"member-b",version:"1.0.0"});await custody.recheck();})).rejects.toThrow("Bun current ownership changed");
  await withPublishedBunWorkspaceOwnershipV1(root,{kind:"scope",owner:"."},control,async custody=>{expect(custody.scopes[0]!.packages).toEqual(input.expectedAfterAddition);});
 }finally{rmSync(root,{recursive:true,force:true});}
});


test("canonical sibling dependency members stay in original repository Source and flat slots",async()=>{
 const api=await import("../🟦️.ts"),paths=(api as any).javascriptDependencyPackagePathsV1;expect(typeof paths).toBe("function");const validate=new Ajv({strict:true}).compile(schema),platforms=await import("node:path"),database=new Database(":memory:");
 try{for(const row of corpus.siblingScope.cases){expect(validate({version:1,scope:row.scope,members:row.members,generation:"a".repeat(64)})).toBe(true);const actual=paths(row.scope,row.members);expect(actual).toEqual(row.paths);for(const platform of corpus.siblingScope.platforms){const path=platform.kind==="win32"?platforms.win32:platforms.posix;for(const[index,value]of actual.entries()){const requested=index===0?path.resolve(platform.root,row.scope):path.resolve(platform.root,row.scope,row.members[index-1]!);expect(path.resolve(platform.root,value)).toBe(requested);expect(path.relative(platform.root,requested).startsWith("..")).toBe(false);}}const bindings=javascriptDependencyBindingsV1("/r","/d",row.scope,row.members,"a".repeat(64));for(const[index,binding]of bindings.entries()){const oracle=database.query("SELECT ?1 || ?2 || ?3 AS path").get(resolve("/r"),sep,actual[index]!.split("/").join(sep)+sep+"node_modules") as {path:string};expect(binding.sourceDirectory).toBe(oracle.path);expect(binding.targetDirectory).toBe(join(resolve("/d"),"j","a".repeat(20),...(index===0?[]:["w",String(index-1)]),"node_modules"));}}
 for(const row of corpus.siblingScope.refused)expect(()=>paths(row.scope,row.members)).toThrow();
 }finally{database.close();}
});
