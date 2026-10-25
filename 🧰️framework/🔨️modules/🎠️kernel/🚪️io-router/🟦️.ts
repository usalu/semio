/** 🚪️ The IO entry graph: fidelity and confidence ranking, the entry graph of hops between dialects, `ioRun` and `ioIdentify`.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import { dialectCoordinate, type IoFidelity, type IoEntryDescriptor, type IoRoute } from "../../🚪️io/🧬️schema/📦️packages/🟦️typescript/🟦️.ts";
import { dialectEquals } from "../🔀️dialect-equality/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️IoRouter
function ioFidelityRank(fidelity: IoFidelity): number {
  switch (fidelity) {
    case "Exact":
      return 3;
    case "Canonical":
      return 2;
    case "Semantic":
      return 1;
    case "Lossy":
      return 0;
  }
}

function ioFidelityFromRank(rank: number): IoFidelity {
  if (rank >= 3) return "Exact";
  if (rank === 2) return "Canonical";
  if (rank === 1) return "Semantic";
  return "Lossy";
}

/** 🎚️ Mirrors Rust `io_schema::Confidence` — how sure an `io-identify`/`io-sniff` is that a payload
 * is a given dialect. Same no-`rename_all` wire form as {@link IoFidelity}. */
export type IoConfidence = "None" | "Low" | "Medium" | "High";

function ioConfidenceRank(confidence: IoConfidence): number {
  switch (confidence) {
    case "None":
      return 0;
    case "Low":
      return 1;
    case "Medium":
      return 2;
    case "High":
      return 3;
  }
}

/** 🌉️ Inverse of {@link ioConfidenceRank} — the WIT `io-sniff` guest export returns a raw `u8` rank
 * byte (`Confidence::rank()`); the caller of {@link ioIdentify} reconstructs the typed value. */
export function ioConfidenceFromRank(rank: number): IoConfidence {
  if (rank >= 3) return "High";
  if (rank === 2) return "Medium";
  if (rank === 1) return "Low";
  return "None";
}

/** 🗂️ One plugin's `list-io-entries` roster, as `IoEntryGraph.build` consumes it. */
export type IoEntryGraphPlugin = {
  readonly pluginId: string;
  readonly entries: readonly IoEntryDescriptor[];
};

function ioEntryKey(from: ArtifactDialect, into: ArtifactDialect): string {
  return `${dialectCoordinate(from)}->${dialectCoordinate(into)}`;
}

/**
 * 🧭️ TS twin of the host `IoRouter`'s NEW io-mechanism graph (`💻️os/🔌️plugin/🖥️host/🦀️.rs`,
 * region `🔖️IoRouter` — the `io_entries`/`resolve_io_route`/`run_io`/`identify` additions,
 * `📓️w1-d-report.md`). `(from, into) -> owning pluginId` merged from every loaded plugin's
 * `list-io-entries` roster, plus deterministic route resolution: highest minimum fidelity, then
 * fewest hops, then lexicographic `into` coordinate order — a pure function of the (from,into) KEY
 * SET, never of plugin registration order (mirrors Rust `resolve_io_route`'s `BTreeMap` +
 * full-candidate-set-sorted-at-the-end shape). Parity with the Rust side is asserted by running the
 * identical fixture through both — see `🧪️w1d-io-router-parity.ts` in this ticket's folder.
 */
export class IoEntryGraph {
  private readonly ownerByEntry: ReadonlyMap<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>;

  private constructor(ownerByEntry: ReadonlyMap<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>) {
    this.ownerByEntry = ownerByEntry;
  }

