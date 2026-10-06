import { readFileSync, mkdtempSync, mkdirSync, openSync, ftruncateSync, closeSync, statSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { globSync } from "glob";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { minimatch } from "minimatch";

/** 🃏️ One neutral policy-glob expectation: the pattern, the path it is matched against and the verdict. */
interface GlobFixture {
  readonly cases: readonly Readonly<{ id: string; pattern: string; path: string; match: boolean }>[];
}

/** 🧭️ Verifies the neutral policy-glob expectations against an independent matcher. */
export function verifyFixtureGlobOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🃏️glob/🔣️.json", import.meta.url), "utf8"));
  
  for (const row of fixture.cases) if (minimatch(row.path, row.pattern) !== row.match) throw new Error(`Reference glob mismatch: ${row.id}`);
  console.log(`Fixture glob reference: ${fixture.cases.length} vectors verified`);
}

/** 🪟️ Independently enumerates the neutral lifecycle filesystem with the existing third-party glob oracle. */
export function verifyTicketRetentionOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🎫️tickets/🧫️fixtures/🔓️open-close-reopen-lifecycle/🔓️lifecycle.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../../../🎫️tickets/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  if (!new Ajv2020({ strict: false }).validate({ ...schema, $ref: "#/$defs/LifecycleVectorFile" }, fixture)) throw new Error("Invalid neutral retention fixture");
  const owner = process.env.SEMIO_TICKET_DIR;
  const root = mkdtempSync(join(owner ? join(owner, "🗑️generated") : tmpdir(), "ticket-retention-oracle-"));
  try {
    for (const row of fixture.purge.retention) {
      const path = join(root, row.path); mkdirSync(dirname(path), { recursive: true });
      const fd = openSync(path, "w"); try { ftruncateSync(fd, row.bytes); } finally { closeSync(fd); }
    }
    const paths = globSync("**/*", { cwd: root, dot: true, nodir: true }).map(path => path.replaceAll("\\", "/")).sort();
    const eligible = globSync("🗑️generated/**/*", { cwd: root, dot: true, nodir: true }).map(path => path.replaceAll("\\", "/"));
    const totals = new Map<string, number>();
    for (const path of eligible) for (let folder = dirname(path); folder !== "🗑️generated"; folder = dirname(folder)) totals.set(folder, (totals.get(folder) ?? 0) + statSync(join(root, path)).size);
    const removed = paths.filter(path => eligible.includes(path) && (statSync(join(root, path)).size > 5 * 1024 * 1024 || [...totals].some(([folder, bytes]) => bytes > 10 * 1024 * 1024 && path.startsWith(folder + "/"))));
    const expected = fixture.purge.retention.filter((row: { retained: boolean }) => !row.retained).map((row: { path: string }) => row.path).sort();
    if (JSON.stringify(removed) !== JSON.stringify(expected)) throw new Error(`Independent generated-only filesystem oracle differs: ${JSON.stringify({ removed, expected, eligible })}`);
    console.log(`Ticket retention filesystem reference: ${fixture.purge.retention.length} vectors verified with glob and AJV`);
  } finally { rmSync(root, { recursive: true, force: true }); }
}
