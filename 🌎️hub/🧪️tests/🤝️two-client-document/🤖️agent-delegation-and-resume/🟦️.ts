/** 🤖️ The agent delegation and revocation, presence lease expiry, resume and edit growth.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the two-client document e2e}: a phase of the scenario over the same bindings, moved verbatim. */
import { setTimeout as sleep } from "node:timers/promises";
import { expect } from "vitest";
import type { Frame, Holder, Minted } from "../🧰️scenario-harness/📐️scenario-types/🟦️.ts";

/** 🔑️ Delegates an agent session, commits its edit and checks revocation closes its socket and roster entry. */
export async function runAgentDelegationAndRevocation({ tokenA, fetchTimed, origin, scenario, fixture, openSocket, mint, a, waitFrame, beat, b, rosterWith, encodeClientFrame, encodePackValue, since, rosterWithout }: {
  readonly tokenA: string;
  readonly fetchTimed: (url: string, init?: RequestInit, ms?: number) => Promise<Response>;
  readonly origin: string;
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly fixture: any;
  readonly openSocket: (minted: Minted, resumeToken?: string | null, admitWithinMs?: number) => Promise<Holder>;
  readonly mint: (label: string, token: string) => Promise<Minted>;
  readonly a: Holder;
  readonly waitFrame: (holder: Holder, pred: (f: Frame) => boolean, label: string, ms?: number) => Promise<Frame>;
  readonly beat: (holder: Holder) => void;
  readonly b: Holder;
  readonly rosterWith: (actor: string) => (f: Frame) => boolean;
  readonly encodeClientFrame: any;
  readonly encodePackValue: any;
  readonly since: (holder: Holder) => (pred: (f: Frame) => boolean) => (f: Frame) => boolean;
  readonly rosterWithout: (actor: string) => (f: Frame) => boolean;
}) {
  const humanHeaders = { "content-type": "application/json", authorization: `Bearer ${tokenA}`, origin: "http://127.0.0.1:6066" };
  const delegated = await fetchTimed(`${origin}/auth/agent-delegations`, {
    method: "POST",
    headers: humanHeaders,
    body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: scenario.spaceId, agentLabel: fixture.agent.agentLabel, audience: fixture.agent.audience, ttlSecs: fixture.agent.ttlSecs }),
  });
  const delegationText = await delegated.text();
  expect(delegated.status, delegationText.slice(0, 200)).toBe(201);
  const delegation = JSON.parse(delegationText) as { delegationId: string; token: string };
  const exchanged = await fetchTimed(`${origin}/auth/agent-sessions`, {
    method: "POST",
    headers: { "content-type": "application/json", authorization: `Bearer ${delegation.token}`, origin: "http://127.0.0.1:6066" },
    body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: fixture.agent.audience, agentInstanceId: fixture.agent.agentInstanceId }),
  });
  const exchangedText = await exchanged.text();
  expect(exchanged.status, exchangedText.slice(0, 200)).toBe(200);
  const agentToken = (JSON.parse(exchangedText) as { token: string }).token;
  const agentExecutionTargetMs: number[] = [];
  const agent = await openSocket(await mint("agent", agentToken));
  expect(agent.actor).not.toBe(a.actor);
  await waitFrame(agent, (f) => "Session" in f, "agent Session");
  beat(agent);
  await waitFrame(b, rosterWith(agent.actor), "presence of the agent");
  const agentBatch = fixture.command.batchId + 7;
  agent.socket.send(
    encodeClientFrame(
      {
        Commands: {
          batch_id: agentBatch,
          envelopes: [
            {
              mutation_id: `${fixture.command.mutationIdPrefix}agent-1`,
              document_id: scenario.documentId,
              actor: agent.actor,
              dependencies: [],
              observed: null,
              target: [],
              diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.agent.diffValue)) },
              inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
              timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
              transaction: null,
              verb: null,
            },
          ],
        },
      },
      "command",
    ),
  );
  const agentAck = await waitFrame(agent, (f) => "Ack" in f && f.Ack.batch_id === agentBatch, "agent Ack");
  expect(JSON.stringify(agentAck.Ack.stages), "the delegated agent's own edit commits").toContain("Accepted");
  const agentTargetHeaders = { "content-type": "application/json", authorization: `Bearer ${agentToken}` };
  const agentTargetIntent = JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId: scenario.spaceId, documentId: scenario.documentId }, requestedSurfaceId: scenario.surfaceId, clientInstanceId: "tc-agent-target" });
  for (const part of ["manifest", "descriptor"]) {
    const askedAt = Date.now();
    const answered = await fetchTimed(`${origin}/spaces/${encodeURIComponent(scenario.spaceId)}/documents/${encodeURIComponent(scenario.documentId)}/execution-target/${part}`, { method: "POST", headers: agentTargetHeaders, body: agentTargetIntent });
    const answeredText = part === "manifest" ? await answered.text() : `${(await answered.arrayBuffer()).byteLength} bytes`;
    const answeredMs = Date.now() - askedAt;
    expect(answered.status, `the agent's execution-target ${part} after its own commit: ${answeredText.slice(0, 200)}`).toBe(200);
    expect(answeredMs, `the agent's execution-target ${part} answered within the bound`).toBeLessThanOrEqual(fixture.agent.executionTargetWithinMs);
    agentExecutionTargetMs.push(answeredMs);
  }
  const afterRevocation = since(b);
  const revokedAt = Date.now();
  const revoked = await fetchTimed(`${origin}/auth/agent-delegations/${encodeURIComponent(delegation.delegationId)}/revoke`, { method: "POST", headers: humanHeaders });
  expect(revoked.status).toBe(204);
  const agentClosed = await Promise.race([agent.closed, sleep(fixture.agent.revocationWithinMs).then(() => null)]);
  expect(agentClosed?.code, "the agent's open document socket closes on revocation").toBe(fixture.agent.closeCode);
  await waitFrame(b, afterRevocation(rosterWithout(agent.actor)), "roster after the agent was revoked", fixture.agent.revocationWithinMs);
  const agentRevocationMs = Date.now() - revokedAt;
  expect(agentRevocationMs, "revocation reached the socket and the roster within the declared bound").toBeLessThanOrEqual(fixture.agent.revocationWithinMs);
  const agentAfter = await fetchTimed(`${origin}/auth/sessions/me`, { headers: { authorization: `Bearer ${agentToken}` } });
  expect(agentAfter.status, "the revoked agent's next request is refused").toBe(fixture.agent.refusedStatus);
  const reexchanged = await fetchTimed(`${origin}/auth/agent-sessions`, {
    method: "POST",
    headers: { "content-type": "application/json", authorization: `Bearer ${delegation.token}`, origin: "http://127.0.0.1:6066" },
    body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: fixture.agent.audience, agentInstanceId: fixture.agent.agentInstanceId }),
  });
  const reexchangedText = await reexchanged.text();
  expect(reexchanged.status, `a revoked delegation mints no further session: ${reexchangedText.slice(0, 200)}`).toBe(fixture.agent.reexchangeStatus);
  expect(JSON.parse(reexchangedText).error).toBe(fixture.agent.reexchangeError);
  return { agentRevocationMs, agentExecutionTargetMs };
}

