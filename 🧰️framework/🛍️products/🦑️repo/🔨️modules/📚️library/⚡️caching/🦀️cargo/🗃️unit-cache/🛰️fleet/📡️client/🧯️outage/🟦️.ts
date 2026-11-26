import { closeSync, fstatSync, openSync, readFileSync, readSync, statSync, type BigIntStats } from "node:fs";

const tailBytes = 4096n;

type Evidence = { stamp: BigIntStats; cutoff: number | undefined; complete: boolean };

function sameFile(left: BigIntStats, right: BigIntStats): boolean {
  return left.dev === right.dev && left.ino === right.ino && left.size === right.size &&
    left.mtimeNs === right.mtimeNs && left.ctimeNs === right.ctimeNs;
}

/** 📈️ Once ranked, a history can be checked against any wall clock using only the threshold timestamp. */
function cutoffTimestamp(history: string, faults: number): number | undefined {
  const latest: number[] = [];
  let start = 0;
  while (start < history.length) {
    const newline = history.indexOf("\n", start);
    const end = newline === -1 ? history.length : newline;
    const value = end === start ? NaN : Number(history.slice(start, end));
    start = end + 1;
    if (Number.isNaN(value) || (latest.length === faults && value <= latest[faults - 1]!)) continue;
    const position = latest.findIndex(timestamp => timestamp < value);
    latest.splice(position === -1 ? latest.length : position, 0, value);
    if (latest.length > faults) latest.pop();
  }
  return latest.length === faults ? latest[faults - 1] : undefined;
}

/** 🧯️ Proves an outage from a bounded tail, then resolves ambiguous tails against the whole history. */
export function createFleetOutageCheckV1(path: string, faults: number, windowMs: number): (now?: number) => boolean {
  let cached: Evidence | undefined;
  return (now = Date.now()) => {
    const down = (cutoff: number | undefined) => cutoff !== undefined && now - cutoff < windowMs;
    try {
      for (let attempt = 0; attempt < 2; attempt++) {
        const stamp = statSync(path, { bigint: true });
        if (!stamp.isFile()) break;
        const prior = cached && sameFile(cached.stamp, stamp) ? cached : undefined;
        if (prior && (prior.complete || down(prior.cutoff))) return down(prior.cutoff);

        const file = openSync(path, "r");
        try {
          if (!sameFile(stamp, fstatSync(file, { bigint: true }))) continue;
          let cutoff = prior?.cutoff;
          let complete = false;
          if (!prior) {
            const size = stamp.size < tailBytes ? stamp.size : tailBytes;
            const start = stamp.size - size;
            const bytes = Buffer.allocUnsafe(Number(size));
            if (readSync(file, bytes, 0, bytes.length, start) !== bytes.length) continue;
            complete = start === 0n;
            const first = complete ? 0 : bytes.indexOf(10) + 1;
            const tail = complete || first > 0 ? bytes.toString("utf8", first) : "";
            cutoff = cutoffTimestamp(tail, faults);
          }
          if (!complete && !down(cutoff)) {
            cutoff = cutoffTimestamp(readFileSync(file, "utf8"), faults);
            complete = true;
          }
          if (!sameFile(stamp, fstatSync(file, { bigint: true }))) continue;
          cached = { stamp, cutoff, complete };
          return down(cutoff);
        } finally {
          closeSync(file);
        }
      }
      cached = undefined;
      return down(cutoffTimestamp(readFileSync(path, "utf8"), faults));
    } catch {
      cached = undefined;
      return false;
    }
  };
}
