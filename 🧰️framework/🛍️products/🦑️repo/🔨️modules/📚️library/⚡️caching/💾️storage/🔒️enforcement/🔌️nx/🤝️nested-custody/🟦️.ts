/** 🤝️ The decision for a cache operation inside an open fixed-cache custody (schema `./🧬️schema/🔣️.json`, tests `./🧪️tests/🟦️.ts`, Nx patch `⚡️caching/🩹️patches/nx@23.2.0.patch`: `cache.get` of a remote hit restores through an exclusive custody inside the shared custody of `resolveCachedTasks`). */
export type NxNestedCustodyModeV1="shared"|"exclusive";
/** 📏️ The rules a nested custody can break. */
export type NxNestedCustodyRuleV1="workspace"|"tooling"|"environment"|"graph"|"implementation"|"reader-holds-outputs";
/** 🔒️ What the open custody holds that the nested one is compared with. */
export interface NxNestedCustodyHeldV1{readonly workspace:string;readonly toolingIdentity:string;readonly environment:object;readonly graphIdentity:string|null;readonly implementationIdentity:string;readonly mode:NxNestedCustodyModeV1;readonly outputs:ReadonlyMap<string,unknown>;readonly cohorts:ReadonlyMap<string,unknown>;readonly products?:unknown}
/** 📥️ What the nested cache operation asks for. */
export interface NxNestedCustodyRequestV1{readonly workspace:string;readonly toolingIdentity:string;readonly environment:Readonly<Record<string,string|undefined>>;readonly mode:NxNestedCustodyModeV1;readonly paths:readonly string[]}
const texts:Readonly<Record<NxNestedCustodyRuleV1,readonly [string,string]>>=Object.freeze({
 workspace:["a nested custody must name the workspace of the held custody","ein verschachtelter Gewahrsam muss den Arbeitsbereich des gehaltenen Gewahrsams nennen"],
 tooling:["a nested custody must carry the tooling identity of the held custody","ein verschachtelter Gewahrsam muss die Werkzeugidentitaet des gehaltenen Gewahrsams tragen"],
 environment:["a nested custody must receive the very process environment that opened the held custody","ein verschachtelter Gewahrsam muss genau die Prozessumgebung erhalten, die den gehaltenen Gewahrsam geoeffnet hat"],
 graph:["a nested custody must carry the graph digest of the held custody","ein verschachtelter Gewahrsam muss den Graph-Digest des gehaltenen Gewahrsams tragen"],
 implementation:["a nested custody must carry the implementation identity of the held custody","ein verschachtelter Gewahrsam muss die Implementierungsidentitaet des gehaltenen Gewahrsams tragen"],
 "reader-holds-outputs":["a reader that already holds outputs cannot become a writer (only a reader that holds no output may)","ein Leser, der bereits Ausgaben haelt, kann kein Schreiber werden (nur ein Leser ohne Ausgaben darf es)"]
});
const short=(value:string|null|undefined):string|null=>value===undefined||value===null?null:value.length>64&&/^[a-f0-9]+$/u.test(value)?value.slice(0,12):value;
/** 🚫️ The typed bilingual refusal of a nested custody: the task (when the environment names it), the output paths, the rule and both sides of the broken comparison. */
export class NxNestedCustodyRefusedErrorV1 extends Error{
 readonly code="nx-nested-cache-custody-refused" as const;readonly rule:NxNestedCustodyRuleV1;readonly task:string|null;readonly paths:readonly string[];readonly heldMode:NxNestedCustodyModeV1;readonly requestedMode:NxNestedCustodyModeV1;readonly held:string|null;readonly requested:string|null;readonly en:string;readonly de:string;
 constructor(rule:NxNestedCustodyRuleV1,task:string|null,paths:readonly string[],heldMode:NxNestedCustodyModeV1,requestedMode:NxNestedCustodyModeV1,held:string|null,requested:string|null){
  const where=paths.length?paths.join(", "):"-",forEn=task??"unnamed task",forDe=task??"unbenannte Aufgabe",sides=held===null&&requested===null?"":" (held "+held+", requested "+requested+")",seiten=held===null&&requested===null?"":" (gehalten "+held+", angefragt "+requested+")",
  en="Original nested cache custody refused for "+forEn+" on "+where+": "+texts[rule][0]+sides+"; held "+heldMode+", requested "+requestedMode,
  de="Verschachtelter Cache-Gewahrsam verweigert fuer "+forDe+" auf "+where+": "+texts[rule][1]+seiten+"; gehalten "+heldMode+", angefragt "+requestedMode;
  super(en+" | "+de);this.name="NxNestedCustodyRefusedErrorV1";this.rule=rule;this.task=task;this.paths=Object.freeze([...paths]);this.heldMode=heldMode;this.requestedMode=requestedMode;this.held=held;this.requested=requested;this.en=en;this.de=de;
 }
 toJSON(){return{version:1 as const,code:this.code,rule:this.rule,task:this.task,paths:[...this.paths],heldMode:this.heldMode,requestedMode:this.requestedMode,held:this.held,requested:this.requested,en:this.en,de:this.de};}
}
/** 🏷️ The task the process environment names, when it does (Nx sets these for a task's own process). */
export function nxNestedCustodyTaskV1(environment:Readonly<Record<string,string|undefined>>):string|null{const project=environment.NX_TASK_TARGET_PROJECT,target=environment.NX_TASK_TARGET_TARGET;if(!project||!target)return null;const configuration=environment.NX_TASK_TARGET_CONFIGURATION;return project+":"+target+(configuration?":"+configuration:"");}
/** ✅️ Admits a nested custody: the held scope itself, or (a reader that holds no output asked for a writer) a copy of it in exclusive mode, which the caller runs the nested body in; every other mismatch is the typed refusal. */
export function admitNestedNxCustodyV1<H extends NxNestedCustodyHeldV1>(held:H,request:NxNestedCustodyRequestV1):H{
 const refuse=(rule:NxNestedCustodyRuleV1,heldValue:string|null,requestedValue:string|null):never=>{throw new NxNestedCustodyRefusedErrorV1(rule,nxNestedCustodyTaskV1(request.environment),request.paths,held.mode,request.mode,short(heldValue),short(requestedValue));};
 if(held.workspace!==request.workspace)refuse("workspace",held.workspace,request.workspace);
 if(held.toolingIdentity!==request.toolingIdentity)refuse("tooling",held.toolingIdentity,request.toolingIdentity);
 if(held.environment!==request.environment)refuse("environment",null,null);
 if(held.graphIdentity!==(request.environment.SEMIO_BUILD_GRAPH_DIGEST??null))refuse("graph",held.graphIdentity,request.environment.SEMIO_BUILD_GRAPH_DIGEST??null);
 if(held.implementationIdentity!==request.environment.SEMIO_NX_IMPLEMENTATION_IDENTITY)refuse("implementation",held.implementationIdentity,request.environment.SEMIO_NX_IMPLEMENTATION_IDENTITY??null);
 if(held.mode==="shared"&&request.mode==="exclusive"){if(held.outputs.size||held.cohorts.size||held.products)refuse("reader-holds-outputs",[...held.outputs.keys(),...held.cohorts.keys()].join(", ")||null,request.paths.join(", ")||null);return Object.freeze({...held,mode:"exclusive" as const});}
 return held;
}
