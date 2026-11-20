import { closeSync, openSync, writeFileSync, writeSync } from "node:fs";
import { setTimeout as pause } from "node:timers/promises";
import { formatStampProtocolV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";

/** 🪵 Names each job log distinctly on case-insensitive filesystems. */
export function nativeBuildJobLogNamesV1(ids: readonly string[]): ReadonlyMap<string, string> {
  const used = new Set<string>();
  return new Map(ids.map((id, index) => {
    const key = id.toLowerCase(), suffix = used.has(key) ? `.j${index}` : "";
    used.add(key);
    return [id, `job-${id}.log${suffix}`];
  }));
}

/** 🔢 Identifies the attempt with a suffix after its base log name. */
export function nativeBuildJobAttemptLogPathV1(base: string, attempt: number): string {
  return attempt === 1 ? base : `${base}.${attempt}`;
}

type JobAttempts = Readonly<{
  id: string;
  logPath: string;
  signal: AbortSignal;
  check: () => void;
  log: (message: string) => void;
  logLimitBytes: number;
  tailLines: number;
  tailLineChars: number;
}>;

/** 🔁 Preserves bounded output for each settled native attempt and waits for the shared restart backoff under caller cancellation. */
export async function runNativeBuildJobAttemptsV1(options: JobAttempts, run: (capture: (line: string) => void, attempt: number) => Promise<number>): Promise<void> {
  const restart = formatStampProtocolV1();
  for (let attempt = 1;; attempt++) {
    options.check();
    const path = nativeBuildJobAttemptLogPathV1(options.logPath, attempt);
    const output = openSync(path, "wx"), tail: string[] = [];
    let written = 0, exit: number, primary: { error: unknown } | undefined;
    try {
      exit = await run(line => {
        if (written < options.logLimitBytes) {
          const bytes = Buffer.allocUnsafe(Math.min(Buffer.byteLength(line) + 1, options.logLimitBytes - written));
          let length = bytes.write(line);
          if (length < bytes.length) bytes[length++] = 10;
          written += writeSync(output, bytes.subarray(0, length));
        }
        tail.push(line.slice(0, options.tailLineChars));
        if (tail.length > options.tailLines) tail.shift();
      }, attempt);
    } catch (error) {
      primary = { error };
      throw error;
    } finally {
      try { closeSync(output); }
      catch (error) {
        if (primary && !Object.is(primary.error, error)) throw new AggregateError([primary.error, error], "Native job and log close failed");
        throw error;
      }
    }
    if (exit === 0) return;
    const tailPath = path + ".tail";
    writeFileSync(tailPath, tail.length ? tail.join("\n") + "\n" : "", { flag: "wx" });
    const location = `attempt ${attempt}, log ${path}, tail ${tailPath}`;
    const late = tail.slice(-40).join("\n");
    if (exit !== restart.exitCode || attempt > restart.maximumReexecutions) throw Error(`exit ${exit} (${location})\n${late}`);
    const backoffs = restart.editFailure.backoffMs;
    const delay = backoffs[Math.min(attempt - 1, backoffs.length - 1)]!;
    options.log(`[native-build] job:${options.id} asked for a restart (exit ${exit}, ${location}); starting it again in ${delay} ms (${attempt} of ${restart.maximumReexecutions})\n${late}`);
    try { await pause(delay, undefined, { signal: options.signal }); }
    catch (error) { options.signal.throwIfAborted(); throw error; }
  }
}
