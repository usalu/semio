/** 📨️ Supplies a monotonic clock and a synchronous sink that consumes/copies bytes before returning. */
export interface BrowserWasiPort {
  nowNs(): bigint;
  write(channel: "stdout" | "stderr", bytes: Uint8Array): void;
  exit?(status: "ok" | "err"): void;
}


export type GuestLogLevel = "debug" | "error" | "log";

export type GuestLogLine = {
  channel: "stdout" | "stderr";
  text: string;
  level: GuestLogLevel;
};

/** 🗣️ `[DEBUG]` prefixes are routine guest chatter; anything else on stderr is a fault. */
export function classifyGuestLogLine(channel: "stdout" | "stderr", text: string): GuestLogLevel {
  if (channel === "stdout") return "log";
  return text.startsWith("[DEBUG]") ? "debug" : "error";
}

/** 🧵 One logical guest line is one host emit — tokens without a newline stay pending. */
export function createGuestLogLineSink(emit: (line: GuestLogLine) => void) {
  const pending: Record<"stdout" | "stderr", Uint8Array> = { stdout: new Uint8Array(0), stderr: new Uint8Array(0) };
  const decoder = new TextDecoder();
  return {
    write(channel: "stdout" | "stderr", bytes: Uint8Array) {
      const prev = pending[channel];
      const merged = new Uint8Array(prev.length + bytes.byteLength);
      merged.set(prev);
      merged.set(bytes, prev.length);
      let start = 0;
      for (let i = 0; i < merged.length; i++) {
        if (merged[i] === 10) {
          const text = decoder.decode(merged.subarray(start, i));
          emit({ channel, text, level: classifyGuestLogLine(channel, text) });
          start = i + 1;
        }
      }
      pending[channel] = start === 0 ? merged : merged.subarray(start);
    },
    pendingBytes(channel: "stdout" | "stderr") {
      return pending[channel].byteLength;
    },
  };
}

export const browserWasiInterfaces = Object.freeze([
  "wasi:cli/environment@0.2.0", "wasi:cli/exit@0.2.0", "wasi:cli/stdin@0.2.0", "wasi:cli/stdout@0.2.0", "wasi:cli/stderr@0.2.0",
  "wasi:cli/terminal-input@0.2.0", "wasi:cli/terminal-output@0.2.0", "wasi:cli/terminal-stdin@0.2.0", "wasi:cli/terminal-stdout@0.2.0", "wasi:cli/terminal-stderr@0.2.0",
  "wasi:clocks/monotonic-clock@0.2.0", "wasi:io/error@0.2.0", "wasi:io/poll@0.2.0", "wasi:io/streams@0.2.0",
]);

