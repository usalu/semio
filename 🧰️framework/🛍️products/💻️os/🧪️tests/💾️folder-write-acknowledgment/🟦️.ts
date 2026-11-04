/** 💾️ LAW: a folder document is saved only after the folder acknowledged an archive that covers every admitted edit. Writes run
 * one at a time in submission order, a newer archive supersedes a queued older one, an edit admitted while a write is in flight
 * stays unsaved, and a refused write keeps its edit pending, reports unsaved and is retried. The corpus is language-agnostic
 * (`🏪️store/🧪️tests/🧫️fixtures/💾️folder-write-acknowledgment`, the `worker` section); the native actor twin is `💾️folder-write-acknowledgment` in the store sync tests. */
import { encodeDocumentBackboneEnvelopeBatchExact } from "@semio-tech/framework-replication";
import { encodeBackboneMessage, encodeDocumentArchiveBytes, encodePackValue, type ArtifactActorConfig, type BackboneWorkerRequest, type BackboneWorkerResponse } from "../../🟦️.ts";
import corpus from "../../🔨️modules/🏪️store/🧪️tests/🧫️fixtures/💾️folder-write-acknowledgment/🔣️.json" with { type: "json" };

type Step = Readonly<{ admit?: string; archive?: string; folder?: "hold" | "accept" | "refuse"; release?: string; retry?: "accept"; expect?: Readonly<{ pending: number; persisted: boolean; puts: readonly string[] }> }>;

export async function registerFolderWriteAcknowledgmentTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  worker: {
    readonly testSeams: { workerPostTestSink: ((message: BackboneWorkerResponse) => void) | null };
    readonly artifactState: (documentId: string, spaceId?: string) => { readonly pendingMutations: readonly unknown[]; readonly status: { readonly persisted: boolean } } | undefined;
    readonly closeArtifact: (documentId: string, spaceId: string | undefined, clientInstanceId: string) => void;
    readonly handleTsRequest: (request: BackboneWorkerRequest) => void;
    readonly installStreamMuxEndpoint: (endpoint: null) => void;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const binding = { kind: "folder", dataClass: "persistedLocalOnly", path: "/tmp/folder-write-acknowledgment" } as const;
  const settle = async (done: () => boolean): Promise<void> => {
    for (let turn = 0; turn < 400 && !done(); turn += 1) await new Promise((resolve) => setTimeout(resolve, 5));
    expect(done()).toBe(true);
  };
  const archiveBytes = (name: string) => Array.from(encodeDocumentArchiveBytes({ parent_pack: Array.from(new TextEncoder().encode(name)), parent_spr: [], members: [] }));
  const nameOf = (body: Uint8Array) => ["a1", "a2", "a3"].find((name) => new TextDecoder("latin1").decode(body).includes(name)) ?? "?";

  describe("folder write acknowledgment", () => {
    for (const scenario of corpus.cases as readonly { readonly name: string; readonly steps: readonly Step[] }[]) {
      it(scenario.name, async () => {
        const documentId = `folder-write-${scenario.name}`;
        const clientInstanceId = crypto.randomUUID();
        const puts: string[] = [];
        const held = new Map<string, () => void>();
        let mode: "hold" | "accept" | "refuse" = "accept";
        let inFlight = 0;
        let overlapped = false;
        const originalFetch = globalThis.fetch;
        (globalThis as unknown as { fetch: unknown }).fetch = async (_url: string, init?: RequestInit) => {
          if (init?.method !== "PUT") return new Response(null, { status: 204 });
          const name = nameOf(init.body as Uint8Array);
          inFlight += 1;
          overlapped ||= inFlight > 1;
          try {
            if (mode === "hold") await new Promise<void>((resolve) => held.set(name, resolve));
            if (mode === "refuse") return new Response(null, { status: 500 });
            puts.push(name);
            return new Response(null, { status: 200 });
          } finally {
            inFlight -= 1;
          }
        };
        worker.installStreamMuxEndpoint(null);
        worker.testSeams.workerPostTestSink = () => {};
        const actor = "actor-1";
        let edits = 0;
        const send = (message: Extract<BackboneWorkerRequest, { kind: "send" }>["message"]) => worker.handleTsRequest({ kind: "send", documentId, clientInstanceId, message });
        try {
          worker.handleTsRequest({ kind: "open", actorIdentityGrant: { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 }, clientInstanceId, documentId, schema: "demo/v1", bindings: [binding], watchExternal: false, actor } as ArtifactActorConfig & { readonly kind: "open"; readonly clientInstanceId: string });
          await settle(() => worker.artifactState(documentId) !== undefined);
          for (const step of scenario.steps) {
            if (step.admit !== undefined) {
              edits += 1;
              const before = worker.artifactState(documentId)!.pendingMutations.length;
              send({ kind: "documentBackbone", message: encodeBackboneMessage({ kind: "mutations", envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: `${actor}-${step.admit}`, document_id: documentId, actor, dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue({ edit: step.admit }) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: BigInt(1_000 + edits), logical: 0n }, transaction: null, verb: null, line: null }]) }) });
              await settle(() => worker.artifactState(documentId)!.pendingMutations.length === before + 1);
            }
            if (step.archive !== undefined) {
              mode = step.folder ?? "accept";
              send({ kind: "localDocumentArchive", archive: archiveBytes(step.archive) });
              await new Promise((resolve) => setTimeout(resolve, 20));
            }
            if (step.release !== undefined) {
              await settle(() => held.has(step.release!));
              mode = "accept";
              held.get(step.release)!();
              held.delete(step.release);
              await new Promise((resolve) => setTimeout(resolve, 20));
            }
            if (step.retry !== undefined) mode = step.retry;
            if (step.expect !== undefined) {
              const expected = step.expect;
              await settle(() => worker.artifactState(documentId)!.pendingMutations.length === expected.pending && worker.artifactState(documentId)!.status.persisted === expected.persisted && puts.length === expected.puts.length);
              expect(puts).toEqual(expected.puts);
            }
          }
          expect(overlapped).toBe(false);
        } finally {
          for (const release of held.values()) release();
          worker.closeArtifact(documentId, undefined, clientInstanceId);
          worker.testSeams.workerPostTestSink = null;
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        }
      });
    }
  });
}
