import {createPackageManagerV1,type LockMemberV1,type LockPackageV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

/** 🔷️ Reads the restore lock: one table per target framework, `Project` rows are workspace members. */
export const readNugetLockV1:LockReaderV1=text=>{
 const lock=JSON.parse(text) as {version?:number;dependencies?:Record<string,Record<string,{type?:string;resolved?:string;contentHash?:string}>>};
 if(lock.version!==1||!lock.dependencies)throw Error("Supported restore lock required");
 const members:LockMemberV1[]=[],packages:LockPackageV1[]=[];
 for(const frameworkRows of Object.values(lock.dependencies))for(const [name,row] of Object.entries(frameworkRows)){
  if(row.type==="Project"){if(!members.some(member=>member.name===name))members.push({name,path:null});continue;}
  packages.push({name,version:row.resolved??"",key:name,source:row.type??null,integrity:row.contentHash??null});
 }
 return {packages,members};
};
/** 📦️ The adapter: descriptor data plus the lock reader. */
export const dotnetPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readNugetLockV1);
