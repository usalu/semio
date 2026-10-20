/** 🗂️ Owns the bounded retained numeric table and sibling-key containers shared by retained graph validation and patch transactions. */
import { NumericIndex, type NumericIndexEdit, type NumericIndexReader, type NumericIndexRetirement, type NumericIndexGrant } from "@semio-tech/value-ts/ordered/numeric";

export type Program<T> = Generator<number, T, void>;

class Table<V> {
  #index: NumericIndex<V> | null;
  #edit: NumericIndexEdit<V> | null = null;
  #reader: NumericIndexReader<V> | null = null;
  #retirement: NumericIndexRetirement<V> | null = null;
  #old: NumericIndex<V> | null = null;
  readonly grant: () => NumericIndexGrant;
  private readonly retired: (value: V) => void;

  constructor(index: NumericIndex<V>, grant: () => NumericIndexGrant, retired: (value: V) => void = () => {}) { this.grant = grant; this.retired = retired; this.#index = index; }

  get index(): NumericIndex<V> { if (!this.#index) throw new Error("Retained table owner is closed"); return this.#index; }
  get size(): number { return this.index.size; }

  *#drain(): Program<void> {
    while (this.#retirement) {
      const step = this.#retirement.advance(this.grant());
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "complete") this.#retirement = null;
      yield step.bytes;
    }
  }

  *lookup(id: number): Program<V | undefined> {
    this.#reader = this.index.beginLookup(id);
    let result: V | undefined;
    for (;;) {
      const step = this.#reader.advance(this.grant());
      if (step.kind === "value") result = step.value;
      yield step.bytes;
      if (step.kind === "complete") break;
    }
    this.#retirement = this.#reader.beginClose(); this.#reader = null;
    yield* this.#drain();
    return result;
  }

  *entries(): Generator<number | readonly [number, V], void, void> {
    this.#reader = this.index.beginRead();
    for (;;) {
      const step = this.#reader.advance(this.grant());
      if (step.kind === "value") yield [step.id, step.value];
      else yield step.bytes;
      if (step.kind === "complete") break;
    }
    this.#retirement = this.#reader.beginClose(); this.#reader = null;
    yield* this.#drain();
  }

  *set(id: number, value: V): Program<void> { yield* this.#change(this.index.beginSet(id, value)); }
  *remove(id: number): Program<void> { yield* this.#change(this.index.beginRemove(id)); }

  *#change(edit: NumericIndexEdit<V>): Program<void> {
    this.#edit = edit;
    for (;;) {
      const step = edit.advance(this.grant());
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "rejected") throw new RangeError("Retained UI insertion ordinal exhausted");
      yield step.bytes;
      if (step.kind === "ready") break;
    }
    this.#old = this.#index;
    this.#index = edit.takeResult();
    this.#retirement = edit.beginClose(); this.#edit = null;
    yield* this.#drain();
    this.#retirement = this.#old!.beginClose(); this.#old = null;
    yield* this.#drain();
  }

  take(): NumericIndex<V> {
    if (!this.#index || this.#edit || this.#reader || this.#retirement || this.#old) throw new Error("Retained table is not transferable");
    const result = this.#index; this.#index = null;
    return result;
  }

  closeStep(grant: NumericIndexGrant): { readonly complete: boolean; readonly bytes: number } {
    if (this.#retirement) {
      const step = this.#retirement.advance(grant);
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "complete") this.#retirement = null;
      return { complete: false, bytes: step.bytes };
    }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return { complete: false, bytes: 64 }; }
    if (this.#edit) { this.#retirement = this.#edit.beginClose(); this.#edit = null; return { complete: false, bytes: 128 }; }
    if (this.#old) { this.#retirement = this.#old.beginClose(); this.#old = null; return { complete: false, bytes: 64 }; }
    if (this.#index) { this.#retirement = this.#index.beginClose(); this.#index = null; return { complete: false, bytes: 64 }; }
    return { complete: true, bytes: 0 };
  }
}

export { Table as RetainedUiNumericTable };

type KeyCell = { key: string | null; collision: KeyCell | null; ownedNext: KeyCell | null };

class SiblingKeys {
  #table: Table<KeyCell> | null = null;
  #owned: KeyCell | null = null;
  private readonly grant: () => NumericIndexGrant;
  constructor(grant: () => NumericIndexGrant) { this.grant = grant; }

  *insert(key: string): Program<boolean> {
    this.#table ??= new Table(NumericIndex.empty<KeyCell>(), this.grant);
    let hash = 0x811c9dc5;
    let offset = 0;
    yield 32;
    while (offset < key.length) {
      let work = 0;
      while (offset < key.length && work + 8 <= this.grant().maxBytes) {
        const unit = key.charCodeAt(offset++);
        hash = Math.imul(hash ^ (unit & 255), 0x01000193) >>> 0;
        hash = Math.imul(hash ^ (unit >>> 8), 0x01000193) >>> 0;
        work += 8;
      }
      yield work;
    }
    const head = yield* this.#table.lookup(hash);
    let cell = head;
    while (cell) {
      const existing = cell.key!;
      yield 16;
      if (existing.length === key.length) {
        offset = 0;
        let mismatch = false;
        while (offset < key.length && !mismatch) {
          let work = 0;
          while (offset < key.length && work + 8 <= this.grant().maxBytes) {
            const left = key.charCodeAt(offset);
            const right = existing.charCodeAt(offset);
            work += 8;
            if (left !== right) { mismatch = true; break; }
            offset++;
          }
          yield work;
        }
        if (!mismatch) return true;
      }
      cell = cell.collision ?? undefined;
    }
    const inserted = { key, collision: head ?? null, ownedNext: this.#owned };
    this.#owned = inserted;
    yield 48;
    yield* this.#table.set(hash, inserted);
    return false;
  }

  *clear(): Program<void> { for (;;) { const step = this.closeStep(this.grant()); yield step.bytes; if (step.complete) return; } }

  closeStep(grant: NumericIndexGrant): { readonly complete: boolean; readonly bytes: number } {
    if (this.#table) {
      const step = this.#table.closeStep(grant);
      if (step.complete) this.#table = null;
      return { complete: false, bytes: step.bytes };
    }
    if (this.#owned) {
      const cell = this.#owned; this.#owned = cell.ownedNext;
      cell.key = null; cell.collision = null; cell.ownedNext = null;
      return { complete: false, bytes: 48 };
    }
    return { complete: true, bytes: 0 };
  }
}

export { SiblingKeys as RetainedUiSiblingKeys };
