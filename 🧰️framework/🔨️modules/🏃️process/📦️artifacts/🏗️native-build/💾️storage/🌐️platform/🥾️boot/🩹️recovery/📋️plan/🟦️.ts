import{createHash}from"node:crypto";
import{validateJsonSchemaSubset}from"../../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
const protocol=Object.freeze({...schema.definitions.Protocol.const});

/** 📋️ Names one persisted record family whose recovery decision is a pure function of finite facts. */
export type NativeBootFamilyV1="publication-residue"|"lease-ticket"|"lease-claims"|"tool-server"|"directory-swap"|"persistent-record"|"creator-claim"|"image-receipt"|"leaked-image";

/** 🧾️ Carries only closed observed facts of one scope; it holds no path authority and no live handle. */
export type NativeBootFactV1=Readonly<Record<string,unknown>&{family:NativeBootFamilyV1;scope:string}>;

/** 👁️ Describes one partition or tree at one instant of one boot session. */
export type NativeBootObservationV1=Readonly<{version:1;bootSession:string;bootEpochMs:number;facts:readonly NativeBootFactV1[]}>;

/** 🪜️ Orders one idempotent recovery effect inside the closed phase table. */
export type NativeBootStepV1=Readonly<{index:number;phase:number;family:"publication-residue"|"lease-ticket"|"lease-claims"|"tool-server"|"directory-swap"|"leaked-image";scope:string;action:"remove-file"|"remove-directory"|"retire-ticket"|"retire-settled-claims"|"recover-directory-swap"|"detach-image";target:string;reason:string}>;

/** 🔒️ Records a decision that deliberately changes nothing: live owners, owner-resumed records and refusals. */
export type NativeBootRetainedV1=Readonly<{family:NativeBootFamilyV1;scope:string;verdict:"keep"|"refuse";reason:string}>;

/** 📜️ Lists the complete ordered recovery of one observation. */
export type NativeBootPlanV1=Readonly<{version:1;bootSession:string;steps:readonly NativeBootStepV1[];retained:readonly NativeBootRetainedV1[]}>;

type Verdict=Readonly<{verdict:"keep"|"act"|"refuse";reason:string;effects:readonly Readonly<{action:NativeBootStepV1["action"];target:string}>[]}>;
const keep=(reason:string):Verdict=>({verdict:"keep",reason,effects:[]}),refuse=(reason:string):Verdict=>({verdict:"refuse",reason,effects:[]}),act=(reason:string,effects:Verdict["effects"]):Verdict=>({verdict:"act",reason,effects});
const publicationDirectory=protocol.publication.directory,[intentFile,receiptFile]=protocol.publication.files as readonly string[] as readonly[string,string],intentPath=publicationDirectory+"/"+intentFile,receiptPath=publicationDirectory+"/"+receiptFile;

function decidePublication(f:Record<string,any>):Verdict{
 if(!f.writerFree)return keep("live-writer");
 if(f.directory==="absent")return keep("settled");
 if(f.directory==="unsafe"||f.unknownEntries>0||f.intent==="unsafe"||f.receipt==="unsafe")return refuse("unknown-shape");
 const candidate=f.receipt!=="absent"?[{action:"remove-file" as const,target:receiptPath}]:[],close=[{action:"remove-file" as const,target:intentPath},{action:"remove-directory" as const,target:publicationDirectory}];
 if(f.intent==="absent")return f.receipt==="absent"?act("empty-directory",[{action:"remove-directory",target:publicationDirectory}]):refuse("candidate-without-intent");
 if(f.intent==="torn")return f.receipt==="absent"?act("torn-intent",close):refuse("torn-intent-with-candidate");
 if(f.image.state!=="valid"||f.intentReceiptSha256===null||f.image.sha256===null)return refuse("receipt-unavailable");
 if(f.image.sha256===f.intentReceiptSha256)return act("rolled-back",[...candidate,...close]);
 return f.receipt==="absent"?act("completed-before-crash",close):refuse("ambiguous-candidate");
}
function decideTicket(f:Record<string,any>,boot:NativeBootObservationV1):Verdict{
 if(!f.parsed||f.arrivedAtMs===null||f.pid===null)return refuse("unknown-name");
 return f.arrivedAtMs+protocol.toleranceMilliseconds<=boot.bootEpochMs?act("previous-boot",[{action:"retire-ticket",target:f.queue+"/"+f.name}]):keep("current-boot");
}
function decideServer(f:Record<string,any>,boot:NativeBootObservationV1):Verdict{
 const stale=(f.files as readonly{path:string;modifiedAtMs:number}[]).filter(row=>row.modifiedAtMs+protocol.toleranceMilliseconds<=boot.bootEpochMs);
 return stale.length?act("previous-boot",stale.map(row=>({action:"remove-file" as const,target:row.path}))):keep(f.files.length?"current-boot":"settled");
}
function decideRecord(f:Record<string,any>):Verdict{
 const a=f.final,b=f.temporary;
 if(a.state==="absent"&&b.state==="absent")return keep("settled");
 if(a.state==="absent"&&b.state==="present")return b.links===1?keep("owner-resumes-prefix"):refuse("unknown-links");
 if(a.state==="present"&&b.state==="absent")return a.links===1?keep("published"):refuse("unknown-links");
 return a.links===2&&b.links===2&&f.pair==="same-inode"?keep("owner-resumes-link"):refuse("unpaired-transaction");
}
function decideLeak(f:Record<string,any>):Verdict{
 if(f.backing==="absent")return act("backing-removed",[{action:"detach-image",target:f.key}]);
 const stale=f.location==="workspace"&&f.attachment!==undefined;
 if(f.location!=="scratch"&&!stale)return keep("owned");
 if(!f.idle)return keep("live");
 if(f.lease==="unavailable")return keep("unleased");
 return f.lease==="free"?act(stale?"stale-attachment":"abandoned",[{action:"detach-image",target:f.key}]):keep("live");
}
const bound=(binding:string,ok:string):Verdict=>binding==="mismatch"?refuse("identity-mismatch"):keep(binding==="not-applicable"?"settled":ok);

