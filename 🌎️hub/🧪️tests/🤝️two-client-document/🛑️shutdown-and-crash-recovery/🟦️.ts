/** 🛑️ The receipt, the graceful shutdown and restart and the crash recovery of the hub.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the two-client document e2e}: a phase of the scenario over the same bindings, moved verbatim. */
import type { LocalProfile } from "../../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { withBuiltLocalHubHostV1, stopLocalHubV1, waitForLocalHubExitV1, startLocalHub, finishLocalHub } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import Ajv from "ajv";
import { setTimeout as sleep } from "node:timers/promises";
import { expect } from "vitest";
import type { Holder, Minted } from "../🧰️scenario-harness/📐️scenario-types/🟦️.ts";

/** 🧯️ Writes the receipt notes, stops the hub gracefully and times its exit and restart spawn. */
export async function runGracefulShutdown({ backend, fixture, growthMsBeforeRestart, agentRevocationMs, scenario, agentExecutionTargetMs, writerFence, run, openSocket, mint, host, profiles, port }: {
  readonly backend: string | undefined;
  readonly fixture: any;
  readonly growthMsBeforeRestart: number;
  readonly agentRevocationMs: number;
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly agentExecutionTargetMs: number[];
  readonly writerFence: any;
  readonly run: Awaited<ReturnType<typeof startLocalHub>>;
  readonly openSocket: (minted: Minted, resumeToken?: string | null, admitWithinMs?: number) => Promise<Holder>;
  readonly mint: (label: string, token: string) => Promise<Minted>;
  readonly host: Parameters<Parameters<typeof withBuiltLocalHubHostV1>[2]>[0];
  readonly profiles: readonly LocalProfile[];
  readonly port: number;
}) {
  const receipt: Record<string, unknown> = { backend };
  const note = (key: string, value: unknown) => {
    receipt[key] = value;
    console.log(`[two-client] backend=${backend} ${key}=${JSON.stringify(value)}`);
  };
  note("growthEditsAcceptedBeforeRestart", fixture.growth.editsBeforeRestart);
  note("growthMsBeforeRestart", growthMsBeforeRestart);
  note("agentRevocationMs", agentRevocationMs);
  note("directoryTimings", scenario.directoryTimings);
  note("agentExecutionTargetMs", agentExecutionTargetMs);
  const fenceRelease = writerFence.backends.find((row: any) => row.backend === backend)?.crashRelease as string | undefined;
  const exitOf = async (hubRun: typeof run, ms: number) => {
    const started = Date.now(), result = await waitForLocalHubExitV1(hubRun);
    expect(Date.now() - started, hubRun.output().slice(-8000)).toBeLessThanOrEqual(ms);
    return { code: result.status, signal: result.signal };
  };
  const reopen = async (label: string, token: string, until: number) => {
    let refused = 0;
    for (;;) {
      try {
        return { holder: await openSocket(await mint(`${label}-${refused}`, token), null, fixture.shutdown.reopenWithinMs), refused };
      } catch (error) {
        refused++;
        if (Date.now() >= until) throw error;
        await sleep(500);
      }
    }
  };

  const writerProbe=backend!=="sqlite";
  const liveWriters=():number=>{throw Error("External writer assertions require a genuine owned backend observer; selection is refused before effects");};
  if (writerProbe) {
    const before = liveWriters();
    note("liveWritersWhileOpen", before);
    expect(before, "the open document holds a live WAL writer on the server").toBeGreaterThan(0);
  }

  const sigtermAt = Date.now();
  await stopLocalHubV1(run,"graceful");
  const gracefulExit = await exitOf(run, fixture.shutdown.gracefulExitWithinMs);
  const exitedAt = Date.now();
  expect(gracefulExit.code, run.output().slice(-8000)).toBe(0);
  await finishLocalHub(run);
    const run2 = await startLocalHub(host, profiles, { port, capture: true,credentialSignIn:true,traceLevel:"info" });
  const exitToRestartSpawnMs = Date.now() - exitedAt;
  note("exitToRestartSpawnMs", exitToRestartSpawnMs);
  expect(exitToRestartSpawnMs, "the restart is spawned within 1 s of the graceful exit").toBeLessThan(fixture.shutdown.restartWithinMs);
  return { note, exitedAt, sigtermAt, writerProbe, liveWriters, run2, exitOf, fenceRelease, reopen };
}

