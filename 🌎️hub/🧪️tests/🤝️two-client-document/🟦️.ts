// #region Header
/**
 * 🤝️ Two-client document collaboration e2e (language-agnostic fixture).
 * Gated by HUB_E2E=1. PR1 socket chain + HC1 catalog clone + note creation. The hubs run with
 * `SEMIO_TRACE_LEVEL=info` on the default stderr sink; after the first hub's graceful exit every trace line must validate
 * against the trace record schema (Ajv) and name the document-socket, directory-command, presence,
 * catalog and shutdown events the run caused.
 */
// #endregion Header

import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 2_400_000;

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth++) {
    if (readdirSync(current).some((n) => n.endsWith("hub") && existsSync(join(current, n, "📦️packages")))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found");
}

function pick(parent: string, pred: (n: string) => boolean): string {
  const name = readdirSync(parent).find((n) => n !== "node_modules" && pred(n));
  if (!name) throw new Error(`missing child in ${parent}`);
  return join(parent, name);
}

const repoRoot = findRepoRoot(dirname(fileURLToPath(import.meta.url)));
const hubRoot = pick(repoRoot, (n) => n.endsWith("hub") && !n.startsWith("."));

describe("two-client document collaboration fixture", () => {
  it("loads the language-agnostic scenario fixture", () => {
    const fixtures = pick(hubRoot, (n) => n.includes("fixtures"));
    const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
    const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));
    const scenarioSchema = JSON.parse(readFileSync(join(pick(pick(hubRoot, (n) => n.includes("schema")), (n) => n.includes("two-client-document")), "🔣️.json"), "utf8"));
    const validate = new Ajv({ allErrors: true, strict: false }).compile(scenarioSchema.$defs.TwoClientDocumentScenarioV1);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, agent: { ...fixture.agent, closeCode: 1000 } }), "a revoked agent's socket closes 4401").toBe(false);
    expect(fixture.schema).toBe("semio.hub.two-client-document-scenario/v1");
    expect(fixture.authors).toEqual(["author-a", "author-b"]);
    expect(fixture.steps.length).toBeGreaterThanOrEqual(8);
    expect(fixture.expectations.bReceivesCommandsEnvelope).toBe(true);
    expect(fixture.expectations.presenceJoinReplayShowsA).toBe(true);
    expect(fixture.expectations.presenceLeaseExpiryStripsA).toBe(true);
    expect(fixture.presence.leaseTtlMs).toBe(15000);
    expect(fixture.command.diffSchema).toBe("db.pathmap.v1");
    expect(fixture.shutdown.closeCode).toBe(1012);
    expect(fixture.shutdown.restartWithinMs).toBe(1000);
    expect(fixture.growth.editsBeforeRestart).toBeGreaterThan(fixture.growth.distinctPaths * 4);
    expect(fixture.growth.payloadBytes).toBeGreaterThanOrEqual(16384);
    expect(fixture.expectations.gracefulShutdownClosesSocketsAndReleasesWriters).toBe(true);
    expect(fixture.expectations.crashReleasesWritersPerBackendContract).toBe(true);
    expect(fixture.expectations.revocationEndsAgentSession).toBe(true);
    expect(fixture.agent.revocationWithinMs).toBeLessThanOrEqual(5000);
  });
});

