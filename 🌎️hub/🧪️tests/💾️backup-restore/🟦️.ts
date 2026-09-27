/** 💾️ The hub backup/restore drill, zero-touch: a fresh data root carrying a copy of a published trusted catalog, one
 * credential user, the real `os-hub` binary, and per round:
 *
 *   seed (a private space, one document of the chosen kind, N chained edits over the document socket, the reopened
 *   frontier, the active checkpoint pair digest, the descriptor digest, the directory listing) → SIGTERM (timed) →
 *   backup (`tar`, the platform's own archiver — bsdtar on macOS and Windows, GNU tar on Linux) → restore into a
 *   DIFFERENT root (relocation) → every restored file byte-identical to its original (sha256 per file) → the hub boots
 *   on the restored root (timed) → the same frontier, checkpoint pair, descriptor and listing, and the next chained edit
 *   is accepted.
 *
 * `README.md` § "Backup and restore" is the procedure this drill executes; the backup unit is the whole data root with the
 * hub stopped. The same module holds the graceful-shutdown drill (`runShutdownDrill`): the stop that procedure relies on,
 * taken while the hub is busy — its catalog verifying, a socket open with an edit in flight, a creation interpreting. Promoted from the session-12 ticket harness `wp-h10/h10-backup-drill.ts` + `h10-hub.sh` (ticket 26/09/23,
 * measured there SIGTERM 277 ms → tar 164 MB / 25 s → ready 13.5 s, byte-identical, 5/5).
 */
import { createHash, randomBytes } from "node:crypto";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenDocument, hubProbeSignIn, hubSeedTrustedCatalog } from "../../🤝️integration-harness/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { finishLocalHub, startLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS, waitForChildExit, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";

/** 🎛️ One drill. */
export type BackupRestoreDrillOptions = Readonly<{
  repoRoot: string;
  binaryPath: string;
  catalogRoot: string;
  kind: string;
  edits: number;
  rounds: number;
  keepRoots: boolean;
  signal: AbortSignal;
  onProgress: (line: string) => void;
}>;

/** 📊️ One round's measurements. */
export type BackupRestoreRound = {
  round: number;
  pass: boolean;
  kindId?: string;
  documentId?: string;
  seedMs?: number;
  sigtermToExitMs?: number;
  archiveBytes?: number;
  archiveMs?: number;
  restoreMs?: number;
  files?: number;
  byteIdentical?: boolean;
  restoredReadyMs?: number;
  checks?: Record<string, boolean>;
  undecodableFrames?: number;
  error?: string;
  roots?: string;
};

/** 🧑‍💻️ The launcher profile the drill hub is bootstrapped for (the drill itself signs in with a credential). */
const DRILL_PROFILE = Object.freeze({ profileId: "developer", subject: "local-backup-drill-01", displayName: "Backup Drill", allowedClientClasses: Object.freeze(["native", "mcp", "react-relay"] as const) });
const EMAIL = "backup-drill@semio.dev";

function fileDigests(root: string): Map<string, string> {
  const digests = new Map<string, string>();
  const walk = (dir: string): void => {
    for (const name of readdirSync(dir)) {
      const path = join(dir, name);
      const stat = statSync(path);
      if (stat.isDirectory()) walk(path);
      else if (stat.isFile()) digests.set(relative(root, path), createHash("sha256").update(readFileSync(path)).digest("hex"));
    }
  };
  walk(root);
  return digests;
}

function tar(args: readonly string[]): void {
  const answer = spawnSync("tar", [...args], { encoding: "utf8" });
  if (answer.status !== 0) throw new Error(`tar ${args.join(" ")} exited ${answer.status}: ${answer.stderr}`);
}

