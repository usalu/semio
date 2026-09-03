//#region 🧬️Contract
export type NumericIndexGrant = { readonly maxItems: number; readonly maxBytes: number };
export type NumericIndexStep<V> =
  | { readonly kind: "blocked" | "pending" | "ready" | "complete"; readonly items: number; readonly bytes: number }
  | { readonly kind: "retired"; readonly value: V; readonly items: number; readonly bytes: number }
  | { readonly kind: "rejected"; readonly reason: "ordinal-exhausted"; readonly items: number; readonly bytes: number };
export type NumericIndexOrdinal = { readonly high: number; readonly low: number };
export type NumericIndexReadStep<V> =
  | { readonly kind: "blocked" | "pending" | "complete"; readonly items: number; readonly bytes: number }
  | { readonly kind: "value"; readonly id: number; readonly ordinal: NumericIndexOrdinal; readonly value: V; readonly items: number; readonly bytes: number };
type Key = NumericIndexOrdinal;
type Entry<V> = { refs: number; readonly id: number; readonly ordinal: Key; payload: { value: V } | null };
type Node<V> = { refs: number; readonly key: Key; readonly height: number; left: Node<V> | null; right: Node<V> | null; entry: Entry<V> | null };
type Frame<V> = { node: Node<V> | null; readonly right: boolean; next: Frame<V> | null };
type Owners<V> = { ids: Node<V> | null; order: Node<V> | null; readonly size: number; readonly next: Key };
type Task<V> = { readonly kind: "node"; readonly node: Node<V> } | { readonly kind: "entry"; readonly entry: Entry<V> } | { readonly kind: "frame"; readonly frame: Frame<V> } | { readonly kind: "allocation"; readonly allocation: TreeAllocation<V> };
type Queue<V> = { task: Task<V> | null; next: Queue<V> | null };
let queueTask: <V>(retirement: NumericIndexRetirement<V>, task: Task<V> | null) => void;
let ownersOf: <V>(index: NumericIndex<V>) => Owners<V>;
let closeOwner: <V>(index: NumericIndex<V>, retirement: NumericIndexRetirement<V>) => void;
let beginEdit: <V>(source: NumericIndex<V>, key: Key, input: { value: V } | null) => NumericIndexEdit<V>;
let beginRead: <V>(source: NumericIndex<V>, key: Key | null, ordered: boolean) => NumericIndexReader<V>;
const MIN_STEP_BYTES = 256;
const idle = (kind: "blocked" | "complete" | "ready"): NumericIndexStep<never> => ({ kind, items: 0, bytes: 0 });
const pending = (bytes: number): NumericIndexStep<never> => ({ kind: "pending", items: 1, bytes });
const admitted = (grant: NumericIndexGrant): boolean => grant.maxItems >= 1 && grant.maxBytes >= MIN_STEP_BYTES;

function idKey(id: number): Key {
  if (!Number.isSafeInteger(id) || id < 0) throw new RangeError("Numeric index IDs must be nonnegative safe integers");
  return { high: 0, low: id === 0 ? 0 : id };
}

function compare(a: Key, b: Key): number {
  return a.high < b.high ? -1 : a.high > b.high ? 1 : a.low < b.low ? -1 : a.low > b.low ? 1 : 0;
}

function nextOrdinal(key: Key): Key {
  if (key.low < Number.MAX_SAFE_INTEGER) return { high: key.high, low: key.low + 1 };
  if (key.high === Number.MAX_SAFE_INTEGER) throw new RangeError("Numeric index insertion ordinal exhausted");
  return { high: key.high + 1, low: 0 };
}

function retain<V>(node: Node<V> | null): Node<V> | null {
  if (node) { checkReferences(node, 1); node.refs++; }
  return node;
}

function checkReferences(owner: { refs: number }, count: number): void { if (!Number.isSafeInteger(owner.refs) || owner.refs < 1 || owner.refs > Number.MAX_SAFE_INTEGER - count) throw new RangeError("Numeric index reference capacity exhausted"); }

