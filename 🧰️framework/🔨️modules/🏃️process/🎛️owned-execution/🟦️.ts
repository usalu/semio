import { terminateOwnedChildTree } from "../🪓️termination/🟦️.ts";
import { StringDecoder } from "node:string_decoder";
import { spawn } from "node:child_process";
import { OwnedCommandFailure } from "./📪️outcome/🟦️.ts";
import stdoutDestination from "./🧬️schema/📤️stdout.json";
import outputSchema from "./🧬️schema/📬️output.json";
import {Transform} from "node:stream";
import {validateJsonSchemaSubset} from "../../🧬️schema/✅️validator/🟦️.ts";
export { OwnedCommandFailure, readOwnedCommandOutcome, type OwnedCommandOutcome } from "./📪️outcome/🟦️.ts";

/** 🎛️ Receives actual output, cancellation and awaited publication ports. */
export interface OwnedCommandOptions {
  readonly stdout?: "inherit" | "ignore" | "stderr";
  readonly output?: Readonly<{maximumBytes:number;maximumLines:number}>;
  readonly env?: Readonly<Record<string, string | undefined>>;
  readonly signal?: AbortSignal;
  readonly onLine?: (line: string) => void;
  readonly onProgress?: (line: string) => void | Promise<void>;
}


/** ⏱️ Keeps long native queue waits observable until the owning operation completes. */
export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => console.log(line)): () => void {
  const started = Date.now(),
    progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. Its stdout (unless ignored) and stderr are piped and
 * forwarded, never inherited: an inherited pipe shares this Bun process's `O_NONBLOCK`, and a burst from the command then fails with
 * `EAGAIN` once a slow reader lets the pipe fill ([[cargoStreamingStatus]], ticket 26/09/23 W4). */
export async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs: number, options: OwnedCommandOptions = {}): Promise<void> {
  if (options.stdout !== undefined && !stdoutDestination.enum.includes(options.stdout)) throw new Error(`${label} has an invalid stdout destination`);
  if(options.output!==undefined&&validateJsonSchemaSubset(outputSchema,options.output).length)throw new Error(`${label} has invalid output limits`);
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  const child = spawn(command, args, { cwd, env: options.env ?? process.env, detached: process.platform !== "win32", stdio: ["inherit", options.stdout === "ignore" && options.output === undefined ? "ignore" : "pipe", "pipe"], windowsHide: true });
  let stopped = "", closed = false, publicationRefused = false, publicationFailure: unknown, interruptPublication!: () => void;
  const pending = new Set<Promise<void>>(), interrupted = new Promise<void>(accept => { interruptPublication = accept; });
  const terminate = (reason: string): void => {
    stopped ||= reason;
    interruptPublication();
    if (!closed) terminateOwnedChildTree(child);
  };
  const abort = (): void => terminate("cancelled");
  const observe = (stream: NodeJS.ReadableStream): void => {
    const decoder = new StringDecoder("utf8");
    let pending = "";
    stream.on("data", (bytes: Buffer) => {
      pending += decoder.write(bytes);
      let boundary: number;
      while ((boundary = pending.indexOf("\n")) >= 0) {
        options.onLine!(pending.slice(0, boundary).replace(/\r$/u, ""));
        pending = pending.slice(boundary + 1);
      }
    });
    stream.once("end", () => { pending += decoder.end(); if (pending) options.onLine!(pending); });
  };
  let outputBytes=0,outputLines=0,outputRefused=false;
  const forward=(stream:NodeJS.ReadableStream,destination:NodeJS.WritableStream|undefined,observed:boolean):void=>{
    const limits=options.output;
    let tail=false;
    const admitted=limits?new Transform({transform(bytes:Buffer,_encoding,done){
      if(outputRefused){done();return;}
      if(bytes.length>limits.maximumBytes-outputBytes){outputRefused=true;terminate("output byte limit");done();return;}
      let lines=outputLines,open=tail;
      for(const byte of bytes){if(!open)lines++;open=byte!==10;if(lines>limits.maximumLines){outputRefused=true;terminate("output line limit");done();return;}}
      outputBytes+=bytes.length;outputLines=lines;tail=open;done(null,bytes);
    }}):stream;
    if(options.onLine&&observed)observe(admitted);
    if(destination)admitted.pipe(destination,{end:false});else (admitted as Transform).resume();
    if(limits)stream.pipe(admitted as Transform);
  };
  if(child.stdout)forward(child.stdout,options.stdout==="ignore"?undefined:options.stdout==="stderr"?process.stderr:process.stdout,options.stdout!=="ignore");
  forward(child.stderr!,process.stderr,true);
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
  const timeout = timeoutMs > 0 ? setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs) : undefined;
  try {
    const status = await new Promise<{code:number|null;signal:string|null}>((accept) => {
      child.once("error", error => console.error(error.message));
      child.once("close", (code, signal) => { closed = true; accept({code,signal}); });
    });
    stopProgress();
    await Promise.race([Promise.all([...pending]), interrupted]);
    await Promise.all([process.stdout,process.stderr].map(output=>new Promise<void>((accept,reject)=>output.write("",error=>error?reject(error):accept()))));
    if (publicationRefused) throw publicationFailure;
    if (stopped) throw new Error(`${label} stopped: ${stopped}`);
    if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
  } finally {
    options.signal?.removeEventListener("abort", abort);
    stopProgress();
    if (timeout) clearTimeout(timeout);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}
