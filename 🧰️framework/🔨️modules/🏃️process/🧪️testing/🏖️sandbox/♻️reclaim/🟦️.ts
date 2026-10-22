import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { detachDiskImagesUnderV1 } from "../🖼️images/🟦️.ts";
import { readTestSandboxProtocolV1, testProcessAliveV1, testSandboxParentV1, type TestSandboxProtocolV1 } from "../📐️layout/🟦️.ts";

/** ♻️ Names the host facts a reclaim decision depends on, so the decision stays a pure function. */
export type TestSandboxFactsV1 = Readonly<{ pid?: number; heartbeatAgeMilliseconds: number; directoryAgeMilliseconds: number }>;

/** ⚖️ Decides whether a sandbox is abandoned: its owner process is gone, or it stopped beating for longer than the declared stale window; an unreadable owner row is only reclaimed after the grace period. */
export function abandonedTestSandboxV1(facts: TestSandboxFactsV1, alive: (pid: number) => boolean = testProcessAliveV1, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): boolean {
  if (facts.pid === undefined) return facts.directoryAgeMilliseconds > declared.layout.graceMilliseconds;
  return !alive(facts.pid) || facts.heartbeatAgeMilliseconds > declared.layout.staleHeartbeatMilliseconds;
}

/** ♻️ Lists the sandbox directories of this host with the facts a reclaim decision needs: the sandboxes of every agent session directory below the host-wide parent. */
export function listTestSandboxesV1(parent: string, now: number = Date.now(), declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): readonly Readonly<{ root: string; facts: TestSandboxFactsV1 }>[] {
  if (!existsSync(parent)) return [];
  return readdirSync(parent).filter(name => name !== declared.ports.directory && /^[a-z][a-z0-9]{0,11}$/u.test(name)).flatMap(name => listSessionSandboxesV1(join(parent, name), now, declared));
}

/** ♻️ Lists the sandbox directories below the root of one agent session. */
export function listSessionSandboxesV1(home: string, now: number = Date.now(), declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): readonly Readonly<{ root: string; facts: TestSandboxFactsV1 }>[] {
  if (!existsSync(home)) return [];
  return readdirSync(home).filter(name => /^[0-9a-f]{8}$/u.test(name)).flatMap(name => {
    const root = join(home, name);
    try {
      const status = statSync(root);
      if (!status.isDirectory()) return [];
      const owner = join(root, declared.layout.owner);
      let pid: number | undefined, beat = status.mtimeMs;
      try { pid = (JSON.parse(readFileSync(owner, "utf8")) as { pid?: number }).pid; beat = statSync(owner).mtimeMs; } catch {}
      return [{ root, facts: { pid, heartbeatAgeMilliseconds: now - beat, directoryAgeMilliseconds: now - status.mtimeMs } }];
    } catch { return []; }
  });
}

/** ♻️ Releases the sandboxes and the temp root of one agent session: abandoned sandboxes go (images detached first), a live one keeps the session root; when nothing is kept the whole root goes. */
export function reclaimAgentSessionSandboxesV1(session: Readonly<{ resources: Readonly<{ tests: Readonly<{ root: string }> }> }>, options: Readonly<{ parent?: string; now?: number; alive?: (pid: number) => boolean; detach?: (root: string) => readonly string[] }> = {}, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): Readonly<{ released: number; kept: number }> {
  const home = join(testSandboxParentV1(options.parent, declared), session.resources.tests.root), now = options.now ?? Date.now(), alive = options.alive ?? testProcessAliveV1, detach = options.detach ?? detachDiskImagesUnderV1;
  if (!existsSync(home)) return { released: 0, kept: 0 };
  let released = 0, kept = 0;
  for (const { root, facts } of listSessionSandboxesV1(home, now, declared)) {
    if (!abandonedTestSandboxV1(facts, alive, declared)) { kept++; continue; }
    try { detach(root); rmSync(root, { recursive: true, force: true }); released++; } catch { kept++; }
  }
  if (kept === 0) { try { detach(home); rmSync(home, { recursive: true, force: true }); released++; } catch { kept++; } }
  return { released, kept };
}

/** ♻️ Reclaims abandoned sandboxes and port leases: images under a dead sandbox are detached first, then the tree is removed; at most one sweep per declared interval unless forced. */
export function reclaimAbandonedTestSandboxesV1(options: Readonly<{ parent?: string; force?: boolean; now?: number; alive?: (pid: number) => boolean; detach?: (root: string) => readonly string[] }> = {}, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): Readonly<{ reclaimed: readonly string[]; kept: readonly string[]; throttled: boolean }> {
  const parent = testSandboxParentV1(options.parent, declared), now = options.now ?? Date.now(), stamp = join(parent, declared.layout.stamp);
  mkdirSync(parent, { recursive: true, mode: 0o700 });
  if (!options.force) { try { if (now - statSync(stamp).mtimeMs < declared.layout.reclaimIntervalMilliseconds) return { reclaimed: [], kept: [], throttled: true }; } catch {} }
  try { writeFileSync(stamp, String(now)); utimesSync(stamp, now / 1000, now / 1000); } catch {}
  const reclaimed: string[] = [], kept: string[] = [], alive = options.alive ?? testProcessAliveV1, detach = options.detach ?? detachDiskImagesUnderV1;
  for (const { root, facts } of listTestSandboxesV1(parent, now, declared)) {
    if (!abandonedTestSandboxV1(facts, alive, declared)) { kept.push(root); continue; }
    try { detach(root); rmSync(root, { recursive: true, force: true }); reclaimed.push(root); } catch { kept.push(root); }
  }
  const ports = join(parent, declared.ports.directory);
  if (existsSync(ports)) for (const name of readdirSync(ports)) {
    try {
      const row = JSON.parse(readFileSync(join(ports, name, declared.layout.owner), "utf8")) as { pid?: number };
      if (row.pid === undefined || !alive(row.pid)) rmSync(join(ports, name), { recursive: true, force: true });
    } catch { try { if (now - statSync(join(ports, name)).mtimeMs > declared.layout.graceMilliseconds) rmSync(join(ports, name), { recursive: true, force: true }); } catch {} }
  }
  return { reclaimed, kept, throttled: false };
}