function valueEntry<V>(node: Node<V>): Entry<V> {
  if (!node.entry || node.refs < 1) throw new Error("Retired numeric index node");
  return node.entry;
}

function entryValue<V>(entry: Entry<V>): V {
  if (!entry.payload) throw new Error("Retired numeric index payload");
  return entry.payload.value;
}
//#endregion 🧬️Contract

//#region ♻️Retirement
export class NumericIndexRetirement<V> {
  #head: Queue<V> | null = null;
  constructor() { Object.freeze(this); }

  static { queueTask = <T>(owner: NumericIndexRetirement<T>, task: Task<T> | null) => owner.#enqueue(task); }

  #enqueue(task: Task<V> | null): void {
    if (task) this.#head = { task, next: this.#head };
  }

  terminalIsEmpty(): boolean { return this.#head === null; }

  advance(grant: NumericIndexGrant): NumericIndexStep<V> {
    if (!this.#head) return idle("complete");
    if (!admitted(grant)) return idle("blocked");
    const cell = this.#head;
    this.#head = cell.next;
    cell.next = null;
    const task = cell.task!;
    cell.task = null;
    if (task.kind === "allocation") { const bytes = task.allocation.closeStep(this); if (!task.allocation.terminalIsEmpty()) this.#enqueue(task); return pending(bytes + 24); }
    if (task.kind === "frame") {
      const frame = task.frame;
      const next = frame.next;
      frame.node = null;
      frame.next = null;
      if (next) this.#enqueue({ kind: "frame", frame: next });
      return pending(64);
    }
    if (task.kind === "entry") {
      const entry = task.entry;
      if (--entry.refs < 0) throw new Error("Numeric index entry released twice");
      if (entry.refs !== 0) return pending(24);
      const value = entryValue(entry);
      entry.payload = null;
      return { kind: "retired", value, items: 1, bytes: 40 };
    }
    const node = task.node;
    if (--node.refs < 0) throw new Error("Numeric index node released twice");
    if (node.refs !== 0) return pending(24);
    const left = node.left;
    const right = node.right;
    const entry = node.entry;
    node.left = null;
    node.right = null;
    node.entry = null;
    if (entry) this.#enqueue({ kind: "entry", entry });
    if (right) this.#enqueue({ kind: "node", node: right });
    if (left) this.#enqueue({ kind: "node", node: left });
    return pending(128);
  }
}
//#endregion ♻️Retirement

//#region 🌳️PersistentTree
type AllocationChild<V> = Node<V> | number | null;
type AllocationNode<V> = { key: Key; entry: Entry<V>; left: AllocationChild<V>; right: AllocationChild<V>; built: Node<V> | null; allocated: boolean };
type Reservation<V> = { owner: Node<V> | Entry<V>; count: number };

function allocationNode<V>(key: Key, entry: Entry<V>, left: AllocationChild<V>, right: AllocationChild<V>): AllocationNode<V> { return { key, entry, left, right, built: null, allocated: false }; }

class TreeAllocation<V> {
  #reservations: Reservation<V>[] = [];
  #slot = 0;
  #scan = 0;
  #target: Node<V> | Entry<V> | null = null;
  #reserved = false;
  #allocated = 0;
  #closed = false;
  private readonly nodes: AllocationNode<V>[];
  constructor(nodes: AllocationNode<V>[]) { this.nodes = nodes; }

  advance(): number {
    if (!this.#reserved) {
      if (this.#target) {
        const reservation = this.#reservations[this.#scan];
        if (!reservation) { this.#reservations.push({ owner: this.#target, count: 1 }); this.#target = null; this.#scan = 0; return 40; }
        if (reservation.owner === this.#target) { reservation.count++; this.#target = null; this.#scan = 0; } else this.#scan++;
        return 24;
      }
      if (this.#slot < this.nodes.length * 3) {
        const spec = this.nodes[Math.floor(this.#slot / 3)]!;
        const value = this.#slot % 3 === 0 ? spec.entry : this.#slot % 3 === 1 ? spec.left : spec.right;
        this.#slot++; if (value !== null && typeof value !== "number") this.#target = value;
        return 32;
      }
      for (const reservation of this.#reservations) checkReferences(reservation.owner, reservation.count);
      for (const reservation of this.#reservations) reservation.owner.refs += reservation.count;
      this.#reserved = true;
      return 32 + this.#reservations.length * 16;
    }
    if (this.#allocated === this.nodes.length) return 0;
    const spec = this.nodes[this.#allocated]!;
    const take = (child: AllocationChild<V>): Node<V> | null => { if (typeof child !== "number") return child; const source = this.nodes[child]!; const result = source.built; source.built = null; return result; };
    const left = take(spec.left); const right = take(spec.right);
    spec.built = { refs: 1, key: spec.key, height: 1 + Math.max(left?.height ?? 0, right?.height ?? 0), left, right, entry: spec.entry };
    spec.allocated = true; this.#allocated++;
    return 96;
  }

  get ready(): boolean { return this.#reserved && this.#allocated === this.nodes.length; }
  get reservationsReady(): boolean { return this.#target === null && this.#slot === this.nodes.length * 3; }
  takeRoot(): Node<V> { if (!this.ready) throw new Error("Numeric allocation is incomplete"); const last = this.nodes[this.nodes.length - 1]!; if (!last.built) throw new Error("Numeric allocation already transferred"); const root = last.built; last.built = null; return root; }

  closeStep(retirement: NumericIndexRetirement<V>): number {
    this.#target = null;
    const spec = this.nodes.pop();
    if (spec) {
      if (spec.built) queueTask(retirement, { kind: "node", node: spec.built });
      if (this.#reserved && !spec.allocated) {
        queueTask(retirement, { kind: "entry", entry: spec.entry });
        if (spec.left !== null && typeof spec.left !== "number") queueTask(retirement, { kind: "node", node: spec.left });
        if (spec.right !== null && typeof spec.right !== "number") queueTask(retirement, { kind: "node", node: spec.right });
      }
      spec.built = null; return 136;
    }
    if (this.#reservations.length) { this.#reservations.pop(); return 24; }
    this.#closed = true; return 16;
  }
  terminalIsEmpty(): boolean { return this.#closed; }
}

function balancedAllocation<V>(key: Key, entry: Entry<V>, left: Node<V> | null, right: Node<V> | null): TreeAllocation<V> {
  const balance = (left?.height ?? 0) - (right?.height ?? 0);
  if (balance > 1 && left) {
    if ((left.left?.height ?? 0) >= (left.right?.height ?? 0)) return new TreeAllocation([allocationNode(key, entry, left.right, right), allocationNode(left.key, valueEntry(left), left.left, 0)]);
    const pivot = left.right!;
    return new TreeAllocation([allocationNode(left.key, valueEntry(left), left.left, pivot.left), allocationNode(key, entry, pivot.right, right), allocationNode(pivot.key, valueEntry(pivot), 0, 1)]);
  }
  if (balance < -1 && right) {
    if ((right.right?.height ?? 0) >= (right.left?.height ?? 0)) return new TreeAllocation([allocationNode(key, entry, left, right.left), allocationNode(right.key, valueEntry(right), 0, right.right)]);
    const pivot = right.left!;
    return new TreeAllocation([allocationNode(key, entry, left, pivot.left), allocationNode(right.key, valueEntry(right), pivot.right, right.right), allocationNode(pivot.key, valueEntry(pivot), 0, 1)]);
  }
  return new TreeAllocation([allocationNode(key, entry, left, right)]);
}

class TreeEdit<V> {
  #scan: Node<V> | null;
  #path: Frame<V> | null = null;
  #successorPath: Frame<V> | null = null;
  #target: Node<V> | null = null;
  #replacement: Entry<V> | null = null;
  #replacementKey: Key | null = null;
  #work: Node<V> | null = null;
  #phase: "search" | "successor" | "successor-rebuild" | "rebuild" | "ready" | "closed" = "search";
  #allocation: TreeAllocation<V> | null = null;
  private readonly key: Key;
  private readonly entry: Entry<V> | null;
  private readonly retirement: NumericIndexRetirement<V>;

  constructor(root: Node<V> | null, key: Key, entry: Entry<V> | null, retirement: NumericIndexRetirement<V>) {
    this.key = key;
    this.entry = entry;
    this.retirement = retirement;
    this.#scan = root;
  }

  #replaceWork(node: Node<V> | null): void {
    if (this.#work) queueTask(this.retirement, { kind: "node", node: this.#work });
    this.#work = node;
  }

  advance(): number {
    if (this.#phase === "ready" || this.#phase === "closed") return 0;
    if (this.#allocation) {
      if (!this.#allocation.ready) return this.#allocation.advance();
      const allocation = this.#allocation; this.#allocation = null;
      this.#replaceWork(allocation.takeRoot()); queueTask(this.retirement, { kind: "allocation", allocation }); return 64;
    }
    if (this.#phase === "search") {
      const node = this.#scan;
      if (!node) {
        if (this.entry) this.#allocation = balancedAllocation(this.key, this.entry, null, null);
        this.#phase = "rebuild";
      } else {
        const order = compare(this.key, node.key);
        if (order !== 0) {
          this.#path = { node, right: order > 0, next: this.#path };
          this.#scan = order > 0 ? node.right : node.left;
        } else if (this.entry) {
          this.#allocation = balancedAllocation(this.key, this.entry, node.left, node.right);
          this.#scan = null;
          this.#phase = "rebuild";
        } else if (!node.left || !node.right) {
          this.#work = retain(node.left ?? node.right);
          this.#scan = null;
          this.#phase = "rebuild";
        } else {
          this.#target = node;
          this.#scan = node.right;
          this.#phase = "successor";
        }
      }
      return 112;
    }
    if (this.#phase === "successor") {
      const node = this.#scan!;
      if (node.left) {
        this.#successorPath = { node, right: false, next: this.#successorPath };
        this.#scan = node.left;
      } else {
        this.#replacement = valueEntry(node);
        this.#replacementKey = node.key;
        this.#work = retain(node.right);
        this.#scan = null;
        this.#phase = "successor-rebuild";
      }
      return 48;
    }
    if (this.#phase === "successor-rebuild") {
      const frame = this.#successorPath;
      if (frame) {
        const node = frame.node!;
        this.#successorPath = frame.next;
        frame.next = null; frame.node = null;
        this.#allocation = balancedAllocation(node.key, valueEntry(node), this.#work, node.right);
      } else {
        this.#allocation = balancedAllocation(this.#replacementKey!, this.#replacement!, this.#target!.left, this.#work);
        this.#target = null; this.#replacement = null; this.#replacementKey = null;
        this.#phase = "rebuild";
      }
      return 240;
    }
    const frame = this.#path;
    if (frame) {
      const node = frame.node!;
      this.#path = frame.next;
      frame.next = null; frame.node = null;
      this.#allocation = balancedAllocation(node.key, valueEntry(node), frame.right ? node.left : this.#work, frame.right ? this.#work : node.right);
    } else this.#phase = "ready";
    return 240;
  }

  get ready(): boolean { return this.#phase === "ready"; }

  takeRoot(): Node<V> | null {
    if (!this.ready) throw new Error("Numeric tree edit is not ready");
    const root = this.#work;
    this.#work = null;
    this.#phase = "closed";
    return root;
  }

  closeInto(retirement: NumericIndexRetirement<V>): void {
    if (this.#work) queueTask(retirement, { kind: "node", node: this.#work });
    if (this.#path) queueTask(retirement, { kind: "frame", frame: this.#path });
    if (this.#successorPath) queueTask(retirement, { kind: "frame", frame: this.#successorPath });
    if (this.#allocation) queueTask(retirement, { kind: "allocation", allocation: this.#allocation });
    this.#allocation = null;
    this.#work = null; this.#path = null; this.#successorPath = null;
    this.#target = null; this.#replacement = null; this.#replacementKey = null; this.#scan = null;
    this.#phase = "closed";
  }
}
//#endregion 🌳️PersistentTree

//#region 🗂️Index
let adopt: <V>(owners: Owners<V>) => NumericIndex<V>;

export class NumericIndex<V> implements Iterable<readonly [number, V]> {
  #owners: Owners<V> | null;

  private constructor(owners: Owners<V>) { this.#owners = owners; Object.freeze(this); }

  static {
    adopt = <T>(owners: Owners<T>) => new NumericIndex<T>(owners);
    ownersOf = <T>(index: NumericIndex<T>) => index.#live();
    closeOwner = <T>(index: NumericIndex<T>, retirement: NumericIndexRetirement<T>) => index.#closeInto(retirement);
  }

  static empty<T>(firstOrdinal: NumericIndexOrdinal = { high: 0, low: 0 }): NumericIndex<T> {
    idKey(firstOrdinal.high); idKey(firstOrdinal.low);
    return new NumericIndex({ ids: null, order: null, size: 0, next: { high: firstOrdinal.high, low: firstOrdinal.low } });
  }

  #live(): Owners<V> {
    if (!this.#owners) throw new Error("Numeric index owner is closed");
    return this.#owners;
  }

  get size(): number { return this.#live().size; }

  terminalIsEmpty(): boolean { return this.#owners === null; }

  nextOrdinal(): NumericIndexOrdinal { const next = this.#live().next; return { high: next.high, low: next.low }; }

  capture(): NumericIndex<V> {
    const source = this.#live();
    this.#checkCaptureCapacity();
    return adopt({ ids: retain(source.ids), order: retain(source.order), size: source.size, next: source.next });
  }

  #checkCaptureCapacity(): void {
    const source = this.#live();
    if (source.ids) checkReferences(source.ids, source.ids === source.order ? 2 : 1);
    if (source.order && source.order !== source.ids) checkReferences(source.order, 1);
  }
  assertCaptureCapacity(): void { this.#checkCaptureCapacity(); }

  get(id: number): V | undefined {
    const key = idKey(id);
    let node = this.#live().ids;
    while (node) {
      const order = compare(key, node.key);
      if (order === 0) return entryValue(valueEntry(node));
      node = order > 0 ? node.right : node.left;
    }
    return undefined;
  }

  *[Symbol.iterator](): IterableIterator<readonly [number, V]> {
    let node = this.#live().order;
    let path: Frame<V> | null = null;
    while (node || path) {
      this.#live();
      if (node) { path = { node, right: false, next: path }; node = node.left; continue; }
      const frame: Frame<V> = path!;
      path = frame.next;
      node = frame.node!;
      const entry = valueEntry(node);
      yield [entry.id, entryValue(entry)];
      this.#live();
      node = node.right;
    }
  }

  beginSet(id: number, value: V): NumericIndexEdit<V> {
    const key = idKey(id);
    return beginEdit(this.capture(), key, { value });
  }

  beginRemove(id: number): NumericIndexEdit<V> {
    const key = idKey(id);
    return beginEdit(this.capture(), key, null);
  }

  beginRead(): NumericIndexReader<V> { return beginRead(this.capture(), null, true); }

  beginSortedRead(): NumericIndexReader<V> { return beginRead(this.capture(), null, false); }

  beginLookup(id: number): NumericIndexReader<V> {
    const key = idKey(id);
    return beginRead(this.capture(), key, false);
  }

  beginClose(): NumericIndexRetirement<V> {
    const retirement = new NumericIndexRetirement<V>();
    this.#closeInto(retirement);
    return retirement;
  }

  #closeInto(retirement: NumericIndexRetirement<V>): void {
    const owners = this.#owners;
    this.#owners = null;
    if (owners?.ids) queueTask(retirement, { kind: "node", node: owners.ids });
    if (owners?.order) queueTask(retirement, { kind: "node", node: owners.order });
  }

}
//#endregion 🗂️Index

//#region 📖️Reader
export class NumericIndexReader<V> {
  #source: NumericIndex<V> | null;
  #scan: Node<V> | null;
  #path: Frame<V> | null = null;
  #complete = false;
  private readonly key: Key | null;

  static { beginRead = <T>(source: NumericIndex<T>, key: Key | null, ordered: boolean) => new NumericIndexReader(source, key, ordered); }

  private constructor(source: NumericIndex<V>, key: Key | null, ordered: boolean) {
    this.key = key;
    this.#source = source;
    const owners = ownersOf(source);
    this.#scan = ordered ? owners.order : owners.ids;
    Object.freeze(this);
  }

  advance(grant: NumericIndexGrant): NumericIndexReadStep<V> {
    if (!this.#source) throw new Error("Numeric index reader is closed");
    if (this.#complete) return { kind: "complete", items: 0, bytes: 0 };
    if (!admitted(grant)) return { kind: "blocked", items: 0, bytes: 0 };
    if (this.key) {
      const node = this.#scan;
      if (!node) { this.#complete = true; return { kind: "complete", items: 1, bytes: 16 }; }
      const order = compare(this.key, node.key);
      if (order !== 0) {
        this.#scan = order > 0 ? node.right : node.left;
        return { kind: "pending", items: 1, bytes: 32 };
      }
      this.#scan = null; this.#complete = true;
      const entry = valueEntry(node);
      return { kind: "value", id: entry.id, ordinal: { ...entry.ordinal }, value: entryValue(entry), items: 1, bytes: 64 };
    }
    if (this.#scan) {
      this.#path = { node: this.#scan, right: false, next: this.#path };
      this.#scan = this.#scan.left;
      return { kind: "pending", items: 1, bytes: 64 };
    }
    const frame = this.#path;
    if (!frame) { this.#complete = true; return { kind: "complete", items: 1, bytes: 16 }; }
    const node = frame.node!;
    this.#path = frame.next;
    frame.next = null; frame.node = null;
    this.#scan = node.right;
    const entry = valueEntry(node);
    return { kind: "value", id: entry.id, ordinal: { ...entry.ordinal }, value: entryValue(entry), items: 1, bytes: 80 };
  }

  terminalIsEmpty(): boolean { return this.#source === null && this.#path === null && this.#scan === null; }

  beginClose(): NumericIndexRetirement<V> {
    if (!this.#source) throw new Error("Numeric index reader is already closed");
    const retirement = new NumericIndexRetirement<V>();
    closeOwner(this.#source, retirement);
    if (this.#path) queueTask(retirement, { kind: "frame", frame: this.#path });
    this.#source = null; this.#scan = null; this.#path = null; this.#complete = true;
    return retirement;
  }
}
//#endregion 📖️Reader

//#region ✏️Edit
export class NumericIndexEdit<V> {
  #input: { value: V } | null;
  #source: NumericIndex<V> | null;
  #scan: Node<V> | null;
  #old: Entry<V> | null = null;
  #entry: Entry<V> | null = null;
  #tree: TreeEdit<V> | null = null;
  #ids: Node<V> | null = null;
  #order: Node<V> | null = null;
  #result: NumericIndex<V> | null = null;
  #retirement: NumericIndexRetirement<V> | null = new NumericIndexRetirement<V>();
  #phase: "lookup" | "entry" | "ids" | "order" | "publish" | "ready" | "rejected" | "closed" = "lookup";
  private readonly key: Key;

  static { beginEdit = <T>(source: NumericIndex<T>, key: Key, input: { value: T } | null) => new NumericIndexEdit<T>(source, key, input); }

  private constructor(source: NumericIndex<V>, key: Key, input: { value: V } | null) {
    this.key = key;
    this.#source = source;
    this.#scan = ownersOf(source).ids;
    this.#input = input;
    Object.freeze(this);
  }

  advance(grant: NumericIndexGrant): NumericIndexStep<V> {
    if (this.#phase === "closed") return idle("complete");
    if (!admitted(grant)) return idle("blocked");
    const retirement = this.#retirement!;
    if (!retirement.terminalIsEmpty()) return retirement.advance(grant);
    if (this.#phase === "ready") return idle("ready");
    if (this.#phase === "rejected") return { kind: "rejected", reason: "ordinal-exhausted", items: 0, bytes: 0 };
    const source = ownersOf(this.#source!);
    if (this.#phase === "lookup") {
      const node = this.#scan;
      if (!node) this.#phase = "entry";
      else {
        const order = compare(this.key, node.key);
        if (order === 0) { this.#old = valueEntry(node); this.#scan = null; this.#phase = "entry"; }
        else this.#scan = order > 0 ? node.right : node.left;
      }
      return pending(32);
    }
    if (this.#phase === "entry") {
      if (!this.#input && !this.#old) { this.#result = this.#source!.capture(); this.#phase = "ready"; return pending(32); }
      if (this.#input) {
        if (!this.#old && source.next.high === Number.MAX_SAFE_INTEGER && source.next.low === Number.MAX_SAFE_INTEGER) {
          this.#phase = "rejected";
          return { kind: "rejected", reason: "ordinal-exhausted", items: 1, bytes: 16 };
        }
        this.#entry = { refs: 1, id: this.key.low, ordinal: this.#old?.ordinal ?? source.next, payload: this.#input };
        this.#input = null;
      }
      this.#tree = new TreeEdit(source.ids, this.key, this.#entry, retirement);
      this.#phase = "ids";
      return pending(96);
    }
    if (this.#phase === "ids" || this.#phase === "order") {
      if (!this.#tree!.ready) return pending(this.#tree!.advance());
      const root = this.#tree!.takeRoot();
      this.#tree = null;
      if (this.#phase === "ids") {
        this.#ids = root;
        this.#tree = new TreeEdit(source.order, this.#old?.ordinal ?? this.#entry!.ordinal, this.#entry, retirement);
        this.#phase = "order";
      } else { this.#order = root; this.#phase = "publish"; }
      return pending(64);
    }
    const inserted = this.#entry !== null && this.#old === null;
    this.#result = adopt({ ids: this.#ids, order: this.#order, size: source.size + (inserted ? 1 : this.#entry ? 0 : -1), next: inserted ? nextOrdinal(source.next) : source.next });
    this.#ids = null; this.#order = null;
    this.#phase = "ready";
    return pending(64);
  }

  takeResult(): NumericIndex<V> | null {
    if (this.#phase !== "ready") return null;
    const result = this.#result;
    this.#result = null;
    return result;
  }

  terminalIsEmpty(): boolean { return this.#phase === "closed" && this.#retirement === null; }

  beginClose(): NumericIndexRetirement<V> {
    if (this.#phase === "closed") throw new Error("Numeric index edit is already closed");
    const retirement = this.#retirement!;
    this.#retirement = null;
    if (this.#source) closeOwner(this.#source, retirement);
    if (this.#result) closeOwner(this.#result, retirement);
    if (this.#ids) queueTask(retirement, { kind: "node", node: this.#ids });
    if (this.#order) queueTask(retirement, { kind: "node", node: this.#order });
    if (this.#entry) queueTask(retirement, { kind: "entry", entry: this.#entry });
    if (this.#input) queueTask(retirement, { kind: "entry", entry: { refs: 1, id: this.key.low, ordinal: this.key, payload: this.#input } });
    this.#tree?.closeInto(retirement);
    this.#source = null; this.#result = null; this.#ids = null; this.#order = null;
    this.#entry = null; this.#input = null; this.#tree = null; this.#scan = null; this.#old = null;
    this.#phase = "closed";
    return retirement;
  }
}
//#endregion ✏️Edit

//#region 🧪️PrivateReferenceProbe
function numericReferenceSaturation(): readonly { name: string; rejected: boolean; preserved: boolean }[] {
  const rows: { name: string; rejected: boolean; preserved: boolean }[] = [];
  for (const name of ["capture-ids", "capture-order", "capture-shadowed-checker"]) {
    const entry: Entry<string> = { refs: 2, id: 1, ordinal: { high: 0, low: 0 }, payload: { value: name } };
    const ids: Node<string> = { refs: 1, key: { high: 0, low: 1 }, height: 1, left: null, right: null, entry };
    const order: Node<string> = { ...ids, key: entry.ordinal };
    const index = adopt({ ids, order, size: 1, next: { high: 0, low: 1 } });
    const target = name === "capture-ids" ? ids : order;
    target.refs = Number.MAX_SAFE_INTEGER;
    let captured: NumericIndex<string> | null = null;
    let rejected = false;
    try { if (name === "capture-shadowed-checker") Object.defineProperty(index, "assertCaptureCapacity", { value: () => {} }); captured = index.capture(); } catch { rejected = true; }
    const preserved = target.refs === Number.MAX_SAFE_INTEGER && (target === ids ? order : ids).refs === 1;
    ids.refs = captured ? 2 : 1; order.refs = captured ? 2 : 1;
    if (captured) { const close = captured.beginClose(); while (!close.terminalIsEmpty()) close.advance({ maxItems: 1, maxBytes: 256 }); }
    const close = index.beginClose(); while (!close.terminalIsEmpty()) close.advance({ maxItems: 1, maxBytes: 256 });
    rows.push({ name, rejected, preserved });
  }
  for (const name of ["allocation-entry", "allocation-left", "allocation-right", "allocation-duplicate-owner"]) {
    const key = { high: 0, low: 1 };
    const entry: Entry<string> = { refs: 1, id: 1, ordinal: key, payload: { value: name } };
    const leaf = (id: number): Node<string> => ({ refs: 1, key: { high: 0, low: id }, height: 1, left: null, right: null, entry: { refs: 1, id, ordinal: { high: 0, low: id }, payload: { value: String(id) } } });
    const left = leaf(0); const right = leaf(2);
    const plan = new TreeAllocation([allocationNode(key, entry, left, name === "allocation-duplicate-owner" ? left : right)]);
    while (!plan.reservationsReady) { const bytes = plan.advance(); if (bytes > 256) throw new Error("Reference collection exceeded its grant"); }
    const owners = [entry, left, right];
    const target = name === "allocation-entry" ? entry : name === "allocation-right" ? right : left;
    target.refs = Number.MAX_SAFE_INTEGER - (name === "allocation-duplicate-owner" ? 1 : 0);
    const before = owners.map(owner => owner.refs);
    let rejected = false;
    try { plan.advance(); } catch { rejected = true; }
    const preserved = owners.every((owner, index) => owner.refs === before[index]);
    if (rejected) target.refs = 1;
    const retirement = new NumericIndexRetirement<string>();
    queueTask(retirement, { kind: "entry", entry }); queueTask(retirement, { kind: "node", node: left }); queueTask(retirement, { kind: "node", node: right }); queueTask(retirement, { kind: "allocation", allocation: plan });
    if (!rejected) { entry.refs = 2; left.refs = name === "allocation-duplicate-owner" ? 3 : 2; right.refs = name === "allocation-duplicate-owner" ? 1 : 2; }
    while (!retirement.terminalIsEmpty()) { const step = retirement.advance({ maxItems: 1, maxBytes: 256 }); if (step.bytes > 256) throw new Error("Reference retirement exceeded its grant"); }
    rows.push({ name, rejected, preserved });
  }
  return rows;
}
//#endregion 🧪️PrivateReferenceProbe
