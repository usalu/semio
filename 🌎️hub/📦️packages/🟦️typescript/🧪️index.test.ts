// #region Header
/**
 * 🧪️ Hub e2e — boots the real `os-hub` binary and drives it with two independent
 * `DirectoryClient`s plus raw document-WS wire frames to prove the hub's collaboration contract
 * (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0-C2, lane 3-E): studio
 * creation + visibility, live membership over `/directory/ws`, surface-scoped presence vs.
 * document-scoped command relay on `/spaces/{space}/documents/index/ws`, an admin kick, and
 * survival of a real hub restart against the same `OS_HUB_DATA`.
 *
 * Gated behind `HUB_E2E=1` — it compiles/boots a real server, so the default `bun nx run
 * os-hub-ts:test` must stay fast; without the env var this test reports as skipped in well under
 * a second. Run it for real: `HUB_E2E=1 bun nx run os-hub-ts:test` (`📜️script.ts`'s `TestScript`
 * builds the binary first, default cargo features only — contract-freeze Amendment 2, never
 * `--all-features`).
 */
// #endregion Header

import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  type ArtifactPresencePeer,
  type ClientFrame,
  type ConnectionView,
  type DirectoryStreamMessage,
  type ServerFrame,
  DirectoryClient,
  decodePresencePeer,
  decodeServerFrame,
  encodeClientFrame,
  encodePresencePeer,
  isDirectoryEventBodyKind,
  isDirectoryStreamMessageKind,
} from "@semio-tech/framework-os";
import { expect, it } from "vitest";
import { type HubHandle, findFreePort, getWorkspaceRoot, startHub } from "./📦️index.ts";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 240_000;
const EDITOR_SURFACE = "s.space.space@1/*#editor";
const VIEWER_SURFACE = "s.space.space@1/*#viewer";

//#region 🔖️Polling
/** ⏳️ Polls `predicate` until it's true or `timeoutMs` elapses. */
async function waitUntil(predicate: () => boolean, timeoutMs = 5_000, intervalMs = 50): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (predicate()) return;
    await new Promise((resolveDelay) => setTimeout(resolveDelay, intervalMs));
  }
  if (!predicate()) throw new Error("waitUntil: condition never became true within budget");
}
//#endregion 🔖️Polling

//#region 🔖️FrameSocket
/** 📡️ A raw document-WS connection speaking `protocol_wire`'s binary frames via
 * `@semio-tech/framework-os`'s `encodeClientFrame`/`decodeServerFrame` — never hand-rolled. */
type FrameSocket = {
  readonly ws: WebSocket;
  next(timeoutMs?: number): Promise<ServerFrame>;
  waitFor(predicate: (frame: ServerFrame) => boolean, timeoutMs?: number): Promise<ServerFrame>;
  send(frame: ClientFrame): void;
  close(): void;
};

/** 🔌️ Opens a document WS, wiring an inbox that either resolves a pending `next()`/`waitFor()`
 * call immediately or queues the frame for the next call. */
