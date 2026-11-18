import{createHash}from"node:crypto";
import{realpathSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{expandRootV1,neutralizeRootV1}from"../../🛰️exchange/🟦️.ts";
import{indexTextRewriterV1}from"../../../🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";
import{createRowPackerV1,unpackRowListsV1,type PackedRowsV1}from"../🗜️packed-rows/🟦️.ts";

/** 🧳️ Which observations of a record name the producer's machine or its way of spelling a path: the kinds of the location rows, the dependency generation directory and its token, the kinds whose path or fact is a path, the tokens of the root and of the analysis provider owner. See `./🧬️schema/🔣️.json`. */
export const portableRowsPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type PortableRowsPolicyV1=typeof portableRowsPolicyV1;
export type ObservationRowV1=Readonly<{kind:string;path:string;fact?:unknown}>;
/** 🪟️ The checkout of one machine and how it spells paths: a Windows root starts with a drive letter or is a UNC share. */
export type HostV1=Readonly<{root:string;windows:boolean}>;
export type PortableRecordV1=Readonly<{digest:string;authority:readonly ObservationRowV1[];observations:readonly ObservationRowV1[];image:string}>;
export type ClosureBundleV1=Readonly<{recipe:unknown;rows:readonly ObservationRowV1[];layers:Iterable<readonly ObservationRowV1[]>}>&Readonly<Record<string,unknown>>;
export type KeyMapV1=ReadonlyMap<string,string>;

/** 🚫️ A record or text that names the root in a spelling that cannot be mapped: it is neither published nor taken, the project is inferred locally. */
export class UnportableV1 extends Error{constructor(reason:string){super(reason);this.name="UnportableV1";}}

const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),slashed=(text:string)=>text.replaceAll("\\","/");
const generationPattern=(policy:PortableRowsPolicyV1)=>new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])([0-9a-f]{16,64})(?=[\\\\/]|$)`,"gu");
const mapStrings=(value:unknown,change:(text:string)=>string):unknown=>typeof value==="string"?change(value):Array.isArray(value)?value.map(item=>mapStrings(item,change)):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,mapStrings(item,change)])):value;
const rewrite=(row:ObservationRowV1,change:(text:string)=>string):ObservationRowV1=>({...row,path:change(row.path),...row.fact===undefined?{}:{fact:mapStrings(row.fact,change)}});
/** 🔎️ Whether a string, a key or a string inside `value` holds `token`: what `JSON.stringify(value).includes(token)` answered for a token that JSON writes as it is, without writing the row out for every row of every layer of every entry a machine takes. */
const carries=(value:unknown,token:string):boolean=>typeof value==="string"?value.includes(token):value!==null&&typeof value==="object"?Array.isArray(value)?value.some(item=>carries(item,token)):Object.keys(value).some(key=>key.includes(token)||carries((value as Record<string,unknown>)[key],token)):false;
const LEAD="(?<![\\p{L}\\p{N}_.-])",BOUNDARY="(?=$|[^\\p{L}\\p{N}_.-])";

/** 🖥️ The host of a checkout root, by the way the root is spelled. */
export const hostOfRootV1=(root:string):HostV1=>Object.freeze({root,windows:/^(?:[A-Za-z]:(?:[\\/]|$)|[\\/]{2}[^\\/])/u.test(root)});
const bare=(text:string)=>slashed(text).replace(/^\/\/\?\//u,"").replace(/\/+$/u,"");
const rootSlash=(host:HostV1)=>bare(host.root);

/** 📍️ Whether `path` is a directory above `root` (every spelling of the separators, any drive; on Windows without regard to case). */
export function aboveRootV1(path:string,root:string):boolean{
 const windows=hostOfRootV1(root).windows,fold=(text:string)=>windows?text.toLowerCase():text,below=fold(slashed(root).replace(/\/+$/u,"")+"/"),above=fold(slashed(path).endsWith("/")?slashed(path):slashed(path)+"/");
 return below!==above&&below.startsWith(above);
}

/** 🧭️ What follows the root in `path` (empty for the root itself, otherwise starting with a forward slash, trailing separator kept), or undefined when `path` lies elsewhere. Windows: either separator, the extended prefix is ignored, the root is compared without case and the rest keeps its spelling. */
export function relativeToRootV1(path:string,host:HostV1):string|undefined{
 if(!host.windows){
  const root=host.root.replace(/\/+$/u,"");
  if(path===root)return"";
  return path.startsWith(root+"/")?path.slice(root.length):undefined;
 }
 const text=slashed(path).replace(/^\/\/\?\//u,""),root=rootSlash(host);
 if(text.length<root.length||text.slice(0,root.length).toLowerCase()!==root.toLowerCase())return undefined;
 if(text.length===root.length)return"";
 return text[root.length]==="/"?text.slice(root.length):undefined;
}

/** 🔣️ A path in the neutral form: the root token followed by the rest with forward slashes, or the path itself when it lies outside the root. */
export function neutralPathV1(path:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string{
 const rest=relativeToRootV1(path,host);
 return rest===undefined?path:policy.rootToken+rest;
}

/** 🏡️ A neutral path in the spelling of the host: a Windows host writes the root as given and the rest with backslashes, as its own inference records paths; a path that does not start with the root token stays. */
export function localPathV1(path:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string{
 if(!path.startsWith(policy.rootToken))return path;
 const rest=path.slice(policy.rootToken.length),root=host.windows?host.root.replace(/[\\/]+$/u,""):host.root.replace(/\/+$/u,"");
 if(rest!==""&&rest[0]!=="/")return path;
 return host.windows?root+rest.replaceAll("/","\\"):root+rest;
}

/** 📤️ The observations of a record as every machine can replay them: the `lstat` of the directories above `root` is dropped, and when the rows name exactly one dependency generation its id is replaced by the token. */
export const portableRowsV1=(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]=>generationRowsV1(withoutLocationRowsV1(rows,root,policy),policy);
const isLocationRow=(row:ObservationRowV1,root:string,policy:PortableRowsPolicyV1)=>policy.locationKinds.includes(row.kind)&&row.fact==="directory"&&aboveRootV1(row.path,root);
const withoutLocationRowsV1=(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1):readonly ObservationRowV1[]=>rows.filter(row=>!isLocationRow(row,root,policy));
function generationRowsV1(kept:readonly ObservationRowV1[],policy:PortableRowsPolicyV1,memo?:WeakMap<object,readonly string[]>):readonly ObservationRowV1[]{
 const pattern=generationPattern(policy),ids=new Set<string>(),idsOf=(row:ObservationRowV1)=>{let found=memo?.get(row);if(found===undefined){found=[...JSON.stringify([row.path,row.fact??null]).matchAll(pattern)].map(match=>match[2]!);memo?.set(row,found);}return found;};
 for(const row of kept)for(const id of idsOf(row))ids.add(id);
 if(ids.size!==1)return kept;
 const[id]=[...ids],replacement=new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])${id}(?=[\\\\/]|$)`,"gu");
 return kept.map(row=>JSON.stringify([row.path,row.fact??null]).includes(id)?rewrite(row,text=>text.replaceAll(replacement,`$1${policy.token}`)):row);
}

