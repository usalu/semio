/** 🔌️ Opening both sockets, presence replay, the first command, its remote delivery and the late join.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the two-client document e2e}: a phase of the scenario over the same bindings, moved verbatim. */
import { expect } from "vitest";
import type { Frame, Holder, Minted } from "../🧰️scenario-harness/📐️scenario-types/🟦️.ts";

/** 📨️ Opens both sockets, replays presence, delivers the first command and checks the late join. */
export async function runFirstCommandAndLateJoin({ mint, tokenA, tokenB, scenario, openSocket, waitFrame, decodePresencePeer, encodeClientFrame, encodePresencePeer, fixture, encodePackValue, signIn, BO }: {
  readonly mint: (label: string, token: string) => Promise<Minted>;
  readonly tokenA: string;
  readonly tokenB: string;
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly openSocket: (minted: Minted, resumeToken?: string | null, admitWithinMs?: number) => Promise<Holder>;
  readonly waitFrame: (holder: Holder, pred: (f: Frame) => boolean, label: string, ms?: number) => Promise<Frame>;
  readonly decodePresencePeer: any;
  readonly encodeClientFrame: any;
  readonly encodePresencePeer: any;
  readonly fixture: any;
  readonly encodePackValue: any;
  readonly signIn: (email: string, password: string, device: string) => Promise<string>;
  readonly BO: { email: string; password: string; display: string; };
}) {
  const mintedA = await mint("a", tokenA);
  const mintedB = await mint("b", tokenB);
  scenario.artifactSchema = mintedA.artifactSchema;
  scenario.packSchemaHashHex = mintedA.packSchemaHashHex;
  scenario.surfaceId = mintedA.surfaceId;
  const a = await openSocket(mintedA);
  const b = await openSocket(mintedB);
  await waitFrame(a, (f) => "Session" in f, "Session");
  await waitFrame(b, (f) => "Session" in f, "Session");

  const roster = (frame: Frame) => (frame.Presence.peers as number[][]).map((peer) => decodePresencePeer(Uint8Array.from(peer), [0]));
  const rosterActors = (frame: Frame): string[] => roster(frame).map((peer) => peer.actor);
  const beat = (holder: Holder) =>
    holder.socket.send(encodeClientFrame({ Presence: { peer: encodePresencePeer({ actor: holder.actor, connectedAtMs: 0, label: fixture.presence.labelA, views: [], ui: { hoveredPath: fixture.presence.hoveredPath } }) } }, fixture.presence.lane));
  const rosterWith = (actor: string) => (f: Frame) => "Presence" in f && rosterActors(f).includes(actor);
  const rosterWithout = (actor: string) => (f: Frame) => "Presence" in f && !rosterActors(f).includes(actor);
  const since = (holder: Holder) => {
    const seen = new Set(holder.frames);
    return (pred: (f: Frame) => boolean) => (f: Frame) => !seen.has(f) && pred(f);
  };
  beat(a);
  const presenceOfA = await waitFrame(b, (f) => rosterWith(a.actor)(f) && roster(f).some((peer) => peer.actor === a.actor && peer.ui?.hoveredPath === fixture.presence.hoveredPath), "presence of A");
  expect(roster(presenceOfA).find((peer) => peer.actor === a.actor)?.label).toBe("Ada TC");

  const mutationId = `${fixture.command.mutationIdPrefix}1`;
  const envelope = {
    mutation_id: mutationId,
    document_id: scenario.documentId,
    actor: a.actor,
    dependencies: [],
    observed: null,
    target: [],
    diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.diffValue)) },
    inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
    timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
    transaction: null,
    verb: null,
  };
  a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId, envelopes: [envelope] } }, "command"));
  const ack = await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId, "Ack");
  expect(JSON.stringify(ack.Ack.stages), JSON.stringify(ack.Ack.stages)).toContain("Accepted");
  expect(ack.Ack.frontier.document_id ?? ack.Ack.frontier.documentId).toBe(scenario.documentId);

  const remote = await waitFrame(b, (f) => "Commands" in f && f.Commands.envelopes?.[0]?.mutation_id === mutationId, "Commands");
  expect(remote.Commands.envelopes[0].mutation_id).toBe(mutationId);
  expect(remote.Commands.envelopes[0].document_id).toBe(scenario.documentId);
  expect(remote.Commands.origin).toBe(a.actor);

  const tokenC = await signIn(BO.email, BO.password, "device-bo-late");
  beat(a);
  await waitFrame(b, rosterWith(a.actor), "presence of A before the late join");
  const c = await openSocket(await mint("c-late", tokenC));
  expect(c.actor).not.toBe(b.actor);
  expect(c.welcome.server_frontier.last_commit_seq ?? 0).toBeGreaterThanOrEqual(ack.Ack.frontier.last_commit_seq ?? 0);
  await waitFrame(c, rosterWith(a.actor), "join replay of A");
  beat(c);
  await waitFrame(b, rosterWith(c.actor), "presence of the late joiner");
  const afterLeave = since(b);
  c.socket.close();
  await waitFrame(b, afterLeave(rosterWithout(c.actor)), "roster after the late joiner left");
  return { a, beat, b, rosterWith, since, rosterWithout, roster, envelope, ack };
}
