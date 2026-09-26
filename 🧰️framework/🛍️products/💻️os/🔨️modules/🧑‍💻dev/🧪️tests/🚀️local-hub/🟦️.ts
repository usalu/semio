/** @emoji 🚀️ Laws for the development hub path over `🧫️fixtures/🚀️local-hub.json` (ticket 26/09/23 U5, coordinator rule 23):
 * a named hub is only joined — against a real fake hub that binds late, with `detect-port` as the oracle that nothing else
 * ever bound its port —; the default hub is started once behind an owner lease that racing PROCESSES claim exactly once; a
 * catalog the current hub cannot load is republished (with progress and cancel) instead of booted; strict Ajv over the owned
 * schema `🧬️schema/🔣️.json` classifies every lease record and catalog header the way the product does; and the catalog
 * contract matches the publisher and loader it mirrors. No hub binary is built or run here. */

import { spawn } from "node:child_process";
import { createServer } from "node:http";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { detectPort } from "detect-port";
import { describe, expect, it } from "vitest";
import {
  claimDevHubLeaseV1,
  devHubCatalogFreshnessV1,
  devHubLeasePathsV1,
  devHubLocaleV1,
  devHubRoleV1,
  devHubStatusTextV1,
  devHubWorldV1,
  ensureCurrentTrustedCatalogV1,
  ensureDevLocalHub,
  joinDevHubV1,
  ownDevHubV1,
  readDevHubLeaseV1,
  releaseDevHubLeaseV1,
  type DevHubCatalogPublisherV1,
  type DevHubStatusV1,
  type DevHubWorldV1,
} from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import fixture from "../../🧫️fixtures/🚀️local-hub.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };

const MODULE = fileURLToPath(new URL("../../🚀️local-hub/🏃️execution/🟦️.ts", import.meta.url));
const REPO = fileURLToPath(new URL("../../../../../../../", import.meta.url));

