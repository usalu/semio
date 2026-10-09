#!/usr/bin/env bun
import { existsSync } from "node:fs";
/** 🧭️ Behaviours of the journey fixture workspace: each mode is one task the dashboard journeys start. */
const [mode = "", ...rest] = process.argv.slice(2);
const mark = process.env.JOURNEY_MARK ?? "unset";
const out = (text: string): void => { process.stdout.write(`${text}\n`); };

async function lines(handle: (line: string) => boolean | void): Promise<void> {
  for await (const line of console) if (handle(line) === true) return;
}

if (mode === "words") {
  out("WORDS-READY type words, `quit` ends");
  await lines((line) => { out(`ECHO:${line}`); return line === "quit"; });
  out("WORDS-BYE");
} else if (mode === "serve") {
  const port = Number(process.env.JOURNEY_PORT);
  const server = Bun.serve({ port, hostname: "127.0.0.1", fetch: async (request) => { if (process.env.JOURNEY_HTTP_DELAY) await Bun.sleep(Number(process.env.JOURNEY_HTTP_DELAY)); return process.env.JOURNEY_HTTP_GATE && !existsSync(process.env.JOURNEY_HTTP_GATE) ? new Response("warming", { status: 503 }) : new Response(`journey-ok ${mark} ${new URL(request.url).pathname}`); } });
  out(`journey server listening on http://127.0.0.1:${server.port}`);
  await new Promise(() => {});
} else if (mode === "ticker") {
  out(`TICKER-START pid=${process.pid} mark=${mark}`);
  for (let tick = 1; ; tick++) { await Bun.sleep(300); out(`TICK ${tick} pid=${process.pid} mark=${mark}`); }
} else if (mode === "burst") {
  const bytes = Number(rest[0] ?? process.env.JOURNEY_BURST_BYTES ?? "1048576");
  const line = `${"0123456789abcdef".repeat(4)}\n`;
  const block = line.repeat(1024);
  let written = 0;
  while (written < bytes) { process.stdout.write(block); written += block.length; }
  out(`BURST-DONE ${written}`);
} else if (mode === "exit") {
  out(`EXITING with ${rest[0] ?? "3"}`);
  process.exit(Number(rest[0] ?? "3"));
} else if (mode === "env") {
  out(`ENV mark=${mark} cwd=${process.cwd().length > 0}`);
} else if (mode === "probe") {
  const response = await fetch(rest[0]!);
  out(`PROBE ${response.status} ${await response.text()}`);
  process.exit(response.ok ? 0 : 1);
} else if (mode === "title") {
  process.stdout.write(`\u001b]0;journey-title-${mark}\u0007`);
  out("TITLE-SET");
  await Bun.sleep(3000);
} else if (mode === "resize") {
  const measure = (): string => {
    const probe = process.platform === "win32" ? Bun.spawnSync(["cmd", "/c", "mode con"], { stdin: "inherit", stdout: "pipe", stderr: "ignore" }) : Bun.spawnSync(["stty", "size"], { stdin: "inherit", stdout: "pipe", stderr: "ignore" });
    const text = probe.stdout.toString();
    if (process.platform === "win32") return `SIZE ${/Columns:\s*(\d+)/.exec(text)?.[1]}x${/Lines:\s*(\d+)/.exec(text)?.[1]}`;
    const [rows, columns] = text.trim().split(/\s+/);
    return `SIZE ${columns}x${rows}`;
  };
  let shown = "";
  for (;;) {
    const size = measure();
    if (size !== shown) { out(size); shown = size; }
    await Bun.sleep(250);
  }
} else {
  out(`unknown mode ${mode}`);
  process.exit(2);
}
