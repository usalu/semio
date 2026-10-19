import type { ActorInstanceLifetime } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import type { BrowserActorUiPatchOfferV1, BrowserActorUiPatchResultV1 } from "@semio-tech/framework/browser-bundle/patch-handoff";
import type { ResolvedPluginViewState } from "@semio-tech/framework/manifest";
import type { ActorDocumentBindingV1 } from "@semio-tech/framework/backbone/binding";
import type { BrowserActorEphemeralSnapshotV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { BROWSER_ACTOR_VISIBLE_SURFACES_V1 } from "../../🪟️visible-surfaces/🟦️.ts";
import type { DocumentBrowserActorChild, DocumentBrowserActorGrant } from "../../🧩️actor-wire-helpers/🟦️.ts";
import type { VerifiedColdDocumentPair } from "../../🧊️cold-pair/🟦️.ts";
import type { ArtifactState } from "../../🧾️document-state/🟦️.ts";
import type { DocumentExecutionTargetLease } from "../../🪪️execution-lease/🟦️.ts";

/** 🗃️ The single private session object of one {@link DocumentBrowserActorReservation}: the artifact state, the lease and grant it answers to and every mutable lane, ledger and flag of the child; the facade and its collaborators share it instead of holding many private fields. */
export class ReservationSession {
  readonly generation: bigint;
  readonly scope: Readonly<{ spaceId: string; documentId: string }>;
  readonly windowKindId: string;
  /** 🪟️ Every window kind of the verified app; an app command is admitted in any of them. */
  readonly windowKeys: ReadonlySet<string>;
  /** 🪟️ The window and panel bodies this child renders, by the key the shell's stores and the guest's surface refs share. */
  readonly surfaceKeys: ReadonlySet<string>;
  readonly abort = new AbortController();
  child: DocumentBrowserActorChild | null = null;
  activation: Promise<void> | null = null;
  lifetime: ActorInstanceLifetime | null = null;
  coldOwner: VerifiedColdDocumentPair | null = null;
  coldApplied: VerifiedColdDocumentPair | null = null;
  coldTransfer: Promise<void> | null = null;
  socket: WebSocket | null = null;
  linkSuspended = false;
  /** 🩹️ The one patch offer awaiting the main thread; `settled` mirrors its outcome so a queued action can wait on it without owning it. */
  pendingUiPatch: { readonly offer: BrowserActorUiPatchOfferV1; readonly resolve: (result: BrowserActorUiPatchResultV1) => void; readonly reject: (error: Error) => void; readonly timer: ReturnType<typeof setTimeout>; readonly settled: Promise<void> } | null = null;
  renderedUiPatch = false;
  /** 🩹️ The last revision the shell acknowledged per rendered surface; an intent is admitted against its own surface's. */
  readonly renderedSurfaceRevisions = new Map<string, number>();
  acknowledgedUiRevision = 0;
  mountedUiRevision = 0;
  renderedViewState: ResolvedPluginViewState | null = null;
  viewRefresh: Promise<void> | null = null;
  documentBinding: ActorDocumentBindingV1 | null = null;
  documentBindingGeneration = 0n;
  documentBackboneReady = false;
  backboneDraining = false;
  pendingBackboneBeforeBinding: Uint8Array[] = [];
  pendingBackboneBeforeBindingBytes = 0;
  pollTail: Promise<void> = Promise.resolve();
  turnTail: Promise<void> = Promise.resolve();
  localEffectTail: Promise<void> = Promise.resolve();
  localEffectFailure: Error | null = null;
  lastActionSequence = 0;
  lastEphemeral: BrowserActorEphemeralSnapshotV1 | null = null;
  retirement: Promise<"retired" | "unconfirmed"> | null = null;
  closed = false;
  constructor(readonly artifact: ArtifactState, readonly lease: DocumentExecutionTargetLease, public grant: DocumentBrowserActorGrant, readonly retire: () => void, nextGeneration: () => bigint) {
    const fields = lease.fields();
    this.scope = Object.freeze({ ...fields.scope });
    this.windowKindId = fields.surface.windowKindId;
    this.windowKeys = new Set(lease.renderSurfaces().windows.map(({ key }) => key));
    this.surfaceKeys = new Set([...this.windowKeys, ...lease.renderSurfaces().panels.map(({ key }) => key), ...BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.map(({ bodyKey }) => bodyKey)]);
    this.generation = nextGeneration();
  }

  get renderedUiRevision(): number {
    return this.renderedSurfaceRevisions.get(this.windowKindId) ?? 0;
  }
}
