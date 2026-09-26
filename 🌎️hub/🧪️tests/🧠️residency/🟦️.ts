/** 🧠️ Residency watch: what a running hub keeps resident while every creatable kind is opened, and whether it lets go.
 *
 * Signs one credential user in, creates a private probe space, and per creatable kind of the space's creation catalog runs
 * the server-owned creation to `ready` and asks for the editor open plan (the hub loads the kind's package to answer it),
 * sampling the hub process's resident set before and after each kind; then keeps sampling through a settle window so an
 * idle release or a residency-LRU eviction shows as a falling resident set. The sampler is the operating system's own
 * process table (`ps` on macOS and Linux, `tasklist` on Windows); the hub process is the one listening on the hub's port
 * (`lsof` on macOS and Linux, `netstat` on Windows) unless named.
 *
 * With `rounds` above one the kinds are created round-robin that many times, and after every creation the hub's own
 * compiled-guest residency (`GET /admin/api/observability` `residency`, the probe user being an admin subject) is read:
 * per round the watch reports the compiles, hits, admissions, bypasses and releases, so a round-robin over more kinds than
 * the configured budget holds shows whether the resident set stays stable (compiles per round = kinds that do not fit) or
 * thrashes (every kind compiles every round). Every created document's active checkpoint pair is fetched and digested with
 * its own document id replaced by a fixed token: a kind whose digest differs between rounds answered differently after its
 * guest was released and compiled again.
 *
 * Promoted from the session-12/13 ticket harnesses `wp-h10/rss-sampler.sh` + `wp-h11/h11-open-plan-probe.ts` (ticket
 * 26/09/23, acceptance ledger 2.7: "RSS-watch wrapper around the post-publish open-plan probe").
 */
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import type { TrustedCatalogGuestResidencyStateV1 } from "../../📊️observability/🟦️.ts";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenPlan, hubProbeSignIn } from "../../🤝️integration-harness/🟦️.ts";
import { listeningProcessId } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";

/** 🎛️ One watch. */
export type ResidencyWatchOptions = Readonly<{
  hub: string;
  pid: number | null;
  email: string;
  password: string;
  kinds: readonly string[];
  intervalMs: number;
  settleMs: number;
  budgetMiB: number | null;
  rounds: number;
  signal: AbortSignal;
  onProgress: (line: string) => void;
}>;

/** 📏️ One sample of the hub's resident set. */
export type ResidencySample = Readonly<{ atMs: number; phase: string; rssMiB: number }>;

/** 🧾️ One kind in one round: creation, open plan, the resident set around it, the hub's residency after it and the digest
 * of the created pair with its document id replaced. */
export type ResidencyRow = { round: number; kindId: string; createMs?: number; openPlanStatus?: number; rssBeforeMiB?: number; rssAfterMiB?: number; residency?: TrustedCatalogGuestResidencyStateV1; pairDigest?: string; error?: string };

/** 🔁️ One round's residency deltas and whether its resident set equals the previous round's. */
export type ResidencyRound = { round: number; compiles: number; hits: number; admitted: number; bypassed: number; released: number; residentGuests: number; residentBytes: number; compileMs: number };

/** 📊️ The watch's report. */
export type ResidencyReport = {
  hub: string;
  pid: number;
  samples: ResidencySample[];
  rows: ResidencyRow[];
  rounds: ResidencyRound[];
  pairsAgree: boolean;
  divergentKinds: string[];
  baselineMiB: number;
  peakMiB: number;
  finalMiB: number;
  releasedMiB: number;
  budgetMiB: number | null;
  cancelled: boolean;
};

/** 🧊️ The hub's compiled-guest residency, or `undefined` when the route refuses the probe user or the hub has no catalog. */
async function readResidency(hub: string, token: string): Promise<TrustedCatalogGuestResidencyStateV1 | undefined> {
  const answer = await hubProbeCall(hub, "GET", "/admin/api/observability", token);
  return answer.status === 200 && answer.json?.residency ? (answer.json.residency as TrustedCatalogGuestResidencyStateV1) : undefined;
}

/** 🧬️ SHA-256 of a document's active checkpoint pair content — the pack and SPR bytes of the canonical pair stream's data
 * records (`🛰️lag-rebootstrap`: frames of a u32 big-endian length and a payload; a data payload is kind 2, part, u32 ordinal,
 * u64 offset, u32 length, bytes), never its header, which names the checkpoint — with every occurrence of the document's
 * own id replaced by a fixed token. */
