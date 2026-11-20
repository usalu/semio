import{createHash,randomBytes}from"node:crypto";
import{mkdirSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{basename,dirname,join}from"node:path";
import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import containment from"../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🧬️schema/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

export const READY_SLICE_REPORT_POLICY_V1=schema.definitions.Owned.const;
export const SLICE_REPORT_SAMPLE_MILLISECONDS_V1=schema.definitions.Policy.const.intervalMilliseconds;
const policy=READY_SLICE_REPORT_POLICY_V1,pattern=new RegExp(schema.definitions.StageOwner.pattern,"u"),slotPattern=new RegExp(`^[0-9a-f]{${policy.slotHexCharacters}}$`,"u"),grantPattern=new RegExp(`^[0-9a-f]{${policy.grantHexCharacters}}$`,"u");
const directory=(root:string):string=>machinePathV1(root,...policy.directory);
const grantPath=(file:string):string=>join(dirname(file),policy.grantName);

/** 🏷️ The overlaid readiness slot for a valid stage owner. Only the readiness supervisor creates slots.
 * @see ../🧬️schema/🔣️.json */
export function ownedSliceReportV1(root:string,owner:string|undefined):Readonly<{owner:string;file:string}>|undefined{
 if(owner===undefined||!pattern.test(owner))return undefined;
 const key=createHash("sha256").update(owner).digest("hex").slice(0,policy.slotHexCharacters);
 return{owner,file:join(directory(root),key,schema.definitions.Policy.const.name)};
}

/** 🗂️ Opens the current stage report slot and removes obsolete slots under the readiness lease. Re-adoption preserves the current file. */
export function prepareOwnedSliceReportV1(root:string,owner:string):string|undefined{
 const slot=ownedSliceReportV1(root,owner);if(slot===undefined)return;
 const slotDirectory=dirname(slot.file),parent=dirname(slotDirectory),current=basename(slotDirectory);
 try{
  mkdirSync(parent,{recursive:true});
  for(const entry of readdirSync(parent,{withFileTypes:true})){
   if(entry.isDirectory()&&entry.name!==current&&slotPattern.test(entry.name))rmSync(join(parent,entry.name),{recursive:true,force:true,maxRetries:3,retryDelay:20});
  }
  mkdirSync(slotDirectory,{recursive:true,mode:0o700});
  const file=grantPath(slot.file);
  try{writeFileSync(file,randomBytes(policy.grantHexCharacters/2).toString("hex"),{flag:"wx",mode:0o600});}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")return undefined;}
  const grant=readFileSync(file,"utf8");
  return grantPattern.test(grant)?grant:undefined;
 }catch{}
}

/** 🖊️ Writes a machine report atomically with a process-specific temporary path. */
export function writeSliceReportTextV1(file:string,text:string):void{
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,text);renameSync(temporary,file);}
 catch{try{rmSync(temporary,{force:true});}catch{}}
}

/** 📮️ Accepts report snapshots only for a client carrying the grant issued with its stage owner. */
export function ownedSliceReportSinkV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):((text:string)=>void)|undefined{
 const slot=ownedSliceReportV1(root,environment[containment.definitions.Policy.const.ownerEnvironment]),grant=environment[policy.grantEnvironment];
 if(slot===undefined||grant===undefined||!grantPattern.test(grant))return undefined;
 try{if(readFileSync(grantPath(slot.file),"utf8")!==grant)return undefined;}catch{return undefined;}
 return text=>{
  if(typeof text!=="string"||Buffer.byteLength(text)>policy.fileBytes)return;
  try{
   const report=JSON.parse(text);
   if(report?.version!==1||!Number.isSafeInteger(report.pid)||report.pid<1)return;
   const bound=JSON.stringify({...report,readyOwner:slot.owner});
   if(Buffer.byteLength(bound)<=policy.fileBytes)writeSliceReportTextV1(slot.file,bound);
  }catch{}
 };
}
