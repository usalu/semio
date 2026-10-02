import { spawnSync } from "node:child_process";

/** 🌳️ Captures the first-party ownership state needed to terminate a live child. */
export interface OwnedChildHandle { readonly pid?: number; readonly exitCode: number | null; readonly signalCode: string | null }

interface OwnedProcessRow {
  pid: number;
  parent: number;
  group: number;
  depth: number;
}

/** 🪓️Terminates one owned subprocess tree, including descendant-created POSIX groups, before ancestry is lost. */
export function terminateOwnedProcessTree(rootPid: number): void {
  if (!Number.isSafeInteger(rootPid) || rootPid <= 0) return;
  if (process.platform === "win32") {
    spawnSync("taskkill", ["/pid", String(rootPid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
    return;
  }
  const snapshot = spawnSync("ps", ["-axo", "pid=,ppid=,pgid="], { encoding: "utf8", windowsHide: true });
  if (snapshot.status !== 0 || typeof snapshot.stdout !== "string") {
    try { process.kill(-rootPid, "SIGKILL"); } catch {}
    try { process.kill(rootPid, "SIGKILL"); } catch {}
    return;
  }
  const rows = snapshot.stdout.split("\n").flatMap((line): Omit<OwnedProcessRow, "depth">[] => {
    const fields = line.trim().split(/\s+/).map(Number);
    return fields.length === 3 && fields.every((field) => Number.isSafeInteger(field) && field > 0)
      ? [{ pid: fields[0]!, parent: fields[1]!, group: fields[2]! }]
      : [];
  });
  const depth = new Map<number, number>([[rootPid, 0]]);
  for (let changed = true; changed;) {
    changed = false;
    for (const row of rows) {
      if (depth.has(row.pid)) continue;
      const parentDepth = depth.get(row.parent);
      if (parentDepth === undefined) continue;
      depth.set(row.pid, parentDepth + 1);
      changed = true;
    }
  }
  const owned: OwnedProcessRow[] = rows.filter((row) => depth.has(row.pid)).map((row) => ({ ...row, depth: depth.get(row.pid)! }));
  if (!owned.some((row) => row.pid === rootPid)) owned.push({ pid: rootPid, parent: 0, group: rootPid, depth: 0 });
  const ownedPids = new Set(owned.map((row) => row.pid));
  const groups = new Map<number, number>();
  for (const row of owned) {
    if (ownedPids.has(row.group)) groups.set(row.group, Math.max(groups.get(row.group) ?? 0, row.depth));
  }
  for (const [group] of [...groups].sort((left, right) => right[1] - left[1])) {
    try { process.kill(-group, "SIGKILL"); } catch {}
  }
  for (const row of owned.sort((left, right) => right.depth - left.depth)) {
    try { process.kill(row.pid, "SIGKILL"); } catch {}
  }
}

/** 🪓️ Terminates one still-running spawned child with its whole descendant tree — POSIX groups and `taskkill /T`
 * on Windows through [[terminateOwnedProcessTree]] — and never signals a pid its exited child no longer owns. */
export function terminateOwnedChildTree(child: OwnedChildHandle): void {
  if (child.exitCode === null && child.signalCode === null && child.pid) terminateOwnedProcessTree(child.pid);
}

