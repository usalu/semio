import{createHash}from"node:crypto";
import{realpathSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{expandRootV1,neutralizeRootV1}from"../../🛰️exchange/🟦️.ts";

/** 🧳️ Which observations of a record name the producer's machine or its way of spelling a path: the kinds of the location rows, the dependency generation directory and its token, the kinds whose path or fact is a path, the tokens of the root and of the analysis provider owner. See `./🧬️schema/🔣️.json`. */
export const portableRowsPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type PortableRowsPolicyV1=typeof portableRowsPolicyV1;
export type ObservationRowV1=Readonly<{kind:string;path:string;fact?:unknown}>;
/** 🪟️ The checkout of one machine and how it spells paths: a Windows root starts with a drive letter or is a UNC share. */
export type HostV1=Readonly<{root:string;windows:boolean}>;
export type PortableRecordV1=Readonly<{digest:string;authority:readonly ObservationRowV1[];observations:readonly ObservationRowV1[];image:string}>;
export type ClosureBundleV1=Readonly<{recipe:unknown;rows:readonly ObservationRowV1[];layers:readonly (readonly ObservationRowV1[])[]}>&Readonly<Record<string,unknown>>;
export type KeyMapV1=ReadonlyMap<string,string>;

/** 🚫️ A record or text that names the root in a spelling that cannot be mapped: it is neither published nor taken, the project is inferred locally. */
export class UnportableV1 extends Error{constructor(reason:string){super(reason);this.name="UnportableV1";}}

const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),slashed=(text:string)=>text.replaceAll("\\","/");
const generationPattern=(policy:PortableRowsPolicyV1)=>new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])([0-9a-f]{16,64})(?=[\\\\/]|$)`,"gu");
const mapStrings=(value:unknown,change:(text:string)=>string):unknown=>typeof value==="string"?change(value):Array.isArray(value)?value.map(item=>mapStrings(item,change)):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,mapStrings(item,change)])):value;
const rewrite=(row:ObservationRowV1,change:(text:string)=>string):ObservationRowV1=>({...row,path:change(row.path),...row.fact===undefined?{}:{fact:mapStrings(row.fact,change)}});
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
const withoutLocationRowsV1=(rows:readonly ObservationRowV1[],root:string,policy:PortableRowsPolicyV1):readonly ObservationRowV1[]=>rows.filter(row=>!(policy.locationKinds.includes(row.kind)&&row.fact==="directory"&&aboveRootV1(row.path,root)));
function generationRowsV1(kept:readonly ObservationRowV1[],policy:PortableRowsPolicyV1):readonly ObservationRowV1[]{
 const pattern=generationPattern(policy),ids=new Set<string>();
 for(const row of kept){for(const match of JSON.stringify([row.path,row.fact??null]).matchAll(pattern))ids.add(match[2]!);}
 if(ids.size!==1)return kept;
 const[id]=[...ids];
 return kept.map(row=>rewrite(row,text=>text.replaceAll(new RegExp(`(${escape(policy.dependencyRoot)}[\\\\/])${id}(?=[\\\\/]|$)`,"gu"),`$1${policy.token}`)));
}

/** 📥️ The observations of a portable record for this machine: the token becomes the machine's own dependency generation, or a name no directory has when it has none, so the project is inferred here. */
export function localRowsV1(rows:readonly ObservationRowV1[],generation:string|undefined,policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const put=generation??policy.unavailable;
 return rows.map(row=>JSON.stringify([row.path,row.fact??null]).includes(policy.token)?rewrite(row,text=>text.replaceAll(policy.token,put)):row);
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

type Direction=Readonly<{host:HostV1;keys:KeyMapV1;owner:(text:string)=>string;path:(text:string)=>string;text:(text:string)=>string}>;
const mapPathJson=(text:string,change:(item:unknown,index:number)=>unknown):string=>{
 try{const parsed=JSON.parse(text);return Array.isArray(parsed)?JSON.stringify(parsed.map(change)):text;}catch{return text;}
};
function convertRow(row:ObservationRowV1,way:Direction,policy:PortableRowsPolicyV1):ObservationRowV1{
 const{kind}=row;
 if(policy.jsonKinds.includes(kind)){
  const path=mapPathJson(row.path,(item,index)=>index===0&&typeof item==="string"?way.path(item):index===1&&kind==="closure"&&typeof item==="string"?way.keys.get(item)??item:index===0?item:mapStrings(item,way.text));
  const fact=kind==="closure"&&typeof row.fact==="string"?way.text(row.fact):policy.factPathKinds.includes(kind)&&typeof row.fact==="string"?way.path(row.fact):row.fact;
  return{...row,path,...fact===undefined?{}:{fact}};
 }
 if(kind==="analysis"&&row.fact!==null&&typeof row.fact==="object"){
  const fact=row.fact as Record<string,unknown>,nested=Array.isArray(fact.nestedObservations)?(fact.nestedObservations as ObservationRowV1[]).map(item=>convertRow(item,way,policy)):fact.nestedObservations;
  return{...row,path:way.path(row.path),fact:{...fact,...typeof fact.provider==="string"?{provider:way.owner(fact.provider)}:{},...nested===undefined?{}:{nestedObservations:nested}}};
 }
 if(policy.pathKinds.includes(kind))return{...row,path:way.path(row.path),...policy.factPathKinds.includes(kind)&&typeof row.fact==="string"?{fact:way.path(row.fact)}:{}};
 return{...row,path:way.text(row.path),...typeof row.fact==="string"?{fact:way.text(row.fact)}:{}};
}
const neutralWay=(host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1):Direction=>({host,keys,owner:text=>neutralOwner(text,policy),path:text=>neutralPathV1(text,host,policy),text:text=>neutralTextV1(text,host,policy)});
const localWay=(host:HostV1,keys:KeyMapV1,providers:readonly string[],policy:PortableRowsPolicyV1):Direction=>({host,keys,owner:localOwner(providers,policy),path:text=>localPathV1(text,host,policy),text:text=>localTextV1(text,host,policy)});

/** 🌍️ The rows of a record in the neutral form of the host's spelling (see {@link neutralRecordV1}): the location rows and the generation as {@link portableRowsV1}, then paths, provider owners and closure keys. */
export function neutralRowsV1(rows:readonly ObservationRowV1[],host:HostV1,keys:KeyMapV1=new Map(),policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const way=neutralWay(host,keys,policy);
 return generationRowsV1(withoutLocationRowsV1(rows,host.root,policy).map(row=>convertRow(row,way,policy)),policy);
}

/** 🏠️ The rows of a neutral record for the host: paths spelled the host's way, the provider owners the host registered and its own closure keys; the generation is put in by {@link localRowsV1} through `generation`. */
export function localizeRowsV1(rows:readonly ObservationRowV1[],host:HostV1,input:Readonly<{keys?:KeyMapV1;providers?:readonly string[];generation?:string|undefined}>={},policy:PortableRowsPolicyV1=portableRowsPolicyV1):readonly ObservationRowV1[]{
 const way=localWay(host,input.keys??new Map(),input.providers??[],policy);
 return localRowsV1(rows.map(row=>convertRow(row,way,policy)),input.generation,policy);
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
  const{rows,layers,rest}=bundleParts(input.bundle),moved=JSON.parse(neutralTextV1(JSON.stringify(rest),host,policy)) as Record<string,unknown>;
  return{recipeKey:closureKeyV1(moved.recipe),bundle:{...moved,rows:neutralRowsV1(rows,host,keys,policy),layers:layers.map(layer=>neutralRowsV1(layer,host,keys,policy))} as unknown as ClosureBundleV1};
 }catch{return undefined;}
}

/** 🚛️ A neutral closure bundle for the host, named by the key of its recipe in the host's spelling; undefined when it cannot be mapped. */
export function localBundleV1(input:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>,host:HostV1,keys:KeyMapV1,context:Readonly<{providers?:readonly string[];generation?:string|undefined}>={},policy:PortableRowsPolicyV1=portableRowsPolicyV1):Readonly<{recipeKey:string;bundle:ClosureBundleV1}>|undefined{
 try{
  const{rows,layers,rest}=bundleParts(input.bundle),moved=JSON.parse(localTextV1(JSON.stringify(rest),host,policy)) as Record<string,unknown>;
  return{recipeKey:closureKeyV1(moved.recipe),bundle:{...moved,rows:localizeRowsV1(rows,host,{...context,keys},policy),layers:layers.map(layer=>localizeRowsV1(layer,host,{...context,keys},policy))} as unknown as ClosureBundleV1};
 }catch{return undefined;}
}

/** 🗝️ The map from the neutral key of every recipe in the table of a neutral image to the key of the same recipe in the host's spelling. */
export const localKeysV1=(image:string,host:HostV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):KeyMapV1=>{try{return moveImage(image,text=>localTextV1(text,host,policy),false).keys;}catch{return new Map();}};
