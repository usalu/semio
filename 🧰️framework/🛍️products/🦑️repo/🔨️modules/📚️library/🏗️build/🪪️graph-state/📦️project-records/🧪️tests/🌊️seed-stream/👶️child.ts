import{mkdirSync}from"node:fs";
import{join}from"node:path";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{createProjectFleetPortV1}from"../../🟦️.ts";
import{bindClosureStoreV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";

/** 👶️ The taker of the seed-stream law: a cold machine in its own process that takes every owner named in `owners` from the fleet cache and reports the peak resident set of the process, how many blobs it downloaded and how many closure entries it stored. */
const{root,owners}=JSON.parse(process.argv[2]!) as {root:string;owners:string[]};
const environment={...process.env};
mkdirSync(join(root,"ce"),{recursive:true});bindClosureStoreV1(root,join(root,"ce"),true);
const real=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
let blobs=0;
const client={...real,async getBlob(hash:string,size:number){blobs++;return real.getBlob(hash,size);}};
const port=createProjectFleetPortV1({root,environment,client,owner:"taker:1"})!;
const out:{name:string;seeded:boolean;closures:number;blobs:number}[]=[];
for(const name of owners){
 const before=port.stats().closures,blobsBefore=blobs;
 const gate=await port.gate({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,name]),exportRecord:()=>undefined,async import(){return true;},closures:closureTravelV1});
 out.push({name,seeded:gate.seeded,closures:port.stats().closures-before,blobs:blobs-blobsBefore});
}
console.log("RESULT "+JSON.stringify({out,peakMegabytes:Math.round(process.resourceUsage().maxRSS*1024/1048576)}));
process.exit(0);
