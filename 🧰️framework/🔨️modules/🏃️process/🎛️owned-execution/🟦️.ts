import {captureOwnedCommandBodyV1,writeOwnedCommandOutputV1} from "./📥️capture/🟦️.ts";
export {withOwnedCommandChannelV1,readOwnedCommandChannelLineV1,writeOwnedCommandChannelV1,closeOwnedCommandChannelV1,readOwnedCommandChannelDiagnosticsV1,type OwnedCommandChannelV1,type OwnedCommandTerminalV1,writeOwnedCommandOutputV1,type OwnedCommandOutputSinkV1,captureOwnedCommandResultV1,readOwnedCommandCaptureV1,type OwnedCommandCaptureV1,type OwnedCommandCaptureOptionsV1,type OwnedCommandInputV1} from "./📥️capture/🟦️.ts";
import { terminateOwnedChildTree } from "../🪓️termination/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../🔒️leases/🟦️.ts";
import { StringDecoder } from "node:string_decoder";
import { spawn } from "node:child_process";
import { OwnedCommandFailure } from "./📪️outcome/🟦️.ts";
import stdoutDestination from "./🧬️schema/📤️stdout.json" with { type: "json" };
import outputSchema from "./🧬️schema/📬️output.json" with { type: "json" };
import processBudgetSchema from "../⏱️budget/🧬️schema/🔣️.json" with { type: "json" };
import {Transform} from "node:stream";
import{requireIssuedResourceControlV1}from"../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {validateJsonSchemaSubset} from "../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
export { OwnedCommandFailure, readOwnedCommandOutcome, type OwnedCommandOutcome } from "./📪️outcome/🟦️.ts";
/** 🌓️ Names the closed stdout and stderr roles declared by the owned output stream schema. */
export type OwnedCommandStreamV1="stdout"|"stderr";

/** 🎛️ Receives actual output, cancellation and awaited publication ports. */
export interface OwnedCommandOptions {
  readonly stdout?: "inherit" | "ignore" | "stderr";
  readonly outputStyle?: "stream" | "stream-without-prefixes" | "static";
  readonly control?: ResourceLeaseControl;
  readonly input?: string | Uint8Array;
  readonly output?: Readonly<{maximumBytes:number;maximumLines:number}>;
  readonly outputConsumer?: (bytes: Uint8Array, stream: OwnedCommandStreamV1) => unknown;
  readonly env?: Readonly<Record<string, string | undefined>>;
  readonly signal?: AbortSignal;
  readonly onLine?: (line: string,stream:OwnedCommandStreamV1) => void;
  readonly onProgress?: (line: string) => void | Promise<void>;
}


/** 🧺️ Runs one tool without blocking the event loop and without forwarding its output: several tools run at once (jco, wasm-opt, a descriptor emitter, a
 * module probe), so their output is kept as a 64 KiB tail and surfaced only in the failure. Cancellation terminates the tool's process tree, forcefully after 2 s.
 * Resolves with the output tail. */
export async function captureOwnedCommandV1(command: string, args: readonly string[], cwd: string, options: Readonly<{ env?: Readonly<Record<string, string | undefined>>; signal?: AbortSignal; control?: ResourceLeaseControl; output?: OwnedCommandOptions["output"] }> = {}): Promise<string> {
 const {result,tail}=await captureOwnedCommandBodyV1(command,args,cwd,options,false);
 if(result.status===0&&result.signal===null)return tail;
 throw new Error(`${command} ${args.join(" ")} exited with status ${result.status} signal ${result.signal}\n${tail}`);
}

/** ⏱️ Keeps long native queue waits observable until the owning operation completes. */
export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => console.log(line)): () => void {
  const started = Date.now(),
    progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}

/** ⌛️ Checks one monotonic command deadline in intervals within the native timer range. */
function startOwnedCommandTimeoutV1(milliseconds:number,expire:()=>void):()=>void {
  const started=performance.now();let active=milliseconds>0,timer:ReturnType<typeof setTimeout>|undefined;
  const check=():void=>{if(!active)return;const remaining=milliseconds-(performance.now()-started);if(remaining<=0){active=false;expire();}else timer=setTimeout(check,Math.min(processBudgetSchema.$defs.TimerMaximumMilliseconds.const,Math.ceil(remaining)));};
  if(active)check();
  return()=>{active=false;if(timer!==undefined)clearTimeout(timer);};
}

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. Its stdout (unless ignored) and stderr are piped and
 * forwarded, never inherited: an inherited pipe shares this Bun process's `O_NONBLOCK`, and a burst from the command then fails with
 * `EAGAIN` once a slow reader lets the pipe fill ([[cargoStreamingStatus]], ticket 26/09/23 W4). */
