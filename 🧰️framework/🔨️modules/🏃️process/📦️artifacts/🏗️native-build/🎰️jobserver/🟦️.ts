import { spawnSync } from "node:child_process";
import { randomBytes } from "node:crypto";
import { closeSync, constants, mkdirSync, openSync, readSync, rmSync, writeSync } from "node:fs";
import { join } from "node:path";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { runtimeSupportsV1 } from "../../../../⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeNativeSystemV1 } from "../../../../⚙️runtime/🧲️native/🟦️.ts";

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
  readonly makeflags: string;
  environment(): Record<string, string>;
  acquire(signal?: AbortSignal): Promise<JobSlotV1>;
  acquireAll(signal?: AbortSignal): Promise<readonly JobSlotV1[]>;
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

/** 🔌️ Non-blocking access to the published tokens. */
interface TokenChannelV1 {
  readonly endpoint: string;
  tryAcquire(): boolean;
  release(): void;
  close(): void;
}

/** 🧵️ A named fifo holding the published tokens, read without blocking so the driver's event loop never stalls. */
function fifoChannel(directory: string, published: number): TokenChannelV1 {
  mkdirSync(directory, { recursive: true });
  const endpoint = join(directory, protocol.fifoName);
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
async function semaphoreChannel(published: number): Promise<TokenChannelV1> {
  if (!runtimeSupportsV1("ffi")) throw Error("Win32 jobserver semaphores require the Bun runtime");
  const { dlopen, FFIType } = runtimeNativeSystemV1();
  const kernel = dlopen("kernel32.dll", {
    CreateSemaphoreW: { args: [FFIType.ptr, FFIType.i32, FFIType.i32, FFIType.ptr], returns: FFIType.ptr },
    WaitForSingleObject: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.u32 },
    ReleaseSemaphore: { args: [FFIType.ptr, FFIType.i32, FFIType.ptr], returns: FFIType.i32 },
    CloseHandle: { args: [FFIType.ptr], returns: FFIType.i32 },
  });
  const endpoint = `${protocol.semaphorePrefix}${process.pid}-${randomBytes(6).toString("hex")}`;
  const handle = kernel.symbols.CreateSemaphoreW(null, published, Math.max(published, 1), Buffer.from(`${endpoint}\0`, "utf16le"));
  if (!handle) throw Error(`CreateSemaphoreW refused ${endpoint}`);
  return {
    endpoint,
    tryAcquire: () => kernel.symbols.WaitForSingleObject(handle, 0) === 0,
    release: () => { if (!kernel.symbols.ReleaseSemaphore(handle, 1, null)) throw Error("Jobserver semaphore refused a returned token"); },
    close: () => { kernel.symbols.CloseHandle(handle); kernel.close(); },
  };
}

/** 🏊️ Creates a pool of `config.tokens` job slots: the driver keeps one implicit slot and publishes the rest, exactly as a top-level GNU make does.
 * Waiters are served first come, first served; a cancelled waiter leaves no token behind.
 * @see https://www.gnu.org/software/make/manual/html_node/POSIX-Jobserver.html */
export async function createJobserverV1(value: unknown, directory: string): Promise<JobserverV1> {
  const config = Object.freeze({...admitJobserverConfigV1(value)});
  const channel = config.style === "fifo" ? fifoChannel(directory, config.tokens - 1) : await semaphoreChannel(config.tokens - 1);
  const makeflags = jobserverMakeflagsV1(config.tokens, channel.endpoint, config.style);
  const queue: { count: number; slots: JobSlotV1[]; accept: (slots: readonly JobSlotV1[]) => void; reject: (reason: unknown) => void; signal?: AbortSignal; abort: () => void }[] = [];
  let implicitFree = true, held = 0, closed = false, timer: ReturnType<typeof setInterval> | undefined;
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
  const pump = (): void => {
    while (queue.length) {
      const waiter = queue[0]!;
      while (waiter.slots.length < waiter.count) {
        const granted = take();
        if (!granted) break;
        waiter.slots.push(granted);
      }
      if (waiter.slots.length < waiter.count) break;
      queue.shift();
      waiter.signal?.removeEventListener("abort", waiter.abort);
      waiter.accept(waiter.slots);
    }
    if (queue.length && !timer) timer = setInterval(pump, protocol.pollMs);
    if (!queue.length && timer) { clearInterval(timer); timer = undefined; }
  };
  const acquire = (count: number, signal?: AbortSignal): Promise<readonly JobSlotV1[]> => {
    if (closed) return Promise.reject(Error("Jobserver closed"));
    if (signal?.aborted) return Promise.reject(signal.reason);
    return new Promise((accept, reject) => {
      const waiter = { count, slots: [] as JobSlotV1[], accept, reject, signal, abort: () => {
        const index = queue.indexOf(waiter);
        if (index >= 0) queue.splice(index, 1);
        for (const held of waiter.slots) held.release();
        reject(signal!.reason);
        pump();
      } };
      signal?.addEventListener("abort", waiter.abort, { once: true });
      queue.push(waiter);
      pump();
    });
  };
  const server:JobserverV1=Object.freeze({
    config,
    endpoint: channel.endpoint,
    makeflags,
    environment: () => Object.fromEntries(protocol.clientEnvironment.map((name) => [name, makeflags])),
    acquire: async (signal?: AbortSignal) => (await acquire(1, signal))[0]!,
    acquireAll: (signal?: AbortSignal) => acquire(config.tokens, signal),
    tryAcquire: () => closed || queue.length ? null : take(),
    held: () => held,
    waiting: () => queue.length,
    close: () => {
      if (closed) return;
      closed = true;
      issuedJobservers.delete(server);
      if (timer) clearInterval(timer);
      for (const waiter of queue.splice(0)) { waiter.signal?.removeEventListener("abort", waiter.abort); for (const held of waiter.slots) held.release(); waiter.reject(Error("Jobserver closed")); }
      channel.close();
    },
  });
  issuedJobservers.add(server);return server;
}