function openFrameSocket(url: string): Promise<FrameSocket> {
  return new Promise((resolveOpen, rejectOpen) => {
    const ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    const queue: ServerFrame[] = [];
    let waiters: Array<(error: Error | null, frame?: ServerFrame) => void> = [];
    let closedWith: Error | null = null;
    ws.onmessage = (event: MessageEvent) => {
      const bytes = new Uint8Array(event.data as ArrayBuffer);
      const { frame } = decodeServerFrame(bytes);
      const waiter = waiters.shift();
      if (waiter) waiter(null, frame);
      else queue.push(frame);
    };
    ws.onerror = () => rejectOpen(new Error(`openFrameSocket: failed to open ${url}`));
    ws.onopen = () => {
      ws.onerror = () => {
        closedWith = new Error(`openFrameSocket: ${url} errored`);
        for (const waiter of waiters.splice(0)) waiter(closedWith);
      };
      ws.onclose = (event: CloseEvent) => {
        closedWith = new Error(`openFrameSocket: ${url} closed (code ${event.code}, reason ${JSON.stringify(event.reason)})`);
        for (const waiter of waiters.splice(0)) waiter(closedWith);
      };
      const next = (timeoutMs = 5_000): Promise<ServerFrame> => {
        const queued = queue.shift();
        if (queued) return Promise.resolve(queued);
        if (closedWith) return Promise.reject(closedWith);
        return new Promise((resolveNext, rejectNext) => {
          const timer = setTimeout(() => {
            waiters = waiters.filter((waiter) => waiter !== onFrame);
            rejectNext(new Error("openFrameSocket: timed out waiting for a frame"));
          }, timeoutMs);
          const onFrame = (error: Error | null, frame?: ServerFrame): void => {
            clearTimeout(timer);
            if (error) rejectNext(error);
            else resolveNext(frame as ServerFrame);
          };
          waiters.push(onFrame);
        });
      };
      const waitFor = async (predicate: (frame: ServerFrame) => boolean, timeoutMs = 5_000): Promise<ServerFrame> => {
        const deadline = Date.now() + timeoutMs;
        while (Date.now() < deadline) {
          const frame = await next(Math.max(50, deadline - Date.now()));
          if (predicate(frame)) return frame;
        }
        throw new Error("openFrameSocket: timed out waiting for a matching frame");
      };
      resolveOpen({
        ws,
        next,
        waitFor,
        send: (frame: ClientFrame) => ws.send(encodeClientFrame(frame, "command")),
        close: () => ws.close(),
      });
    };
  });
}

function helloFrame(actor: string, token: string): ClientFrame {
  return { Hello: { wire_version: 1, protocol_version: 1, schema: "s.space.space@1/*", pack_schema_hash: new Array(32).fill(0), actor, token, resume_token: null, frontier: null } };
}

function presenceFrame(actor: string): ClientFrame {
  const peer: ArtifactPresencePeer = { actor, connectedAtMs: Date.now() };
  return { Presence: { peer: encodePresencePeer(peer) } };
}

function commandsFrame(batchId: number, documentId: string, actor: string, mutationId: string): ClientFrame {
  return {
    Commands: {
      batch_id: batchId,
      envelopes: [
        {
          mutation_id: mutationId,
          document_id: documentId,
          actor,
          dependencies: [],
          diff: { schema: "e2e.opaque.v1", payload: [] },
          inverse: { schema: "e2e.opaque.v1", payload: [] },
          timestamp: { actor: 0, physical_ms: Date.now(), logical: 0 },
        },
      ],
    },
  };
}

/** 🪣️ Collects every frame `socket` receives within `windowMs`, stopping early only on the
 * final per-call timeout — used to assert something about a whole burst of activity rather than
 * exactly one frame, which real network/broadcast timing makes fragile to pin down. */
async function drainFrames(socket: FrameSocket, windowMs: number): Promise<ServerFrame[]> {
  const frames: ServerFrame[] = [];
  const deadline = Date.now() + windowMs;
  for (;;) {
    const remaining = deadline - Date.now();
    if (remaining <= 0) return frames;
    try {
      frames.push(await socket.next(remaining));
    } catch {
      return frames;
    }
  }
}

function presenceActors(frame: ServerFrame): Set<string> {
  if (!("Presence" in frame)) throw new Error("presenceActors: not a Presence frame");
  return new Set(frame.Presence.peers.map((raw) => decodePresencePeer(new Uint8Array(raw), [0]).actor));
}
//#endregion 🔖️FrameSocket

