import {hostname,networkInterfaces} from "node:os";
import {join} from "node:path";
import {fleetCacheEndpointsV1,fleetGroupViewV1,fleetNetworkV1,publishFleetCacheFactsV1} from "../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {createFleetPeerReadThroughV1,directPeerTransportV1,fleetPeerEndpointsV1,relayedPeerTransportV1,servePeerExchangeV1,FLEET_HOP_HEADER_V1,type FleetPeerTransportV1} from "../../../🌐️remote-cache/🧭️peers/🟦️.ts";
import {readLocalFleetCacheFactsV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {createSealedFetchV1} from "../../../🌐️remote-cache/🔐️sealed/🟦️.ts";
import {fleetInterfaceAddressV1,fleetSignedFetchV1,loadFleetSignerV1,readFleetMachineV1,readFleetOriginKeyV1,syncFleetIdentityV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {readRemoteCachePolicyV1} from "../../../🌐️remote-cache/🟦️.ts";
import {startRemoteCacheServerV1} from "../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {type RemoteExecutionSidecarContextV1,sleep} from "../🧰️shared/🟦️.ts";

/** 🗄️ The cache server of the group (run on the group server only, supervised by the replica daemon): the current code of this worktree, its index in the local state `.cache/f/cache-server` (blobs in the host store), its own port (`ports.cache` of the topology row), bound to the fleet interface, trusting only the origins registered in the stores of the fleet (its own group, and read-only the peer groups of the topology), by signature and never by address (the network of the group is an access condition only when the topology says `access.enforceNetwork`). On a miss it reads through to the cache servers of the peer groups (direct or relayed as each link says) and keeps what it got; it answers the relayed requests of the peers. Its port, the hosts it advertises to other networks and its peers come from the topology row of the group. It is the only publisher of the fleet facts, and it publishes only because it holds the admin signing key of the group server origin, which exists on that machine's disk alone; a restart of the sidecar (new code) needs no owner action. */
export async function runCache(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const all=readRemoteCachePolicyV1(),policy=all.server,environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},say=context.write,machine=readFleetMachineV1(context.root);
 if(machine===undefined||machine.storeRoot===undefined||machine.originId!==`${machine.group}-server`)throw Error("the cache sidecar runs on the group server only");
 await syncFleetIdentityV1({root:context.root});
 const admin=loadFleetSignerV1(context.root,"admin");
 if(admin===undefined)throw Error("the admin signing key of the group server is missing");
 const view=await fleetGroupViewV1(context.root,context.environment),network=await fleetNetworkV1(context.root,context.environment),previous=readLocalFleetCacheFactsV1(context.root)?.facts.cache.endpoints.map(endpoint=>{try{return new URL(endpoint).hostname;}catch{return"";}})??[];
 let address=network===undefined?undefined:fleetInterfaceAddressV1(network,undefined,previous),declared=network;
 if(address===undefined){
  const found=Object.values(networkInterfaces()).flatMap(rows=>rows??[]).find(row=>row.family==="IPv4"&&!row.internal&&/^(10\.|192\.168\.|172\.(1[6-9]|2[0-9]|3[01])\.)/u.test(row.address));
  if(found!==undefined){address=found.address;declared=`${found.address.split(".").slice(0,3).join(".")}.0/24`;say(`the declared fleet network does not match; using ${address} and ${declared}`);}
 }
 if(address===undefined||declared===undefined)throw Error("no address of this machine inside the fleet network (SEMIO_FLEET_NETWORK)");
 const port=Number(environment.SEMIO_REMOTE_CACHE_FLEET_PORT??view?.ports?.cache);
 if(!Number.isInteger(port)||port<1||port>65535)throw Error("the cache port is a fact of the topology (ports.cache of the group row) or SEMIO_REMOTE_CACHE_FLEET_PORT");
 const peerStores=view?.peerStores??{},inGroup=view?.inGroup??"sealed",transports:FleetPeerTransportV1[]=[];
 const signedRead=fleetSignedFetchV1({root:context.root,audience:"cache",roleOf:()=>"cache-read",...(inGroup==="sealed"?{fetch:createSealedFetchV1({group:machine.group,serverKey:()=>readFleetOriginKeyV1([machine.storeRoot!],`${machine.group}-server`)})}:{})});
 const sealedPeer=(peer:string,pinned:string|undefined)=>fleetSignedFetchV1({root:context.root,audience:"cache",roleOf:()=>"cache-read",fetch:createSealedFetchV1({group:peer,serverKey:async()=>pinned??await readFleetOriginKeyV1([peerStores[peer]!,machine.storeRoot!],`${peer}-server`),maximumBytes:policy.maximumEntryBytes})});
 for(const peer of Object.keys(peerStores)){
  const link=view?.links[peer];
  transports.push(link?.transport==="relayed"?relayedPeerTransportV1({group:peer,ownGroup:machine.group,ownStore:machine.storeRoot,peerStore:peerStores[peer]!,maximumBytes:policy.maximumEntryBytes}):directPeerTransportV1({group:peer,endpoints:()=>fleetPeerEndpointsV1(peerStores[peer],link?.endpoints),fetch:sealedPeer(peer,link?.serverKey),maximumBytes:policy.maximumEntryBytes}));
 }
 const pinned=Object.fromEntries(Object.entries(view?.links??{}).filter(([,link])=>link.serverKey!==undefined).map(([peer,link])=>[`${peer}-server`,link.serverKey!]));
 const directory=join(context.root,policy.fleetDirectory),{createCasStoreV1,casRootV1}=await import("../../../🗄️cas/🗃️store/🟦️.ts");
 const server=await startRemoteCacheServerV1({hostStore:createCasStoreV1({root:casRootV1(environment)}),directory,host:address,port,maximumBytes:policy.maximumBytes,maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(policy.maximumBytes/2)),maximumAgeDays:policy.maximumAgeDays,graceMilliseconds:policy.graceSeconds*1000,defaultInstance:all.defaults.instance,tokens:{},validation:policy.actionResults as "complete",maximumActionResultBytes:policy.maximumActionResultBytes,maintenanceSeconds:policy.maintenanceSeconds,mirror:{directory:join(directory,"mirror")},fleet:{storeRoot:machine.storeRoot,group:machine.group,root:context.root,inGroup,...(view?.enforceNetwork===true?{networks:[declared]}:{}),peerStores,pinned},peers:createFleetPeerReadThroughV1({transports}),signal:context.signal});
 const leaving=new AbortController(),exchange=servePeerExchangeV1({ownGroup:machine.group,ownStore:machine.storeRoot,peers:peerStores,pollMilliseconds:1000,signal:AbortSignal.any([context.signal,leaving.signal]),answer:async path=>{const response=await signedRead(server.url+path,{headers:{[FLEET_HOP_HEADER_V1]:"1"}});return response.status===200?{status:200,bytes:new Uint8Array(await response.arrayBuffer())}:(await response.body?.cancel(),{status:response.status});}});
 say(`serving the cache on ${server.url}, index ${directory}`);
 const publish=async():Promise<void>=>{
  try{
   const published=await publishFleetCacheFactsV1({admin,storeRoot:machine.storeRoot!,directory,group:machine.group,instance:all.defaults.instance,port:server.port,transport:inGroup,endpoints:fleetCacheEndpointsV1(server.port,[`http://${address}:${server.port}`,...(view?.advertise??[]).map(host=>`http://${host}:${server.port}`)],[]).filter(endpoint=>endpoint.startsWith(`http://${address}:`)||(view?.advertise??[]).some(host=>endpoint.startsWith(`http://${host}:`)))});
   if(published.changed)say(`fleet facts published: generation ${published.facts.generation}`);
  }catch(error){say(`publishing the fleet facts failed: ${(error as Error).message}`);}
 };
 await publish();
 try{while(!context.signal.aborted){await sleep(60_000,context.signal);if(!context.signal.aborted)await publish();}}finally{leaving.abort();await server.stop().catch(()=>{});await exchange.catch(()=>{});}
}