/** 📥️ The observations of a portable record for this machine: the token becomes the machine's own dependency generation, or a name no directory has when it has none, so the project is inferred here. */
export function localRowsV1(rows:readonly ObservationRowV1[],generation:string|undefined,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const put=generation??policy.unavailable;
 return rows.map(row=>carries(row.path,policy.token)||carries(row.fact,policy.token)?rewrite(row,text=>text.replaceAll(policy.token,put)):row);
}

/** 🔎️ The id of the dependency generation this machine runs its inference tooling from: the real path of the `node_modules` of `root`, undefined where it is no generation directory. */
export function dependencyGenerationV1(root:string,resolve:(path:string)=>string=realpathSync,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string|undefined{
 try{const found=generationPattern(policy).exec(slashed(resolve(join(root,policy.probe))));return found?.[2];}catch{return undefined;}
}

const sortedJson=(value:unknown):unknown=>Array.isArray(value)?value.map(sortedJson):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).filter(key=>(value as Record<string,unknown>)[key]!==undefined).sort().map(key=>[key,sortedJson((value as Record<string,unknown>)[key])])):value;
/** 🔑️ The key of a closure recipe: the digest of its sorted JSON, which names the root the recipe was made for (the same function as `closureRecipeKeyV1` of the inference). */
export const closureKeyV1=(recipe:unknown):string=>createHash("sha256").update(JSON.stringify(sortedJson(recipe))).digest("hex");

