import { afterAll, describe, expect, test } from "bun:test";
import { spawnSync, type ChildProcess } from "node:child_process";
import { createConnection } from "node:net";
import { startAdmissionBrokerV1, type AdmitRequestV1 } from "../../🟦️.ts";
import { spawnSharedChildV1 } from "../../../../../🪓️termination/🤝️shared-child/🟦️.ts";
import requests from "../../🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { createSharedBuildsV1 } from "../🟦️.ts";

/** 🤝️ Shared build law: requesters of one build share one cargo; one that leaves never stops it while another waits; the last one out ends cargo and rustc, recorded with identity; the outcome reaches everyone left. Two requesters are also admitted by a real broker: a leaver's broker entry goes with it, the other's stays.
 * @see ../🧬️schema/🔣️.json */
const alive = (pid: number): boolean => {
  try { process.kill(pid, 0); } catch { return false; }
  const state = spawnSync("ps", ["-o", "stat=", "-p", String(pid)], { encoding: "utf8" });
  return state.status === 0 && !state.stdout.trim().startsWith("Z");
};
const below = (pid: number): number[] => spawnSync("pgrep", ["-P", String(pid)], { encoding: "utf8" }).stdout.split("\n").filter(Boolean).map(Number);
const until = async (predicate: () => boolean, milliseconds: number): Promise<boolean> => {
  const deadline = Date.now() + milliseconds;
  while (!predicate()) { if (Date.now() > deadline) return false; await Bun.sleep(40); }
  return true;
};
const children: ChildProcess[] = [];
afterAll(() => { for (const child of children) try { process.kill(-child.pid!, "SIGKILL"); } catch {} });

const artifact = '{"reason":"compiler-artifact","executable":"/x/scheduler","target":{"name":"semio-build-scheduler"}}';
const cargoLike = (script: string) => (): ChildProcess => { const child = spawnSharedChildV1("sh", ["-c", script], { stdio: ["ignore", "pipe", "pipe"] }); children.push(child); return child; };
const read = (child: ChildProcess): Promise<Readonly<{ status: number; lines: string[] }>> => new Promise((accept) => {
  const lines: string[] = [];
  child.stdout!.on("data", (bytes: Buffer) => lines.push(...bytes.toString().split("\n").filter(Boolean)));
  child.once("close", (code) => accept({ status: code ?? 1, lines }));
});

describe.skipIf(process.platform === "win32")("two requesters of one build", () => {
  test("the first to leave refuses only itself, the last to leave ends cargo and rustc and records the stop", async () => {
    const records: string[] = [], builds = createSharedBuildsV1({ write: (line) => records.push(line) });
    let started = 0;
    const start = (): ChildProcess => { started++; return cargoLike(`echo '${artifact}'; sleep 300 & wait`)(); };
    const first = new AbortController(), second = new AbortController();
    const one = builds.run({ key: "k", requester: "one", signal: first.signal, start, collect: read });
    const two = builds.run({ key: "k", requester: "two", signal: second.signal, start, collect: read });
    expect(started).toBe(1);
    expect(builds.requesters("k")).toEqual(["one", "two"]);
    const pid = [...children].at(-1)!.pid!;
    expect(await until(() => below(pid).length === 1, 5000)).toBe(true);
    const rustc = below(pid)[0]!;
    first.abort(new Error("one left"));
    await expect(one).rejects.toThrow("one left");
    await Bun.sleep(500);
    expect([alive(pid), alive(rustc), builds.requesters("k")]).toEqual([true, true, ["two"]]);
    expect(records).toEqual([]);
    second.abort(new Error("two left"));
    await expect(two).rejects.toThrow("two left");
    expect(await until(() => !alive(pid) && !alive(rustc), 15000)).toBe(true);
    expect(builds.size()).toBe(0);
    expect(records.length).toBeGreaterThan(0);
    expect(records[0]).toContain("shared cargo build");
    expect(records[0]).toContain("the last requester two left the shared child k / der letzte Anforderer two hat das gemeinsame Kind k verlassen");
    expect(records[0]).toMatch(/Victim created at \d+, sender created at \d+/);
  }, 30000);

  test("the outcome of one build reaches every requester that waits, and a later requester starts a fresh build", async () => {
    const builds = createSharedBuildsV1({ write: () => {} });
    let started = 0;
    const start = (): ChildProcess => { started++; return cargoLike(`echo '${artifact}'; sleep 1`)(); };
    const [a, b] = await Promise.all([1, 2].map((id) => builds.run({ key: "same", requester: `r${id}`, start, collect: read })));
    expect(started).toBe(1);
    expect(a).toBe(b);
    expect(a.status).toBe(0);
    expect(a.lines).toEqual([artifact]);
    expect(builds.size()).toBe(0);
    await builds.run({ key: "same", requester: "r3", start, collect: read });
    expect(started).toBe(2);
  }, 30000);

  test("a requester whose signal is already aborted never starts a child; different keys are different builds", async () => {
    const builds = createSharedBuildsV1({ write: () => {} });
    let started = 0;
    const start = (): ChildProcess => { started++; return cargoLike("sleep 1")(); };
    const gone = AbortSignal.abort(new Error("gone"));
    await expect(builds.run({ key: "x", requester: "a", signal: gone, start, collect: read })).rejects.toThrow("gone");
    await Promise.all([builds.run({ key: "p", requester: "a", start, collect: read }), builds.run({ key: "q", requester: "b", start, collect: read })]);
    expect(started).toBe(3);
  }, 30000);
});

