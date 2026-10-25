import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import table from"./🔣️.json" with { type: "json" };
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🗓️ One scheduling domain: directory prefixes of code that only schedules, admits, stores or reports an action, its contract in English and German, and the ports through which keyed code may import it. See `./🔣️.json` (the policy) and `./🧪️tests/🟦️.ts` (the law). */
export interface BuildSchedulingDomainV1{readonly id:string;readonly contract:{readonly en:string;readonly de:string};readonly prefixes:readonly string[];readonly ports:readonly string[]}

/** 📚️ The closed policy of all scheduling domains. */
export interface BuildSchedulingV1{readonly version:1;readonly domains:readonly BuildSchedulingDomainV1[]}

/** 🏷️ The file name suffix that turns a snapshot name into the name of its unkeyed scheduling overlay. */
export const buildSchedulingOverlaySuffixV1=".scheduling.json";

let declared:BuildSchedulingV1|undefined;

/** 🔐️ Admits a policy: closed by its schema, unique domain ids, prefixes that are clean directories no other prefix contains, ports inside their own domain; any other table is refused so nothing is left unkeyed by accident. */
export function buildSchedulingV1(candidate:unknown=table):BuildSchedulingV1{
 if(candidate===table&&declared)return declared;
 if(validateJsonSchemaSubset(schema,candidate).length)throw refuseBuildV1("invalid","scheduling.table-required");
 const value=candidate as BuildSchedulingV1,ids=new Set<string>(),prefixes:string[]=[];
 for(const domain of value.domains){
  if(ids.has(domain.id))throw refuseBuildV1("invalid","scheduling.table-required");ids.add(domain.id);
  for(const prefix of domain.prefixes){if(prefix.split("/").slice(0,-1).some(part=>part===""||part==="."||part===".."))throw refuseBuildV1("invalid","scheduling.table-required");prefixes.push(prefix);}
  for(const port of domain.ports)if(!domain.prefixes.some(prefix=>port.startsWith(prefix))||port.endsWith("/"))throw refuseBuildV1("invalid","scheduling.table-required");
 }
 for(const [index,prefix] of prefixes.entries())if(prefixes.some((other,at)=>at!==index&&other.startsWith(prefix)))throw refuseBuildV1("invalid","scheduling.table-required");
 const frozen=Object.freeze(value);if(candidate===table)declared=frozen;return frozen;
}

/** 🧲️ Resolves the named domains of a policy; a name no domain carries is refused. */
function selected(value:BuildSchedulingV1,ids?:readonly string[]):readonly BuildSchedulingDomainV1[]{
 if(ids===undefined)return value.domains;
 return ids.map(id=>{const row=value.domains.find(domain=>domain.id===id);if(!row)throw refuseBuildV1("missing","scheduling.domain-required",{id:String(id)});return row;});
}

/** 📏️ The sorted directory prefixes of the named domains, or of all domains when none are named. */
export function buildSchedulingPrefixesV1(value:BuildSchedulingV1=buildSchedulingV1(),ids?:readonly string[]):readonly string[]{
 return [...new Set(selected(value,ids).flatMap(domain=>domain.prefixes))].sort();
}

/** 🔎️ The id of the domain a workspace-relative path belongs to, or nothing when the path stays keyed. */
export function buildSchedulingDomainOfV1(path:string,value:BuildSchedulingV1=buildSchedulingV1()):string|undefined{
 return value.domains.find(domain=>domain.prefixes.some(prefix=>path.startsWith(prefix)))?.id;
}

/** 🚪️ The ports of the named domains (all when none are named): the only scheduling modules keyed code may import. */
export function buildSchedulingPortsV1(value:BuildSchedulingV1=buildSchedulingV1(),ids?:readonly string[]):ReadonlySet<string>{
 return new Set(selected(value,ids).flatMap(domain=>domain.ports));
}

/** 🙈️ The named build input that carries the negations of every scheduling prefix, so a target's key can leave scheduling code out; `./🧪️tests/🟦️.ts` keeps it equal to the policy in the build policy and its Nx mirror. */
export const buildSchedulingExclusionsInputV1="schedulingSourceExclusions";

/** 🧮️ The negated file patterns of all scheduling prefixes, sorted. */
export function buildSchedulingExclusionPatternsV1(value:BuildSchedulingV1=buildSchedulingV1()):readonly string[]{
 return buildSchedulingPrefixesV1(value).map(prefix=>"!{workspaceRoot}/"+prefix+"**/*");
}

/** 🪞️ Names the overlay file that belongs to a snapshot file. */
export function buildSchedulingOverlayNameV1(snapshot:string):string{
 if(!snapshot.endsWith(".json"))throw refuseBuildV1("invalid","scheduling.snapshot-name-required",{name:String(snapshot)});
 return snapshot.slice(0,-".json".length)+buildSchedulingOverlaySuffixV1;
}

/** ✂️ Splits a captured snapshot into the keyed part, which holds every output-producing file and carries the whole identity, and the overlay of the scheduling files the named domains cover; the entry module can never be scheduling code, and a snapshot with nothing to split has no overlay. */
export function splitBuildSourceClosureV1<File extends {readonly path:string},Snapshot extends {readonly entry:string;readonly files:readonly File[]}>(snapshot:Snapshot,ids:readonly string[],value:BuildSchedulingV1=buildSchedulingV1()):{readonly keyed:Snapshot;readonly overlay:Snapshot|undefined}{
 const prefixes=buildSchedulingPrefixesV1(value,ids),keyed:File[]=[],overlay:File[]=[];
 for(const file of snapshot.files)(prefixes.some(prefix=>file.path.startsWith(prefix))?overlay:keyed).push(file);
 if(!keyed.some(file=>file.path===snapshot.entry))throw refuseBuildV1("invalid","scheduling.entry-required",{entry:String(snapshot.entry)});
 return {keyed:Object.freeze({...snapshot,files:Object.freeze(keyed)}),overlay:overlay.length?Object.freeze({...snapshot,files:Object.freeze(overlay)}):undefined};
}
