export type LcovFileRecord = { path: string; lines: Map<number, number> };

/** 📊️Parses one LCOV text blob into per-file line-hit maps (only `SF:`/`DA:`/`end_of_record` — the subset every toolchain here emits). */
export function parseLcov(text: string): LcovFileRecord[] {
  const records: LcovFileRecord[] = [];
  let current: LcovFileRecord | undefined;
  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (line.startsWith("SF:")) {
      current = { path: line.slice(3), lines: new Map() };
      records.push(current);
    } else if (line.startsWith("DA:") && current) {
      const [lineNoStr, hitsStr] = line.slice(3).split(",");
      const lineNo = Number(lineNoStr);
      const hits = Number(hitsStr);
      current.lines.set(lineNo, (current.lines.get(lineNo) ?? 0) + hits);
    } else if (line === "end_of_record") {
      current = undefined;
    }
  }
  return records;
}

/** 📊️Merges LCOV records from multiple toolchain runs — line-hit counts sum, covered line numbers union. */
export function mergeLcov(recordSets: LcovFileRecord[][]): Map<string, Map<number, number>> {
  const merged = new Map<string, Map<number, number>>();
  for (const records of recordSets) {
    for (const record of records) {
      const target = merged.get(record.path) ?? new Map<number, number>();
      for (const [lineNo, hits] of record.lines) target.set(lineNo, (target.get(lineNo) ?? 0) + hits);
      merged.set(record.path, target);
    }
  }
  return merged;
}

/** 📊️Renders a merged coverage map back to LCOV text (one `SF:`/`DA:`×N/`end_of_record` block per file, lines sorted ascending). */
export function renderLcov(merged: Map<string, Map<number, number>>): string {
  const chunks: string[] = [];
  for (const [path, lines] of merged) {
    chunks.push(`SF:${path}`);
    for (const lineNo of [...lines.keys()].sort((a, b) => a - b)) chunks.push(`DA:${lineNo},${lines.get(lineNo)}`);
    chunks.push("end_of_record", "");
  }
  return chunks.join("\n");
}

/** 📊️Expands a `go test -coverprofile` text block (`mode: <mode>` header, then `file:startLine.startCol,endLine.endCol numStmt count` records) into per-line LCOV records — every line in a statement's range is marked hit iff `count > 0`. */
export function goProfileToLcov(text: string): LcovFileRecord[] {
  const byFile = new Map<string, Map<number, number>>();
  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("mode:")) continue;
    const match = /^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$/.exec(line);
    if (!match) continue;
    const [, file, startStr, endStr, countStr] = match;
    if (file === undefined || startStr === undefined || endStr === undefined || countStr === undefined) throw new Error("Go coverage record captures required");
    const start = Number(startStr);
    const end = Number(endStr);
    const hits = Number(countStr);
    const lines = byFile.get(file) ?? new Map<number, number>();
    for (let lineNo = start; lineNo <= end; lineNo++) lines.set(lineNo, (lines.get(lineNo) ?? 0) + hits);
    byFile.set(file, lines);
  }
  return [...byFile.entries()].map(([path, lines]) => ({ path, lines }));
}
