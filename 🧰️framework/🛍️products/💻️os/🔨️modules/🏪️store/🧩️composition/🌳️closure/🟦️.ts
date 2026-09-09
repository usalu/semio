/** 🌳️ Fuel-bounded closure admission over immutable decoded document projections. */
import type { ArtifactRef } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { OwnerRef } from "../../🪆️child/🏠️owner/🧬️schema/🟦️.ts";
export type { ArtifactRef } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
export type { OwnerRef } from "../../🪆️child/🏠️owner/🧬️schema/🟦️.ts";
export interface OwnedDocumentChildProjection { slot: string; childId: string; target: ArtifactRef; }
export interface OwnedDocumentRootProjection { reference: ArtifactRef; children: readonly OwnedDocumentChildProjection[]; }
export interface OwnedDocumentMemberProjection extends OwnedDocumentRootProjection { owner: OwnerRef; }
export interface OwnedDocumentClosureInput { root: OwnedDocumentRootProjection; members: readonly OwnedDocumentMemberProjection[]; }
export interface OwnedDocumentClosureSource extends OwnedDocumentClosureInput { generation: number; }
export interface OwnedDocumentClosureGrant { operation: number; generation: number; maximumItems: number; nowMicros: number; cancelled: boolean; }
export type OwnedDocumentClosureStep = { status: "pending" } | { status: "complete"; members: number } | { status: "rejected"; reason: string };
const textEncoder = new TextEncoder();

function text(value: string): boolean {
  return typeof value === "string" && value.length > 0 && value.length <= 256 && textEncoder.encode(value).length <= 256 && !/[\u0000-\u001f\u007f-\u009f]/.test(value);
}
function reference(value: ArtifactRef): boolean {
  return [value.artifactId, value.dialect.artifactKind, value.dialect.standard, value.dialect.subset].every(text) && /^s\.[a-z0-9]+(?:-[a-z0-9]+)*\.[a-z0-9]+(?:-[a-z0-9]+)*$/.test(value.dialect.artifactKind);
}
function same(a: ArtifactRef, b: ArtifactRef): boolean {
  return a.artifactId === b.artifactId && a.dialect.artifactKind === b.dialect.artifactKind && a.dialect.standard === b.dialect.standard && a.dialect.subset === b.dialect.subset;
}

/** 🪪️ The source generation fences the complete candidate registry for every continuation. */
export class OwnedDocumentClosure {
  readonly progress = { indexed: 0, visited: 0, steps: 0 };
  private phase: "root" | "index" | "walk" = "root";
  private readonly index = new Map<string, number>();
  private readonly seen = new Set<number>();
  private readonly queue = [-1];
  private head = 0;
  private child = 0;
  private expectedMembers: number | undefined;
  private terminal: OwnedDocumentClosureStep | undefined;
  constructor(private readonly operation: number, private readonly generation: number, private readonly sourceGeneration: number, private readonly expiresAtMicros: number) {}

  /** 🚦️ One item admits one bounded record or visits one declared child edge. */
  step(source: OwnedDocumentClosureSource, grant: OwnedDocumentClosureGrant): OwnedDocumentClosureStep {
    if (grant.operation !== this.operation || grant.generation !== this.generation || source.generation !== this.sourceGeneration || (this.expectedMembers !== undefined && this.expectedMembers !== source.members.length)) return this.reject("stale");
    if (grant.cancelled) return this.reject("cancelled");
    if (!Number.isFinite(grant.nowMicros) || grant.nowMicros >= this.expiresAtMicros) return this.reject("expired");
    if (this.terminal) return this.terminal;
    if (source.members.length > 1024) return this.reject("member-limit");
    if (!Number.isSafeInteger(grant.maximumItems) || grant.maximumItems < 0) return this.reject("grant");
    for (let item = 0; item < grant.maximumItems; item += 1) {
      this.progress.steps += 1;
      if (this.phase === "root") {
        if (!reference(source.root.reference)) return this.reject("reference");
        this.expectedMembers = source.members.length;
        this.phase = "index";
      } else if (this.phase === "index") {
        if (this.progress.indexed === source.members.length) { this.phase = "walk"; continue; }
        const member = source.members[this.progress.indexed]!;
        if (!reference(member.reference) || !reference(member.owner.parent) || !text(member.owner.slot) || member.owner.childId !== member.reference.artifactId) return this.reject("owner");
        if (member.reference.artifactId === source.root.reference.artifactId || this.index.has(member.reference.artifactId)) return this.reject("duplicate");
        this.index.set(member.reference.artifactId, this.progress.indexed);
        this.progress.indexed += 1;
      } else {
        if (this.head === this.queue.length) {
          if (this.seen.size !== source.members.length) return this.reject("incomplete");
          return this.terminal = { status: "complete", members: source.members.length };
        }
        const parentIndex = this.queue[this.head]!;
        const parent = parentIndex < 0 ? source.root : source.members[parentIndex]!;
        if (parent.children.length > 64) return this.reject("reference-limit");
        if (this.child === parent.children.length) { this.head += 1; this.child = 0; this.progress.visited += 1; continue; }
        const edge = parent.children[this.child++]!;
        if (!text(edge.slot) || !reference(edge.target) || edge.childId !== edge.target.artifactId) return this.reject("projection");
        const index = this.index.get(edge.childId);
        if (index === undefined) return this.reject("incomplete");
        const member = source.members[index]!;
        if (!same(member.reference, edge.target) || !same(member.owner.parent, parent.reference) || member.owner.slot !== edge.slot || member.owner.childId !== edge.childId) return this.reject("owner");
        if (this.seen.has(index)) return this.reject("duplicate");
        this.seen.add(index);
        this.queue.push(index);
      }
    }
    return { status: "pending" };
  }

  private reject(reason: string): OwnedDocumentClosureStep {
    return this.terminal = { status: "rejected", reason };
  }
}
