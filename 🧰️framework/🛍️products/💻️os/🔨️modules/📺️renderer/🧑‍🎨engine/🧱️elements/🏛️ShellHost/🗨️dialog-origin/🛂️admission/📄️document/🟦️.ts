/** 📄️ The exact route and worker admission owned by one document-opening attempt. */
export interface DocumentOpeningAttemptV1 {
  readonly current: () => boolean;
  readonly socket: () => Promise<unknown>;
  readonly attach: () => Promise<void>;
  readonly commit: () => void;
  readonly close: () => void;
  readonly detach: () => Promise<void>;
  readonly retire: () => void;
  readonly deadlineMs: number;
}

export type DocumentOpeningReceiptV1 = Readonly<{
  committed: true;
  runtimeKey: string;
  clientInstanceId: string;
}>;

/** 🚦️ Only foreground openings may retire a document or app instance already in use. */
export function admitDocumentOpeningV1<Plugin>(
  opening: Readonly<{ runtimeKey: string; plugin: Plugin; instanceId: number; background: boolean }>,
  owners: ReadonlyMap<string, Readonly<{ plugin: Plugin; session: Readonly<{ instanceId: number }>; clientInstanceId: string }>>,
  close: (runtimeKey: string, clientInstanceId: string) => void,
): boolean {
  const predecessors = [...owners].filter(([key, owner]) => key === opening.runtimeKey || (owner.plugin === opening.plugin && owner.session.instanceId === opening.instanceId));
  if (opening.background && predecessors.length > 0) return false;
  for (const [key, owner] of predecessors) close(key, owner.clientInstanceId);
  return true;
}

/** 🧹️ Failed or retired openings release only their admission; every socket deadline is retired. */
export async function runDocumentOpeningAttemptV1(port: DocumentOpeningAttemptV1): Promise<boolean> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  let committed = false;
  let attaching = false;
  try {
    if (!Number.isFinite(port.deadlineMs) || port.deadlineMs <= 0) throw new Error("invalid document opening deadline");
    const deadline = new Promise<never>((_, reject) => { timer = setTimeout(() => reject(new Error("document opening deadline exceeded")), port.deadlineMs); });
    await Promise.race([port.socket(), deadline]);
    if (!port.current()) return false;
    attaching = true;
    await Promise.race([port.attach(), deadline]);
    if (!port.current()) return false;
    port.commit();
    committed = true;
    return true;
  } finally {
    if (timer !== undefined) clearTimeout(timer);
    try {
      if (!committed) {
        try { port.close(); }
        finally { if (attaching) await port.detach(); }
      }
    }
    finally { port.retire(); }
  }
}

/** 🧵️ Orders physical attachment and retirement for one exact plugin app instance. */
export class DocumentAttachmentLaneV1 {
  #tail: Promise<unknown> = Promise.resolve();
  #desired: Readonly<{ owner: string }> | null = null;
  #attached: string | null = null;
  #pending = 0;
  readonly #detach: () => Promise<void>;

