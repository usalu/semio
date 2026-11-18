import{createHash}from"node:crypto";
import{mkdtempSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import type{GraphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{hostOfRootV1,type ObservationRowV1}from"../../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import type{SelectedSealedV1}from"../../🟦️.ts";

export const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");

/** 🌐️ The shared state of a simulated fleet cache: results with the first-bytes rule and the compare-and-swap of a head, blobs, claims with a lease. */
export function makeFleet(){
 const results=new Map<string,Uint8Array>(),blobs=new Map<string,Uint8Array>(),claims=new Map<string,{owner:string;until:number}>(),log:string[]=[];
 const state={results,blobs,claims,log,down:false,corruptBlobs:false,lastClaim:undefined as string|undefined};
 const client=(who:string):GraphFleetClientV1=>{
  const c={
   settings:{endpoint:"http://127.0.0.1:1",instance:"main",token:undefined,upload:true,stateDirectory:"/nonexistent"},
   usable:()=>!state.down,claimsUsable:()=>!state.down,
   getResult:async(key:string)=>{log.push(who+" get "+key.slice(0,8));return results.get(key)??"miss" as const;},
   getBlob:async(hash:string)=>{const bytes=blobs.get(hash);if(bytes===undefined)return undefined;if(state.corruptBlobs){const copy=Buffer.from(bytes);copy[0]=copy[0]!^1;return copy;}return bytes;},
   hasBlob:async(hash:string)=>blobs.has(hash),
   putBlob:async(hash:string,bytes:Uint8Array)=>{blobs.set(hash,bytes);return{stored:true as const};},
   putResult:async(key:string,bytes:Uint8Array,_slot:string,expect?:string)=>{
    const held=results.get(key);
    if(expect!==undefined){const have=held===undefined?"none":sha(held);if(have!==expect)return{stored:false as const,refusal:{code:"conflict"}} as never;}
    else if(held!==undefined&&sha(held)!==sha(bytes))return{stored:false as const,refusal:{code:"divergent"}} as never;
    results.set(key,bytes);log.push(who+" put "+key.slice(0,8));return{stored:true as const};
   },
   claim:async(key:string,owner:string,ttl:number)=>{
    if(results.has(key))return{state:"stored" as const};
    const held=claims.get(key),now=Date.now();
    if(held&&held.until>now&&held.owner!==owner)return{state:"held" as const,by:held.owner,untilMilliseconds:held.until};
    claims.set(key,{owner,until:now+ttl});state.lastClaim=owner;log.push(who+" claim "+key.slice(0,8));return{state:"granted" as const,untilMilliseconds:now+ttl};
   },
   renew:async(key:string,owner:string,ttl:number)=>{const held=claims.get(key);if(!held||held.owner!==owner||held.until<Date.now())return"lost" as const;claims.set(key,{owner,until:Date.now()+ttl});return"granted" as const;},
   release:async(key:string,owner:string)=>{if(claims.get(key)?.owner===owner)claims.delete(key);},
   await:async(key:string,timeout:number)=>{
    const end=Date.now()+timeout;
    for(;;){
     if(results.has(key))return"stored" as const;
     const held=claims.get(key);if(!held||held.until<Date.now())return"lapsed" as const;
     if(Date.now()>=end)return"timeout" as const;
     await new Promise(done=>setTimeout(done,8));
    }
   }
  };
  return c as unknown as GraphFleetClientV1;
 };
 return Object.assign(state,{client});
}
export type FleetV1=ReturnType<typeof makeFleet>;

/** 🗂️ The model of the owner records the fleet already shares: a record is keyed by the content of the files its inference read, computed once under a claim. */
export function makeRecords(){return{records:new Map<string,string>(),inferences:new Map<string,number>(),takes:{n:0}};}
export type RecordsV1=ReturnType<typeof makeRecords>;

export const ownerNameV1=(index:number)=>`o${index}`;
export const ownerOfV1=(relative:string)=>{const found=/^libs\/o(\d+)\//u.exec(relative);return found?ownerNameV1(Number(found[1])):undefined;};

/** 🖥️ One simulated machine: its own state root, home and host name, a tree of `count` owners (owner i reads its own file and the file of owner i-1), file stamps of its own, and counters for the work the exchange saves. */
export function makeMachine(input:Readonly<{name:string;count:number;records:RecordsV1;content?:(index:number)=>string;stamp?:number}>){
 const root=mkdtempSync(join(tmpdir(),"sel-"+input.name+"-")),home=mkdtempSync(join(tmpdir(),"home-"+input.name+"-")),host=hostOfRootV1(root);
 const tree=new Map<string,string>(),seeded=new Set<string>(),counters={observations:0,inferences:0,takes:0,reads:0},stamp=input.stamp??Math.floor(Math.random()*1e9);
 const file=(index:number)=>`libs/o${index}/src/a.ts`;
 for(let index=0;index<input.count;index++)tree.set(file(index),input.content?.(index)??`v0-${index}`);
 const rowOf=(index:number,path=file(index)):ObservationRowV1=>({kind:"file",path:`${root}/${path}`,fact:{sha256:sha(tree.get(path)??""),size:(tree.get(path)??"").length,mtimeMs:stamp+index,ctimeMs:stamp+index,ino:stamp*7+index}});
 const machine={name:input.name,root,home,host,tree,seeded,counters,graph:"",installed:false,file,
  edit(index:number,text:string){tree.set(file(index),text);},
  holds:async(rows:readonly ObservationRowV1[])=>rows.every(row=>{const relative=String(row.path).startsWith(root+"/")?String(row.path).slice(root.length+1):String(row.path);const fact=row.fact as{sha256:string}|undefined;return fact!==undefined&&tree.has(relative)&&sha(tree.get(relative)!)===fact.sha256;}),
  digestsOf:(rows:readonly ObservationRowV1[])=>rows,
  read:async(differing:readonly string[]|undefined):Promise<SelectedSealedV1>=>{
   counters.reads++;const rows:ObservationRowV1[]=[],admission:SelectedSealedV1["admission"][number][]=[];
   for(let index=0;index<input.count;index++){
    const name=ownerNameV1(index);
    if(!seeded.has(name)){counters.observations++;seeded.add(name);}
    admission.push({owner:name,suffix:`${name}\0[[],[]]`,facts:{closure:index,root},rows:[rowOf(index)]});
    const own=sha(tree.get(file(index))!),dep=index>0?sha(tree.get(file(index-1))!):"",key=sha(JSON.stringify([index,own,dep]));
    if(input.records.records.has(key))input.records.takes.n++,counters.takes++;
    else{input.records.records.set(key,name);counters.inferences++;input.records.inferences.set(name,(input.records.inferences.get(name)??0)+1);}
    rows.push(rowOf(index));if(index>0)rows.push(rowOf(index,file(index-1)));
   }
   const graph=JSON.stringify({owners:Array.from({length:input.count},(_,index)=>({name:ownerNameV1(index),root:`${root}/libs/o${index}`,digest:sha(tree.get(file(index))!)}))});
   machine.graph=graph;
   return{graph,rows,membership:sha(JSON.stringify([...tree.keys()].sort())),admission};
  },
  install:async(plan:Readonly<{fetched:{graph:string;admission:readonly {owner:string}[]};matching:readonly string[];differing:readonly string[];complete:boolean}>)=>{
   for(const name of plan.matching)seeded.add(name);
   if(plan.complete){machine.graph=plan.fetched.graph;machine.installed=true;}
  }
 };
 return machine;
}
export type MachineV1=ReturnType<typeof makeMachine>;
