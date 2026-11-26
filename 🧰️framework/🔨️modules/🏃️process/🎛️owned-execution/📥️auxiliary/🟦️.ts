import { EventEmitter } from "node:events";
import { closeSync, write as writeDescriptor } from "node:fs";
import { Readable, Writable } from "node:stream";
import{types}from"node:util";
import { terminateOwnedChildTree } from "../../🪓️termination/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
/** 📥️ Carries finite immutable UTF-8 DATA on the owned auxiliary channel while preserving ordinary stdin. */
export type OwnedCommandAuxiliaryInputV1=Readonly<{version:1;text:string}>;
/** 🔏️ Snapshots own primitive DATA before process birth and refuses malformed or excessive UTF-8. */
export function readOwnedCommandAuxiliaryInputV1(value:unknown):OwnedCommandAuxiliaryInputV1{
 if(!value||typeof value!=="object"||types.isProxy(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))throw Error("Owned auxiliary DATA required");
 const keys=Reflect.ownKeys(value),version=Object.getOwnPropertyDescriptor(value,"version"),text=Object.getOwnPropertyDescriptor(value,"text");if(keys.length!==2||!keys.includes("version")||!keys.includes("text")||!version||!("value"in version)||!version.enumerable||version.value!==1||!text||!("value"in text)||!text.enumerable||typeof text.value!=="string")throw Error("Closed own auxiliary DATA required");
 const content=text.value,maximum=schema.definitions.Policy.const.maximumBytes;if(content.length>maximum||Buffer.byteLength(content,"utf8")>maximum||Buffer.from(content,"utf8").toString("utf8")!==content)throw Error("Bounded canonical UTF-8 auxiliary DATA required");return Object.freeze({version:1,text:content});
}

/** 🔌️ Receives the writable auxiliary descriptor without exposing a tool-specific stream type. */
export interface OwnedAuxiliaryPipeV1{once(event:"error"|"close",listener:(error?:unknown)=>void):unknown;end(text:string,encoding:"utf8",callback:(error?:Error|null)=>void):unknown}
/** 🧭️ The auxiliary writer boundary that refused publication. */
export type OwnedAuxiliaryPublicationPhaseV1="pipe-missing"|"error-event"|"close-event"|"end-callback"|"end-call";
/** 🧾️ Closed transport context independent of the original thrown value. */
export type OwnedAuxiliaryPublicationContextV1=Readonly<{operation:"auxiliary-publication";descriptor:number;phase:OwnedAuxiliaryPublicationPhaseV1;code:string|null;syscall:string|null}>;

/** 🔎️ Reads only own data without invoking a failure object accessor. */
function failureData(value:unknown,key:string):string|null{
 if(!value||(typeof value!=="object"&&typeof value!=="function")||types.isProxy(value))return null;
 try{const field=Object.getOwnPropertyDescriptor(value,key);return field&&"value"in field&&typeof field.value==="string"?field.value:null;}catch{return null;}
}

/** 🚨️ Names the failed descriptor and writer phase while retaining its exact primary cause. */
export class OwnedAuxiliaryPublicationFailureV1 extends Error{
 readonly cause:unknown;
 readonly context:OwnedAuxiliaryPublicationContextV1;
 readonly code:string|null;
 readonly syscall:string|null;
 constructor(cause:unknown,phase:OwnedAuxiliaryPublicationPhaseV1){
  const descriptor=schema.definitions.Policy.const.descriptor,code=failureData(cause,"code"),syscall=failureData(cause,"syscall"),detail=failureData(cause,"message")??(typeof cause==="string"?cause:null);
  super(`Owned auxiliary publication failed: descriptor=${descriptor} phase=${phase} code=${code??"unknown"} syscall=${syscall??"unknown"}${detail?": "+detail.slice(0,512):""}`);
  this.name="OwnedAuxiliaryPublicationFailureV1";this.cause=cause;this.context=Object.freeze({operation:"auxiliary-publication",descriptor,phase,code,syscall});this.code=code;this.syscall=syscall;
 }
}

