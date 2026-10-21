import { plugin } from "bun";
import { appendFileSync, readFileSync } from "node:fs";
import net from "node:net";
import { join } from "node:path";
import { decideListenV1, decideSpawnV1, decideWriteV1, guardContextFromEnvironmentV1, guardOpenWritesV1, guardWriteTargetsV1, type GuardViolationV1 } from "../🟦️.ts";
import { readTestSandboxProtocolV1 } from "../../📐️layout/🟦️.ts";

const declared = readTestSandboxProtocolV1(), context = guardContextFromEnvironmentV1();

function frame(): string {
  const line = (new Error().stack ?? "").split("\n").find(row => row.includes("/🧪️tests/") && !row.includes("/🏖️sandbox/"));
  return line?.trim() ?? "";
}

if (context) {
  const report = process.env[declared.environment.report];
  const settle = (violation: GuardViolationV1 | undefined): void => {
    if (!violation) return;
    const row = { time: Date.now(), pid: process.pid, ...violation, frame: frame() };
    if (report) { try { appendFileSync(report, JSON.stringify(row) + "\n"); } catch {} }
    if (context.mode === "enforce") throw Error(`[sandbox-guard] ${violation.kind} refused: ${violation.function} ${violation.target}`);
  };
  Object.defineProperty(globalThis, "__semioTestGuardV1", {
    value: Object.freeze({
      write(fn: string, args: readonly unknown[]): void { for (const target of guardWriteTargetsV1(declared, fn, args)) settle(decideWriteV1(context, fn, target)); },
      open(fn: string, args: readonly unknown[]): void { if (guardOpenWritesV1(args[1]) && typeof args[0] === "string") settle(decideWriteV1(context, fn, args[0])); },
      spawn(command: unknown, args: readonly unknown[]): void { if (typeof command === "string") settle(decideSpawnV1(context, command, Array.isArray(args) ? args.map(String) : [])); },
      exec(command: unknown): void { if (typeof command !== "string") return; const [program = "", ...rest] = command.trim().split(/\s+/u); settle(decideSpawnV1(context, program, rest)); },
    }), configurable: false,
  });
  const listen = net.Server.prototype.listen;
  let listening = false;
  (net.Server.prototype as { listen: unknown }).listen = function (this: net.Server, ...args: unknown[]): net.Server {
    const first = args[0], port = typeof first === "number" ? first : typeof first === "object" && first !== null ? (first as { port?: number }).port : undefined;
    if (typeof port === "number" && !listening) settle(decideListenV1(context, port));
    const nested = listening;
    listening = true;
    try { return (listen as (...rest: unknown[]) => net.Server).apply(this, args); } finally { listening = nested; }
  };
  const bun = Bun as unknown as Record<string, (...rest: unknown[]) => unknown>;
  for (const name of ["serve", "listen"] as const) {
    const original = bun[name];
    if (typeof original !== "function") continue;
    try { bun[name] = function (this: unknown, ...args: unknown[]): unknown { const port = (args[0] as { port?: unknown } | undefined)?.port; if (typeof port === "number" && !listening) settle(decideListenV1(context, port)); return original.apply(this, args); }; } catch {}
  }
  const write = bun.write;
  if (typeof write === "function") { try { bun.write = function (this: unknown, ...args: unknown[]): unknown { const target = args[0]; settle(typeof target === "string" ? decideWriteV1(context, "Bun.write", target) : undefined); return write.apply(this, args); }; } catch {} }
  const here = import.meta.dir, shims: Readonly<Record<string, string>> = {
    fs: join(here, "../🩻️fs/🟦️.ts"), "fs/promises": join(here, "../🩻️fs-promises/🟦️.ts"), child_process: join(here, "../🩻️child-process/🟦️.ts"), "bun:sqlite": join(here, "../🩻️sqlite/🟦️.ts"),
  };
  const guardRoot = join(here, "../..");
  const specifier = /(from\s*|import\s*\(\s*|require\s*\(\s*)(["'])(?:node:)?(fs\/promises|fs|child_process|bun:sqlite)\2/gu;
  plugin({
    name: "semio-test-guard",
    setup(build) {
      build.onLoad({ filter: /^(?!.*[\\/]node_modules[\\/]).*\.(?:[cm]?ts|tsx|mjs)$/u }, args => {
        const loader = args.path.endsWith("x") ? "tsx" : args.path.endsWith(".mjs") ? "js" : "ts", text = readFileSync(args.path, "utf8");
        if (args.path.startsWith(guardRoot) || !/(?:fs|child_process|sqlite)["']/u.test(text)) return { contents: text, loader };
        return { contents: text.replace(specifier, (_all, lead: string, _quote: string, name: string) => `${lead}${JSON.stringify(shims[name])}`), loader };
      });
    },
  });
}
