//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏷️ Hub build-freshness laws over a synthetic staged executable: the verdict for a missing executable, a missing or
// malformed sources record, untouched sources, a touched source and a vanished source. The oracle for "changed since the
// build started" is `find -newer <reference>` (BSD and GNU find) against a reference file stamped at the build start.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { checkHubBuildFreshness } from "../🏷️build-freshness/🟦️.ts";
import { HUB_BINARY_SOURCES_FILE } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
//#endregion 🔌️Adapters

const ABSENT_HUB = "http://127.0.0.1:9";

function stagedHub(): { dir: string; binary: string; sources: string[]; builtAtMs: number } {
  const dir = mkdtempSync(join(tmpdir(), "semio-hub-freshness-law-"));
  const binary = join(dir, "os-hub");
  writeFileSync(binary, "binary");
  const builtAtMs = Date.now() - 60_000;
  const sources = ["a.rs", "b.rs", "c.rs"].map((name) => join(dir, name));
  for (const source of sources) {
    writeFileSync(source, "fn main() {}\n");
    utimesSync(source, new Date(builtAtMs - 120_000), new Date(builtAtMs - 120_000));
  }
  writeFileSync(join(dir, HUB_BINARY_SOURCES_FILE), JSON.stringify({ schema: "semio.cargo.binary-sources/v1", builtAtMs, sources }));
  return { dir, binary, sources, builtAtMs };
}

function findNewer(dir: string, builtAtMs: number, sources: readonly string[]): string[] {
  const reference = join(dir, "reference");
  writeFileSync(reference, "");
  utimesSync(reference, new Date(builtAtMs), new Date(builtAtMs));
  const found = spawnSync("find", [...sources, "-newer", reference], { encoding: "utf8" });
  return found.stdout.split("\n").filter(Boolean).sort();
}

describe("hub build freshness", () => {
  test("a missing executable is unverifiable", async () => {
    const report = await checkHubBuildFreshness(ABSENT_HUB, join(tmpdir(), "no-such-os-hub-binary"));
    expect(report.verdict).toBe("unverifiable");
    expect(report.reason.de.length).toBeGreaterThan(0);
  });

  test("an executable without a sources record is unverifiable", async () => {
    const hub = stagedHub();
    try {
      rmSync(join(hub.dir, HUB_BINARY_SOURCES_FILE));
      expect((await checkHubBuildFreshness(ABSENT_HUB, hub.binary)).verdict).toBe("unverifiable");
    } finally {
      rmSync(hub.dir, { recursive: true, force: true });
    }
  });

  test("a malformed sources record is unverifiable", async () => {
    const hub = stagedHub();
    try {
      writeFileSync(join(hub.dir, HUB_BINARY_SOURCES_FILE), JSON.stringify({ schema: "semio.cargo.binary-sources/v1", builtAtMs: "yesterday", sources: [] }));
      expect((await checkHubBuildFreshness(ABSENT_HUB, hub.binary)).verdict).toBe("unverifiable");
    } finally {
      rmSync(hub.dir, { recursive: true, force: true });
    }
  });

  test("untouched sources are fresh, and find agrees nothing is newer", async () => {
    const hub = stagedHub();
    try {
      const report = await checkHubBuildFreshness(ABSENT_HUB, hub.binary);
      expect(report.verdict).toBe("fresh");
      expect(report.sources).toBe(3);
      expect(report.changed).toEqual([]);
      expect(findNewer(hub.dir, hub.builtAtMs, hub.sources)).toEqual([]);
    } finally {
      rmSync(hub.dir, { recursive: true, force: true });
    }
  });

  test("a source touched after the build started is stale, exactly as find reports", async () => {
    const hub = stagedHub();
    try {
      utimesSync(hub.sources[1]!, new Date(), new Date());
      const report = await checkHubBuildFreshness(ABSENT_HUB, hub.binary);
      expect(report.verdict).toBe("stale");
      expect([...report.changed].sort()).toEqual(findNewer(hub.dir, hub.builtAtMs, hub.sources));
    } finally {
      rmSync(hub.dir, { recursive: true, force: true });
    }
  });

  test("a vanished source is stale", async () => {
    const hub = stagedHub();
    try {
      rmSync(hub.sources[2]!);
      const report = await checkHubBuildFreshness(ABSENT_HUB, hub.binary);
      expect(report.verdict).toBe("stale");
      expect(report.changed).toEqual([hub.sources[2]!]);
    } finally {
      rmSync(hub.dir, { recursive: true, force: true });
    }
  });
});
