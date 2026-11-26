import {loadFleetSignerV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {selectRemoteExecutionSandboxV1} from "../../🔒️sandbox/🟦️.ts";
import {remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyV1} from "../../🌡️load/🔎️why/🟦️.ts";
import type {RemoteExecutionPolicyV1,RemoteExecutionSandboxV1} from "../../🟦️.ts";

/** 🩺️ Whether this machine can run a build worker at all, before it tries: the policy runs actions, the machine has its signed worker key, and the confinement the policy requires exists. The first thing that is wrong is returned as the reason a worker is absent; `undefined` means the worker may start. Free disk space is not asked here: it changes within minutes on a busy machine, so the running worker asks it with every admission and takes fewer actions, or none, while the space is short (`💽️disk`). Pure apart from reading the key directory and the sandbox probe. */
export function checkRemoteExecutionAgentReadinessV1(input:Readonly<{root:string;policy:RemoteExecutionPolicyV1["worker"]}>):RemoteExecutionWorkerWhyV1|undefined{
 const {root,policy}=input;
 if(policy.actions==="none")return remoteExecutionWorkerWhyV1("policy-off");
 if(loadFleetSignerV1(root,"worker")===undefined)return remoteExecutionWorkerWhyV1("identity-unregistered","no worker key in .cache/f/identity");
 try{
  const sandbox=selectRemoteExecutionSandboxV1(policy.sandbox as "auto"|RemoteExecutionSandboxV1);
  if(policy.confinement==="required"&&sandbox.name==="isolated-root")return remoteExecutionWorkerWhyV1("sandbox-unavailable","only isolated-root");
 }catch(error){return remoteExecutionWorkerWhyV1("sandbox-unavailable",(error as Error).message);}
 return undefined;
}