/** ⚖️ Decides one fact; the decision never reads the filesystem, a clock or a process table. */
export function decideNativeBootFactV1(fact:NativeBootFactV1,observation:NativeBootObservationV1):Readonly<Verdict>{
 const f=fact as Record<string,any>;
 switch(fact.family){
  case"publication-residue":return decidePublication(f);
  case"lease-ticket":return decideTicket(f,observation);
  case"lease-claims":return f.claimDatabases>0&&f.previousBootClaimDatabases>0?act("previous-boot",[{action:"retire-settled-claims",target:protocol.claimsDirectory}]):keep("settled");
  case"tool-server":return decideServer(f,observation);
  case"directory-swap":return !f.writerFree?keep("live-writer"):f.intent==="present"?act("pending-swap",[{action:"recover-directory-swap",target:"."}]):keep("settled");
  case"persistent-record":return decideRecord(f);
  case"creator-claim":return f.sidecar==="absent"?keep("settled"):bound(f.binding,f.binding==="rebound"?"owner-retires-rebound":"owner-retires");
  case"leaked-image":return decideLeak(f);
  case"image-receipt":return f.state==="unreadable"?refuse("unreadable-receipt"):f.state==="absent"?keep("unpublished"):bound(f.binding,f.binding==="rebound"?"bound-at-open":"current");
 }
}

const familyOrder:Readonly<Record<string,number>>={"lease-ticket":protocol.phases["lease-ticket"],"lease-claims":protocol.phases["lease-catalog"],"publication-residue":protocol.phases["publication-residue"],"directory-swap":protocol.phases["directory-swap"],"tool-server":protocol.phases["tool-server"],"leaked-image":protocol.phases["leaked-image"]};

/** 🗺️ Plans the ordered idempotent effects and the retained decisions of one observation. */
export function planNativeBootRecoveryV1(value:unknown):NativeBootPlanV1{
 const errors=validateJsonSchemaSubset(schema.definitions.Observation,value,schema);if(errors.length)throw Error("Closed boot recovery observation required: "+errors.slice(0,3).join("; "));
 const observation=value as NativeBootObservationV1,rows:{phase:number;family:NativeBootStepV1["family"];scope:string;action:NativeBootStepV1["action"];target:string;reason:string;order:number}[]=[],retained:NativeBootRetainedV1[]=[];
 const claimTickets=new Set<string>();let order=0;
 for(const fact of observation.facts){
  const result=decideNativeBootFactV1(fact,observation);
  if(result.verdict!=="act"){retained.push(Object.freeze({family:fact.family,scope:fact.scope,verdict:result.verdict,reason:result.reason}));continue;}
  for(const effect of result.effects){rows.push({phase:familyOrder[fact.family]!,family:fact.family as NativeBootStepV1["family"],scope:fact.scope,action:effect.action,target:effect.target,reason:result.reason,order:order++});if(effect.action==="retire-ticket")claimTickets.add(fact.scope);}
 }
 for(const scope of claimTickets)if(!rows.some(row=>row.action==="retire-settled-claims"&&row.scope===scope))rows.push({phase:protocol.phases["lease-catalog"],family:"lease-claims",scope,action:"retire-settled-claims",target:protocol.claimsDirectory,reason:"retired-tickets",order:order++});
 rows.sort((a,b)=>a.phase-b.phase||(a.scope<b.scope?-1:a.scope>b.scope?1:0)||a.order-b.order);
 if(rows.length>protocol.maximumSteps)throw Error("Bounded boot recovery plan required");
 const steps=rows.map((row,index)=>Object.freeze({index,phase:row.phase,family:row.family,scope:row.scope,action:row.action,target:row.target,reason:row.reason}));
 const plan=Object.freeze({version:1 as const,bootSession:observation.bootSession,steps:Object.freeze(steps),retained:Object.freeze(retained)});
 if(validateJsonSchemaSubset(schema.definitions.Plan,plan,schema).length)throw Error("Closed boot recovery plan required");return plan;
}