  constructor(detach: () => Promise<void>) { this.#detach = detach; }

  get idle(): boolean { return this.#pending === 0 && this.#desired === null && this.#attached === null; }

  drain(): Promise<void> { return this.#tail.then(() => {}); }

  attach(owner: string, current: () => boolean, apply: () => Promise<void>): Promise<void> {
    const intent = { owner };
    this.#desired = intent;
    return this.#append(async () => {
      if (this.#desired !== intent || !current()) return;
      if (this.#attached !== null && this.#attached !== owner) {
        await this.#detach();
        this.#attached = null;
        if (this.#desired !== intent || !current()) return;
      }
      this.#attached = owner;
      await apply();
    });
  }

  close(owner: string): Promise<void> {
    if (this.#desired?.owner === owner) this.#desired = null;
    return this.#append(async () => {
      if (this.#attached !== owner) return;
      await this.#detach();
      this.#attached = null;
    });
  }

  replace(owner: string, current: () => boolean, apply: () => Promise<void>): Promise<void> {
    const intent = { owner };
    this.#desired = intent;
    return this.#append(async () => {
      if (this.#desired !== intent || !current()) return;
      if (this.#attached !== null) {
        await this.#detach();
        this.#attached = null;
        if (this.#desired !== intent || !current()) return;
      }
      this.#attached = owner;
      try { await apply(); }
      catch (error) {
        await this.#detach();
        this.#attached = null;
        if (this.#desired === intent) this.#desired = null;
        throw error;
      }
    });
  }

  #append(operation: () => Promise<void>): Promise<void> {
    this.#pending++;
    const result = this.#tail.then(operation, operation).finally(() => { this.#pending--; });
    this.#tail = result.catch(() => {});
    return result;
  }
}

/** 🧊️ Retains one active and one latest cold pair; superseded work cannot publish a binding. */
export class LatestDocumentReplacementV1<Value> {
  #desired: object | null = null;
  #active: object | null = null;
  #running: Promise<void> | null = null;
  #next: { identity: object; value: Value; apply(value: Value, current: () => boolean): Promise<void>; resolve(value: boolean): void; reject(error: unknown): void } | null = null;

  get pending(): boolean { return this.#active !== null || this.#next !== null; }

  replace(value: Value, apply: (value: Value, current: () => boolean) => Promise<void>): Promise<boolean> {
    const identity = {};
    this.#desired = identity;
    this.#next?.resolve(false);
    const result = new Promise<boolean>((resolve, reject) => { this.#next = { identity, value, apply, resolve, reject }; });
    this.#start();
    return result;
  }

  invalidate(): void {
    this.#desired = null;
    this.#next?.resolve(false);
    this.#next = null;
  }

  #start(): void {
    if (this.#running !== null || this.#next === null) return;
    this.#running = Promise.resolve().then(async () => {
      while (this.#next !== null) {
        const operation = this.#next;
        this.#next = null;
        this.#active = operation.identity;
        const current = () => this.#desired === operation.identity;
        try {
          await operation.apply(operation.value, current);
          this.#active = null;
          operation.resolve(current());
        } catch (error) {
          this.#active = null;
          if (current()) operation.reject(error);
          else operation.resolve(false);
        }
      }
    }).finally(() => { this.#running = null; this.#start(); });
  }
}

/** 🏘️ Operations on one reusable background document admission. */
export interface BackgroundDocumentSessionPortV1<Value> {
  readonly current: (value: Value) => boolean;
  readonly create: () => Promise<Value | null>;
  readonly release: (value: Value) => Promise<void>;
  readonly visit: (value: Value) => Promise<void>;
}

/** 🧺️ Serializes work per Space and drains all owned admissions before Shell disposal. */
export class BackgroundDocumentSessionsV1<Value> {
  readonly #sessions = new Map<string, { value: Value; release: (value: Value) => Promise<void> }>();
  readonly #pending = new Map<string, Promise<void>>();
  #closed = false;
  #closing: Promise<void> | undefined;

  run(key: string, port: BackgroundDocumentSessionPortV1<Value>): Promise<void> {
    if (this.#closed) return Promise.resolve();
    return this.#append(key, async () => {
      if (this.#closed) return;
      let session = this.#sessions.get(key);
      if (session !== undefined && !port.current(session.value)) {
        this.#sessions.delete(key);
        await session.release(session.value);
        session = undefined;
      }
      if (this.#closed) return;
      if (session === undefined) {
        const value = await port.create();
        if (value === null) return;
        session = { value, release: port.release };
        if (this.#closed || !port.current(value)) { await session.release(value); return; }
        this.#sessions.set(key, session);
      }
      try { await port.visit(session.value); }
      finally {
        if (this.#closed || !port.current(session.value)) {
          this.#sessions.delete(key);
          await session.release(session.value);
        }
      }
    });
  }

  retire(key: string, matches: (value: Value) => boolean): Promise<void> {
    if (this.#closed || (!this.#sessions.has(key) && !this.#pending.has(key))) return Promise.resolve();
    return this.#append(key, async () => {
      const session = this.#sessions.get(key);
      if (session === undefined || !matches(session.value)) return;
      this.#sessions.delete(key);
      await session.release(session.value);
    });
  }

  async retain(current: (value: Value) => boolean): Promise<void> {
    const keys = new Set([...this.#sessions.keys(), ...this.#pending.keys()]);
    const results = await Promise.allSettled([...keys].map(key => this.retire(key, value => !current(value))));
    const failed = results.find((result): result is PromiseRejectedResult => result.status === "rejected");
    if (failed !== undefined) throw failed.reason;
  }

  close(): Promise<void> {
    if (this.#closing !== undefined) return this.#closing;
    this.#closed = true;
    this.#closing = (async () => {
      await Promise.allSettled(this.#pending.values());
      const sessions = [...this.#sessions.values()];
      this.#sessions.clear();
      const results = await Promise.allSettled(sessions.map(session => session.release(session.value)));
      const failed = results.find((result): result is PromiseRejectedResult => result.status === "rejected");
      if (failed !== undefined) throw failed.reason;
    })();
    return this.#closing;
  }

  #append(key: string, operation: () => Promise<void>): Promise<void> {
    const result = (this.#pending.get(key) ?? Promise.resolve()).catch(() => {}).then(operation)
      .finally(() => { if (this.#pending.get(key) === result) this.#pending.delete(key); });
    this.#pending.set(key, result);
    return result;
  }
}
