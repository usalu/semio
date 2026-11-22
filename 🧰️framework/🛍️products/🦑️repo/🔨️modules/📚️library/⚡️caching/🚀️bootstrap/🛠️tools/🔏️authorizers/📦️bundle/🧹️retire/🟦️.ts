import {lstatSync,readdirSync,rmSync} from "node:fs";
import {join} from "node:path";
import {nxAuthorizerBundleDirectoryV1,nxAuthorizerBundlePolicyV1 as policy} from "../🟦️.ts";
import {withResourceLeases,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🗂️ One bundle as the census sees it: its key, when it was born or last marked used and whether a live reader holds it. */
export type NxAuthorizerBundleRowV1=Readonly<{identity:string;touchedAt:number;held:boolean}>;
/** 🧮️ Names the bundles retirement may remove, least recently used first: everything behind the newest ones that no reader holds and that was not used within the minimum age. */
export function planNxAuthorizerBundleRetirementV1(rows:readonly NxAuthorizerBundleRowV1[],nowMs:number,keepNewest:number=policy.keepNewest,minimumAgeMilliseconds:number=policy.minimumAgeMilliseconds):readonly string[]{
 const newestFirst=[...rows].sort((left,right)=>right.touchedAt-left.touchedAt||(left.identity<right.identity?-1:left.identity>right.identity?1:0));
 return newestFirst.slice(keepNewest).filter(row=>!row.held&&nowMs-row.touchedAt>=minimumAgeMilliseconds).map(row=>row.identity).reverse();
}
/** 🧹️ Removes released old bundles of one tools image: each candidate needs the exclusive reader lease without waiting, so a bundle any running Nx process, daemon or provisioner holds stays. */
export async function retireNxAuthorizerBundlesV1(toolsRoot:string,leaseDirectory:string,control?:ResourceLeaseControl,options:Readonly<{nowMs?:number;keepNewest?:number;minimumAgeMilliseconds?:number;signal?:AbortSignal}>={}):Promise<readonly string[]>{
 let names:string[];try{names=readdirSync(join(toolsRoot,policy.directory));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return Object.freeze([]);throw error;}
 const hex=names.filter(name=>/^[a-f0-9]{64}$/u.test(name));if(hex.length>policy.maximumCandidates)throw Error("Bounded original authorizer bundle census required");
 const rows=hex.flatMap(identity=>{try{const row=lstatSync(nxAuthorizerBundleDirectoryV1(toolsRoot,identity));return row.isDirectory()?[{identity,touchedAt:Math.max(0,row.birthtimeMs>0?row.birthtimeMs:0,row.mtimeMs),held:false}]:[];}catch{return [];}});
 const signal=control?.signal??options.signal??new AbortController().signal,retired:string[]=[];
 for(const identity of planNxAuthorizerBundleRetirementV1(rows,options.nowMs??Date.now(),options.keepNewest,options.minimumAgeMilliseconds)){
  signal.throwIfAborted();
  try{await withResourceLeases({directory:leaseDirectory,resources:[{resource:policy.lease+identity,mode:"exclusive"}],signal,timeoutMs:0,...(control?{control}:{})},async()=>{const directory=nxAuthorizerBundleDirectoryV1(toolsRoot,identity),row=lstatSync(directory);if(!row.isDirectory()||row.isSymbolicLink())return;rmSync(directory,{recursive:true,force:true});retired.push(identity);});}
  catch(error){signal.throwIfAborted();if((error as Error).message!=="Resource lease timed out: "+policy.lease+identity)throw error;}
 }
 return Object.freeze(retired);
}
