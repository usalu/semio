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
 * hub stopped. Promoted from the session-12 ticket harness `wp-h10/h10-backup-drill.ts` + `h10-hub.sh` (ticket 26/09/23,
 * measured there SIGTERM 277 ms → tar 164 MB / 25 s → ready 13.5 s, byte-identical, 5/5).
 */
import { createHash, randomBytes } from "node:crypto";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenDocument, hubProbeSignIn, hubSeedTrustedCatalog, startHub, type HubHandle } from "../../🤝️integration-harness/🟦️.ts";

/** 🎛️ One drill. */
export type BackupRestoreDrillOptions = Readonly<{
  repoRoot: string;
  binaryPath: string;
  catalogRoot: string;
  kind: string;
  edits: number;
  rounds: number;
  readyTimeoutMs: number;
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
  error?: string;
  roots?: string;
};

const ADMIN_TOKEN = "backup-drill-admin";
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

/** 🚀️ Boots the hub on `dataDir` and answers once its own `/readyz` reports `ready` (the harness's bind wait accepts any
 * HTTP answer, which a hub still loading its catalog already gives). */
async function bootHub(options: BackupRestoreDrillOptions, dataDir: string): Promise<{ hub: HubHandle; readyMs: number }> {
  const started = Date.now();
  const hub = await startHub({ repoRoot: options.repoRoot, dataDir, adminToken: ADMIN_TOKEN, binaryPath: options.binaryPath, readyTimeoutMs: options.readyTimeoutMs, env: { OS_HUB_MODE: "development", OS_HUB_BIND: "127.0.0.1", OS_HUB_CREDENTIAL_SIGN_IN: "1" } });
  for (let reported = 0; ; ) {
    const readiness = await hubProbeCall(hub.baseUrl, "GET", "/readyz").catch(() => null);
    if (readiness?.status === 200 && readiness.json?.status === "ready") return { hub, readyMs: Date.now() - started };
    if (options.signal.aborted || Date.now() - started > options.readyTimeoutMs) {
      await hub.stop();
      throw new Error(`hub on ${dataDir} not ready within ${options.readyTimeoutMs} ms (last /readyz ${readiness?.status ?? "no answer"} ${String(readiness?.json?.status ?? "")})`);
    }
    if (Date.now() - started - reported >= 30_000) {
      reported = Date.now() - started;
      options.onProgress(`waiting for /readyz on ${dataDir}: ${readiness?.status ?? "no answer"} ${JSON.stringify(readiness?.json?.blockedBy ?? []).slice(0, 200)} (${Math.round(reported / 1000)} s)`);
    }
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 1_000));
  }
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
    let hub: HubHandle | undefined;
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
      session.close();
      const reopened = await hubProbeOpenDocument(hub.baseUrl, token, spaceId, artifactId, "backup-drill");
      const frontier = frontierOf(reopened.welcome);
      reopened.close();
      const before = await snapshot(hub.baseUrl, token, spaceId, artifactId);
      result.seedMs = Date.now() - seedStarted;
      const stopStarted = Date.now();
      await hub.stop();
      hub = undefined;
      result.sigtermToExitMs = Date.now() - stopStarted;
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
