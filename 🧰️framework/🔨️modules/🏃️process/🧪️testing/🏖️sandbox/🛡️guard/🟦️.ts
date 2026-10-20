import { existsSync, realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { readTestSandboxProtocolV1, testPathInsideV1, type TestSandboxProtocolV1 } from "../📐️layout/🟦️.ts";

/** 🛡️ Names what the guard refused or observed. */
export type GuardViolationKindV1 = "checkout-write" | "protected-write" | "kernel-image" | "process-kill" | "fixed-port";

/** 🛡️ One recorded violation. */
export type GuardViolationV1 = Readonly<{ kind: GuardViolationKindV1; function: string; target: string }>;

/** 🛡️ Carries everything a guard decision needs, resolved once per process. */
export type GuardContextV1 = Readonly<{
  mode: "report" | "enforce"; checkout: string | undefined; cwd: string; platform: NodeJS.Platform;
  allowed: readonly string[]; sandboxRoots: readonly string[]; protectedPrefixes: readonly string[]; allowedCheckoutPrefixes: readonly string[];
  portsDirectory: string | undefined; sandbox: string | undefined; declared: TestSandboxProtocolV1;
}>;

function variants(path: string): string[] {
  const rows = [resolve(path)];
  try { const real = realpathSync(rows[0]!); if (real !== rows[0]) rows.push(real); } catch {}
  return rows;
}

/** 🛡️ Builds the guard context from the sandbox environment; undefined when the guard is off or no sandbox exists. */
export function guardContextFromEnvironmentV1(env: NodeJS.ProcessEnv = process.env, cwd: string = process.cwd(), platform: NodeJS.Platform = process.platform, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): GuardContextV1 | undefined {
  const names = declared.environment, sandbox = env[names.sandbox], requested = env[names.guard] ?? (sandbox ? declared.guard.defaultMode : "off");
  if (!sandbox || requested === "off" || !declared.guard.modes.includes(requested)) return undefined;
  const checkoutRaw = env[names.checkout] ?? env.SEMIO_WORKSPACE_ROOT, artifacts = env[names.artifacts];
  const allowed = [...variants(sandbox), ...variants(tmpdir()), ...declared.guard.scratchRoots.filter(root => existsSync(root)).flatMap(variants), ...(artifacts ? variants(artifacts) : [])];
  return Object.freeze({
    mode: requested as "report" | "enforce", checkout: checkoutRaw ? resolve(checkoutRaw) : undefined, cwd, platform, allowed: [...new Set(allowed)], sandboxRoots: [...new Set([...variants(sandbox), ...(artifacts ? variants(artifacts) : [])])],
    protectedPrefixes: declared.guard.protectedCheckoutPrefixes, allowedCheckoutPrefixes: declared.guard.allowedCheckoutPrefixes,
    portsDirectory: join(dirname(resolve(sandbox)), declared.ports.directory), sandbox, declared,
  });
}

/** 🛡️ Decides whether one write target is allowed: only writes inside the real checkout are judged, and only the sandbox and the declared diagnostic prefixes are allowed there, even when the checkout itself lies below a temp directory. */
export function decideWriteV1(context: GuardContextV1, fn: string, target: string): GuardViolationV1 | undefined {
  const absolute = resolve(context.cwd, target), checkout = context.checkout;
  if (!checkout || !variants(checkout).some(root => testPathInsideV1(root, absolute, context.platform))) return undefined;
  if (context.sandboxRoots.some(root => testPathInsideV1(root, absolute, context.platform))) return undefined;
  const local = relative(checkout, absolute).split(/[\\/]/u).join("/");
  if (context.allowedCheckoutPrefixes.some(prefix => local === prefix || local.startsWith(prefix + "/"))) return undefined;
  const first = local.split("/")[0] ?? "";
  return { kind: context.protectedPrefixes.includes(first) ? "protected-write" : "checkout-write", function: fn, target: absolute };
}

/** 🛡️ Decides one process spawn: kernel image tools may only touch sandbox paths, and name-based process killers are refused. */
export function decideSpawnV1(context: GuardContextV1, command: string, args: readonly string[]): GuardViolationV1 | undefined {
  const program = basename(command).replace(/\.exe$/iu, ""), declared = context.declared.guard;
  if (declared.killPrograms.includes(program)) return { kind: "process-kill", function: program, target: args.join(" ") };
  if (!declared.kernelPrograms.includes(program)) return undefined;
  const verb = args.find(argument => !argument.startsWith("-"));
  if (!verb || !declared.kernelVerbs.includes(verb)) return undefined;
  for (const argument of args) {
    if (argument.startsWith("-") || !/[\\/]/u.test(argument)) continue;
    const absolute = resolve(context.cwd, argument);
    if (!context.allowed.some(root => testPathInsideV1(root, absolute, context.platform))) return { kind: "kernel-image", function: program + " " + verb, target: absolute };
  }
  return undefined;
}

/** 🛡️ Decides one listen: a fixed port is only allowed when this sandbox leased it. */
export function decideListenV1(context: GuardContextV1, port: number): GuardViolationV1 | undefined {
  if (!Number.isInteger(port) || port === 0 || port < 1024) return undefined;
  if (context.portsDirectory && context.sandbox && existsSync(join(context.portsDirectory, String(port)))) return undefined;
  return { kind: "fixed-port", function: "listen", target: String(port) };
}

/** 🛡️ Extracts the written path of one call from its arguments; descriptors and unknown shapes yield nothing. */
export function guardWriteTargetsV1(declared: TestSandboxProtocolV1, fn: string, args: readonly unknown[]): readonly string[] {
  const indexes = (declared.guard.writeTargets as Readonly<Record<string, readonly number[]>>)[fn];
  if (!indexes) return [];
  const rows: string[] = [];
  for (const index of indexes) {
    const value = args[index];
    if (typeof value === "string") rows.push(value);
    else if (value instanceof URL && value.protocol === "file:") rows.push(decodeURIComponent(value.pathname));
    else if (value instanceof Uint8Array) rows.push(Buffer.from(value).toString("utf8"));
  }
  return rows;
}

/** 🛡️ Tells whether an open call writes: a flag string with w, a or +, or numeric flags with a write bit. */
export function guardOpenWritesV1(flags: unknown): boolean {
  if (typeof flags === "string") return /[wa+]/u.test(flags);
  if (typeof flags === "number") return (flags & 3) !== 0 || (flags & 0o100) !== 0;
  return false;
}

/** 🛡️ Renders the recorded violations of one run as a short grouped summary for the runner, empty when the run was clean. */
export function summarizeGuardReportV1(text: string, limit = 10): readonly string[] {
  const rows = text.split("\n").filter(line => line !== "").flatMap(line => { try { return [JSON.parse(line) as { kind: string; function: string; target: string; frame?: string }]; } catch { return []; } });
  if (rows.length === 0) return [];
  const counts = new Map<string, number>();
  for (const row of rows) counts.set(row.kind, (counts.get(row.kind) ?? 0) + 1);
  return [`[sandbox-guard] ${rows.length} violation(s): ${[...counts].map(([kind, count]) => `${kind}=${count}`).join(" ")}`, ...rows.slice(0, limit).map(row => `[sandbox-guard]   ${row.kind} ${row.function} ${row.target}${row.frame ? "  at " + row.frame : ""}`)];
}

/** 🛡️ Inserts the guard preload after the `test` verb of a bun test command when the guard is active and the preload exists next to this module; every other command is returned unchanged. */
export function guardArgumentsV1(command: string, args: readonly string[], env: NodeJS.ProcessEnv = process.env, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): readonly string[] {
  const mode = env[declared.environment.guard] ?? declared.guard.defaultMode, program = basename(command).replace(/\.exe$/iu, "");
  if (mode === "off" || args[0] !== "test" || !(program === "bun" || resolve(command) === resolve(process.execPath))) return args;
  const preload = fileURLToPath(new URL("./🔌️preload/🟦️.ts", import.meta.url));
  return existsSync(preload) && !args.includes("--preload") ? [args[0], "--preload", preload, ...args.slice(1)] : args;
}

/** 🛡️ Finds the real checkout a test command runs for: the declared environment, else the nearest ancestor holding a `.git` entry or a `.🧬semio` directory. */
export function testCheckoutRootV1(cwd: string, env: NodeJS.ProcessEnv = process.env, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): string | undefined {
  const given = env[declared.environment.checkout] ?? env.SEMIO_WORKSPACE_ROOT;
  if (given) return resolve(given);
  for (let directory = resolve(cwd); ; directory = dirname(directory)) {
    if (existsSync(join(directory, ".git")) || existsSync(join(directory, ".🧬semio"))) return directory;
    if (dirname(directory) === directory) return undefined;
  }
}
