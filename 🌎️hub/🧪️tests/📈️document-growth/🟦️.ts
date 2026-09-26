// #region Header
/**
 * 📈️ Document growth e2e (language-agnostic fixture `🧫️fixtures/📈️document-growth-v1`).
 * Gated by HUB_E2E=1. One author creates every fixture document through the hub creation catalog (genesis on the
 * real guests), grows all of them at once with chained edits, then the hub is stopped with SIGTERM and restarted on
 * the same data root; every document must reopen and accept its next edits. A document stays writable as it grows:
 * no edit is refused, the ack latency of its last window stays within `latencyGrowthMax` of its first window at the
 * same concurrency (a long document is compared from the edit the shortest documents finished at), and the hub output
 * carries none of the fixture's refusal markers.
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
const TEST_TIMEOUT_MS = 7_200_000;

function findRepoRoot(start: string): string {
  let dir = start;
  for (;;) {
    if (existsSync(join(dir, "AGENTS.md")) && existsSync(join(dir, "nx.json"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) throw new Error(`repo root not found from ${start}`);
    dir = parent;
  }
}

function pick(parent: string, pred: (n: string) => boolean): string {
  const hit = readdirSync(parent).find((name) => name !== "node_modules" && pred(name));
  if (!hit) throw new Error(`no entry under ${parent}`);
  return join(parent, hit);
}

const repoRoot = findRepoRoot(dirname(fileURLToPath(import.meta.url)));
const hubRoot = pick(repoRoot, (n) => n.endsWith("hub") && !n.startsWith("."));
const fixture = JSON.parse(readFileSync(join(pick(pick(hubRoot, (n) => n.includes("fixtures")), (n) => n.includes("document-growth")), "🔣️.json"), "utf8"));
const scenarioSchema = JSON.parse(readFileSync(join(pick(pick(hubRoot, (n) => n.includes("schema")), (n) => n.includes("document-growth")), "🔣️.json"), "utf8"));

type Frame = Record<string, any>;
type Holder = { label: string; actor: string; socket: WebSocket; frames: Frame[]; waiters: Array<(f: Frame) => void>; closed: Promise<{ code: number; reason: string }>; welcome?: any; closeInfo?: string };
type Doc = { label: string; kindId: string; documentId: string; edits: number; last: string; latencies: number[]; schema: string };

const median = (values: number[]) => [...values].sort((a, b) => a - b)[Math.floor(values.length / 2)] ?? 0;

describe("document growth fixture", () => {
  it("loads the language-agnostic scenario fixture", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(scenarioSchema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, bounds: { ...fixture.bounds, concurrentCreations: 9 } }), "more creations in flight than the hub admits").toBe(false);
    expect(fixture.schema).toBe("semio.hub.document-growth-scenario/v1");
    const documents = fixture.documents.reduce((sum: number, row: any) => sum + row.count, 0);
    expect(documents).toBeGreaterThanOrEqual(fixture.bounds.concurrentDocumentsMin);
    expect(Math.max(...fixture.documents.map((row: any) => row.editsBeforeRestart))).toBeGreaterThanOrEqual(fixture.bounds.longDocumentEditsMin);
    expect(new Set(fixture.documents.map((row: any) => row.kindId)).size).toBeGreaterThanOrEqual(2);
    expect(fixture.bounds.latencyWindowEdits * 2).toBeLessThanOrEqual(Math.min(...fixture.documents.map((row: any) => row.editsBeforeRestart)));
    expect(fixture.command.diffSchema).toBe("artifact");
  });
});

describe.skipIf(!HUB_E2E)("document growth e2e", () => {
  it(
    "keeps every concurrently grown document writable across a hub restart",
    async () => {
      const packages = pick(hubRoot, (n) => n.includes("packages"));
      const hubRustRoot = pick(packages, (n) => n.includes("rust"));
      const execution = pick(pick(hubRoot, (n) => n.includes("local-bootstrap")), (n) => n.includes("execution"));
      const fw = pick(repoRoot, (n) => n.includes("framework"));
      const osRoot = join(pick(fw, (n) => n.includes("products")), readdirSync(pick(fw, (n) => n.includes("products"))).find((n) => n.endsWith("os"))!);
      const osMods = pick(osRoot, (n) => n.includes("modules"));
      const schemaDir = pick(pick(osMods, (n) => n.includes("directory")), (n) => n.includes("schema"));
      const creation = pick(schemaDir, (n) => n.includes("space-artifact-creation"));
      const replication = pick(pick(fw, (n) => n.includes("modules") && !n.includes("products")), (n) => n.includes("replication"));

      const { startLocalHub, finishLocalHub, waitForReadiness, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } = await import(join(execution, "🟦️.ts"));
      const { sealDirectoryCommandRequestV1, directoryCommandRequestJson } = await import(join(schemaDir, "🟦️.ts"));
      const { sealSpaceArtifactCreateV1 } = await import(join(creation, "🟦️.ts"));
      const { encodeClientFrame, decodeServerFrame } = await import(join(replication, "🟦️.ts"));
      const { parseDocumentSocketGrantReceiptV1 } = await import(join(osRoot, "🟦️.ts"));

      const bin = process.env.OS_HUB_BINARY!;
      if (!bin || !existsSync(bin)) throw new Error(`os-hub binary missing at ${bin}`);
      const catalogSource = process.env.OS_HUB_TRUSTED_CATALOG_SOURCE!;
      if (!catalogSource || !existsSync(catalogSource)) throw new Error(`trusted catalog missing: ${catalogSource}`);
      const dataParent = process.env.HUB_E2E_DATA_PARENT ?? tmpdir();
      mkdirSync(dataParent, { recursive: true, mode: 0o700 });
      const dataRoot = mkdtempSync(join(realpathSync(dataParent), "document-growth-"));
      cpSync(catalogSource, join(dataRoot, "trusted-catalog"), { recursive: true });

      const author = { email: "growth-author@example.org", password: "correct horse battery staple", display: "Growth Author" };
      const provisioned = spawnSync(bin, ["credential", "set", "--email", author.email, "--display-name", author.display], { env: { ...process.env, OS_HUB_DATA: dataRoot }, input: author.password, encoding: "utf8" });
      if (provisioned.status !== 0) throw new Error(`credential set failed status=${provisioned.status} stderr=${provisioned.stderr}`);

      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      process.env.SEMIO_TRACE_LEVEL = "info";
      delete process.env.SEMIO_TRACE_SINK;
      const port = Number(process.env.HUB_TWO_CLIENT_PORT ?? 7712);
      const profiles = [{ profileId: "a", subject: fixture.author, displayName: author.display, allowedClientClasses: ["native", "mcp"] }];
      const origin = `http://127.0.0.1:${port}`;
      const wsOrigin = `ws://127.0.0.1:${port}`;
      const backend = process.env.OS_HUB_STORAGE_BACKEND ?? "fs";
      const receipt: Record<string, unknown> = { backend };
      const note = (key: string, value: unknown) => {
        receipt[key] = value;
        console.log(`[document-growth] backend=${backend} ${key}=${JSON.stringify(value)}`);
        if (process.env.HUB_E2E_RECEIPT) writeFileSync(process.env.HUB_E2E_RECEIPT, JSON.stringify(receipt, null, 2));
      };
      const fetchTimed = (url: string, init: RequestInit = {}, ms = 60_000) => fetch(url, { ...init, signal: AbortSignal.timeout(ms) });
      const hex = () => [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");

      let run = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
      const waitReady = async (label: string) => {
        try {
          return await waitForReadiness(run, true, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
        } catch (error) {
          throw new Error(`${label}: ${(error as Error).message}\n${run.output().slice(-12000)}`);
        }
      };
      const signIn = async (device: string) => {
        const res = await fetchTimed(`${origin}/auth/sessions`, {
          method: "POST",
          headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
          body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: author.email, password: author.password, deviceInstanceId: device, clientClass: "browser" }),
        });
        expect(res.status).toBe(200);
        return ((await res.json()) as { token: string }).token;
      };
      const waitFrame = (holder: Holder, pred: (f: Frame) => boolean, label: string, ms: number) => {
        const hit = holder.frames.find(pred);
        if (hit) return Promise.resolve(hit);
        return new Promise<Frame>((resolve, reject) => {
          const onFrame = (frame: Frame) => {
            if (!pred(frame)) return;
            clearTimeout(timer);
            holder.waiters = holder.waiters.filter((w) => w !== onFrame);
            resolve(frame);
          };
          holder.waiters.push(onFrame);
          const timer = setTimeout(() => {
            holder.waiters = holder.waiters.filter((w) => w !== onFrame);
            const errors = holder.frames.filter((f) => "Error" in f).map((f) => JSON.stringify(f.Error));
            reject(new Error(`${holder.label} missing ${label} (n=${holder.frames.length}) socket=${holder.closeInfo ?? "open"} errors=${errors.join(";")}\nhub-output:\n${run.output().slice(-8000)}`));
          }, ms);
        });
      };

      try {
        await waitReady("boot");
        const token = await signIn("device-growth");
        const headers = { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" };
        const created = await fetchTimed(`${origin}/directory/commands`, { method: "POST", headers, body: directoryCommandRequestJson(sealDirectoryCommandRequestV1(hex(), { kind: "create-space", name: fixture.spaceName, spaceKind: "studio", visibility: "private" } as any)) });
        expect(created.status).toBe(202);
        const spaceId = JSON.parse(await created.text()).events.find((e: any) => e.body?.kind === "space.created").body.spaceId as string;
        const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
        const catalog = await (await fetchTimed(`${origin}${route}`, { headers })).json();
        note("creationCatalogKinds", catalog.kinds.map((row: any) => row.kindId));

        const creationStartedAt = Date.now();
        const requested = fixture.documents.flatMap((spec: any, row: number) => {
          const kind = catalog.kinds.find((entry: any) => entry.kindId === spec.kindId);
          expect(kind, `catalog kinds=${catalog.kinds.map((entry: any) => entry.kindId).join(",")}`).toBeTruthy();
          return Array.from({ length: spec.count }, (_, index) => ({ label: `${row}-${index}`, kindId: kind.kindId as string, edits: spec.editsBeforeRestart as number, request: sealSpaceArtifactCreateV1({ requestId: hex(), expectedCatalogGenerationId: catalog.catalogGenerationId, kindId: kind.kindId, name: `Growth ${row}-${index}` }) }));
        });
        const docs: Doc[] = [];
        const creationMs: number[] = [];
        const pending = [...requested];
        await Promise.all(
          Array.from({ length: fixture.bounds.concurrentCreations }, async () => {
            for (let entry = pending.shift(); entry; entry = pending.shift()) {
              const posted = await fetchTimed(`${origin}${route}`, { method: "POST", headers, body: JSON.stringify(entry.request) });
              expect(posted.status, `creation ${entry.label}: ${posted.status} ${await posted.clone().text()}`).toBe(202);
              for (;;) {
                const polled = await (await fetchTimed(`${origin}${route}/${entry.request.requestId}`, { headers })).json();
                if (polled.phase === "ready") {
                  docs.push({ label: entry.label, kindId: entry.kindId, documentId: polled.ready.artifactId, edits: entry.edits, last: "", latencies: [], schema: "" });
                  creationMs.push(Date.now() - creationStartedAt);
                  note("documentsCreated", docs.length);
                  break;
                }
                if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) throw new Error(`creation ${polled.phase}: ${JSON.stringify(polled)}\n${run.output().slice(-8000)}`);
                await sleep(500);
              }
            }
          }),
        );
        docs.sort((left, right) => left.label.localeCompare(right.label));
        note("creationMsFirstAndLast", [Math.min(...creationMs), Math.max(...creationMs)]);

        const open = async (doc: Doc, bearer: string): Promise<{ holder: Holder; welcomeMs: number }> => {
          const bearerHeaders = { ...headers, authorization: `Bearer ${bearer}` };
          const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(doc.documentId)}`;
          const planRes = await fetchTimed(`${origin}${scope}/open-plan`, { method: "POST", headers: bearerHeaders, body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId: doc.documentId }, clientInstanceId: `growth-${doc.label}` }) });
          const planText = await planRes.text();
          expect(planRes.status, planText.slice(0, 300)).toBe(200);
          const plan = JSON.parse(planText);
          const grantRes = await fetchTimed(`${origin}${scope}/socket-grants`, { method: "POST", headers: bearerHeaders, body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }) });
          const grantText = await grantRes.text();
          expect(grantRes.status, grantText.slice(0, 300)).toBe(200);
          const granted = parseDocumentSocketGrantReceiptV1(JSON.parse(grantText));
          doc.schema = plan.artifact.schema;
          const socket = new WebSocket(`${wsOrigin}/scopes/${encodeURIComponent(`${spaceId}/${doc.documentId}`)}/document/ws?surface=${encodeURIComponent(plan.surface.surfaceId)}`, ["semio.session.v1", bearer]);
          socket.binaryType = "arraybuffer";
          const closed = new Promise<{ code: number; reason: string }>((resolve) => socket.addEventListener("close", (event) => resolve({ code: event.code, reason: event.reason })));
          const holder: Holder = { label: doc.label, actor: granted.actorId, socket, frames: [], waiters: [], closed };
          void closed.then(({ code, reason }) => {
            holder.closeInfo = `closed ${code} ${reason}`;
          });
          socket.addEventListener("message", (event) => {
            if (!(event.data instanceof ArrayBuffer)) return;
            const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
            if ("Welcome" in frame) holder.welcome = frame.Welcome;
            if ("Commands" in frame || "Presence" in frame) return;
            holder.frames.push(frame);
            if (holder.frames.length > 64) holder.frames.splice(0, holder.frames.length - 32);
            for (const waiter of [...holder.waiters]) waiter(frame);
          });
          for (let i = 0; i < 400 && socket.readyState === WebSocket.CONNECTING; i++) await sleep(50);
          expect(socket.readyState, `${doc.label} socket did not open`).toBe(WebSocket.OPEN);
          const helloAt = Date.now();
          const packSchemaHash = [...(plan.artifact.packSchemaHash.match(/../gu) ?? [])].map((p: string) => Number.parseInt(p, 16));
          socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
          const admitted = await waitFrame(holder, (f) => "Welcome" in f || "Error" in f, "Welcome", fixture.bounds.reopenWithinMs);
          if ("Error" in admitted) throw new Error(`${doc.label} refused: ${JSON.stringify(admitted.Error)}`);
          return { holder, welcomeMs: Date.now() - helloAt };
        };

        const edit = async (doc: Doc, holder: Holder, index: number) => {
          const mutationId = `${fixture.command.mutationIdPrefix}${doc.label}-${index}`;
          const batchId = fixture.command.batchIdBase + index;
          const payload = Array.from(new TextEncoder().encode(`${doc.label}:${index}:${"g".repeat(fixture.command.payloadBytes)}`));
          const sentAt = Date.now();
          holder.socket.send(
            encodeClientFrame(
              {
                Commands: {
                  batch_id: batchId,
                  envelopes: [
                    {
                      mutation_id: mutationId,
                      document_id: doc.documentId,
                      actor: holder.actor,
                      dependencies: doc.last ? [doc.last] : [],
                      observed: null,
                      target: [],
                      diff: { schema: doc.schema, payload },
                      inverse: { schema: doc.schema, payload: [] },
                      timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 },
                    },
                  ],
                },
              },
              "command",
            ),
          );
          const acked = await waitFrame(holder, (f) => "Ack" in f && f.Ack.batch_id === batchId, `Ack ${index}`, fixture.bounds.ackWithinMs);
          expect(JSON.stringify(acked.Ack.stages), `${doc.label} edit ${index}: ${JSON.stringify(acked.Ack)}`).toContain("Accepted");
          doc.latencies.push(Date.now() - sentAt);
          doc.last = mutationId;
        };

        const holders = new Map<string, Holder>();
        for (const doc of docs) holders.set(doc.label, (await open(doc, token)).holder);
        const grownAt = Date.now();
        await Promise.all(
          docs.map(async (doc) => {
            for (let index = 0; index < doc.edits; index++) {
              await edit(doc, holders.get(doc.label)!, index);
              if ((index + 1) % 20 === 0) note(`editsAccepted.${doc.label}`, index + 1);
            }
          }),
        );
        note("growthMs", Date.now() - grownAt);
        note("editsAccepted", docs.reduce((sum, doc) => sum + doc.edits, 0));
        const window = fixture.bounds.latencyWindowEdits;
        const settled = Math.min(...docs.map((doc) => doc.edits));
        const baselineFrom = (doc: Doc) => (doc.edits >= settled + 2 * window ? settled : 0);
        const latency = docs.map((doc) => ({ label: doc.label, kindId: doc.kindId, edits: doc.edits, baselineFrom: baselineFrom(doc), firstMs: median(doc.latencies.slice(baselineFrom(doc), baselineFrom(doc) + window)), lastMs: median(doc.latencies.slice(-window)) }));
        note("ackLatencyMedians", latency);
        for (const row of latency) expect(row.lastMs, `${row.label} (${row.kindId}) ack latency grew with the document: ${JSON.stringify(row)}`).toBeLessThanOrEqual(Math.max(row.firstMs, 20) * fixture.bounds.latencyGrowthMax);

        const exitOf = (ms: number) =>
          new Promise<{ code: number | null; signal: string | null }>((resolve, reject) => {
            if (run.child.exitCode !== null || run.child.signalCode !== null) return resolve({ code: run.child.exitCode, signal: run.child.signalCode });
            const timer = setTimeout(() => reject(new Error(`hub did not exit within ${ms} ms\n${run.output().slice(-8000)}`)), ms);
            run.child.once("exit", (code: number | null, signal: string | null) => {
              clearTimeout(timer);
              resolve({ code, signal });
            });
          });
        const sigtermAt = Date.now();
        run.child.kill("SIGTERM");
        const exited = await exitOf(fixture.bounds.gracefulExitWithinMs);
        const exitedAt = Date.now();
        expect(exited.code, run.output().slice(-8000)).toBe(0);
        const firstOutput = run.output() as string;
        expect(firstOutput).toContain(fixture.shutdown.databaseClosedMarker);
        note("sigtermToExitMs", exitedAt - sigtermAt);
        await finishLocalHub(run);
        process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
        run = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
        note("exitToRestartSpawnMs", Date.now() - exitedAt);
        expect(Date.now() - exitedAt).toBeLessThan(fixture.bounds.restartWithinMs);
        await waitReady("restart");
        note("sigtermToReadyMs", Date.now() - sigtermAt);
        const token2 = await signIn("device-growth-2");
        const welcomes: number[] = [];
        await Promise.all(
          docs.map(async (doc) => {
            const reopened = await open(doc, token2);
            welcomes.push(reopened.welcomeMs);
            for (let index = doc.edits; index < doc.edits + fixture.editsAfterRestart; index++) await edit(doc, reopened.holder, index);
            reopened.holder.socket.close();
          }),
        );
        note("reopenWelcomeMsMax", Math.max(...welcomes));
        note("reopenWelcomeMsMedian", median(welcomes));
        note("editsAcceptedAfterRestart", docs.length * fixture.editsAfterRestart);
        const output = `${firstOutput}\n${run.output()}`;
        const refusals = output.split("\n").filter((line) => fixture.refusalMarkers.some((marker: string) => line.includes(marker)));
        note("refusalLines", refusals.length);
        expect(refusals, refusals.slice(0, 8).join("\n")).toEqual([]);
      } finally {
        try {
          await finishLocalHub(run);
        } catch {
          /* stopped */
        }
        rmSync(dataRoot, { recursive: true, force: true });
      }
    },
    TEST_TIMEOUT_MS,
  );
});
