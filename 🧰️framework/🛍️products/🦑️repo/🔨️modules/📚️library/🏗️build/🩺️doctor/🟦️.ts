import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policyDocument from "./🔣️policy.json" with {type:"json"};
import rulesDocument from "./📖️rules/🔣️.json" with {type:"json"};
import textsDocument from "./📖️texts/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {isRuntimeExecutableV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🩺️ The closed constants of the doctor contract: areas, severities and report bounds. */
export const DOCTOR_PROTOCOL_V1=schema.definitions.Protocol.const;
export type DoctorAreaV1=(typeof DOCTOR_PROTOCOL_V1.areas)[number];
export type DoctorSeverityV1="error"|"warn"|"info";
export type DoctorLanguageV1="en"|"de";
export type DoctorTextV1=Readonly<{en:string;de:string}>;
export type DoctorDataV1=Readonly<Record<string,string|number|boolean|null>>;
export type DoctorFixKindV1="command"|"automatic"|"wait"|"ask";
export type DoctorFixV1=Readonly<{kind:DoctorFixKindV1;command?:string;repair?:string;note:DoctorTextV1}>;

/** 📜️ One catalogued rule: who owns it, which severities it may take, the data its texts use and how to resolve it. */
export type DoctorRuleV1=Readonly<{area:DoctorAreaV1;owner:string;severities:readonly DoctorSeverityV1[];data:readonly string[];title:DoctorTextV1;detail:DoctorTextV1;fix:DoctorFixV1}>;
export type DoctorRepairDeclarationV1=Readonly<{id:string;area:DoctorAreaV1;rules:readonly string[];summary:DoctorTextV1;safety:DoctorTextV1}>;

/** 🎚️ The authored thresholds, process patterns and declared safe repairs. */
export type DoctorPolicyV1=Readonly<{
 version:1;probe:Readonly<{timeoutMilliseconds:number;timeoutOverrides:Readonly<Record<string,number>>;parallelism:number}>;
 leases:Readonly<{waitWarnSeconds:number;waitErrorSeconds:number;holdWarnSeconds:number;holdErrorSeconds:number;queueDepthWarn:number;registryOrphanFoldersWarn:number;violationsLimit:number}>;
 capacity:Readonly<{waitWarnSeconds:number;waitErrorSeconds:number;idleGrantSeconds:number;idleObservedPercent:number}>;
 host:Readonly<{lowMemoryHeadroomPercent:number;swapWarnMegabytes:number;swapOutWarnMegabytesPerSecond:number;loadPerCoreWarn:number;loadPerCoreError:number;openFilesWarnPercent:number;imagesWarn:number;imagesError:number}>;
 storage:Readonly<{freePercentWarn:number;freePercentError:number;floorBytes:Readonly<Record<string,number>>;hostFreeWarnBytes:number;hostFreeErrorBytes:number;publicationErrorSeconds:number}>;
 tools:Readonly<{essential:readonly string[]}>;sessions:Readonly<{crowdedLive:number}>;
 graph:Readonly<{implementationRoots:readonly string[];implementationSkip:readonly string[];walkEntries:number}>;
 dependencies:Readonly<{orphanCountWarn:number;unreadyAgeSeconds:number}>;
 caches:Readonly<{deadPublications:readonly Readonly<{partition:string;path:readonly string[];prefix:string}>[];deadPublicationWarnBytes:number;deadPublicationQuietSeconds:number;walkEntries:number;retiredNamesShown:number}>;
 processes:Readonly<{patterns:readonly string[];exclude:readonly string[];orphanAgeSeconds:number;idleHolderSeconds:number;idleCpuPercent:number}>;
 repairs:readonly DoctorRepairDeclarationV1[];
}>;

