/** 🌉️ Public receiving-host contract of the adjacent authored JavaScript implementation. */
import type { FlowWasmExports, FlowRetainedGrant, FlowRetainedProgress, FlowHandle, FlowTask } from "../../🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts";
export interface FlowHostOptions {
  readonly exports: FlowWasmExports;
  readonly memory: WebAssembly.Memory;
  readonly schedule?: (step:()=>void)=>void;
  readonly now?: ()=>number;
  readonly maximumInFlight?: number;
  readonly retainedGrant?: FlowRetainedGrant;
}
export interface FlowSessionLifetime {
  readonly session: FlowHandle;
  close():Promise<void>;
  terminalIsEmpty():boolean;
  isClosing():boolean;
}
export interface FlowHost {
  readonly state: object;
  start(operation:number,args?:Readonly<Record<string,unknown>>,session?:FlowHandle):FlowTask<unknown>;
  cancel(requestId:bigint):boolean;
  sessionLifetime(handle:FlowHandle,requestId:bigint):FlowSessionLifetime;
  close():Promise<void>;
  stepProgress():FlowRetainedProgress;
  terminalIsEmpty():boolean;
}
export function createFlowHost(options:FlowHostOptions):FlowHost;
