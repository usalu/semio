import {expect,test} from "bun:test";
import {mkdir,mkdtemp,realpath,rename,rm,writeFile,symlink} from "node:fs/promises";
import {join,relative,resolve,isAbsolute,sep} from "node:path";
import {createRequire} from "node:module";
import {createRepositoryOutputObservationV1,type OutputObservationPortsV1,type OutputObservationEventV1} from "../../🟦️.ts";
import {createSystemOutputObservationPortsV1} from "../../🔌️system/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const fixture=async()=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Caller-owned output observation artifacts required");await mkdir(artifacts,{recursive:true});return(await realpath(await mkdtemp(join(artifacts,"o")))).replaceAll("\\","/");};
const issued=<T,>(body:(control:ReturnType<typeof createIssuedResourceControlV1>)=>Promise<T>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:output-observation-kernel",maximumElapsedMilliseconds:60000},{},Date.now()),original=>body(createIssuedResourceControlV1(original,"output-observation:kernel",50000)));
const recursive=(root:string,started:string[],stopped:string[]):OutputObservationPortsV1["watch"]=>async(scope,receive)=>{
 const require=createRequire(join(process.cwd(),".nx/installation/package.json")),{Watcher}=require("nx/src/native"),watcher=new Watcher(scope.path,undefined,false);
 const project=(events:readonly {path:string;type:"create"|"update"|"delete"}[])=>events.map(event=>{const physical=resolve(scope.path,event.path),path=relative(root,physical);if(path===".."||path.startsWith(".."+sep)||isAbsolute(path))throw Error("Native output event leaves its workspace");return{path:path.replaceAll("\\","/")||".",type:event.type};});
 try{watcher.watch((error:unknown,events:readonly any[])=>{try{receive(error,error?[]:project(events));}catch(error){receive(error,[]);}});}catch(error){await watcher.stop();throw error;}
 started.push(scope.path);return{flush:()=>project(watcher.forceFlushPending()),stop:async()=>{await watcher.stop();stopped.push(scope.path);}};
};

test("system membership catches immediate exact revisions, missing paths and replacement without callback sleeps",()=>issued(async control=>{
 const root=await fixture(),events:OutputObservationEventV1[]=[],ports=createSystemOutputObservationPortsV1(root,control,async()=>{throw Error("A file endpoint needs no recursive watcher");}),observer=createRepositoryOutputObservationV1({root,control,ports,receive:(_error,rows)=>{events.push(...rows);}});
 try{await mkdir(join(root,"app"));await writeFile(join(root,"app/receipt.json"),"before");await observer.register(["app/receipt.json","app/new/deep/result.json"]);await writeFile(join(root,"app/receipt.json"),"after");await observer.flush();expect(events.some(event=>event.path==="app/receipt.json")).toBe(true);events.length=0;
  await rename(join(root,"app/receipt.json"),join(root,"app/older.json"));await writeFile(join(root,"app/receipt.json"),"replacement");await mkdir(join(root,"app/new/deep"),{recursive:true});await writeFile(join(root,"app/new/deep/result.json"),"created");await observer.flush();expect(events.some(event=>event.path==="app/receipt.json")).toBe(true);expect(events.some(event=>event.path==="app/new"||event.path==="app/new/deep/result.json")).toBe(true);
 }finally{await observer.stop();await rm(root,{recursive:true,force:true});}
}),60000);

test("actual native content watches stay scoped and logical links invalidate their recorded endpoints",()=>issued(async control=>{
 const root=await fixture(),events:OutputObservationEventV1[]=[],started:string[]=[],stopped:string[]=[],ports=createSystemOutputObservationPortsV1(root,control,recursive(root,started,stopped)),observer=createRepositoryOutputObservationV1({root,control,ports,receive:(_error,rows)=>{events.push(...rows);}});
 try{await mkdir(join(root,"app"));await mkdir(join(root,".cache/unit/deep"),{recursive:true});await mkdir(join(root,".cache/unrelated/many"),{recursive:true});await writeFile(join(root,".cache/unit/deep/result.bin"),"before");await symlink(join(root,".cache/unit"),join(root,"app/output"),process.platform==="win32"?"junction":"dir");await observer.register(["app/output"]);expect(started).toEqual([join(root,".cache/unit").replaceAll("\\","/")]);
  await writeFile(join(root,".cache/unit/deep/result.bin"),"after");await observer.flush();expect(events.some(event=>event.path==="app/output/deep/result.bin")).toBe(true);events.length=0;await mkdir(join(root,".cache/unrelated/many/more"),{recursive:true});await writeFile(join(root,".cache/unrelated/many/more/file"),"unrelated");await observer.flush();expect(started.length).toBe(1);expect(events.some(event=>event.path.startsWith("app/output/"))).toBe(false);
 }finally{await observer.stop();expect(stopped).toEqual(started);await rm(root,{recursive:true,force:true});}
}),60000);

test("actual alias retarget and directory replacement rebind before immediate content changes",()=>issued(async control=>{
 const root=await fixture(),events:OutputObservationEventV1[]=[],started:string[]=[],stopped:string[]=[],ports=createSystemOutputObservationPortsV1(root,control,recursive(root,started,stopped)),observer=createRepositoryOutputObservationV1({root,control,ports,receive:(_error,rows)=>{events.push(...rows);}});
 try{await mkdir(join(root,"app"));for(const name of ["first","next"])await mkdir(join(root,".cache",name,"deep"),{recursive:true});await symlink(join(root,".cache/first"),join(root,"app/output"),process.platform==="win32"?"junction":"dir");await observer.register(["app/output"]);
  await rm(join(root,"app/output"));await symlink(join(root,".cache/next"),join(root,"app/output"),process.platform==="win32"?"junction":"dir");await writeFile(join(root,".cache/next/deep/created.bin"),"before attachment");await observer.flush();expect(events.some(event=>event.path==="app/output")).toBe(true);expect(started.some(path=>path.endsWith("/.cache/next"))).toBe(true);events.length=0;
  await rename(join(root,".cache/next"),join(root,".cache/retired"));await mkdir(join(root,".cache/next/deep"),{recursive:true});await writeFile(join(root,".cache/next/deep/replacement.bin"),"replacement before attachment");await observer.flush();expect(events.some(event=>event.path==="app/output")).toBe(true);events.length=0;await writeFile(join(root,".cache/next/deep/replacement.bin"),"after attachment");await observer.flush();expect(events.some(event=>event.path==="app/output/deep/replacement.bin")).toBe(true);
 }finally{await observer.stop();expect(stopped.slice().sort()).toEqual(started.slice().sort());await rm(root,{recursive:true,force:true});}
}),60000);