/** 🧭️ Owns a bounded Preview2 profile without ambient process, filesystem or network authority. */
export function createBrowserWasiActivation(port: BrowserWasiPort, signal?: AbortSignal) {
  const admission = Symbol("wasi-admission"), dispose = Symbol.dispose ?? Symbol.for("dispose");
  const resources = new Set<Resource>();
  const timers = new Map<Pollable, ReturnType<typeof setTimeout>>();
  const waiters = new Set<{ pollables: Pollable[]; resolve(indices: Uint32Array): void; reject(error: unknown): void }>();
  let closed = false, outputBytes = 0, errorReservations = 0, lastNow = 0n, closing: Promise<void> | undefined;
  const check = () => { if (closed) throw new Error("browser wasi: closed"); };
  const u64 = (value: unknown): bigint => {
    if (typeof value !== "bigint" || value < 0n || value > 0xffffffffffffffffn) throw new Error("browser wasi: invalid u64");
    return value;
  };
  const now = () => {
    check();
    const value = u64(port.nowNs());
    check();
    if (value < lastNow) throw new Error("browser wasi: nonmonotonic clock");
    lastNow = value;
    return value;
  };
  const readyIndices = (pollables: Pollable[]) => Uint32Array.from(pollables.flatMap((pollable, index) => pollable.ready() ? [index] : []));
  const notify = () => {
    for (const waiter of waiters) {
      try {
        const ready = readyIndices(waiter.pollables);
        if (ready.length) { waiters.delete(waiter); waiter.resolve(ready); }
      } catch (error) { waiters.delete(waiter); waiter.reject(error); }
    }
  };
  class Resource {
    children = 0;
    constructor(token: symbol, readonly parent?: Resource) {
      check();
      if (token !== admission) throw new Error("browser wasi: private resource constructor");
      if (resources.size + errorReservations >= 256) throw new Error("browser wasi: resource capacity");
      parent?.check();
      resources.add(this);
      if (parent) parent.children++;
    }
    check() { check(); if (!resources.has(this)) throw new Error("browser wasi: foreign or retired resource"); }
    retire(force = false) {
      if (!resources.has(this)) return;
      if (this.children && !force) throw new Error("browser wasi: resource has live children");
      resources.delete(this);
      if (this.parent) this.parent.children--;
    }
    [dispose]() { this.retire(); }
  }
  class WasiIoError extends Resource {
    constructor(token: symbol, private readonly message: string) { super(token); }
    toDebugString() { this.check(); return this.message; }
  }
  class Pollable extends Resource {
    failure: unknown;
    constructor(token: symbol, parent?: Resource, private readonly deadline?: bigint) {
      super(token, parent);
      if (deadline !== undefined) {
        try { this.arm(); } catch (error) { this.retire(); throw error; }
      }
    }
    arm() {
      this.check();
      if (this.deadline === undefined) return;
      const remaining = this.deadline - now();
      this.check();
      if (remaining <= 0n) return;
      const delay = (remaining + 999999n) / 1000000n;
      timers.set(this, setTimeout(() => {
        timers.delete(this);
        if (closed || !resources.has(this)) return;
        try { this.arm(); } catch (error) { this.failure = error; }
        notify();
      }, Number(delay > 2147483647n ? 2147483647n : delay)));
    }
    ready() {
      this.check();
      if (this.failure !== undefined) throw this.failure;
      if (this.deadline === undefined) return true;
      const current = now();
      this.check();
      return this.deadline <= current;
    }
    async block() { await poll([this]); }
    override retire(force = false) {
      const timer = timers.get(this);
      if (timer !== undefined) clearTimeout(timer);
      timers.delete(this);
      super.retire(force);
      notify();
    }
  }
  const poll = (list: Pollable[]): Promise<Uint32Array> => {
    const result = new Promise<Uint32Array>((resolve, reject) => {
      check();
      if (!Array.isArray(list) || !list.length || list.length > 128) throw new Error("browser wasi: invalid poll list");
      for (const item of list) {
        if (!(item instanceof Pollable)) throw new Error("browser wasi: foreign pollable");
        item.check();
      }
      const ready = readyIndices(list);
      if (ready.length) { resolve(ready); return; }
      if (waiters.size >= 128) throw new Error("browser wasi: waiter capacity");
      waiters.add({ pollables: list.slice(), resolve, reject });
    });
    result.catch(() => {});
    return result;
  };
  class InputStream extends Resource {
    read(length: bigint): Uint8Array { this.check(); u64(length); throw { tag: "closed" }; }
    async blockingRead(length: bigint) { return this.read(length); }
    skip(length: bigint): bigint { this.read(length); return 0n; }
    async blockingSkip(length: bigint) { return this.skip(length); }
    subscribe() { this.check(); return new Pollable(admission, this); }
  }
  class OutputStream extends Resource {
    private permit = 0;
    private failed = false;
    private errorCredit = true;
    constructor(token: symbol, private readonly channel: "stdout" | "stderr") {
      if (resources.size + errorReservations + 2 > 256) throw new Error("browser wasi: resource capacity");
      super(token);
      errorReservations++;
    }
    override retire(force = false) {
      super.retire(force);
      if (this.errorCredit) { this.errorCredit = false; errorReservations--; }
    }
    private writable() { this.check(); if (this.failed || outputBytes >= 1048576) throw { tag: "closed" }; }
    checkWrite() { this.writable(); this.permit = Math.min(65536, 1048576 - outputBytes); return BigInt(this.permit); }
    write(bytes: Uint8Array) {
      this.writable();
      if (!(bytes instanceof Uint8Array) || bytes.byteLength > this.permit || bytes.byteLength > 65536 || bytes.byteLength > 1048576 - outputBytes) throw new Error("browser wasi: write permit exceeded");
      this.permit = 0;
      const owned = bytes.slice();
      outputBytes += owned.byteLength;
      try { port.write(this.channel, owned); }
      catch (error) {
        this.failed = true;
        if (closed || !resources.has(this) || !this.errorCredit) throw { tag: "closed" };
        let message = "output sink failed";
        try {
          const detail = typeof error === "string" ? error : error instanceof Error ? Object.getOwnPropertyDescriptor(error, "message")?.value : undefined;
          if (typeof detail === "string") message = detail.slice(0, 1024);
        } catch {}
        if (closed || !resources.has(this) || !this.errorCredit) throw { tag: "closed" };
        this.errorCredit = false;
        errorReservations--;
        throw { tag: "last-operation-failed", val: new WasiIoError(admission, message) };
      } finally { owned.fill(0); }
    }
    flush() { this.writable(); this.permit = 0; }
    async blockingFlush() { this.flush(); }
    async blockingWriteAndFlush(bytes: Uint8Array) {
      this.writable();
      if (!(bytes instanceof Uint8Array) || bytes.byteLength > 4096) throw new Error("browser wasi: blocking write byte capacity");
      this.checkWrite(); this.write(bytes); this.flush();
    }
    writeZeroes(length: bigint) {
      const count = u64(length);
      if (count > BigInt(this.permit) || count > 65536n) throw new Error("browser wasi: write permit exceeded");
      this.write(new Uint8Array(Number(count)));
    }
    async blockingWriteZeroesAndFlush(length: bigint) {
      const count = u64(length);
      if (count > 4096n) throw new Error("browser wasi: blocking write byte capacity");
      await this.blockingWriteAndFlush(new Uint8Array(Number(count)));
    }
    splice(input: InputStream, length: bigint) {
      this.writable();
      if (!(input instanceof InputStream)) throw new Error("browser wasi: foreign input stream");
      const count = u64(length), permit = this.checkWrite();
      const bytes = input.read(count < permit ? count : permit);
      this.write(bytes);
      return BigInt(bytes.byteLength);
    }
    async blockingSplice(input: InputStream, length: bigint) { return this.splice(input, length); }
    subscribe() { this.check(); return new Pollable(admission, this); }
  }
  class TerminalInput { private constructor() {} }
  class TerminalOutput { private constructor() {} }
  const close = (): Promise<void> => {
    if (closing) return closing;
    closed = true;
    signal?.removeEventListener("abort", onAbort);
    for (const waiter of waiters) waiter.reject(new Error("browser wasi: closed"));
    waiters.clear();
    for (const resource of resources) resource.retire(true);
    closing = Promise.resolve();
    return closing;
  };
  const onAbort = () => { void close(); };
  const imports = Object.freeze({
    "wasi:cli/environment@0.2.0": Object.freeze({ getEnvironment() { check(); return []; }, getArguments() { check(); return []; }, initialCwd() { check(); return undefined; } }),
    "wasi:cli/exit@0.2.0": Object.freeze({ exit(status: { tag: "ok" | "err" }) {
      check();
      if (!status || !["ok", "err"].includes(status.tag)) throw new Error("browser wasi: invalid exit");
      void close();
      port.exit?.(status.tag);
      throw new Error(`browser wasi: component exit ${status.tag}`);
    } }),
    "wasi:cli/stdin@0.2.0": Object.freeze({ getStdin() { return new InputStream(admission); } }),
    "wasi:cli/stdout@0.2.0": Object.freeze({ getStdout() { return new OutputStream(admission, "stdout"); } }),
    "wasi:cli/stderr@0.2.0": Object.freeze({ getStderr() { return new OutputStream(admission, "stderr"); } }),
    "wasi:cli/terminal-input@0.2.0": Object.freeze({ TerminalInput }),
    "wasi:cli/terminal-output@0.2.0": Object.freeze({ TerminalOutput }),
    "wasi:cli/terminal-stdin@0.2.0": Object.freeze({ getTerminalStdin() { check(); return undefined; } }),
    "wasi:cli/terminal-stdout@0.2.0": Object.freeze({ getTerminalStdout() { check(); return undefined; } }),
    "wasi:cli/terminal-stderr@0.2.0": Object.freeze({ getTerminalStderr() { check(); return undefined; } }),
    "wasi:clocks/monotonic-clock@0.2.0": Object.freeze({ now, resolution() { check(); return 1000000n; }, subscribeInstant(instant: bigint) { return new Pollable(admission, undefined, u64(instant)); }, subscribeDuration(duration: bigint) { return new Pollable(admission, undefined, now() + u64(duration)); } }),
    "wasi:io/error@0.2.0": Object.freeze({ Error: WasiIoError }),
    "wasi:io/poll@0.2.0": Object.freeze({ Pollable, poll }),
    "wasi:io/streams@0.2.0": Object.freeze({ InputStream, OutputStream }),
  });
  signal?.addEventListener("abort", onAbort, { once: true });
  if (signal?.aborted) void close();
  return Object.freeze({ imports, close, progress: () => Object.freeze({ phase: closed ? "closed" : "open", resources: resources.size + errorReservations, waiters: waiters.size, timers: timers.size }) });
}