async function pairDigest(hub: string, token: string, spaceId: string, documentId: string): Promise<string> {
  const answer = await hubProbeCall(hub, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  if (answer.status !== 200) throw new Error(`active pair ${answer.status}`);
  const stream = Buffer.from(answer.bytes);
  const parts = new Map<number, Buffer[]>();
  for (let at = 0; at + 4 <= stream.length; ) {
    const length = stream.readUInt32BE(at);
    const payload = stream.subarray(at + 4, at + 4 + length);
    if (payload[0] === 2 && payload.length >= 18) parts.set(payload[1]!, [...(parts.get(payload[1]!) ?? []), payload.subarray(18)]);
    at += 4 + length;
  }
  const id = Buffer.from(documentId, "utf8");
  const hash = createHash("sha256");
  for (const part of [...parts.keys()].sort()) {
    const bytes = Buffer.concat(parts.get(part)!);
    let from = 0;
    for (let found = bytes.indexOf(id, from); found >= 0; found = bytes.indexOf(id, from)) {
      hash.update(bytes.subarray(from, found)).update("<document>");
      from = found + id.length;
    }
    hash.update(bytes.subarray(from)).update(`<part ${part}>`);
  }
  return hash.digest("hex");
}

/** 📏️ The resident set of `pid` in MiB, from the operating system's process table. */
export function residentMiB(pid: number): number {
  if (process.platform === "win32") {
    const answer = spawnSync("tasklist", ["/FI", `PID eq ${pid}`, "/FO", "CSV", "/NH"], { encoding: "utf8" });
    const kib = Number((answer.stdout.split(",").at(-1) ?? "").replace(/[^0-9]/gu, ""));
    if (!Number.isFinite(kib) || kib <= 0) throw new Error(`tasklist reports no memory for pid ${pid}`);
    return kib / 1024;
  }
  const answer = spawnSync("ps", ["-o", "rss=", "-p", String(pid)], { encoding: "utf8" });
  const kib = Number(answer.stdout.trim());
  if (!Number.isFinite(kib) || kib <= 0) throw new Error(`ps reports no resident set for pid ${pid}`);
  return kib / 1024;
}

/** 🧠️ Runs the watch. */
export async function runResidencyWatch(options: ResidencyWatchOptions): Promise<ResidencyReport> {
  const pid = options.pid ?? listeningProcessId(Number(new URL(options.hub).port));
  const started = Date.now();
  const samples: ResidencySample[] = [];
  const sample = (phase: string): number => {
    const rssMiB = Math.round(residentMiB(pid) * 10) / 10;
    samples.push({ atMs: Date.now() - started, phase, rssMiB });
    return rssMiB;
  };
  let phase = "baseline";
  const ticker = setInterval(() => sample(phase), options.intervalMs);
  const baselineMiB = sample("baseline");
  const rows: ResidencyRow[] = [];
  const roundsReport: ResidencyRound[] = [];
  try {
    const token = await hubProbeSignIn(options.hub, options.email, options.password, "residency");
    const spaceId = await hubProbeCreateSpace(options.hub, token, `Residency watch ${new Date().toISOString()}`);
    const catalog = await hubProbeCreationCatalog(options.hub, token, spaceId);
    const kinds = catalog.kinds.filter((kind) => options.kinds.length === 0 || options.kinds.includes(kind.kindId));
    options.onProgress(`pid ${pid} baseline ${baselineMiB} MiB; ${kinds.length} creatable kinds × ${options.rounds} rounds`);
    let previous = await readResidency(options.hub, token);
    for (let round = 1; round <= options.rounds && !options.signal.aborted; round += 1) {
      const before = previous;
      for (const [index, kind] of kinds.entries()) {
        if (options.signal.aborted) break;
        phase = `round:${round}:kind:${kind.kindId}`;
        const row: ResidencyRow = { round, kindId: kind.kindId, rssBeforeMiB: sample(phase) };
        try {
          const created = await hubProbeCreateArtifact(options.hub, token, spaceId, catalog.generationId, kind.kindId, `Residency ${round} ${kind.kindId}`);
          row.createMs = created.ms;
          row.openPlanStatus = (await hubProbeOpenPlan(options.hub, token, spaceId, created.artifactId, "residency-watch")).status;
          row.pairDigest = await pairDigest(options.hub, token, spaceId, created.artifactId);
        } catch (error) {
          row.error = String(error instanceof Error ? error.message : error).slice(0, 300);
        }
        row.residency = await readResidency(options.hub, token);
        row.rssAfterMiB = sample(phase);
        rows.push(row);
        options.onProgress(`round ${round} ${index + 1}/${kinds.length} ${kind.kindId}: plan ${row.openPlanStatus ?? "-"} create ${row.createMs ?? "-"} ms rss ${row.rssBeforeMiB} → ${row.rssAfterMiB} MiB${row.residency ? ` resident ${row.residency.residentGuests}/${row.residency.registeredGuests} compiles ${row.residency.compiles} hits ${row.residency.hits} released ${row.residency.released}` : ""}${row.error ? ` ${row.error}` : ""}`);
      }
      const after = rows.at(-1)?.residency ?? previous;
      if (before && after) {
        roundsReport.push({ round, compiles: after.compiles - before.compiles, hits: after.hits - before.hits, admitted: after.admitted - before.admitted, bypassed: after.bypassed - before.bypassed, released: after.released - before.released, residentGuests: after.residentGuests, residentBytes: after.residentBytes, compileMs: Math.round((after.compileMicros - before.compileMicros) / 1_000) });
        options.onProgress(`round ${round}: ${JSON.stringify(roundsReport.at(-1))}`);
      }
      previous = after;
    }
    phase = "settle";
    const settleUntil = Date.now() + options.settleMs;
    while (Date.now() < settleUntil && !options.signal.aborted) await new Promise((resolveDelay) => setTimeout(resolveDelay, Math.min(options.intervalMs, settleUntil - Date.now())));
  } finally {
    clearInterval(ticker);
  }
  const finalMiB = sample("final");
  const peakMiB = Math.max(...samples.map((entry) => entry.rssMiB));
  const digests = new Map<string, Set<string>>();
  for (const row of rows) if (row.pairDigest) digests.set(row.kindId, (digests.get(row.kindId) ?? new Set()).add(row.pairDigest));
  const divergentKinds = [...digests.entries()].filter(([, seen]) => seen.size > 1).map(([kindId]) => kindId);
  return { hub: options.hub, pid, samples, rows, rounds: roundsReport, pairsAgree: divergentKinds.length === 0, divergentKinds, baselineMiB, peakMiB, finalMiB, releasedMiB: Math.round((peakMiB - finalMiB) * 10) / 10, budgetMiB: options.budgetMiB, cancelled: options.signal.aborted };
}
