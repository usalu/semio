import {isAbsolute,join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import native from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import budgetPolicy from "../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 🌐️ The two languages every cache text is authored in; English comes first and nothing else is assumed. */
export type CacheLanguageV1="en"|"de";
/** 🏠️ The two declared loose roots: the repository cache root and the native cache root. */
export type CacheRootIdV1="repository"|"native";
/** 🧑‍🔧️ Names who keeps an area within its budget when the generic collector must not. */
export type CacheOwnerV1="image"|"control"|"cargo"|"nx"|"bazel"|"test-evidence"|"dashboard"|"graph"|"remote-cache"|"remote-execution";
type Bilingual=Readonly<Record<CacheLanguageV1,string>>;
/** 💰️ One declared budget: explicit bytes, a share of the partition, or whatever its siblings leave. */
export type CacheBudgetV1=Readonly<{bytes:number}>|Readonly<{basisPoints:number}>|Readonly<{remainder:true}>;
/** 🧹️ How an area is collected: how long it may stay unused, how long a unit must be quiet, and what proves a unit is in use. */
export type CacheCollectV1=Readonly<{unusedAgeMs:number|null;quietMs:number;custody:"quiet"|"sqlite"|"lease";lease?:string}>;
/** 🚰️ The free space below which an owner is asked to reclaim in the background, and the free space it reclaims up to. */
export type CacheReclaimV1=Readonly<{softAvailableBytes:number;targetAvailableBytes:number}>;
/** 📏️ One part of a partition's measured need: its cap (an area's budget or explicit bytes) and what was measured. */
export type CacheNeedComponentV1=Readonly<{id:string;area?:string;bytes?:number;measuredBytes:number}>;
/** 🧪️ A partition's need derived from measurement: the caps of its parts plus headroom plus the floor that must stay free. */
export type CacheNeedV1=Readonly<{partition:string;measuredOn:string;headroomPercent:number;floorBytes:number;components:readonly CacheNeedComponentV1[]}>;
/** 📍️ One declared home of cached bytes. */
export type CacheAreaV1=Readonly<{id:string;root:CacheRootIdV1;kind:"partition"|"control"|"nested"|"tree";directory?:string;pattern?:string;partition?:string;budget?:CacheBudgetV1;managed?:Readonly<{owner:CacheOwnerV1}>;recreate?:"derived"|"carried";carry?:readonly string[];reclaim?:CacheReclaimV1;collect?:CacheCollectV1;writers:readonly string[];purpose:Bilingual}>;
/** 🪦️ One root that no writer feeds any more and the collector removes once it has been quiet. */
export type CacheRetirementV1=Readonly<{root:CacheRootIdV1;directory:string;quietMs:number;reason:Bilingual}>;
/** 🗃️ One loose root with the budget of all its areas and the rule for entries nobody declared. */
export type CacheRootV1=Readonly<{directory:string;maximumBytes:number;undeclaredUnusedAgeMs:number;undeclaredQuietMs:number;ignored:readonly string[]}>;
/** 🧾️ The whole closed ledger of cache homes. */
export type CacheLedgerV1=Readonly<{version:1;roots:Readonly<Record<CacheRootIdV1,CacheRootV1>>;areas:readonly CacheAreaV1[];retirements:readonly CacheRetirementV1[];needs?:readonly CacheNeedV1[]}>;
/** 🔢️ The partition facts the ledger needs from the authored budget; tests replace them. */
export type CachePartitionBudgetsV1=Readonly<Record<string,Readonly<{directory:string;capacityBytes:number;maximumLogicalBytes:number}>>>;

/** ⚙️ The authored bounds of one scan, one progress interval and one removal batch. */
export const CACHE_RUNTIME_V1=schema.$defs.runtime.const;
const mountDirectory=native.definitions.ImageProtocol.const.mountDirectory;
const refuse=(reason:string):never=>{throw Error("Closed cache ledger required: "+reason);};
const segments=(path:string):string[]=>path.split("/");
const portable=(path:string):string=>path.normalize("NFC").toLowerCase();
const inside=(parent:readonly string[],child:readonly string[]):boolean=>parent.length<=child.length&&parent.every((part,index)=>part===child[index]);
const bytesOf=(budget:CacheBudgetV1|undefined):number=>budget&&"bytes" in budget?budget.bytes:0;

/** 📋️ Validates the entire ledger, its budgets and its disjointness before any cache read or write relies on it. */
export function readCacheLedgerV1(value:unknown,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):CacheLedgerV1{
 const errors=validateJsonSchemaSubset(schema,value);if(errors.length)refuse(errors[0]!);
 const ledger=value as CacheLedgerV1,ids=new Set<string>(),claimed=new Map<CacheRootIdV1,string[][]>([["repository",[]],["native",[]]]),nested=new Map<string,{paths:string[][];explicit:number;remainders:number}>();
 for(const area of ledger.areas){
  if(ids.has(area.id))refuse("duplicate area "+area.id);ids.add(area.id);
  if(area.recreate!==undefined&&area.kind!=="partition")refuse("only a partition may declare how it is recreated: "+area.id);
  if((area.recreate==="carried")!==(area.carry!==undefined))refuse("exactly a carried partition names what it carries: "+area.id);
  if(area.carry?.some(path=>segments(path).some(part=>part==="."||part===".."||part==="")))refuse("canonical carried directories required for "+area.id);
  if(area.directory!==undefined&&segments(area.directory).some(part=>part==="."||part===".."))refuse("canonical directory required for "+area.id);
  if(area.pattern!==undefined)try{new RegExp(area.pattern,"u");}catch{refuse("valid pattern required for "+area.id);}
  const remainder=area.budget!==undefined&&"remainder" in area.budget;
  if(area.kind==="nested"){
   if(area.reclaim&&(area.reclaim.softAvailableBytes>=area.reclaim.targetAvailableBytes||area.reclaim.targetAvailableBytes>2684354560))refuse("reclaim needs soft below target and a target within the headroom schema: "+area.id);
   if(!area.partition||!Object.hasOwn(partitions,area.partition)||area.pattern!==undefined||area.collect!==undefined||!area.managed||!area.budget||remainder===(area.directory!==undefined))refuse("nested area "+area.id+" needs a declared partition, a manager and a budget");
   const row=nested.get(area.partition!)??{paths:[],explicit:0,remainders:0};nested.set(area.partition!,row);
   if(remainder)row.remainders++;else{const own=segments(area.directory!);if(row.paths.some(other=>inside(other,own)||inside(own,other)))refuse("overlapping nested areas in "+area.partition);row.paths.push(own);}
   const limit=partitions[area.partition!]!.maximumLogicalBytes,budget=area.budget!;row.explicit+="bytes" in budget?budget.bytes:"basisPoints" in budget?Math.floor(limit*budget.basisPoints/10000):0;
   continue;
  }
  if((area.directory!==undefined)===(area.pattern!==undefined))refuse("exactly one of directory and pattern required for "+area.id);
  if(area.pattern!==undefined&&area.kind!=="tree")refuse("only tree areas may match by pattern: "+area.id);
  if(area.budget&&!("bytes" in area.budget))refuse("only nested areas may share a partition budget: "+area.id);
  if(area.kind==="tree"&&(!area.budget||(area.collect===undefined)===(area.managed===undefined)))refuse("tree area "+area.id+" needs a budget and exactly one of collect and managed");
  if(area.kind==="partition"){
   const row=area.partition?partitions[area.partition]:undefined,root=ledger.roots[area.root];
   if(!row||area.budget||area.collect||area.managed?.owner!=="image"||join(root.directory,area.directory!).replaceAll("\\","/")!==row.directory)refuse("partition area "+area.id+" must name its authored budget partition and directory");
  }
  if(area.kind==="control"&&(area.collect===undefined)===(area.managed===undefined))refuse("control area "+area.id+" needs exactly one of collect and managed");
  if(area.collect&&(area.collect.custody==="lease")!==(area.collect.lease!==undefined))refuse("lease custody needs a resource: "+area.id);
  if(area.directory!==undefined)claimed.get(area.root)!.push(segments(area.directory));
 }
 for(const retirement of ledger.retirements){
  if(segments(retirement.directory).some(part=>part==="."||part===".."))refuse("canonical retirement required: "+retirement.directory);
  claimed.get(retirement.root)!.push(segments(retirement.directory));
 }
 for(const[root,paths]of claimed){
  const folded=paths.map(path=>path.map(portable));
  for(let first=0;first<folded.length;first++)for(let second=first+1;second<folded.length;second++)if(inside(folded[first]!,folded[second]!)||inside(folded[second]!,folded[first]!))refuse("overlapping homes in "+root+": "+paths[first]!.join("/")+" and "+paths[second]!.join("/"));
  for(const area of ledger.areas)if(area.root===root&&area.pattern!==undefined){const matcher=new RegExp(area.pattern,"u");if(paths.some(path=>matcher.test(path[0]!)))refuse("pattern of "+area.id+" matches a declared home");}
 }
 for(const area of ledger.areas)if(area.reclaim){const row=nested.get(area.partition!)!,room=partitions[area.partition!]!.capacityBytes-schema.$defs.runtime.const.imageOverheadBytes-row.explicit;if(room<area.reclaim.softAvailableBytes)refuse("area caps leave less free space than the soft reclaim threshold of "+area.id);}
 for(const area of ledger.areas)if(area.carry)for(const path of area.carry){
  const own=segments(path);
  if(!ledger.areas.some(other=>other.kind==="nested"&&other.partition===area.partition&&other.directory!==undefined&&inside(segments(other.directory),own)))refuse("carried directory "+path+" of "+area.id+" lies in no declared nested area");
 }
 for(const need of ledger.needs??[]){
  if(!Object.hasOwn(partitions,need.partition))refuse("need names an unknown partition "+need.partition);
  const required=cacheNeedRequiredBytesV1(ledger,need,partitions);if(partitions[need.partition]!.capacityBytes<required)refuse("partition "+need.partition+" is smaller than its measured need of "+required+" bytes");
 }
 for(const[partition,row]of nested){
  const limit=partitions[partition]!.maximumLogicalBytes;if(row.remainders>1||row.explicit>=limit||row.remainders===1&&limit-row.explicit<1048576)refuse("nested budgets exceed partition "+partition);
 }
 for(const root of["repository","native"] as const){
  const total=ledger.areas.filter(area=>area.root===root&&area.kind==="tree"||area.root===root&&area.kind==="control"&&area.collect).reduce((sum,area)=>sum+bytesOf(area.budget),0);
  if(total>ledger.roots[root].maximumBytes)refuse("area budgets exceed the budget of the "+root+" root");
 }
 return Object.freeze(ledger);
}

let memo:CacheLedgerV1|undefined;
/** 🧭️ The authored ledger, validated once. */
export function cacheLedgerV1():CacheLedgerV1{return memo??=readCacheLedgerV1(policy);}

/** 💵️ Resolves an area's budget in bytes; partitions are bounded by their image and controls by nothing. */
export function cacheAreaBudgetBytesV1(ledger:CacheLedgerV1,area:CacheAreaV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):number|null{
 if(area.kind==="partition")return Math.min(partitions[area.partition!]!.capacityBytes,partitions[area.partition!]!.maximumLogicalBytes);
 const budget=area.budget;if(!budget)return null;
 if("bytes" in budget)return budget.bytes;
 const limit=partitions[area.partition!]!.maximumLogicalBytes;
 if("basisPoints" in budget)return Math.floor(limit*budget.basisPoints/10000);
 return limit-ledger.areas.filter(other=>other.kind==="nested"&&other.partition===area.partition&&other.id!==area.id).reduce((sum,other)=>sum+(cacheAreaBudgetBytesV1(ledger,other,partitions)??0),0);
}

/** 📂️ Resolves the absolute directory of an area; nested areas live in the mounted root of their partition. */
export function cacheAreaDirectoryV1(workspace:string,ledger:CacheLedgerV1,area:CacheAreaV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):string|null{
 if(!isAbsolute(workspace)||resolve(workspace)!==workspace)throw Error("Canonical workspace required");
 if(area.kind==="nested")return area.directory===undefined?join(workspace,partitions[area.partition!]!.directory,mountDirectory):join(workspace,partitions[area.partition!]!.directory,mountDirectory,...segments(area.directory));
 return area.directory===undefined?null:join(workspace,ledger.roots[area.root].directory,...segments(area.directory));
}

/** 🗂️ Resolves a loose root to its absolute directory. */
export function cacheRootDirectoryV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1):string{return join(workspace,...segments(ledger.roots[root].directory));}

