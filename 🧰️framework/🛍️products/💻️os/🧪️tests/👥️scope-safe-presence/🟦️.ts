/** 👥️ Scope-safe presence: verified surface authority binds only after the exact socket Session and equal document ids stay isolated. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type ArtifactState = import("../../🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../🟦️.ts").BackboneWorkerResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerScopeSafePresenceTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, handleHubFrame, emitEvent } = dependencies;
  describe("backbone-worker scope-safe presence", () => {
    beforeEach(resetWorkerLaw);
    it("binds verified surface authority only after the exact socket Session and isolates equal document ids", async () => {
      const responses: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => responses.push(message);
      const socketA = { close: vi.fn() } as unknown as WebSocket;
      const socketB = { close: vi.fn() } as unknown as WebSocket;
      const makeState = (spaceId: string, socket: WebSocket): ArtifactState =>
        ({
          config: { documentId: "same-document", schema: "demo/v1", actor: "requested", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "https://hub.example", spaceId }] },
          socket,
          actor: "",
          hubActorReady: false,
          pendingSocketActorId: `hub.v1.${spaceId === "space-a" ? "a" : "b"}`.padEnd(71, spaceId === "space-a" ? "a" : "b"),
          presenceAuthority: null,
          outbox: [],
          sessionColor: null,
        }) as unknown as ArtifactState;
      const stateA = makeState("space-a", socketA);
      const stateB = makeState("space-b", socketB);
      const actorA = stateA.pendingSocketActorId!;
      const actorB = stateB.pendingSocketActorId!;
      try {
        await handleHubFrame(stateA, { Session: { actor: actorA, color: 2 } }, { socket: socketA, scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor" });
        await handleHubFrame(stateB, { Session: { actor: actorB, color: 5 } }, { socket: socketB, scope: { spaceId: "space-b", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#viewer" });
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, label: "Ada", connectedAtMs: 101, color: 2, surface: "map@1/*#editor", views: [] }] });
        emitEvent(stateB, { kind: "presence", peers: [{ actor: actorB, label: "Berta", connectedAtMs: 202, color: 5, surface: "map@1/*#viewer", views: [] }] });
        const presence = responses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "event" }> => message.kind === "event" && message.event.kind === "presence");
        expect(presence.map((message) => [message.scope?.spaceId, message.verifiedSurfaceId, message.event.kind === "presence" ? message.event.peers[0]?.label : undefined])).toEqual([
          ["space-a", "map@1/*#editor", "Ada"],
          ["space-b", "map@1/*#viewer", "Berta"],
        ]);
        emitEvent(stateA, { kind: "presence", peers: [] });
        stateA.presenceAuthority = null;
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, connectedAtMs: 303, surface: "map@1/*#editor", views: [] }] });
        const tail = responses.slice(-2) as Extract<BackboneWorkerResponse, { kind: "event" }>[];
        expect(tail[0]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor", event: { kind: "presence", peers: [] } });
        expect(tail[1]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, event: { kind: "presence", peers: [] } });
        expect(tail[1]?.verifiedSurfaceId).toBeUndefined();
      } finally {
        testSeams.workerPostTestSink = null;
      }
    });
  });
}
