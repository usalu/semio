/** 🪤️ The fake browser actor worker of the cold pair transfer: load, describe, stage, invoke and terminate over a message port.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { workerLawFakesV1 } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, ArtifactState, MutationEnvelope, ResolvedPluginViewState, Vitest } from "../📐️document-open-types/🟦️.ts";

/** 🤖️ Declares the fake browser actor worker class over the recorded traffic. */
export function createColdPairWorkerClass({ vitest, dependencies, fakes, probe, actorBytes, guest, decodeAppCommand, commandSequences, commandInvocations, commandViews, corpus, pageIndexes, received, windowKindId, panelVisible, state, sectionVisible, actionSequences, backboneIngress, exact, bodyKey, visibleViews, secondWindowVisible, renderEvents, lifecycleClose, decodeDocumentBackboneControlV1, documentControls, encodeDocumentBackboneControlV1, fields, documentId, encodeAppFrame, actionFixture, transferableBuffer }: {
  readonly vitest: Vitest;
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly fakes: ReturnType<typeof workerLawFakesV1>;
  readonly probe: ColdPairProbe;
  readonly actorBytes: Uint8Array<ArrayBuffer>;
  readonly guest: Record<string, any>;
  readonly decodeAppCommand: typeof import("@semio-tech/framework-os").decodeAppCommand;
  readonly commandSequences: number[];
  readonly commandInvocations: Record<string, unknown>[];
  readonly commandViews: Record<string, unknown>[];
  readonly corpus: any;
  readonly pageIndexes: number[];
  readonly received: Uint8Array<ArrayBuffer>;
  readonly windowKindId: string;
  readonly panelVisible: (view: ResolvedPluginViewState) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; }[];
  readonly state: ArtifactState;
  readonly sectionVisible: (view: ResolvedPluginViewState, mount: boolean) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; }[];
  readonly actionSequences: bigint[];
  readonly backboneIngress: Uint8Array<ArrayBufferLike>[];
  readonly exact: { packLength: number; sprLength: number; pageCount: number; packPattern: { multiplier: number; addend: number; }; sprPattern: { multiplier: number; addend: number; }; packSha256: string; sprSha256: string; aggregateSha256: string; };
  readonly bodyKey: string;
  readonly visibleViews: Record<string, unknown>[];
  readonly secondWindowVisible: (view: ResolvedPluginViewState) => { tag: string; val: { surface: { instance: number; surface: string; }; bodyKey: string; viewState: Uint8Array<ArrayBuffer>; }; };
  readonly renderEvents: string[];
  readonly lifecycleClose: string[];
  readonly decodeDocumentBackboneControlV1: typeof import("@semio-tech/framework/backbone/binding").decodeDocumentBackboneControlV1;
  readonly documentControls: string[];
  readonly encodeDocumentBackboneControlV1: typeof import("@semio-tech/framework/backbone/binding").encodeDocumentBackboneControlV1;
  readonly fields: Awaited<ReturnType<ReturnType<typeof workerLawFakesV1>["executionTargetLeaseFixture"]>>["manifest"];
  readonly documentId: string;
  readonly encodeAppFrame: typeof import("@semio-tech/framework-os").encodeAppFrame;
  readonly actionFixture: any;
  readonly transferableBuffer: (bytes: Uint8Array) => ArrayBuffer;
}) {
  const { expect } = vitest;
  const { encodePackValue, decodePackValue } = dependencies;
  const { exactDocumentBackboneMessage } = fakes;
  class ColdPairWorker {
    port!: MessagePort;
    binding!: { schema: string; nonce: string; generation: string };
    postMessage(init: Record<string, any>): void {
      this.port = init.port;
      this.binding = { schema: init.schema, nonce: init.nonce, generation: init.generation };
      this.port.onmessage = (event) => {
        const message = event.data;
        if (message.kind === "load") {
          probe.loaded++;
          expect(new Uint8Array(message.bytes)).toEqual(actorBytes);
          const byteLength = message.bytes.byteLength;
          new Uint8Array(message.bytes).fill(0);
          this.port.postMessage({ ...this.binding, kind: "loaded", byteLength, sha256: message.sha256 });
          return;
        }
        if (message.kind !== "invoke") return;
        if (message.path[0] === "describe") {
          probe.described++;
          const bytes = encodePackValue(guest);
          this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: bytes }, [bytes.buffer]);
          this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 1 });
          return;
        }
        if (message.path[1] === "stageCommandPage") {
          const cursor = message.args[0] as Record<string, any>;
          expect(cursor).toMatchObject({ owner: 0n, generation: probe.reservation!.generation, commandIndex: 0, commandCount: 1, instance: 0, pageIndex: 0, pageCount: 1 });
          const staged = decodeAppCommand(message.args[1] as Uint8Array);
          if (!("Command" in staged)) throw new Error("expected browser actor AppCommand::Command");
          probe.stagedCommandSequence = staged.Command.seq;
          expect(cursor.seq).toBe(BigInt(probe.stagedCommandSequence));
          commandSequences.push(probe.stagedCommandSequence);
          commandInvocations.push(decodePackValue(Uint8Array.from(staged.Command.command)) as Record<string, unknown>);
          commandViews.push(decodePackValue(Uint8Array.from(staged.Command.view_state)) as Record<string, unknown>);
          this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: null });
          this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 0 });
          return;
        }
        if (message.path[1] === "stageColdPairPage") {
          const staged = message.args[0] as Record<string, any>;
          expect(probe.lifecycleAcknowledged).toBe(true);
          const header = staged.header as Record<string, any>,
            pageIndex = staged.pageIndex as number,
            bytes = staged.bytes as Uint8Array,
            start = pageIndex * corpus.limits.pageBytes;
          expect(pageIndexes).toHaveLength(pageIndex);
          expect(bytes.byteLength).toBe(Math.min(corpus.limits.pageBytes, received.byteLength - start));
          received.set(bytes, start);
          bytes.fill(0);
          pageIndexes.push(pageIndex);
          probe.activeLifetime = header.lifetime;
          probe.stagedColdPairIngress =
            pageIndex + 1 === header.pageCount
              ? { tag: "applied", val: { lifetime: header.lifetime, transferGeneration: header.transferGeneration, baselineFrontier: header.baselineFrontier, aggregateSha256: header.aggregateSha256 } }
              : { tag: "page-accepted", val: { lifetime: header.lifetime, transferGeneration: header.transferGeneration, pageIndex, pageCount: header.pageCount } };
          this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: null });
          this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 0 });
          return;
        }
        expect(message.path).toEqual(["reactor", "poll"]);
        const events = message.args[0] as Record<string, any>[];
        const open = events[0]?.tag === "instance-open" ? events[0].val : null;
        const close = events[0]?.tag === "instance-close" ? events[0].val : null;
        const lifecycleAck = events[0]?.tag === "instance-lifecycle-ack" ? events[0].val : null;
        const acknowledged = lifecycleAck?.tag === "captured";
        const patchAck = events[0]?.tag === "patch-ack" ? events[0].val : null;
        const patchRejection = events[0]?.tag === "patch-rejected" ? events[0].val : null;
        const panelRefresh = events[0]?.tag === "surface-visible" && events[0].val.surface.surface !== windowKindId;
        if (panelRefresh) {
          expect(events).toEqual([...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, false)]);
          probe.panelRefreshes++;
        }
        const visible = events[0]?.tag === "surface-visible" && !panelRefresh ? events[0].val : null;
        const wake = events[0]?.tag === "wake";
        const shellMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "shell" ? events[0].val : null;
        const backboneMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "backbone" ? events[0].val : null;
        const uiIntent = events[0]?.tag === "ui-intent" ? events[0].val : null;
        if (uiIntent !== null) {
          expect(uiIntent.instance).toBe(0);
          const decoded = decodePackValue(uiIntent.intent) as Record<string, any>;
          expect(decoded.surface).toBe(`0:${windowKindId}`);
          actionSequences.push(decoded.seq.value);
        }
        const commandSequence = probe.stagedCommandSequence;
        probe.stagedCommandSequence = null;
        if (backboneMessage !== null) backboneIngress.push(Uint8Array.from(backboneMessage.payload));
        if (visible) {
          expect(probe.lifecycleAcknowledged).toBe(true);
          expect(pageIndexes).toHaveLength(exact.pageCount);
          expect(visible.surface).toEqual({ instance: 0, surface: windowKindId });
          expect(visible.bodyKey).toBe(bodyKey);
          const visibleView = decodePackValue(visible.viewState) as Record<string, unknown>;
          expect(visibleView).toEqual(state.browserActorViewState);
          visibleViews.push(visibleView);
          expect(events.slice(1)).toEqual([secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, visibleViews.length === 1)]);
        }
        if (wake) {
          expect(renderEvents[0]).toBe("surface-visible");
          probe.renderWakes++;
        }
        if (visible || wake || patchAck || patchRejection) renderEvents.push(events[0].tag);
        const coldPairIngress = probe.stagedColdPairIngress;
        probe.stagedColdPairIngress = { tag: "idle" };
        if (acknowledged) probe.lifecycleAcknowledged = true;
        if (patchAck) {
          expect(patchAck.surface).toEqual({ instance: 0, surface: windowKindId });
          if (patchAck.receipt.patchSequence === 1n) {
            expect(patchAck.revision).toBe(1n);
            probe.patchAcknowledged = true;
          } else {
            expect(patchAck.receipt.patchSequence).toBe(3n);
            expect(patchAck.revision).toBe(2n);
            probe.actionPatchAcknowledged = true;
          }
        }
        if (patchRejection) {
          expect(patchRejection.surface).toEqual({ instance: 0, surface: windowKindId });
          expect(patchRejection.revision).toBe(1n);
          expect(patchRejection.receipt.patchSequence).toBe(2n);
          expect(patchRejection.reason).toBe("revisionMismatch");
          probe.patchRejected = true;
        }
        if (close) lifecycleClose.push("close");
        if (lifecycleAck?.tag === "accepted") lifecycleClose.push("accepted-ack");
        if (lifecycleAck?.tag === "retired") lifecycleClose.push("retired-ack");
        const lifecycleReceipt = open
          ? { tag: "captured", val: { lifetime: { activationGeneration: open.activationGeneration, instanceId: open.instance, guestLifetime: 1n }, requestSequence: open.requestSequence } }
          : close
            ? { tag: "accepted", val: { lifetime: close.lifetime, requestSequence: close.requestSequence, closeGeneration: 1n } }
            : lifecycleAck?.tag === "accepted"
              ? { tag: "retired", val: { lifetime: lifecycleAck.val.lifetime, requestSequence: lifecycleAck.val.requestSequence, closeGeneration: lifecycleAck.val.closeGeneration } }
              : null;
        const initialScene = wake && probe.renderWakes === corpus.browserRender.lazyWakeTurns;
        const lifetime = probe.activeLifetime;
        const node = {
          id: 0,
          key: "map-root",
          component: { type: "surface", kind: "tiled-map", docSchema: "tiled-map@1", doc: { bytes: [1, 3, 5] }, bindings: [] },
          layout: { kind: "leaf", width: "fill", height: "fill" },
          style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
          activity: "idle",
          disabled: false,
          transition: null,
          accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
          bindings: [],
          menu: null,
          children: [],
        };
        const emitsPatch = initialScene || uiIntent !== null || patchAck?.receipt.patchSequence === 1n;
        const nodeBytes = initialScene ? encodePackValue(node) : null;
        const patchReceipt = emitsPatch ? { lifetime: lifetime!, patchSequence: initialScene ? 1n : uiIntent !== null ? 3n : 2n } : null;
        const effects: Record<string, unknown>[] = [];
        let controlReceipt: Uint8Array | null = null;
        if (shellMessage !== null) {
          const command = decodeDocumentBackboneControlV1(shellMessage.payload);
          documentControls.push(command.operation);
          controlReceipt = encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" });
          effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: controlReceipt } });
        }
        let commandBackbone: Uint8Array | null = null;
        const commandMutation = commandSequence === 3
          ? (() => {
              const mutationId = "direct-command-mutation",
                invocationId = "direct-command:0:3",
                forward = encodePackValue({ sequenceNumber: 3 }),
                inverse = encodePackValue(null),
                inverseVector = Uint8Array.of(1, inverse.byteLength, ...inverse),
                mutation = {
                  id: mutationId,
                  document: "0",
                  baseVersion: 0,
                  invocationId,
                  diff: { schema: `${fields.artifact.schema}.operation`, payload: Array.from(forward) },
                  inverse: { targetMutation: mutationId, inverseDiff: { schema: `${fields.artifact.schema}.operation.inverse`, payload: Array.from(inverseVector) }, baseVersion: 0, dependencies: [], undoPolicy: "ExactBaseOnly" },
                  dependencies: [],
                  author: "actor-1",
                  timestamp: { actor: 7, physical_ms: 8, logical: 9 },
                },
                envelope: MutationEnvelope = {
                  id: mutationId,
                  actor: "actor-1",
                  document: documentId,
                  schemaVersion: fields.artifact.schema,
                  deps: [],
                  payloadHash: "unused",
                  diff: { schemaId: fields.artifact.schema, payload: { sequenceNumber: 3 } },
                  inverse: { targetOperation: mutationId, inverseDiff: { schemaId: fields.artifact.schema, payload: null }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
                };
              commandBackbone = exactDocumentBackboneMessage(envelope, forward, { actor: 7n, physical_ms: 8n, logical: 9n });
              return { mutations: Array.from(encodePackValue([mutation])), inverseGroup: Array.from(encodePackValue({ invocationId, mutations: [mutationId], inverseMutations: [mutation.inverse] })) };
            })()
          : null;
        const actionPublication = uiIntent !== null
          ? encodeAppFrame(actionFixture.publication.emit)
          : commandSequence !== null
            ? encodeAppFrame({ Invocation: { in_reply_to: commandSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: commandMutation?.mutations ?? [], inverse_group: commandMutation?.inverseGroup ?? [] } })
            : null;
        if (actionPublication !== null) {
          effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: actionPublication } });
          effects.push(actionFixture.publication.hostEffect);
        }
        if (commandBackbone !== null) effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: `actor://${state.runtimeKey}` }, payload: commandBackbone } });
        const resultTransfers: ArrayBuffer[] = initialScene ? [transferableBuffer(nodeBytes!)] : [];
        if (controlReceipt !== null) resultTransfers.push(transferableBuffer(controlReceipt));
        if (actionPublication !== null) resultTransfers.push(transferableBuffer(actionPublication));
        if (commandBackbone !== null) resultTransfers.push(transferableBuffer(commandBackbone));
        this.port.postMessage(
          {
            ...this.binding,
            kind: "result",
            sequence: message.sequence,
            value: {
              uiPatches: initialScene
                ? [
                    {
                      surface: { instance: 0, surface: windowKindId },
                      baseRevision: 0n,
                      revision: 1n,
                      ops: [
                        { tag: "upsert", val: { node: nodeBytes } },
                        { tag: "set-root", val: 0n },
                      ],
                    },
                  ]
                : uiIntent !== null
                  ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 1n, revision: 2n, ops: [] }]
                : patchAck?.receipt.patchSequence === 1n
                  ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 0n, revision: 2n, ops: [] }]
                  : [],
              effects,
              presence: [],
              nextWake: null,
              status: { tag: (visible && visibleViews.length === 1) || (wake && !initialScene) ? "more-work" : "idle" },
              fuelUsed: 1n,
              commandIngress: { kind: commandSequence === null ? 0 : 4, cursor: { owner: 0n, generation: 0n, commandIndex: 0, commandCount: 0, instance: 0, seq: 0n, kind: 0, pageIndex: 0, pageCount: 0, itemCount: 0, metadata: 0 }, fault: [] },
              coldPairIngress,
              lifecycleReceipt,
              uiPatchReceipt: patchReceipt === null ? { tag: "none" } : { tag: "some", val: patchReceipt },
            },
          },
          resultTransfers,
        );
        this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: resultTransfers.length });
      };
      this.port.start();
      this.port.postMessage({ ...this.binding, kind: "ready" });
    }
    terminate(): void {
      probe.terminated++;
      this.port?.close();
    }
  }
  return { ColdPairWorker };
}
