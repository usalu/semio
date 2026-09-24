import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { encodeClientFrame, encodePresencePeer, type MutationEnvelope, type ServerFrame, type WireFrontierSummary, type WireMutationEnvelope } from "@semio-tech/framework-replication";
import type { ArtifactActorConfig } from "../../../../../🟦️.ts";
import type { ArtifactState, BackboneWorkerTestDependencies } from "../../../👷️worker/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

type Fixture = {
  readonly version: 1;
  readonly scenarios: readonly Scenario[];
};

type Scenario = {
  readonly id: string;
  readonly title: string;
  readonly documentId: string;
  readonly spaceId: string;
  readonly actor: string;
  readonly steps: readonly Step[];
};

type Step = {
  readonly op: "serverFrame" | "localDispatch" | "expect";
  readonly frame?: Record<string, unknown>;
  readonly dispatch?: Record<string, unknown>;
  readonly expect?: Record<string, unknown>;
};

class FakeHubWebSocket {
  static readonly CONNECTING = 0;
  static readonly OPEN = 1;
  static readonly CLOSING = 2;
  static readonly CLOSED = 3;
  readonly CONNECTING = 0 as const;
  readonly OPEN = 1 as const;
  readonly CLOSING = 2 as const;
  readonly CLOSED = 3 as const;
  readonly url = "ws://parity.test/socket";
  readonly protocol = "semio.socket.v1";
  readonly extensions = "";
  readonly bufferedAmount = 0;
  readyState = FakeHubWebSocket.OPEN;
  binaryType: BinaryType = "arraybuffer";
  readonly sent: Uint8Array[] = [];
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: (() => void) | null = null;
  send(data: Uint8Array): void {
    this.sent.push(Uint8Array.from(data));
  }
  close(): void {
    this.readyState = FakeHubWebSocket.CLOSED;
    this.onclose?.();
  }
  addEventListener(): void {}
  removeEventListener(): void {}
  dispatchEvent(): boolean {
    return false;
  }
}

function frontier(value: Record<string, unknown>): WireFrontierSummary {
  const byte = Number(value.chainHashByte);
  return {
    document_id: String(value.documentId),
    head_edit_ordinal: Number(value.headEditOrdinal),
    head_edit_id: String(value.headEditId),
    last_commit_seq: Number(value.lastCommitSeq),
    chain_hash: Array.from({ length: 32 }, () => byte),
  };
}

function wireEnvelope(documentId: string, mutationId: string, n: number): WireMutationEnvelope {
  return {
    mutation_id: mutationId,
    document_id: documentId,
    actor: "peer",
    dependencies: [],
    diff: { schema: "demo/v1", payload: [n] },
    inverse: { schema: "demo/v1", payload: [0] },
    timestamp: { actor: 1, physical_ms: 1, logical: 1 },
  };
}

