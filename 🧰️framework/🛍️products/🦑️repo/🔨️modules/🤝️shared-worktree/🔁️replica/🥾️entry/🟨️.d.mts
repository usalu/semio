/** 🥾️ Declares the boot program of the replica daemon for its TypeScript consumers; the boot program itself stays a plain script that is installed outside every worktree. */
export const bootVersionV1: number;
/** 🧭️ Where the module lives below the repository root. */
export const moduleRelativeV1: string;
/** ⏱️ The limits of the boot loop in milliseconds. */
export interface EntryLimitsV1 { readonly earlyMs: number; readonly readyMs: number; readonly silentMs: number; readonly killMs: number; readonly graceMs: number; readonly pollMs: number; readonly backoffMs: number; readonly backoffMaxMs: number; readonly storeMs: number; readonly refuseMs: number; }
/** 🥾️ What one boot loop needs: the installed entry directory, the worktree, the module path below it, the store root, the environment of the daemon, the limits, the daemon arguments and the log sink. */
export interface BootContextV1 { readonly entry: string; readonly worktree: string; readonly moduleRel: string; readonly storeRoot: () => string | undefined; readonly environment: Readonly<Record<string, string | undefined>>; readonly limits: EntryLimitsV1; readonly args: readonly string[]; readonly say: (line: string) => unknown; }
/** 📂️ The per-account directory of the installed boot program. */
export function entryDirectoryV1(env?: Readonly<Record<string, string | undefined>>, platform?: string, home?: string): string;
/** ⏱️ The limits in milliseconds, each overridable by `SEMIO_FLEET_ENTRY_<NAME>`. */
export function limitsV1(env?: Readonly<Record<string, string | undefined>>): EntryLimitsV1;
/** 🥾️ The boot loop: it resolves to the exit code of the boot program once it is stopped. */
export function runBootV1(context: BootContextV1, signal: AbortSignal): Promise<number>;
/** 🧪️ Proves the boot program on throwaway directories; resolves to `undefined` when it holds, else to the reason. */
export function selfTestV1(): Promise<string | undefined>;
/** 🔍️ Checks the installed files against `entry.json`: the list of problems, empty when the installation is intact. */
export function verifyV1(entry: string): string[];
/** 📥️ The outcome of one installation. */
export interface InstallResultV1 { readonly action: string; readonly version: number; readonly reason?: string; readonly installed?: number; readonly changed?: boolean; }
/** 📥️ Installs the boot program of this directory into `entry`. */
export function installV1(options: Readonly<{ entry: string; worktree: string; store?: string | null; moduleRel?: string; takeOver?: boolean; from?: string; now?: number }>): Promise<InstallResultV1>;
