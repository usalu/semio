//#region 🧬️ReadLeaseContract
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import type { OwnedUiNode, RetainedUiNodeRecord, UiNodeRetirement } from "../📦️wire/🧾️typed/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";
export interface OwnedUiReadSubscription { readonly snapshot: OwnedUiNodeReadSnapshot | null }
export interface OwnedUiReadSource {
  subscribeNode(id: number, notify: () => void): OwnedUiReadSubscription;
  acknowledgeRead(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot): void;
  unsubscribeNode(subscription: OwnedUiReadSubscription): void;
}

const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function natural(value: number): number { if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("Invalid owned UI read identity"); return value === 0 ? 0 : value; }
let issue: (version: number, node: OwnedUiNode | null) => OwnedUiNodeReadSnapshot;
let release: (snapshot: OwnedUiNodeReadSnapshot) => UiNodeRetirement | null;
type CommitState = { readonly owner: OwnedUiReadPublication; readonly version: number; status: "pending" | "published" | "cancelled" };
let commitState: (commit: OwnedUiReadCommit) => CommitState;
let createCommit: (owner: OwnedUiReadPublication, version: number) => OwnedUiReadCommit;
//#endregion 🧬️ReadLeaseContract

//#region ⚛️ReadPublication
/** ⚛️ One constant-size epoch makes every prepared consumer read visible without an owner loop. */
export class OwnedUiReadCommit {
  readonly #state: CommitState;
  private constructor(owner: OwnedUiReadPublication, version: number) { this.#state = { owner, version, status: "pending" }; Object.freeze(this); }
  static { createCommit = (owner, version) => new OwnedUiReadCommit(owner, version); commitState = commit => commit.#state; }
}

export class OwnedUiReadPublication {
  #version: number;
  #pending: OwnedUiReadCommit | null = null;
  constructor(version: number) { this.#version = natural(version); Object.freeze(this); }
  get version(): number { return this.#version; }
  begin(version: number): OwnedUiReadCommit {
    const next = natural(version); if (next <= this.#version || this.#pending) throw new Error("Owned UI read publication is not available");
    this.#pending = createCommit(this, next); return this.#pending;
  }
  publish(commit: OwnedUiReadCommit): boolean {
    if (this.#pending === null || commit !== this.#pending) return false;
    const exact = commitState(commit); exact.status = "published"; this.#version = exact.version; this.#pending = null; return true;
  }
  cancel(commit: OwnedUiReadCommit): boolean {
    if (this.#pending === null || commit !== this.#pending) return false;
    commitState(commit).status = "cancelled"; this.#pending = null; return true;
  }
}
//#endregion ⚛️ReadPublication

//#region 📸️IssuedSnapshot
/** 📸️ One exact issued read; only its issuing consumer's commit can authorize older-root retirement. */
export class OwnedUiNodeReadSnapshot {
  #node: OwnedUiNode | null;
  #active = true;
  private constructor(readonly version: number, node: OwnedUiNode | null) { this.#node = node; Object.freeze(this); }
  static {
    issue = (version, node) => new OwnedUiNodeReadSnapshot(version, node);
    release = snapshot => { if (!snapshot.#active) throw new Error("Owned UI read snapshot already retired"); snapshot.#active = false; const node = snapshot.#node; snapshot.#node = null; return node?.beginClose() ?? null; };
  }
  get record(): RetainedUiNodeRecord | undefined { if (!this.#active) throw new Error("Owned UI read snapshot is retired"); return this.#node?.value; }
}
//#endregion 📸️IssuedSnapshot

//#region 📖️ConsumerLease
/** 📖️ Two issued roots bound speculative-read ownership; capacity returns only after retained retirement. */
export class OwnedUiNodeReadLease {
  readonly #id: number;
  #first: OwnedUiNodeReadSnapshot | null;
  #second: OwnedUiNodeReadSnapshot | null = null;
  #retirement: UiNodeRetirement | null = null;
  #releaseFirst = false;
  #started = false;
  #closing = false;
  #closed = false;
  #publication: OwnedUiReadPublication | null;
  #commit: OwnedUiReadCommit | null = null;
  #discard = false;

  constructor(id: number, version: number, node: OwnedUiNode | null, publication: OwnedUiReadPublication | null = null) {
    this.#id = natural(id); natural(version);
    if (publication && publication.version !== version) throw new Error("Owned UI read publication version mismatch");
    if (node && node.value.id !== this.#id) throw new Error("Owned UI read node identity mismatch");
    this.#publication = publication;
    this.#first = issue(version, node?.capture() ?? null); Object.freeze(this);
  }
  #visible(): boolean { return this.#commit === null || commitState(this.#commit).status === "published"; }
  #cancelled(): boolean { return this.#commit !== null && commitState(this.#commit).status === "cancelled"; }
  get snapshot(): OwnedUiNodeReadSnapshot { if (this.#closing || this.#closed) throw new Error("Owned UI read lease is closing"); return this.#second && this.#visible() ? this.#second : this.#first!; }
  get retirementPending(): boolean { return this.#releaseFirst || this.#cancelled() || this.#discard; }
  get hasCapacity(): boolean { return !this.#closing && !this.#closed && !this.#second && !this.#releaseFirst; }

  offer(version: number, node: OwnedUiNode | null): boolean {
    if (this.#publication) throw new Error("Owned UI publication reads require an exact staging token");
    return this.#offer(version, node);
  }

  stage(commit: OwnedUiReadCommit, node: OwnedUiNode | null): boolean {
    const exact = commitState(commit);
    if (exact.owner !== this.#publication || exact.status !== "pending") throw new Error("Foreign or terminal owned UI read publication");
    if (!this.#offer(exact.version, node)) return false;
    this.#commit = commit; return true;
  }

  #offer(version: number, node: OwnedUiNode | null): boolean {
    natural(version);
    if (this.#closing || this.#closed) return false;
    const latest = this.#second ?? this.#first!;
    if (version <= latest.version) throw new Error("Owned UI read version did not advance");
    if (node && node.value.id !== this.#id) throw new Error("Owned UI read node identity mismatch");
    if (this.#second || this.#releaseFirst) return false;
    this.#second = issue(version, node?.capture() ?? null);
    return true;
  }

  acknowledge(snapshot: OwnedUiNodeReadSnapshot): boolean {
    if (this.#closing || this.#closed) return false;
    if (this.#second !== null && snapshot === this.#second && this.#visible()) { this.#releaseFirst = true; return true; }
    return this.#first !== null && snapshot === this.#first && !this.#releaseFirst;
  }

  #advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!this.#closing && (this.#cancelled() || this.#discard)) {
      if (!this.#discard) { this.#retirement = release(this.#second!); this.#discard = true; return state("pending", "read-staging-release", 64); }
      if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
      this.#second = null; this.#commit = null; this.#discard = false; return state("pending", "read-staging-capacity", 64);
    }
    if (!this.#releaseFirst) return state("ready", "read-lease-idle");
    if (!this.#started) { this.#retirement = release(this.#first!); this.#started = true; return state("pending", "read-snapshot-release", 64); }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
    this.#first = this.#second; this.#second = null; this.#commit = null; this.#releaseFirst = false; this.#started = false;
    return state("pending", "read-capacity-release", 64);
  }

  advanceRetirement(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#closing || this.#closed) throw new Error("Use closeStep after closing an owned UI read lease");
    if (!admitted(grant)) return state("blocked", "read-retirement");
    return this.#advance(grant);
  }
  beginClose(): void { if (this.#closed || this.#closing) return; this.#closing = true; if (this.#first) this.#releaseFirst = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#closed) return state("complete", "read-lease-close");
    if (!this.#closing) throw new Error("Owned UI read close has not begun");
    if (!admitted(grant)) return state("blocked", "read-lease-close");
    if (this.#discard) {
      if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
      this.#second = null; this.#commit = null; this.#discard = false; return state("pending", "read-staging-close", 64);
    }
    if (this.#first) { this.#releaseFirst = true; return this.#advance(grant); }
    this.#publication = null; this.#closed = true; return state("complete", "read-lease-close");
  }
  terminalIsEmpty(): boolean { return this.#closed && !this.#first && !this.#second && !this.#retirement && !this.#releaseFirst && !this.#started && !this.#commit && !this.#discard && !this.#publication; }
}
//#endregion 📖️ConsumerLease