async function snapshot(origin: string, token: string, spaceId: string, documentId: string) {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const descriptor = await hubProbeCall(origin, "GET", scope, token);
  const pair = await hubProbeCall(origin, "GET", `${scope}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  const spaces = await hubProbeCall(origin, "GET", "/directory/spaces", token);
  return {
    descriptorStatus: descriptor.status,
    descriptorSha256: createHash("sha256").update(descriptor.bytes).digest("hex"),
    pairStatus: pair.status,
    pairSha256: createHash("sha256").update(pair.bytes).digest("hex"),
    spaceListed: JSON.stringify(spaces.json ?? []).includes(spaceId),
  };
}

const frontierOf = (welcome: any): string => JSON.stringify({ lastCommitSeq: welcome.server_frontier.last_commit_seq, headEditId: welcome.server_frontier.head_edit_id, headEditOrdinal: welcome.server_frontier.head_edit_ordinal, chainHash: Buffer.from(welcome.server_frontier.chain_hash).toString("hex") });

/** 🌎️ One booted drill hub: its origin, a SIGTERM stop that answers how long the process took to exit, how it exited
 * (`null` while it runs) and everything it printed. */
type DrillHub = Readonly<{ baseUrl: string; stop: () => Promise<number>; exit: () => { code: number | null; signal: NodeJS.Signals | null } | null; output: () => string }>;

/** 🚀️ Boots the hub on `dataDir` the way every local launcher does — the authenticated local-bootstrap pipe on fd 3
 * (`startLocalHub`), credential sign-in on — and answers once its own `/readyz` admits it, bounded by the hub's own
 * catalog no-progress bound. `stop` sends SIGTERM (the backup procedure's stop), waits for the exit, then releases the
 * launcher's run root. */
async function bootHub(options: Pick<BackupRestoreDrillOptions, "repoRoot" | "binaryPath" | "signal">, dataDir: string): Promise<{ hub: DrillHub; readyMs: number }> {
  const started = Date.now();
  process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
  const run = await startLocalHub(options.repoRoot, join(options.repoRoot, "🌎️hub", "📦️packages", "🦀️rust"), [DRILL_PROFILE], { dataDir, binaryPath: options.binaryPath, capture: true });
  const stop = async (): Promise<number> => {
    const stopping = Date.now();
    if (run.child.exitCode === null) {
      run.child.kill("SIGTERM");
      await waitForChildExit(run.child, 60_000).catch(() => undefined);
    }
    const exitMs = Date.now() - stopping;
    await finishLocalHub(run);
    return exitMs;
  };
  const abort = (): void => void run.child.kill("SIGTERM");
  options.signal.addEventListener("abort", abort, { once: true });
  try {
    await waitForReadiness(run, false, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
  } catch (error) {
    const tail = run.output().slice(-1_200);
    await stop();
    throw new Error(`hub on ${dataDir} never became ready: ${String(error instanceof Error ? error.message : error)}\n${tail}`);
  } finally {
    options.signal.removeEventListener("abort", abort);
  }
  const exit = () => (run.child.exitCode === null && run.child.signalCode === null ? null : { code: run.child.exitCode, signal: run.child.signalCode });
  return { hub: { baseUrl: `http://127.0.0.1:${run.port}`, stop, exit, output: run.output }, readyMs: Date.now() - started };
}

/** 💾️ Runs every round; each round owns two fresh roots (removed afterwards unless `keepRoots` or the round failed). */
export async function runBackupRestoreDrill(options: BackupRestoreDrillOptions): Promise<BackupRestoreRound[]> {
  const rounds: BackupRestoreRound[] = [];
  const password = randomBytes(18).toString("hex");
  for (let round = 1; round <= options.rounds && !options.signal.aborted; round += 1) {
    const result: BackupRestoreRound = { round, pass: false };
    const scratch = mkdtempSync(join(tmpdir(), "semio-backup-drill-"));
    const original = join(scratch, "original");
    const restored = join(scratch, "restored");
    const archive = join(scratch, "backup.tar");
    let hub: DrillHub | undefined;
    try {
      mkdirSync(original, { recursive: true, mode: 0o700 });
      const generation = hubSeedTrustedCatalog(options.catalogRoot, original);
      const provisioned = spawnSync(options.binaryPath, ["credential", "set", "--email", EMAIL, "--display-name", "Backup Drill"], { env: { ...process.env, OS_HUB_DATA: original }, input: password, encoding: "utf8" });
      if (provisioned.status !== 0) throw new Error(`credential set failed: ${provisioned.stderr}`);
      options.onProgress(`round ${round}: booting on ${original} (catalog ${generation.slice(0, 12)})`);
      const booted = await bootHub(options, original);
      hub = booted.hub;
      options.onProgress(`round ${round}: ready in ${booted.readyMs} ms; seeding ${options.edits} edits`);
      const seedStarted = Date.now();
      const token = await hubProbeSignIn(hub.baseUrl, EMAIL, password, "backupdrill");
      const spaceId = await hubProbeCreateSpace(hub.baseUrl, token, `Backup drill ${round} ${new Date().toISOString()}`);
      const catalog = await hubProbeCreationCatalog(hub.baseUrl, token, spaceId);
      const kind = catalog.kinds.find((entry) => entry.kindId === options.kind || entry.schema.startsWith(options.kind));
      if (!kind) throw new Error(`the catalog offers no kind ${options.kind}; it offers ${catalog.kinds.map((entry) => entry.kindId).join(", ")}`);
      result.kindId = kind.kindId;
      const { artifactId } = await hubProbeCreateArtifact(hub.baseUrl, token, spaceId, catalog.generationId, kind.kindId, `Backup drill ${kind.kindId}`);
      result.documentId = artifactId;
      const session = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "backup-drill");
      let last = "";
      for (let index = 0; index < options.edits; index += 1) last = await session.edit(index, last);
      result.undecodableFrames = session.undecodableFrames();
      session.close();
      const reopened = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "backup-drill");
      const frontier = frontierOf(reopened.welcome);
      reopened.close();
      const before = await snapshot(hub.baseUrl, token, spaceId, artifactId);
      result.seedMs = Date.now() - seedStarted;
      result.sigtermToExitMs = await hub.stop();
      hub = undefined;
      options.onProgress(`round ${round}: SIGTERM → exit ${result.sigtermToExitMs} ms; archiving`);
      const archiveStarted = Date.now();
      tar(["-cf", archive, "-C", original, "."]);
      result.archiveMs = Date.now() - archiveStarted;
      result.archiveBytes = statSync(archive).size;
      const restoreStarted = Date.now();
      mkdirSync(restored, { recursive: true, mode: 0o700 });
      tar(["-xf", archive, "-C", restored]);
      result.restoreMs = Date.now() - restoreStarted;
      const originalDigests = fileDigests(original);
      const restoredDigests = fileDigests(restored);
      result.files = originalDigests.size;
      result.byteIdentical = originalDigests.size === restoredDigests.size && [...originalDigests].every(([path, digest]) => restoredDigests.get(path) === digest);
      options.onProgress(`round ${round}: archive ${result.archiveBytes} B in ${result.archiveMs} ms, restored ${result.files} files byte-identical=${result.byteIdentical}; booting the restored root`);
      const rebooted = await bootHub(options, restored);
      hub = rebooted.hub;
      result.restoredReadyMs = rebooted.readyMs;
      const tokenAfter = await hubProbeSignIn(hub.baseUrl, EMAIL, password, "backupdrill");
      const session2 = await hubProbeOpenDocument(hub.baseUrl, tokenAfter, spaceId, artifactId, "backup-drill");
      const after = await snapshot(hub.baseUrl, tokenAfter, spaceId, artifactId);
      const checks: Record<string, boolean> = {
        byteIdentical: result.byteIdentical,
        frontier: frontierOf(session2.welcome) === frontier,
        descriptor: after.descriptorStatus === 200 && after.descriptorSha256 === before.descriptorSha256,
        checkpointPair: after.pairStatus === 200 && after.pairSha256 === before.pairSha256,
        spaceListed: after.spaceListed,
        nextEditAccepted: false,
      };
      await session2.edit(options.edits, last);
      checks.nextEditAccepted = true;
      session2.close();
      result.checks = checks;
      result.pass = Object.values(checks).every(Boolean);
    } catch (error) {
      result.error = String(error instanceof Error ? error.message : error).slice(0, 600);
    } finally {
      if (hub) await hub.stop().catch(() => undefined);
      if (result.pass && !options.keepRoots) rmSync(scratch, { recursive: true, force: true });
      else result.roots = scratch;
    }
    rounds.push(result);
    options.onProgress(`round ${round}: ${result.pass ? "PASS" : "FAIL"} ${JSON.stringify(result.checks ?? {})}${result.error ? ` ${result.error}` : ""}`);
  }
  return rounds;
}