const rootPattern=(host:HostV1)=>new RegExp(LEAD+escape(rootSlash(host))+BOUNDARY,"giu");
const rawPatterns=(host:HostV1)=>{const root=bare(host.root).replaceAll("/","\\"),doubled=root.replaceAll("\\","\\\\");return[new RegExp(LEAD+escape(root)+BOUNDARY,"iu"),new RegExp(LEAD+escape(doubled)+BOUNDARY,"iu")];};

/** 🧼️ A text in the neutral form: every spelling of the root becomes the root token. A POSIX root is replaced as everywhere in the fleet; a Windows text spells the root with forward slashes (the way the commands of its inference are written), and a text that still holds the backslash or the doubled backslash spelling cannot be mapped and is refused. */
export function neutralTextV1(text:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string{
 if(!host.windows)return Buffer.from(neutralizeRootV1(Buffer.from(text),host.root)).toString("utf8");
 const moved=text.replace(rootPattern(host),policy.rootToken);
 if(rawPatterns(host).some(pattern=>pattern.test(moved)))throw new UnportableV1("root spelled with backslashes");
 return moved;
}

/** 🏘️ A neutral text in the spelling of the host (the root with forward slashes on every host); a text that still holds a token of this family is refused. */
export function localTextV1(text:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):string{
 const moved=text.includes(policy.rootToken)?host.windows?text.replaceAll(policy.rootToken,()=>rootSlash(host)):Buffer.from(expandRootV1(Buffer.from(text),host.root)).toString("utf8"):text;
 if(moved.includes(policy.residue))throw new UnportableV1("root token of another spelling");
 return moved;
}

const providerPrefix=(id:string,policy:PortableRowsPolicyV1)=>{const match=new RegExp(policy.ownerPattern,"u").exec(id);return match===null?undefined:match[1];};
const neutralOwner=(text:string,policy:PortableRowsPolicyV1)=>{const prefix=providerPrefix(text,policy);return prefix===undefined?text:prefix+policy.ownerToken;};
const localOwner=(providers:readonly string[],policy:PortableRowsPolicyV1)=>(text:string)=>{
 if(!text.endsWith(policy.ownerToken))return text;
 const prefix=text.slice(0,text.length-policy.ownerToken.length);
 return providers.find(id=>providerPrefix(id,policy)===prefix)??text;
};

type Direction=Readonly<{host:HostV1;keys:KeyMapV1;owner:(text:string)=>string;path:(text:string)=>string;text:(text:string)=>string;item?:(text:string)=>string}>;
const mapPathJson=(text:string,change:(item:unknown,index:number)=>unknown):string=>{
 let parsed:unknown;
 try{parsed=JSON.parse(text);}catch{return text;}
 return Array.isArray(parsed)?JSON.stringify(parsed.map(change)):text;
};
function convertRow(row:ObservationRowV1,way:Direction,policy:PortableRowsPolicyV1):ObservationRowV1{
 const{kind}=row;
 if(policy.jsonKinds.includes(kind)){
  const path=mapPathJson(row.path,(item,index)=>index===0&&typeof item==="string"?way.path(item):index===1&&kind==="closure"&&typeof item==="string"?way.keys.get(item)??item:index===0?item:mapStrings(item,way.item??way.text));
  const fact=kind==="closure"&&typeof row.fact==="string"?way.text(row.fact):policy.factPathKinds.includes(kind)&&typeof row.fact==="string"?way.path(row.fact):row.fact;
  return{...row,path,...fact===undefined?{}:{fact}};
 }
 if(kind==="analysis"&&row.fact!==null&&typeof row.fact==="object"){
  const fact=row.fact as Record<string,unknown>,nested=Array.isArray(fact.nestedObservations)?(fact.nestedObservations as ObservationRowV1[]).map(item=>convertRow(item,way,policy)):fact.nestedObservations;
  return{...row,path:way.path(row.path),fact:{...fact,...typeof fact.provider==="string"?{provider:way.owner(fact.provider)}:{},...nested===undefined?{}:{nestedObservations:nested}}};
 }
 if(kind==="field"&&row.fact!==null&&typeof row.fact==="object")return{...row,path:way.path(row.path),fact:mapStrings(row.fact,way.item??way.text)};
 if(policy.pathKinds.includes(kind))return{...row,path:way.path(row.path),...policy.factPathKinds.includes(kind)&&typeof row.fact==="string"?{fact:way.path(row.fact)}:{}};
 return{...row,path:way.text(row.path),...typeof row.fact==="string"?{fact:way.text(row.fact)}:{}};
}
const zeroStamps=(fact:Record<string,unknown>,policy:PortableRowsPolicyV1):Record<string,unknown>=>Object.fromEntries(Object.entries(fact).map(([name,value])=>[name,policy.stampFields.includes(name)&&typeof value==="number"?policy.stampValue:value]));
/** 🕰️ A row without the stamps of the machine that recorded it: the modification time, the change time and the inode of a file are hints that let the machine that recorded the row skip hashing the file, they differ on every machine for the same content, and a taker decides by the size and the SHA-256 when they do not match. The neutral form carries the declared value instead, so two machines that infer one project write the same bytes; the `file` rows and the sources of `analysis` rows (and their nested rows) are covered. */
export function unstampRowV1(row:ObservationRowV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):ObservationRowV1{
 const fact=row.fact;
 if(fact===null||typeof fact!=="object"||Array.isArray(fact))return row;
 const body=fact as Record<string,unknown>;
 if(row.kind==="file"&&body.error===undefined)return{...row,fact:zeroStamps(body,policy)};
 if(row.kind!=="analysis")return row;
 const source=body.source!==null&&typeof body.source==="object"&&!Array.isArray(body.source)?zeroStamps(body.source as Record<string,unknown>,policy):body.source,nested=Array.isArray(body.nestedObservations)?(body.nestedObservations as ObservationRowV1[]).map(item=>unstampRowV1(item,policy)):body.nestedObservations;
 return{...row,fact:{...body,...source===undefined?{}:{source},...nested===undefined?{}:{nestedObservations:nested}}};
}
const neutralWay=(host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1):Direction=>({host,keys,owner:text=>neutralOwner(text,policy),path:text=>neutralPathV1(text,host,policy),text:text=>neutralTextV1(text,host,policy),item:text=>relativeToRootV1(text,host)===undefined?neutralTextV1(text,host,policy):neutralPathV1(text,host,policy)});
const localWay=(host:HostV1,keys:KeyMapV1,providers:readonly string[],policy:PortableRowsPolicyV1):Direction=>({host,keys,owner:localOwner(providers,policy),path:text=>localPathV1(text,host,policy),text:text=>localTextV1(text,host,policy)});

/** 🧠️ What one conversion remembers per row object, valid for one host, one key map and one policy: the converted row (null for a dropped location row) and the generation ids of the converted rows. A closure entry names the same row objects in thousands of layers, so a conversion that shares a memo converts each distinct row once. */
export type RowMemoV1=Readonly<{rows:WeakMap<object,ObservationRowV1|null>;ids:WeakMap<object,readonly string[]>}>;
export const rowMemoV1=():RowMemoV1=>({rows:new WeakMap(),ids:new WeakMap()});

/** 🌍️ The rows of a record in the neutral form of the host's spelling (see {@link neutralRecordV1}): the location rows and the generation as {@link portableRowsV1}, then paths, provider owners and closure keys. Rows that `memo` has seen are not converted again. */
export function neutralRowsV1(rows:readonly ObservationRowV1[],host:HostV1,keys:KeyMapV1=new Map(),policy:PortableRowsPolicyV1=portableRowsPolicyV1,memo:RowMemoV1=rowMemoV1()):readonly ObservationRowV1[]{
 const way=neutralWay(host,keys,policy),kept:ObservationRowV1[]=[];
 for(const row of rows){
  let moved=memo.rows.get(row);
  if(moved===undefined){moved=isLocationRow(row,host.root,policy)?null:unstampRowV1(convertRow(row,way,policy),policy);memo.rows.set(row,moved);}
  if(moved!==null)kept.push(moved);
 }
 return generationRowsV1(kept,policy,memo.ids);
}

/** 🏠️ The rows of a neutral record for the host: paths spelled the host's way, the provider owners the host registered and its own closure keys; the generation is put in as {@link localRowsV1} does, through `generation`. Rows that `memo` has seen are not converted again. */
export function localizeRowsV1(rows:readonly ObservationRowV1[],host:HostV1,input:Readonly<{keys?:KeyMapV1;providers?:readonly string[];generation?:string|undefined}>={},policy:PortableRowsPolicyV1=portableRowsPolicyV1,memo:WeakMap<object,ObservationRowV1>=new WeakMap()):readonly ObservationRowV1[]{
 const way=localWay(host,input.keys??new Map(),input.providers??[],policy),put=input.generation??policy.unavailable;
 return rows.map(row=>{
  let moved=memo.get(row);
  if(moved===undefined){const converted=convertRow(row,way,policy);moved=carries(converted.path,policy.token)||carries(converted.fact,policy.token)?rewrite(converted,text=>text.replaceAll(policy.token,put)):converted;memo.set(row,moved);}
  return moved;
 });
}

const isMarker=(item:unknown):item is{closureRef:string}=>item!==null&&typeof item==="object"&&!Array.isArray(item)&&Object.keys(item).length===1&&typeof(item as{closureRef?:unknown}).closureRef==="string";
const mapMarkers=(value:unknown,keys:KeyMapV1):unknown=>Array.isArray(value)?value.map(item=>mapMarkers(item,keys)):isMarker(value)?{closureRef:keys.get(value.closureRef)??value.closureRef}:value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,mapMarkers(item,keys)])):value;