  /** 🏗️ Merges every `(pluginId, entries)` roster into one graph — mirrors Rust `IoRouter::
   * register_plugin`'s io-entries half. A `(from,into)` key already owned by a DIFFERENT plugin
   * throws (`IoEntryRouteConflict`'s TS twin); re-registering the SAME plugin's own key is
   * idempotent (first registration wins). */
  static build(plugins: readonly IoEntryGraphPlugin[]): IoEntryGraph {
    const ownerByEntry = new Map<string, { readonly pluginId: string; readonly descriptor: IoEntryDescriptor }>();
    for (const plugin of plugins) {
      for (const descriptor of plugin.entries) {
        const key = ioEntryKey(descriptor.from, descriptor.into);
        const existing = ownerByEntry.get(key);
        if (existing) {
          if (existing.pluginId !== plugin.pluginId) {
            throw new Error(`io entry route conflict for ${key}: ${JSON.stringify(existing.pluginId)} already owns it; ${JSON.stringify(plugin.pluginId)} cannot replace it`);
          }
          continue;
        }
        ownerByEntry.set(key, { pluginId: plugin.pluginId, descriptor });
      }
    }
    return new IoEntryGraph(ownerByEntry);
  }

  /** 🛤️ SAME deterministic ranking rule as Rust `resolve_io_route`/`io::io_mechanism::
   * resolve_route`: breadth-bounded (`maxHops` clamped to ≤3), cycle-free simple-path enumeration,
   * ranked by (highest minimum fidelity, fewest hops, lexicographic joined `into` coordinate) —
   * the FULL candidate set is sorted at the END (never short-circuited), so the winner never
   * depends on iteration/insertion/registration order. */
  route(from: ArtifactDialect, into: ArtifactDialect, maxHops = 3): IoRoute {
    const bound = Math.min(maxHops, 3);
    if (bound <= 0) throw new Error(`io_routes ${dialectCoordinate(from)} -> ${dialectCoordinate(into)}: max hops clamped to 0`);
    const candidates: IoEntryDescriptor[][] = [];
    const path: IoEntryDescriptor[] = [];
    const visited = new Set<string>([dialectCoordinate(from)]);
    const walk = (current: ArtifactDialect, remainingHops: number): void => {
      if (remainingHops === 0) return;
      for (const { descriptor } of this.ownerByEntry.values()) {
        if (!dialectEquals(descriptor.from, current)) continue;
        const nextCoordinate = dialectCoordinate(descriptor.into);
        if (visited.has(nextCoordinate)) continue;
        path.push(descriptor);
        if (dialectEquals(descriptor.into, into)) {
          candidates.push([...path]);
        } else {
          visited.add(nextCoordinate);
          walk(descriptor.into, remainingHops - 1);
          visited.delete(nextCoordinate);
        }
        path.pop();
      }
    };
    walk(from, bound);
    if (candidates.length === 0) throw new Error(`no io route from ${dialectCoordinate(from)} to ${dialectCoordinate(into)} within ${bound} hops`);
    const rank = (hops: readonly IoEntryDescriptor[]): readonly [number, number, string] => {
      const minFidelity = Math.min(...hops.map((hop) => ioFidelityRank(hop.fidelity)));
      const joined = hops.map((hop) => dialectCoordinate(hop.into)).join(",");
      return [-minFidelity, hops.length, joined];
    };
    const sorted = [...candidates].sort((a, b) => {
      const [aInverseFidelity, aLength, aJoined] = rank(a);
      const [bInverseFidelity, bLength, bJoined] = rank(b);
      if (aInverseFidelity !== bInverseFidelity) return aInverseFidelity - bInverseFidelity;
      if (aLength !== bLength) return aLength - bLength;
      return aJoined.localeCompare(bJoined);
    });
    const best = sorted[0]!;
    const minFidelityRank = Math.min(...best.map((hop) => ioFidelityRank(hop.fidelity)));
    return { hops: best, fidelity: ioFidelityFromRank(minFidelityRank) };
  }

  /** 🪪️ The plugin that owns hop `(from,into)`, or `undefined`. */
  ownerOf(from: ArtifactDialect, into: ArtifactDialect): string | undefined {
    return this.ownerByEntry.get(ioEntryKey(from, into))?.pluginId;
  }

