import { appendFileSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { dirname } from "node:path";
import { Worker } from "node:worker_threads";
import { join } from "node:path";
import { OWNER_POOL_POLICY_V1 as P, ownerPoolFitsV1, ownerPoolHeavyV1, ownerPoolRecycleV1, pullOwnerPoolV1 } from "../🟦️.ts";
import { fleetGlobalsKeyV1 } from "../../📥️take/🟨️.mjs";
import { ownerPoolReadFleetV1, ownerPoolTurnMillisecondsV1 } from "../🤝️fleet/🟦️.ts";

const MEGABYTE = 1048576, megabytes = () => Math.round(process.memoryUsage().rss / MEGABYTE);

/** 🧒️ What a child process of the pool ends with: the wave is stored by somebody (0), it passed its memory or age and is replaced (75), the record code differs from the wave's (76), it could not open the wave (77) or the supervisor asked it to stop (78). */
export const OWNER_POOL_CHILD_EXIT_V1 = Object.freeze({ finished: 0, recycled: P.recycleExitCode, revision: 76, failed: 77, drained: 78 });

/** 🧒️ One child of the pool: ONE process, ONE owner at a time (an owner is CPU bound JavaScript, so the cores of a machine are used by processes and not by slots of one event loop), which opens the wave under this machine's root once, walks the list from its own start (see {@link pullOwnerPoolV1}), claims, infers and publishes, and is replaced by the supervisor when its resident memory passed `recycleMegabytes` after an owner or its age passed the lifetime (the allocator never returns memory, so a fresh process is the only way to give it back). It reports one JSON object per line (`ready`, `beat`, `done`, `end`: resident size, the cpu and wall time of every owner, the claim statistics); a line `drain` on its input, or the end of its input (the supervisor died), makes it stop after the owner in flight, and a watchdog thread (see the watchdog member) ends the process within seconds when its pool is gone or no longer works its wave, even while an owner holds its event loop. `driver` is the module that opens the inference (the real one unless a test brings another). */
export async function runOwnerPoolChildV1({ args, write, input, environment = process.env }) {
  const opened = Date.now();
  const head = JSON.parse(readFileSync(join(args.dir, "head.json"), "utf8")), units = JSON.parse(readFileSync(join(args.dir, "units.json"), "utf8")), session = readFileSync(join(args.dir, "session.bin"));
  const driver = await (args.driver === undefined || args.driver === null ? import("../🔬️inference/🟨️.mjs").then(module => module.openOwnerPoolInferenceV1) : import(args.driver).then(module => module.openOwnerPoolInferenceV1));
  let opening;
  try { opening = await driver({ root: args.root, fleetRoot: args.fleetRoot ?? args.root, environment, session, globals: head.globals, units }); }
  catch (error) { write({ t: "end", reason: "open", message: String(error?.message ?? error).slice(0, 300) }); return OWNER_POOL_CHILD_EXIT_V1.failed; }
  if (opening.revision !== undefined && head.revision !== undefined && head.revision !== opening.revision) { write({ t: "end", reason: "revision", revision: opening.revision }); opening.close?.(); return OWNER_POOL_CHILD_EXIT_V1.revision; }
  const known = new Set(), presence = args.presence, load = () => { try { for (const line of readFileSync(presence, "utf8").split("\n")) { const at = line.indexOf(" "); if (at > 0 && Date.now() - Number(line.slice(0, at)) < P.presenceMaximumAgeMilliseconds) known.add(line.slice(at + 1)); } } catch {} };
  if (presence !== undefined) { try { mkdirSync(dirname(presence), { recursive: true }); } catch {} load(); }
  const published = typeof args.store === "string" && typeof args.origin === "string" ? join(args.store, "v1", "origins", args.origin, "pool-published-" + String(args.dir).split("/").pop().slice(0, 12) + "-" + fleetGlobalsKeyV1(head.globals) + ".txt") : undefined;
  let drain = false, recycle = false, running = 0, cpu = process.cpuUsage(), latest;
  const leave = () => { drain = true; setTimeout(() => process.exit(OWNER_POOL_CHILD_EXIT_V1.drained), P.drainExitMilliseconds).unref(); };
  input?.on("data", chunk => { if (String(chunk).includes("drain")) leave(); });
  input?.on("end", leave);
  const readyRss = megabytes();
  write({ t: "ready", revision: opening.revision, pid: process.pid, rss: megabytes(), openMilliseconds: Date.now() - opened, units: units.length });
  const beat = setInterval(() => { load(); write({ t: "beat", rss: megabytes(), running, stats: latest, port: opening.stats?.(), known: known.size }); }, P.beatMilliseconds);
  const began = Date.now(), over = () => drain || recycle || Date.now() - began > (args.lifetimeMilliseconds ?? P.childLifetimeMilliseconds);
  let stats, fleet = [], fleetAt = -Infinity;
  try {
    stats = await pullOwnerPoolV1({
      units: () => units, known, stored: unit => { if (presence !== undefined) { try { appendFileSync(presence, Date.now() + " " + unit.name + "\n"); } catch {} } if (published !== undefined) { try { appendFileSync(published, unit.name + "\n"); } catch {} } }, fits: unit => ownerPoolFitsV1(unit, args.grantMegabytes ?? Infinity), turn: unit => { if (!ownerPoolHeavyV1(unit) || args.origin === undefined) return 0; if (Date.now() - fleetAt > 5000) { fleet = ownerPoolReadFleetV1(args.store); fleetAt = Date.now(); } return ownerPoolTurnMillisecondsV1(args.origin, fleet, unit.megabytes ?? 0, Date.now()) ?? 0; }, puller: args.puller, slots: () => (over() ? 0 : 1), stop: over,
      claim: async unit => { await opening.resend?.().catch(() => undefined); return opening.claim(unit); }, run: unit => opening.run(unit), fail: (name, message, deterministic, code) => { write({ t: "fail", name, message: message.slice(0, 300), deterministic, code: code ?? "" }); opening.fail?.(name, message, deterministic); },
      busy: count => { running = count; }, progress: stats => { latest = { ...stats }; },
      done: (unit, ok, milliseconds) => {
        const used = process.cpuUsage(cpu); cpu = process.cpuUsage();
        const rss = megabytes(); if (ownerPoolRecycleV1({ rss, readyRss, recycleMegabytes: args.recycleMegabytes ?? Infinity })) recycle = true;
        write({ t: "done", name: unit.name, kind: unit.kind, ok, milliseconds, cpuMilliseconds: Math.round((used.user + used.system) / 1000), rss, peak: Math.round(process.resourceUsage().maxRSS / 1024) });
      },
    });
  } finally { clearInterval(beat); await Promise.race([opening.resend?.({ force: true }).catch(() => undefined), new Promise(done => setTimeout(done, P.resendFinalMilliseconds))]); opening.close?.(); }
  const reason = recycle ? "recycled" : drain ? "drained" : Date.now() - began > (args.lifetimeMilliseconds ?? P.childLifetimeMilliseconds) ? "recycled" : "finished";
  write({ t: "end", reason, stats, port: opening.stats?.(), rss: megabytes() });
  return reason === "finished" ? 0 : reason === "recycled" ? OWNER_POOL_CHILD_EXIT_V1.recycled : OWNER_POOL_CHILD_EXIT_V1.drained;
}

if (import.meta.main) {
  const args = JSON.parse(readFileSync(process.argv[2], "utf8")), write = value => { try { process.stdout.write(JSON.stringify(value) + "\n"); } catch {} };
  process.stdout.on("error", () => {});
  if (args.parentPid !== undefined) { try { new Worker(new URL("../🐕️watchdog/🟨️.mjs", import.meta.url), { workerData: { parentPid: args.parentPid, wave: args.waveId, lease: args.lease, exitFile: args.exitFile, lostGraceMilliseconds: args.lostGraceMilliseconds, staleMilliseconds: args.staleMilliseconds, watchMilliseconds: args.watchMilliseconds, graceMilliseconds: args.graceMilliseconds } }).unref(); } catch {} }
  process.stdin.resume();
  const code = await runOwnerPoolChildV1({ args, write, input: process.stdin });
  if (args.registry !== undefined) { try { rmSync(join(args.registry, process.pid + ".json"), { force: true }); } catch {} }
  process.exit(code);
}
