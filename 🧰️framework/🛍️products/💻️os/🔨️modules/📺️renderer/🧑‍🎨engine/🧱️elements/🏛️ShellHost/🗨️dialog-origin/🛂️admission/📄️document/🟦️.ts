import { latestWins } from "@semio-tech/framework";

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

export type BrowserDocumentMountOpeningV1 = Readonly<{
  clientInstanceId: string;
  scope?: Readonly<{ spaceId: string; documentId: string }>;
  instanceId: number;
}>;

export type BrowserDocumentMountSurfaceV1 = BrowserDocumentMountOpeningV1 & Readonly<{
  scope: Readonly<{ spaceId: string; documentId: string }>;
  activationGeneration: string;
  verifiedSurfaceId: string;
  revision: number;
}>;

/** 🖥️ Binds a native actor-zero mount to the exact current Shell session and retained revision. */
export function browserDocumentMountIsCurrentV1(opening: BrowserDocumentMountOpeningV1, retained: BrowserDocumentMountSurfaceV1, receipt: BrowserDocumentMountSurfaceV1): boolean {
  return receipt.instanceId === 0
    && opening.instanceId === retained.instanceId
    && opening.clientInstanceId === retained.clientInstanceId
    && retained.clientInstanceId === receipt.clientInstanceId
    && opening.scope?.spaceId === retained.scope.spaceId
    && opening.scope.documentId === retained.scope.documentId
    && retained.scope.spaceId === receipt.scope.spaceId
    && retained.scope.documentId === receipt.scope.documentId
    && retained.activationGeneration === receipt.activationGeneration
    && retained.verifiedSurfaceId === receipt.verifiedSurfaceId
    && retained.revision === receipt.revision;
}

export type DocumentOpeningOwnerV1<Plugin> = Readonly<{ plugin: Plugin; session: Readonly<{ instanceId: number }>; clientInstanceId: string }>;

export type DocumentOpeningParkV1<Owner> = Readonly<{
  key: string;
  next: Owner;
  previous: Owner | undefined;
  superseded: ReadonlyArray<readonly [string, Owner]>;
}>;

/** 🚦️ Foreground admission may name predecessors; only a committed successor may retire them. */
export function documentOpeningPredecessorsV1<Plugin, Owner extends DocumentOpeningOwnerV1<Plugin>>(
  opening: Readonly<{ runtimeKey: string; plugin: Plugin; instanceId: number }>,
  owners: ReadonlyMap<string, Owner>,
): ReadonlyArray<readonly [string, Owner]> {
  return [...owners].filter(([key, owner]) => key === opening.runtimeKey || (owner.plugin === opening.plugin && owner.session.instanceId === opening.instanceId));
}

/** 🚦️ Background openings never replace a live document or app instance. */
export function admitDocumentOpeningV1<Plugin>(
  opening: Readonly<{ runtimeKey: string; plugin: Plugin; instanceId: number; background: boolean }>,
  owners: ReadonlyMap<string, DocumentOpeningOwnerV1<Plugin>>,
): boolean {
  return !(opening.background && documentOpeningPredecessorsV1(opening, owners).length > 0);
}

/** 🧵️ Parks the successor in the owner map without retiring the predecessor until commit. */
export function parkDocumentOpeningReplacementV1<Owner extends DocumentOpeningOwnerV1<Owner["plugin"]>>(
  opening: Readonly<{ runtimeKey: string; plugin: Owner["plugin"]; instanceId: number; background: boolean }>,
  owners: Map<string, Owner>,
  next: Owner,
): DocumentOpeningParkV1<Owner> | null {
  if (!admitDocumentOpeningV1(opening, owners)) return null;
  const superseded = documentOpeningPredecessorsV1(opening, owners);
  const previous = owners.get(opening.runtimeKey);
  owners.set(opening.runtimeKey, next);
  return {
    key: opening.runtimeKey,
    next,
    previous: previous === next ? undefined : previous,
    superseded: superseded.filter(([key, owner]) => owner !== next && (key !== opening.runtimeKey || owner !== previous)),
  };
}

