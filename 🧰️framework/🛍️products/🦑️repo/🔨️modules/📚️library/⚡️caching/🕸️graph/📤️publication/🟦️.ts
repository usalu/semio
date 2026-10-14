import {randomBytes,createHash} from "node:crypto";
import {constants,copyFileSync,existsSync,lstatSync,mkdirSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,withResourceLeases,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {graphResourceLeaseDirectoryV1} from "../🏁️startup/🟦️.ts";

export interface GraphPublicationFileV1 {readonly name:"project-graph.json"|"source-maps.json"|"file-map.json";readonly value:unknown}
export interface GraphPublicationLeaseV1 {readonly directory:string;readonly toolingIdentity:string;readonly control:ResourceLeaseControl}
export interface GraphPublicationV1 {readonly directory:string;readonly toolingIdentity:string;readonly files:readonly GraphPublicationFileV1[];readonly environment:Readonly<Record<string,string|undefined>>;readonly generationHolds:()=>boolean;readonly published?:(lease:GraphPublicationLeaseV1)=>Promise<void>}
const publications=new WeakSet<GraphPublicationLeaseV1>();
/** 🔒️ A nested receipt may retain only the exact live writer's namespace, toolkit and original control. */
export async function withGraphPublicationLeaseV1<T>(directory:string,toolingIdentity:string,control:ResourceLeaseControl,body:(lease:GraphPublicationLeaseV1)=>Promise<T>,publication?:GraphPublicationLeaseV1):Promise<T>{
 if(publication!==undefined){if(!publications.has(publication)||publication.directory!==directory||publication.toolingIdentity!==toolingIdentity||publication.control!==control)throw Error("Original graph publication lease refused");checkResourceLeaseControl(control);return body(publication);}
 const lease=Object.freeze({directory,toolingIdentity,control}),resource="nx:graph-publication:"+createHash("sha256").update(toolingIdentity+"\0"+directory).digest("hex");
 return withResourceLeases({directory:graphResourceLeaseDirectoryV1(),resources:[{resource,mode:"exclusive"}],signal:control.signal,control},async()=>{publications.add(lease);try{return await body(lease);}finally{publications.delete(lease);}});
}
function generationImage(file:string,image:string):string{
 const value=JSON.parse(image);if(file.endsWith("project-graph.json")&&value&&typeof value==="object"&&!Array.isArray(value)&&Object.hasOwn(value,"computedAt")){if(!Number.isSafeInteger(value.computedAt)||value.computedAt<1)throw Error("Original graph publication time required");const {computedAt,...graph}=value;return JSON.stringify(graph);}return JSON.stringify(value);
}
function generationUnchanged(rows:readonly {file:string;image:string}[],check:()=>void):boolean{
 const facts:{file:string;status:string}[]=[];for(const row of rows){check();let status;try{status=lstatSync(row.file,{bigint:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1n)throw Error("Graph publication regular single-owner file required");const stamp=(value:typeof status)=>[value.dev,value.ino,value.nlink,value.size,value.mtimeNs,value.ctimeNs].map(String).join(":"),before=stamp(status),bytes=readFileSync(row.file,"utf8");if(stamp(lstatSync(row.file,{bigint:true}))!==before)throw Error("Original graph publication changed during comparison");try{if(generationImage(row.file,bytes)!==generationImage(row.file,row.image))return false;}catch(error){if(error instanceof SyntaxError)return false;throw error;}facts.push({file:row.file,status:before});}
 for(const fact of facts){check();const status=lstatSync(fact.file,{bigint:true});if([status.dev,status.ino,status.nlink,status.size,status.mtimeNs,status.ctimeNs].map(String).join(":")!==fact.status)throw Error("Original graph publication changed during comparison");}check();return true;
}
/** 🧯️ Publishes one original graph generation under SQL writer ownership; faults restore prior copied images and retain their cause. */
export async function publishGraphFilesV1(value:GraphPublicationV1):Promise<void>{
 if(!/^[a-f0-9]{64}$/u.test(value.toolingIdentity)||!value.files.length||new Set(value.files.map(file=>file.name)).size!==value.files.length||value.files.some(file=>!["project-graph.json","source-maps.json","file-map.json"].includes(file.name))||!value.files.some(file=>file.name==="project-graph.json")||!value.files.some(file=>file.name==="source-maps.json"))throw Error("Closed original graph generation required");
 const directory=resolve(value.directory),generation=randomBytes(8).toString("hex"),rows=value.files.map(file=>({file:requireMaterializedPathV1(join(directory,file.name)),next:requireMaterializedPathV1(join(directory,file.name+"~"+generation+".next")),prior:requireMaterializedPathV1(join(directory,file.name+"~"+generation+".prior")),image:JSON.stringify(file.value),existed:false,published:false,restored:false}));
 for(let path=directory;;path=dirname(path)){try{const stat=lstatSync(path);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Graph publication no-follow directory required");}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}if(dirname(path)===path)break;}
 mkdirSync(directory,{recursive:true});if(realpathSync(directory)!==directory)throw Error("Original graph directory changed");
 await receiveScriptProcessInvocation(value.environment,async original=>{
  const control=scriptCompilerAdmissionControlV1(original,"nx:graph-publication",0),check=()=>{checkResourceLeaseControl(control);if(!value.generationHolds())throw Error("Original graph publication generation changed");};
  await withGraphPublicationLeaseV1(directory,value.toolingIdentity,control,async lease=>{
   if(!value.published&&generationUnchanged(rows,check))return;
   let committed=false;
   try{
    for(const row of rows){check();row.existed=existsSync(row.file);if(row.existed){const stat=lstatSync(row.file);if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1)throw Error("Graph publication regular single-owner file required");copyFileSync(row.file,row.prior,constants.COPYFILE_FICLONE|constants.COPYFILE_EXCL);}writeFileSync(row.next,row.image,{flag:"wx"});await control.yieldContinuation();}
    check();for(const row of [...rows].sort((a,b)=>Number(a.file.endsWith("project-graph.json"))-Number(b.file.endsWith("project-graph.json")))){check();renameSync(row.next,row.file);row.published=true;}
    check();committed=true;await value.published?.(lease);
   }catch(error){
    const failures:unknown[]=[];for(const row of [...rows].reverse())if(!committed&&row.published)try{if(row.existed)renameSync(row.prior,row.file);else rmSync(row.file,{force:true});row.restored=true;}catch(rollback){failures.push(rollback);}
    if(failures.length&&error instanceof Error)Object.defineProperty(error,"graphPublicationRollbackErrors",{value:failures});throw error;
   }finally{for(const row of rows){try{rmSync(row.next,{force:true});}catch{}if(committed||!row.published||row.restored)try{rmSync(row.prior,{force:true});}catch{}}}
  });
 });
}

/** ⏳️ Forwards one authenticated publication caller until its owned journal capture settles. */
export async function withGraphPublisherControlV1<T>(environment:Readonly<Record<string,string|undefined>>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 return receiveScriptProcessInvocation(environment,original=>body(scriptCompilerAdmissionControlV1(original,"nx:graph-publisher",0)));
}