function contract(name: string): (value: unknown) => boolean {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${name}`);
  if (!validate) throw new Error(`no $defs/${name}`);
  return validate as (value: unknown) => boolean;
}

function scratch(name: string): string {
  return mkdtempSync(join(tmpdir(), `u5-dev-hub-${name}-`));
}

function writeCatalog(dataDir: string, pointer: unknown, header: unknown): void {
  if (pointer === null) return;
  mkdirSync(join(dataDir, "trusted-catalog"), { recursive: true });
  writeFileSync(join(dataDir, "trusted-catalog", "current.json"), JSON.stringify(pointer));
  const generationId = (pointer as { generationId?: string }).generationId ?? "";
  if (header === null || !/^[0-9a-f]{64}$/u.test(generationId)) return;
  mkdirSync(join(dataDir, "trusted-catalog", "generations", generationId), { recursive: true });
  writeFileSync(join(dataDir, "trusted-catalog", "generations", generationId, "trusted-catalog.json"), JSON.stringify(header));
}

/** 🧫️ A world whose clock and sleep are real but short, whose reports are recorded, and whose owner spawn is a double. */
function recordingWorld(overrides: Partial<DevHubWorldV1> = {}): DevHubWorldV1 & { readonly statuses: DevHubStatusV1[] } {
  const statuses: DevHubStatusV1[] = [];
  return {
    ...devHubWorldV1(REPO, "en"),
    sleep: (ms) => new Promise<void>((resolveDelay) => setTimeout(resolveDelay, Math.min(ms, 50))),
    report: (status) => void statuses.push(status),
    spawnOwner: () => {
      throw new Error("the join path must never start a hub");
    },
    ...overrides,
    statuses,
  };
}

describe("development hub roles, language and status lines", () => {
  it("joins every explicitly named hub and owns only the default one", () => {
    for (const row of fixture.roles) expect(devHubRoleV1(row.explicit), JSON.stringify(row.explicit)).toBe(row.role);
  });

  it("speaks the terminal's language from the POSIX locale variables", () => {
    for (const row of fixture.locales) expect(devHubLocaleV1(row.env as Record<string, string>), JSON.stringify(row.env)).toBe(row.locale);
  });

  it("says every status in English and German exactly as the fixture does", () => {
    const kinds = new Set(fixture.statuses.map((row) => row.status.kind));
    expect(kinds.size).toBe(12);
    for (const row of fixture.statuses) {
      expect(devHubStatusTextV1(row.status as DevHubStatusV1, "en")).toBe(row.en);
      expect(devHubStatusTextV1(row.status as DevHubStatusV1, "de")).toBe(row.de);
    }
  });
});

describe("joining a named hub", () => {
  for (const row of fixture.joins) {
    it(`${row.name} — never starting one`, async () => {
      const port = await detectPort(0);
      const hubUrl = `http://127.0.0.1:${port}`;
      const server = createServer((request, response) => {
        response.statusCode = request.url === "/auth/sessions/me" ? 401 : 404;
        response.end();
      });
      const timer = row.bindsAfterMs === null ? null : setTimeout(() => server.listen(port, "127.0.0.1"), row.bindsAfterMs);
      try {
        const world = recordingWorld();
        const outcome = await joinDevHubV1(hubUrl, world, row.boundMs, 300);
        expect(outcome.kind).toBe(row.expected);
        expect(world.statuses.at(-1)?.kind).toBe(row.expected);
        if (row.expected === "gave-up") {
          expect(world.statuses.filter((status) => status.kind === "waiting").length, "the wait is never silent").toBeGreaterThanOrEqual(2);
          expect(await detectPort(port), "nothing bound the named hub's port while the serve waited").toBe(port);
        }
      } finally {
        if (timer !== null) clearTimeout(timer);
        await new Promise<void>((resolveClosed) => (server.listening ? server.close(() => resolveClosed()) : resolveClosed()));
      }
    }, 30_000);
  }

  it("a serve given an explicit hub joins or continues local-first and never spawns an owner", async () => {
    const saved = process.env.S_HUB_URL;
    const port = await detectPort(0);
    try {
      const world = recordingWorld();
      const unreachable = await ensureDevLocalHub(REPO, { hubUrl: `http://127.0.0.1:${port}`, dataDir: scratch("explicit"), world, joinBoundMs: 800 });
      expect(unreachable).toBeNull();
      expect(world.statuses.map((status) => status.kind)).toContain("gave-up");
      const server = createServer((_request, response) => {
        response.statusCode = 401;
        response.end();
      }).listen(port, "127.0.0.1");
      try {
        const joined = await ensureDevLocalHub(REPO, { hubUrl: `http://127.0.0.1:${port}/`, dataDir: scratch("explicit-up"), world, joinBoundMs: 5_000 });
        expect(joined).toMatchObject({ hubUrl: `http://127.0.0.1:${port}`, userId: "" });
        expect(world.statuses.at(-1)?.kind, "a joined hub without a broker says to sign in through the shell").toBe("no-broker");
      } finally {
        await new Promise<void>((resolveClosed) => server.close(() => resolveClosed()));
      }
    } finally {
      if (saved === undefined) delete process.env.S_HUB_URL;
      else process.env.S_HUB_URL = saved;
    }
  }, 30_000);
});