export async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs: number, options: OwnedCommandOptions = {}): Promise<void> {
  if (options.stdout !== undefined && !stdoutDestination.enum.includes(options.stdout)) throw new Error(`${label} has an invalid stdout destination`);
  if(options.output!==undefined&&validateJsonSchemaSubset(outputSchema,options.output).length)throw new Error(`${label} has invalid output limits`);
  if(options.outputConsumer!==undefined&&(typeof options.outputConsumer!=="function"||options.output===undefined))throw new Error(`${label} output consumer requires finite output limits`);
  if(options.outputStyle!==undefined&&!outputSchema.definitions.Style.enum.includes(options.outputStyle))throw Error(`${label} has an invalid output style`);
  if(options.outputStyle==="static"){if(!options.output||!options.control)throw Error(`${label} static output requires finite limits and actual control`);requireIssuedResourceControlV1(options.control);}
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  const child = spawn(command, args, { cwd, env: (options.env ?? process.env) as NodeJS.ProcessEnv, detached: process.platform !== "win32", stdio: [options.input === undefined ? "inherit" : "pipe", options.stdout === "ignore" && options.output === undefined && !options.outputConsumer ? "ignore" : "pipe", "pipe"], windowsHide: true });
  let stopped = "", closed = false, publicationRefused = false, publicationFailure: unknown, interruptPublication!: () => void;
  const pending = new Set<Promise<void>>(), interrupted = new Promise<void>(accept => { interruptPublication = accept; });
  const terminate = (reason: string): void => {
    stopped ||= reason;
    interruptPublication();
    if (!closed) terminateOwnedChildTree(child);
  };
  const abort = (): void => terminate("cancelled");
  const observe = (stream: NodeJS.ReadableStream,source:OwnedCommandStreamV1): void => {
    const decoder = new StringDecoder("utf8");
    let pending = "";
    stream.on("data", (bytes: Buffer) => {
      pending += decoder.write(bytes);
      let boundary: number;
      while ((boundary = pending.indexOf("\n")) >= 0) {
        options.onLine!(pending.slice(0, boundary).replace(/\r$/u, ""),source);
        pending = pending.slice(boundary + 1);
      }
    });
    stream.once("end", () => { pending += decoder.end(); if (pending) options.onLine!(pending,source); });
  };
  let deliveryFailed=false,deliveryFailure:unknown;const deliveryStreams=new Set<NodeJS.EventEmitter>(),deliveryRefused=(error:unknown):void=>{if(!deliveryFailed){deliveryFailed=true;deliveryFailure=error;}terminate("output delivery refused");},watchDelivery=(stream:NodeJS.EventEmitter):void=>{if(!deliveryStreams.has(stream)){deliveryStreams.add(stream);stream.on("error",deliveryRefused);}};
  let outputBytes=0,outputLines=0,outputRefused=false,consumerRefused=false,consumerFailure:unknown;const consumerWork:Promise<PromiseSettledResult<unknown>>[]=[];
  const buffered:{destination:NodeJS.WritableStream;text:string}[]=[];
  const forward=(stream:NodeJS.ReadableStream,destination:NodeJS.WritableStream|undefined,observed:boolean,source:OwnedCommandStreamV1):void=>{
    const limits=options.output;
    let tail=false;
    const admitted:NodeJS.ReadableStream=limits?new Transform({transform(bytes:Buffer,_encoding,done){
      if(outputRefused||consumerRefused){done();return;}
      if(bytes.length>limits.maximumBytes-outputBytes){outputRefused=true;terminate("output byte limit");done();return;}
      let lines=outputLines,open=tail;
      for(const byte of bytes){if(!open)lines++;open=byte!==10;if(lines>limits.maximumLines){outputRefused=true;terminate("output line limit");done();return;}}
      outputBytes+=bytes.length;outputLines=lines;tail=open;
      if(options.outputConsumer)try{const result=options.outputConsumer(bytes,source);if(result!==undefined){if(result&&typeof result==="object"&&typeof(result as PromiseLike<unknown>).then==="function")consumerWork.push(Promise.resolve(result).then(value=>({status:"fulfilled" as const,value}),reason=>({status:"rejected" as const,reason})));throw Error(`${label} output consumer must settle synchronously without a return value`);}}catch(error){consumerRefused=true;consumerFailure=error;terminate("output consumer refused");done();return;}done(null,bytes);
    }}):stream;
    if(destination&&options.outputStyle!=="static"){watchDelivery(destination);watchDelivery(admitted);}
    if(options.onLine&&observed)observe(admitted,source);
    if(destination&&options.outputStyle==="static"){const decoder=new StringDecoder("utf8");admitted.on("data",(bytes:Buffer)=>{const text=decoder.write(bytes);if(text)buffered.push({destination,text});});admitted.once("end",()=>{const text=decoder.end();if(text)buffered.push({destination,text});});admitted.resume();}
    else if(destination&&options.outputStyle==="stream"){const decoder=new StringDecoder("utf8"),prefix=`[${label}] `;let beginning=true;const render=(text:string)=>{if(!text)return"";const result=(beginning?prefix:"")+text.replace(/\n(?=[\s\S])/gu,"\n"+prefix);beginning=text.endsWith("\n");return result;},rendered=new Transform({transform(bytes:Buffer,_encoding,done){done(null,render(decoder.write(bytes)));},flush(done){const text=render(decoder.end());if(text)this.push(text);done();}});watchDelivery(rendered);admitted.pipe(rendered).pipe(destination,{end:false});}
    else if(destination)admitted.pipe(destination,{end:false});else (admitted as Transform).resume();
    if(limits)stream.pipe(admitted as Transform);
  };
  if(child.stdout)forward(child.stdout,options.stdout==="ignore"?undefined:options.stdout==="stderr"?process.stderr:process.stdout,options.stdout!=="ignore","stdout");
  forward(child.stderr!,process.stderr,true,"stderr");
  options.signal?.addEventListener("abort", abort, { once: true });
  if (options.signal?.aborted) abort();
  const interrupt = (): void => terminate("SIGINT");
  const stop = (): void => terminate("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", stop);
  const publish = (line: string): void => {
    const publication = Promise.resolve().then(() => options.onProgress ? options.onProgress(line) : console.log(line));
    pending.add(publication);
    publication.then(() => { pending.delete(publication); }, error => {
      publicationRefused = true;
      publicationFailure = error;
      pending.delete(publication);
      terminate("progress publication refused");
    });
  };
  const stopProgress = startNativeProgress(label, 10_000, publish);
  const stopTimeout = startOwnedCommandTimeoutV1(timeoutMs,() => terminate(`timeout ${timeoutMs}ms`));
  try {
    const inputPublication = options.input === undefined || !child.stdin ? Promise.resolve() : new Promise<void>(accept => {
      child.stdin!.once("finish", accept); child.stdin!.once("close", accept);
      child.stdin!.once("error", () => { if (!closed && !stopped) terminate("input publication refused"); accept(); });
      child.stdin!.end(options.input);
    });
    const status = await new Promise<{code:number|null;signal:string|null}>((accept) => {
      child.once("error", error => console.error(error.message));
      child.once("close", (code, signal) => { closed = true; accept({code,signal}); });
    });
    await inputPublication;
    stopProgress();
    if(consumerRefused)await Promise.allSettled([...pending]);else await Promise.race([Promise.all([...pending]), interrupted]);
    if(options.outputStyle==="static")try{for(const row of buffered)await Promise.race([writeOwnedCommandOutputV1(row.destination,row.text,options.control!),interrupted.then(()=>{throw Error(`${label} output delivery stopped: ${stopped}`);})]);}catch(error){deliveryFailed=true;deliveryFailure=error;}
    try{await Promise.all([process.stdout,process.stderr].map(output=>new Promise<void>((accept,reject)=>output.write("",error=>error?reject(error):accept()))));}catch(error){if(!deliveryFailed){deliveryFailed=true;deliveryFailure=error;}}
    if(consumerRefused){const returned=await Promise.all(consumerWork),failures=[consumerFailure,...returned.flatMap(row=>row.status==="rejected"?[row.reason]:[]),...(publicationRefused?[publicationFailure]:[])];if(failures.length>1)throw new AggregateError(failures,`${label} output consumer and accepted work failed`);throw consumerFailure;}
    if (publicationRefused) throw publicationFailure;
    if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
    if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
    if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
    if(deliveryFailed)throw deliveryFailure;
  } finally {
    for(const stream of deliveryStreams)stream.off("error",deliveryRefused);
    options.signal?.removeEventListener("abort", abort);
    stopProgress();
    stopTimeout();
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}
