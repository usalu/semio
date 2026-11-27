import { afterEach, expect, test } from "bun:test";
import { spawn } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { liveAgentBridgeOfferFor } from "../🟦️.ts";

const roots: string[] = [];
afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

const offer = (pid: number, publishedAtMs: number) => ({ schemaVersion: 2, url: `ws://127.0.0.1:${50000 + pid % 1000}/bridge`, admissionProof: `proof-${pid}`, principal: "agent:local", pid, publishedAtMs, scope: { kind: "local" } });

const rendezvous = (files: Record<string, unknown>): string => {
  const root = mkdtempSync(join(tmpdir(), "semio-offers-"));
  roots.push(root);
  mkdirSync(join(root, "offers"));
  for (const [name, body] of Object.entries(files)) writeFileSync(join(root, "offers", name), typeof body === "string" ? body : JSON.stringify(body));
  return root;
};

const deadPid = async (): Promise<number> => {
  const child = spawn(process.execPath, ["--version"], { stdio: "ignore" });
  const pid = child.pid!;
  await new Promise((done) => child.once("exit", done));
  return pid;
};

test("a shell is only ever offered a gateway whose process is alive, never the newest dead one", async () => {
  const dead = await deadPid();
  const root = rendezvous({ [`${dead}.json`]: offer(dead, 9000), [`${process.pid}.json`]: offer(process.pid, 1000) });
  expect(liveAgentBridgeOfferFor(null, root)?.pid).toBe(process.pid);
});

test("only dead, malformed, foreign-version or non-json offer files mean no offer at all", async () => {
  const dead = await deadPid();
  const root = rendezvous({ [`${dead}.json`]: offer(dead, 1), "garbage.json": "{not json", "old.json": { ...offer(process.pid, 2), schemaVersion: 1 }, "note.txt": JSON.stringify(offer(process.pid, 3)) });
  expect(liveAgentBridgeOfferFor(null, root)).toBeNull();
});

test("a missing rendezvous directory is the ordinary no-offer answer", () => {
  expect(liveAgentBridgeOfferFor(null, join(tmpdir(), "semio-offers-absent-xyz"))).toBeNull();
});