export type DoctorProbeStatusV1="ok"|"unavailable"|"failed"|"timeout"|"skipped";
export type DoctorProbeRowV1=Readonly<{area:DoctorAreaV1;status:DoctorProbeStatusV1;durationMs:number;reason?:string}>;
export type HostFactsV1=Readonly<{cores:number;totalMegabytes:number;availableMegabytes:number;floorMegabytes:number;pressure:"normal"|"warn"|"critical";swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;load:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null;governor:"on"|"off"}>;
export type PartitionFactsV1=Readonly<{id:string;state:"mounted"|"detached"|"directory"|"absent";capacityBytes:number|null;freeBytes:number|null;imageBytes:number|null;backing:"present"|"missing"|"none";publication:"none"|"intent"|"receipt";publicationAgeSeconds:number|null;image:"ok"|"damaged"|"none"}>;
export type StorageFactsV1=Readonly<{hostFreeBytes:number|null;hostTotalBytes:number|null;partitions:readonly PartitionFactsV1[]}>;
export type ToolRowFactsV1=Readonly<{id:string;state:"ok"|"missing"|"drift"|"unprobed";pin:string|null;found:string|null}>;
/** 🔒️ One lockfile a wasm build resolves against: the `wasm-bindgen` crate version it locks (null when unreadable) and the version of the pinned binding generator. */
export type WasmLockFactsV1=Readonly<{path:string;version:string|null;pinned:string}>;
export type ToolsFactsV1=Readonly<{rows:readonly ToolRowFactsV1[];wasmLocks?:readonly WasmLockFactsV1[]}>;
export type HolderFactsV1=Readonly<{resource:string;hash:string;mode:"shared"|"exclusive";pid:number;heldSeconds:number;class:string;owner:string;command:string}>;
export type WaiterFactsV1=Readonly<{resource:string;hash:string;mode:"shared"|"exclusive";pid:number;waitedSeconds:number;class:string;owner:string;position:number|null;queued:number|null;blockers:readonly number[];command:string}>;
export type LeasesFactsV1=Readonly<{
 holders:readonly HolderFactsV1[];waiters:readonly WaiterFactsV1[];cycles:readonly Readonly<{members:readonly string[];victim:string}>[];
 violations:readonly Readonly<{kind:"out-of-order"|"hold-and-wait"|"self-deadlock"|"cycle";held:string;requested:string}>[];
 registry:Readonly<{folders:number;orphanFolders:number;deadEntries:number;liveEntries:number}>;deadTickets:number;deadHolders:readonly number[];
}>;
export type GrantFactsV1=Readonly<{operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes:number|null;heldSeconds:number}>;
export type CapacityWaiterFactsV1=Readonly<{operation:string;pid:number;needMegabytes:number;waitedSeconds:number;reason:string}>;
export type CapacityFactsV1=Readonly<{headroomMegabytes:number;grants:readonly GrantFactsV1[];waiters:readonly CapacityWaiterFactsV1[]}>;
export type SessionsFactsV1=Readonly<{live:number;idle:number;gone:number;damaged:number}>;
export type GraphFactsV1=Readonly<{pointer:"present"|"absent"|"damaged";generation:number|null;manifest:"present"|"missing"|"damaged"|"none";owners:number|null;publishedAgeSeconds:number|null;implementationNewerSeconds:number|null}>;
export type DependencyScopeFactsV1=Readonly<{scope:string;generation:string;directory:"ready"|"unready"|"missing";inputsNewer:string|null}>;
export type DependenciesFactsV1=Readonly<{pointers:"present"|"absent";scopes:readonly DependencyScopeFactsV1[];orphans:number;orphanEstimatedBytes:number;unready:number;unreadyOldestSeconds:number|null}>;
export type CachesFactsV1=Readonly<{deadPublications:number;deadPublicationBytes:number;deadPublicationPartial:boolean;retired:readonly string[];undeclared:readonly string[]}>;
export type ProcessFactsV1=Readonly<{pid:number;parent:number;ageSeconds:number;rssMegabytes:number;cpuPercent:number;command:string}>;

/** 📸️ Everything a probe run read from one machine; a section is absent when its probe had nothing to read. */
export type DoctorFactsV1=Readonly<{
 version:1;atMs:number;platform:"darwin"|"linux"|"win32";probes:readonly DoctorProbeRowV1[];
 host?:HostFactsV1;storage?:StorageFactsV1;tools?:ToolsFactsV1;leases?:LeasesFactsV1;capacity?:CapacityFactsV1;sessions?:SessionsFactsV1;graph?:GraphFactsV1;dependencies?:DependenciesFactsV1;caches?:CachesFactsV1;processes?:readonly ProcessFactsV1[];
}>;