/** 📈️ Checks presence lease expiry, resumes a closed socket and grows the edit history before the restart. */
export async function runPresenceExpiryAndGrowth({ since, b, beat, a, waitFrame, roster, fixture, encodeClientFrame, envelope, encodePackValue, scenario, openSocket, mint, tokenB, ack }: {
  readonly since: (holder: Holder) => (pred: (f: Frame) => boolean) => (f: Frame) => boolean;
  readonly b: Holder;
  readonly beat: (holder: Holder) => void;
  readonly a: Holder;
  readonly waitFrame: (holder: Holder, pred: (f: Frame) => boolean, label: string, ms?: number) => Promise<Frame>;
  readonly roster: (frame: Frame) => any[];
  readonly fixture: any;
  readonly encodeClientFrame: any;
  readonly envelope: { mutation_id: string; document_id: string; actor: string; dependencies: never[]; observed: null; target: never[]; diff: { schema: any; payload: unknown[]; }; inverse: { schema: any; payload: unknown[]; }; timestamp: { actor: number; physical_ms: number; logical: number; }; transaction: null; verb: null; };
  readonly encodePackValue: any;
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly openSocket: (minted: Minted, resumeToken?: string | null, admitWithinMs?: number) => Promise<Holder>;
  readonly mint: (label: string, token: string) => Promise<Minted>;
  readonly tokenB: string;
  readonly ack: Frame;
}) {
  const afterExpiry = since(b);
  beat(a);
  const expired = await waitFrame(b, afterExpiry((f) => "Presence" in f && roster(f).some((peer) => peer.actor === a.actor && peer.ui === undefined)), "roster after A's lease expired", fixture.presence.leaseTtlMs + 5000);
  expect(roster(expired).find((peer) => peer.actor === a.actor)?.label).toBe("Ada TC");
  expect(a.socket.readyState).toBe(WebSocket.OPEN);

  const resume = b.resumeToken!;
  expect(resume.length).toBeGreaterThan(0);
  b.socket.close();
  await sleep(500);
  const mutationId2 = `${fixture.command.mutationIdPrefix}2`;
  a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId + 1, envelopes: [{ ...envelope, mutation_id: mutationId2, diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({ value: "two-client-a2" })) } }] } }, "command"));
  await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId + 1, "Ack2");
  const grow = async (holder: Holder, from: number, count: number, parent: string): Promise<string> => {
    let previous = parent;
    for (let index = from; index < from + count; index++) {
      const growthId = `${fixture.growth.mutationIdPrefix}${index}`;
      const batchId = fixture.growth.batchIdBase + index;
      const value = { [`grow-${index % fixture.growth.distinctPaths}`]: `${index}:${"g".repeat(fixture.growth.payloadBytes)}` };
      holder.socket.send(
        encodeClientFrame(
          {
            Commands: {
              batch_id: batchId,
              envelopes: [
                {
                  mutation_id: growthId,
                  document_id: scenario.documentId,
                  actor: holder.actor,
                  dependencies: [previous],
                  observed: null,
                  target: [],
                  diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(value)) },
                  inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({})) },
                  timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
                  transaction: null,
                  verb: null,
                },
              ],
            },
          },
          "command",
        ),
      );
      const grown = await waitFrame(holder, (f) => "Ack" in f && f.Ack.batch_id === batchId, `growth Ack ${index}`, 60_000).catch(async (error: Error) => {
        const closed = await Promise.race([holder.closed, sleep(10).then(() => undefined)]);
        throw new Error(`${error.message} socket=${closed ? `closed ${closed.code} ${closed.reason}` : "open"}\nhub-output:\n${scenario.hubOutput().slice(-12000)}`);
      });
      expect(JSON.stringify(grown.Ack.stages), `growth edit ${index}: ${JSON.stringify(grown.Ack)}`).toContain("Accepted");
      previous = growthId;
    }
    return previous;
  };
  const grownAt = Date.now();
  const lastGrown = await grow(a, 0, fixture.growth.editsBeforeRestart, mutationId2);
  const growthMsBeforeRestart = Date.now() - grownAt;
  const b2 = await openSocket(await mint("b-re", tokenB), resume);
  const recovered =
    b2.frames.some((f) => "Commands" in f && f.Commands.envelopes?.some((e: any) => e.mutation_id === mutationId2)) ||
    b2.frames.some((f) => "RebootstrapRequired" in f) ||
    (b2.welcome.server_frontier.last_commit_seq ?? 0) >= (ack.Ack.frontier.last_commit_seq ?? 0);
  expect(recovered).toBe(true);
  const frontierBefore = b2.welcome.server_frontier;
  b2.socket.close();
  return { growthMsBeforeRestart, grow, lastGrown, frontierBefore };
}
