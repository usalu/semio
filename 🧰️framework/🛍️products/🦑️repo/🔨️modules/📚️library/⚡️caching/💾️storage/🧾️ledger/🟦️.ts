import {statSync} from "node:fs";
import {isAbsolute,join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import native from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️json/🟦️.ts";
import budgetPolicy from "../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 🌐️ The two languages every cache text is authored in; English comes first and nothing else is assumed. */
export type CacheLanguageV1="en"|"de";
/** 🏠️ Closed authored roots for repository, native and compact Rust storage. */
export type CacheRootIdV1=keyof typeof policy.roots;
/** 🗝️ Enumerates every schema-closed root for validation and physical accounting. */
export const CACHE_ROOT_IDS_V1:readonly CacheRootIdV1[]=Object.freeze(Object.keys(policy.roots) as CacheRootIdV1[]);
/** 🧑‍🔧️ Names the owner responsible for an area outside generic collection. */
export type CacheOwnerV1="image"|"control"|"fleet"|"cargo"|"nx"|"nx-tooling"|"bazel"|"semio"|"test-evidence"|"dashboard"|"graph"|"remote-cache"|"remote-execution";
type Bilingual=Readonly<Record<CacheLanguageV1,string>>;
/** 💰️ One declared budget: explicit bytes, a share of the partition, or whatever its siblings leave. */
export type CacheBudgetV1=Readonly<{bytes:number}>|Readonly<{basisPoints:number}>|Readonly<{remainder:true}>;
/** 🧹️ How an area is collected: how long it may stay unused, how long a unit must be quiet, what proves a unit is in use (`pid`: the unit name carries the process id, `(?<pid>…)` in `units`, and a live process holds it), whether a unit is a child (default) or a grandchild of the area directory, and which children of the directory are units at all (`units`, a nested area may then share its directory with the homes of other areas). */
export type CacheCollectV1=Readonly<{unusedAgeMs:number|null;quietMs:number;custody:"quiet"|"sqlite"|"lease"|"pid";lease?:string;depth?:1|2;units?:string;pressure?:"reclaimable"|"pinned"}>;
/** 🚰️ The free space below which an owner is asked to reclaim in the background, and the free space it reclaims up to. */
export type CacheReclaimV1=Readonly<{softAvailableBytes:number;targetAvailableBytes:number}>;
/** 📐️ The same thresholds relative to the volume of an image, for images whose size differs per machine: a share of the volume in basis points, raised to a floor that never exceeds one and a half times the share, so a small image can always satisfy it and soft stays below target. */
export type CacheReclaimShareV1=Readonly<{softBasisPoints:number;targetBasisPoints:number;softFloorBytes:number;targetFloorBytes:number}>;
const shareBytes=(volumeBytes:number,basisPoints:number,floorBytes:number):number=>{const share=Math.floor(volumeBytes*basisPoints/10000);return Math.max(share,Math.min(floorBytes,Math.floor(share*3/2)));};
/** 🎯️ The soft and target available bytes of a share rule on an image of this volume. Pure. */
export function cacheReclaimShareBytesV1(share:CacheReclaimShareV1,volumeBytes:number):CacheReclaimV1{return Object.freeze({softAvailableBytes:shareBytes(volumeBytes,share.softBasisPoints,share.softFloorBytes),targetAvailableBytes:shareBytes(volumeBytes,share.targetBasisPoints,share.targetFloorBytes)});}
/** 📏️ One part of a partition's measured need: its cap (an area's budget or explicit bytes) and what was measured. */
export type CacheNeedComponentV1=Readonly<{id:string;area?:string;bytes?:number;measuredBytes:number}>;
/** 🧪️ A partition's need derived from measurement: the caps of its parts plus headroom plus the floor that must stay free. */
export type CacheNeedV1=Readonly<{partition:string;measuredOn:string;headroomPercent:number;floorBytes:number;components:readonly CacheNeedComponentV1[]}>;
/** 📍️ One declared home of cached bytes. */
export type CacheAreaV1=Readonly<{id:string;root:CacheRootIdV1;kind:"partition"|"control"|"nested"|"tree";directory?:string;pattern?:string;partition?:string;budget?:CacheBudgetV1;managed?:Readonly<{owner:CacheOwnerV1}>;recreate?:"derived"|"carried";carry?:readonly string[];reclaim?:CacheReclaimV1;reclaimShare?:CacheReclaimShareV1;collect?:CacheCollectV1;writers:readonly string[];purpose:Bilingual}>;
/** 🪦️ One root that no writer feeds any more and the collector removes once it has been quiet. */
export type CacheRetirementV1=Readonly<{root:CacheRootIdV1;directory:string;quietMs:number;reason:Bilingual;within?:string}>;
/** 🥀️ The rule for cache roots that tools left below member directories: how long they stay quiet, which segments are never entered, which top-level directories are not looked at, and why they go. */
export type CacheStrayV1=Readonly<{quietMs:number;ignoredSegments:readonly string[];excludedDirectories:readonly string[];reason:Bilingual}>;
/** 🗃️ One loose root with the budget of all its areas and the rule for entries nobody declared. */
export type CacheRootV1=Readonly<{directory:string;maximumBytes:number;undeclaredUnusedAgeMs:number;undeclaredQuietMs:number;ignored:readonly string[]}>;
/** 🧾️ The whole closed ledger of cache homes. */
export type CacheLedgerV1=Readonly<{version:1;roots:Readonly<Record<CacheRootIdV1,CacheRootV1>>;areas:readonly CacheAreaV1[];retirements:readonly CacheRetirementV1[];strays?:CacheStrayV1;needs?:readonly CacheNeedV1[]}>;
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
type NestedHome={path:string[];units:RegExp|undefined};
const carved=(parent:NestedHome,child:NestedHome):boolean=>parent.units!==undefined&&parent.path.length<child.path.length&&inside(parent.path,child.path)&&!parent.units.test(child.path[parent.path.length]!);

/** 📋️ Validates the entire ledger, its budgets and its disjointness before any cache read or write relies on it. */
export function readCacheLedgerV1(value:unknown,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):CacheLedgerV1{
 const errors=validateJsonSchemaSubset(schema,value);if(errors.length)refuse(errors[0]!);
 const ledger=value as CacheLedgerV1,ids=new Set<string>(),claimed=new Map<CacheRootIdV1,string[][]>(CACHE_ROOT_IDS_V1.map(root=>[root,[]])),nested=new Map<string,{paths:NestedHome[];explicit:number;remainders:number}>();
 for(const area of ledger.areas){
  if(ids.has(area.id))refuse("duplicate area "+area.id);ids.add(area.id);
  if(area.recreate!==undefined&&area.kind!=="partition")refuse("only a partition may declare how it is recreated: "+area.id);
  if((area.recreate==="carried")!==(area.carry!==undefined))refuse("exactly a carried partition names what it carries: "+area.id);
  if(area.carry?.some(path=>segments(path).some(part=>part==="."||part===".."||part==="")))refuse("canonical carried directories required for "+area.id);
  if(area.directory!==undefined&&segments(area.directory).some(part=>part==="."||part===".."))refuse("canonical directory required for "+area.id);
  if(area.pattern!==undefined)try{new RegExp(area.pattern,"u");}catch{refuse("valid pattern required for "+area.id);}
  if(area.collect&&((area.collect.custody==="pid")!==(area.collect.units?.includes("(?<pid>")===true)||area.collect.units!==undefined&&(area.kind!=="nested"&&area.kind!=="tree"||area.collect.depth===2)))refuse("pid custody needs a unit pattern with a (?<pid>) group, and unit patterns need a depth-1 nested or tree area: "+area.id);
  const remainder=area.budget!==undefined&&"remainder" in area.budget;
  if(area.kind==="nested"){
   if(area.reclaim&&(area.reclaim.softAvailableBytes>=area.reclaim.targetAvailableBytes||area.partition==="graph"&&area.reclaim.targetAvailableBytes>2684354560))refuse("reclaim needs soft below target and, in the graph partition, a target within the headroom schema: "+area.id);
   if(area.reclaimShare&&(area.reclaim||area.reclaimShare.softBasisPoints>=area.reclaimShare.targetBasisPoints||area.reclaimShare.softFloorBytes>=area.reclaimShare.targetFloorBytes||!area.partition||[1073741824,4294967296,17179869184,103080263680,1099511627776,Object.hasOwn(partitions,area.partition)?partitions[area.partition]!.capacityBytes:0].some(volume=>{const row=cacheReclaimShareBytesV1(area.reclaimShare!,volume);return row.softAvailableBytes>=row.targetAvailableBytes||row.targetAvailableBytes>volume/2;})))refuse("reclaim share needs soft below target for every image size and no absolute reclaim beside it: "+area.id);
   if(!area.partition||!Object.hasOwn(partitions,area.partition)||area.pattern!==undefined||(area.collect===undefined)===(area.managed===undefined)||area.collect?.custody==="lease"||!area.budget||remainder===(area.directory!==undefined)||remainder&&area.collect!==undefined)refuse("nested area "+area.id+" needs a declared partition, exactly one of manager and collection, and a budget");
   const row=nested.get(area.partition!)??{paths:[],explicit:0,remainders:0};nested.set(area.partition!,row);
   if(remainder)row.remainders++;else{let units:RegExp|undefined;try{units=area.collect?.units===undefined?undefined:new RegExp(area.collect.units,"u");}catch{refuse("valid unit pattern required for "+area.id);}const own:NestedHome={path:segments(area.directory!),units};if(row.paths.some(other=>(inside(other.path,own.path)||inside(own.path,other.path))&&!carved(other,own)&&!carved(own,other)))refuse("overlapping nested areas in "+area.partition);row.paths.push(own);}
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
 const carvedOut:string[][][]=[];
 for(const retirement of ledger.retirements){
  if(segments(retirement.directory).some(part=>part==="."||part===".."))refuse("canonical retirement required: "+retirement.directory);
  if(retirement.within!==undefined){
   const home=ledger.areas.find(area=>area.id===retirement.within);
   if(home===undefined||home.kind!=="control"||home.root!==retirement.root||home.directory===undefined||!inside(segments(home.directory),segments(retirement.directory))||segments(home.directory).length===segments(retirement.directory).length)refuse("a carved-out retirement lies strictly inside a control area of its root: "+retirement.directory);
   if(carvedOut.some(other=>other.some(path=>inside(path,segments(retirement.directory))||inside(segments(retirement.directory),path))))refuse("overlapping carved-out retirements: "+retirement.directory);
   carvedOut.push([segments(retirement.directory)]);continue;
  }
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
 for(const root of CACHE_ROOT_IDS_V1){
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

/** 🏠️ The home of one declared area for a long-running writer: its directory, its budget in bytes and, on macOS where a partition is an attached image, a refusal while that image is not attached (a write below an unattached mount point lands on the host volume and is hidden when the image attaches later). A missing partition directory is refused the same way: on macOS every partition becomes an image, and a home written before it exists is content the image receiver must refuse. The refusal is a `CacheAreaDetachedError`. */
export function cacheAreaHomeV1(workspace:string,id:string,ledger:CacheLedgerV1=cacheLedgerV1(),partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):Readonly<{directory:string;budgetBytes:number|null}>{
 const area=ledger.areas.find(row=>row.id===id);if(area===undefined)throw Error("Declared cache area required: "+id);
 const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions);if(directory===null)throw Error("Directory of the cache area required: "+id);
 if(process.platform==="darwin"&&area.partition!==undefined){
  const parent=join(workspace,partitions[area.partition]!.directory),mount=join(parent,mountDirectory),device=(path:string)=>{try{return statSync(path).dev;}catch{return undefined;}};
  if(device(mount)===undefined||device(mount)===device(parent))throw Object.assign(Error("The partition "+area.partition+" is not attached, so the area "+id+" has no home yet / Die Partition "+area.partition+" ist nicht angehängt, der Bereich "+id+" hat noch kein Zuhause"),{name:"CacheAreaDetachedError"});
 }
 return Object.freeze({directory,budgetBytes:cacheAreaBudgetBytesV1(ledger,area,partitions)});
}

/** 🗂️ Resolves a loose root to its absolute directory. */
export function cacheRootDirectoryV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1):string{return join(workspace,...segments(ledger.roots[root].directory));}

/** 🚧️ Refuses a cache path whose first segments name no declared home of `root` (the repository cache root by default, `native` for `.cache`); the one runtime door every TypeScript writer of the repository root passes and the test of the native law. */
export function requireDeclaredCacheSegmentsV1(path:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1(),root:CacheRootIdV1="repository"):void{
 if(!path.length)return;
 const parts=path.flatMap(part=>part.split("/")),declared=ledger.areas.filter(area=>area.root===root);
 if(declared.some(area=>area.directory!==undefined&&(inside(segments(area.directory),parts)||inside(parts,segments(area.directory)))||area.pattern!==undefined&&new RegExp(area.pattern,"u").test(parts[0]!)))return;
 throw Error(cacheTextV1("en","error.undeclared",{segment:parts.join("/")})+"\n"+cacheTextV1("de","error.undeclared",{segment:parts.join("/")}));
}

/** 🔎️ Names what a path below a loose root is: a declared area, a container of declared homes, a retired root, an ignored file or nobody's. */
export type CacheClassV1=Readonly<{kind:"area"|"container"|"retired"|"ignored"|"undeclared";id?:string}>;
/** 🌿️ The single-segment directory names of the loose roots (`.cache`, `.r`): a directory of such a name below any member directory is a stray root. */
export function cacheStrayNamesV1(ledger:CacheLedgerV1):readonly string[]{return Object.freeze([...new Set(CACHE_ROOT_IDS_V1.map(root=>ledger.roots[root].directory).filter(directory=>!directory.includes("/")))]);}
/** 🧮️ Classifies one root-relative path against the ledger: an ignored entry is a listed root-relative path, or a listed file name anywhere inside a container of declared homes. */
export function classifyCachePathV1(ledger:CacheLedgerV1,root:CacheRootIdV1,path:string):CacheClassV1{
 const parts=segments(path),ignored=ledger.roots[root].ignored;if(ignored.includes(path))return{kind:"ignored"};
 for(const retirement of ledger.retirements)if(retirement.within!==undefined&&retirement.root===root&&inside(segments(retirement.directory),parts))return{kind:"retired",id:retirement.directory};
 for(const area of ledger.areas){
  if(area.root!==root)continue;
  if(area.pattern!==undefined&&new RegExp(area.pattern,"u").test(parts[0]!))return{kind:"area",id:area.id};
  if(area.directory===undefined||area.kind==="nested")continue;
  const own=segments(area.directory);if(inside(own,parts))return{kind:"area",id:area.id};
 }
 for(const retirement of ledger.retirements)if(retirement.root===root&&inside(segments(retirement.directory),parts))return{kind:"retired",id:retirement.directory};
 const homes=[...ledger.areas.filter(area=>area.root===root&&area.kind!=="nested"&&area.directory!==undefined).map(area=>segments(area.directory!)),...ledger.retirements.filter(row=>row.root===root).map(row=>segments(row.directory))];
 return homes.some(home=>inside(parts,home))?{kind:"container"}:ignored.includes(parts[parts.length-1]!)?{kind:"ignored"}:{kind:"undeclared"};
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

/** ⏳️ When a file was last used: its newest access or modification time that is not dated beyond `futureSkewMs` ahead of now (Bazel dates its installed files ten years ahead, and a skewed clock does the same), and the inode change time, which no tool can set, when every other time lies in the future; a future time never makes a file look recently used for ever. */
export function cacheRecencyMsV1(times:Readonly<{mtimeMs:number;atimeMs:number;ctimeMs:number}>,nowMs:number,skewMs:number=CACHE_RUNTIME_V1.futureSkewMs):number{
 const limit=nowMs+skewMs,past=[times.mtimeMs,times.atimeMs].filter(time=>time<=limit);
 return past.length?Math.max(...past):Math.min(times.ctimeMs,nowMs);
}

/** 🧱️ One independently removable child of an area, with the newest file time inside it and the holder that proves it is in use. */
export type CacheUnitV1=Readonly<{name:string;bytes:number;recencyMs:number;held?:string}>;
/** 🎯️ One thing the collector may shrink: a declared area, a retired root or the undeclared entries of a root. */
export type CacheTargetV1=Readonly<{id:string;kind:"area"|"retired"|"undeclared";budgetBytes:number|null;unusedAgeMs:number|null;quietMs:number;heldBy?:string;pinned?:true;units:readonly CacheUnitV1[]}>;
/** 🗑️ One planned removal and why. */
export type CacheRemovalV1=Readonly<{target:string;name:string;bytes:number;reason:"retired"|"undeclared"|"age"|"budget"|"pressure"}>;
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
export function planCacheCollectionV1(targets:readonly CacheTargetV1[],nowMs:number,reclaimBytes:number=0):readonly CacheTargetPlanV1[]{
 const plans=planCacheTargetsV1(targets,nowMs);
 return reclaimBytes>0?planCachePressureV1(targets,plans,nowMs,reclaimBytes):plans;
}
const planCacheTargetsV1=(targets:readonly CacheTargetV1[],nowMs:number):readonly CacheTargetPlanV1[]=>{
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
};
/**
 * 🗜️ Frees `reclaimBytes` more than the age and budget rules removed, oldest first across every declared area: a unit that is neither held nor touched within the quiet time of its area is stale or superseded and may go when the host disk is short, however young its age or however far below its budget the area is. Retired and undeclared targets were removed in full by the base plan already; held and quiet units stay, and so does every unit of a pinned target (`pressure: "pinned"`: a tool install that only a tool setup provisions and nothing recreates on demand, so pressure never takes it; its own age and budget still apply). Pure, so every language reproduces it from the same fixture.
 * @see ./🧪️tests/🟦️.ts
 */
export function planCachePressureV1(targets:readonly CacheTargetV1[],plans:readonly CacheTargetPlanV1[],nowMs:number,reclaimBytes:number):readonly CacheTargetPlanV1[]{
 let remaining=reclaimBytes-plans.reduce((sum,plan)=>sum+plan.removals.reduce((inner,removal)=>inner+removal.bytes,0),0);
 if(remaining<=0)return plans;
 const gone=new Set(plans.flatMap(plan=>plan.removals.map(removal=>plan.id+"\0"+removal.name))),extra=new Map<string,CacheRemovalV1[]>();
 const candidates=targets.flatMap(target=>target.kind!=="area"||target.heldBy!==undefined||target.pinned?[]:target.units.filter(unit=>unit.held===undefined&&nowMs-unit.recencyMs>=target.quietMs&&!gone.has(target.id+"\0"+unit.name)).map(unit=>({target,unit}))).sort((left,right)=>left.unit.recencyMs-right.unit.recencyMs||byName(left.target.id,right.target.id)||byName(left.unit.name,right.unit.name));
 for(const{target,unit}of candidates){
  if(remaining<=0)break;
  extra.set(target.id,[...extra.get(target.id)??[],{target:target.id,name:unit.name,bytes:unit.bytes,reason:"pressure"}]);remaining-=unit.bytes;
 }
 return plans.map(plan=>{
  const added=extra.get(plan.id);if(added===undefined)return plan;
  const names=new Set(added.map(row=>row.name)),bytes=added.reduce((sum,row)=>sum+row.bytes,0);
  return Object.freeze({...plan,retainedBytes:plan.retainedBytes-bytes,overBudgetBytes:plan.budgetBytes===null?0:Math.max(0,plan.retainedBytes-bytes-plan.budgetBytes),removals:Object.freeze([...plan.removals,...added]),kept:Object.freeze(plan.kept.filter(row=>!names.has(row.name)))});
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
  const note=(row.owner==="control"||row.owner==="image"?"":text("note.managed",{owner:row.owner}))+(row.recreate&&row.envelopeBytes!==null&&row.declaredBytes!==null&&row.envelopeBytes!==row.declaredBytes?text("note.recreate",{actual:formatCacheBytesV1(row.envelopeBytes),declared:formatCacheBytesV1(row.declaredBytes)}):"");
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
 if(reason==="age"&&unusedAgeMs!==null&&unusedAgeMs<86400000)return cacheTextV1(language,"reason.ageHours",{hours:Math.max(1,Math.round(unusedAgeMs/3600000))});
 return cacheTextV1(language,"reason."+reason,{days:Math.round((unusedAgeMs??0)/86400000)});
}

/** 📹️ One recorded size: bytes, whether the walk that produced it was cut short, and when. */
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

/** 💽️ Charges remembered sizes only for the cache homes that the current ledger recognizes. */
export function cacheSizeChargeV1(ledger:CacheLedgerV1,sizes:CacheSizeLedgerV1):number{
 const known=new Set([...ledger.areas.map(row=>row.id),...ledger.retirements.map(row=>"retired:"+row.root+":"+row.directory)]);
 let bytes=0;
 for(const [key,record] of Object.entries(sizes.records)){
  if(known.has(key)){bytes+=record.usedBytes;continue;}
  const root=CACHE_ROOT_IDS_V1.find(id=>key.startsWith("undeclared:"+id+":"));
  if(root===undefined)continue;
  const path=key.slice("undeclared:".length+root.length+1);
  if(segments(path).every(part=>part!==""&&part!=="."&&part!=="..")&&classifyCachePathV1(ledger,root,path).kind==="undeclared")bytes+=record.usedBytes;
 }
 return bytes;
}

/** ⚡️ The capacity a measured need demands: every part at its cap, plus headroom, plus the floor that must stay free. */
export function cacheNeedRequiredBytesV1(ledger:CacheLedgerV1,need:CacheNeedV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):number{
 const caps=need.components.reduce((sum,part)=>{
  if(part.area===undefined)return sum+(part.bytes??0);
  const area=ledger.areas.find(row=>row.id===part.area);if(!area)refuse("need names an unknown area "+part.area);
  return sum+(cacheAreaBudgetBytesV1(ledger,area!,partitions)??0);
 },0);
 return Math.ceil(caps*(100+need.headroomPercent)/100)+need.floorBytes;
}
