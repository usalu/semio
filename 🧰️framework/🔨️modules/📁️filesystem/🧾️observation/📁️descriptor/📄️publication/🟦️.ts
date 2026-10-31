import {types} from "node:util";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {ArtifactReadFileIdentityV1} from "../🟦️.ts";
const fields=["dev","ino","mode","size","nlink","mtimeNs","ctimeNs"] as const;
const preparedFields=["version","platform","parent","name","stage","prior","retainedName","names","authority","windows"] as const;
const maximum=(1n<<128n)-1n,signedMaximum=(1n<<127n)-1n,policy=schema.definitions.RelativeFilePublicationProtocol.const;
/** 🪪️ Portable lossless facts, separate from every live filesystem capability. */
export type ArtifactRelativeFileIdentityDataV1=Readonly<Record<typeof fields[number],string>>;
/** 🪟️ NTFS identity and security facts remain portable data, never native handles. */
export interface ArtifactRelativeWindowsPublicationFactsV1{readonly volumeSerial:string;readonly fileId:string;readonly attributes:number;readonly security:string;readonly streams:string}
/** 🧷️ Every persisted Windows owner retains the complete existing platform fence. */
export interface ArtifactRelativeWindowsPublicationV1{readonly parent:ArtifactRelativeWindowsPublicationFactsV1;readonly stage:ArtifactRelativeWindowsPublicationFactsV1;readonly prior:ArtifactRelativeWindowsPublicationFactsV1|null}
/** 📜️ Detached durable preconditions never issue a root, entry or transition authority. */
export interface ArtifactRelativeFilePublicationPreparedDataV1{readonly version:1;readonly platform:"posix"|"windows";readonly parent:ArtifactRelativeFileIdentityDataV1;readonly name:string;readonly stage:Readonly<{name:string;identity:ArtifactRelativeFileIdentityDataV1;sha256:string}>;readonly prior:Readonly<{identity:ArtifactRelativeFileIdentityDataV1;sha256:string}>|null;readonly retainedName:string|null;readonly names:readonly string[];readonly authority:false;readonly windows:ArtifactRelativeWindowsPublicationV1|null}
/** 🧊️ Actual prepared facts are copied while the stage and original parent remain held. */
export interface ArtifactRelativeFilePublicationPreparedV1{readonly version:1;readonly platform:"posix"|"windows";readonly parent:ArtifactReadFileIdentityV1;readonly name:string;readonly stage:Readonly<{name:string;identity:ArtifactReadFileIdentityV1;sha256:string}>;readonly prior:Readonly<{identity:ArtifactReadFileIdentityV1;sha256:string}>|null;readonly retainedName:string|null;readonly names:readonly string[];readonly authority:false;readonly windows:ArtifactRelativeWindowsPublicationV1|null}
/** 📝️ The owner persists outside this directory before any publication rename begins. */
export interface ArtifactRelativeFilePublicationHooksV1{readonly prepared:(event:ArtifactRelativeFilePublicationPreparedV1)=>void|Promise<void>}
/** 🧭️ A newly observed layout still needs its original live private recovery token. */
export type ArtifactRelativeFileRecoveryStateV1="prepared"|"old-displaced"|"published-with-retained"|"completed";
/** 🎟️ Public state is descriptive; only the issuing live frame recognizes this object. */
export interface ArtifactRelativeFileRecoveryV1{readonly name:string;readonly state:ArtifactRelativeFileRecoveryStateV1;readonly authority:false}
/** 📆️ Internally reserved names can be journaled before exclusive stage creation. */
export interface ArtifactRelativeFilePublicationPlanV1{readonly name:string;readonly stageName:string;readonly retainedName:string|null;readonly windows:Readonly<{parent:ArtifactRelativeWindowsPublicationFactsV1;prior:ArtifactRelativeWindowsPublicationFactsV1|null}>|null}
/** 🔎️ Current exact entry facts remain data and carry no metadata cleanup grant. */
export interface ArtifactRelativeFilePublicationAttestationV1{readonly name:string;readonly parent:ArtifactReadFileIdentityV1;readonly identity:ArtifactReadFileIdentityV1;readonly sha256:string;readonly windows:Readonly<{parent:ArtifactRelativeWindowsPublicationFactsV1;entry:ArtifactRelativeWindowsPublicationFactsV1}>|null;readonly authority:false}
/** 📎️ Fresh descriptor observations, including every unrelated basename, drive classification. */
export interface ArtifactRelativeFilePublicationObservationV1{readonly parent:ArtifactReadFileIdentityV1;readonly names:readonly string[];readonly target:Readonly<{identity:ArtifactReadFileIdentityV1;sha256:string}>|null;readonly stage:Readonly<{identity:ArtifactReadFileIdentityV1;sha256:string}>|null;readonly retained:Readonly<{identity:ArtifactReadFileIdentityV1;sha256:string}>|null;readonly windows:Readonly<{parent:ArtifactRelativeWindowsPublicationFactsV1;target:ArtifactRelativeWindowsPublicationFactsV1|null;stage:ArtifactRelativeWindowsPublicationFactsV1|null;retained:ArtifactRelativeWindowsPublicationFactsV1|null}>|null}
function record(value:unknown,keys:readonly string[]):Record<string,unknown>{
 if(value===null||typeof value!=="object"||types.isProxy(value))throw Error("Ordinary closed publication record required");
 const prototype=Object.getPrototypeOf(value);if(prototype!==Object.prototype&&prototype!==null)throw Error("Plain publication record required");
 const names=Reflect.ownKeys(value);if(names.length!==keys.length||names.some(name=>typeof name!=="string"||!keys.includes(name)))throw Error("Exact publication record fields required");
 const result:Record<string,unknown>={};for(const key of keys){const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!("value"in field))throw Error("Publication data accessors are refused");result[key]=field.value;}return result;
}
function name(value:unknown):string{if(typeof value!=="string"||!value.length||value.length>255||value==="."||value===".."||/[/\\\0]/u.test(value)||Buffer.from(value,"utf8").toString("utf8")!==value)throw Error("Bounded publication basename required");return value;}
function digest(value:unknown):string{if(typeof value!=="string"||!/^[a-f0-9]{64}$/u.test(value))throw Error("Complete publication SHA256 required");return value;}
function names(value:unknown):readonly string[]{
 if(!Array.isArray(value)||types.isProxy(value)||Object.getPrototypeOf(value)!==Array.prototype)throw Error("Ordinary publication roster required");
 const length=Object.getOwnPropertyDescriptor(value,"length")!.value;if(!Number.isSafeInteger(length)||length<0||length>schema.definitions.RelativeProtocol.const.maximumDirectoryEntries||Reflect.ownKeys(value).length!==length+1)throw Error("Finite closed publication roster required");
 const result:string[]=[];for(let index=0;index<length;index++){const field=Object.getOwnPropertyDescriptor(value,String(index));if(!field||!("value"in field))throw Error("Publication roster accessors are refused");result.push(name(field.value));}
 const ordered=[...result].sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)));if(new Set(result).size!==length||result.some((value,index)=>value!==ordered[index]))throw Error("Exact sorted disjoint publication roster required");return Object.freeze(result);
}
function scalar(value:unknown,key:typeof fields[number]):bigint{const signed=key==="mtimeNs"||key==="ctimeNs";if(typeof value!=="bigint"||value>(signed?signedMaximum:maximum)||value<(signed?-signedMaximum-1n:0n))throw Error("Bounded actual publication identity required");return value;}
function identity(value:unknown):ArtifactReadFileIdentityV1{const row=record(value,fields);return Object.freeze(Object.fromEntries(fields.map(key=>[key,scalar(row[key],key)]))) as unknown as ArtifactReadFileIdentityV1;}
function encodeIdentity(value:unknown):ArtifactRelativeFileIdentityDataV1{const row=identity(value);return Object.freeze(Object.fromEntries(fields.map(key=>[key,row[key].toString()]))) as ArtifactRelativeFileIdentityDataV1;}
function decodeIdentity(value:unknown):ArtifactReadFileIdentityV1{
 const row=record(value,fields),result:Record<string,bigint>={};for(const key of fields){const text=row[key];if(typeof text!=="string"||text.length>40||!/^-?(?:0|[1-9][0-9]*)$/u.test(text))throw Error("Canonical bounded decimal publication identity required");const number=BigInt(text);if(number.toString()!==text)throw Error("Canonical publication identity spelling required");result[key]=scalar(number,key);}return Object.freeze(result) as unknown as ArtifactReadFileIdentityV1;
}
function windowsFacts(value:unknown):ArtifactRelativeWindowsPublicationFactsV1{const row=record(value,["volumeSerial","fileId","attributes","security","streams"]);if(typeof row.volumeSerial!=="string"||row.volumeSerial.length>20||!/^(?:0|[1-9][0-9]*)$/u.test(row.volumeSerial)||BigInt(row.volumeSerial)>18446744073709551615n||typeof row.fileId!=="string"||!/^[a-f0-9]{32}$/u.test(row.fileId)||typeof row.attributes!=="number"||!Number.isSafeInteger(row.attributes)||row.attributes<0||row.attributes>4294967295)throw Error("Complete bounded Windows publication facts required");return Object.freeze({volumeSerial:row.volumeSerial,fileId:row.fileId,attributes:row.attributes,security:digest(row.security),streams:digest(row.streams)});}
function windows(value:unknown,prior:boolean,platform:"posix"|"windows"):ArtifactRelativeWindowsPublicationV1|null{if(platform==="posix"){if(value!==null)throw Error("POSIX publication has no Windows facts");return null;}const row=record(value,["parent","stage","prior"]);if((row.prior!==null)!==prior)throw Error("Exact Windows prior facts required");return Object.freeze({parent:windowsFacts(row.parent),stage:windowsFacts(row.stage),prior:row.prior===null?null:windowsFacts(row.prior)});}
/** 🔐️ Windows security, stream and 128-bit identity facts never weaken on rename. */
export function sameArtifactRelativeWindowsPublicationV1(a:ArtifactRelativeWindowsPublicationFactsV1,b:ArtifactRelativeWindowsPublicationFactsV1):boolean{return a.volumeSerial===b.volumeSerial&&a.fileId===b.fileId&&a.attributes===b.attributes&&a.security===b.security&&a.streams===b.streams;}
/** 🎲️ Recognizes only the exact finite basename grammar used by the defining stage generator. */
export function artifactRelativeFilePublicationStageNameV1(targetName:string,candidateName:string):boolean{
 if(typeof targetName!=="string"||typeof candidateName!=="string"||!targetName.length||targetName.length>255||targetName==="."||targetName===".."||/[/\\\0]/u.test(targetName)||Buffer.from(targetName,"utf8").toString("utf8")!==targetName)return false;
 const count=Math.min(targetName.length,schema.definitions.RelativeBindingProtocol.const.maximumStageUnits);return candidateName!==targetName&&candidateName.length===count&&(count===1?/^[a-f0-9]$/u.test(candidateName):candidateName[0]==="."&&new RegExp("^[a-f0-9]{"+(count-1)+"}$","u").test(candidateName.slice(1)));
}
/** 🛡️ Capture callback identity before any stage reservation is consumed or native effect starts. */
export function captureArtifactRelativeFilePublicationHookV1(value:unknown):ArtifactRelativeFilePublicationHooksV1["prepared"]|undefined{
 if(value===undefined)return;const callback=record(value,["prepared"]).prepared;if(typeof callback!=="function"||types.isProxy(callback))throw Error("Ordinary actual publication preparation callback required");return callback as ArtifactRelativeFilePublicationHooksV1["prepared"];
}
/** ⏳️ An ordinary promise or void is the only accepted asynchronous owner acknowledgement. */
export async function invokeArtifactRelativeFilePublicationHookV1(callback:ArtifactRelativeFilePublicationHooksV1["prepared"]|undefined,event:ArtifactRelativeFilePublicationPreparedV1):Promise<void>{
 if(!callback)return;const result=callback(event);if(result!==undefined&&(!types.isPromise(result)||types.isProxy(result)))throw Error("Ordinary publication owner acknowledgement required");if(await result!==undefined)throw Error("Void publication owner acknowledgement required");
}
/** 🧱️ Keep only primitive actual facts, with no frame, getter or entry capability escaping. */
export function makeArtifactRelativeFilePublicationPreparedV1(value:ArtifactRelativeFilePublicationPreparedV1):ArtifactRelativeFilePublicationPreparedV1{
 return Object.freeze({version:1,platform:value.platform,parent:identity(value.parent),name:value.name,stage:Object.freeze({name:value.stage.name,identity:identity(value.stage.identity),sha256:value.stage.sha256}),prior:value.prior?Object.freeze({identity:identity(value.prior.identity),sha256:value.prior.sha256}):null,retainedName:value.retainedName,names:Object.freeze([...value.names]),authority:false,windows:windows(value.windows,value.prior!==null,value.platform)});
}
/** 🧬️ Serialize actual data into the defining language-neutral recovery schema without getters. */
export function artifactRelativeFilePublicationPreparedDataV1(value:ArtifactRelativeFilePublicationPreparedV1):ArtifactRelativeFilePublicationPreparedDataV1{
 const row=record(value,preparedFields),stage=record(row.stage,["name","identity","sha256"]),prior=row.prior===null?null:record(row.prior,["identity","sha256"]);
 const result=Object.freeze({version:row.version,platform:row.platform,parent:encodeIdentity(row.parent),name:row.name,stage:Object.freeze({name:stage.name,identity:encodeIdentity(stage.identity),sha256:stage.sha256}),prior:prior?Object.freeze({identity:encodeIdentity(prior.identity),sha256:prior.sha256}):null,retainedName:row.retainedName,names:names(row.names),authority:row.authority,windows:row.windows});
 captureArtifactRelativeFilePublicationPreparedDataV1(result);return result as ArtifactRelativeFilePublicationPreparedDataV1;
}
/** 📖️ Rehydrate only closed bounded decimal DATA; a live root still has to prove every fact. */
export function captureArtifactRelativeFilePublicationPreparedDataV1(value:unknown):ArtifactRelativeFilePublicationPreparedV1{
 const row=record(value,preparedFields);if(row.version!==1||row.authority!==false||row.platform!=="posix"&&row.platform!=="windows")throw Error("Exact prepared publication data version required");
 const target=name(row.name),roster=names(row.names),stage=record(row.stage,["name","identity","sha256"]),stageName=name(stage.name),prior=row.prior===null?null:record(row.prior,["identity","sha256"]),retainedName=row.retainedName===null?null:name(row.retainedName);
 if(!artifactRelativeFilePublicationStageNameV1(target,stageName)||roster.includes(stageName)||roster.includes(target)!==Boolean(prior)||(prior?(retainedName===null||row.platform==="posix"&&retainedName!==stageName||row.platform==="windows"&&(!artifactRelativeFilePublicationStageNameV1(target,retainedName!)||retainedName===stageName||roster.includes(retainedName!))):retainedName!==null))throw Error("Exact prepared publication namespace data required");
 const result=makeArtifactRelativeFilePublicationPreparedV1({version:1,platform:row.platform,parent:decodeIdentity(row.parent),name:target,stage:{name:stageName,identity:decodeIdentity(stage.identity),sha256:digest(stage.sha256)},prior:prior?{identity:decodeIdentity(prior.identity),sha256:digest(prior.sha256)}:null,retainedName,names:roster,authority:false,windows:windows(row.windows,prior!==null,row.platform)});
 if((result.parent.mode&0o170000n)!==0o040000n)throw Error("Actual original directory identity data required");
 for(const leaf of [result.stage,...(result.prior?[result.prior]:[])])if((leaf.identity.mode&0o170000n)!==0o100000n||leaf.identity.nlink!==1n||leaf.identity.size>BigInt(policy.maximumFileBytes))throw Error("Bounded single regular prepared file data required");return result;
}
/** 🪞️ Namespace moves preserve all six stable inode facts and only advance ctime. */
export function sameArtifactRelativePublicationFileV1(before:ArtifactReadFileIdentityV1,after:ArtifactReadFileIdentityV1,moved=false,windows=false):boolean{return fields.every(key=>moved&&(key==="ctimeNs"||windows&&key==="mtimeNs")?after[key]>=before[key]:after[key]===before[key]);}
/** 🧩️ Classifies first-party observations only; this result cannot resume or delete a file. */
export function classifyArtifactRelativeFileRecoveryV1(prepared:ArtifactRelativeFilePublicationPreparedV1,current:ArtifactRelativeFilePublicationObservationV1):ArtifactRelativeFileRecoveryStateV1{
 if(prepared.parent.dev!==current.parent.dev||prepared.parent.ino!==current.parent.ino||prepared.parent.mode!==current.parent.mode)throw Error("Original prepared publication root identity required");
 if(prepared.platform==="windows"?(!prepared.windows||!current.windows||!sameArtifactRelativeWindowsPublicationV1(prepared.windows.parent,current.windows.parent)):current.windows!==null)throw Error("Complete original Windows publication root facts required");const base=prepared.names.filter(value=>value!==prepared.name),equalNames=(desired:readonly string[])=>JSON.stringify(current.names)===JSON.stringify([...desired].sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)))),matches=(slot:"target"|"stage"|"retained",role:"stage"|"prior",moved:boolean)=>{const leaf=current[slot],expected=prepared[role],native=current.windows?.[slot],original=prepared.windows?.[role];return leaf!==null&&expected!==null&&leaf.sha256===expected.sha256&&sameArtifactRelativePublicationFileV1(expected.identity,leaf.identity,moved,prepared.platform==="windows")&&(prepared.platform!=="windows"||Boolean(native&&original&&sameArtifactRelativeWindowsPublicationV1(original,native)));};
 if(matches("stage","stage",false)&&(prepared.prior?matches("target","prior",false):current.target===null)&&(prepared.platform==="posix"||current.retained===null)&&equalNames([...base,...(prepared.prior?[prepared.name]:[]),prepared.stage.name]))return "prepared";
 if(prepared.platform==="windows"&&prepared.prior&&current.target===null&&matches("stage","stage",false)&&matches("retained","prior",true)&&equalNames([...base,prepared.stage.name,prepared.retainedName!]))return "old-displaced";
 if(matches("target","stage",true)&&prepared.prior&&matches("retained","prior",true)&&(prepared.platform==="windows"?current.stage===null:matches("stage","prior",true))&&equalNames([...base,prepared.name,prepared.retainedName!]))return "published-with-retained";
 if(matches("target","stage",true)&&current.stage===null&&current.retained===null&&equalNames([...base,prepared.name]))return "completed";
 throw Error("Exact prepared, displaced, published or completed publication layout required");
}

/** 🧮️ A resumed token fences every newly observed fact exactly, before any effect. */
export function sameArtifactRelativePublicationObservationV1(a:ArtifactRelativeFilePublicationObservationV1,b:ArtifactRelativeFilePublicationObservationV1):boolean{if(!sameArtifactRelativePublicationFileV1(a.parent,b.parent)||JSON.stringify(a.names)!==JSON.stringify(b.names)||(a.windows===null)!==(b.windows===null))return false;for(const slot of ["target","stage","retained"]as const){const before=a[slot],after=b[slot];if((before===null)!==(after===null)||before&&after&&(before.sha256!==after.sha256||!sameArtifactRelativePublicationFileV1(before.identity,after.identity)))return false;}if(a.windows&&b.windows){for(const slot of ["parent","target","stage","retained"]as const){const before=a.windows[slot],after=b.windows[slot];if((before===null)!==(after===null)||before&&after&&!sameArtifactRelativeWindowsPublicationV1(before,after))return false;}}return true;}
