/** 🧪️ The local stack's own parts: waiting for a server reports progress and fails clearly when the server exits, never
 * becomes ready or the wait is cancelled; the end-to-end gate reads its command line as documented; an owned command
 * stops with its whole process tree; and the static origin of the
 * release rehearsal answers every request like a static CDN — status, media type and body equal to Vite's preview server
 * in multi-page mode (the third-party oracle: files as they are, `index.html` for a directory, status 404 for anything
 * else), a miss carrying `404.html` as GitHub Pages serves it. The dev launcher never strands a developer on development
 * data of another storage format: it reads the format row the proctor stamps (the proctor's own source is the oracle
 * for the format it reads), sets the launcher's own disposable folder aside and starts fresh, saying so once, and leaves
 * a folder the developer named alone, with one message that says how to go on — the dev command ends over it with that
 * line and status 1, not with a stack trace. The proctor the dev command runs is supervised: built anew for a changed
 * Rust source of a crate it is built from (cargo's own resolution is the oracle for which those are), launched anew for
 * a changed catalog or quiz, and launched again ever more patiently when it ends by itself. And the dev server survives
 * its proctor: a proctor that answers an upgrade without upgrading — as one that restarts or goes away does — never
 * takes it along, also where sockets lack `destroySoon` (Bun 1.3), whose soft ending is held to Node's own.
 * @see ../../🧱️stack/🟦️.ts — the stack under test
 * @see ../../../../🛂️proctor/🏗️bootstrap/🟦️.ts — the dev launcher of the proctor
 * @see ../../../../🛂️proctor/🔨️modules/🗄️storage/🦀️.rs — `FORMAT_SCHEMA`, `FORMAT_VERSION`
 * @see https://vite.dev/guide/api-javascript.html#preview — the oracle
 * @see https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site */
import { existsSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { createServer } from "node:http";
import { connect, createServer as createNetServer, type AddressInfo, type Socket } from "node:net";
import { dirname, join, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { createServer as createViteServer, preview } from "vite";
import { destroySoon, semioServeUpgradeVitePlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { afterEach, describe, expect, it } from "vitest";
import deployment from "../../🚀️deploy/🔣️.json" with { type: "json" };
import { QUIZ_E2E_PORTS, endToEndPlan } from "../../🎭️e2e/🟦️.ts";
import { SITE_MEDIA_TYPES, awaitReady, catalogFiles, httpAnswers, launchOwned, relaunchDelay, serveStaticSite } from "../../🧱️stack/🟦️.ts";
import { DevelopmentDataRefused, PROCTOR_DATABASE_FILE, PROCTOR_DEV_DATA_DIRECTORY, PROCTOR_STORAGE_FORMAT, developmentDataSettled, proctorDevelopmentEnvironment, proctorSourceChanged, proctorSourceDirectories, settleDevelopmentData, storedProctorFormat } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

const scratch: string[] = [];
const closing: (() => Promise<unknown>)[] = [];

/** 📂️ A throw-away directory with `files` (relative path → content). */
function directory(files: Record<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "quiz-stack-"));
  scratch.push(root);
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

/** 🕰️ A clock that advances `step` ms on every reading. */
function clock(step: number): () => number {
  let at = 0;
  return () => (at += step);
}

afterEach(async () => {
  for (const close of closing.splice(0)) await close();
  for (const root of scratch.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("waiting for a server of the local stack", () => {
  it("resolves once the probe answers and reports progress while it waits", async () => {
    const lines: string[] = [];
    let asked = 0;
    await awaitReady({ label: "the proctor at http://127.0.0.1:1", probe: async () => ++asked >= 4, pollMs: 1, progressMs: 2_000, now: clock(1_000), say: (line) => lines.push(line) });
    expect(asked).toBe(4);
    expect(lines.length).toBeGreaterThanOrEqual(1);
    expect(lines.every((line) => /^waiting for the proctor at http:\/\/127\.0\.0\.1:1 \(\d+ s\)$/u.test(line))).toBe(true);
  });

  it("fails with the exit status when the server exits before it is ready", async () => {
    await expect(awaitReady({ label: "the proctor", probe: async () => false, exited: Promise.resolve(3), pollMs: 1 })).rejects.toThrow("the proctor exited with status 3 before it became ready");
  });

  it("fails when the server never becomes ready", async () => {
    await expect(awaitReady({ label: "the site", probe: async () => false, pollMs: 1, timeoutMs: 5_000, now: clock(1_000) })).rejects.toThrow("the site did not become ready within 5 s");
  });

  it("stops waiting when it is cancelled", async () => {
    const cancelling = new AbortController();
    const waiting = awaitReady({ label: "the site", probe: async () => false, pollMs: 60_000, signal: cancelling.signal });
    cancelling.abort();
    await expect(waiting).rejects.toThrow("waiting for the site was cancelled");
  });
});

describe("the command line of the end-to-end gate", () => {
  it("runs both topologies at once unless one is named or they are asked for one after the other", () => {
    expect(endToEndPlan([])).toEqual({ topologies: ["dev", "rehearsal"], serial: false, keep: false, proctor: undefined, playwright: [] });
    expect(endToEndPlan(["rehearsal"]).topologies).toEqual(["rehearsal"]);
    expect(endToEndPlan(["rehearsal", "dev"]).topologies).toEqual(["dev", "rehearsal"]);
    expect(endToEndPlan(["--serial", "--keep"])).toMatchObject({ serial: true, keep: true, playwright: [] });
  });

  it("hands everything it does not know to Playwright and takes a proctor executable for itself", () => {
    expect(endToEndPlan(["dev", "--grep", "leaderboard", "--workers", "1"])).toMatchObject({ topologies: ["dev"], playwright: ["--grep", "leaderboard", "--workers", "1"] });
    expect(endToEndPlan(["--proctor", "target/release/proctor", "--headed"])).toMatchObject({ proctor: "target/release/proctor", playwright: ["--headed"] });
    expect(() => endToEndPlan(["--proctor"])).toThrow("--proctor needs the path of a proctor executable");
  });

  it("keeps the throw-away ports apart from each other and from the dev ports", () => {
    const ports = Object.values(QUIZ_E2E_PORTS).flatMap((stack) => [stack.site, stack.proctor]);
    expect(new Set(ports).size).toBe(ports.length);
    for (const port of ports) expect([6061, deployment.proctor.port]).not.toContain(port);
  });
});

describe("the supervision of the dev proctor", () => {
  it("launches a proctor that ended by itself again after one second, then ever more patiently, at most every half minute", () => {
    expect([1, 2, 3, 4, 5, 6, 7, 20].map(relaunchDelay)).toEqual([1_000, 2_000, 4_000, 8_000, 16_000, 30_000, 30_000, 30_000]);
  });

  it("follows the catalog and every quiz it lists, and the catalog alone when it cannot be read", () => {
    const root = directory({ "site/🔣️.json": JSON.stringify({ quizzes: ["../energy/a/🔣️.json", "b.json", 3] }), "broken/🔣️.json": "{" });
    expect(catalogFiles(join(root, "site", "🔣️.json"))).toEqual([join(root, "site", "🔣️.json"), resolve(root, "energy", "a", "🔣️.json"), join(root, "site", "b.json")]);
    expect(catalogFiles(join(root, "broken", "🔣️.json"))).toEqual([join(root, "broken", "🔣️.json")]);
  });

  it("builds anew for a changed Rust source or manifest only, never for build output or installed packages", () => {
    expect(["🔨️modules/🎭️actors/🦀️.rs", "📦️packages\\🦀️rust\\Cargo.toml", "🧬️schema/🦀️.rs"].map(proctorSourceChanged)).toEqual([true, true, true]);
    expect(["🧬️schema/🔣️.json", "🟦️.ts", "target/debug/build/x.rs", "node_modules/a/b.rs", "dist/Cargo.toml", "Cargo.lock"].map(proctorSourceChanged)).toEqual([false, false, false, false, false, false]);
  });

  it("follows the owner of every crate of this repository the proctor is built from, as cargo resolves it, and nothing twice", async () => {
    const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
    const roots = await proctorSourceDirectories(repoRoot);
    for (const owner of [["🎓️teaching", "🛂️proctor"], ["🧰️framework", "🛍️products", "❓️quiz"], ["🧰️framework", "🛍️products", "🖥️server"]]) expect(roots).toContain(join(repoRoot, ...owner));
    expect(roots.every((root) => root.startsWith(repoRoot) && existsSync(root) && !root.includes("📦️packages"))).toBe(true);
    expect(roots.some((root, index) => roots.some((other, at) => at !== index && root.startsWith(`${other}${sep}`)))).toBe(false);
  }, 120_000);
});

describe("an owned command of the local stack", () => {
  it("stops with the processes it started", async () => {
    const root = directory({});
    const port = 20_000 + Math.floor(Math.random() * 20_000);
    const child = `require('node:http').createServer((_, answer) => answer.end('grandchild')).listen(${port}, '127.0.0.1')`;
    const parent = `require('node:child_process').spawn(process.execPath, ['-e', ${JSON.stringify(child)}], { stdio: 'inherit' }); setInterval(() => undefined, 1_000);`;
    const owned = launchOwned(process.execPath, ["-e", parent], root, process.env, join(root, "log.txt"));
    closing.push(() => owned.stop());
    await awaitReady({ label: "the grandchild", probe: () => httpAnswers(`http://127.0.0.1:${port}`), exited: owned.exited, pollMs: 50, timeoutMs: 20_000 });
    await owned.stop();
    expect(await httpAnswers(`http://127.0.0.1:${port}`, 500)).toBe(false);
  }, 30_000);
});

describe("development data of another storage format", () => {
  const AT = new Date("2026-10-02T06:15:23.456Z");

  /** ✍️ A writing connection to the SQLite file `file` by the SQLite of the runtime the tests run in (`bun:sqlite` under
   * Bun, `node:sqlite` under Node); `run` executes statements one batch at a time. */
  async function connect(file: string): Promise<{ run(statements: string): void; close(): void }> {
    if (process.versions.bun !== undefined) {
      const { Database } = await import("bun:sqlite");
      const connection = new Database(file);
      return { run: (statements) => void connection.run(statements), close: () => connection.close() };
    }
    const { DatabaseSync } = await import("node:sqlite");
    const connection = new DatabaseSync(file);
    return { run: (statements) => connection.exec(statements), close: () => connection.close() };
  }

  const STAMP = "CREATE TABLE proctor_format (singleton INTEGER PRIMARY KEY CHECK (singleton = 1), schema TEXT NOT NULL, version INTEGER NOT NULL)";

  /** 🗄️ A proctor database in `data` stamped with `format`, as the proctor stamps its own (in WAL mode). */
  async function database(data: string, format: { readonly schema: string; readonly version: number }): Promise<void> {
    mkdirSync(data, { recursive: true });
    const connection = await connect(join(data, PROCTOR_DATABASE_FILE));
    connection.run("PRAGMA journal_mode = WAL");
    connection.run(STAMP);
    connection.run(`INSERT INTO proctor_format (singleton, schema, version) VALUES (1, '${format.schema.replaceAll("'", "''")}', ${format.version})`);
    connection.close();
  }

  /** 📋️ Copies the files of `from` into a new `to`; `cpSync` of a directory in a path with emoji is not reliable on Windows. */
  function mirror(from: string, to: string): void {
    mkdirSync(to, { recursive: true });
    for (const name of readdirSync(from)) writeFileSync(join(to, name), readFileSync(join(from, name)));
  }

  /** 🧾️ What a data directory holds besides the two files of SQLite's write-ahead log. */
  function held(data: string): readonly string[] {
    return readdirSync(data).filter((name) => !/-(?:shm|wal)$/u.test(name));
  }

  /** 🏠️ A throw-away repository root and the environment a dev proctor gets there (`PROCTOR_DATA` as `named`, else the
   * launcher's own folder). */
  function checkout(named?: string): { readonly root: string; readonly env: NodeJS.ProcessEnv; readonly own: string } {
    const root = directory({});
    return { root, env: proctorDevelopmentEnvironment(root, named === undefined ? {} : { PROCTOR_DATA: join(root, named) }), own: join(root, ...PROCTOR_DEV_DATA_DIRECTORY) };
  }

  it("reads the format the proctor itself writes and accepts", () => {
    const source = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../🛂️proctor/🔨️modules/🗄️storage/🦀️.rs"), "utf8");
    expect(PROCTOR_STORAGE_FORMAT.schema).toBe(/pub const FORMAT_SCHEMA: &str = "([^"]+)";/u.exec(source)?.[1]);
    expect(PROCTOR_STORAGE_FORMAT.version).toBe(Number(/pub const FORMAT_VERSION: i64 = (\d+);/u.exec(source)?.[1]));
    expect(PROCTOR_DATABASE_FILE).toBe(/pub const DATABASE_FILE: &str = "([^"]+)";/u.exec(source)?.[1]);
  });

  it("reads the format row of a database and changes nothing of it", async () => {
    const { own } = checkout();
    await database(own, { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    const before = readFileSync(join(own, PROCTOR_DATABASE_FILE));
    expect(await storedProctorFormat(own)).toEqual({ schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    expect(held(own)).toEqual([PROCTOR_DATABASE_FILE]);
    expect(readFileSync(join(own, PROCTOR_DATABASE_FILE)).equals(before)).toBe(true);
  });

  it("reads a format row that only the write-ahead log of a proctor that was killed holds yet", async () => {
    const { root, own } = checkout();
    mkdirSync(own, { recursive: true });
    const writing = await connect(join(own, PROCTOR_DATABASE_FILE));
    writing.run("PRAGMA journal_mode = WAL");
    writing.run("PRAGMA wal_autocheckpoint = 0");
    writing.run(STAMP);
    writing.run(`INSERT INTO proctor_format VALUES (1, '${PROCTOR_STORAGE_FORMAT.schema}', 1)`);
    const killed = join(root, "killed");
    mirror(own, killed);
    writing.close();
    expect(statSync(join(killed, `${PROCTOR_DATABASE_FILE}-wal`)).size).toBeGreaterThan(0);
    expect(await storedProctorFormat(killed)).toEqual({ schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    rmSync(own, { recursive: true });
    mirror(killed, own);
    expect(await settleDevelopmentData(root, proctorDevelopmentEnvironment(root, {}), AT)).toContain(`moved aside to ${own}.v1-20261002T061523 and`);
    expect(existsSync(own)).toBe(false);
  });

  it("knows no format where there is no database, no format row or no SQLite file", async () => {
    const { root, own } = checkout();
    expect(await storedProctorFormat(own)).toBeUndefined();
    mkdirSync(own, { recursive: true });
    expect(await storedProctorFormat(own)).toBeUndefined();
    const unstamped = await connect(join(own, PROCTOR_DATABASE_FILE));
    unstamped.run("CREATE TABLE something_else (id INTEGER)");
    unstamped.close();
    expect(await storedProctorFormat(own)).toBeUndefined();
    writeFileSync(join(root, PROCTOR_DATABASE_FILE), "not a database");
    expect(await storedProctorFormat(root)).toBeUndefined();
  });

  it("leaves data of the format the proctor reads, and a folder without data, alone", async () => {
    const { root, env, own } = checkout();
    expect(await settleDevelopmentData(root, env, AT)).toBeUndefined();
    await database(own, PROCTOR_STORAGE_FORMAT);
    expect(await settleDevelopmentData(root, env, AT)).toBeUndefined();
    expect(readdirSync(dirname(own))).toEqual(["proctor-dev"]);
  });

  it("sets the launcher's own folder of an older format aside, starts fresh and says so in one message", async () => {
    const { root, env, own } = checkout();
    await database(own, { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    const aside = `${own}.v1-20261002T061523`;
    const said = await settleDevelopmentData(root, env, AT);
    expect(said).toBe(
      `${own} held disposable development data in an older storage format (${PROCTOR_STORAGE_FORMAT.schema} v1; this proctor reads v${PROCTOR_STORAGE_FORMAT.version}, and there is no migration). It was moved aside to ${aside} and the proctor starts with empty data. Delete ${aside} when you do not need it; to reset development data yourself at any time, stop the proctor and delete ${own}.`,
    );
    expect(existsSync(own)).toBe(false);
    expect(await storedProctorFormat(aside)).toEqual({ schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    expect(await settleDevelopmentData(root, env, AT)).toBeUndefined();
  });

  it("never overwrites what it set aside before", async () => {
    const { root, env, own } = checkout();
    for (const expected of [`${own}.v1-20261002T061523`, `${own}.v1-20261002T061523-1`, `${own}.v1-20261002T061523-2`]) {
      await database(own, { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
      expect(await settleDevelopmentData(root, env, AT)).toContain(`moved aside to ${expected} and`);
      expect(existsSync(expected)).toBe(true);
    }
  });

  it("calls a format ahead of the proctor newer and a foreign one another", async () => {
    const { root, env, own } = checkout();
    await database(own, { schema: PROCTOR_STORAGE_FORMAT.schema, version: PROCTOR_STORAGE_FORMAT.version + 1 });
    expect(await settleDevelopmentData(root, env, AT)).toContain(`in a newer storage format (${PROCTOR_STORAGE_FORMAT.schema} v${PROCTOR_STORAGE_FORMAT.version + 1};`);
    await database(own, { schema: "someone.else", version: 7 });
    expect(await settleDevelopmentData(root, env, AT)).toContain("in another storage format (someone.else v7;");
  });

  it("leaves a folder the developer named alone and says in one message how to go on", async () => {
    const { root, env, own } = checkout("my-data");
    const named = join(root, "my-data");
    await database(named, { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
    const before = readFileSync(join(named, PROCTOR_DATABASE_FILE));
    await expect(settleDevelopmentData(root, env, AT)).rejects.toThrow(
      `${named} holds proctor data in an older storage format (${PROCTOR_STORAGE_FORMAT.schema} v1; this proctor reads v${PROCTOR_STORAGE_FORMAT.version}, and there is no migration). PROCTOR_DATA names that folder, so it is left as it is: delete or move it, or unset PROCTOR_DATA to use the launcher's own disposable folder ${own}.`,
    );
    await expect(settleDevelopmentData(root, env, AT)).rejects.toBeInstanceOf(DevelopmentDataRefused);
    expect(held(named)).toEqual([PROCTOR_DATABASE_FILE]);
    expect(readFileSync(join(named, PROCTOR_DATABASE_FILE)).equals(before)).toBe(true);
    expect(existsSync(own)).toBe(false);
  });

  it("lets a dev command start over settled data, and ends it over a refusal with that one line and status 1 instead of a stack trace", async () => {
    const status = process.exitCode;
    try {
      const fresh = checkout();
      const lines: string[] = [];
      expect(await developmentDataSettled(fresh.root, fresh.env, (line) => lines.push(line))).toBe(true);
      await database(fresh.own, { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
      expect(await developmentDataSettled(fresh.root, fresh.env, (line) => lines.push(line), AT)).toBe(true);
      expect(lines).toEqual([expect.stringContaining(`moved aside to ${fresh.own}.v1-20261002T061523 and`)]);
      expect(process.exitCode ?? 0).toBe(status ?? 0);

      const { root, env } = checkout("my-data");
      await database(join(root, "my-data"), { schema: PROCTOR_STORAGE_FORMAT.schema, version: 1 });
      const refusal: string[] = [];
      expect(await developmentDataSettled(root, env, (line) => refusal.push(line), AT)).toBe(false);
      expect(refusal).toEqual([expect.stringMatching(/^.+my-data holds proctor data in an older storage format .+ PROCTOR_DATA names that folder, so it is left as it is: delete or move it, or unset PROCTOR_DATA .+\.$/u)]);
      expect(process.exitCode).toBe(1);
    } finally {
      process.exitCode = status;
    }
  });
});

describe("a dev server whose proctor goes away while it proxies a socket", () => {
  /** 🔌️ A socket pair on loopback: what `serve` is handed for each connection, and the client end's events in order. */
  async function socketPair(serve: (socket: Socket) => void): Promise<string[]> {
    const seen: string[] = [];
    const server = createNetServer(serve);
    closing.push(() => new Promise((closed) => server.close(closed)));
    await new Promise<void>((listening) => server.listen(0, "127.0.0.1", listening));
    const client = connect((server.address() as AddressInfo).port, "127.0.0.1");
    client.on("data", (chunk) => seen.push(`data:${String(chunk)}`));
    await new Promise<void>((closed) => {
      client.on("end", () => seen.push("end"));
      client.on("close", () => {
        seen.push("close");
        closed();
      });
    });
    return seen;
  }

  it("ends a socket softly as Node's own destroySoon does: what was written arrives, then the end, then the close", async () => {
    const ours = await socketPair((socket) => {
      socket.write("bye");
      destroySoon(socket);
    });
    const node = await socketPair((socket) => {
      socket.write("bye");
      (socket as Socket & { destroySoon(): void }).destroySoon();
    });
    expect(ours).toEqual(node);
    expect(ours).toEqual(["data:bye", "end", "close"]);
  });

  it("keeps serving when the proctor answers an upgrade without upgrading, also where sockets lack destroySoon", async () => {
    const backend = createServer((_, answer) => answer.end("http"));
    backend.on("upgrade", (_request, socket) => socket.end("HTTP/1.1 503 Service Unavailable\r\ncontent-length: 4\r\nconnection: close\r\n\r\ngone"));
    closing.push(() => new Promise((closed) => backend.close(closed)));
    await new Promise<void>((listening) => backend.listen(0, "127.0.0.1", listening));
    const root = directory({ "index.html": "<!doctype html><title>site</title>" });
    const site = await createViteServer({ configFile: false, root, logLevel: "silent", plugins: [semioServeUpgradeVitePlugin()], server: { host: "127.0.0.1", port: 0, hmr: false, proxy: { "/scopes": { target: `http://127.0.0.1:${(backend.address() as AddressInfo).port}`, ws: true } } } });
    closing.push(() => site.close());
    site.httpServer!.prependListener("upgrade", (_request: unknown, socket: { destroySoon?: unknown }) => {
      socket.destroySoon = undefined;
    });
    await site.listen();
    const origin = site.resolvedUrls!.local[0]!.replace(/\/$/u, "");
    const port = Number(new URL(origin).port);
    for (let attempt = 0; attempt < 3; attempt++) {
      const answered = await new Promise<string>((done) => {
        let text = "";
        const socket = connect(port, "127.0.0.1", () => socket.write("GET /scopes/a/presence/ws HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\r\n"));
        socket.on("data", (chunk) => (text += String(chunk)));
        socket.on("close", () => done(text));
        socket.on("error", () => done(text));
      });
      expect(answered).toMatch(/^HTTP\/1\.1 503/u);
    }
    expect(await httpAnswers(origin, 5_000)).toBe(true);
  }, 30_000);
});

describe("the static origin of the release rehearsal", () => {
  const files = {
    "index.html": "<!doctype html><title>site</title>",
    "404.html": "<!doctype html><title>not found</title>",
    "CNAME": "quizzes.example\n",
    "favicon.svg": "<svg xmlns='http://www.w3.org/2000/svg'/>",
    "favicon.ico": "icon",
    "assets/app.js": "export const quiz = 1;",
    "assets/app.css": "body{margin:0}",
    "assets/data.json": "{}",
    "manifest.webmanifest": "{}",
    "robots.txt": "User-agent: *",
    "🖼️assets/🔤️fonts/🚀️anta/font.woff2": "font",
    "nested/index.html": "<!doctype html><title>nested</title>",
  };
  const paths = ["/", "/index.html", "/404.html", "/favicon.svg", "/favicon.ico", "/assets/app.js", "/assets/app.css", "/assets/data.json", "/manifest.webmanifest", "/robots.txt", `/${encodeURIComponent("🖼️assets")}/${encodeURIComponent("🔤️fonts")}/${encodeURIComponent("🚀️anta")}/font.woff2`, "/nested/", "/missing", "/missing/deeper.html", "/assets/", "/assets/missing.js"];

  /** 📨️ What an origin answers for `path`: status, media type without parameters, and body. */
  async function answer(origin: string, path: string): Promise<{ readonly status: number; readonly type: string; readonly body: string }> {
    const response = await fetch(`${origin}${path}`, { redirect: "manual" });
    return { status: response.status, type: (response.headers.get("content-type") ?? "").split(";")[0]!.trim(), body: await response.text() };
  }

  it("answers every request like Vite's multi-page preview server", async () => {
    const root = directory(files);
    const site = await serveStaticSite(root, 0);
    closing.push(() => site.close());
    const oracle = await preview({ configFile: false, root, appType: "mpa", logLevel: "silent", build: { outDir: root }, preview: { host: "127.0.0.1", port: 20_000 + Math.floor(Math.random() * 20_000) } });
    closing.push(() => oracle.close());
    const expected = oracle.resolvedUrls!.local[0]!.replace(/\/$/u, "");
    const statuses: number[] = [];
    for (const path of paths) {
      const [ours, theirs] = [await answer(site.origin, path), await answer(expected, path)];
      statuses.push(theirs.status);
      expect({ path, ...ours }).toEqual({ path, ...(theirs.status === 404 ? { status: 404, type: "text/html", body: files["404.html"] } : theirs) });
    }
    expect(statuses.filter((status) => status === 200).length).toBe(12);
    expect(statuses.filter((status) => status === 404).length).toBe(4);
  }, 30_000);

  it("never leaves the directory it serves", async () => {
    const root = directory({ "site/index.html": "<!doctype html>", "site/404.html": "missing", "secret.txt": "secret" });
    const site = await serveStaticSite(join(root, "site"), 0);
    closing.push(() => site.close());
    for (const path of ["/../secret.txt", "/..%2Fsecret.txt", "/%2e%2e/secret.txt", "/%00"]) {
      const escaped = await answer(site.origin, path);
      expect({ path, status: escaped.status, leaked: escaped.body.includes("secret") }).toEqual({ path, status: 404, leaked: false });
    }
  });

  it("types module scripts and styles so a browser runs them", () => {
    expect(SITE_MEDIA_TYPES[".js"]).toMatch(/^text\/javascript/u);
    expect(SITE_MEDIA_TYPES[".mjs"]).toMatch(/^text\/javascript/u);
    expect(SITE_MEDIA_TYPES[".css"]).toMatch(/^text\/css/u);
    expect(SITE_MEDIA_TYPES[".html"]).toMatch(/^text\/html/u);
  });
});
