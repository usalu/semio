import { tmpdir } from "node:os";
import { join } from "node:path";
import { ownProcessIdentityV1, pinSpawnedProcessV1, readProcessStartV1 } from "../../🔖️identity/🟦️.ts";
import { ownedKillLedgerWriterV1, ownedKillRecordLineV1, terminateOwnedProcessTreeGracefullyV1 } from "../🟦️.ts";
import { parseReaperLineV1, REAPER_POLICY_V1 } from "./🟦️.ts";
import { createJobTableV1, windowsJobKernelV1 } from "./🪟️job/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🧹️ The sentinel: reads registrations from its stdin and, when the pipe closes, stops what is still registered by pid and creation time.
 * @see ./🧬️schema/🔣️.json */
const parent = Number(process.argv[2]);
const pinned = new Map<number, number>();
let buffer = "", reaping = false;

const log = ownedKillLedgerWriterV1(join(tmpdir(), REAPER_POLICY_V1.directoryName, `${parent}.log`));

const kernel = await windowsJobKernelV1(), jobs = kernel === undefined ? undefined : createJobTableV1(kernel);
const say = (key: keyof typeof schema.definitions.Texts.const.en, pid: number, cause = ""): void => log(["en", "de"].map((language) => schema.definitions.Texts.const[language as "en" | "de"][key].replace("{pid}", String(pid)).replace("{cause}", cause).replace("{parent}", String(parent))).join(" / "));

const register = (pid: number, spawnedMs: number): void => {
  const read = readProcessStartV1(pid);
  const identity = typeof read === "number" ? pinSpawnedProcessV1({ pid, spawnedMs }, read) : undefined;
  if (identity === undefined) return;
  pinned.set(pid, identity.startMs);
  if (process.platform === "win32" && jobs?.add(pid) !== true) say("job-unavailable", pid, jobs === undefined ? "bun:ffi" : "assign");
};

const terminateOne = (pid: number, startMs: number | undefined): Promise<void> => {
  const sender = ownProcessIdentityV1();
  if (jobs?.stop(pid) === true) say("job-stopped", pid);
  return terminateOwnedProcessTreeGracefullyV1(pid, {
    ...startMs === undefined ? {} : { rootStartMs: startMs },
    graceMilliseconds: REAPER_POLICY_V1.graceMilliseconds,
    record: (row, signal) => log(ownedKillRecordLineV1({ pid: row.pid, signal, killer: sender.pid, label: "reaper", reason: ["en", "de"].map((language) => schema.definitions.Texts.const[language as "en" | "de"].reason.replace("{parent}", String(parent)).replace("{pid}", String(pid))).join(" / "), startMs: row.startMs, killerStartMs: sender.startMs })),
  }).catch(() => undefined);
};

const reap = async (): Promise<void> => {
  if (reaping) return;
  reaping = true;
  jobs?.stopAll();
  await Promise.all([...pinned].map(([pid, startMs]) => terminateOne(pid, startMs)));
  process.exit(0);
};

process.stdin.setEncoding("utf8");
process.stdin.on("data", (chunk: string) => {
  buffer += chunk;
  for (let end = buffer.indexOf("\n"); end >= 0; end = buffer.indexOf("\n")) {
    const line = parseReaperLineV1(buffer.slice(0, end));
    buffer = buffer.slice(end + 1);
    if (line?.kind === "add") register(line.pid, line.spawnedMs);
    else if (line?.kind === "remove") { pinned.delete(line.pid); jobs?.release(line.pid); }
    else if (line?.kind === "stop") { const startMs = pinned.get(line.pid); pinned.delete(line.pid); void terminateOne(line.pid, startMs); }
  }
});
process.stdin.on("end", () => { void reap(); });
process.stdin.on("close", () => { void reap(); });
process.stdin.on("error", () => { void reap(); });
for (const signal of ["SIGTERM", "SIGHUP", "SIGINT"] as const) process.on(signal, () => { void reap(); });