/** 🚧️ Refuses a repository cache path whose first segments name no declared home; the one runtime door every TypeScript writer passes. */
export function requireDeclaredCacheSegmentsV1(path:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1()):void{
 if(!path.length)return;
 const parts=path.flatMap(part=>part.split("/")),declared=ledger.areas.filter(area=>area.root==="repository"&&area.kind!=="nested");
 if(declared.some(area=>area.directory!==undefined&&(inside(segments(area.directory),parts)||inside(parts,segments(area.directory)))||area.pattern!==undefined&&new RegExp(area.pattern,"u").test(parts[0]!)))return;
 throw Error(cacheTextV1("en","error.undeclared",{segment:parts.join("/")})+"\n"+cacheTextV1("de","error.undeclared",{segment:parts.join("/")}));
}

/** 🔎️ Names what a path below a loose root is: a declared area, a container of declared homes, a retired root, an ignored file or nobody's. */
export type CacheClassV1=Readonly<{kind:"area"|"container"|"retired"|"ignored"|"undeclared";id?:string}>;
/** 🧮️ Classifies one root-relative path against the ledger. */
export function classifyCachePathV1(ledger:CacheLedgerV1,root:CacheRootIdV1,path:string):CacheClassV1{
 const parts=segments(path);if(parts.length===1&&ledger.roots[root].ignored.includes(parts[0]!))return{kind:"ignored"};
 for(const area of ledger.areas){
  if(area.root!==root)continue;
  if(area.pattern!==undefined&&new RegExp(area.pattern,"u").test(parts[0]!))return{kind:"area",id:area.id};
  if(area.directory===undefined||area.kind==="nested")continue;
  const own=segments(area.directory);if(inside(own,parts))return{kind:"area",id:area.id};
 }
 for(const retirement of ledger.retirements)if(retirement.root===root&&inside(segments(retirement.directory),parts))return{kind:"retired",id:retirement.directory};
 const homes=[...ledger.areas.filter(area=>area.root===root&&area.kind!=="nested"&&area.directory!==undefined).map(area=>segments(area.directory!)),...ledger.retirements.filter(row=>row.root===root).map(row=>segments(row.directory))];
 return homes.some(home=>inside(parts,home))?{kind:"container"}:{kind:"undeclared"};
}