function domainEnvelope(documentId: string, mutationId: string, n: number): MutationEnvelope {
  return {
    id: mutationId,
    actor: "local-actor",
    document: documentId,
    schemaVersion: "demo/v1",
    deps: [],
    payloadHash: "parity",
    diff: { schemaId: "demo/v1", payload: { n } },
    inverse: { targetOperation: mutationId, inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
  };
}


function armRelay(state: ArtifactState, frontierSummary: WireFrontierSummary): void {
  state.currentPack = new Uint8Array([1]);
  state.currentSpr = new Uint8Array([1]);
  state.frontier = frontierSummary;
  state.verifiedColdPair = { assertCurrent() {}, drop() {} } as ArtifactState["verifiedColdPair"];
}

function seedState(documentId: string, spaceId: string, actor: string, deps: BackboneWorkerTestDependencies): ArtifactState {
  const config: ArtifactActorConfig = {
    documentId,
    schema: "demo/v1",
    bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://parity.test", spaceId }],
    actor,
    watchExternal: false,
  };
  const runtimeKey = deps.documentRuntimeKeyForConfig(config);
  const state = {
    runtimeKey,
    config,
    openClientInstanceId: "parity-client",
    actor: "",
    hubActorReady: false,
    pendingSocketActorId: null,
    channel: { postMessage() {}, close() {} } as unknown as BroadcastChannel,
    socket: null,
    presenceAuthority: null,
    docAbort: new AbortController(),
    executionTargetOpen: null,
    executionTargetLease: null,
    browserActorReservation: null,
    browserActorViewState: null,
    sanityPollTimer: null,
    sseHealthy: false,
    revalidateFolder: async () => {},
    reconnectDelayMs: 500,
    outbox: [],
    pendingMutations: [],
    exactLocalEnvelopes: new WeakMap(),
    pendingDocumentBackboneBytes: 0,
    pendingDocumentBackboneMessages: 0,
    status: { persisted: false, pendingMutations: 0, remote: { kind: "detached" } },
    frontier: null,
    pendingResumeToken: null,
    requiredTailFrontier: null,
    artifactBootstrap: null,
    artifactBootstrapOwner: null,
    artifactBootstrapDeadlineMs: null,
    artifactBootstrapDeadlineTimer: null,
    artifactRebootstrapOwner: null,
    artifactRebootstrapDeadlineMs: null,
    artifactRebootstrapDeadlineTimer: null,
    artifactRebootstrapRequired: false,
    artifactBootstrapProgress: [],
    canonicalFolderMirror: null,
    verifiedColdPair: null,
    currentPack: null,
    currentSpr: null,
    hubFrameChain: Promise.resolve(),
    resumeToken: null,
    sessionColor: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    ingestedMutationIds: new Set(),
    hlcCounter: 0,
    closed: false,
  } as unknown as ArtifactState;
  deps.artifacts.set(runtimeKey, state);
  return state;
}

function clientFrameKind(bytes: Uint8Array, decodeClientFrame: BackboneWorkerTestDependencies["decodeClientFrame"]): string {
  const frame = decodeClientFrame(bytes).frame;
  if (typeof frame === "string") return frame;
  if ("SocketHelloV1" in frame) return "socketHelloV1";
  if ("Commands" in frame) return "commands";
  if ("Presence" in frame) return "presence";
  if ("PreviewPublish" in frame) return "previewPublish";
  return "other";
}

function buildServerFrame(documentId: string, frame: Record<string, unknown>): ServerFrame {
  switch (frame.kind) {
    case "welcome":
      return {
        Welcome: {
          session_id: String(frame.sessionId ?? "session"),
          resume_token: String(frame.resumeToken),
          server_frontier: frontier(frame.frontier as Record<string, unknown>),
          bootstrap: frame.bootstrap === "tail" ? "Tail" : "None",
        },
      };
    case "session":
      return { Session: { actor: String(frame.actor), color: Number(frame.color ?? 0) } };
    case "commands":
      return {
        Commands: {
          origin: String(frame.origin),
          frontier: frontier(frame.frontier as Record<string, unknown>),
          envelopes: ((frame.envelopes as readonly { mutationId: string; n: number }[]) ?? []).map((row) => wireEnvelope(documentId, row.mutationId, row.n)),
        },
      };
    case "ack":
      return {
        Ack: {
          batch_id: Number(frame.batchId ?? 0),
          frontier: frontier(frame.frontier as Record<string, unknown>),
          stages: [
            {
              Applied: {
                outcome:
                  frame.outcome === "rejected"
                    ? { Rejected: { reason: String(frame.rejectReason ?? "rejected"), messages: [] } }
                    : "Accepted",
              },
            },
          ],
        },
      };
    case "presence": {
      const count = Number(frame.peerCount ?? 0);
      const peers = Array.from({ length: count }, (_, index) =>
        encodePresencePeer({ actor: `peer-${index}`, connectedAtMs: 1, views: [], color: index }),
      );
      return { Presence: { peers } };
    }
    case "rebootstrapRequired":
      return {
        RebootstrapRequired: {
          control: {
            space_id: String(frame.spaceId),
            document_id: String(frame.documentId),
            checkpoint_id: Array(32).fill(1),
            descriptor_hash: Array(32).fill(2),
            baseline_frontier: frontier(frame.frontier as Record<string, unknown>),
          },
        },
      };
    default:
      throw new Error(`unsupported frame kind ${String(frame.kind)}`);
  }
}

/** ⚖️ Registers language-agnostic backbone parity scenarios against the TS worker twin. */
export async function registerBackboneParityTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, sourceUrl: string): Promise<void> {
  const parityRoot = new URL("../../⚖️parity/", import.meta.url);
  const { readdirSync } = await import("node:fs");
  const parityDir = fileURLToPath(parityRoot);
  const fixtureDirName = readdirSync(parityDir).find((name) => name.includes("fixture"));
  const schemaDirName = readdirSync(parityDir).find((name) => name.includes("schema") || name.includes("🧬"));
  if (!fixtureDirName || !schemaDirName) throw new Error("backbone parity fixture/schema directories missing");
  const fixture = JSON.parse(readFileSync(`${parityDir}/${fixtureDirName}/🔣️.json`, "utf8")) as Fixture;
  const schema = JSON.parse(readFileSync(`${parityDir}/${schemaDirName}/🔣️.json`, "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/BackboneParity`);
  if (!validate) throw new Error("backbone parity schema missing");
  vitest.expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);

  for (const scenario of fixture.scenarios) {
    vitest.it(`backbone parity: ${scenario.id}`, async () => {
      const priorSink = dependencies.testSeams.workerPostTestSink;
      const posted: unknown[] = [];
      const observedEvents: string[] = [];
      let lastOutcome: string | undefined;
      dependencies.testSeams.workerPostTestSink = (message) => {
        posted.push(message);
        if (message && typeof message === "object" && "kind" in message && message.kind === "event" && "event" in message) {
          const event = (message as { event: { kind: string; outcome?: { kind?: string } | string } }).event;
          observedEvents.push(event.kind);
          if (event.kind === "commandOutcome" && event.outcome) {
            lastOutcome = typeof event.outcome === "string" ? event.outcome : event.outcome.kind;
          }
        }
      };
      const state = seedState(scenario.documentId, scenario.spaceId, scenario.actor, dependencies);
      armRelay(state, {
        document_id: scenario.documentId,
        head_edit_ordinal: 0,
        head_edit_id: "genesis",
        last_commit_seq: 0,
        chain_hash: Array(32).fill(1),
      });
      const outgoing: string[] = [];
      try {
        for (const step of scenario.steps) {
          if (step.op === "localDispatch") {
            const dispatch = step.dispatch!;
            switch (dispatch.kind) {
              case "queueMutation":
                dependencies.queueOutbox(state, [domainEnvelope(scenario.documentId, String(dispatch.mutationId), Number(dispatch.n ?? 0))]);
                break;
              case "installSocketActor":
                state.pendingSocketActorId = String(dispatch.actor);
                state.socket = new FakeHubWebSocket() as unknown as WebSocket;
                break;
              case "connectSocket": {
                state.pendingSocketActorId = String(dispatch.actor);
                const socket = new FakeHubWebSocket();
                state.socket = socket as unknown as WebSocket;
                socket.send(
                  encodeClientFrame(
                    {
                      SocketHelloV1: {
                        wire_version: 1,
                        protocol_version: 1,
                        schema: state.config.schema,
                        pack_schema_hash: Array.from({ length: 32 }, () => 0),
                        resume_token: state.resumeToken,
                        frontier: state.frontier,
                      },
                    },
                    "command",
                  ),
                );
                break;
              }
              case "failConnection":
                state.socket?.close();
                state.socket = null;
                state.hubActorReady = false;
                state.pendingSocketActorId = null;
                break;
              default:
                throw new Error(`unsupported dispatch ${String(dispatch.kind)}`);
            }
          } else if (step.op === "serverFrame") {
            const frame = buildServerFrame(scenario.documentId, step.frame!);
            const commandBatch = "Commands" in frame && frame.Commands.envelopes.length > 0 ? new Uint8Array([1, 2, 3]) : null;
            await dependencies.handleHubFrame(state, frame, null, state.socket, commandBatch);
            const socket = state.socket as FakeHubWebSocket | null;
            if (socket) {
              for (const bytes of socket.sent.splice(0)) outgoing.push(clientFrameKind(bytes, dependencies.decodeClientFrame));
            }
          } else if (step.op === "expect") {
            const expect = step.expect!;
            if ("resumeToken" in expect) vitest.expect(state.resumeToken).toEqual(expect.resumeToken ?? null);
            if ("frontierEditId" in expect) vitest.expect(state.frontier?.head_edit_id ?? null).toEqual(expect.frontierEditId ?? null);
            if (typeof expect.remoteKind === "string") vitest.expect(state.status.remote.kind).toBe(expect.remoteKind);
            if (typeof expect.peerCount === "number" && state.status.remote.kind === "live") vitest.expect(state.status.remote.peerCount).toBe(expect.peerCount);
            if (Array.isArray(expect.outboxMutationIds)) vitest.expect(state.outbox.map((row) => row.id)).toEqual(expect.outboxMutationIds);
            if (typeof expect.pendingBatchCount === "number") vitest.expect(state.pendingBatches.size).toBe(expect.pendingBatchCount);
            if (typeof expect.socketActorConfirmed === "boolean") vitest.expect(state.hubActorReady).toBe(expect.socketActorConfirmed);
            if (typeof expect.rebootstrapRequired === "boolean") vitest.expect(state.artifactRebootstrapRequired).toBe(expect.rebootstrapRequired);
            if (Array.isArray(expect.eventKinds)) {
              for (const kind of expect.eventKinds) vitest.expect(observedEvents.includes(String(kind)), `missing event ${kind} in ${JSON.stringify(observedEvents)}`).toBe(true);
            }
            if (Array.isArray(expect.outgoingClientFrameKinds)) vitest.expect(outgoing).toEqual(expect.outgoingClientFrameKinds);
            if (Array.isArray(expect.ingestedMutationIds)) {
              for (const id of expect.ingestedMutationIds) vitest.expect(state.ingestedMutationIds.has(String(id))).toBe(true);
            }
            if (typeof expect.commandOutcome === "string") vitest.expect(lastOutcome).toBe(expect.commandOutcome);
          }
        }
      } finally {
        dependencies.testSeams.workerPostTestSink = priorSink;
        dependencies.artifacts.delete(state.runtimeKey);
      }
    });
  }
}
