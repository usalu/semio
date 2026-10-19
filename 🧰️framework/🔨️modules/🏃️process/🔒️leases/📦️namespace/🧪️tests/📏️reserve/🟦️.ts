import authoritySchema from "../../../🧬️schema/📜️ticket.json" with {type:"json"};
import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
import {openSync,closeSync,writeSync,fsyncSync,ftruncateSync,fstatSync,lstatSync,statfsSync,unlinkSync,mkdirSync,type Stats} from "node:fs";
import {join} from "node:path";
import {withNativeImageStorageV1} from "../../../../📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts";
import {acquireResourceLease,requireResourceLeaseCustodyV1,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../🟦️.ts";
import {retainResourceQueueMetadataHeadroomV1} from "../../../📦️metadata/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../🎛️owned-execution/🟦️.ts";

function row(db:Database,sql:string):unknown{const statement=db.prepare(sql);try{return statement.get();}finally{statement.finalize();}}
function change(db:Database,sql:string,value:Buffer):void{const statement=db.prepare(sql);try{statement.run(value);}finally{statement.finalize();}}

/** 🧪️ Measures two real rollback journals after full-filesystem refusal and exact same-inode reserve recovery. */
export async function proveFiniteResourceReserveV1(directory:string,control:ResourceLeaseControl,environment:NodeJS.ProcessEnv):Promise<Record<string,boolean>>{
 return withNativeImageStorageV1({...{directory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control},leaseDirectory:join(directory,"..","leases")},async(image,control)=>{const root=image.root,data=join(root,"data");mkdirSync(data);
 const guard=await acquireResourceLease({directory:data,resource:authoritySchema.definitions.ClaimRetentionPolicy.properties.resource.const,mode:"exclusive",signal:control.signal,control,maximumDatabaseBytes:16384}),reservePath=join(data,"metadata-headroom.bin"),filler=join(root,"filler"),paths=[join(data,"catalog.sqlite"),join(data,"guard.sqlite")],databases:Database[]=[];
 const free=()=>{const fs=statfsSync(root);return fs.bavail*fs.bsize;},extent=(size:number)=>{requireResourceLeaseCustodyV1(guard,data,authoritySchema.definitions.ClaimRetentionPolicy.properties.resource.const,control);const fd=openSync(reservePath,"r+");try{assert.equal(fstatSync(fd).ino,reserveIdentity.ino);ftruncateSync(fd,size);fsyncSync(fd);}finally{closeSync(fd);}};
 let reserveIdentity!:Stats,fillFd:number|undefined;
 try{
  retainResourceQueueMetadataHeadroomV1(data,guard,control,requireResourceLeaseCustodyV1);reserveIdentity=lstatSync(reservePath);assert.equal(reserveIdentity.size,4194304);assert.equal(reserveIdentity.blocks*512,4194304);extent(131072);assert.equal(lstatSync(reservePath).blocks*512,131072);
  for(const path of paths){const db=new Database(path);databases.push(db);db.exec("PRAGMA page_size=4096;PRAGMA max_page_count=4;PRAGMA journal_mode=DELETE;PRAGMA synchronous=FULL;CREATE TABLE payload(id INTEGER PRIMARY KEY,bytes BLOB NOT NULL)");change(db,"INSERT INTO payload VALUES(1,?)",Buffer.alloc(10000,0x11));assert.equal((row(db,"PRAGMA page_count") as {page_count:number}).page_count,4);}
  const identities=paths.map(path=>lstatSync(path));fillFd=openSync(filler,"wx");const bytes=Buffer.alloc(65536,0xc3);let filled=0;
  for(const chunk of[bytes,Buffer.alloc(4096,0xc3)])for(;;){checkResourceLeaseControl(control);try{filled+=writeSync(fillFd,chunk);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOSPC")throw error;break;}}fsyncSync(fillFd);assert.equal(free(),0);
  let actualFullRefusedMutation=false;try{change(databases[0]!,"UPDATE payload SET bytes=? WHERE id=1",Buffer.alloc(10000,0x22));}catch(error){if(![13,14].includes((error as {errcode?:number;errno?:number}).errcode??(error as {errno?:number}).errno??-1))throw error;actualFullRefusedMutation=true;}assert.equal(actualFullRefusedMutation,true,"actual full volume must refuse journaled mutation before recovery");
  const originalRowsPreserved=databases.every(db=>(row(db,"SELECT hex(substr(bytes,1,1)) AS value FROM payload WHERE id=1") as {value:string}).value==="11");assert.equal(originalRowsPreserved,true);extent(0);const recovered=free();assert.ok(recovered>=131072);
  for(const db of databases){db.exec("BEGIN EXCLUSIVE");change(db,"UPDATE payload SET bytes=? WHERE id=1",Buffer.alloc(10000,0x22));}
  const journalBytes=paths.reduce((sum,path)=>sum+lstatSync(path+"-journal").blocks*512,0),peakAvailable=free();assert.ok(journalBytes>0&&journalBytes<=49152,"closed two-journal bound49152");for(const db of databases)db.exec("COMMIT");
  const fd=openSync(reservePath,"r+");try{for(let offset=0;offset<131072;offset+=bytes.length)writeSync(fd,bytes,0,bytes.length,offset);fsyncSync(fd);}finally{closeSync(fd);}const restored=lstatSync(reservePath);assert.equal(restored.size,131072);assert.equal(restored.blocks*512,131072);assert.equal(free(),0);
  const oracle="import sqlite3,sys,json;print(json.dumps([sqlite3.connect('file:'+p+'?mode=ro',uri=True).execute('select hex(substr(bytes,1,1)),length(bytes) from payload where id=1').fetchone() for p in sys.argv[1:]]))";assert.deepEqual(JSON.parse((await captureOwnedCommandV1("python3",["-c",oracle,...paths],root,{control,env:environment,output:{maximumBytes:4096,maximumLines:32}})).trim()),[["22",10000],["22",10000]]);
  const result={actualFullRefusedMutation,originalRowsPreserved,sameReserveInode:restored.dev===reserveIdentity.dev&&restored.ino===reserveIdentity.ino,twoJournalsWithinBound:journalBytes<=49152,commitAfterRecovery:databases.every(db=>!db.inTransaction),reserveRestoredAtFull:free()===0,sameGuardInodes:paths.every((path,index)=>{const now=lstatSync(path),old=identities[index]!;return now.dev===old.dev&&now.ino===old.ino;}),independentRowsConfirmed:true};
  await image.recheck();return result;
 }catch(error){throw error;}finally{
  const errors:unknown[]=[],step=(name:string,run:()=>void)=>{try{run();}catch(error){errors.push(error);}};
  if(fillFd!==undefined)step("filler close",()=>closeSync(fillFd!));step("filler unlink",()=>{try{unlinkSync(filler);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}});
  for(const db of databases){step("rollback",()=>{if(db.inTransaction)db.exec("ROLLBACK");});step("connection finalize",()=>db.close(true));}
  step("catalog release",()=>guard.release());if(errors.length)throw errors[0];
 }
});}