describe("owning the default hub", () => {
  const OWNER = 7_000_001;
  const STALE = 7_000_002;
  for (const row of fixture.owns) {
    it(row.name, async () => {
      const leaseRoot = scratch("leases");
      const dataDir = scratch("data");
      const hubUrl = "http://127.0.0.1:8787";
      const paths = devHubLeasePathsV1(leaseRoot, 8787, dataDir);
      const lease = (pid: number) => ({ pid, port: 8787, dataDir, hubUrl, acquiredAt: 1 });
      if (row.lease === "live") writeFileSync(paths[0], JSON.stringify(lease(OWNER)));
      if (row.lease === "stale") writeFileSync(paths[0], JSON.stringify(lease(STALE)));
      let polls = 0;
      let spawned = 0;
      const world = recordingWorld({
        ready: async () => row.readyAfterPolls !== null && polls++ >= row.readyAfterPolls,
        portInUse: () => row.portInUse,
        alive: (pid) => pid === OWNER,
        spawnOwner: () => {
          spawned += 1;
          writeFileSync(paths[0], JSON.stringify(lease(OWNER)));
          return OWNER;
        },
      });
      const up = await ownDevHubV1(hubUrl, dataDir, leaseRoot, world);
      expect({ spawned, statuses: world.statuses.map((status) => status.kind), up }).toEqual(row.expected);
    });
  }

  it("racing owner processes claim one hub port and data root exactly once, also over a stale claim, and every claim validates against DevHubLeaseV1", async () => {
    const valid = contract("DevHubLeaseV1");
    for (const record of fixture.leases.valid) {
      expect(valid(record), JSON.stringify(record)).toBe(true);
      expect(readDevHubLeaseV1Json(record), JSON.stringify(record)).not.toBeNull();
    }
    for (const record of fixture.leases.invalid) {
      expect(valid(record), JSON.stringify(record)).toBe(false);
      expect(readDevHubLeaseV1Json(record), JSON.stringify(record)).toBeNull();
    }
    for (const seed of ["empty", "stale"] as const) {
      const leaseRoot = scratch(`race-${seed}`);
      const dataDir = scratch(`race-data-${seed}`);
      const paths = devHubLeasePathsV1(leaseRoot, 8787, dataDir);
      if (seed === "stale") {
        mkdirSync(leaseRoot, { recursive: true });
        writeFileSync(paths[0], JSON.stringify({ pid: await deadPid(), port: 8787, dataDir, hubUrl: "http://127.0.0.1:8787", acquiredAt: 1 }));
      }
      const go = join(leaseRoot, "go");
      const racers = Array.from({ length: 6 }, () => raceClaim(leaseRoot, dataDir, go));
      await Promise.all(racers.map((racer) => racer.ready));
      writeFileSync(go, "");
      const outcomes = await Promise.all(racers.map((racer) => racer.done));
      expect(outcomes.filter((kind) => kind === "claimed"), `${seed}: exactly one owner`).toHaveLength(1);
      expect(outcomes.filter((kind) => kind === "held")).toHaveLength(5);
      const holder = readDevHubLeaseV1(paths[0]);
      expect(holder === null ? false : valid(holder), `${seed}: the surviving claim is a DevHubLeaseV1 — ${JSON.stringify(holder)} ${readdirSync(leaseRoot).join(",")}`).toBe(true);
      expect(readDevHubLeaseV1(paths[1])?.pid).toBe(holder?.pid);
    }
  }, 120_000);

  it("an owner whose data root is held gives its port claim back, and releases only its own claims", () => {
    const leaseRoot = scratch("both");
    const dataDir = scratch("both-data");
    const paths = devHubLeasePathsV1(leaseRoot, 8787, dataDir);
    writeFileSync(paths[1], JSON.stringify({ pid: process.pid, port: 8788, dataDir, hubUrl: "http://127.0.0.1:8788", acquiredAt: 1 }));
    const claim = claimDevHubLeaseV1(paths, { pid: 7_000_003, port: 8787, dataDir, hubUrl: "http://127.0.0.1:8787", acquiredAt: 2 }, (pid) => pid === process.pid || pid === 7_000_003);
    expect(claim).toMatchObject({ kind: "held", lease: { pid: process.pid } });
    expect(existsSync(paths[0]), "no half claim is left on the port").toBe(false);
    releaseDevHubLeaseV1(paths, 7_000_003);
    expect(readDevHubLeaseV1(paths[1])?.pid, "another owner's claim is never released").toBe(process.pid);
  });
});