/** 🗜️ Moves the recipe table of a result image between the producer's keys and the neutral ones (or the other way): each recipe is rewritten by `text`, its key is the digest of the rewritten recipe, and the `closureRef` markers follow. `check` demands that a key of the input is the digest of its recipe. */
function moveImage(image:string,text:(text:string)=>string,check:boolean):Readonly<{image:string;keys:KeyMapV1}>{
 let parsed:Record<string,unknown>;
 try{parsed=JSON.parse(image) as Record<string,unknown>;}catch{throw new UnportableV1("result is no JSON");}
 const keys=new Map<string,string>(),table=parsed.closureRecipes as Record<string,unknown>|undefined,moved:[string,unknown][]=[];
 if(table!==undefined&&table!==null&&typeof table==="object")for(const[key,recipe] of Object.entries(table)){
  if(check&&closureKeyV1(recipe)!==key)throw new UnportableV1("closure key is not the digest of its recipe");
  const next=JSON.parse(text(JSON.stringify(recipe))),nextKey=closureKeyV1(next);
  keys.set(key,nextKey);moved.push([nextKey,next]);
 }
 const body=mapMarkers({...parsed,...table===undefined||table===null?{}:{closureRecipes:Object.fromEntries(check?moved.sort((a,b)=>a[0]<b[0]?-1:1):moved)}},keys);
 return{image:text(JSON.stringify(body)),keys};
}