  /** 🔍️ Every registered hop whose `from` is `carrier` and which declares a `sniff` — the fan-out
   * set {@link ioIdentify} sniffs, mirrors Rust `IoRouter::identify`'s carrier filter. */
  carrierEntries(carrier: ArtifactDialect): ReadonlyArray<{ readonly into: ArtifactDialect; readonly pluginId: string }> {
    const found: Array<{ readonly into: ArtifactDialect; readonly pluginId: string }> = [];
    for (const { pluginId, descriptor } of this.ownerByEntry.values()) {
      if (dialectEquals(descriptor.from, carrier) && descriptor.sniffs) found.push({ into: descriptor.into, pluginId });
    }
    return found;
  }
}

/** ▶️ Runs one hop of a resolved {@link IoRoute} — the caller's bridge into an actual loaded
 * plugin's `io-run` export (this domain-neutral framework module never calls a plugin worker
 * itself, same boundary {@link AppRouter}/{@link ArtifactMutationRouter} already draw). */
export type IoHopRunner = (pluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array) => Promise<Uint8Array> | Uint8Array;

/**
 * 🪞️ TS twin of Rust `IoRouter::run_io` — resolves the WHOLE `from -> into` route over `graph`,
 * then, BEFORE running any hop, refuses the ENTIRE route (no partial execution) if any hop is
 * owned by `callingPluginId` itself: executing that hop would call back into the calling plugin's
 * own in-flight worker call — the same reentrancy hazard the Rust guard exists to prevent. Each
 * hop's output payload feeds the next hop's input via `runHop`.
 */
export async function ioRun(graph: IoEntryGraph, callingPluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array, runHop: IoHopRunner, maxHops = 3): Promise<Uint8Array> {
  const route = graph.route(from, into, maxHops);
  const hops = route.hops.map((hop) => {
    const owner = graph.ownerOf(hop.from, hop.into);
    if (owner === undefined) throw new Error(`io-run: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} vanished from the graph between resolve and execute`);
    if (owner === callingPluginId) {
      throw new Error(
        `io-run refused: hop ${dialectCoordinate(hop.from)} -> ${dialectCoordinate(hop.into)} is owned by the calling plugin ${JSON.stringify(callingPluginId)} itself — executing it would re-enter that plugin's own in-flight worker call`,
      );
    }
    return { hop, owner };
  });
  let current = payload;
  for (const { hop, owner } of hops) {
    current = await runHop(owner, hop.from, hop.into, current);
  }
  return current;
}

/** 👃️ Sniffs one plugin's `(from,into)` hop — the caller's bridge into an actual loaded plugin's
 * `io-sniff` export, returning the raw `Confidence::rank()` byte. Same DI boundary as {@link IoHopRunner}. */
export type IoSniffRunner = (pluginId: string, from: ArtifactDialect, into: ArtifactDialect, payload: Uint8Array) => Promise<number> | number;

/**
 * 🕵️ TS twin of Rust `IoRouter::identify` — fans {@link IoSniffRunner} out across every OTHER
 * plugin's `carrier`-`from` entries (skipping `callingPluginId`'s own, same reentrancy reason
 * {@link ioRun} refuses a self-owned hop — a fan-out is best-effort, so this SKIPS rather than
 * refuses the whole call), merges by confidence descending then coordinate ascending.
 */
export async function ioIdentify(graph: IoEntryGraph, callingPluginId: string, carrier: ArtifactDialect, payload: Uint8Array, sniffHop: IoSniffRunner): Promise<ReadonlyArray<readonly [ArtifactDialect, IoConfidence]>> {
  const candidates = graph.carrierEntries(carrier).filter((entry) => entry.pluginId !== callingPluginId);
  const found: Array<[ArtifactDialect, IoConfidence]> = [];
  for (const { into, pluginId } of candidates) {
    const confidence = ioConfidenceFromRank(await sniffHop(pluginId, carrier, into, payload));
    if (confidence !== "None") found.push([into, confidence]);
  }
  found.sort((a, b) => {
    const rankDiff = ioConfidenceRank(b[1]) - ioConfidenceRank(a[1]);
    if (rankDiff !== 0) return rankDiff;
    return dialectCoordinate(a[0]).localeCompare(dialectCoordinate(b[0]));
  });
  return found;
}
//#endregion 🔖️IoRouter
// #endregion 🎠️Kernel