describe("the development catalog", () => {
  it("classifies every data root the way the product and the schema contract both do", () => {
    const header = contract("DevHubCatalogHeaderV1");
    const pointer = contract("DevHubCatalogPointerV1");
    for (const row of fixture.catalogs) {
      const dataDir = scratch("catalog");
      writeCatalog(dataDir, row.pointer, row.header);
      const freshness = devHubCatalogFreshnessV1(dataDir);
      expect(freshness.kind, row.name).toBe(row.expected);
      if (freshness.kind === "stale") expect(freshness.reason, row.name).toBe(row.reason);
      if (row.pointer !== null) expect(pointer(row.pointer), `${row.name}: pointer contract`).toBe(row.reason !== "current.json names no generation");
      if (row.header !== null) expect(header(row.header), `${row.name}: Ajv agrees`).toBe(row.expected === "current");
    }
  });

  it("republishes a stale catalog with progress instead of booting it, and a cancelled publication keeps the previous generation", async () => {
    const stale = fixture.catalogs.find((row) => row.expected === "stale" && row.header !== null && row.header.schemaVersion === 2)!;
    const current = fixture.catalogs.find((row) => row.expected === "current")!;
    const publisher = (lines: readonly string[]): DevHubCatalogPublisherV1 & { calls: number } => {
      const run = Object.assign(
        async (dataDir: string, _packages: string, _signal: AbortSignal, onLine: (line: string) => void) => {
          run.calls += 1;
          for (const line of lines) onLine(line);
          const next = { ...current.pointer!, generationId: "c".repeat(64) };
          writeCatalog(dataDir, next, current.header);
        },
        { calls: 0 },
      );
      return run;
    };
    const dataDir = scratch("republish");
    writeCatalog(dataDir, stale.pointer, stale.header);
    const statuses: DevHubStatusV1[] = [];
    const publish = publisher(["stage=Build 1/3", "stage=GuestCodecExecuting 2/3", "stage=Publish 3/3"]);
    expect(await ensureCurrentTrustedCatalogV1(dataDir, publish, { report: (status) => void statuses.push(status) })).toBe("published");
    expect(statuses.map((status) => status.kind)).toEqual(["catalog-stale", "catalog-publishing", "catalog-publishing", "catalog-publishing", "catalog-published"]);
    expect(devHubCatalogFreshnessV1(dataDir).kind).toBe("current");
    expect(await ensureCurrentTrustedCatalogV1(dataDir, publish), "a current catalog is never republished").toBe("current");
    expect(publish.calls).toBe(1);

    const cancelled = scratch("cancel");
    writeCatalog(cancelled, stale.pointer, stale.header);
    const before = readFileSync(join(cancelled, "trusted-catalog", "current.json"), "utf8");
    const abort = new AbortController();
    const waiting: DevHubCatalogPublisherV1 = (_dataDir, _packages, signal, onLine) =>
      new Promise<void>((_resolve, reject) => {
        onLine("stage=Build 1/3");
        signal.addEventListener("abort", () => reject(new DOMException("cancelled", "AbortError")), { once: true });
      });
    const cancelStatuses: DevHubStatusV1[] = [];
    const outcome = ensureCurrentTrustedCatalogV1(cancelled, waiting, { signal: abort.signal, report: (status) => void cancelStatuses.push(status) });
    abort.abort();
    expect(await outcome).toBe("cancelled");
    expect(cancelStatuses.map((status) => status.kind)).toEqual(["catalog-stale", "catalog-publishing", "catalog-cancelled"]);
    expect(readFileSync(join(cancelled, "trusted-catalog", "current.json"), "utf8"), "the previous generation stays current").toBe(before);

    const absent = scratch("absent");
    const fresh = publisher([]);
    expect(await ensureCurrentTrustedCatalogV1(absent, fresh)).toBe("published");
    const broken = scratch("broken");
    writeCatalog(broken, stale.pointer, stale.header);
    await expect(ensureCurrentTrustedCatalogV1(broken, async () => undefined), "a publication that leaves the catalog stale is a failure, not a boot").rejects.toThrow(/without a current catalog/u);
  });

  it("declares the catalog format the hub's publisher writes and its loader requires", () => {
    const declared = schema.$defs.DevHubCatalogHeaderV1.properties;
    const publisher = readFileSync(join(REPO, "🌎️hub/📦️packages/🦀️rust/📜️script.ts"), "utf8");
    const start = publisher.indexOf("\nexport async function materializeTrustedCatalogBundle");
    const body = publisher.slice(start, publisher.indexOf("\nfunction ", start + 1));
    expect(start).toBeGreaterThan(0);
    expect(body).toContain(`schemaVersion: ${declared.schemaVersion.const},`);
    for (const key of declared.packages.items.required) expect(body, key).toMatch(new RegExp(`\\n\\s+${key}[:,]`, "u"));
    const loader = readFileSync(join(REPO, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"), "utf8");
    expect(loader).toContain(`bundle.schema_version != ${declared.schemaVersion.const}`);
  });
});

function readDevHubLeaseV1Json(record: unknown): unknown {
  const dir = scratch("lease-read");
  const path = join(dir, "lease.json");
  writeFileSync(path, JSON.stringify(record));
  const read = readDevHubLeaseV1(path);
  rmSync(dir, { recursive: true, force: true });
  return read;
}

async function deadPid(): Promise<number> {
  const child = spawn(process.execPath, ["-e", "0"], { stdio: "ignore" });
  const pid = child.pid!;
  await new Promise<void>((resolveExit) => child.once("exit", () => resolveExit()));
  return pid;
}

/** 🏁️ One owner process racing for the lease: it loads the module, reports ready, waits for the shared start file, claims,
 * and keeps its claim for a moment so the others meet a live holder. */
function raceClaim(leaseRoot: string, dataDir: string, go: string): { readonly ready: Promise<void>; readonly done: Promise<string> } {
  const script = `
    const { claimDevHubLeaseV1, devHubLeasePathsV1 } = await import(${JSON.stringify(MODULE)});
    const { existsSync } = await import("node:fs");
    console.log("ready");
    while (!existsSync(${JSON.stringify(go)})) await new Promise((r) => setTimeout(r, 2));
    const claim = claimDevHubLeaseV1(devHubLeasePathsV1(${JSON.stringify(leaseRoot)}, 8787, ${JSON.stringify(dataDir)}), { pid: process.pid, port: 8787, dataDir: ${JSON.stringify(dataDir)}, hubUrl: "http://127.0.0.1:8787", acquiredAt: Date.now() });
    console.log("claim:" + claim.kind);
    await new Promise((r) => setTimeout(r, 1500));
  `;
  const child = spawn("bun", ["-e", script], { stdio: ["ignore", "pipe", "pipe"] });
  let output = "";
  let errors = "";
  const settled: { ready: (error?: Error) => void } = { ready: () => undefined };
  const ready = new Promise<void>((resolveReady, rejectReady) => (settled.ready = (error) => (error === undefined ? resolveReady() : rejectReady(error))));
  child.stdout.on("data", (chunk: Buffer) => {
    output += chunk.toString("utf8");
    if (output.includes("ready")) settled.ready();
  });
  child.stderr.on("data", (chunk: Buffer) => void (errors += chunk.toString("utf8")));
  const done = new Promise<string>((resolveDone, rejectDone) =>
    child.once("exit", (code) => {
      const kind = /claim:(\w+)/u.exec(output)?.[1];
      const failure = code !== 0 || kind === undefined ? new Error(`racer exited ${code}: ${errors.slice(-400)}`) : null;
      settled.ready(failure ?? undefined);
      if (failure !== null) rejectDone(failure);
      else resolveDone(kind!);
    }),
  );
  done.catch(() => undefined);
  return { ready, done };
}
