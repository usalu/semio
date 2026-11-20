import {createHash,randomBytes} from "node:crypto";
import {lstatSync,mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {sha256V1,UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout;
/** ⏱️ A file touched less than this long before the admission began is racy: its stamp alone cannot prove that the bytes read are the bytes stamped, so it is remembered with the digest of the bytes read and recalled only when its bytes still hash to that digest. */
export const RACY_NANOSECONDS_V1=2_000_000_000n;
/** 🧾️ One observed path: its no-follow stamp, or the fact that it was absent. */
export type AdmissionStampV1=Readonly<{path:string;present:boolean;dev:string;ino:string;size:string;mtimeNs:string;ctimeNs:string;racy?:true}>;
/** 🧠️ A unit's complete admission result with the stamps of everything its source closure probed. */
export type AdmissionMemoV1=Readonly<{version:1;key:string;stamps:readonly AdmissionStampV1[];inputs:readonly Readonly<{path:string;digest:string;bytes:number}>[];cfg:readonly string[];environment:Readonly<Record<string,string|null>>|null;createdMs:number}>;
/** 🔌️ The unit as the memo sees it: every field that decides the admission result except the two outputs it computes. */
export type AdmissionUnitV1=object;

const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).sort(([left],[right])=>left<right?-1:left>right?1:0).map(([key,item])=>[key,canonical(item)])):value;
/** 🧷️ The unit facts that decide an admission result. Run-specific facts (unit ids, dependency edges, planning and compilation digests, the configuration digest and the invoking process's paths) never enter the key, so two plans of one workspace share it. */
export const ADMISSION_KEYED_FIELDS_V1:readonly string[]=Object.freeze(["packageId","packageName","packageVersion","edition","manifest","sourceRoot","provenance","entry","name","kind","operation","platform","roles","profileName","compilerProfile","profileDigest","compilerFlags","compilerArguments","features","boundary","support","compiler"]);
/** 🌬️ The names a unit's compiler environment copies from the invoking process as paths; they are left out of the key and replaced by the current values when an admission is recalled. */
export const ADMISSION_VOLATILE_ENVIRONMENT_V1:readonly string[]=Object.freeze(["PATH","SystemRoot","WINDIR","PATHEXT","RUSTUP_HOME"]);
type EnvironmentV1=Readonly<Record<string,string|null>>|null;
/** 🔑️ Identity of one admission: the keyed unit facts, the unit's compiler environment without its volatile names, the policy and the toolchain. */
export function admissionMemoKeyV1(unit:AdmissionUnitV1,policy:unknown,toolchain:string):string{
 const row=unit as Record<string,unknown>,environment=(row.compilerEnvironment??null) as EnvironmentV1;
 const rest={...Object.fromEntries(ADMISSION_KEYED_FIELDS_V1.map(name=>[name,row[name]??null])),environment:environment&&Object.fromEntries(Object.entries(environment).filter(([name])=>!ADMISSION_VOLATILE_ENVIRONMENT_V1.includes(name)))};
 return sha256V1(JSON.stringify(canonical([UNIT_CACHE_POLICY_V1.namespace,toolchain,policy,rest])));
}
/** 🌿️ A remembered compiler environment with the current values of the volatile names. */
export function admissionEnvironmentV1(remembered:EnvironmentV1,unit:AdmissionUnitV1):EnvironmentV1{
 const now=(unit as {compilerEnvironment?:EnvironmentV1}).compilerEnvironment;if(remembered===null||!now)return remembered;
 const result:Record<string,string|null>={...remembered};for(const name of ADMISSION_VOLATILE_ENVIRONMENT_V1)if(name in now)result[name]=now[name]!;else delete result[name];return result;
}

/** 📸️ Stamps a path without following links; a missing path is a recorded absence. */
export function stampPathV1(path:string):AdmissionStampV1{
 try{const stat=lstatSync(path,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink())return{path,present:true,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};return{path,present:true,dev:String(stat.dev),ino:String(stat.ino),size:String(stat.size),mtimeNs:String(stat.mtimeNs),ctimeNs:String(stat.ctimeNs)};}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{path,present:false,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};return{path,present:true,dev:"",ino:"",size:"",mtimeNs:"",ctimeNs:""};}
}