/** 💬️ Renders one catalogue message with its placeholders. */
export function cacheTextV1(language:CacheLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const row=(texts.messages as Readonly<Record<string,Bilingual>>)[key];if(!row)throw Error("Declared cache message required: "+key);
 return row[language].replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error("Cache message variable missing: "+name);return String(value);});
}

/** 🔠️ Spells a byte count in binary units, deterministically. */
export function formatCacheBytesV1(bytes:number):string{
 if(bytes<1024)return bytes+" B";
 const units=["KiB","MiB","GiB","TiB"];let value=bytes,index=-1;do{value/=1024;index++;}while(value>=1024&&index<units.length-1);
 return value.toFixed(2)+" "+units[index];
}

/** 🧱️ One independently removable child of an area, with the newest file time inside it and the holder that proves it is in use. */
export type CacheUnitV1=Readonly<{name:string;bytes:number;recencyMs:number;held?:string}>;
/** 🎯️ One thing the collector may shrink: a declared area, a retired root or the undeclared entries of a root. */
export type CacheTargetV1=Readonly<{id:string;kind:"area"|"retired"|"undeclared";budgetBytes:number|null;unusedAgeMs:number|null;quietMs:number;heldBy?:string;units:readonly CacheUnitV1[]}>;
/** 🗑️ One planned removal and why. */
export type CacheRemovalV1=Readonly<{target:string;name:string;bytes:number;reason:"retired"|"undeclared"|"age"|"budget"}>;
/** 🛡️ One unit the plan wanted to remove but its holder or its recent use forbids. */
export type CacheKeptV1=Readonly<{target:string;name:string;bytes:number;reason:"held"|"quiet"}>;
/** 📊️ The plan for one target. */
export type CacheTargetPlanV1=Readonly<{id:string;kind:CacheTargetV1["kind"];totalBytes:number;retainedBytes:number;budgetBytes:number|null;overBudgetBytes:number;removals:readonly CacheRemovalV1[];kept:readonly CacheKeptV1[]}>;
const byName=(left:string,right:string):number=>left<right?-1:left>right?1:0;