//#region 🔖️Scenario
it.skipIf(!HUB_E2E)(
  "boots the real hub and proves directory + presence-per-surface + document-scoped commands + admin kick + restart persistence",
  async () => {
    const repoRoot = getWorkspaceRoot();
    const dataDir = mkdtempSync(join(tmpdir(), "os-hub-e2e-"));
    const adminToken = "e2e-admin";
    const sockets: FrameSocket[] = [];
    let hub: HubHandle | null = null;

    try {
      //#region 🔖️Boot
      hub = await startHub({ repoRoot, dataDir, adminToken });
      //#endregion

      //#region 🔖️Sessions
      const client1 = new DirectoryClient(hub.baseUrl);
      const client2 = new DirectoryClient(hub.baseUrl);
      const session1 = await client1.mintSession("user1@semio.dev");
      const session2 = await client2.mintSession("user2@semio.dev");
      const me1 = await client1.me();
      const me2 = await client2.me();
      expect(me1?.email).toBe("user1@semio.dev");
      expect(me2?.email).toBe("user2@semio.dev");
      //#endregion

      //#region 🔖️CreateSpace
      const created = await client1.command({ kind: "create-space", name: "E2E Studio", spaceKind: "studio", visibility: "private" });
      const spaceCreated = created.events.find((event) => isDirectoryEventBodyKind(event.body, "space.created"));
      expect(spaceCreated).toBeDefined();
      if (!spaceCreated || !isDirectoryEventBodyKind(spaceCreated.body, "space.created")) throw new Error("unreachable");
      const spaceId = spaceCreated.body.spaceId;
      expect(spaceCreated.body.name).toBe("E2E Studio");
      expect(spaceCreated.body.spaceKind).toBe("studio");
      const spacesForUser2Before = await client2.spaces();
      expect(spacesForUser2Before.some((space) => space.id === spaceId)).toBe(false);
      //#endregion

      //#region 🔖️Membership
      const streamMessages2: DirectoryStreamMessage[] = [];
      const stream2 = client2.stream(0, (message) => streamMessages2.push(message));
      await client1.command({ kind: "upsert-member", spaceId, email: "user2@semio.dev", role: "author" });
      await waitUntil(
        () =>
          streamMessages2.some(
            (message) => isDirectoryStreamMessageKind(message, "event") && isDirectoryEventBodyKind(message.event.body, "member.upserted") && message.event.body.spaceId === spaceId && message.event.body.userId === session2.userId,
          ),
        5_000,
      );
      stream2.close();
      const spacesForUser2After = await client2.spaces();
      const memberSpace = spacesForUser2After.find((space) => space.id === spaceId);
      expect(memberSpace?.role).toBe("author");
      //#endregion

      //#region 🔖️PresenceAndCommands
      const actorA = `user:${session1.userId}#e2e-a`;
      const actorB = `user:${session2.userId}#e2e-b`;
      const actorC = `user:${session1.userId}#e2e-c`;
      const documentWsUrl = (surface: string): string => `${hub!.wsBaseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index/ws?surface=${encodeURIComponent(surface)}`;

      const sockA = await openFrameSocket(documentWsUrl(EDITOR_SURFACE));
      sockets.push(sockA);
      sockA.send(helloFrame(actorA, session1.token));
      await sockA.waitFor((frame) => "Welcome" in frame);

      const sockB = await openFrameSocket(documentWsUrl(EDITOR_SURFACE));
      sockets.push(sockB);
      sockB.send(helloFrame(actorB, session2.token));
      await sockB.waitFor((frame) => "Welcome" in frame);

      sockA.send(presenceFrame(actorA));
      sockB.send(presenceFrame(actorB));
      const rosterA = presenceActors(await sockA.waitFor((frame) => "Presence" in frame && frame.Presence.peers.length === 2));
      const rosterB = presenceActors(await sockB.waitFor((frame) => "Presence" in frame && frame.Presence.peers.length === 2));
      expect(rosterA).toEqual(new Set([actorA, actorB]));
      expect(rosterB).toEqual(new Set([actorA, actorB]));

      const sockC = await openFrameSocket(documentWsUrl(VIEWER_SURFACE));
      sockets.push(sockC);
      sockC.send(helloFrame(actorC, session1.token));
      await sockC.waitFor((frame) => "Welcome" in frame);
      sockC.send(presenceFrame(actorC));

      // 🔬️ Drain (rather than assert zero) a short window here: `handle_client_frame`'s
      // `ClientFrame::Presence` arm does `presence.insert(...)` then a separate
      // `presence_peers(...)` snapshot read (`📦️bin.rs` ~566-568) — under near-simultaneous
      // presence sends from A and B above, this can legitimately produce one extra, IDENTICAL
      // [actorA, actorB] broadcast (a benign race in the ephemeral/best-effort presence lane, not
      // a correctness bug — observed in `🧪️3-e-hub-e2e-run1.txt`). The actual contract this step
      // proves — C's surface never appears in A/B's roster — is checked directly below regardless
      // of how many (harmless) frames arrive.
      for (const frame of await drainFrames(sockA, 500)) {
        if (!("Presence" in frame)) continue;
        const actors = presenceActors(frame);
        expect(actors.has(actorC)).toBe(false);
        expect(actors.size).toBeLessThanOrEqual(2);
      }
      for (const frame of await drainFrames(sockB, 500)) {
        if (!("Presence" in frame)) continue;
        const actors = presenceActors(frame);
        expect(actors.has(actorC)).toBe(false);
        expect(actors.size).toBeLessThanOrEqual(2);
      }

      const documentId = `${spaceId}:index`;
      const mutationId = crypto.randomUUID();
      sockA.send(commandsFrame(1, documentId, actorA, mutationId));
      const ack = await sockA.waitFor((frame) => "Ack" in frame && frame.Ack.batch_id === 1);
      if (!("Ack" in ack)) throw new Error("unreachable");
      const applied = ack.Ack.stages.find((stage) => typeof stage === "object" && "Applied" in stage);
      expect(applied && typeof applied === "object" && "Applied" in applied && applied.Applied.outcome === "Accepted").toBe(true);

      const commandsOnB = await sockB.waitFor((frame) => "Commands" in frame);
      const commandsOnC = await sockC.waitFor((frame) => "Commands" in frame);
      if (!("Commands" in commandsOnB) || !("Commands" in commandsOnC)) throw new Error("unreachable");
      expect(commandsOnB.Commands.envelopes[0]?.mutation_id).toBe(mutationId);
      expect(commandsOnC.Commands.envelopes[0]?.mutation_id).toBe(mutationId);
      //#endregion

      //#region 🔖️AdminKick
      const connections = (await (await fetch(`${hub.baseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${adminToken}` } })).json()) as ConnectionView[];
      expect(connections.length).toBe(3);
      const surfaces = connections.map((connection) => connection.surface).sort();
      expect(surfaces).toEqual([EDITOR_SURFACE, EDITOR_SURFACE, VIEWER_SURFACE].sort());
      const connectionC = connections.find((connection) => connection.actor === actorC);
      expect(connectionC).toBeDefined();
      if (!connectionC) throw new Error("unreachable");

      let sockCClosed = false;
      sockC.ws.onclose = () => {
        sockCClosed = true;
      };
      const closeResponse = await fetch(`${hub.baseUrl}/admin/api/connections/${encodeURIComponent(connectionC.syncSessionId)}/close`, { method: "POST", headers: { authorization: `Bearer ${adminToken}` } });
      expect(closeResponse.status).toBe(204);
      await waitUntil(() => sockCClosed, 5_000);
      //#endregion

      //#region 🔖️RestartPersistence
      const beforeRestartStatus = (await (await fetch(`${hub.baseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index`, { headers: { authorization: `Bearer ${session1.token}` } })).json()) as { commit_seq: number; head_seq: number };
      expect(beforeRestartStatus.commit_seq).toBeGreaterThan(0);

      sockA.close();
      sockB.close();
      const oldHub = hub;
      hub = null;
      await oldHub.stop();

      hub = await startHub({ repoRoot, dataDir, adminToken, port: await findFreePort() });
      const client1AfterRestart = new DirectoryClient(hub.baseUrl);
      const sessionAfterRestart = await client1AfterRestart.mintSession("user1@semio.dev");
      const spacesAfterRestart = await client1AfterRestart.spaces();
      const spaceAfterRestart = spacesAfterRestart.find((space) => space.id === spaceId);
      expect(spaceAfterRestart?.role).toBe("author");

      const detailAfterRestart = await client1AfterRestart.space(spaceId);
      expect(detailAfterRestart.members.some((member) => member.userId === session2.userId && member.role === "author")).toBe(true);

      const afterRestartStatus = (await (await fetch(`${hub.baseUrl}/spaces/${encodeURIComponent(spaceId)}/documents/index`, { headers: { authorization: `Bearer ${sessionAfterRestart.token}` } })).json()) as { commit_seq: number; head_seq: number };
      expect(afterRestartStatus.commit_seq).toBe(beforeRestartStatus.commit_seq);
      expect(afterRestartStatus.head_seq).toBe(beforeRestartStatus.head_seq);
      //#endregion
    } finally {
      for (const socket of sockets) socket.close();
      if (hub) await hub.stop();
      rmSync(dataDir, { recursive: true, force: true });
    }
  },
  TEST_TIMEOUT_MS,
);
//#endregion 🔖️Scenario
