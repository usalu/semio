import {readFleetExecutionV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../🟦️.ts";

/** 🎭️ The two roles the replica daemon supervises: the agent (build worker and load reporter) and the graph server of every machine and, on the group server, the scheduler and the cache. */
export type RemoteExecutionSidecarRoleV1="agent"|"scheduler"|"cache"|"graph"|"pool"|"ready";

/** 🧰️ What a sidecar needs: the worktree, the environment the replica daemon hands over (`SEMIO_FLEET_ORIGIN`, `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_STORE_ROOT`), cancellation and a log sink. */
export type RemoteExecutionSidecarContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;drain?:AbortSignal;write:(line:string)=>void;work?:SidecarWorkV1}>;

/** 🧵️ What a role that holds a long computation tells its restart and hears back: `busy` while it serves (the restart of a code change waits for the end of it), `restartPending` while the restart waits (the role takes no new work) and `running`, how many actions the agent serves for other agents, and `wake`, which the role calls when `busy` turns false so that the restart happens at once. */
export type SidecarWorkV1={busy:boolean;running?:number;restartPending:boolean;wake?:()=>void};

export const sleep=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise<void>(done=>{
 if(signal.aborted)return done();
 const timer=setTimeout(done,milliseconds);
 signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});
});

/** 🔌️ The ports of the scheduler: `SEMIO_REMOTE_EXECUTION_PORTS=<grpc>,<worker>` when set (tests and machines where the ports are taken), else the ports the topology row of the group declares (`declared`: what every machine of a fleet uses), else the defaults of the standalone `repo remote-execution serve` in the policy. */
export function remoteExecutionPortsV1(environment:Readonly<Record<string,string|undefined>>,declared?:Readonly<{scheduler:number;worker:number}>):Readonly<{grpc:number;worker:number}>{
 const server=readRemoteExecutionPolicyV1().server,named=environment.SEMIO_REMOTE_EXECUTION_PORTS?.split(",").map(Number);
 if(named!==undefined&&named.length===2&&named.every(port=>Number.isInteger(port)&&port>=0&&port<=65535))return {grpc:named[0]!,worker:named[1]!};
 return declared===undefined?{grpc:server.port,worker:server.workerPort}:{grpc:declared.scheduler,worker:declared.worker};
}

/** 🔗️ The worker API (HTTP) of the scheduler that a gRPC endpoint names: the same host on the policy's worker port. */
export function remoteExecutionWorkerApiV1(endpoint:string,workerPort:number):string{
 const url=new URL(endpoint.replace(/^grpc(s?):/u,"http$1:"));
 return `${url.protocol}//${url.hostname.includes(":")&&!url.hostname.startsWith("[")?`[${url.hostname}]`:url.hostname}:${workerPort}`;
}

/** 🧭️ Where the scheduler of the fleet is: the endpoint the scheduler publishes in the fleet facts (`execution.endpoint`: `http://<host>:<grpc port>`, the Remote Execution API over plain HTTP/2; its worker and load API is on the same host at the worker port), else the explicit environment variable. */
export function remoteExecutionSchedulerEndpointV1(context:Pick<RemoteExecutionSidecarContextV1,"root"|"environment">):string|undefined{
 const names=readRemoteExecutionPolicyV1().environment,environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root};
 const explicit=environment[names.endpoint];
 if(explicit!==undefined&&explicit!=="")return explicit;
 return readFleetExecutionV1(environment,{root:context.root})?.endpoint;
}
