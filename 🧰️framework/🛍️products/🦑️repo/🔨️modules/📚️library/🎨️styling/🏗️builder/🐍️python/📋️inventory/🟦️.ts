/** 📋️ Inventory: module exporting `StylingPythonInventoryV1`, `observeStylingPythonEnvironmentV1`. */
import{createHash}from"node:crypto";
import{lstat,opendir,readlink,realpath}from"node:fs/promises";
import{isAbsolute,join,relative,sep}from"node:path";
import{requireIssuedResourceControlV1}from"@semio-tech/process/routing/invocation/process";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"@semio-tech/process/leases";
import{observePhysicalFileV1,type PhysicalFileClaimV1}from"@semio-tech/framework/filesystem/observation";
import{withArtifactDescriptorV1}from"@semio-tech/framework/filesystem/observation/descriptor";
import{STYLING_PYTHON_POLICY_V1 as policy}from"@semio-tech/ui-styling/builder/python/contract";
/** 🧾️ Complete bounded byte observations; this DATA cannot issue environment or interpreter custody. */
export type StylingPythonInventoryV1=Readonly<{version:1;sha256:string;files:number;entries:number;bytes:number}>;
type Stat=Awaited<ReturnType<typeof lstat>>;
type Entry={path:string;stat:Stat;target?:string;resolved?:string;resolvedStat?:Stat};
const same=(a:Stat,b:Stat)=>a.dev===b.dev&&a.ino===b.ino&&a.mode===b.mode&&a.size===b.size&&a.mtimeMs===b.mtimeMs&&a.ctimeMs===b.ctimeMs;
const inside=(root:string,path:string)=>{const value=relative(root,path);return value===""||!isAbsolute(value)&&value!==".."&&!value.startsWith(".."+sep);};
/** 🔍️ Streams every entry before allocation and observes full descriptor bytes under the actual carried phase.
 * @see ../../🧬️schema/🔣️.json
 */
export async function observeStylingPythonEnvironmentV1(root:string,control:ResourceLeaseControl,verifyExternalInterpreter:(claim:PhysicalFileClaimV1,control:ResourceLeaseControl)=>Promise<void>):Promise<StylingPythonInventoryV1>{
 requireIssuedResourceControlV1(control);if(!isAbsolute(root)||typeof verifyExternalInterpreter!=="function")throw Error("Actual environment observation required");
 const check=()=>checkResourceLeaseControl(control),entries:Entry[]=[],rows:unknown[]=[],directories:{path:string;names:string[]}[]=[];let files=0,bytes=0,reserved=1;
 const pathCheck=(path:string)=>{check();if(path.length>policy.maximumEnvironmentPathUnits)throw Error("Python environment path bound exceeded");};
 const charge=(length:number)=>{if(!Number.isSafeInteger(length)||length<0||length>policy.maximumEnvironmentBytes-bytes)throw Error("Python environment byte bound exceeded");bytes+=length;};
 const observe=async(path:string,length:number)=>observePhysicalFileV1(path,{maxBytes:length,maxWork:Math.ceil(length/65536)+5,chunkBytes:65536,cancelled:()=>control.signal.aborted,remainingMs:()=>check()??Number.MAX_SAFE_INTEGER,onProgress:()=>{check();}});
 const list=async(path:string,admit:(name:string)=>void)=>withArtifactDescriptorV1(()=>opendir(path,{bufferSize:1}),async directory=>{for(;;){check();const entry=await directory.read();check();if(!entry)break;admit(entry.name);await control.yieldContinuation();}});
 const canonical=await realpath(root);check();pathCheck(canonical);const initial=await lstat(root);if(!initial.isDirectory()||initial.isSymbolicLink())throw Error("Actual Python environment root required");
 const visit=async(path:string,depth:number):Promise<void>=>{pathCheck(path);if(depth>policy.maximumEnvironmentDepth||entries.length>=policy.maximumEnvironmentEntries)throw Error("Python environment entry bound exceeded");const stat=await lstat(path);check();const entry:Entry={path,stat};entries.push(entry);const name=relative(root,path).split(sep).join("/");
  if(stat.isDirectory()&&!stat.isSymbolicLink()){const names:string[]=[];await list(path,item=>{if(reserved>=policy.maximumEnvironmentEntries)throw Error("Python environment entry bound exceeded");pathCheck(join(path,item));reserved++;names.push(item);});names.sort();directories.push({path,names});rows.push([name,"directory",stat.mode]);for(const item of names)await visit(join(path,item),depth+1);}
  else if(stat.isFile()&&!stat.isSymbolicLink()){if(files>=policy.maximumEnvironmentFiles)throw Error("Python environment file bound exceeded");files++;charge(stat.size);const claim=await observe(path,stat.size);rows.push([name,"file",stat.mode,claim.byteLength,claim.sha256]);}
  else if(stat.isSymbolicLink()){const target=await readlink(path);check();if(target.length>policy.maximumEnvironmentPathUnits)throw Error("Python environment link bound exceeded");charge(Buffer.byteLength(target));const resolved=await realpath(path);check();pathCheck(resolved);entry.target=target;entry.resolved=resolved;const linked=await lstat(resolved);entry.resolvedStat=linked;check();if(inside(canonical,resolved)){const parts=relative(canonical,resolved).split(sep).filter(Boolean);let current=canonical;for(const part of parts.slice(0,-1)){current=join(current,part);const ancestor=await lstat(current);check();if(!ancestor.isDirectory()||ancestor.isSymbolicLink())throw Error("Contained Python link ancestry changed");}if(!linked.isFile()&&!linked.isDirectory()||linked.isSymbolicLink())throw Error("Contained Python link target required");}
   else{if(files>=policy.maximumEnvironmentFiles)throw Error("Python environment file bound exceeded");files++;if(!linked.isFile()||linked.isSymbolicLink())throw Error("Unknown external Python environment link");charge(linked.size);const claim=await observe(resolved,linked.size);await verifyExternalInterpreter(claim,control);check();rows.push([name,"interpreter",claim.byteLength,claim.sha256]);}
   rows.push([name,"link",stat.mode,target,relative(canonical,resolved).split(sep).join("/")]);
  }else throw Error("Unknown Python environment entry");
  if(!same(stat,await lstat(path)))throw Error("Python environment entry changed");check();await control.yieldContinuation();
 };
 await visit(root,0);
 for(const directory of directories){let index=0;const observed:string[]=[];await list(directory.path,name=>{if(index>=directory.names.length)throw Error("Python environment membership changed");index++;observed.push(name);});observed.sort();if(observed.some((name,i)=>name!==directory.names[i])||index!==directory.names.length)throw Error("Python environment membership changed");}
 for(const entry of entries){check();if(!same(entry.stat,await lstat(entry.path)))throw Error("Python environment entry changed");if(entry.target!==undefined&&(await readlink(entry.path)!==entry.target||await realpath(entry.path)!==entry.resolved||entry.resolved!==undefined&&entry.resolvedStat!==undefined&&!same(entry.resolvedStat,await lstat(entry.resolved))))throw Error("Python environment link changed");await control.yieldContinuation();}
 if(await realpath(root)!==canonical||!same(initial,await lstat(root)))throw Error("Python environment root changed");check();
 rows.sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:JSON.stringify(a)>JSON.stringify(b)?1:0);return Object.freeze({version:1,sha256:createHash("sha256").update(JSON.stringify(rows)).digest("hex"),files,entries:entries.length,bytes});
}
