import { Database } from "bun:sqlite";
import { expect, spyOn, test } from "bun:test";
import { appendFileSync, mkdirSync, mkdtempSync, renameSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createFleetClientV1, readFleetSettingsV1 } from "../../🟦️.ts";
import { createFleetOutageCheckV1 } from "../../🧯️outage/🟦️.ts";
import { FLEET_POLICY_V1 } from "../../../🪪️identity/🟦️.ts";
import corpus from "./🧫️cases.json" with { type: "json" };

const breaker = FLEET_POLICY_V1.breaker;
const scratch = () => mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "fleet-down-"));

function fixtureHistory(caseFixture: typeof corpus[number]): string {
  const lines = [
    ...(caseFixture.leading ?? []),
    ...Array.from({ length: caseFixture.repeat?.count ?? 0 }, () => caseFixture.repeat!.line),
    ...(caseFixture.longLineLength ? ["x".repeat(caseFixture.longLineLength)] : []),
    ...caseFixture.lines,
  ];
  return lines.join("\n") + (lines.length && caseFixture.trailingNewline !== false ? "\n" : "");
}

test("the bounded reader reproduces the original timestamp law and SQLite on long and non-monotonic histories", () => {
  const directory = scratch();
  const path = join(directory, "down");
  const db = new Database(":memory:");
  db.run("CREATE TABLE failure(timestamp REAL)");
  const add = db.prepare("INSERT INTO failure VALUES (?)");
  const tally = db.query<{ total: number }, [number, number]>("SELECT count(*) AS total FROM failure WHERE ? - timestamp < ?");
  const record = db.transaction((lines: string[]) => {
    db.run("DELETE FROM failure");
    for (const line of lines) {
      const timestamp = Number(line);
      if (!Number.isNaN(timestamp)) add.run(timestamp);
    }
  });
  const recent = createFleetOutageCheckV1(path, breaker.downFaults, breaker.downMilliseconds);
  try {
    for (const scenario of corpus) {
      const history = fixtureHistory(scenario);
      writeFileSync(path, history, "utf8");
      const lines = history.split("\n").filter(Boolean);
      record(lines);
      for (const [now, down] of scenario.checks as [number, boolean][]) {
        const exact = lines.filter(at => now - Number(at) < breaker.downMilliseconds).length >= breaker.downFaults;
        const fromSqlite = tally.get(now, breaker.downMilliseconds)!.total >= breaker.downFaults;
        expect([exact, fromSqlite, recent(now)], scenario.name).toEqual([down, down, down]);
      }
    }
  } finally {
    db.close();
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the client sees append, atomic replacement, in-place reset and removal after a cached result", () => {
  const directory = scratch();
  const settings = readFleetSettingsV1(directory, { SEMIO_REMOTE_CACHE_ENDPOINT: "http://localhost.invalid" })!;
  mkdirSync(settings.stateDirectory);
  const path = join(settings.stateDirectory, FLEET_POLICY_V1.layout.down);
  const client = createFleetClientV1(settings);
  const other = createFleetClientV1(settings);
  const clock = spyOn(Date, "now").mockReturnValue(100_000);
  try {
    expect(client.state()).toBe("ok");
    appendFileSync(path, "99999\n");
    expect(client.usable()).toBe(true);
    expect(other.usable()).toBe(true);
    appendFileSync(path, "99998\n");
    expect(client.state()).toBe("down");
    expect(other.usable()).toBe(false);

    const replacement = join(settings.stateDirectory, "down.new");
    writeFileSync(replacement, "80000\n80000\n");
    renameSync(replacement, path);
    const timestamp = new Date("2026-11-25T00:00:00.000Z");
    utimesSync(path, timestamp, timestamp);
    expect(client.usable()).toBe(true);
    clock.mockReturnValue(90_000);
    expect(other.state()).toBe("down");
    expect(client.state()).toBe("down");

    writeFileSync(path, "00000\n00000\n");
    utimesSync(path, timestamp, timestamp);
    expect(client.usable()).toBe(true);
    rmSync(path);
    expect(other.state()).toBe("ok");
    appendFileSync(path, "89000\n89001\n");
    expect(client.state()).toBe("down");
    expect(other.usable()).toBe(false);
  } finally {
    clock.mockRestore();
    rmSync(directory, { recursive: true, force: true });
  }
});