/** 🌐️ A project record in the neutral form every host can take (see the schema): rows, result text, recipe keys. Throws {@link UnportableV1} for a record that names the root in a spelling that cannot be mapped. */
export function neutralRecordV1(record:PortableRecordV1,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{record:PortableRecordV1;keys:KeyMapV1}>{
 const{image,keys}=moveImage(record.image,text=>neutralTextV1(text,host,policy),true);
 return{keys,record:{digest:record.digest,authority:neutralRowsV1(record.authority,host,keys,policy),observations:neutralRowsV1(record.observations,host,keys,policy),image}};
}

/** 🏙️ A neutral record for the host: the inverse of {@link neutralRecordV1}. `providers` are the ids of the source analyses this host registered, `generation` its dependency generation. Throws {@link UnportableV1} when a root token of another spelling remains. */
export function localRecordV1(record:PortableRecordV1,host:HostV1,input:Readonly<{providers?:readonly string[];generation?:string|undefined}>={},policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{record:PortableRecordV1;keys:KeyMapV1}>{
 const{image,keys}=moveImage(record.image,text=>localTextV1(text,host,policy),false);
 return{keys,record:{digest:record.digest,authority:localizeRowsV1(record.authority,host,{...input,keys},policy),observations:localizeRowsV1(record.observations,host,{...input,keys},policy),image}};
}

const FALLBACK=Symbol("not plain JSON");
const moveRest=(value:unknown,change:(text:string)=>string,seen:Map<string,string>):unknown=>{
 if(typeof value==="string"){let moved=seen.get(value);if(moved===undefined){moved=change(value);seen.set(value,moved);}return moved;}
 if(value===null||typeof value==="boolean")return value;
 if(typeof value==="number"){if(!Number.isFinite(value))throw FALLBACK;return value;}
 if(Array.isArray(value)){const moved=new Array<unknown>(value.length);for(let at=0;at<value.length;at++)moved[at]=moveRest(value[at],change,seen);return moved;}
 const body=value as Record<string,unknown>;
 if(typeof value!=="object"||typeof body.toJSON==="function"||Object.getPrototypeOf(value)!==Object.prototype)throw FALLBACK;
 const moved:Record<string,unknown>={};
 for(const key of Object.keys(body)){if(body[key]===undefined)throw FALLBACK;moved[moveRest(key,change,seen) as string]=moveRest(body[key],change,seen);}
 return moved;
};

/** 🪶️ The members of a bundle besides its rows and layers (its value and its sidecar, which for the library closures hold hundreds of megabytes) in the neutral or the host's spelling: every distinct string is moved once and the structure is copied, where writing the whole member out as one JSON text and reading it back held the text, its copy and the parsed result beside the member (the 4.4 GB of one worker). A Windows host, an empty root and members that are no plain JSON take the text path, which gives the same result. @see ./🧪️tests/🪶️rest-walk */
export function movedRestV1(rest:Readonly<Record<string,unknown>>,direction:"neutral"|"local",host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Record<string,unknown>{
 if(!host.windows&&host.root!==""){
  const rewrite=indexTextRewriterV1(host.root,direction==="neutral"?"neutralize":"expand"),change=direction==="neutral"?rewrite:(text:string)=>{const moved=text.includes(policy.rootToken)?rewrite(text):text;if(moved.includes(policy.residue))throw new UnportableV1("root token of another spelling");return moved;};
  try{return moveRest(rest,change,new Map()) as Record<string,unknown>;}catch(error){if(error!==FALLBACK)throw error;}
 }
 return JSON.parse((direction==="neutral"?neutralTextV1:localTextV1)(JSON.stringify(rest),host,policy)) as Record<string,unknown>;
}

const bundleParts=(bundle:ClosureBundleV1)=>{const{rows,layers,...rest}=bundle;return{rows,layers,rest};};

/** 🚚️ The keys of the recipes of the closure bundles (shared entries that are not in the table of the record), so rows that name them follow: from the producer's key to the neutral one, or from the neutral key to the host's own. A bundle whose key is not the digest of its recipe (neutral direction) or whose recipe cannot be moved is left out. */
export function bundleKeysV1(bundles:readonly Readonly<{recipeKey:string;bundle:ClosureBundleV1}>[],host:HostV1,direction:"neutral"|"local",policy:PortableRowsPolicyV1=portableRowsPolicyV1):KeyMapV1{
 const keys=new Map<string,string>(),move=direction==="neutral"?neutralTextV1:localTextV1;
 for(const{recipeKey,bundle} of bundles){try{if(direction==="local"||closureKeyV1(bundle.recipe)===recipeKey)keys.set(recipeKey,closureKeyV1(JSON.parse(move(JSON.stringify(bundle.recipe),host,policy))));}catch{}}
 return keys;
}

/** 📦️ A closure bundle in the neutral form (its recipe, its other fields and its rows), named by the neutral key; undefined when it cannot be mapped (the taker derives the entry itself). */
export function neutralBundleV1(input:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>,host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipeKey:string;bundle:ClosureBundleV1}>|undefined{
 try{
  if(closureKeyV1(input.bundle.recipe)!==input.recipeKey)return undefined;
  const{rows,layers,rest}=bundleParts(input.bundle),moved=movedRestV1(rest,"neutral",host,policy),memo=rowMemoV1();
  return{recipeKey:closureKeyV1(moved.recipe),bundle:{...moved,rows:neutralRowsV1(rows,host,keys,policy,memo),layers:Array.from(layers,layer=>neutralRowsV1(layer,host,keys,policy,memo))} as unknown as ClosureBundleV1};
 }catch{return undefined;}
}

/** 🚛️ A neutral closure bundle for the host, named by the key of its recipe in the host's spelling; undefined when it cannot be mapped. */
export function localBundleV1(input:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>,host:HostV1,keys:KeyMapV1,context:Readonly<{providers?:readonly string[];generation?:string|undefined}>={},policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipeKey:string;bundle:ClosureBundleV1}>|undefined{
 try{
  const{rows,layers,rest}=bundleParts(input.bundle),moved=movedRestV1(rest,"local",host,policy),memo=new WeakMap<object,ObservationRowV1>();
  return{recipeKey:closureKeyV1(moved.recipe),bundle:{...moved,rows:localizeRowsV1(rows,host,{...context,keys},policy,memo),layers:Array.from(layers,layer=>localizeRowsV1(layer,host,{...context,keys},policy,memo))} as unknown as ClosureBundleV1};
 }catch{return undefined;}
}

/** 🗝️ The map from the neutral key of every recipe in the table of a neutral image to the key of the same recipe in the host's spelling. */
export const localKeysV1=(image:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):KeyMapV1=>{try{return moveImage(image,text=>localTextV1(text,host,policy),false).keys;}catch{return new Map();}};

/** 🗜️ A project record as it is stored in the fleet cache: the rows of both lists once each, see `../🗜️packed-rows`. */
export type PackedRecordV1=Readonly<{version:2;digest:string;image:string;rows:PackedRowsV1}>;
/** 🗜️ A closure entry as it is stored in the fleet cache: list 0 of `rows` holds its rows and list n + 1 its layer n, extra 0 its value and extra 1 its sidecar; `rest` holds the other members. */
export type PackedClosureV1=Readonly<{version:2;recipe:unknown;rest:Readonly<Record<string,unknown>>;rows:PackedRowsV1}>;
/** 🔑️ The closure keys a conversion asked for and what it was told, so that a cached conversion is used again only while the key map still answers the same. */
export type ConsultedKeysV1=ReadonlyMap<string,string|undefined>;

/** 👂️ A key map that answers like `keys` and remembers every key it was asked for. */
export function recordingKeysV1(keys:KeyMapV1):Readonly<{keys:KeyMapV1;consulted:Map<string,string|undefined>}>{
 const consulted=new Map<string,string|undefined>();
 return{consulted,keys:{get:(key:string)=>{const found=keys.get(key);consulted.set(key,found);return found;}} as unknown as KeyMapV1};
}

/** 🎯️ Whether `keys` answers every key of `consulted` as it did when `consulted` was recorded. */
export const consultedHoldV1=(consulted:ConsultedKeysV1,keys:KeyMapV1):boolean=>[...consulted].every(([key,value])=>keys.get(key)===value);

/** 📦️ The neutral record in the stored form (see {@link PackedRecordV1}). */
export function packRecordV1(record:PortableRecordV1):PackedRecordV1{
 const packer=createRowPackerV1();packer.add(record.authority);packer.add(record.observations);
 return{version:2,digest:record.digest,image:record.image,rows:packer.finish()};
}

/** 📖️ The neutral record of a stored form; throws when the form does not hold together. */
export function unpackRecordV1(packed:PackedRecordV1):PortableRecordV1{
 if(packed===null||typeof packed!=="object"||packed.version!==2||typeof packed.digest!=="string"||typeof packed.image!=="string")throw new UnportableV1("record form");
 const{lists}=unpackRowListsV1(packed.rows);
 if(lists.length!==2)throw new UnportableV1("record lists");
 return{digest:packed.digest,authority:lists[0]!,observations:lists[1]!,image:packed.image};
}

/** 🌊️ A closure bundle in the neutral stored form, built layer by layer: only the distinct rows and the numbers of each layer are held, whatever the number of layers, so memory follows the distinct material and not the expanded entry. `consulted` names the closure keys the rows asked for. Undefined when the bundle cannot be mapped (the taker derives the entry itself). */
export function packBundleV1(input:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>,host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipeKey:string;document:PackedClosureV1;consulted:ConsultedKeysV1}>|undefined{
 try{
  if(closureKeyV1(input.bundle.recipe)!==input.recipeKey)return undefined;
  const{rows,layers,rest}=bundleParts(input.bundle),moved=movedRestV1(rest,"neutral",host,policy),memo=rowMemoV1(),{keys:asked,consulted}=recordingKeysV1(keys),packer=createRowPackerV1();
  packer.add(neutralRowsV1(rows,host,asked,policy,memo));
  for(const layer of layers)packer.add(neutralRowsV1(layer,host,asked,policy,memo));
  const{recipe,value,sidecar,...others}=moved;
  return{recipeKey:closureKeyV1(recipe),consulted,document:{version:2,recipe,rest:others,rows:packer.finish([value??null,sidecar??null])}};
 }catch{return undefined;}
}

const CLOSURE_HEAD_V1=Buffer.from('{"version":2,"recipe":'),CLOSURE_HEAD_SCAN_V1=1<<20;
const headRecipeV1=(view:Buffer):unknown=>{
 if(view.length<CLOSURE_HEAD_V1.length+2||!view.subarray(0,CLOSURE_HEAD_V1.length).equals(CLOSURE_HEAD_V1))return undefined;
 const open=CLOSURE_HEAD_V1.length,end=Math.min(view.length,open+CLOSURE_HEAD_SCAN_V1);
 if(view[open]!==123)return undefined;
 let depth=0,quoted=false;
 for(let at=open;at<end;at++){
  const byte=view[at]!;
  if(quoted){if(byte===92)at++;else if(byte===34)quoted=false;continue;}
  if(byte===34)quoted=true;
  else if(byte===123||byte===91)depth++;
  else if(byte===125||byte===93){
   depth--;
   if(depth!==0)continue;
   if(view[at+1]!==44)return undefined;
   try{const recipe=JSON.parse(view.toString("utf8",open,at+1)) as unknown;return recipe!==null&&typeof recipe==="object"?recipe:undefined;}catch{return undefined;}
  }
 }
 return undefined;
};

/** 🔖️ The recipe of a stored closure document and whether it names the owner token, read from its first bytes: the packer writes the recipe (a few hundred characters) before the rows, so naming the keys of the documents of an entry parses none of their rows (the intake parsed every document twice, once for its recipe and once to localize it, 57 s of 192 s for one owner on a cold machine). The bytes are viewed, never copied; a document that does not start the way the packer writes it is read by the full parse; undefined when it is no closure document.
 * @see ./🧪️tests/🔖️document-head */
export function closureDocumentHeadV1(bytes:Uint8Array,policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipe:unknown;owned:boolean}>|undefined{
 const view=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength),recipe=headRecipeV1(view);
 if(recipe!==undefined)return{recipe,owned:view.includes(policy.ownerToken)};
 try{const full=JSON.parse(view.toString("utf8")) as{version?:unknown;recipe?:unknown}|null;return full!==null&&typeof full==="object"&&full.version===2&&full.recipe!==null&&typeof full.recipe==="object"?{recipe:full.recipe,owned:view.includes(policy.ownerToken)}:undefined;}catch{return undefined;}
}

/** 📖️ The neutral closure bundle of a stored form (rows and layers share their row objects); throws when the form does not hold together. */
export function unpackBundleV1(document:PackedClosureV1):ClosureBundleV1{
 if(document===null||typeof document!=="object"||document.version!==2||document.recipe===null||typeof document.recipe!=="object"||document.rest===null||typeof document.rest!=="object")throw new UnportableV1("closure form");
 const{lists,extras}=unpackRowListsV1(document.rows);
 if(lists.length<1||extras.length!==2)throw new UnportableV1("closure lists");
 return{...document.rest,recipe:document.recipe,value:extras[0],sidecar:extras[1],rows:lists[0]!,layers:lists.slice(1)} as unknown as ClosureBundleV1;
}
