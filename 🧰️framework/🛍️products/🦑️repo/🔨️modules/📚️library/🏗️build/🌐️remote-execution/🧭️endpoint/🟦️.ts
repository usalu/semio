import {fleetCacheRootV1,readFleetExecutionV1,readLocalFleetCacheFactsV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {remoteExecutionPortsV1,remoteExecutionWorkerApiV1} from "../🛰️sidecar/🧰️shared/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;

/** 🧭️ Where the scheduler of the fleet is and who said so: `environment` (the explicit variable) or `fleet-facts` (the endpoint `🛰️sidecar/🗓️scheduler` publishes in the fleet `cache.json`; `http://<host>:<grpc port>`, the Remote Execution API over plain HTTP/2). `protocolVersion` is the published one, `null` for an explicit variable; `factsGeneration` is the generation of the fleet facts of this machine (`null` without any). */
export type RemoteExecutionEndpointV1=Readonly<{endpoint:string;endpointFrom:"environment"|"fleet-facts";protocolVersion:number|null;factsGeneration:number|null}>;

/** 🗓️ The worker and load API (HTTP) of the scheduler: `environment` (the explicit variable) or `derived` (the endpoint host on the declared worker port). */
export type RemoteExecutionSchedulerV1=Readonly<{url:string;schedulerFrom:"environment"|"derived"}>;

const filled=(environment:Environment,name:string):string|undefined=>{const value=environment[name];return value===undefined||value===""?undefined:value;};

/** 🧭️ THE resolver of the remote execution endpoint, used by `status`, `fleet status`, the rustc offload and the Bazel flags: the explicit variable first, then the fleet facts of the local copy of `root` (default: `SEMIO_WORKSPACE_ROOT`, so a process outside a build never reads the facts of the machine it happens to run on), else `undefined`. Synchronous, reads one small file, never touches the network. */
export function resolveRemoteExecutionEndpointV1(input:Readonly<{environment:Environment;root?:string}>):RemoteExecutionEndpointV1|undefined{
 const root=input.root??fleetCacheRootV1(input.environment),copy=root===undefined?undefined:readLocalFleetCacheFactsV1(root),factsGeneration=copy?.facts.generation??null,explicit=filled(input.environment,readRemoteExecutionPolicyV1().environment.endpoint);
 if(explicit!==undefined)return {endpoint:explicit,endpointFrom:"environment",protocolVersion:null,factsGeneration};
 const published=readFleetExecutionV1(input.environment,copy===undefined||root===undefined?{}:{root,copy});
 return published===undefined?undefined:{endpoint:published.endpoint,endpointFrom:"fleet-facts",protocolVersion:published.protocolVersion,factsGeneration};
}

/** 🗓️ The scheduler API of a resolved endpoint: the explicit variable, else the endpoint host on the worker port that the environment (`SEMIO_REMOTE_EXECUTION_PORTS`), the topology row of the group or the policy declares; `undefined` when the endpoint is not a URL. */
export async function resolveRemoteExecutionSchedulerV1(input:Readonly<{environment:Environment;root?:string}>,endpoint:string):Promise<RemoteExecutionSchedulerV1|undefined>{
 const explicit=filled(input.environment,readRemoteExecutionPolicyV1().environment.scheduler);
 if(explicit!==undefined)return {url:explicit,schedulerFrom:"environment"};
 const root=input.root??fleetCacheRootV1(input.environment);
 try{
  const {fleetGroupViewV1}=await import("../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts"),view=root===undefined?undefined:await fleetGroupViewV1(root,input.environment);
  return {url:remoteExecutionWorkerApiV1(endpoint,remoteExecutionPortsV1(input.environment,view?.ports).worker),schedulerFrom:"derived"};
 }catch{return undefined;}
}

/** 🔎️ Why no scheduler endpoint resolves, one line in English and German (the detail of a `no-endpoint` decline): no checkout known, no readable copy of the fleet facts, no scheduler published, or no address of the scheduler origin answered. Synchronous, reads the local copy only. */
export function explainRemoteExecutionEndpointAbsenceV1(input:Readonly<{environment:Environment;root?:string}>):string{
 const root=input.root??fleetCacheRootV1(input.environment);
 if(root===undefined)return "no fleet checkout is known (SEMIO_WORKSPACE_ROOT) / kein Flotten-Checkout bekannt (SEMIO_WORKSPACE_ROOT)";
 const copy=readLocalFleetCacheFactsV1(root);
 if(copy===undefined)return "no readable copy of the fleet facts (.cache/f/cache-facts.json) / keine lesbare Kopie der Flottenfakten (.cache/f/cache-facts.json)";
 const execution=copy.facts.execution;
 if(execution===undefined)return "the fleet facts publish no scheduler / die Flottenfakten veröffentlichen keinen Scheduler";
 return `no address of ${execution.origin} answers on port ${execution.port} / keine Adresse von ${execution.origin} antwortet am Port ${execution.port}`;
}
