import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{countEventV1,readEventCountsV1}from"../../../📈️counter-ledger/🟦️.ts";

/** 🧮️ What the remote cache looked like from this machine: one count file per kind (`ac`, `cas`, `nx`) and outcome, appended by every client of the machine through the one fetch that signs the requests, so a hit, miss, put, conflict or refusal is counted where it was seen and never lost on a restart. */
export type RemoteCacheKindV1="ac"|"cas"|"nx";
export type RemoteCacheOutcomeV1="hits"|"misses"|"puts"|"conflicts"|"refused";
export const REMOTE_CACHE_CLIENT_KINDS_V1:readonly RemoteCacheKindV1[]=["ac","cas","nx"];
export const REMOTE_CACHE_CLIENT_OUTCOMES_V1:readonly RemoteCacheOutcomeV1[]=["hits","misses","puts","conflicts","refused"];

/** 📁️ Where the client counts of a worktree live. */
export const remoteCacheClientDirectoryV1=(root:string):string=>machinePathV1(root,".cache","f","remote-cache-client");

/** 🔎️ Names what a request to the cache server was: Bazel's `/<instance>?/ac/<digest>` and `/cas/<digest>`, Nx's `/<instance>?/v1/cache/<hash>`; anything else (status, claims, slots, execution) is not a cache access. */
export function classifyRemoteCacheRequestV1(url:string,method:string,status:number):Readonly<{kind:RemoteCacheKindV1;outcome:RemoteCacheOutcomeV1}>|undefined{
 let segments:string[];try{segments=new URL(url).pathname.split("/").slice(1);}catch{return undefined;}
 let kind:RemoteCacheKindV1|undefined;
 for(let at=0;at<=Math.min(1,segments.length-1)&&kind===undefined;at++){
  const [head,second,third]=[segments[at],segments[at+1],segments[at+2]];
  if((head==="ac"||head==="cas")&&second!==undefined&&third===undefined&&segments.length===at+2)kind=head;
  else if(head==="v1"&&second==="cache"&&third!==undefined&&segments.length===at+3)kind="nx";
 }
 if(kind===undefined)return undefined;
 const read=method==="GET"||method==="HEAD",write=method==="PUT"||method==="POST";
 if(status===401||status===403||status===429)return {kind,outcome:"refused"};
 if(read&&status>=200&&status<300)return {kind,outcome:"hits"};
 if(read&&status===404)return {kind,outcome:"misses"};
 if(write&&status===409)return {kind,outcome:"conflicts"};
 if(write&&status>=200&&status<300)return {kind,outcome:"puts"};
 return undefined;
}

/** ➕️ Counts one answer of the cache server; never throws and never waits. */
export function countRemoteCacheClientV1(root:string|undefined,url:string,method:string,status:number):void{
 if(root===undefined)return;
 try{const found=classifyRemoteCacheRequestV1(url,method.toUpperCase(),status);if(found!==undefined)countEventV1(remoteCacheClientDirectoryV1(root),`${found.kind}-${found.outcome}`);}catch{}
}

/** 📖️ The client counts of this machine grouped the way the cache server's own ledger groups its counters (`ac.hits`, `cas.misses`, ...). */
export function readRemoteCacheClientGroupsV1(root:string):Readonly<Record<string,Readonly<Record<string,number>>>>{
 const names=REMOTE_CACHE_CLIENT_KINDS_V1.flatMap(kind=>REMOTE_CACHE_CLIENT_OUTCOMES_V1.map(outcome=>`${kind}-${outcome}`)),read=readEventCountsV1(remoteCacheClientDirectoryV1(root),names).counts;
 return Object.freeze(Object.fromEntries(REMOTE_CACHE_CLIENT_KINDS_V1.map(kind=>[kind,Object.freeze(Object.fromEntries(REMOTE_CACHE_CLIENT_OUTCOMES_V1.map(outcome=>[outcome,read[`${kind}-${outcome}`]??0])))])));
}
