import {lstatSync,readFileSync,readdirSync,utimesSync} from "node:fs";
import {basename,dirname,isAbsolute,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {nxAuthorizerBundleIdentityV1} from "../../📦️sdk/🪪️identity/🟦️.ts";
import {nxAuthorizerDigestManifestPolicyV1,nxAuthorizerReceiptHoldsV1,type NxAuthorizerReceiptV1} from "../📤️receipt/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📐️ Names where compiled authorizer bundles live below the tools image and how long and how many of them retirement keeps. @see ./🧬️schema/🔣️.json */
export const nxAuthorizerBundlePolicyV1=schema.$defs.Policy.const;
/** 🏷️ Binds one bundle directory to its own key and to the dependency generation it runs beside. */
export interface NxAuthorizerBundleMarkerV1{readonly version:1;readonly digest:string;readonly deps:string;readonly authorizers:NxAuthorizerReceiptV1}
const markerPlan=compileJsonSchemaSubset({$ref:"#/$defs/Marker"},schema);
const readExact=(directory:string,name:string,maximumBytes:number):Buffer=>{
 const path=join(directory,name),before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(maximumBytes))throw Error("Bounded original authorizer bundle file required");
 const bytes=readFileSync(path),after=lstatSync(path,{bigint:true});
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Authorizer bundle file changed while read");
 return bytes;
};
/** 📍️ The directory of one bundle below a tools image root. */
export const nxAuthorizerBundleDirectoryV1=(toolsRoot:string,identity:string):string=>join(toolsRoot,nxAuthorizerBundlePolicyV1.directory,identity);
/** 📜️ Reads the closed marker of one bundle directory whose name must be the key it declares. */
export function readNxAuthorizerBundleMarkerV1(directory:string):NxAuthorizerBundleMarkerV1{
 const marker=JSON.parse(readExact(directory,nxAuthorizerBundlePolicyV1.marker,nxAuthorizerBundlePolicyV1.maximumMarkerBytes).toString("utf8"));
 if(markerPlan.validate(marker).length||basename(directory)!==marker.digest)throw Error("Exact authorizer bundle marker required");
 return marker;
}
/** ✅️ Holds only when the marker, the key recomputed from both manifests, the receipt and the compiled bytes all agree and the bundle belongs to the named dependency generation. */
export function nxAuthorizerBundleHoldsV1(directory:string,dependencies:string):NxAuthorizerBundleMarkerV1|null{
 try{
  const marker=readNxAuthorizerBundleMarkerV1(directory);if(marker.deps!==dependencies)return null;
  const files=new Map(nxAuthorizerBundlePolicyV1.manifests.map(name=>[name,readExact(directory,name,nxAuthorizerDigestManifestPolicyV1.maximumBytes)] as const));
  if(nxAuthorizerBundleIdentityV1(marker.deps,files)!==marker.digest)return null;
  return nxAuthorizerReceiptHoldsV1(directory,marker.authorizers)?marker:null;
 }catch{return null;}
}
/** 🧭️ Derives from a loaded bundle directory the dependency generation it runs beside: the lease identity and the toolkit root every later use hangs on. */
export function nxAuthorizerBundleToolkitV1(bundleRoot:string):Readonly<{authorizer:string;identity:string;toolkitRoot:string;bundle:string}>{
 const root=resolve(bundleRoot),parent=dirname(root);
 if(!isAbsolute(root)||basename(parent)!==nxAuthorizerBundlePolicyV1.directory)throw Error("Exact loaded authorizer bundle directory required");
 const marker=readNxAuthorizerBundleMarkerV1(root);
 return Object.freeze({authorizer:marker.digest,identity:marker.deps,toolkitRoot:join(dirname(parent),"nx",marker.deps),bundle:join(root,nxAuthorizerBundlePolicyV1.bundle)});
}
/** 🖐️ Marks one bundle as just used, so retirement keeps it for its minimum age; a volume that refuses the mark only loses that protection. */
export function touchNxAuthorizerBundleV1(directory:string):void{try{const now=new Date();utimesSync(directory,now,now);}catch{}}
/** 🔎️ The most recently used bundle that holds beside one dependency generation, for harnesses that start the actual toolkit without having provisioned it. */
export function newestNxAuthorizerBundleV1(toolsRoot:string,dependencies:string):string|undefined{
 let names:string[];try{names=readdirSync(join(toolsRoot,nxAuthorizerBundlePolicyV1.directory));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
 const born=(directory:string)=>{const row=lstatSync(directory);return Math.max(row.birthtimeMs>0?row.birthtimeMs:0,row.mtimeMs);};
 const rows=names.filter(name=>/^[a-f0-9]{64}$/u.test(name)).slice(0,nxAuthorizerBundlePolicyV1.maximumCandidates).map(name=>nxAuthorizerBundleDirectoryV1(toolsRoot,name)).filter(directory=>nxAuthorizerBundleHoldsV1(directory,dependencies)!==null).map(directory=>({directory,at:born(directory)}));
 return rows.sort((left,right)=>right.at-left.at||(left.directory<right.directory?-1:1))[0]?.directory;
}

/** 🔄️ A bundle that holds beside the same dependency generation and was used or created after the named one: the landing a running daemon born with the named bundle has not applied. Only candidates newer than the named bundle are verified, so the check of an unchanged tools image costs one directory listing and one stat per bundle. */
export function supersedingNxAuthorizerBundleV1(toolsRoot:string,dependencies:string,current:string):string|undefined{
 let names:string[];try{names=readdirSync(join(toolsRoot,nxAuthorizerBundlePolicyV1.directory));}catch{return undefined;}
 const used=(directory:string)=>{const row=lstatSync(directory);return Math.max(row.birthtimeMs>0?row.birthtimeMs:0,row.mtimeMs);};
 let since:number;try{since=used(nxAuthorizerBundleDirectoryV1(toolsRoot,current));}catch{return undefined;}
 const rows:{directory:string;at:number}[]=[];
 for(const name of names.filter(value=>value!==current&&/^[a-f0-9]{64}$/u.test(value)).slice(0,nxAuthorizerBundlePolicyV1.maximumCandidates)){const directory=nxAuthorizerBundleDirectoryV1(toolsRoot,name);try{const at=used(directory);if(at>since)rows.push({directory,at});}catch{}}
 return rows.sort((left,right)=>right.at-left.at||(left.directory<right.directory?-1:1)).find(row=>nxAuthorizerBundleHoldsV1(row.directory,dependencies)!==null)?.directory;
}