/** 📤️ Retains first publication failure independently from completion, including an explicit undefined primary. */
export function publishOwnedAuxiliaryInputV1(pipe:OwnedAuxiliaryPipeV1|null|undefined,text:string,refuse:(error:unknown,phase:OwnedAuxiliaryPublicationPhaseV1)=>void):Promise<void>{return new Promise<void>(accept=>{let settled=false,failed=false;const succeed=()=>{if(!settled){settled=true;accept();}},fail=(error:unknown,phase:OwnedAuxiliaryPublicationPhaseV1)=>{if(!failed){failed=true;refuse(error,phase);}succeed();};if(!pipe){fail(Error("Owned auxiliary pipe required"),"pipe-missing");return;}pipe.once("error",error=>fail(error,"error-event"));pipe.once("close",()=>{if(!settled)fail(Error("Owned auxiliary pipe closed before publication"),"close-event");});try{pipe.end(text,"utf8",error=>{if(error!==undefined&&error!==null)fail(error,"end-callback");else succeed();});}catch(error){fail(error,"end-call");}});}

/** 🔌️ Writes Bun's caller-owned POSIX descriptor with an explicit loop over partial and would-block writes: `Bun.write` on a socket descriptor resent the whole chunk after a partial write, so a reader that started late saw a truncated copy followed by a second copy and refused it as a changed snapshot. */
function ownedBunAuxiliaryWriter(descriptor: number): Writable {
  let closed = false;
  let writing: Promise<void> | undefined;
  const close = () => {
    if (closed) return;
    closed = true;
    closeSync(descriptor);
  };
  return new Writable({
    write(chunk: Buffer, _encoding, callback) {
      writing = (async () => {
        let offset = 0;
        while (offset < chunk.length) {
          try {
            offset += await new Promise<number>((resolve, reject) => writeDescriptor(descriptor, chunk, offset, chunk.length - offset, null, (error, written) => error ? reject(error) : resolve(written)));
          } catch (error) {
            if ((error as NodeJS.ErrnoException).code !== "EAGAIN") throw error;
            await new Promise<void>(resolve => setTimeout(resolve, 1));
          }
        }
      })();
      void writing.then(() => callback(), callback);
    },
    final(callback) {
      try { close(); callback(); }
      catch (error) { callback(error as Error); }
    },
    destroy(error, callback) {
      void Promise.resolve(writing).catch(() => {}).then(close).then(() => callback(error), failure => callback(error ?? failure));
    },
  });
}

/** 🪢️ Retains the native Bun child's existing stream and settlement interface around its owned input channels. */
export function spawnOwnedBunCommandV1(
  command: string,
  args: readonly string[],
  cwd: string,
  env: Readonly<Record<string, string | undefined>>,
  streams: Readonly<{ input: "inherit" | Uint8Array; output: boolean; auxiliary?: true }>,
) {
  const events = new EventEmitter();
  const child = Bun.spawn([command, ...args], {
    cwd,
    env,
    detached: true,
    stdio: [streams.input, streams.output ? "pipe" : "ignore", "pipe", ...(streams.auxiliary ? ["socket-fd" as const] : [])],
    onExit(_child, code, signal, error) {
      queueMicrotask(async () => {
        const reason = signal === null ? null : String(signal);
        if (error) events.emit("error", error);
        events.emit("exit", code, reason);
        await drained;
        events.emit("close", code, reason);
      });
    },
  });
  const descriptor = streams.auxiliary ? child.stdio[schema.definitions.Policy.const.descriptor] : undefined;
  if (streams.auxiliary && typeof descriptor !== "number") {
    terminateOwnedChildTree({ pid: child.pid, exitCode: child.exitCode, signalCode: child.signalCode === null ? null : String(child.signalCode) });
    throw Error("Owned auxiliary descriptor missing");
  }
  const auxiliary = typeof descriptor === "number" ? ownedBunAuxiliaryWriter(descriptor) : undefined;
  const source = child.stdout ? Readable.fromWeb(child.stdout as unknown as Parameters<typeof Readable.fromWeb>[0]) : null;
  const diagnostic = Readable.fromWeb(child.stderr as unknown as Parameters<typeof Readable.fromWeb>[0]);
  const drained = Promise.all([source, diagnostic].map(stream => stream && new Promise<void>(resolve => stream.once("close", resolve))));
  return {
    pid: child.pid,
    get exitCode() { return child.exitCode; },
    get signalCode() { return child.signalCode === null ? null : String(child.signalCode); },
    once: events.once.bind(events),
    stdin: null,
    stdout: source,
    stderr: diagnostic,
    auxiliary,
  };
}