export type DoctorFindingV1=Readonly<{id:string;area:DoctorAreaV1;owner:string;severity:DoctorSeverityV1;subject:string;data:DoctorDataV1;title:DoctorTextV1;detail:DoctorTextV1;fix:DoctorFixV1}>;
export type DoctorPartyV1=Readonly<{pid:number;command:string;role:"holder"|"ahead"|"grant";heldSeconds:number|null;reservedMegabytes:number|null}>;
export type DoctorWaitV1=Readonly<{
 kind:"lease"|"capacity";pid:number;command:string;waitedSeconds:number;subject:string;reason:string|null;owner:string;position:number|null;behind:readonly DoctorPartyV1[];
 root:Readonly<{pid:number;command:string;heldSeconds:number|null;state:"running"|"waiting"|"gone"|"unknown"}>|null;
}>;
export type DoctorPhaseV1="waiting-lease"|"waiting-capacity"|"holding"|"admitted"|"running"|"orphan";
export type DoctorProcessRowV1=Readonly<{pid:number;parent:number;ageSeconds:number;rssMegabytes:number;cpuPercent:number;phase:DoctorPhaseV1;via:number|null;command:string}>;
export type DoctorRepairResultV1=Readonly<{id:string;area:DoctorAreaV1;dryRun:boolean;status:"done"|"nothing"|"unavailable"|"failed"|"cancelled";removed:number;bytes:number;reason?:string}>;
export type DoctorVerdictV1="ok"|"degraded"|"blocked";
export type DoctorReportV1=Readonly<{
 version:1;atMs:number;platform:"darwin"|"linux"|"win32";verdict:DoctorVerdictV1;counts:Readonly<{error:number;warn:number;info:number}>;
 probes:readonly DoctorProbeRowV1[];findings:readonly DoctorFindingV1[];waits:readonly DoctorWaitV1[];processes:readonly DoctorProcessRowV1[];repairs:readonly DoctorRepairResultV1[];
}>;

const placeholders=(text:string):Set<string>=>new Set([...text.matchAll(/\{([A-Za-z][A-Za-z0-9]*)\}/gu)].map(match=>match[1]!));
const sameSet=(left:ReadonlySet<string>,right:ReadonlySet<string>):boolean=>left.size===right.size&&[...left].every(name=>right.has(name));

/** 🔎️ Lists the schema errors of a value against one definition of the doctor contract; an empty list means valid. */
export function validateDoctorV1(definition:keyof typeof schema.definitions,value:unknown):string[]{
 return validateJsonSchemaSubset(schema.definitions[definition],value,schema);
}

/** 🧾️ Reads the policy: closed shape, ordered thresholds and repairs that name catalogued rules. */
export function readDoctorPolicyV1(value:unknown=policyDocument,rules:Readonly<Record<string,DoctorRuleV1>>=rulesDocument as never):DoctorPolicyV1{
 const errors=validateDoctorV1("Policy",value);if(errors.length)throw Error("Closed doctor policy required: "+errors.slice(0,3).join("; "));
 const policy=value as DoctorPolicyV1,order:[string,number,number][]=[["leases.wait",policy.leases.waitWarnSeconds,policy.leases.waitErrorSeconds],["leases.hold",policy.leases.holdWarnSeconds,policy.leases.holdErrorSeconds],["capacity.wait",policy.capacity.waitWarnSeconds,policy.capacity.waitErrorSeconds],["host.load",policy.host.loadPerCoreWarn,policy.host.loadPerCoreError],["host.images",policy.host.imagesWarn,policy.host.imagesError],["storage.free",policy.storage.freePercentError,policy.storage.freePercentWarn],["storage.host",policy.storage.hostFreeErrorBytes,policy.storage.hostFreeWarnBytes]];
 for(const [name,low,high] of order)if(low>=high)throw Error("Doctor policy thresholds must rise from warn to error: "+name);
 const seen=new Set<string>();
 for(const repair of policy.repairs){
  if(seen.has(repair.id))throw Error("Doctor repair declared twice: "+repair.id);seen.add(repair.id);
  for(const rule of repair.rules){const row=rules[rule];if(!row||row.fix.kind!=="automatic"||row.fix.repair!==repair.id||row.area!==repair.area)throw Error("Doctor repair names a rule that does not point back at it: "+repair.id+" "+rule);}
 }
 return policy;
}

