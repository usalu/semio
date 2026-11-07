import driveVectors from "./drive.vectors.json" with { type: "json" };
import { existsSync } from "node:fs";
import { nativeTemporarySystemPortV1, nativeTemporaryWindowsDosDeviceV1 } from "../🟦️.ts";
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { withNativeTemporaryDirectoryAsyncV1, type NativeTemporaryPortV1 } from "../🟦️.ts";

test("temporary alias remains held until async work settles", async () => {
  const events: string[] = [], db = new Database(":memory:");
  const port: NativeTemporaryPortV1 = { realpath: path => path, sameDirectory: () => true, junction: () => ({ path: "C:\\tmp\\alias", close: () => { events.push("closed"); } }), drive: () => undefined };
  try {
    await withNativeTemporaryDirectoryAsyncV1({ target: "C:\\ä", candidates: ["C:\\tmp"] }, port, async () => {
      events.push("started");
      await Promise.resolve();
      events.push("settled");
    });
    const expected = db.query("select 'started,settled,closed' as value").get() as { value: string };
    expect(events.join(",")).toBe(expected.value);
    events.length = 0;
    await expect(withNativeTemporaryDirectoryAsyncV1({ target: "C:\\ä", candidates: ["C:\\tmp"] }, port, async () => { await Promise.resolve(); throw Error("cancelled"); })).rejects.toThrow("cancelled");
    expect(events.join(",")).toBe((db.query("select 'closed' as value").get() as { value: string }).value);
  } finally { db.close(); }
});


test("Windows DOS device lease maps and releases only its owned directory", () => {
  if (process.platform !== "win32") return;
  const port = nativeTemporarySystemPortV1(nativeTemporaryWindowsDosDeviceV1());
  const lease = port.drive(import.meta.dir);
  expect(lease).toBeDefined();
  expect(port.sameDirectory(lease!.path, import.meta.dir)).toBe(true);
  lease!.close();
  expect(existsSync(lease!.path)).toBe(false);
});


test("existing dangling mapping is refused and preserved", () => {
  if (process.platform !== "win32") return;
  const acquire = nativeTemporaryWindowsDosDeviceV1(), original = acquire("C:\\semio-nonexistent-drive-test-9e2d");
  expect(original).toBeDefined();
  try {
    const device = original!.path.slice(0, 2);
    expect(nativeTemporaryWindowsDosDeviceV1([device])(process.cwd())).toBeUndefined();
    expect(existsSync(original!.path)).toBe(false);
  } finally { original!.close(); }
});

test("pre-cancelled requests perform no port effects", async () => {
  const abort = new AbortController(); abort.abort();
  let effects = 0;
  const port: NativeTemporaryPortV1 = { realpath: path => { effects++; return path; }, sameDirectory: () => true, junction: () => undefined, drive: () => undefined };
  await expect(withNativeTemporaryDirectoryAsyncV1({ target: "C:\\ä", candidates: [], signal: abort.signal }, port, async () => {})).rejects.toThrow();
  expect(effects).toBe(0);
});


test("native drive survives a real child process and is removed afterward", async () => {
  if (process.platform !== "win32") return;
  const port = nativeTemporarySystemPortV1(nativeTemporaryWindowsDosDeviceV1()), lease = port.drive(import.meta.dir);
  expect(lease).toBeDefined();
  try {
    const repo = Bun.which("repo");
    expect(repo).not.toBeNull();
    const child = Bun.spawn([repo!, "tools", "exec", "bun", "--", "-e", "if (!require('node:fs').statSync(process.argv[1]).isDirectory()) process.exit(1)", lease!.path], { stdout: "pipe", stderr: "pipe" });
    const [code, stderr] = await Promise.all([child.exited, new Response(child.stderr).text()]);
    if (code !== 0) throw Error(`Child exited ${code}: ${stderr}`);
    expect(code).toBe(0);
    expect(port.sameDirectory(lease!.path, import.meta.dir)).toBe(true);
  } finally { lease!.close(); }
  expect(existsSync(lease!.path)).toBe(false);
});


test("drive ownership vectors match SQLite oracle", () => {
  if (process.platform !== "win32") return;
  const db = new Database(":memory:");
  try {
    for (const vector of driveVectors) {
      const held = nativeTemporaryWindowsDosDeviceV1()("C:\\semio-nonexistent-vector-714a");
      expect(held).toBeDefined();
      const device = held!.path.slice(0, 2);
      if (!vector.occupied) held!.close();
      let candidate: ReturnType<ReturnType<typeof nativeTemporaryWindowsDosDeviceV1>> = undefined;
      try {
        candidate = nativeTemporaryWindowsDosDeviceV1([device])(process.cwd());
        const oracle = db.query("select not ? as allowed").get(Number(vector.occupied)) as { allowed: number };
        expect(Boolean(candidate)).toBe(vector.expected);
        expect(Boolean(candidate)).toBe(Boolean(oracle.allowed));
      } finally { candidate?.close(); if (vector.occupied) held!.close(); }
    }
  } finally { db.close(); }
});
