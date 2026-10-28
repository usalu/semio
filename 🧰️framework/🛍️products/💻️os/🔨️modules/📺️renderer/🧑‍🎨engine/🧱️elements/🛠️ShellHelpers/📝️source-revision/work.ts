import type { LocalSourceRevisionV1, SourceRevisionOwnerV1 } from "./🟦️.ts";

export type SourceWorkCompletionV1 = Readonly<{ instanceId: number; operation: bigint }>;
type Request = Readonly<{
  source: LocalSourceRevisionV1;
  owner: SourceRevisionOwnerV1;
  plugin: object;
  subscribe(listener: (completion: SourceWorkCompletionV1) => void): () => void;
  refresh(): void;
}>;
export type SourceRevisionWorkV1 = Readonly<{ admit(operation: bigint | undefined): void; finish(): void }>;
type Operations = Map<bigint, SourceWork | null>;
const operations = new WeakMap<LocalSourceRevisionV1, WeakMap<SourceRevisionOwnerV1, WeakMap<object, Operations>>>();

function operationRegistry(request: Request): Operations {
  let owners = operations.get(request.source);
  if (!owners) operations.set(request.source, owners = new WeakMap());
  let plugins = owners.get(request.owner);
  if (!plugins) owners.set(request.owner, plugins = new WeakMap());
  let registry = plugins.get(request.plugin);
  if (!registry) plugins.set(request.plugin, registry = new Map());
  return registry;
}

/** ⏳️ Keeps uncertain source work unresolved until an authentic terminal or exact owner retirement. */
class SourceWork implements SourceRevisionWorkV1 {
  #admitted = false;
  #operation: bigint | undefined;
  #terminal = false;
  #finished = false;
  #closed = false;
  #sourceUnsubscribe: (() => void) | null = null;
  #completionUnsubscribe: (() => void) | null = null;
  readonly #early = new Set<bigint>();
  readonly #operations: Operations;
  readonly #request: Request;
  readonly #release: () => void;

  constructor(request: Request, release: () => void) {
    this.#request = request;
    this.#release = release;
    this.#operations = operationRegistry(request);
  }

  attach(): void {
    const unsubscribe = this.#request.source.subscribe(() => {
      if (!this.#request.source.isBound(this.#request.owner, this.#request.plugin)) this.#close(false);
    });
    if (this.#closed) unsubscribe();
    else this.#sourceUnsubscribe = unsubscribe;
    if (this.#closed) return;
    try {
      const unsubscribeCompletion = this.#request.subscribe(completion => this.#observe(completion));
      if (this.#closed) unsubscribeCompletion();
      else this.#completionUnsubscribe = unsubscribeCompletion;
    } catch {
      this.#completionUnsubscribe = null;
    }
    if (!this.#request.source.isBound(this.#request.owner, this.#request.plugin)) this.#close(false);
  }

  admit(operation: bigint | undefined): void {
    if (this.#closed || this.#admitted) return;
    this.#admitted = true;
    this.#operation = operation;
    if (operation !== undefined) {
      if (this.#operations.has(operation)) this.#operations.set(operation, null);
      else this.#operations.set(operation, this);
      if (this.#operations.get(operation) === this && this.#early.has(operation)) this.#terminal = true;
    }
    this.#early.clear();
    this.#finishIfTerminal();
  }

  #observe(completion: SourceWorkCompletionV1): void {
    if (this.#closed || completion.instanceId !== this.#request.owner.identity.sessionInstanceId) return;
    if (!this.#admitted) {
      if (this.#early.size < 64) this.#early.add(completion.operation);
      return;
    }
    if (this.#operation === undefined || this.#operations.get(this.#operation) !== this || completion.operation !== this.#operation) return;
    this.#terminal = true;
    this.#finishIfTerminal();
  }

  finish(): void {
    this.#finished = true;
    this.#finishIfTerminal();
  }

  #finishIfTerminal(): void {
    if (this.#finished && this.#admitted && (this.#operation === undefined || (this.#terminal && this.#operations.get(this.#operation) === this))) this.#close(true);
  }

  #close(refresh: boolean): void {
    if (this.#closed) return;
    this.#closed = true;
    this.#sourceUnsubscribe?.();
    this.#completionUnsubscribe?.();
    if (this.#operation !== undefined && this.#operations.get(this.#operation) === this) this.#operations.delete(this.#operation);
    this.#early.clear();
    this.#release();
    if (refresh && this.#request.owner.active && this.#request.source.isBound(this.#request.owner, this.#request.plugin)) this.#request.refresh();
  }
}

/** 🧾️ Starts a bounded original hold before dispatch, with no readiness restored by a watchdog. */
export function beginSourceRevisionWorkV1(request: Request): SourceRevisionWorkV1 | null {
  const release = request.source.hold(request.owner, request.plugin);
  if (release === null) return null;
  const work = new SourceWork(request, release);
  work.attach();
  return work;
}