/** 📚️ Reads the rule catalogue: closed shape, texts that use only declared data in both languages and automatic fixes that are declared repairs. */
export function readDoctorRulesV1(value:unknown=rulesDocument,policy:DoctorPolicyV1=policyDocument as never):Readonly<Record<string,DoctorRuleV1>>{
 const errors=validateDoctorV1("Rules",value);if(errors.length)throw Error("Closed doctor rule catalogue required: "+errors.slice(0,3).join("; "));
 const rules=value as Record<string,DoctorRuleV1>,repairs=new Map(policy.repairs.map(row=>[row.id,row]));
 for(const [id,rule] of Object.entries(rules)){
  const data=new Set(rule.data),texts:[string,string][]=[["title.en",rule.title.en],["title.de",rule.title.de],["detail.en",rule.detail.en],["detail.de",rule.detail.de]];
  if(!id.startsWith(rule.area+"."))throw Error("Doctor rule id must start with its area: "+id);
  if(!sameSet(placeholders(rule.title.en),placeholders(rule.title.de))||!sameSet(placeholders(rule.detail.en),placeholders(rule.detail.de)))throw Error("Doctor rule texts differ in placeholders between languages: "+id);
  for(const [label,text] of texts)for(const name of placeholders(text))if(!data.has(name))throw Error("Doctor rule text uses undeclared data "+name+": "+id+" "+label);
  for(const name of placeholders(rule.fix.command??""))if(!data.has(name))throw Error("Doctor rule command uses undeclared data "+name+": "+id);
  if(rule.fix.kind==="automatic"){const repair=rule.fix.repair?repairs.get(rule.fix.repair):undefined;if(!repair||!repair.rules.includes(id))throw Error("Doctor automatic fix must be a declared repair of the rule: "+id);}
  else if(rule.fix.repair!==undefined)throw Error("Only automatic doctor fixes name a repair: "+id);
  if(rule.fix.kind==="command"&&rule.fix.command===undefined)throw Error("Doctor command fixes need a command: "+id);
 }
 return Object.freeze(rules);
}

/** 🎛️ The authored policy of this checkout. */
export const DOCTOR_POLICY_V1:DoctorPolicyV1=readDoctorPolicyV1();
/** 📖️ The authored rule catalogue of this checkout. */
export const DOCTOR_RULES_V1:Readonly<Record<string,DoctorRuleV1>>=readDoctorRulesV1(rulesDocument,DOCTOR_POLICY_V1);
const TEXTS=textsDocument as unknown as Readonly<Record<string,DoctorTextV1>>;

/** 🧩️ Replaces `{name}` placeholders; a missing value stays visible as the placeholder, an absent value reads `n/a`. */
export function fillDoctorV1(template:string,values:Readonly<Record<string,string|number|boolean|null|undefined>>):string{
 return template.replace(/\{([A-Za-z][A-Za-z0-9]*)\}/gu,(match,name:string)=>name in values?(values[name]===null||values[name]===undefined?"n/a":String(values[name])):match);
}

/** 💬️ Renders one interface text in one language. */
export function doctorTextV1(language:DoctorLanguageV1,key:string,values:Readonly<Record<string,string|number|boolean|null|undefined>>={}):string{
 const row=TEXTS[key];if(!row)throw Error("Declared doctor text required: "+key);
 return fillDoctorV1(row[language],values);
}