/** 💥️ Kills the restarted hub and checks the writer release, the reopen and the recovered frontier. */
export async function runCrashRecovery({ a, fixture, run, note, exitedAt, sigtermAt, writerProbe, liveWriters, validateTraceLine, traceVocabulary, scenario, waitReady, run2, signIn, ADA, fetchTimed, origin, openSocket, mint, grow, lastGrown, frontierBefore, exitOf, fenceRelease, host, profiles, port, writerFence, reopen }: {
  readonly a: Holder;
  readonly fixture: any;
  readonly run: Awaited<ReturnType<typeof startLocalHub>>;
  readonly note: (key: string, value: unknown) => void;
  readonly exitedAt: number;
  readonly sigtermAt: number;
  readonly writerProbe: boolean;
  readonly liveWriters: () => number;
  readonly validateTraceLine: ReturnType<InstanceType<typeof Ajv>["compile"]>;
  readonly traceVocabulary: { events: string[]; };
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly waitReady: (hubRun: Awaited<ReturnType<typeof startLocalHub>>, label: string) => Promise<Record<string, any>>;
  readonly run2: Awaited<ReturnType<typeof startLocalHub>>;
  readonly signIn: (email: string, password: string, device: string) => Promise<string>;
  readonly ADA: { email: string; password: string; display: string; };
  readonly fetchTimed: (url: string, init?: RequestInit, ms?: number) => Promise<Response>;
  readonly origin: string;
  readonly openSocket: (minted: Minted, resumeToken?: string | null, admitWithinMs?: number) => Promise<Holder>;
  readonly mint: (label: string, token: string) => Promise<Minted>;
  readonly grow: (holder: Holder, from: number, count: number, parent: string) => Promise<string>;
  readonly lastGrown: string;
  readonly frontierBefore: any;
  readonly exitOf: (hubRun: Awaited<ReturnType<typeof startLocalHub>>, ms: number) => Promise<{ code: number | null; signal: string | null }>;
  readonly fenceRelease: string | undefined;
  readonly host: Parameters<Parameters<typeof withBuiltLocalHubHostV1>[2]>[0];
  readonly profiles: readonly LocalProfile[];
  readonly port: number;
  readonly writerFence: any;
  readonly reopen: (label: string, token: string, until: number) => Promise<{ holder: Holder; refused: number; }>;
}) {
  {
    const drained = await Promise.race([a.closed, sleep(2000).then(() => ({ code: -1, reason: "no close frame" }))]);
    expect(drained.code).toBe(fixture.shutdown.closeCode);
    expect(run.output()).toContain(fixture.shutdown.databaseClosedMarker);
    note("sigtermToExitMs", exitedAt - sigtermAt);
    note("socketCloseCode", drained.code);
    if (writerProbe) {
      const after = liveWriters();
      note("liveWritersAfterSigtermExit", after);
      expect(after, "a SIGTERM-stopped hub left no live WAL writer on the server").toBe(0);
    }

    const traced: Record<string, any>[] = (run.output() as string)
      .split("\n")
      .filter((line) => line.startsWith('{"level":'))
      .map((line) => JSON.parse(line) as Record<string, any>);
    for (const record of traced) expect(validateTraceLine(record), `${JSON.stringify(record)} ${JSON.stringify(validateTraceLine.errors)}`).toBe(true);
    const traceCounts: Record<string, number> = {};
    for (const record of traced) traceCounts[`${record.event}:${record.outcome}`] = (traceCounts[`${record.event}:${record.outcome}`] ?? 0) + 1;
    note("traceLines", traced.length);
    note("traceEventOutcomes", traceCounts);
    expect(traced.filter((record) => !traceVocabulary.events.includes(record.event))).toEqual([]);
    const socketOf = (outcome: string, detail: string) => traced.filter((record) => record.event === "server.document.socket" && record.outcome === outcome && record.detail === detail && record.space === scenario.spaceId && record.artifact === scenario.documentId);
    const socketClosed = socketOf("cancelled", "closed");
    const socketPair = socketOf("ok", "upgrade").find((open) => socketClosed.some((close) => close.requestId === open.requestId && close.principal === open.principal));
    expect(socketPair, "one document socket's admission and close share a span id").toBeDefined();
    note("documentSocketSpan", { open: socketPair, close: socketClosed.find((close) => close.requestId === socketPair!.requestId) });
    const seen = (event: string, outcome: string) => traced.some((record) => record.event === event && record.outcome === outcome);
    for (const [event, outcome] of [
      ["server.boot", "ok"],
      ["server.catalog.publication", "ok"],
      ["server.directory.command", "ok"],
      ["server.presence.join", "ok"],
      ["server.presence.expiry", "ok"],
      ["server.presence.leave", "ok"],
      ["server.shutdown", "ok"],
    ])
      expect(seen(event!, outcome!), `${event}:${outcome} in ${JSON.stringify(traceCounts)}`).toBe(true);
    expect(traced.filter((record) => record.event === "server.shutdown").map((record) => record.detail)).toEqual([expect.stringContaining(fixture.shutdown.databaseClosedMarker)]);
    await waitReady(run2, "restart");
    note("sigtermToReadyMs", Date.now() - sigtermAt);
    const tokenA2 = await signIn(ADA.email, ADA.password, "device-ada-2");
    const spaces = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA2}` } })).json();
    expect(spaces.some((r: any) => r?.space?.id === scenario.spaceId)).toBe(true);
    scenario.hubOutput = () => run2.output();
    const reopenAt = Date.now();
    const a3 = await openSocket(await mint("a-restart", tokenA2), null, fixture.shutdown.reopenWithinMs);
    note("reopenedOnFirstAttemptAfterSigterm", true);
    note("readyToReopenedWelcomeMs", Date.now() - reopenAt);
    note("sigtermToReopenedWelcomeFirstAttemptMs", Date.now() - sigtermAt);
    await grow(a3, fixture.growth.editsBeforeRestart, fixture.growth.editsAfterRestart, lastGrown);
    note("growthEditsAcceptedAfterRestart", fixture.growth.editsAfterRestart);
    expect(a3.welcome.server_frontier.document_id ?? a3.welcome.server_frontier.documentId).toBe(scenario.documentId);
    expect((a3.welcome.server_frontier.last_commit_seq ?? 0) >= (frontierBefore.last_commit_seq ?? 0)).toBe(true);

    const crashAt = Date.now();
    await stopLocalHubV1(run2,"crash");
    expect((await exitOf(run2, 5000)).signal).toBe("SIGKILL");
    if (writerProbe && fenceRelease === "after-lease-ttl") {
      const leased = liveWriters();
      note("liveWritersRightAfterSigkill", leased);
      expect(leased, "a crashed hub's lease outlives the process until it expires").toBeGreaterThan(0);
    }
    if (writerProbe && fenceRelease === "immediate") {
      const until = Date.now() + fixture.shutdown.crashReleaseSlackMs;
      while (liveWriters() > 0 && Date.now() < until) await sleep(100);
      expect(liveWriters(), "the server ended a crashed hub's writer session").toBe(0);
      note("sigkillToServerReleaseMs", Date.now() - crashAt);
    }
    await finishLocalHub(run2);
    const run3 = await startLocalHub(host, profiles, { port, capture: true,credentialSignIn:true,traceLevel:"info" });
    {
      scenario.hubOutput = () => run3.output();
      await waitReady(run3, "crash-restart");
      const bootedAfterCrashMs = Date.now() - crashAt;
      const tokenA3 = await signIn(ADA.email, ADA.password, "device-ada-3");
      const releaseBoundMs = fenceRelease === "after-lease-ttl" ? writerFence.neo4j.leaseTtlMs + fixture.shutdown.crashReleaseSlackMs : fixture.shutdown.crashReleaseSlackMs;
      const reopened = await reopen("a-crash", tokenA3, crashAt + releaseBoundMs);
      const crashReopenMs = Date.now() - crashAt;
      note("sigkillToReadyMs", bootedAfterCrashMs);
      note("sigkillToReopenedWelcomeMs", crashReopenMs);
      note("refusedOpensAfterSigkill", reopened.refused);
      if (fenceRelease === "immediate") expect(reopened.refused).toBe(0);
      if (fenceRelease === "after-lease-ttl" && bootedAfterCrashMs < writerFence.neo4j.leaseTtlMs) expect(reopened.refused).toBeGreaterThan(0);
      expect((reopened.holder.welcome.server_frontier.last_commit_seq ?? 0) >= (frontierBefore.last_commit_seq ?? 0)).toBe(true);
      reopened.holder.socket.close();
    }
  }
}
