import{resolve,sep}from"node:path";
export type CompilerDescriptorObservationV1=Readonly<{status:number|null;signal:string|null;reason?:"exit"|"timeout"|"cancelled"|"output-limit"|"spawn-error";stdout:string;stderr:string}>;
/** 🔐️ Only a complete global inventory may admit this exact physical guard's own lock descriptors. */
export function assertCompilerDescriptorCustodyV1(observation:CompilerDescriptorObservationV1,directory:string,guard:Readonly<{pid:number;paths:readonly string[]}>):void{
 if(observation.status!==0||observation.signal!==null||observation.reason!=="exit"||observation.stderr.trim()||!Number.isSafeInteger(guard.pid)||guard.pid<=0)throw Error("Complete compiler descriptor inventory required");
 const root=resolve(directory),seen=new Set<string>();let pid=0;
 for(const row of observation.stdout.split("\n")){if(/^p[0-9]+$/u.test(row))pid=Number(row.slice(1));else if(row.startsWith("n")){const path=row.slice(1);if(path===root||path.startsWith(root+sep)){if(pid!==guard.pid||!guard.paths.includes(path))throw Error("Compiler store has foreign open descriptors");seen.add(path);}}}
 if(guard.paths.some(path=>!seen.has(path)))throw Error("Physical compiler guard is absent from the complete descriptor inventory");
}