/**
 * 📐️ Pure age-then-budget collection over abstract targets; no filesystem, so every language can reproduce it from the same fixture.
 * A unit is removed when it was unused for the target's age; if the target still exceeds its budget, the oldest remaining units go next.
 * A unit that is held, or touched within the target's quiet time, is never removed and is reported as kept instead.
 */
export function planCacheCollectionV1(targets:readonly CacheTargetV1[],nowMs:number):readonly CacheTargetPlanV1[]{
 return targets.map(target=>{
  const base:CacheRemovalV1["reason"]=target.kind==="retired"?"retired":target.kind==="undeclared"?"undeclared":"age",units=[...target.units].sort((left,right)=>byName(left.name,right.name)),removals:CacheRemovalV1[]=[],kept=new Map<string,CacheKeptV1>(),removed=new Set<string>();
  const block=(unit:CacheUnitV1):"held"|"quiet"|null=>target.heldBy!==undefined||unit.held!==undefined?"held":nowMs-unit.recencyMs<target.quietMs?"quiet":null;
  const keep=(unit:CacheUnitV1,reason:"held"|"quiet"):void=>{kept.set(unit.name,{target:target.id,name:unit.name,bytes:unit.bytes,reason});};
  for(const unit of units)if(target.unusedAgeMs!==null&&nowMs-unit.recencyMs>=target.unusedAgeMs){const reason=block(unit);if(reason)keep(unit,reason);else{removals.push({target:target.id,name:unit.name,bytes:unit.bytes,reason:base});removed.add(unit.name);}}
  const totalBytes=units.reduce((sum,unit)=>sum+unit.bytes,0);let remaining=totalBytes-units.filter(unit=>removed.has(unit.name)).reduce((sum,unit)=>sum+unit.bytes,0);
  if(target.budgetBytes!==null&&remaining>target.budgetBytes)for(const unit of units.filter(unit=>!removed.has(unit.name)).sort((left,right)=>left.recencyMs-right.recencyMs||byName(left.name,right.name))){
   if(remaining<=target.budgetBytes)break;
   const reason=block(unit);if(reason){keep(unit,reason);continue;}
   removals.push({target:target.id,name:unit.name,bytes:unit.bytes,reason:"budget"});removed.add(unit.name);remaining-=unit.bytes;
  }
  return Object.freeze({id:target.id,kind:target.kind,totalBytes,retainedBytes:remaining,budgetBytes:target.budgetBytes,overBudgetBytes:target.budgetBytes===null?0:Math.max(0,remaining-target.budgetBytes),removals:Object.freeze(removals),kept:Object.freeze([...kept.values()].filter(row=>!removed.has(row.name)))});
 });
}

