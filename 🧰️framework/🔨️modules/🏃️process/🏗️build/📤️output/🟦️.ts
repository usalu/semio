import { Writable } from "node:stream";
import protocol from "./🔣️.json" with { type: "json" };

/** 🔌️ Accepts text for the result stream and reports when it was delivered; the shape of a Node stream without its type. */
export interface MachineOutputSinkV1 { write(text: string | Uint8Array, complete?: (error?: Error | null) => void): unknown }

type ContractV1 = { result: MachineOutputSinkV1; restore: () => void };

const holder = globalThis as unknown as Record<symbol, ContractV1 | undefined>;
const key = Symbol.for(protocol.contractKey);

/** 🔎️ Tells whether an invocation asks for machine output: the machine flag before the argument separator. */
export function machineOutputRequestedV1(args: readonly string[]): boolean {
  const boundary = args.indexOf(protocol.separator);
  return (boundary < 0 ? args : args.slice(0, boundary)).includes(protocol.machineFlag);
}

/**
 * 🛡️ Enforces the output contract for the whole process: under the machine flag every `process.stdout.write` and listed `console` method is diverted to the diagnostic stream, and only the result writer keeps the real result stream. Installing again is a no-op.
 * @see ./🧬️schema/🔣️.json the contract this is written first in
 */
export function installOutputContractV1(args: readonly string[]): boolean {
  if (!machineOutputRequestedV1(args)) return false;
  if (holder[key]) return true;
  const stdout = process.stdout, stderr = process.stderr, realWrite = stdout.write.bind(stdout) as (...parameters: unknown[]) => boolean, errorWrite = stderr.write.bind(stderr) as (...parameters: unknown[]) => boolean;
  const consoleMethods = console as unknown as Record<string, unknown>, original = new Map<string, unknown>();
  for (const name of protocol.divertedConsole) { original.set(name, consoleMethods[name]); consoleMethods[name] = (...values: unknown[]) => console.error(...values); }
  stdout.write = ((chunk: unknown, ...rest: unknown[]) => errorWrite(chunk, ...rest)) as typeof stdout.write;
  holder[key] = { result: { write: (text, complete) => realWrite(text, complete) }, restore: () => { stdout.write = realWrite as typeof stdout.write; for (const [name, method] of original) consoleMethods[name] = method; holder[key] = undefined; } };
  return true;
}

/** ↩️ Gives the process writers back; used by tests that install the contract in their own process. */
export function restoreOutputContractV1(): void { holder[key]?.restore(); }

/** 📤️ The only way to reach the result stream: the real standard output, also while the contract diverts every other writer. */
export const machineOutputV1: MachineOutputSinkV1 = { write: (text, complete) => (holder[key]?.result ?? (process.stdout as unknown as MachineOutputSinkV1)).write(text, complete) };

/** 🌊️ The result stream as a Node writable: a forwarded child result (`pipe`) reaches the real standard output also while the contract diverts `process.stdout.write`. */
export function resultStreamV1(): Writable { return new Writable({ write(chunk, _encoding, done) { const real = process.stdout as NodeJS.EventEmitter; let settled = false; const finish = (error?: Error | null): void => { if (settled) return; settled = true; if (!error) real.removeListener("error", finish); done(error ?? undefined); }; real.once("error", finish); try { machineOutputV1.write(chunk, finish); } catch (error) { finish(error as Error); } } }); }

/** 🧾️ Writes one line of the result to the result stream. */
export function writeResultLineV1(line: string): void { machineOutputV1.write(line + "\n"); }

/** 📣️ Writes one diagnostic line (progress, waiting, log) to the diagnostic stream whatever the mode. */
export function writeDiagnosticLineV1(line: string): void { process.stderr.write(line + "\n"); }