const sha256Hex=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const same=(left:AdmissionStampV1,right:AdmissionStampV1)=>left.present===right.present&&left.dev===right.dev&&left.ino===right.ino&&left.size===right.size&&left.mtimeNs===right.mtimeNs&&left.ctimeNs===right.ctimeNs&&(!left.present||left.ino!=="");
const file=(root:string,key:string)=>join(root,layout.admissions,key.slice(0,2),key+".json");

/** 🔎️ Returns the remembered admission when every probed path still carries its stamp, so no source byte is read again. Any doubt is a miss. */
export function recallAdmissionV1(root:string,key:string,stampOf:(path:string)=>AdmissionStampV1=stampPathV1):AdmissionMemoV1|undefined{
 let memo:AdmissionMemoV1;try{memo=JSON.parse(readFileSync(file(root,key),"utf8"));}catch{return undefined;}
 if(memo.version!==1||memo.key!==key||!Array.isArray(memo.stamps)||!Array.isArray(memo.inputs))return undefined;
 const digests=new Map(memo.inputs.map(input=>[input.path,input.digest]));
 for(const stamp of memo.stamps){if(!same(stamp,stampOf(stamp.path)))return undefined;if(stamp.racy){const digest=digests.get(stamp.path);try{if(digest===undefined||sha256Hex(readFileSync(stamp.path))!==digest)return undefined;}catch{return undefined;}}}
 return memo;
}

/** 💾️ Remembers an admission; the stamps are taken after the result was computed. A file touched shortly before or during the admission is remembered as racy with the digest of the bytes the admission read, so a stamp taken after a change cannot vouch for older bytes; a racy path without such a digest is not remembered. */
export function rememberAdmissionV1(root:string,key:string,result:Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"];probed:readonly string[];startedMs:number}>,stampOf:(path:string)=>AdmissionStampV1=stampPathV1,nowMs=Date.now()):boolean{
 const horizon=BigInt(Math.floor(result.startedMs))*1_000_000n-RACY_NANOSECONDS_V1,digested=new Set(result.inputs.map(input=>input.path)),stamps=[...new Set(result.probed)].sort().map(stampOf).map((stamp):AdmissionStampV1=>stamp.present&&stamp.ino!==""&&(BigInt(stamp.ctimeNs)>=horizon||BigInt(stamp.mtimeNs)>=horizon)?{...stamp,racy:true}:stamp);
 if(stamps.some(stamp=>stamp.present&&(stamp.ino===""||stamp.racy&&!digested.has(stamp.path))))return false;
 const target=file(root,key),temporary=target+"."+randomBytes(4).toString("hex");
 try{mkdirSync(dirname(target),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,key,stamps,inputs:result.inputs,cfg:result.cfg,environment:result.environment,createdMs:nowMs}));rmSync(target,{force:true});renameSync(temporary,target);return true;}catch{rmSync(temporary,{force:true});return false;}
}

/** 📝️ The memo port the source admission calls once per unit: a unit is remembered under the key it was recalled with, because its admission rewrites the very fields the key reads. */
export function unitAdmissionMemoV1(root:string,toolchain:string){
 const keys=new WeakMap<object,string>();
 return Object.freeze({
  recall(unit:AdmissionUnitV1,policy:unknown):Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"]}>|undefined{const key=admissionMemoKeyV1(unit,policy,toolchain);keys.set(unit,key);const memo=recallAdmissionV1(root,key);return memo&&{inputs:memo.inputs,cfg:memo.cfg,environment:admissionEnvironmentV1(memo.environment,unit)};},
  remember(unit:AdmissionUnitV1,policy:unknown,result:Readonly<{inputs:AdmissionMemoV1["inputs"];cfg:readonly string[];environment:AdmissionMemoV1["environment"];probed:readonly string[];startedMs:number}>):void{rememberAdmissionV1(root,keys.get(unit)??admissionMemoKeyV1(unit,policy,toolchain),result);},
 });
}
