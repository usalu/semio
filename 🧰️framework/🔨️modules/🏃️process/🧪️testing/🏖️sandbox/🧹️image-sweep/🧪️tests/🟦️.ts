import { afterAll, describe, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { attachedDiskImagesV1 } from "../../🖼️images/🟦️.ts";
import { classifyAttachedImagesV1, describeImageSplitV1, processStartV1, readSandboxOwnerV1, recordSandboxImagesV1, sandboxOwnerAliveV1, sweepLeakedSandboxImagesV1, writeSandboxOwnerV1 } from "../🟦️.ts";

const hdiutil = (...args: string[]) => spawnSync("/usr/bin/hdiutil", args, { encoding: "utf8" });
const base = realpathSync(mkdtempSync(join(tmpdir(), "sw-")));
afterAll(() => { for (const row of attachedDiskImagesV1()) if (row.image.startsWith(base + "/")) hdiutil("detach", "-force", row.device || row.target); rmSync(base, { recursive: true, force: true }); });

function attachPair(name: string): { sandbox: string; outer: string; inner: string } {
  const sandbox = join(base, ".cache", "test-sandbox", name), outerDirectory = join(sandbox, ".cache", "b", "o"), outer = join(outerDirectory, "fixed.dmg"), mount = join(outerDirectory, "m");
  mkdirSync(mount, { recursive: true });
  expect(hdiutil("create", "-size", "8m", "-fs", "HFS+", "-volname", "swo", outer).status).toBe(0);
  expect(hdiutil("attach", "-nobrowse", "-mountpoint", mount, outer).status).toBe(0);
  const nestedDirectory = join(mount, "tmp", "gf-abc"), inner = join(nestedDirectory, "fixed.dmg");
  mkdirSync(join(nestedDirectory, "m"), { recursive: true });
  expect(hdiutil("create", "-size", "2m", "-fs", "HFS+", "-volname", "swi", inner).status).toBe(0);
  expect(hdiutil("attach", "-nobrowse", "-mountpoint", join(nestedDirectory, "m"), inner).status).toBe(0);
  return { sandbox, outer: realpathSync(outer), inner: realpathSync(inner) };
}
const listed = (image: string) => attachedDiskImagesV1().some(row => row.image === image);

describe("owner identity", () => {
  test("the schema fixes the owner row and a recycled pid is not the owner", () => {
    const root = mkdtempSync(join(base, "o-")), owner = writeSandboxOwnerV1(root);
    expect(owner.pid).toBe(process.pid);
    expect(readSandboxOwnerV1(root)).toEqual(owner);
    expect(sandboxOwnerAliveV1(owner)).toBe(true);
    expect(sandboxOwnerAliveV1({ ...owner, start: "Mon Jan  1 00:00:00 2001" })).toBe(false);
    writeFileSync(join(root, ".owner"), String(process.pid));
    expect(readSandboxOwnerV1(root)).toEqual({ version: 1, pid: process.pid, start: null });
    expect(Object.keys(schema.definitions.Owner.properties).sort()).toEqual(Object.keys(owner).sort());
  });
  test("the start time of a process agrees with the system tool", () => {
    const independent = spawnSync("/bin/ps", ["-o", "lstart=", "-p", String(process.pid)], { encoding: "utf8" }).stdout.trim();
    expect(processStartV1(process.pid)).toBe(independent);
    expect(processStartV1(2 ** 22 + 12345)).toBeNull();
  });
});

describe("classification and messages", () => {
  const rows = [
    { image: "/r/.cache/b/o/fixed.dmg", target: "/a", device: "/dev/disk1" },
    { image: "/r/.cache/test-sandbox/rAAAAAA/.cache/b/o/fixed.dmg", target: "/b", device: "/dev/disk2" },
    { image: "/r/.cache/test-sandbox/rBBBBBB/.cache/b/l/fixed.dmg", target: "/c", device: "/dev/disk3" },
    { image: "/x/scratch/fixed.dmg", target: "/d", device: "/dev/disk4" },
  ];
  test("ours, leaked and foreign are told apart and worded in both languages", () => {
    const got = classifyAttachedImagesV1(rows, root => root.endsWith("rAAAAAA"));
    expect(got.map(row => row.kind)).toEqual(["ours", "ours", "leaked", "foreign"]);
    const split = { total: 4, ours: 2, leaked: 1, foreign: 1 };
    expect(describeImageSplitV1("en", split)).toBe("4 disk images are attached: 2 are ours, 1 belong to dead test sandboxes and 1 are foreign");
    expect(describeImageSplitV1("de", split)).toContain("1 gehören toten Test-Sandboxes");
  });
});

describe.skipIf(process.platform !== "darwin")("real attached images", () => {
  test("a sandbox whose owner process was killed leaves nested images that the sweep detaches nested first and removes the sandbox of, while a live sandbox stays", async () => {
    const dead = attachPair("rDead01"), live = attachPair("rLive01");
    const child = spawn("/bin/sleep", ["300"], { stdio: "ignore" });
    await new Promise<void>(done => child.once("spawn", () => done()));
    writeSandboxOwnerV1(dead.sandbox, child.pid!);
    writeSandboxOwnerV1(live.sandbox);
    expect(recordSandboxImagesV1(dead.sandbox)).toBe(2);
    expect(recordSandboxImagesV1(dead.sandbox)).toBe(0);
    const rows = readFileSync(join(dead.sandbox, ".images.jsonl"), "utf8").trim().split("\n").map(line => JSON.parse(line) as { device: string; image: string; pid: number; start: string | null });
    expect(rows.map(row => row.image).sort()).toEqual([dead.inner, dead.outer].sort());
    expect(rows.every(row => /^\/dev\/disk[0-9]+$/u.test(row.device) && row.pid === process.pid && row.start === processStartV1(process.pid))).toBe(true);
    child.kill("SIGKILL");
    await new Promise<void>(done => child.once("close", () => done()));
    const report = sweepLeakedSandboxImagesV1({ parents: [join(base, ".cache", "test-sandbox")], under: base });
    expect(report).toMatchObject({ leaked: 2, detached: 2, failed: 0, sandboxesRemoved: 1, sandboxesKept: 1 });
    expect(listed(dead.inner) || listed(dead.outer)).toBe(false);
    expect(existsSync(dead.sandbox)).toBe(false);
    expect(listed(live.outer) && listed(live.inner)).toBe(true);
    expect(existsSync(live.sandbox)).toBe(true);
    expect(sweepLeakedSandboxImagesV1({ parents: [join(base, ".cache", "test-sandbox")], under: base })).toMatchObject({ leaked: 0, detached: 0 });
  }, 120000);
});
