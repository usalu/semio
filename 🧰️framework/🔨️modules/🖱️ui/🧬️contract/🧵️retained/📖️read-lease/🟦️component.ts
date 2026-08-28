//#region 🧬️ReadLeaseContract
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import type { OwnedUiNode, RetainedUiNodeRecord, UiNodeRetirement } from "../📦️wire/🧾️typed/🟦️component.ts";
import { OwnedUiSceneBinding, type OwnedUiSceneBindingRetirement, type OwnedUiSceneDiagnostic } from "../🎬️scene/🔗️binding/🟦️component.ts";
import type { OwnedUiPreparedSceneReader, OwnedUiPreparedSceneReadStep } from "../🎬️scene/🧾️typed/🟦️component.ts";
import type { OwnedUiSceneReader, OwnedUiSceneReadStep } from "../🎬️scene/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";
type ReadOwner = OwnedUiNode | OwnedUiSceneBinding;
type ReadRetirement = UiNodeRetirement | OwnedUiSceneBindingRetirement;
type IssuedRoot = { readonly lease: object; node: ReadOwner | null; active: boolean; readers: number };
type SceneRetirement = { advance(grant: NumericIndexGrant): RetainedUiWireStep; terminalIsEmpty(): boolean };
const ISSUED_MINT = Object.freeze({});
export interface OwnedUiReadSubscription { readonly snapshot: OwnedUiNodeReadSnapshot | null }
export interface OwnedUiReadSource {
  subscribeNode(id: number, notify: () => void): OwnedUiReadSubscription;
  acknowledgeRead(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot): void;
  unsubscribeNode(subscription: OwnedUiReadSubscription): void;
}
export type OwnedUiIssuedSceneReader = OwnedUiIssuedSceneRecord | OwnedUiIssuedSceneText;
export interface OwnedUiSceneRecordView { advance(grant: NumericIndexGrant): OwnedUiPreparedSceneReadStep; close(): boolean }
export interface OwnedUiSceneTextView { advance(grant: NumericIndexGrant): OwnedUiSceneReadStep; close(): boolean }
export interface OwnedUiSceneReadSource extends OwnedUiReadSource {
  retireSceneRead(subscription: OwnedUiReadSubscription, reader: OwnedUiIssuedSceneReader): boolean;
  openSceneRecord(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source?: number): OwnedUiSceneRecordView | null;
  openSceneText(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source: number): OwnedUiSceneTextView | null;
}

const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function natural(value: number): number { if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("Invalid owned UI read identity"); return value === 0 ? 0 : value; }
let issue: (lease: object, version: number, node: ReadOwner | null) => OwnedUiNodeReadSnapshot;
let release: (snapshot: OwnedUiNodeReadSnapshot) => OwnedUiIssuedRetirement;
let retireIssued: (root: IssuedRoot) => OwnedUiIssuedRetirement;
let ownSceneRecord: (root: IssuedRoot, reader: OwnedUiPreparedSceneReader) => OwnedUiIssuedSceneRecord;
let ownSceneText: (root: IssuedRoot, reader: OwnedUiSceneReader) => OwnedUiIssuedSceneText;
let retireScene: (root: IssuedRoot, reader: SceneRetirement) => OwnedUiIssuedSceneRetirement;
let recordLease: (reader: OwnedUiIssuedSceneRecord) => object | null;
let textLease: (reader: OwnedUiIssuedSceneText) => object | null;
let snapshotLease: (snapshot: OwnedUiNodeReadSnapshot) => object | null;
type CommitState = { readonly owner: OwnedUiReadPublication; readonly version: number; status: "pending" | "published" | "cancelled" };
let commitState: (commit: OwnedUiReadCommit) => CommitState;
let createCommit: (owner: OwnedUiReadPublication, version: number) => OwnedUiReadCommit;
let pendingCommit: (owner: OwnedUiReadPublication, commit: OwnedUiReadCommit) => boolean;
//#endregion 🧬️ReadLeaseContract

