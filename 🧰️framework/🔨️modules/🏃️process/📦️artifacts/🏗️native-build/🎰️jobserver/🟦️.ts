import { spawnSync } from "node:child_process";
import { randomBytes } from "node:crypto";
import { closeSync, constants, mkdirSync, openSync, readSync, rmSync, writeSync } from "node:fs";
import { join } from "node:path";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { runtimeSupportsV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeNativeSystemV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";

const protocol = schema.definitions.Protocol.const;
const issuedJobservers=new WeakSet<JobserverV1>();
/** 🔐️ Requires an actual live first-party token pool before stage effects. */
export function requireJobserverV1(value:JobserverV1):void{if(!issuedJobservers.has(value))throw Error("Actual live jobserver required");}

/** 🎰️ How the pool publishes its tokens: a named fifo on Unix, a named semaphore on Windows. */
export type JobserverStyleV1 = "fifo" | "semaphore";

/** 🧾️ The admitted size and style of one job-token pool. */
export type JobserverConfigV1 = Readonly<{ version: 1; tokens: number; style: JobserverStyleV1 }>;

/** 🎫️ One held job slot; releasing it twice is a no-op. */
export type JobSlotV1 = Readonly<{ implicit: boolean; release: () => void }>;

/** 🏊️ A GNU make compatible job-token pool. Cargo, rustc and every driver stage draw from it, so the machine runs exactly `tokens` jobs. */
export interface JobserverV1 {
  readonly config: JobserverConfigV1;
  readonly endpoint: string;
  /** 🔗️ Keeps the published endpoint live until the callback joins the client and its owned descendants. */
  withClient<T>(run: (environment: Readonly<Record<string, string>>) => Promise<T>): Promise<T>;
  acquire(signal?: AbortSignal): Promise<JobSlotV1>;
  /** 🧲️ Waits in FIFO order for the entire pool and leaves partial availability to active clients. */
  acquireAll(signal?: AbortSignal): Promise<readonly JobSlotV1[]>;
  /** 💤️ Waits for a full opportunity behind ordinary waiters without retaining partial slots, then queues for one when the opportunity expires. */
  acquireAvailable(count:number,waitMilliseconds:number,signal?:AbortSignal):Promise<readonly JobSlotV1[]>;
  tryAcquire(): JobSlotV1 | null;
  held(): number;
  waiting(): number;
  close(): void;
}

/** 🛂️ Admits only a pool configuration that satisfies the first-party schema. */
export function admitJobserverConfigV1(value: unknown): JobserverConfigV1 {
  const errors = validateJsonSchemaSubset(schema, value);
  if (errors.length) throw Error(`Invalid jobserver configuration: ${errors.join("; ")}`);
  return value as JobserverConfigV1;
}

/** 🖥️ The pool style the platform's Cargo understands. */
export function defaultJobserverStyleV1(platform: NodeJS.Platform = process.platform): JobserverStyleV1 {
  return platform === "win32" ? "semaphore" : "fifo";
}

/** 🏳️ The `MAKEFLAGS` value that hands the pool to a GNU make jobserver client. */
export function jobserverMakeflagsV1(tokens: number, endpoint: string, style: JobserverStyleV1): string {
  return `-j${tokens} ${protocol.authFlag}${style === "fifo" ? `fifo:${endpoint}` : endpoint}`;
}

/** 🪪️ The `--jobserver-auth` value a client reads from its environment (Cargo's precedence, newest declaration winning), `null` when none is named. */
export function jobserverAuthFromEnvironmentV1(environment: Readonly<Record<string, string | undefined>>): string | null {
  for (const name of protocol.authPrecedence) {
    const words = (environment[name] ?? "").split(/\s+/u).filter((word) => word.startsWith(protocol.authFlag));
    if (words.length) return words.at(-1)!.slice(protocol.authFlag.length);
  }
  return null;
}

/** 📴️ Clears inherited jobserver variables before issuing a fresh client environment or launching reserved work. */
export function withoutJobserverEnvironmentV1(environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  const unbound = { ...environment };
  for (const name of protocol.authPrecedence) delete unbound[name];
  return unbound;
}

/** 🔌️ Non-blocking access to the published tokens. */
interface TokenChannelV1 {
  readonly endpoint: string;
  tryAcquire(): boolean;
  release(): void;
  close(): void;
}

/** 🧵️ A named fifo holding the published tokens, read without blocking so the driver's event loop never stalls. */
function fifoChannel(directory: string, name: string, published: number): TokenChannelV1 {
  mkdirSync(directory, { recursive: true });
  const endpoint = join(directory, name);
  rmSync(endpoint, { force: true });
  const made = spawnSync("mkfifo", ["-m", "600", endpoint], { encoding: "utf8" });
  if (made.status !== 0) throw Error(`mkfifo refused ${endpoint}: ${made.stderr || made.error?.message}`);
  const descriptor = openSync(endpoint, constants.O_RDWR | constants.O_NONBLOCK);
  const token = Buffer.from(protocol.token), scratch = Buffer.alloc(1);
  if (published > 0 && writeSync(descriptor, Buffer.alloc(published, protocol.token)) !== published) throw Error("Jobserver fifo refused its initial tokens");
  return {
    endpoint,
    tryAcquire: () => {
      try { return readSync(descriptor, scratch, 0, 1, null) === 1; }
      catch (error) { if ((error as NodeJS.ErrnoException).code === "EAGAIN") return false; throw error; }
    },
    release: () => { if (writeSync(descriptor, token) !== 1) throw Error("Jobserver fifo refused a returned token"); },
    close: () => { closeSync(descriptor); rmSync(endpoint, { force: true }); },
  };
}

/** 🪟️ A named Win32 semaphore holding the published tokens, polled with a zero timeout; `bun:ffi` is read as a runtime builtin so Node and Vite never resolve it. */
function semaphoreChannel(published: number, maximum: number): TokenChannelV1 {
  if (!runtimeSupportsV1("ffi")) throw Error("Win32 jobserver semaphores require the Bun runtime");
  const { dlopen, FFIType } = runtimeNativeSystemV1();
  const kernel = dlopen("kernel32.dll", {
    CreateSemaphoreW: { args: [FFIType.ptr, FFIType.i32, FFIType.i32, FFIType.ptr], returns: FFIType.ptr },
    WaitForSingleObject: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.u32 },
    ReleaseSemaphore: { args: [FFIType.ptr, FFIType.i32, FFIType.ptr], returns: FFIType.i32 },
    CloseHandle: { args: [FFIType.ptr], returns: FFIType.i32 },
  });
  const endpoint = `${protocol.semaphorePrefix}${process.pid}-${randomBytes(6).toString("hex")}`;
  const handle = kernel.symbols.CreateSemaphoreW(null, published, Math.max(maximum, 1), Buffer.from(`${endpoint}\0`, "utf16le"));
  if (!handle) { kernel.close(); throw Error(`CreateSemaphoreW refused ${endpoint}`); }
  return {
    endpoint,
    tryAcquire: () => kernel.symbols.WaitForSingleObject(handle, 0) === 0,
    release: () => { if (!kernel.symbols.ReleaseSemaphore(handle, 1, null)) throw Error("Jobserver semaphore refused a returned token"); },
    close: () => { kernel.symbols.CloseHandle(handle); kernel.close(); },
  };
}

