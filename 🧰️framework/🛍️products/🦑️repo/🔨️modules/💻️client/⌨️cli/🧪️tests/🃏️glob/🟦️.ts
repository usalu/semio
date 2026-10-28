import { readFileSync, mkdtempSync, mkdirSync, openSync, ftruncateSync, closeSync, statSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { globSync } from "glob";
import Ajv from "ajv";
import { minimatch } from "minimatch";

/** 🃏️ One neutral policy-glob expectation: the pattern, the path it is matched against and the verdict. */
interface GlobFixture {
  readonly cases: readonly Readonly<{ id: string; pattern: string; path: string; match: boolean }>[];
}

/** 🧭️ Verifies the neutral policy-glob expectations against an independent matcher. */
export function verifyFixtureGlobOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🃏️glob/🔣️.json", import.meta.url), "utf8"));
  
  for (const row of fixture.cases) if (minimatch(row.path, row.pattern) !== row.match) throw new Error(`Reference glob mismatch: ${row.id}`);
  console.log(`Fixture glob reference: ${fixture.cases.length} vectors verified`);
}

/** 🪟️ Independently enumerates the frozen generated-output purge case with the existing third-party glob oracle. */
export function verifyTicketRetentionOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🎫️tickets/🧪️tests/🧫️fixtures/🧹️purge-generated/🧹️cases.json", import.meta.url), "utf8"));
  const frozen = fixture.cases.find((row: { id: string }) => row.id === "closed-ticket-oversized-removes-only-oversized-generated-output");
  const ticket = frozen.tickets[0] as { id: string; entries: { path: string; kind: string; bytes: number }[] };
  const owner = process.env.SEMIO_TICKET_DIR;
  const root = mkdtempSync(join(owner ? join(owner, "🗑️generated") : tmpdir(), "ticket-retention-oracle-"));
  try {
    const entries = ticket.entries.filter(row => row.kind === "file");
    for (const row of entries) {
      const path = join(root, row.path); mkdirSync(dirname(path), { recursive: true });
      const fd = openSync(path, "w"); try { ftruncateSync(fd, row.bytes); } finally { closeSync(fd); }
    }
    const paths = globSync("**/*", { cwd: root, dot: true, nodir: true }).map(path => path.replaceAll("\\", "/")).sort();
    const eligible = globSync("🗑️generated/**/*", { cwd: root, dot: true, nodir: true }).map(path => path.replaceAll("\\", "/"));
    const totals = new Map<string, number>();
    for (const path of eligible) for (let folder = dirname(path); folder !== "🗑️generated"; folder = dirname(folder)) totals.set(folder, (totals.get(folder) ?? 0) + statSync(join(root, path)).size);
    const removed = paths.filter(path => eligible.includes(path) && (statSync(join(root, path)).size > 5 * 1024 * 1024 || [...totals].some(([folder, bytes]) => bytes > 10 * 1024 * 1024 && path.startsWith(folder + "/"))));
    const planned = (frozen.expect.removals as { path: string }[]).map(row => row.path.slice(ticket.id.length + 1));
    const expected = entries.map(row => row.path).filter(path => planned.some(root => path === root || path.startsWith(root + "/"))).sort();
    if (JSON.stringify(removed) !== JSON.stringify(expected)) throw new Error(`Independent generated-only filesystem oracle differs: ${JSON.stringify({ removed, expected, eligible })}`);
    console.log(`Ticket retention filesystem reference: ${entries.length} files verified with glob`);
  } finally { rmSync(root, { recursive: true, force: true }); }
}