/** 🧹️ Failed parks restore the predecessor; committed parks return superseded owners to retire. */
export function settleDocumentOpeningReplacementV1<Owner extends { clientInstanceId: string }>(
  owners: Map<string, Owner>,
  parked: DocumentOpeningParkV1<Owner>,
  committed: boolean,
): ReadonlyArray<readonly [string, Owner]> {
  if (!committed) {
    if (owners.get(parked.key) === parked.next) {
      if (parked.previous === undefined) owners.delete(parked.key);
      else owners.set(parked.key, parked.previous);
    }
    return [];
  }
  const retire: Array<readonly [string, Owner]> = [];
  if (parked.previous !== undefined) retire.push([parked.key, parked.previous]);
  for (const pair of parked.superseded) {
    if (pair[0] === parked.key) continue;
    if (owners.get(pair[0]) === pair[1]) {
      owners.delete(pair[0]);
      retire.push(pair);
    }
  }
  return retire;
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

/** 🔁️ What replacing an attached document's content drives: the retirement of the port the document was bound through,
 * the load of the new content into the program, and the binding of a fresh port. */
export interface AttachedDocumentReplacementPortsV1 {
  readonly retired: Promise<void> | undefined;
  readonly load: () => Promise<void>;
  readonly bind: () => Promise<void>;
}

/** 🔁️ Replaces the content of a document that stays attached (a folder read-back, a tutorial restore): the port is retired,
 * the content loaded and a fresh port bound, in the document's attachment lane. A load the program refuses, faults or
 * cancels leaves the previous document exactly as it was, so the document is bound again all the same and STAYS attached —
 * the failure is the caller's to tell, never a silent detach after which nothing the program publishes is saved (ticket
 * 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, live fault F4). A replacement superseded while the port retired loads and binds
 * nothing; its successor does. */
export async function replaceAttachedDocumentV1(lane: DocumentAttachmentLaneV1, owner: string, exact: () => boolean, ports: AttachedDocumentReplacementPortsV1): Promise<void> {
  let failed: { readonly error: unknown } | null = null;
  await lane.replace(owner, exact, async () => {
    await ports.retired;
    if (!exact()) return;
    try {
      await ports.load();
    } catch (error) {
      failed = { error };
    }
    if (exact()) await ports.bind();
  });
  if (failed !== null) throw (failed as { readonly error: unknown }).error;
}

/** 🗃️ What restoring one read-back document archive drives: its load into the owning program (through the document's
 * attachment lane, answering whether the program now holds it), the program's history re-read and a full refresh of every
 * surface the program renders. */
export interface DocumentArchiveRestorePortsV1<Archive> {
  readonly load: (archive: Archive) => Promise<boolean>;
  readonly history: () => Promise<void>;
  readonly refresh: () => Promise<void>;
}

/** 🗃️ Hydrates one restored archive exactly like a fresh load (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up
 * 3): once the program holds it — and only then, a superseded or stale restore hydrates nothing — its history is re-read, so
 * the rows, edits and supersessions are the archive's, and every surface is refreshed, so no window keeps painting the
 * document the archive replaced. Answers whether the archive became the program's current document. */
export async function restoreDocumentArchiveV1<Archive>(archive: Archive, current: () => boolean, ports: DocumentArchiveRestorePortsV1<Archive>): Promise<boolean> {
  if (!(await ports.load(archive)) || !current()) return false;
  await Promise.all([ports.history(), ports.refresh()]);
  return current();
}

/** 📥️ When a folder-bound document's archive is written (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING design §22.22).
 * A local folder is the document's durable copy, so it holds the document whenever this replica's log holds events the
 * folder's archive lacks: after a batch the program published, when the bind's first read found the folder empty, after
 * batches another transport delivered were folded (once, when the folds in flight drain: N batches are one write) and
 * after a read-back merge that left this replica ahead — never because a merge taught it only what the folder already held.
 * Writes are single-flight with one trailing write, so the folder ends on the newest state. Corpus
 * `🧫️fixtures/🧫️folder-archive-persistence/🔣️.json`; law `💻️os/🧪️tests/🧪️folder-archive-persistence/🟦️.ts`. */
export class FolderArchivePersistenceV1 {
  readonly #save: () => Promise<void>;
  #folding = 0;
  #folded = false;

  /** `write` saves the program's current archive as the folder's content. */
  constructor(write: () => Promise<void>) {
    this.#save = latestWins(write);
  }

  /** 📤️ The program published a batch. */
  published(): Promise<void> {
    return this.#save();
  }

  /** 🫙️ The bind's first folder read found no archive. */
  absent(): Promise<void> {
    return this.#save();
  }

  /** 🔀️ A folder read-back was merged into the program; `ahead` counts the events the program holds that the archive lacks. */
  merged(ahead: number): Promise<void> {
    return ahead > 0 ? this.#save() : Promise.resolve();
  }

  /** 📨️ A batch another transport delivered is being folded into the program: answers the fold, after the write it led to. */
  async ingested<Folded>(fold: Promise<Folded>): Promise<Folded> {
    this.#folding += 1;
    let folded = false;
    try {
      const value = await fold;
      folded = true;
      return value;
    } finally {
      this.#folding -= 1;
      this.#folded ||= folded;
      if (this.#folding === 0 && this.#folded) {
        this.#folded = false;
        await this.#save();
      }
    }
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

/** 🔀️ What one read-back merge answered: how many events of the archive the program lacked and took, and how many events the
 * program holds that the archive lacks — or that the program cannot merge it into the document it shows (another
 * document's archive, or one whose members differ). */
export type FolderReadBackMergeV1 = Readonly<{ kind: "merged"; merged: number; ahead: number }> | Readonly<{ kind: "unmergeable" }>;

/** 🏁️ How one read-back ended for the program: `held` — its events are in the program's current document; `superseded` — a
 * newer read-back or another document took its place before it was driven or took hold; `cancelled` — the person cancelled
 * the load of a later read-back, the program keeps the document it had and stays attached to the folder it had adopted;
 * `detached` — the FIRST archive of the attachment was not taken (its load was cancelled or failed), so the folder was
 * detached. A later read-back that fails rejects instead. */
export type FolderReadBackOutcomeV1 = "held" | "superseded" | "cancelled" | "detached";

/** 📥️ What a folder read-back of an attached document drives: the merge of the archive into the document the program shows,
 * the replacement of the program's document by the archive (answering whether the program now holds it; `cancelled` says
 * whether a load's rejection is the person's cancel), the program's history re-read, a full refresh of every surface it
 * renders, and the detach of a folder whose first archive was not taken — the folder stays remembered, so it is offered
 * again; `failure` is `null` for the person's cancel. */
export interface FolderReadBackPortsV1<Archive> {
  readonly merge: (archive: Archive) => Promise<FolderReadBackMergeV1>;
  readonly load: (archive: Archive) => Promise<boolean>;
  readonly cancelled: (error: unknown) => boolean;
  readonly detach: (failure: Readonly<{ error: unknown }> | null) => Promise<void>;
  readonly history: () => Promise<void>;
  readonly refresh: () => Promise<void>;
}

/** 📥️ The read-backs of one folder attachment (design §22.22, live fault F4). The first archive read after the attach is a
 * LOAD: the folder's document replaces whatever the program held, standing where its archive says. From then on the program
 * shows the folder's document, so every later read-back — another writer's archive — MERGES: its events join the program's
 * log, an open history edit sees a base move, the program's viewed alternative stays, nothing the program holds is dropped
 * and its port stays bound; the history is re-read and the surfaces refresh only when the merge took something. An archive
 * the program cannot merge is loaded. Nothing of the program is ever written over an archive it did not take: when the FIRST
 * archive of an attachment is not loaded — the person cancelled it, or it failed — the folder is detached, stays remembered
 * (reconnecting loads it again) and the read-back ends `detached`; later events of that attachment drive nothing. Once the
 * folder is adopted, a cancelled load of a later read-back ends `cancelled`, a failed one rejects, and the document stays
 * attached and written. A folder found empty at the attach makes the program's document the folder's content. After a merge the folder is written exactly when the program is ahead of it
 * ({@link FolderArchivePersistenceV1.merged}). One read-back is driven at a time and only the newest one waits: a folder
 * holds one archive, so a read-back a newer one replaced before it was driven is skipped, and the writer rule brings its
 * events back. Corpus `🧫️fixtures/🧫️folder-read-back/🔣️.json`. */
export class FolderReadBackRouteV1<Archive> {
  readonly #persistence: FolderArchivePersistenceV1;
  readonly #readBacks = new LatestDocumentReplacementV1<Archive>();
  #adopted = false;
  #detached = false;

  constructor(persistence: FolderArchivePersistenceV1) {
    this.#persistence = persistence;
  }

  /** 🫙️ The bind's first folder read found no archive. */
  absent(): Promise<void> {
    if (this.#detached) return Promise.resolve();
    this.#adopted = true;
    return this.#persistence.absent();
  }

  /** 📥️ One archive read back from the folder; answers how it ended for the program. */
  async archive(archive: Archive, current: () => boolean, ports: FolderReadBackPortsV1<Archive>): Promise<FolderReadBackOutcomeV1> {
    let outcome: FolderReadBackOutcomeV1 = "superseded";
    await this.#readBacks.replace(archive, async (candidate) => {
      outcome = await this.#drive(candidate, current, ports);
    });
    return outcome;
  }

  async #drive(archive: Archive, current: () => boolean, ports: FolderReadBackPortsV1<Archive>): Promise<FolderReadBackOutcomeV1> {
    if (this.#detached) return "superseded";
    if (this.#adopted) {
      const merge = await ports.merge(archive);
      if (merge.kind === "merged") {
        if (!current()) return "superseded";
        if (merge.merged > 0) await Promise.all([ports.history(), ports.refresh()]);
        await this.#persistence.merged(merge.ahead);
        return current() ? "held" : "superseded";
      }
    }
    const declined: { load: Readonly<{ error: unknown; cancelled: boolean }> | null } = { load: null };
    const load = (value: Archive): Promise<boolean> =>
      ports.load(value).catch((error: unknown) => {
        declined.load = { error, cancelled: ports.cancelled(error) };
        return false;
      });
    const restored = await restoreDocumentArchiveV1(archive, current, { load, history: ports.history, refresh: ports.refresh });
    if (declined.load !== null) {
      if (!this.#adopted) {
        this.#detached = true;
        await ports.detach(declined.load.cancelled ? null : { error: declined.load.error });
        return "detached";
      }
      if (declined.load.cancelled) return "cancelled";
      throw declined.load.error;
    }
    if (!restored) return "superseded";
    this.#adopted = true;
    return "held";
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