/** 🆕️ Creates a pool of `config.tokens` job slots: the driver keeps one implicit slot and publishes the rest, exactly as a top-level GNU make does.
 * Waiters are served first come, first served; a cancelled waiter leaves no token behind.
 * @see https://www.gnu.org/software/make/manual/html_node/POSIX-Jobserver.html */
export async function createJobserverV1(value: unknown, directory: string): Promise<JobserverV1> {
  const config = Object.freeze({...admitJobserverConfigV1(value)});
  const published = config.tokens - 1;
  const openChannel = (name: string, count: number): TokenChannelV1 => config.style === "fifo" ? fifoChannel(directory, name, count) : semaphoreChannel(count, published);
  let channel = openChannel(protocol.fifoName, published), activeClients = 0;
  const environment = (): Record<string, string> => {
    const flags = jobserverMakeflagsV1(config.tokens, channel.endpoint, config.style);
    return Object.fromEntries(protocol.clientEnvironment.map((name) => [name, flags]));
  };
  type Waiter = { count:number; accept:(slots:readonly JobSlotV1[])=>void; reject:(reason:unknown)=>void; signal?:AbortSignal; abort:()=>void };
  const queue:Waiter[]=[], opportunities:Waiter[]=[];
  let implicitFree = true, held = 0, closed = false, pumping=false, timer: ReturnType<typeof setInterval> | undefined;
  const slot = (implicit: boolean): JobSlotV1 => {
    held++;
    let released = false;
    return Object.freeze({ implicit, release: () => {
      if (released) return;
      released = true;
      held--;
      if (closed) return;
      if (implicit) implicitFree = true; else channel.release();
      pump();
    } });
  };
  const take = (): JobSlotV1 | null => {
    if (implicitFree) { implicitFree = false; return slot(true); }
    return channel.tryAcquire() ? slot(false) : null;
  };
  const reserve = (count: number): readonly JobSlotV1[] | undefined => {
    if (count > config.tokens - held) return;
    const slots: JobSlotV1[] = [];
    try {
      while (slots.length < count) {
        const granted = take();
        if (!granted) return;
        slots.push(granted);
      }
      return slots;
    } finally {
      if (slots.length < count) for (const acquired of slots) acquired.release();
    }
  };
  const drain = (waiters: Waiter[]): void => {
    while (waiters.length) {
      const waiter = waiters[0]!, slots = reserve(waiter.count);
      if (!slots) return;
      waiters.shift();
      waiter.signal?.removeEventListener("abort", waiter.abort);
      waiter.accept(slots);
    }
  };
  const pump = (): void => {
    if(pumping||closed)return;
    pumping=true;
    try{
      drain(queue);
      if (!queue.length) drain(opportunities);
      if(queue.length&&!timer)timer=setInterval(pump,protocol.pollMs);
      if(!queue.length&&timer){clearInterval(timer);timer=undefined;}
    }finally{pumping=false;}
  };
  const reconcileChannel = (): void => {
    const expected = published - (held - Number(!implicitFree)), previous = channel;
    let returned = 0;
    try {
      while (returned < expected && previous.tryAcquire()) returned++;
      if (returned < expected) {
        channel = openChannel(`${protocol.fifoReplacementPrefix}${randomBytes(6).toString("hex")}`, expected);
        previous.close();
      }
    } finally {
      if (channel === previous) for (let index = 0; index < returned; index++) previous.release();
    }
    pump();
  };
  const withClient = async <T>(run: (environment: Readonly<Record<string, string>>) => Promise<T>): Promise<T> => {
    if (closed) throw Error("Jobserver closed");
    activeClients++;
    let primary: { error: unknown } | undefined;
    try { return await run(Object.freeze(environment())); }
    catch (error) { primary = { error }; throw error; }
    finally {
      if (--activeClients === 0 && !closed) {
        try { reconcileChannel(); }
        catch (error) {
          if (primary && !Object.is(primary.error, error)) throw new AggregateError([primary.error, error], "Jobserver client and channel retirement failed");
          throw error;
        }
      }
    }
  };
  const acquire = (count: number, signal?: AbortSignal,waiters=queue): Promise<readonly JobSlotV1[]> => {
    if (closed) return Promise.reject(Error("Jobserver closed"));
    if (signal?.aborted) return Promise.reject(signal.reason);
    return new Promise((accept,reject)=>{
      const waiter:Waiter={count,accept,reject,signal,abort:()=>{
        const index=waiters.indexOf(waiter);
        if(index>=0)waiters.splice(index,1);
        reject(signal!.reason);
        pump();
      }};
      signal?.addEventListener("abort",waiter.abort,{once:true});
      waiters.push(waiter);
      pump();
    });
  };
  const acquireAvailable=async(count:number,waitMilliseconds:number,signal?:AbortSignal):Promise<readonly JobSlotV1[]>=>{
    if(!Number.isSafeInteger(count)||count<1||count>config.tokens||!Number.isSafeInteger(waitMilliseconds)||waitMilliseconds<0||waitMilliseconds>2**31-1)throw Error("Finite jobserver opportunity required");
    if(count===1||waitMilliseconds===0)return acquire(1,signal);
    const timeout=new AbortController(),elapsed=Error("Jobserver opportunity elapsed");
    const combined=signal?AbortSignal.any([signal,timeout.signal]):timeout.signal;
    const timer=setTimeout(()=>timeout.abort(elapsed),waitMilliseconds);
    try{return await acquire(count,combined,opportunities);}
    catch(error){
      if(signal?.aborted)throw signal.reason;
      if(error!==elapsed)throw error;
      return acquire(1,signal);
    }finally{clearTimeout(timer);}
  };
  const server:JobserverV1=Object.freeze({
    config,
    get endpoint() { return channel.endpoint; },
    withClient,
    acquire: async (signal?: AbortSignal) => (await acquire(1, signal))[0]!,
    acquireAll: (signal?: AbortSignal) => acquire(config.tokens, signal),
    acquireAvailable,
    tryAcquire: () => closed || queue.length ? null : take(),
    held: () => held,
    waiting: () => queue.length,
    close: () => {
      if (closed) return;
      closed = true;
      issuedJobservers.delete(server);
      if (timer) clearInterval(timer);
      for (const waiter of [...queue.splice(0),...opportunities.splice(0)]) { waiter.signal?.removeEventListener("abort", waiter.abort); waiter.reject(Error("Jobserver closed")); }
      channel.close();
    },
  });
  issuedJobservers.add(server);return server;
}

/** 🧮 The job count a Cargo process gets: the local grant plus the light slots the environment names (`SEMIO_UNIT_CACHE_LIGHT_SLOTS`, the units that run on a worker), so Cargo keeps every local slot and every remote slot busy. The wrapper's own slots (`SEMIO_UNIT_CACHE_SLOTS`) still bound the compilers that run here and the light slots those that run elsewhere. */
export function cargoJobsV1(local: number, environment: Readonly<Record<string, string | undefined>>): number {
  const light = Number(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS ?? 0);
  return Math.max(1, Math.floor(local)) + (Number.isSafeInteger(light) && light > 0 ? light : 0);
}