/** 🔖️ Names one plan by the digest of its canonical JSON. */
export function nativeBootPlanDigestV1(plan:NativeBootPlanV1):string{return createHash("sha256").update(JSON.stringify(plan)).digest("hex");}

/** 🧭️ Reads the instant the running boot began from the kernel's own text; null for text that is not exactly that. */
export function parseNativeBootEpochV1(platform:"darwin"|"linux"|"win32",text:string):number|null{
 if(Buffer.byteLength(text)>protocol.bootEpoch.maximumBytes)return null;
 if(platform==="darwin"){const row=new RegExp(protocol.bootEpoch.darwin.pattern,"u").exec(text);if(!row)return null;return Number(row[1])*1000+Math.floor(Number(row[2])/1000);}
 if(platform==="win32"){const row=new RegExp(protocol.bootEpoch.win32.pattern,"u").exec(text);if(!row)return null;const base=Date.parse(row[1]+"Z");return Number.isFinite(base)?base+Math.floor(Number((row[2]??"").padEnd(7,"0"))/10000):null;}
 for(const line of text.split("\n")){const row=new RegExp(protocol.bootEpoch.linux.pattern,"u").exec(line);if(row)return Number(row[1])*1000;}
 return null;
}

type Identity=Readonly<{dev:string;ino:string;mode:number}>;
/** 🪢️ Compares recorded identities with current ones by the invariant part only: inode and mode exactly, device numbers only through which identities share a device. */
export function bindNativeIdentitiesV1(recorded:readonly Identity[],current:readonly Identity[]):"exact"|"rebound"|"mismatch"{
 if(recorded.length!==current.length||!recorded.length)return"mismatch";
 for(let i=0;i<recorded.length;i++)if(recorded[i]!.ino!==current[i]!.ino||recorded[i]!.mode!==current[i]!.mode)return"mismatch";
 for(let i=0;i<recorded.length;i++)for(let j=i+1;j<recorded.length;j++)if((recorded[i]!.dev===recorded[j]!.dev)!==(current[i]!.dev===current[j]!.dev))return"mismatch";
 return recorded.every((row,i)=>row.dev===current[i]!.dev)?"exact":"rebound";
}

/** 🧬️ Projects a persisted backing identity onto its restart-proof fields; null for any shape that is not a known formula. */
export function nativeBackingIdentityProjectionV1(identity:string):readonly[string,number,number]|null{
 let row:unknown;try{row=JSON.parse(identity);}catch{return null;}
 if(!Array.isArray(row))return null;
 const number=(value:unknown):value is number=>typeof value==="number"&&Number.isSafeInteger(value)&&value>=0;
 if(row.length===4&&row.every(number))return Object.freeze(["receiver",row[1] as number,row[2] as number] as [string,number,number]);
 if(row.length===5&&typeof row[0]==="string"&&number(row[1])&&number(row[2])&&number(row[3]))return Object.freeze([row[0] as string,row[2] as number,row[3] as number] as [string,number,number]);
 return null;
}

/** 🪪️ Accepts a persisted parent identity when it is identical, or when it differs only by device numbers, which are never persisted truth. */
export function bindNativeBackingIdentityV1(recorded:string,current:string):"exact"|"rebound"|"mismatch"{
 if(recorded===current)return"exact";
 const a=nativeBackingIdentityProjectionV1(recorded),b=nativeBackingIdentityProjectionV1(current);
 if(!a||!b||JSON.stringify(a)!==JSON.stringify(b))return"mismatch";
 return"rebound";
}