/** 🛑️ One graceful-shutdown drill: the drill hub's binary and catalog as the backup drill's, the kind whose document takes
 * the acknowledged edits, the heavy kinds one of which is created while the hub is stopped (the first the catalog offers). */
export type ShutdownDrillOptions = Readonly<{
  repoRoot: string;
  binaryPath: string;
  catalogRoot: string;
  kind: string;
  heavyKinds: readonly string[];
  edits: number;
  rounds: number;
  keepRoots: boolean;
  signal: AbortSignal;
  onProgress: (line: string) => void;
}>;

/** 📊️ One shutdown round's measurements. */
export type ShutdownDrillRound = {
  round: number;
  pass: boolean;
  kindId?: string;
  inFlightKindId?: string;
  documentId?: string;
  bootReadyMs?: number;
  packagesVerifyingAtSigterm?: number;
  observabilityStatus?: number;
  sigtermToExitMs?: number;
  exitCode?: number | null;
  exitSignal?: string | null;
  socketClose?: { code: number; clean: boolean } | null;
  restartReadyMs?: number;
  headEditOrdinal?: number;
  inFlightCreationStatus?: number;
  inFlightCreationPhase?: string;
  checks?: Record<string, boolean>;
  error?: string;
  roots?: string;
};

/** ⏱️ How long a stopping hub may take to exit: the database's own shutdown deadline (`DATABASE_SHUTDOWN_DEADLINE`, 10 s),
 * past which the hub exits 1 anyway. README § "Backup and restore" asks operators for a 30 s stop timeout on top. */