describe.skipIf(process.platform === "win32")("with a real admission broker", () => {
  const MiB = 1 << 20, base = requests.requests.valid[0]! as unknown as AdmitRequestV1;
  const join = (endpoint: string, crate: string): Promise<{ socket: ReturnType<typeof createConnection> }> => new Promise((accept, reject) => {
    const socket = createConnection(endpoint);
    socket.on("error", () => undefined);
    socket.once("data", () => accept({ socket }));
    socket.once("connect", () => socket.write(JSON.stringify({ ...base, op: "admit", crate, package: crate, crateTypes: ["lib"], target: "host", externs: [], pid: process.pid }) + "\n"));
    setTimeout(() => reject(new Error("the broker never answered")), 5000).unref?.();
  });

  test("two requesters are admitted, one leaves (its broker entry goes, the build stays), the other leaves (nothing is left anywhere)", async () => {
    const broker = await startAdmissionBrokerV1({ physicalBytes: 16 * 1024 * MiB, history: new Map(), available: () => 8 * 1024 * MiB, warden: () => "calm", cancels: () => [] });
    broker.registerPass({ id: "wasm-release-00", profile: "release", target: "host" });
    const builds = createSharedBuildsV1({ write: () => {} });
    try {
      const a = await join(broker.endpoint, "first"), b = await join(broker.endpoint, "second");
      const held = (): number => broker.stats().running + broker.stats().waiting;
      expect(await until(() => held() === 2, 5000)).toBe(true);
      const start = cargoLike(`echo '${artifact}'; sleep 300 & wait`);
      const left = { a: new AbortController(), b: new AbortController() };
      const runA = builds.run({ key: "scheduler", requester: "a", signal: left.a.signal, start, collect: read }), runB = builds.run({ key: "scheduler", requester: "b", signal: left.b.signal, start, collect: read });
      const pid = children.at(-1)!.pid!;
      expect(await until(() => below(pid).length === 1, 5000)).toBe(true);
      const rustc = below(pid)[0]!;
      a.socket.destroy();
      left.a.abort(new Error("a left"));
      await expect(runA).rejects.toThrow("a left");
      expect(await until(() => held() === 1, 5000)).toBe(true);
      expect([alive(pid), alive(rustc)]).toEqual([true, true]);
      b.socket.destroy();
      left.b.abort(new Error("b left"));
      await expect(runB).rejects.toThrow("b left");
      expect(await until(() => held() === 0 && !alive(pid) && !alive(rustc), 15000)).toBe(true);
    } finally { await broker.close(); }
  }, 40000);
});
