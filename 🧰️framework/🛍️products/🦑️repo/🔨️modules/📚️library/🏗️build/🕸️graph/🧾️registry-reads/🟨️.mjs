import {createHash} from "node:crypto";
import {readFileSync,statSync} from "node:fs";
import {createRequire} from "node:module";
import {join} from "node:path";
import {observeGlobalFactV1,registerGlobalFactV1} from "../../👁️observation/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const POLICY=schema.definitions.Policy.const,WORKSPACE_ROOT=".",REGISTRY_FILE="📋️build-owners.json";
const handlers=new Map(),indexes=new WeakMap(),disk=new Map();

/** 🧾️ The name under which what a read asked of the owner registry travels as a `global` row. */
export const OWNER_REGISTRY_FACT_V1=POLICY.name;

const sha=value=>createHash("sha256").update(JSON.stringify(value)).digest("hex");
const points=text=>Array.from(text,char=>char.codePointAt(0));
const byCodePoint=(left,right)=>{const a=points(left),b=points(right);for(let at=0;at<a.length&&at<b.length;at++)if(a[at]!==b[at])return a[at]<b[at]?-1:1;return a.length-b.length;};
const sorted=values=>[...values].sort(byCodePoint);
const owned=(path,root)=>root===WORKSPACE_ROOT||path===root||path.startsWith(root+"/");

/** 🔑️ The digest of one owner row: its name, root, manifest, packages and native roots, the order of the lists never counting. */
export function ownerRowDigestV1(owner){return sha([owner.name,owner.root,owner.manifest,sorted(owner.packages),sorted(owner.nativeRoots??[])]);}

/** 🗝️ The digest of a whole registry: the row digests by name and the catalog providers. */
export function ownerRegistryDigestV1(registry){
 const rows=sorted(registry.owners.map(owner=>owner.name)),digests=new Map(registry.owners.map(owner=>[owner.name,ownerRowDigestV1(owner)])),providers=registry.catalogProviders.map(provider=>[provider.ownerRoot,provider.sourcePath,sorted(provider.consumerRoots)]).sort((a,b)=>byCodePoint(a[0],b[0])||byCodePoint(a[1],b[1]));
 return sha([rows.map(name=>[name,digests.get(name)]),providers]);
}

function indexOf(registry){
 let known=indexes.get(registry);
 if(known===undefined){
  const byName=new Map(registry.owners.map(owner=>[owner.name,owner])),byPackage=new Map(),byRoot=new Map(),within=new Map();
  for(const owner of registry.owners){for(const name of owner.packages)byPackage.set(name,owner.name);for(const root of[owner.root,...owner.nativeRoots??[]])byRoot.set(root,owner.name);if(owner.root!==WORKSPACE_ROOT){const prior=within.get(owner.root);if(prior===undefined||byCodePoint(owner.name,prior)<0)within.set(owner.root,owner.name);}}
  known={byName,byPackage,byRoot,within};indexes.set(registry,known);
 }
 return known;
}

const manifestOf=root=>root===WORKSPACE_ROOT?"Cargo.toml":root+"/Cargo.toml";
const core={
 row:(registry,[name])=>{const owner=indexOf(registry).byName.get(name);return owner===undefined?null:ownerRowDigestV1(owner);},
 package:(registry,[name])=>indexOf(registry).byPackage.get(name)??null,
 root:(registry,[root])=>indexOf(registry).byRoot.get(root)??null,
 within:(registry,[directory])=>{const{within}=indexOf(registry);for(let end=directory.length;end>0;end=directory.lastIndexOf("/",end-1)){const hit=within.get(end===directory.length?directory:directory.slice(0,end));if(hit!==undefined)return hit;}return null;},
 providers:(registry,[ownerRoot,hasTargets,inputNames])=>sha(registry.catalogProviders.filter(provider=>hasTargets&&provider.consumerRoots.some(root=>owned(ownerRoot,root))||inputNames.includes("{workspaceRoot}/"+provider.sourcePath)).map(provider=>[provider.ownerRoot,provider.sourcePath])),
 candidates:registry=>sha(sorted(new Set([...registry.owners.flatMap(owner=>[...owner.manifest.endsWith("Cargo.toml")?[owner.manifest]:[],...(owner.nativeRoots??[]).map(manifestOf)]),...registry.owners.map(owner=>manifestOf(owner.root))]))),
 digest:registry=>ownerRegistryDigestV1(registry),
};