//#region ⚛️ReadPublication
/** ⚛️ One constant-size epoch makes every prepared consumer read visible without an owner loop. */
export class OwnedUiReadCommit {
  readonly #state: CommitState;
  private constructor(mint: object, owner: OwnedUiReadPublication, version: number) { if (mint !== ISSUED_MINT) throw new Error("Read commit requires exact mint authority"); this.#state = { owner, version, status: "pending" }; Object.freeze(this); }
  static { createCommit = (owner, version) => new OwnedUiReadCommit(ISSUED_MINT, owner, version); commitState = commit => commit.#state; }
}

export class OwnedUiReadPublication {
  #version: number;
  #pending: OwnedUiReadCommit | null = null;
  constructor(version: number) { this.#version = natural(version); Object.freeze(this); }
  static { pendingCommit = (owner, commit) => owner.#pending === commit; }
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
  readonly #root: IssuedRoot;
  private constructor(mint: object, lease: object, readonly version: number, node: ReadOwner | null) { if (mint !== ISSUED_MINT) throw new Error("Issued snapshot requires exact mint authority"); this.#root = { lease, node, active: true, readers: 0 }; Object.freeze(this); }
  static {
    issue = (lease, version, node) => new OwnedUiNodeReadSnapshot(ISSUED_MINT, lease, version, node);
    release = snapshot => { if (!snapshot.#root.active) throw new Error("Owned UI read snapshot already retired"); snapshot.#root.active = false; return retireIssued(snapshot.#root); };
    snapshotLease = snapshot => snapshot.#root.active ? snapshot.#root.lease : null;
  }
  #live(): IssuedRoot { if (!this.#root.active) throw new Error("Owned UI read snapshot is retired"); return this.#root; }
  get record(): RetainedUiNodeRecord | undefined { return this.#live().node?.value; }
  get sceneDiagnostic(): OwnedUiSceneDiagnostic | null { const node = this.#live().node; return node instanceof OwnedUiSceneBinding ? node.diagnostic : null; }
  get hasPreparedScene(): boolean { const node = this.#live().node; return node instanceof OwnedUiSceneBinding && node.prepared; }
  beginSceneRecord(source = 0): OwnedUiIssuedSceneRecord | null {
    const root = this.#live(); if (root.readers === 2 || !(root.node instanceof OwnedUiSceneBinding)) return null;
    const reader = root.node.beginRecord(source); if (!reader) return null; root.readers++; return ownSceneRecord(root, reader);
  }
  beginSceneText(source: number): OwnedUiIssuedSceneText | null {
    const root = this.#live(); if (root.readers === 2 || !(root.node instanceof OwnedUiSceneBinding)) return null;
    const reader = root.node.beginText(source); if (!reader) return null; root.readers++; return ownSceneText(root, reader);
  }
}
//#endregion 📸️IssuedSnapshot

//#region 🎬️IssuedSceneReaders
export class OwnedUiIssuedSceneRecord {
  #root: IssuedRoot | null;
  #reader: OwnedUiPreparedSceneReader | null;
  private constructor(mint: object, root: IssuedRoot, reader: OwnedUiPreparedSceneReader) { if (mint !== ISSUED_MINT) throw new Error("Issued scene reader requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownSceneRecord = (root, reader) => new OwnedUiIssuedSceneRecord(ISSUED_MINT, root, reader); recordLease = reader => reader.#root?.lease ?? null; }
  advance(grant: NumericIndexGrant): OwnedUiPreparedSceneReadStep { if (!this.#reader) throw new Error("Issued scene reader is closed"); return this.#reader.advance(grant); }
  beginClose(): OwnedUiIssuedSceneRetirement { if (!this.#reader || !this.#root) throw new Error("Issued scene reader is closed"); const result = retireScene(this.#root, this.#reader.beginClose()); this.#reader = null; this.#root = null; return result; }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}

export class OwnedUiIssuedSceneText {
  #root: IssuedRoot | null;
  #reader: OwnedUiSceneReader | null;
  private constructor(mint: object, root: IssuedRoot, reader: OwnedUiSceneReader) { if (mint !== ISSUED_MINT) throw new Error("Issued scene text requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownSceneText = (root, reader) => new OwnedUiIssuedSceneText(ISSUED_MINT, root, reader); textLease = reader => reader.#root?.lease ?? null; }
  advance(grant: NumericIndexGrant): OwnedUiSceneReadStep { if (!this.#reader) throw new Error("Issued scene text is closed"); return this.#reader.advance(grant); }
  beginClose(): OwnedUiIssuedSceneRetirement { if (!this.#reader || !this.#root) throw new Error("Issued scene text is closed"); const result = retireScene(this.#root, this.#reader.beginClose()); this.#reader = null; this.#root = null; return result; }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}

export class OwnedUiIssuedSceneRetirement {
  #root: IssuedRoot | null;
  #reader: SceneRetirement | null;
  private constructor(mint: object, root: IssuedRoot, reader: SceneRetirement) { if (mint !== ISSUED_MINT) throw new Error("Issued scene retirement requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { retireScene = (root, reader) => new OwnedUiIssuedSceneRetirement(ISSUED_MINT, root, reader); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "issued-scene-close");
    if (this.#reader) { const current = this.#reader.advance(grant); if (current.kind === "complete") this.#reader = null; return { ...current, kind: "pending" }; }
    if (this.#root) { this.#root.readers--; this.#root = null; return state("pending", "issued-scene-slot-release", 32); }
    return state("complete", "issued-scene-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}

class OwnedUiIssuedRetirement {
  #root: IssuedRoot | null;
  #node: ReadRetirement | null = null;
  private constructor(mint: object, root: IssuedRoot) { if (mint !== ISSUED_MINT) throw new Error("Issued retirement requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { retireIssued = root => new OwnedUiIssuedRetirement(ISSUED_MINT, root); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "issued-read-close");
    if (this.#root?.readers) return state("blocked", "issued-scene-readers");
    if (this.#node) { const current = this.#node.advance(grant); if (current.kind === "complete") this.#node = null; return { ...current, kind: "pending" }; }
    if (this.#root) { this.#node = this.#root.node?.beginClose() ?? null; this.#root.node = null; this.#root = null; return state("pending", "issued-node-close", 64); }
    return state("complete", "issued-read-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#node === null; }
}
//#endregion 🎬️IssuedSceneReaders

//#region 📖️ConsumerLease
/** 📖️ Two issued roots bound speculative-read ownership; capacity returns only after retained retirement. */
export class OwnedUiNodeReadLease {
  readonly #authority = Object.freeze({});
  readonly #id: number;
  #first: OwnedUiNodeReadSnapshot | null;
  #second: OwnedUiNodeReadSnapshot | null = null;
  #retirement: OwnedUiIssuedRetirement | null = null;
  #releaseFirst = false;
  #started = false;
  #closing = false;
  #closed = false;
  #publication: OwnedUiReadPublication | null;
  #commit: OwnedUiReadCommit | null = null;
  #discard = false;

  constructor(id: number, version: number, node: ReadOwner | null, publication: OwnedUiReadPublication | null = null) {
    this.#id = natural(id); natural(version);
    if (publication && publication.version !== version) throw new Error("Owned UI read publication version mismatch");
    if (node && node.value.id !== this.#id) throw new Error("Owned UI read node identity mismatch");
    this.#publication = publication;
    this.#first = issue(this.#authority, version, node?.capture() ?? null); Object.freeze(this);
  }
  #visible(): boolean { return this.#commit === null || commitState(this.#commit).status === "published"; }
  #cancelled(): boolean { return this.#commit !== null && commitState(this.#commit).status === "cancelled"; }
  get snapshot(): OwnedUiNodeReadSnapshot { if (this.#closing || this.#closed) throw new Error("Owned UI read lease is closing"); return this.#second && this.#visible() ? this.#second : this.#first!; }
  get retirementPending(): boolean { return this.#releaseFirst || this.#cancelled() || this.#discard; }
  get hasCapacity(): boolean { return !this.#closing && !this.#closed && !this.#second && !this.#releaseFirst; }
  canReadSnapshot(snapshot: OwnedUiNodeReadSnapshot): boolean { return !this.#closing && !this.#closed && snapshot instanceof OwnedUiNodeReadSnapshot && snapshotLease(snapshot) === this.#authority; }
  takeSceneRetirement(reader: OwnedUiIssuedSceneReader): OwnedUiIssuedSceneRetirement | null {
    const owner = reader instanceof OwnedUiIssuedSceneRecord ? recordLease(reader) : reader instanceof OwnedUiIssuedSceneText ? textLease(reader) : null;
    return !this.#closed && owner === this.#authority ? reader.beginClose() : null;
  }

  offer(version: number, node: ReadOwner | null): boolean {
    if (this.#publication) throw new Error("Owned UI publication reads require an exact staging token");
    return this.#offer(version, node);
  }

  stage(commit: OwnedUiReadCommit, node: ReadOwner | null): boolean {
    const exact = commitState(commit);
    if (exact.owner !== this.#publication || exact.status !== "pending" || !pendingCommit(exact.owner, commit)) throw new Error("Foreign or terminal owned UI read publication");
    if (!this.#offer(exact.version, node)) return false;
    this.#commit = commit; return true;
  }

  #offer(version: number, node: ReadOwner | null): boolean {
    natural(version);
    if (this.#closing || this.#closed) return false;
    const latest = this.#second ?? this.#first!;
    if (version <= latest.version) throw new Error("Owned UI read version did not advance");
    if (node && node.value.id !== this.#id) throw new Error("Owned UI read node identity mismatch");
    if (this.#second || this.#releaseFirst) return false;
    this.#second = issue(this.#authority, version, node?.capture() ?? null);
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