/** 🔬️ Where a size came from: the filesystem's own (virtual) statfs, the size ledger written by a collection or a deep status, a walk of this very call, or nowhere. */
export type CacheSourceV1="statfs"|"recorded"|"walked"|"unknown";
/** 🏛️ One partition or control row of a status report. */
export type CachePartitionRowV1=Readonly<{id:string;kind:"partition"|"control";directory:string;state:"mounted"|"detached"|"directory"|"absent";usedBytes:number|null;budgetBytes:number|null;capacityBytes:number|null;partial:boolean;owner:CacheOwnerV1;source:CacheSourceV1;recordedAtMs:number|null;envelopeBytes:number|null;declaredBytes:number|null;recreate:boolean}>;
/** 🧩️ One area row of a status report. */
export type CacheAreaRowV1=Readonly<{id:string;root:CacheRootIdV1;partition:string|null;directory:string|null;present:boolean;measurable:boolean;usedBytes:number|null;budgetBytes:number|null;entries:number;partial:boolean;over:boolean;owner:CacheOwnerV1|null;source:CacheSourceV1;recordedAtMs:number|null}>;
/** 🏚️ One retired or undeclared entry still on disk. */
export type CacheEntryRowV1=Readonly<{root:CacheRootIdV1;path:string;kind:"retired"|"undeclared";usedBytes:number|null;entries:number;partial:boolean;source:CacheSourceV1;recordedAtMs:number|null}>;
/** 🌍️ One loose root's use against its declared maximum. */
export type CacheRootRowV1=Readonly<{id:CacheRootIdV1;directory:string;usedBytes:number;budgetBytes:number;partial:boolean}>;
/** 📈️ The status of the whole ledger, serializable as JSON. */
export type CacheReportV1=Readonly<{version:1;workspace:string;generatedAtMs:number;partitions:readonly CachePartitionRowV1[];areas:readonly CacheAreaRowV1[];roots:readonly CacheRootRowV1[];retired:readonly CacheEntryRowV1[];undeclared:readonly CacheEntryRowV1[]}>;
const percent=(used:number,budget:number):number=>budget<=0?0:Math.round(used*100/budget);

