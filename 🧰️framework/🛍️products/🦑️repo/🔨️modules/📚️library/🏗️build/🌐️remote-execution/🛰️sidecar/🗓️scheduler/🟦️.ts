import {hostname,networkInterfaces} from "node:os";
import {dirname,join} from "node:path";
import {fleetGroupViewV1,fleetNetworkV1,setFleetExecutionV1} from "../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {readLocalFleetCacheFactsV1,remoteCacheFetchV1,resolveRemoteCacheV1,fleetStoreRootV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {createFleetRegistryV1,createFleetVerifierV1,fleetIdentityDirectoryV1,fleetInterfaceAddressV1,readFleetMachineV1,syncFleetIdentityV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {collectFleetAddressesV1,orderFleetAddressesV1} from "../../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts";
import {serverNetworksV1} from "../../../../../🤝️shared-worktree/🗺️topology/🟦️.ts";
import {createFleetGateV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {startRemoteExecutionServerV1,type RemoteExecutionServerV1} from "../../🖥️server/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";
import {readSchedulerJournalV1,writeSchedulerJournalV1} from "../../🗓️scheduler/📓️journal/🟦️.ts";
import {type RemoteExecutionSidecarContextV1,sleep,remoteExecutionPortsV1} from "../🧰️shared/🟦️.ts";

/** 🗓️ The scheduler of the group (run on the group server only): serves the Remote Execution API and the worker and load API on the ports of the topology row of the group with signature-checked requests (no shared secret): the origins of its own group may submit and work, the origins of the peer groups of the fleet may submit only, and the network is an access condition only when the topology says `access.enforceNetwork`, bound to the fleet interface only, publishes its origin and port in the fleet facts, republishes it while it runs and leaves when the fleet credential rotates (the supervisor restarts it with the new one). */
export async function runScheduler(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const policy=readRemoteExecutionPolicyV1(),environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},say=context.write,server=policy.server;
 const resolution=resolveRemoteCacheV1(environment,{root:context.root}),machine=readFleetMachineV1(context.root);
 if(resolution===undefined||machine===undefined)throw Error("no fleet facts for the scheduler yet");
 await syncFleetIdentityV1({root:context.root});
 const cacheUrl=resolution.contract.endpoint,storeRoot=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root),view=await fleetGroupViewV1(context.root,context.environment),declared=await fleetNetworkV1(context.root,context.environment),ports=remoteExecutionPortsV1(context.environment,view?.ports);
 const hostOf=(endpoint:string):string=>{try{return new URL(endpoint).hostname;}catch{return"";}},published=[hostOf(cacheUrl),...(readLocalFleetCacheFactsV1(context.root)?.candidates.map(hostOf)??[])];
 const interfaceRows=()=>collectFleetAddressesV1(networkInterfaces()).filter(row=>row.family==="IPv4"),bound=declared===undefined?undefined:fleetInterfaceAddressV1(declared,undefined,published),networks:readonly string[]=bound!==undefined&&declared!==undefined?[declared]:serverNetworksV1({platform:"macos",protocol:"nfs"}),address=bound??published.find(host=>interfaceRows().some(row=>row.address===host))??orderFleetAddressesV1(interfaceRows(),[])[0]?.address;
 if(storeRoot===undefined||address===undefined)throw Error("the scheduler needs the group store and an interface address of this machine");
 const pinned=Object.fromEntries(Object.entries(view?.links??{}).filter(([,link])=>link.serverKey!==undefined).map(([peer,link])=>[`${peer}-server`,link.serverKey!])),registry=createFleetRegistryV1({storeRoot,group:machine.group,peerStores:view?.peerStores??{},pinned,pinsPath:join(fleetIdentityDirectoryV1(context.root),"pins-execution.json")}),verifier=createFleetVerifierV1({registry,audience:"execution",adminOrigin:`${machine.group}-server`}),workerToken=context.environment[policy.environment.workerToken];
 await registry.refresh(true);
 const gate=createFleetGateV1({verifier,...(view?.enforceNetwork===true?{networks}:{}),home:machine.group,tokens:{worker:workerToken}});
 const shown=(()=>{try{return new URL(cacheUrl).hostname;}catch{return hostname();}})();
 const record=context.environment.SEMIO_FLEET_SIDECAR_RECORD,journalPath=join(record===undefined||record===""?join(context.root,".cache","f","sidecars"):dirname(record),"build-scheduler.operations.json");
 let flushing:ReturnType<typeof setTimeout>|undefined,service:RemoteExecutionServerV1|undefined,closed=false;
 const flush=():void=>{if(flushing!==undefined)clearTimeout(flushing);flushing=undefined;if(service!==undefined&&!closed)writeSchedulerJournalV1(journalPath,service.scheduler.journal());};
 const changed=():void=>{if(flushing===undefined){flushing=setTimeout(flush,250);flushing.unref();}};
 const restore=readSchedulerJournalV1(journalPath,Date.now(),server.resumeGraceSeconds*10_000);
 const running:RemoteExecutionServerV1=await startRemoteExecutionServerV1({
  host:address,grpcPort:ports.grpc,workerPort:ports.worker,content:httpRemoteExecutionContentV1({endpoint:cacheUrl,fetch:remoteCacheFetchV1(environment,{root:context.root})}),cacheUrl,tokens:{},gate,maximumWorkers:server.maximumWorkers,maximumRequestBytes:server.maximumRequestBytes,advertise:shown,
  maximumMessageBytes:server.maximumMessageBytes,maximumBatchBytes:server.maximumBatchBytes,
  scheduler:{unmatchedSeconds:server.unmatchedSeconds,lingerSeconds:server.lingerSeconds,leaseSeconds:server.leaseSeconds,maximumAttempts:server.maximumAttempts,defaultActionSeconds:server.defaultActionSeconds,maximumActionSeconds:server.maximumActionSeconds,maximumQueued:server.maximumQueued,heartbeatSeconds:policy.worker.heartbeatSeconds,pollSeconds:policy.worker.pollSeconds,loadExpireSeconds:server.loadExpireSeconds,refusalCooldownSeconds:server.refusalCooldownSeconds,resumeGraceSeconds:server.resumeGraceSeconds,restore,onChange:changed,language:"none"},
  onEvent:event=>{if(event.stage!=="submitted"&&event.stage!=="completed"&&event.stage!=="assigned")say(`scheduler ${event.stage}: ${event.detail}`);},
 });
 service=running;changed();
 const endpoint=`http://${shown}:${new URL(running.grpcUrl.replace(/^grpc/u,"http")).port}`;
 say(`serving ${endpoint}, worker api ${running.workerUrl}`);
 const publish=async():Promise<void>=>{if(storeRoot!==undefined)await setFleetExecutionV1(storeRoot,{origin:machine.originId,port:Number(new URL(running.grpcUrl.replace(/^grpc/u,"http")).port),protocolVersion:1}).catch(error=>{say(`publishing the endpoint failed: ${(error as Error).message}`);});};
 await publish();
 let lastSummary="";
 try{
  while(!context.signal.aborted){
   await sleep(60_000,context.signal);
   if(context.signal.aborted)break;
   const again=resolveRemoteCacheV1(environment,{root:context.root});
   if(again!==undefined&&again.contract.endpoint!==cacheUrl){say("the cache moved, restarting");break;}
   if(bound===undefined&&!interfaceRows().some(row=>row.address===address)){say("the interface address of the scheduler went away, restarting");break;}
   await publish();
   const counters=running.scheduler.status().counters,summary=`restored ${counters.restored}, resumed ${counters.resumed}, cancelled ${counters.cancelled} (before start ${counters.cancelledQueued}, running ${counters.cancelledExecuting}), workers re-registered ${counters.workersReused}, sweep stalls ${counters.sweepStalls}, queue wait p50/p90/max ${counters.queueWaitP50Milliseconds}/${counters.queueWaitP90Milliseconds}/${counters.queueWaitMaxMilliseconds} ms over ${counters.queueWaitSamples} assignments`;
   if(summary!==lastSummary){lastSummary=summary;say(`scheduler summary: ${summary}`);}
  }
 }finally{
  flush();closed=true;
  await running.stop().catch(()=>{});
  if(context.signal.aborted&&storeRoot!==undefined)await setFleetExecutionV1(storeRoot,null).catch(()=>{});
 }
}
