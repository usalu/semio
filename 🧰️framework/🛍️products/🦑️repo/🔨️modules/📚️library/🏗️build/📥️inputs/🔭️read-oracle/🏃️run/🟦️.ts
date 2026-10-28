import {spawn, spawnSync} from "node:child_process";
import {existsSync} from "node:fs";
import {isAbsolute, win32} from "node:path";
import document from "../🔣️.json" with {type: "json"};

/** 🏃️ What a captured command did: exit status (124 on timeout), the tail of stdout and stderr and its wall time. */
export type CapturedV1 = {readonly exit: number; readonly output: string; readonly seconds: number; readonly timedOut: boolean};

const TAIL_BYTES = document.policy.tailBytes;

/** 🌐️ Fills the placeholders of one message of the document in one language; an unknown placeholder stays visible. */
export function readOracleMessageV1(language: "en" | "de", key: keyof typeof document.messages, values: Readonly<Record<string, string | number>> = {}): string {
  return document.messages[key][language].replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name] ?? `{${name}}`));
}

/** 🌍️ Renders one message in both languages, English first, joined by `separator`: for refusals raised before a language is known and for reasons that travel inside other messages. */
export function readOracleBothV1(key: keyof typeof document.messages, values: Readonly<Record<string, string | number>> = {}, separator = "\n"): string {
  return (["en", "de"] as const).map((language) => readOracleMessageV1(language, key, values)).join(separator);
}

/** 🧭️ What a command name stands for: the file to start, or the bilingual reason it must not be started. */
export type CommandResolutionV1 = {readonly path: string} | {readonly refusal: string};

/** 🔎️ Resolves a command the way a Windows shell does, because `CreateProcess` appends only `.exe`: every directory of `PATH` (case-insensitive key) is tried with each `PATHEXT` extension, and the first hit decides. An executable (`.exe`, `.com`) is started directly; a batch file (`.cmd`, `.bat`: `npm`, `yarn`, `pnpm` shims) cannot take arbitrary arguments safely without a shell, so it is refused in every language instead of failing as `ENOENT` and reading as a difference between the runs. Anything else, and every non-Windows host, keeps the name unchanged. */
export function resolveCommandV1(name: string, environment: Readonly<Record<string, string | undefined>>, platform: string = process.platform, exists: (path: string) => boolean = existsSync): CommandResolutionV1 {
  if (platform !== "win32") return {path: name};
  const value = (key: string): string | undefined => environment[Object.keys(environment).find((candidate) => candidate.toLowerCase() === key.toLowerCase()) ?? key];
  const extensions = (value("PATHEXT") ?? ".COM;.EXE;.BAT;.CMD").split(";").filter(Boolean).map((extension) => extension.toLowerCase());
  const hasExtension = win32.extname(name) !== "";
  const directories = /[\\/]/u.test(name) || win32.isAbsolute(name) || isAbsolute(name) ? [""] : (value("PATH") ?? "").split(";").filter(Boolean);
  for (const directory of directories) {
    const base = directory === "" ? name : win32.join(directory, name);
    for (const candidate of [...(hasExtension ? [base] : []), ...extensions.map((extension) => base + extension)]) {
      if (!exists(candidate)) continue;
      const extension = win32.extname(candidate).toLowerCase();
      if (extension === ".cmd" || extension === ".bat") return {refusal: (["en", "de"] as const).map((language) => document.messages.batchCommand[language].replace(/\{(\w+)\}/gu, (_, key: string) => ({command: name, path: candidate} as Record<string, string>)[key] ?? "")).join("\n")};
      return {path: candidate};
    }
  }
  return {path: name};
}

/** 🚀️ Runs a command with bounded memory (only the output tail is kept) and a deadline; on timeout the whole process tree is killed (a process group on POSIX, `taskkill /T` on Windows). */
export function captureV1(command: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string | undefined>>; readonly timeoutMs: number; readonly signal?: AbortSignal; readonly resolution?: (name: string) => CommandResolutionV1}): Promise<CapturedV1> {
  return new Promise((resolve) => {
    const started = Date.now(), windows = process.platform === "win32", resolved = (options.resolution ?? ((name) => resolveCommandV1(name, options.env)))(command[0]!);
    if ("refusal" in resolved) { resolve({exit: 127, output: resolved.refusal, seconds: 0, timedOut: false}); return; }
    const child = spawn(resolved.path, command.slice(1), {cwd: options.cwd, env: options.env as NodeJS.ProcessEnv, detached: !windows, stdio: ["ignore", "pipe", "pipe"]});
    let tail = "", timedOut = false;
    const keep = (chunk: Buffer) => { tail = (tail + chunk.toString("utf8")).slice(-TAIL_BYTES); };
    child.stdout.on("data", keep);
    child.stderr.on("data", keep);
    const stop = () => {
      timedOut = true;
      if (windows) spawnSync("taskkill", ["/T", "/F", "/PID", String(child.pid)]);
      else try { process.kill(-child.pid!, "SIGKILL"); } catch { child.kill("SIGKILL"); }
    };
    const timer = setTimeout(stop, options.timeoutMs);
    options.signal?.addEventListener("abort", stop, {once: true});
    if (options.signal?.aborted) stop();
    const finish = (exit: number, extra = "") => { clearTimeout(timer); resolve({exit: timedOut ? 124 : exit, output: tail + extra, seconds: Math.round((Date.now() - started) / 100) / 10, timedOut}); };
    child.on("error", (error) => finish(127, `\n${error.message}`));
    child.on("close", (code, signal) => finish(code ?? (signal ? 128 : 1)));
  });
}