/** 🖨️ Renders a status report as lines in one language. */
export function formatCacheReportV1(language:CacheLanguageV1,report:CacheReportV1,ledger:CacheLedgerV1=cacheLedgerV1()):readonly string[]{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(language,key,variables),lines=[text("status.title",{root:report.workspace}),"",text("status.partitions")];
 const unit=(row:Measured,plain=false)=>(row.usedBytes===null?"?":formatCacheBytesV1(row.usedBytes)+(row.partial?text("note.partial"):""))+(plain?"":provenance(text,row,report.generatedAtMs));
 for(const row of report.partitions){
  const note=(row.kind==="control"||row.owner==="image"?"":text("note.managed",{owner:row.owner}))+(row.recreate&&row.envelopeBytes!==null&&row.declaredBytes!==null&&row.envelopeBytes!==row.declaredBytes?text("note.recreate",{actual:formatCacheBytesV1(row.envelopeBytes),declared:formatCacheBytesV1(row.declaredBytes)}):"");
  lines.push("  "+(row.budgetBytes===null?text("row.unbudgeted",{id:row.id,used:unit(row),note}):row.usedBytes===null?text("row.partitionUnmeasured",{id:row.id,state:text("state."+row.state),used:unit(row,row.state==="detached"||row.state==="absent"),budget:formatCacheBytesV1(row.budgetBytes),note}):text("row.partition",{id:row.id,state:text("state."+row.state),used:unit(row),budget:formatCacheBytesV1(row.budgetBytes),percent:percent(row.usedBytes,row.budgetBytes),note})));
 }
 const nested=report.areas.filter(row=>row.partition!==null);
 if(nested.length){lines.push("",text("status.nested"));for(const row of nested)lines.push("  "+areaLine(text,row,unit));}
 for(const root of report.roots){
  const rows=report.areas.filter(row=>row.partition===null&&row.root===root.id);
  lines.push("",text("status.loose",{root:root.directory,used:formatCacheBytesV1(root.usedBytes)+(root.partial?text("note.partial"):""),budget:formatCacheBytesV1(root.budgetBytes)}));
  for(const row of rows)lines.push("  "+areaLine(text,row,unit));
 }
 lines.push("");
 if([...report.partitions,...report.areas,...report.retired,...report.undeclared].some(row=>row.source==="recorded"||row.source==="unknown"))lines.push(text("status.deepHint"),"");
 if(report.retired.length){lines.push(text("status.retired"));for(const row of report.retired)lines.push("  "+text("row.entry",{path:row.path,used:unit(row),note:""}));}
 if(report.undeclared.length){lines.push(text("status.undeclared",{days:Math.round(ledger.roots.repository.undeclaredUnusedAgeMs/86400000)}));for(const row of report.undeclared)lines.push("  "+text("row.entry",{path:row.path,used:unit(row),note:""}));}
 if(!report.retired.length&&!report.undeclared.length)lines.push(text("status.clean"));
 return lines;
}

