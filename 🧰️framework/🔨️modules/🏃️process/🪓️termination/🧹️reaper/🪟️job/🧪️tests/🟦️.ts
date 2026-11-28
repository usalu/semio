import { describe, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { createJobTableV1, JOB_OBJECT_V1, windowsJobKernelV1, type JobKernelV1 } from "../🟦️.ts";

/** 🪟️ Job law: every registered child has its own kill-on-close job held by the sentinel; stop and parent death end the whole tree, release of a finished child kills nothing, a failing step leaves no job behind. The table runs here against a recorder of the kernel calls; the real kernel call set runs on Windows only.
 * @see ../../🧬️schema/🔣️.json */
const recorder = (options: Readonly<{ create?: boolean; arm?: boolean; assign?: boolean }> = {}): JobKernelV1 & { readonly calls: string[] } => {
  const calls: string[] = [];
  let next = 0;
  return {
    calls,
    create: () => { if (options.create === false) return undefined; calls.push("create"); return ++next; },
    arm: (job, kill) => { calls.push(`arm:${job}:${kill}`); return options.arm !== false; },
    assign: (job, pid) => { calls.push(`assign:${job}:${pid}`); return options.assign !== false; },
    terminate: (job) => { calls.push(`terminate:${job}`); },
    close: (job) => { calls.push(`close:${job}`); },
  };
};

describe("the job table", () => {
  test("add arms kill-on-close before it assigns; stop terminates then closes once; a second stop does nothing", () => {
    const kernel = recorder(), table = createJobTableV1(kernel);
    expect(table.add(100)).toBe(true);
    expect(table.add(100), "adding again keeps the one job").toBe(true);
    expect(kernel.calls).toEqual(["create", "arm:1:true", "assign:1:100"]);
    expect(table.stop(100)).toBe(true);
    expect(table.stop(100)).toBe(false);
    expect(kernel.calls.slice(3)).toEqual(["terminate:1", "close:1"]);
    expect(table.size()).toBe(0);
  });

  test("release of a child that ended disarms the job and closes it, killing nothing", () => {
    const kernel = recorder(), table = createJobTableV1(kernel);
    table.add(7);
    table.release(7);
    expect(kernel.calls.slice(3)).toEqual(["arm:1:false", "close:1"]);
    expect(kernel.calls.some((call) => call.startsWith("terminate"))).toBe(false);
  });

  test("stopAll ends every job (the parent is gone) and counts them", () => {
    const kernel = recorder(), table = createJobTableV1(kernel);
    for (const pid of [1, 2, 3]) table.add(pid);
    expect(table.stopAll()).toBe(3);
    expect(kernel.calls.filter((call) => call.startsWith("terminate"))).toEqual(["terminate:1", "terminate:2", "terminate:3"]);
    expect(table.size()).toBe(0);
  });

  test("a step that fails leaves no job open and reports false", () => {
    for (const failing of [{ arm: false }, { assign: false }]) {
      const kernel = recorder(failing), table = createJobTableV1(kernel);
      expect(table.add(5)).toBe(false);
      expect(kernel.calls.at(-1)).toBe("close:1");
      expect(table.size()).toBe(0);
    }
    const none = recorder({ create: false }), table = createJobTableV1(none);
    expect(table.add(5)).toBe(false);
    expect(none.calls).toEqual([]);
  });

  test("the constants are the documented ones: class 9, 144 bytes, flags at 16, kill-on-close 0x2000, access 0x101", () => {
    expect(JOB_OBJECT_V1).toMatchObject({ extendedLimitInformationClass: 9, extendedLimitInformationBytes: 144, limitFlagsOffset: 16, killOnJobClose: 0x2000, processSetQuota: 0x100, processTerminate: 1 });
  });

  test("off Windows there is no kernel", async () => {
    if (process.platform !== "win32") expect(await windowsJobKernelV1()).toBeUndefined();
  });
});

describe.skipIf(process.platform !== "win32")("the real kernel on Windows", () => {
  test("a stopped job ends a child and the grandchild whose parent link is gone", async () => {
    const kernel = (await windowsJobKernelV1())!;
    expect(kernel).toBeDefined();
    const table = createJobTableV1(kernel);
    const child = spawn("cmd.exe", ["/c", "start /b ping -n 300 127.0.0.1 >nul & ping -n 300 127.0.0.1 >nul"], { stdio: "ignore", windowsHide: true });
    await Bun.sleep(800);
    expect(table.add(child.pid!)).toBe(true);
    const listing = (): string => spawnSync("tasklist", ["/FI", "IMAGENAME eq PING.EXE"], { encoding: "utf8" }).stdout;
    expect(listing()).toContain("PING.EXE");
    table.stop(child.pid!);
    await Bun.sleep(1500);
    expect(listing()).not.toContain("PING.EXE");
  }, 20000);
});