export const SHUTDOWN_EXIT_BOUND_MS = 10_000;

const SHUTDOWN_EMAIL = "shutdown-drill@semio.dev";

/** 🛑️ Runs every round of the graceful-shutdown drill on a fresh root seeded with a copy of a published catalog: cold boot
 * (the catalog's background verification starts interpreting guests as soon as the hub serves) → one document with
 * `edits` acknowledged edits, its socket left open with one more edit in flight → a creation of a heavy kind in flight →
 * SIGTERM → the process exits 0 within [`SHUTDOWN_EXIT_BOUND_MS`] and records its `server.shutdown` with the database
 * closed, the open socket is ended with a close frame → the hub boots on the same root, every acknowledged edit is there,
 * the next edit is accepted and the interrupted creation answers a typed status. */
export async function runShutdownDrill(options: ShutdownDrillOptions): Promise<ShutdownDrillRound[]> {
  const rounds: ShutdownDrillRound[] = [];
  const password = randomBytes(18).toString("hex");
  const admins = process.env.OS_HUB_ADMIN_SUBJECTS;
  process.env.OS_HUB_ADMIN_SUBJECTS = `credential.password.v1:${SHUTDOWN_EMAIL}`;
  try {
    for (let round = 1; round <= options.rounds && !options.signal.aborted; round += 1) {
      const result: ShutdownDrillRound = { round, pass: false };
      const scratch = mkdtempSync(join(tmpdir(), "semio-shutdown-drill-"));
      const root = join(scratch, "root");
      let hub: DrillHub | undefined;
      try {
        mkdirSync(root, { recursive: true, mode: 0o700 });
        const generation = hubSeedTrustedCatalog(options.catalogRoot, root);
        const provisioned = spawnSync(options.binaryPath, ["credential", "set", "--email", SHUTDOWN_EMAIL, "--display-name", "Shutdown Drill"], { env: { ...process.env, OS_HUB_DATA: root }, input: password, encoding: "utf8" });
        if (provisioned.status !== 0) throw new Error(`credential set failed: ${provisioned.stderr}`);
        options.onProgress(`round ${round}: cold boot on ${root} (catalog ${generation.slice(0, 12)})`);
        const booted = await bootHub(options, root);
        hub = booted.hub;
        result.bootReadyMs = booted.readyMs;
        const token = await hubProbeSignIn(hub.baseUrl, SHUTDOWN_EMAIL, password, "shutdowndrill");
        const spaceId = await hubProbeCreateSpace(hub.baseUrl, token, `Shutdown drill ${round} ${new Date().toISOString()}`);
        const catalog = await hubProbeCreationCatalog(hub.baseUrl, token, spaceId);
        const kind = catalog.kinds.find((entry) => entry.kindId === options.kind || entry.schema.startsWith(options.kind));
        if (!kind) throw new Error(`the catalog offers no kind ${options.kind}; it offers ${catalog.kinds.map((entry) => entry.kindId).join(", ")}`);
        const heavy = options.heavyKinds.map((kindId) => catalog.kinds.find((entry) => entry.kindId === kindId)).find(Boolean) ?? kind;
        result.kindId = kind.kindId;
        result.inFlightKindId = heavy.kindId;
        const { artifactId } = await hubProbeCreateArtifact(hub.baseUrl, token, spaceId, catalog.generationId, kind.kindId, `Shutdown drill ${kind.kindId}`);
        result.documentId = artifactId;
        const session = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "shutdown-drill");
        let last = "";
        for (let index = 0; index < options.edits; index += 1) last = await session.edit(index, last);
        const inFlightEdit = session.submit(options.edits, last).catch((error: unknown) => String(error));
        const requestId = randomBytes(16).toString("hex");
        const creation = await hubProbeCall(hub.baseUrl, "POST", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.generationId, kindId: heavy.kindId, name: `Shutdown drill in flight ${heavy.kindId}` })));
        if (creation.status !== 202) throw new Error(`in-flight creation ${creation.status} ${creation.text.slice(0, 300)}`);
        await new Promise((resolveDelay) => setTimeout(resolveDelay, 1_000));
        const observability = await hubProbeCall(hub.baseUrl, "GET", "/admin/api/observability", token);
        result.observabilityStatus = observability.status;
        if (observability.status === 200) result.packagesVerifyingAtSigterm = (observability.json?.catalog?.packages ?? []).filter((entry: { phase: string }) => entry.phase === "verifying").length;
        options.onProgress(`round ${round}: ${options.edits} edits acknowledged, one in flight, ${heavy.kindId} creation in flight, ${result.packagesVerifyingAtSigterm ?? `? (observability ${result.observabilityStatus})`} package(s) verifying; SIGTERM`);
        const stopping = hub;
        result.sigtermToExitMs = await stopping.stop();
        hub = undefined;
        const exited = stopping.exit();
        result.exitCode = exited?.code ?? null;
        result.exitSignal = exited?.signal ?? null;
        result.socketClose = await session.ended(5_000);
        await inFlightEdit;
        const shutdownLine = stopping.output().split("\n").find((line) => line.includes("server.shutdown") && line.includes("database=closed"));
        options.onProgress(`round ${round}: SIGTERM → exit ${result.sigtermToExitMs} ms code ${result.exitCode} signal ${result.exitSignal}; socket ${JSON.stringify(result.socketClose)}; restarting on the same root`);
        const rebooted = await bootHub(options, root);
        hub = rebooted.hub;
        result.restartReadyMs = rebooted.readyMs;
        const tokenAfter = await hubProbeSignIn(hub.baseUrl, SHUTDOWN_EMAIL, password, "shutdowndrill");
        const reopened = await hubProbeOpenDocument(hub.baseUrl, tokenAfter, spaceId, artifactId, "shutdown-drill");
        result.headEditOrdinal = Number(reopened.welcome.server_frontier.head_edit_ordinal);
        const checks: Record<string, boolean> = {
          exitedZero: result.exitCode === 0 && result.exitSignal === null,
          exitedInBound: result.sigtermToExitMs <= SHUTDOWN_EXIT_BOUND_MS,
          shutdownRecorded: shutdownLine !== undefined,
          socketClosedWithFrame: result.socketClose !== null && result.socketClose.code !== 1006,
          acknowledgedEditsDurable: result.headEditOrdinal >= options.edits,
          nextEditAccepted: false,
          interruptedCreationTyped: false,
        };
        await reopened.edit(options.edits + 1, last);
        checks.nextEditAccepted = true;
        reopened.close();
        const status = await hubProbeCall(hub.baseUrl, "GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations/${requestId}`, tokenAfter);
        result.inFlightCreationStatus = status.status;
        result.inFlightCreationPhase = String(status.json?.phase ?? status.text.slice(0, 80));
        checks.interruptedCreationTyped = status.status < 500 || status.status === 503;
        result.checks = checks;
        result.pass = Object.values(checks).every(Boolean);
      } catch (error) {
        result.error = String(error instanceof Error ? error.message : error).slice(0, 600);
      } finally {
        if (hub) await hub.stop().catch(() => undefined);
        if (result.pass && !options.keepRoots) rmSync(scratch, { recursive: true, force: true });
        else result.roots = scratch;
      }
      rounds.push(result);
      options.onProgress(`round ${round}: ${result.pass ? "PASS" : "FAIL"} ${JSON.stringify(result.checks ?? {})}${result.error ? ` ${result.error}` : ""}`);
    }
  } finally {
    if (admins === undefined) delete process.env.OS_HUB_ADMIN_SUBJECTS;
    else process.env.OS_HUB_ADMIN_SUBJECTS = admins;
  }
  return rounds;
}