describe.skipIf(!HUB_E2E)("two-client document collaboration e2e", () => {
  it(
    "proves A commands and presence reach B over a real os-hub binary",
    async () => {
      const packages = pick(hubRoot, (n) => n.includes("packages"));
      const hubRustRoot = pick(packages, (n) => n.includes("rust"));
      const localBootstrap = pick(hubRoot, (n) => n.includes("local-bootstrap"));
      const execution = pick(localBootstrap, (n) => n.includes("execution"));
      const fw = pick(repoRoot, (n) => n.includes("framework"));
      let osRoot = "";
      for (const pn of readdirSync(fw).filter((n) => n.includes("products"))) {
        for (const on of readdirSync(join(fw, pn)).filter((n) => n.endsWith("os"))) {
          const modsName = readdirSync(join(fw, pn, on)).find((n) => n.includes("modules"));
          if (!modsName) continue;
          const kids = readdirSync(join(fw, pn, on, modsName));
          if (kids.some((k) => k.includes("directory")) && kids.some((k) => k.includes("store"))) {
            osRoot = join(fw, pn, on);
            break;
          }
        }
        if (osRoot) break;
      }
      if (!osRoot) throw new Error("os product not found");
      const osMods = pick(osRoot, (n) => n.includes("modules"));
      const directory = pick(osMods, (n) => n.includes("directory"));
      const schemaDir = pick(directory, (n) => n.includes("schema"));
      const creation = pick(schemaDir, (n) => n.includes("space-artifact-creation"));
      const modules = pick(fw, (n) => n.includes("modules") && !n.includes("products"));
      const replication = pick(modules, (n) => n.includes("replication"));
      const trace = pick(modules, (n) => n.endsWith("trace"));
      const traceRecord = pick(trace, (n) => n.includes("record"));
      const validateTraceLine = new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(join(pick(traceRecord, (n) => n.includes("schema")), "🔣️.json"), "utf8")));
      const traceVocabulary = JSON.parse(readFileSync(join(pick(pick(trace, (n) => n.includes("fixtures")), (n) => n.includes("span-vocabulary")), "🔣️.json"), "utf8")) as { events: string[] };

      const { startLocalHub, finishLocalHub, waitForReadiness, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } = await import(join(execution, "🟦️.ts"));
      const { sealDirectoryCommandRequestV1, directoryCommandRequestJson } = await import(join(schemaDir, "🟦️.ts"));
      const { sealSpaceArtifactCreateV1 } = await import(join(creation, "🟦️.ts"));
      const { encodeClientFrame, decodeServerFrame, encodePresencePeer, decodePresencePeer } = await import(join(replication, "🟦️.ts"));
      const { encodePackValue, parseDocumentSocketGrantReceiptV1 } = await import(join(osRoot, "🟦️.ts"));

      const fixtures = pick(hubRoot, (n) => n.includes("fixtures"));
      const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
      const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));
      const dbMods = pick(osMods, (n) => n.endsWith("db"));
      const storageDir = pick(dbMods, (n) => n.includes("storage"));
      const writerDir = pick(storageDir, (n) => n.includes("writer"));
      const writerFence = JSON.parse(readFileSync(join(pick(pick(writerDir, (n) => n.includes("fixtures")), (n) => n.includes("remote-guard")), "🔣️.json"), "utf8"));

      const staged = join(hubRustRoot, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
      const bin = process.env.OS_HUB_BINARY && existsSync(process.env.OS_HUB_BINARY) ? process.env.OS_HUB_BINARY : staged;
      if (!existsSync(bin)) throw new Error(`os-hub binary missing at ${bin}`);

      const catalogSource =
        process.env.OS_HUB_TRUSTED_CATALOG_SOURCE ?? join(repoRoot, ".🧬semio", "🌐hub", "hc1-boot", "trusted-catalog");
      if (!existsSync(catalogSource)) throw new Error(`trusted catalog missing: ${catalogSource}`);

      const dataParent = process.env.HUB_E2E_DATA_PARENT ?? tmpdir();
      mkdirSync(dataParent, { recursive: true, mode: 0o700 });
      const dataRoot = mkdtempSync(join(realpathSync(dataParent), "two-client-"));
      cpSync(catalogSource, join(dataRoot, "trusted-catalog"), { recursive: true });

      const ADA = { email: "ada-tc@example.org", password: "correct horse battery staple", display: "Ada TC" };
      const BO = { email: "bo-tc@example.org", password: "another perfectly fine phrase", display: "Bo TC" };
      for (const account of [ADA, BO]) {
        const result = spawnSync(bin, ["credential", "set", "--email", account.email, "--display-name", account.display], {
          env: { ...process.env, OS_HUB_DATA: dataRoot },
          input: account.password,
          encoding: "utf8",
        });
        if (result.status !== 0) throw new Error(`credential set failed status=${result.status} signal=${result.signal} stderr=${result.stderr} stdout=${result.stdout}`);
      }

      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      process.env.SEMIO_TRACE_LEVEL = "info";
      delete process.env.SEMIO_TRACE_SINK;
      const port = Number(process.env.HUB_TWO_CLIENT_PORT ?? 7711);
      const profiles = [
        { profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native", "mcp"] },
        { profileId: "b", subject: "author-b", displayName: "Author B", allowedClientClasses: ["native", "mcp"] },
      ];
      const run = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
      const origin = `http://127.0.0.1:${port}`;
      const wsOrigin = `ws://127.0.0.1:${port}`;

      const waitReady = async (hubRun: typeof run, label: string) => {
        try {
          return await waitForReadiness(hubRun, true, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
        } catch (error) {
          throw new Error(`${label}: ${(error as Error).message}\n${hubRun.output().slice(-12000)}`);
        }
      };

      type Frame = Record<string, any>;
      type Holder = { label: string; actor: string; socket: WebSocket; frames: Frame[]; waiters: Array<(f: Frame) => void>; closed: Promise<{ code: number; reason: string }>; welcome?: any; resumeToken?: string };

      try {
        await waitReady(run, "boot");
        const signIn = async (email: string, password: string, device: string) => {
          const res = await fetchTimed(`${origin}/auth/sessions`, {
            method: "POST",
            headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
            body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: device, clientClass: "browser" }),
          });
          expect(res.status).toBe(200);
          return ((await res.json()) as { token: string }).token;
        };

        const fetchTimed = async (url: string, init: RequestInit = {}, ms = 30_000) => {
          const res = await fetch(url, { ...init, signal: AbortSignal.timeout(ms) });
          return res;
        };

        const cmd = async (token: string, body: unknown) => {
          const id = [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");
          const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(id, body as any));
          const res = await fetchTimed(`${origin}/directory/commands`, {
            method: "POST",
            headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" },
            body: sealed,
          });
          return { status: res.status, json: JSON.parse(await res.text()) };
        };

        const tokenA = await signIn(ADA.email, ADA.password, "device-ada");
        const tokenB = await signIn(BO.email, BO.password, "device-bo");
        const created = await cmd(tokenA, { kind: "create-space", name: fixture.spaceName, spaceKind: "studio", visibility: "private" });
        expect(created.status).toBe(202);
        let spaceId = created.json?.events?.find((e: any) => e.body?.kind === "space.created")?.body?.spaceId as string;
        if (!spaceId) {
          const listed = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
          spaceId = listed.find((r: any) => r?.space?.name === fixture.spaceName)?.space?.id;
        }
        expect(spaceId).toBeTruthy();
        expect((await cmd(tokenA, { kind: "upsert-member", spaceId, email: BO.email, role: "author" })).status).toBe(202);

        const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
        const catalog = await (await fetchTimed(`${origin}${route}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
        const kind = catalog.kinds.find((row: any) => row.kindId === fixture.artifactKindId);
        expect(kind, `catalog kinds=${catalog.kinds.map((row: any) => row.kindId).join(",")}`).toBeTruthy();
        const request = sealSpaceArtifactCreateV1({
          requestId: [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join(""),
          expectedCatalogGenerationId: catalog.catalogGenerationId,
          kindId: kind.kindId,
          name: "Two Client Note",
        });
        expect((await fetchTimed(`${origin}${route}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${tokenA}` }, body: JSON.stringify(request) })).ok).toBe(true);

        let documentId = "";
        let surfaceId = fixture.surfaceId as string;
        let artifactSchema = "";
        let packSchemaHashHex = "";
        const deadline = Date.now() + 300_000;
        while (Date.now() < deadline) {
          const polled = await (await fetchTimed(`${origin}${route}/${request.requestId}`, { headers: { authorization: `Bearer ${tokenA}` } }, 60_000)).json();
          if (polled.phase === "ready") {
            documentId = polled.ready.artifactId;
            artifactSchema = polled.ready.artifactSchema ?? kind.schema;
            break;
          }
          if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) {
            throw new Error(`creation ${polled.phase}: ${JSON.stringify(polled)}\nhub-output:\n${run.output().slice(-12000)}`);
          }
          await sleep(1000);
        }
        expect(documentId.startsWith("artifact-")).toBe(true);

        let hubOutput = (): string => run.output();
        const waitFrame = async (holder: Holder, pred: (f: Frame) => boolean, label: string, ms = 15000) => {
          const until = Date.now() + ms;
          const hit = holder.frames.find(pred);
          if (hit) return hit;
          return await new Promise<Frame>((resolve, reject) => {
            const onFrame = (frame: Frame) => {
              if (!pred(frame)) return;
              clearInterval(timer);
              holder.waiters = holder.waiters.filter((w) => w !== onFrame);
              resolve(frame);
            };
            holder.waiters.push(onFrame);
            const timer = setInterval(() => {
              if (Date.now() < until) return;
              clearInterval(timer);
              holder.waiters = holder.waiters.filter((w) => w !== onFrame);
              const kinds = [...new Set(holder.frames.map((f) => Object.keys(f)[0]))].join(",");
              const errors = holder.frames.filter((f) => "Error" in f).map((f) => JSON.stringify(f.Error));
              reject(new Error(`${holder.label} missing ${label} within ${ms} ms: ${kinds} (n=${holder.frames.length}) errors=${errors.join(";")}\nhub-output:\n${hubOutput().slice(-8000)}`));
            }, 100);
          });
        };

        type Minted = { label: string; token: string; actor: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; wsUrl: string; protocols: string[] };

        const mint = async (label: string, token: string): Promise<Minted> => {
          const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
          const planRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
            method: "POST",
            headers,
            body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `tc-${label}` }),
          });
          const planText = await planRes.text();
          expect(planRes.status, planText.slice(0, 200)).toBe(200);
          const plan = JSON.parse(planText);
          const mintedSurface = plan.surface?.surfaceId ?? surfaceId;
          const mintedSchema = plan.artifact.schema;
          const mintedHash = plan.artifact.packSchemaHash;
          const grantRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
            method: "POST",
            headers,
            body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
          });
          const grantText = await grantRes.text();
          expect(grantRes.status, grantText.slice(0, 200)).toBe(200);
          const receipt = parseDocumentSocketGrantReceiptV1(JSON.parse(grantText));
          const scope = `${spaceId}/${documentId}`;
          const wsUrl = `${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(mintedSurface)}`;
          return {
            label,
            token,
            actor: receipt.actorId,
            surfaceId: mintedSurface,
            artifactSchema: mintedSchema,
            packSchemaHashHex: mintedHash,
            wsUrl,
            protocols: ["semio.session.v1", token],
          };
        };

        const openSocket = async (minted: Minted, resumeToken: string | null = null, admitWithinMs = 15000): Promise<Holder> => {
          let closeCode: number | null = null;
          let closeReason = "";
          const socket = new WebSocket(minted.wsUrl, [...minted.protocols]);
          socket.binaryType = "arraybuffer";
          const closed = new Promise<{ code: number; reason: string }>((resolve) =>
            socket.addEventListener("close", (event) => {
              closeCode = event.code;
              closeReason = event.reason;
              resolve({ code: event.code, reason: event.reason });
            }),
          );
          const holder: Holder = { label: minted.label, actor: minted.actor, socket, frames: [], waiters: [], closed };
          socket.addEventListener("message", (event) => {
            if (!(event.data instanceof ArrayBuffer)) return;
            try {
              const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
              if ("Welcome" in frame) {
                holder.welcome = frame.Welcome;
                holder.resumeToken = frame.Welcome.resume_token;
              }
              // Cap retained frames — framework document lane can flood heartbeats and starve the event loop.
              holder.frames.push(frame);
              if (holder.frames.length > 64) holder.frames.splice(0, holder.frames.length - 32);
              for (const waiter of [...holder.waiters]) waiter(frame);
            } catch { /* ignore */ }
          });
          for (let i = 0; i < 200 && socket.readyState === WebSocket.CONNECTING; i++) await sleep(50);
          expect(
            socket.readyState,
            `${minted.label} socket closed before open code=${closeCode} reason=${closeReason} protocols=${JSON.stringify(minted.protocols)} url=${minted.wsUrl}`,
          ).toBe(WebSocket.OPEN);
          const packSchemaHash = [...(minted.packSchemaHashHex.match(/../gu) ?? [])].map((p) => Number.parseInt(p, 16));
          socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: minted.artifactSchema, pack_schema_hash: packSchemaHash, resume_token: resumeToken, frontier: null } }, "command"));
          const admitted = await waitFrame(holder, (f) => "Welcome" in f || "Error" in f, "Welcome", admitWithinMs);
          if ("Error" in admitted) {
            socket.close();
            throw new Error(`${minted.label} refused: ${JSON.stringify(admitted.Error)}`);
          }
          expect(holder.welcome.server_frontier.document_id ?? holder.welcome.server_frontier.documentId).toBe(documentId);
          return holder;
        };

        const mintedA = await mint("a", tokenA);
        const mintedB = await mint("b", tokenB);
        artifactSchema = mintedA.artifactSchema;
        packSchemaHashHex = mintedA.packSchemaHashHex;
        surfaceId = mintedA.surfaceId;
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
          document_id: documentId,
          actor: a.actor,
          dependencies: [],
          diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.diffValue)) },
          inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
          timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
        };
        a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId, envelopes: [envelope] } }, "command"));
        const ack = await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId, "Ack");
        expect(JSON.stringify(ack.Ack.stages), JSON.stringify(ack.Ack.stages)).toContain("Accepted");
        expect(ack.Ack.frontier.document_id ?? ack.Ack.frontier.documentId).toBe(documentId);

        const remote = await waitFrame(b, (f) => "Commands" in f && f.Commands.envelopes?.[0]?.mutation_id === mutationId, "Commands");
        expect(remote.Commands.envelopes[0].mutation_id).toBe(mutationId);
        expect(remote.Commands.envelopes[0].document_id).toBe(documentId);
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

        const humanHeaders = { "content-type": "application/json", authorization: `Bearer ${tokenA}`, origin: "http://127.0.0.1:6066" };
        const delegated = await fetchTimed(`${origin}/auth/agent-delegations`, {
          method: "POST",
          headers: humanHeaders,
          body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: fixture.agent.agentLabel, audience: fixture.agent.audience, ttlSecs: fixture.agent.ttlSecs }),
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
                    document_id: documentId,
                    actor: agent.actor,
                    dependencies: [],
                    diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.agent.diffValue)) },
                    inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
                    timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
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
        const agentTargetIntent = JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: "tc-agent-target" });
        for (const part of ["manifest", "descriptor"]) {
          const askedAt = Date.now();
          const answered = await fetchTimed(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/execution-target/${part}`, { method: "POST", headers: agentTargetHeaders, body: agentTargetIntent });
          const answeredText = part === "manifest" ? await answered.text() : `${(await answered.arrayBuffer()).byteLength} bytes`;
          const answeredMs = Date.now() - askedAt;
          expect(answered.status, `the agent's execution-target ${part} after its own commit: ${answeredText.slice(0, 200)}`).toBe(200);
          expect(answeredMs, `the agent's execution-target ${part} answered within the bound`).toBeLessThanOrEqual(fixture.agent.executionTargetWithinMs);
          agentExecutionTargetMs.push(answeredMs);
        }
        const afterRevocation = since(b);
        const revokedAt = Date.now();
        const revoked = await fetchTimed(`${origin}/auth/agent-delegations/${encodeURIComponent(delegation.delegationId)}`, { method: "DELETE", headers: humanHeaders });
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
                        document_id: documentId,
                        actor: holder.actor,
                        dependencies: [previous],
                        diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(value)) },
                        inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({})) },
                        timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
                      },
                    ],
                  },
                },
                "command",
              ),
            );
            const grown = await waitFrame(holder, (f) => "Ack" in f && f.Ack.batch_id === batchId, `growth Ack ${index}`, 60_000).catch(async (error: Error) => {
              const closed = await Promise.race([holder.closed, sleep(10).then(() => undefined)]);
              throw new Error(`${error.message} socket=${closed ? `closed ${closed.code} ${closed.reason}` : "open"}\nhub-output:\n${hubOutput().slice(-12000)}`);
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

        const backend = process.env.OS_HUB_STORAGE_BACKEND ?? "fs";
        const receipt: Record<string, unknown> = { backend };
        const note = (key: string, value: unknown) => {
          receipt[key] = value;
          console.log(`[two-client] backend=${backend} ${key}=${JSON.stringify(value)}`);
          if (process.env.HUB_E2E_RECEIPT) writeFileSync(process.env.HUB_E2E_RECEIPT, JSON.stringify(receipt, null, 2));
        };
        note("growthEditsAcceptedBeforeRestart", fixture.growth.editsBeforeRestart);
        note("growthMsBeforeRestart", growthMsBeforeRestart);
        note("agentRevocationMs", agentRevocationMs);
        note("agentExecutionTargetMs", agentExecutionTargetMs);
        const fenceRelease = writerFence.backends.find((row: any) => row.backend === backend)?.crashRelease as string | undefined;
        const exitOf = (hubRun: typeof run, ms: number) =>
          new Promise<{ code: number | null; signal: string | null }>((resolve, reject) => {
            if (hubRun.child.exitCode !== null || hubRun.child.signalCode !== null) return resolve({ code: hubRun.child.exitCode, signal: hubRun.child.signalCode });
            const timer = setTimeout(() => reject(new Error(`hub did not exit within ${ms} ms\n${hubRun.output().slice(-8000)}`)), ms);
            hubRun.child.once("exit", (code: number | null, signal: string | null) => {
              clearTimeout(timer);
              resolve({ code, signal });
            });
          });
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

        const writerProbe = process.env.HUB_E2E_WRITER_PROBE ? (JSON.parse(process.env.HUB_E2E_WRITER_PROBE) as string[]) : undefined;
        const liveWriters = (): number => {
          const [command, ...args] = writerProbe!;
          const probed = spawnSync(command!, [...args, fixture.shutdown.liveWriterProbe[backend]], { encoding: "utf8" });
          if (probed.status !== 0) throw new Error(`live writer probe failed: ${probed.stderr}`);
          const count = probed.stdout.trim().split("\n").at(-1)!.trim();
          if (!/^\d+$/u.test(count)) throw new Error(`live writer probe answered ${JSON.stringify(probed.stdout)}`);
          return Number(count);
        };
        if (writerProbe) {
          const before = liveWriters();
          note("liveWritersWhileOpen", before);
          expect(before, "the open document holds a live WAL writer on the server").toBeGreaterThan(0);
        }

        const sigtermAt = Date.now();
        run.child.kill("SIGTERM");
        const gracefulExit = await exitOf(run, fixture.shutdown.gracefulExitWithinMs);
        const exitedAt = Date.now();
        expect(gracefulExit.code, run.output().slice(-8000)).toBe(0);
        await finishLocalHub(run);
        process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
        const run2 = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
        const exitToRestartSpawnMs = Date.now() - exitedAt;
        note("exitToRestartSpawnMs", exitToRestartSpawnMs);
        expect(exitToRestartSpawnMs, "the restart is spawned within 1 s of the graceful exit").toBeLessThan(fixture.shutdown.restartWithinMs);
        try {
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
          const socketOf = (outcome: string, detail: string) => traced.filter((record) => record.event === "server.document.socket" && record.outcome === outcome && record.detail === detail && record.space === spaceId && record.artifact === documentId);
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
          const tokenA2 = await signIn(ADA.email, ADA.password, "device-ada-2");
          const spaces = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA2}` } })).json();
          expect(spaces.some((r: any) => r?.space?.id === spaceId)).toBe(true);
          hubOutput = () => run2.output();
          const reopenAt = Date.now();
          const a3 = await openSocket(await mint("a-restart", tokenA2), null, fixture.shutdown.reopenWithinMs);
          note("reopenedOnFirstAttemptAfterSigterm", true);
          note("readyToReopenedWelcomeMs", Date.now() - reopenAt);
          await grow(a3, fixture.growth.editsBeforeRestart, fixture.growth.editsAfterRestart, lastGrown);
          note("growthEditsAcceptedAfterRestart", fixture.growth.editsAfterRestart);
          const gracefulReopenMs = Date.now() - sigtermAt;
          note("sigtermToReopenedWelcomeFirstAttemptMs", gracefulReopenMs);
          expect(a3.welcome.server_frontier.document_id ?? a3.welcome.server_frontier.documentId).toBe(documentId);
          expect((a3.welcome.server_frontier.last_commit_seq ?? 0) >= (frontierBefore.last_commit_seq ?? 0)).toBe(true);

          const crashAt = Date.now();
          run2.child.kill("SIGKILL");
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
          const run3 = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
          try {
            hubOutput = () => run3.output();
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
          } finally {
            await finishLocalHub(run3);
          }
        } finally {
          await finishLocalHub(run2);
        }
      } finally {
        try { await finishLocalHub(run); } catch { /* done */ }
        rmSync(dataRoot, { recursive: true, force: true });
      }
    },
    TEST_TIMEOUT_MS,
  );
});