type Measured=Readonly<{usedBytes:number|null;partial:boolean;source:CacheSourceV1;recordedAtMs:number|null}>;
type Text=(key:string,variables?:Readonly<Record<string,string|number>>)=>string;
function provenance(text:Text,row:Measured,nowMs:number):string{
 if(row.source==="unknown")return row.usedBytes===null?text("note.unrecorded"):"";
 if(row.source!=="recorded"||row.recordedAtMs===null)return "";
 const minutes=Math.max(0,Math.round((nowMs-row.recordedAtMs)/60000)),age=minutes<60?text("age.minutes",{count:minutes}):minutes<2880?text("age.hours",{count:Math.round(minutes/60)}):text("age.days",{count:Math.round(minutes/1440)});
 return text("note.recorded",{age});
}

function areaLine(text:Text,row:CacheAreaRowV1,unit:(row:Measured,plain?:boolean)=>string):string{
 const id=row.partition?row.id+" ("+row.partition+")":row.id,note=(row.over?text("note.over"):"")+(row.owner&&row.owner!=="image"?text("note.managed",{owner:row.owner}):"")+(row.measurable?"":text("note.unknown"));
 return row.budgetBytes===null?text("row.unbudgeted",{id,used:unit(row),note}):row.usedBytes===null?text("row.areaUnmeasured",{id,used:unit(row,!row.measurable||!row.present),budget:formatCacheBytesV1(row.budgetBytes),note}):text("row.area",{id,used:unit(row),budget:formatCacheBytesV1(row.budgetBytes),percent:percent(row.usedBytes,row.budgetBytes),note});
}

/** 🏷️ Renders a removal reason in one language. */
export function formatCacheReasonV1(language:CacheLanguageV1,reason:CacheRemovalV1["reason"],unusedAgeMs:number|null):string{
 return cacheTextV1(language,"reason."+reason,{days:Math.round((unusedAgeMs??0)/86400000)});
}

/** 🧮️ One recorded size: bytes, whether the walk that produced it was cut short, and when. */
export type CacheSizeRecordV1=Readonly<{usedBytes:number;partial:boolean;atMs:number}>;
/** 📒️ The recorded sizes of areas, retired roots and undeclared entries, keyed `area`, `retired:<root>:<dir>`, `undeclared:<root>:<path>`. */
export type CacheSizeLedgerV1=Readonly<{version:1;records:Readonly<Record<string,CacheSizeRecordV1>>}>;
/** 📛️ The file name of the size ledger inside the repository cache root. */
export const CACHE_SIZE_FILE_V1=schema.$defs.runtime.const.sizeLedgerFile;

/** 📖️ Reads a size ledger; anything that is not a closed ledger is an empty one, because sizes are only a remembered measurement. */
export function readCacheSizeLedgerV1(value:unknown):CacheSizeLedgerV1{
 if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/sizeLedger"},value).length)return{version:1,records:{}};
 return value as CacheSizeLedgerV1;
}

/** 🧮️ The capacity a measured need demands: every part at its cap, plus headroom, plus the floor that must stay free. */
export function cacheNeedRequiredBytesV1(ledger:CacheLedgerV1,need:CacheNeedV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):number{
 const caps=need.components.reduce((sum,part)=>{
  if(part.area===undefined)return sum+(part.bytes??0);
  const area=ledger.areas.find(row=>row.id===part.area);if(!area)refuse("need names an unknown area "+part.area);
  return sum+(cacheAreaBudgetBytesV1(ledger,area!,partitions)??0);
 },0);
 return Math.ceil(caps*(100+need.headroomPercent)/100)+need.floorBytes;
}