/** 🗣️ Chooses the output languages: the request, then the environment, then the locale; when none resolves both languages are printed, because no language is assumed. */
export function resolveDoctorLanguagesV1(requested:string|undefined,environment:Readonly<Record<string,string|undefined>>):readonly DoctorLanguageV1[]{
 const tag=(value:string|undefined):DoctorLanguageV1|undefined=>{const head=value?.toLowerCase().split(/[._@:-]/u)[0];return head==="en"||head==="de"?head:undefined;};
 if(requested!==undefined){const chosen=tag(requested);if(!chosen||requested.length>2)throw Error(fillDoctorV1(TEXTS.errorValue!.en,{option:"--lang"})+"\n"+fillDoctorV1(TEXTS.errorValue!.de,{option:"--lang"}));return [chosen];}
 for(const value of [environment.SEMIO_LANGUAGE,environment.LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const chosen=tag(value);if(chosen)return [chosen];}
 return ["en","de"];
}

/** ✂️ Shortens a process command line to what a person needs: launcher, runtime and script path collapse to `repo`, the rest is clipped. */
export function commandLabelV1(raw:string,limit:number=DOCTOR_PROTOCOL_V1.maximumCommandCharacters):string{
 const words=raw.trim().split(/\s+/u).filter(Boolean),base=(word:string):string=>word.slice(word.lastIndexOf("/")+1).replace(/\.exe$/iu,""),script=(word:string|undefined):boolean=>word!==undefined&&/script\.(ts|mjs|js)$/u.test(word);
 const launch=(rest:readonly string[]):string=>["repo",...rest[0]==="repo"?rest.slice(1):rest].join(" ");
 let text=words.join(" ");const head=words.length?base(words[0]!):"";
 if(isRuntimeExecutableV1(head)){
  let index=1;
  while(index<words.length&&words[index]!.startsWith("-")&&!script(words[index]))index+=words[index]==="--cwd"?2:1;
  const next=words[index];
  text=script(next)||next==="repo"?launch(words.slice(index+1)):[head,...words.slice(index)].join(" ");
 }else if(script(words[0]))text=launch(words.slice(1));
 return text.length>limit?text.slice(0,limit-1)+"…":text;
}

const WRAPPERS=new Set(["sh","bash","zsh","dash","fish","pwsh","powershell","cmd","env","nohup","sudo","time","caffeinate","taskpolicy","nice","xargs","tee","script"]);
/** 🎯️ Decides whether a process belongs to the repo tool chain: its program and first arguments match a declared pattern and no exclusion; shells and wrappers never do, whatever text they carry. */
export function commandRelevantV1(command:string,patterns:readonly RegExp[],excluded:readonly RegExp[]):boolean{
 const words=command.trim().split(/\s+/u).filter(Boolean),program=words.length?words[0]!.slice(words[0]!.lastIndexOf("/")+1).replace(/\.exe$/iu,""):"";
 if(WRAPPERS.has(program))return false;
 const head=words.slice(0,8).join(" ");
 return patterns.some(pattern=>pattern.test(head))&&!excluded.some(pattern=>pattern.test(head));
}

const SECRET_OPTION=/(--?(?:[a-z-]*(?:token|secret|password|passwd|api-?key|authorization|credential)[a-z-]*))(?:=|\s+)(\S+)/giu;
const SECRET_ASSIGNMENT=/\b([A-Za-z0-9_]*(?:TOKEN|SECRET|PASSWORD|PASSWD|API_?KEY|CREDENTIAL)[A-Za-z0-9_]*)=(\S+)/gu;
const SECRET_BEARER=/\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]{8,}/gu;
/** 🙈️ Hides the values of secret-looking options and assignments in a command line before it enters a report that people paste into tickets. */
export function redactCommandV1(command:string):string{
 return command.replace(SECRET_OPTION,"$1=***").replace(SECRET_ASSIGNMENT,"$1=***").replace(SECRET_BEARER,"$1 ***");
}

/** 📏️ Keeps the tail of a long resource name, which is the part that tells resources apart. */
export function clipResourceV1(text:string,limit:number=80):string{return text.length>limit?"…"+text.slice(text.length-limit+1):text;}

/** 🛂️ Rejects a facts document that breaks the contract. */
export function requireDoctorFactsV1(value:unknown):DoctorFactsV1{
 const errors=validateDoctorV1("Facts",value);if(errors.length)throw Error("Closed doctor facts required: "+errors.slice(0,3).join("; "));
 return value as DoctorFactsV1;
}
