export type BrowserHostIdentity = Readonly<{ actorId: string; activationGeneration: bigint }>;
export type BrowserHostResult<T = unknown> = Readonly<{ tag: "ok"; val: T }> | Readonly<{ tag: "err"; val: Uint8Array }>;
export type BrowserHostFrame = BrowserHostIdentity & Readonly<{ kind: "frame"; frame: Readonly<{ kind: "Envelope"; envelope: Readonly<{ to: "kernel"; from: Readonly<{ kind: "actor"; id: string }>; lane: "Background"; seq: number; deadlineMs: null; coalesce: null; cancelOf: null; payload: Readonly<{ kind: string; payload: unknown }> }> }> }>;
export interface BrowserHostPort {
  dispatch(frame: BrowserHostFrame): void;
  cancelEffect(requestId: string): "queued" | "closed";
  log(level: string, message: string): void;
  nowMs(): bigint;
  traceSpan(name: string): void;
}

/** 🌐️ Owns one activation's canonical host imports, tagged effect replies and cancellable byte streams. */
export function createBrowserHostActivation(identity: BrowserHostIdentity, port: BrowserHostPort, signal?: AbortSignal) {
  if (typeof identity.actorId !== "string" || !identity.actorId.length || identity.actorId.length > 512 || /[\x00-\x1f\x7f]/.test(identity.actorId) || typeof identity.activationGeneration !== "bigint" || identity.activationGeneration < 1n || identity.activationGeneration > 0xffffffffffffffffn) throw new Error("browser host: invalid identity");
  const actorId = identity.actorId, activationGeneration = identity.activationGeneration;
  const pending = new Map<string, { resolve(value: unknown): void; reject(error: unknown): void }>();
  const streams = new Set<(cancel: boolean) => Promise<void>>();
  const faultPayloads = new WeakMap<object, Uint8Array>();
  let sequence = 0, pendingStreams = 0, bufferedBytes = 0, closed = false, closing: Promise<void> | undefined;
  class HostFault extends Error {
    readonly payload: Uint8Array;
    constructor(code: string, message: string) {
      super(message);
      this.payload = new TextEncoder().encode(JSON.stringify({ origin: "os", code, severity: "error", message, scope: {}, retryable: false }));
      faultPayloads.set(this, this.payload);
    }
  }
  const fault = (error: unknown, code: string): Uint8Array | HostFault => {
    let message = "browser host: operation failed";
    try {
      if (error instanceof Uint8Array || error instanceof HostFault) return error;
      const detail = typeof error === "string" ? error : error instanceof Error ? Object.getOwnPropertyDescriptor(error, "message")?.value : undefined;
      if (typeof detail === "string") message = detail.slice(0, 1024);
    } catch {}
    return new HostFault(code, message);
  };
  const check = () => { if (closed) throw new HostFault("capability-revoked", "browser host: closed"); };
  const nextSequence = () => {
    check();
    if (sequence >= Number.MAX_SAFE_INTEGER) throw new Error("browser host: sequence exhausted");
    return ++sequence;
  };
  const post = (seq: number, kind: string, payload: unknown) => port.dispatch({ kind: "frame", actorId, activationGeneration, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload } } } });
  const close = (): Promise<void> => {
    if (closing) return closing;
    closed = true;
    signal?.removeEventListener("abort", onAbort);
    const requests = Array.from(pending.keys());
    for (const entry of pending.values()) {
      entry.reject(new HostFault("capability-revoked", "browser host: closed"));
    }
    pending.clear();
    const operations = requests.map(id => Promise.resolve().then(() => {
      const state = port.cancelEffect(id);
      if (state !== "queued" && state !== "closed") throw new Error("browser host: invalid cancellation admission");
    }));
    for (const retire of streams) operations.push(retire(true));
    closing = Promise.allSettled(operations).then(results => {
      streams.clear();
      const errors = results.flatMap(result => result.status === "rejected" ? [result.reason] : []);
      if (errors.length) throw new AggregateError(errors, "browser host: retirement failed");
    });
    closing.catch(() => {});
    return closing;
  };
  const onAbort = () => { void close(); };
  const stream = (body: unknown): AsyncGenerator<number, void, unknown> => {
    check();
    if (!(body instanceof Uint8Array) && !(body instanceof ReadableStream)) throw new Error("browser host: invalid byte stream");
    if (streams.size >= 128) throw new Error("browser host: stream capacity");
    if (body instanceof Uint8Array && body.byteLength > 1048576 - bufferedBytes) throw new Error("browser host: buffered byte capacity");
    const bytes = body instanceof Uint8Array ? body.slice() : undefined;
    const reader = body instanceof ReadableStream ? body.getReader() as ReadableStreamDefaultReader<Uint8Array> : undefined;
    bufferedBytes += bytes?.byteLength ?? 0;
    let retired: Promise<void> | undefined, consumerTerminating = false;
    const checkStream = () => { check(); if (consumerTerminating) return false; if (retired) throw new Error("browser host: stream retired"); return true; };
    const retire = (cancel: boolean): Promise<void> => {
      if (retired) return retired;
      retired = Promise.resolve().then(async () => {
        let succeeded = false;
        try { if (cancel) await reader?.cancel("browser host: stream retired"); succeeded = true; }
        finally { bufferedBytes -= bytes?.byteLength ?? 0; bytes?.fill(0); reader?.releaseLock(); if (succeeded) streams.delete(retire); }
      });
      retired.catch(() => {});
      return retired;
    };
    streams.add(retire);
    const generator = (async function* () {
      let complete = false, sourceFailed = false, failed = false;
      try {
        if (!checkStream()) return;
        if (bytes) {
          for (const byte of bytes) { if (!checkStream()) return; yield byte; }
        } else if (reader) {
          for (;;) {
            if (!checkStream()) return;
            let result: ReadableStreamReadResult<Uint8Array>;
            try { result = await reader.read(); }
            catch (error) { sourceFailed = true; throw error; }
            if (!checkStream()) return;
            if (result.done) break;
            if (!(result.value instanceof Uint8Array)) throw new Error("browser host: invalid stream chunk");
            if (result.value.byteLength > 65536 || result.value.buffer.byteLength > 65536) throw new Error("browser host: chunk byte capacity");
            for (const byte of result.value) { if (!checkStream()) return; yield byte; }
          }
        }
        complete = true;
      } catch (error) { failed = true; throw fault(error, "browser.host.stream-failed"); }
      finally { try { await retire(!complete && !sourceFailed); } catch (error) { if (!failed) throw error; } }
    })();
    const returned = generator.return.bind(generator), thrown = generator.throw.bind(generator);
    const finish = async (operation: () => Promise<IteratorResult<number, void>>): Promise<IteratorResult<number, void>> => {
      consumerTerminating = true;
      const [retirement, completion] = await Promise.allSettled([retire(true), operation()]);
      if (retirement.status === "rejected") throw retirement.reason;
      if (completion.status === "rejected") throw completion.reason;
      return completion.value;
    };
    generator.return = value => finish(() => returned(value));
    generator.throw = error => finish(() => thrown(error));
    return generator;
  };
  const request = <T = unknown>(effect: string, params: unknown, project: (value: unknown) => T = value => value as T, expectsStream = false): Promise<T> => {
    const result = new Promise<T>((resolve, reject) => {
      check();
      if (pending.size >= 128) throw new Error("browser host: pending capacity");
      if (expectsStream && streams.size + pendingStreams >= 128) throw new Error("browser host: stream capacity");
      const seq = nextSequence();
      const requestId = JSON.stringify([actorId, activationGeneration.toString(), effect, seq]);
      if (expectsStream) pendingStreams++;
      let settled = false;
      const settle = (): boolean => {
        if (settled) return false;
        settled = true;
        if (expectsStream) pendingStreams--;
        return true;
      };
      pending.set(requestId, { resolve(value) {
        if (!settle()) return;
        try {
          if (!value || typeof value !== "object" || !("tag" in value) || !("val" in value) || Object.keys(value).length !== 2) throw new Error("browser host: invalid result");
          if (value.tag === "ok") resolve(project(value.val));
          else if (value.tag === "err" && value.val instanceof Uint8Array) reject(value.val);
          else throw new Error("browser host: invalid result");
        } catch (error) { reject(error); }
      }, reject(error) { if (settle()) reject(error); } });
      try { post(seq, "effect-request", { effect, requestId, params }); }
      catch (error) { const entry = pending.get(requestId); pending.delete(requestId); entry?.reject(error); }
    }).catch(error => { throw fault(error, "browser.host.request-failed"); });
    result.catch(() => {});
    return result;
  };
  const call = (effect: string) => (params: unknown) => request(effect, params);
  const fire = (kind: string, payload: unknown) => post(nextSequence(), kind, payload);
  const pure = Object.freeze({ log(level: string, message: string) { check(); port.log(level, message); }, nowMs() { check(); return port.nowMs(); }, traceSpan(name: string) { check(); port.traceSpan(name); } });
  const hostAsync = Object.freeze({
    storageRead: call("storage-read"), storageWrite: call("storage-write"), storageDelete: call("storage-delete"),
    blobLoad: call("blob-load"), blobWrite: call("blob-write"), blobRead: (hash: string) => request("blob-read", { hash }, stream, true),
    httpFetch: (params: unknown) => request("http-fetch", params, value => {
      if (!value || typeof value !== "object" || !("body" in value)) throw new Error("browser host: invalid HTTP response");
      return { ...value, body: stream(value.body) };
    }, true),
    documentRead: call("document-read"), documentWrite: call("document-write"), linkResolve: (link: unknown) => request("link-resolve", { link }),
    registryQuery: call("registry-query"), ioCompose: call("io-compose"), ioRun: call("io-run"), cacheDerive: call("cache-derive"), cacheRead: call("cache-read"),
    invokeExtension: call("invoke-extension"), openWindow: call("open-window"), openDialog: call("open-dialog"), dispatchAction: call("dispatch-action"),
    spawnPluginInstance: call("spawn-plugin-instance"), requestFileOpen: call("request-file-open"), requestMediaFrames: call("request-media-frames"), requestCapability: call("request-capability"),
    spawnJob: (job: bigint, kind: string, input: Uint8Array, placement: unknown) => request("spawn-job", { job, kind, input, placement }),
    emit: (value: unknown) => fire("effect-emit", value), emitPatch: (patch: unknown) => fire("ui-patch-emit", patch),
  });
  signal?.addEventListener("abort", onAbort, { once: true });
  if (signal?.aborted) void close();
  return Object.freeze({
    pure, hostAsync, close,
    invocationFailure(error: unknown): unknown { return error !== null && typeof error === "object" ? faultPayloads.get(error) ?? error : error; },
    resolveEffect(requestId: string, value: unknown): boolean {
      const entry = pending.get(requestId);
      if (!entry || closed) return false;
      pending.delete(requestId);
      entry.resolve(value);
      return true;
    },
    rejectEffect(requestId: string, error: unknown): boolean {
      const entry = pending.get(requestId);
      if (!entry || closed) return false;
      pending.delete(requestId);
      entry.reject(error);
      return true;
    },
  });
}