const digests=new WeakMap();

/** 🗝️ The digest of `registry` computed once per registry object; null for no registry. */
export function ownerRegistryDigestOnceV1(registry){
 if(registry===null||registry===undefined)return null;
 let known=digests.get(registry);
 if(known===undefined){known=ownerRegistryDigestV1(registry);digests.set(registry,known);}
 return known;
}

/** 📖️ The admitted registry of the checkout as it is now (read again only when the file changed). */
export function currentOwnerRegistryV1(workspaceRoot){
 return createRequire(import.meta.url)("../🚪️admission/🟦️.ts").readBuildOwnerRegistryV1(currentRegistryV1(workspaceRoot));
}

/** 🧩️ Registers the answer of one more question kind: `answer(registry, arguments, workspaceRoot)` is a JSON value that depends on the registry (and files the answer reads itself), and is what a row of this kind holds while it stays equal. */
export function registerOwnerRegistryQuestionV1(kind,answer){handlers.set(kind,answer);}

/** ❓️ What `registry` answers to one question, `segments` = [kind, ...arguments]. */
export function answerOwnerRegistryQuestionV1(registry,segments,workspaceRoot=""){
 const[kind,...rest]=segments,answer=handlers.get(kind)??core[kind];
 if(answer===undefined)throw Error("Known owner registry question required: "+kind);
 return answer(registry,rest,workspaceRoot);
}

/** 🖊️ Asks `registry` one question and records the question and its answer as a row of the running observation. */
export function askOwnerRegistryV1(workspaceRoot,registry,segments){
 const answer=answerOwnerRegistryQuestionV1(registry,segments,workspaceRoot);observeGlobalFactV1(OWNER_REGISTRY_FACT_V1,workspaceRoot,segments,answer);return answer;
}

/** ✍️ Records an answer the caller already holds as a row of the running observation. */
export function observeOwnerRegistryAnswerV1(workspaceRoot,segments,answer){observeGlobalFactV1(OWNER_REGISTRY_FACT_V1,workspaceRoot,segments,answer);}

/** 🔭️ The lookups of the owner discovery over `registry`, each answer recorded as a question of the running observation. */
export function observedOwnerRegistryViewV1(workspaceRoot,registry){
 const ask=(...segments)=>askOwnerRegistryV1(workspaceRoot,registry,segments);
 return Object.freeze({
  byName:Object.freeze({has:name=>ask("row",name)!==null}),
  byPackage:Object.freeze({get:name=>ask("package",name)??undefined}),
  byRoot:Object.freeze({get:root=>ask("root",root)??undefined,get size(){return indexOf(registry).byRoot.size;}}),
  relativeProject:path=>{const cut=path.lastIndexOf("/");return ask("within",cut<0?WORKSPACE_ROOT:path.slice(0,cut))??undefined;},
 });
}

function currentRegistryV1(workspaceRoot){
 const file=join(workspaceRoot,REGISTRY_FILE),status=statSync(file,{bigint:true}),stamp=status.size+":"+status.mtimeNs+":"+status.ino,held=disk.get(workspaceRoot);
 if(held?.stamp===stamp)return held.registry;
 const registry=JSON.parse(readFileSync(file,"utf8"));disk.set(workspaceRoot,{stamp,registry});return registry;
}

registerGlobalFactV1(OWNER_REGISTRY_FACT_V1,(workspaceRoot,segments,fact)=>{
 if(!Array.isArray(segments)||typeof segments[0]!=="string")return false;
 return JSON.stringify(answerOwnerRegistryQuestionV1(currentRegistryV1(workspaceRoot),segments,workspaceRoot))===JSON.stringify(fact);
});
