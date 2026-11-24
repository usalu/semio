import{machinePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { availableParallelism, freemem, hostname, setPriority, tmpdir, totalmem } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { graphChildEnvironmentV1 } from "../../../../../🧠️memory/🐏️child-environment/🟨️.mjs";
import { graphFleetClientV1 } from "../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import { graphFleetSettingsV1 } from "../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import { remoteCacheFetchV1 } from "../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import { OWNER_POOL_POLICY_V1 as P, ownerPoolChildLimitsV1, ownerPoolAcceptsRevisionV1, ownerPoolChildMegabytesV1, ownerPoolKeepWaveV1, ownerPoolLiveChildrenV1, ownerPoolReopenMillisecondsV1, ownerPoolSizingV1, ownerPoolWaveOrderV1, ownerPoolWhyIdleV1, readOwnerPoolPointerV1, readOwnerPoolV1, writeOwnerPoolWaveFilesV1 } from "../🟦️.ts";
import { ownerPoolExitRecordV1, ownerPoolExitsAddV1 } from "../🚪️exits/🟦️.ts";
import { ownerPoolDrainPickV1, ownerPoolMayStartV1, ownerPoolSharelessStepV1, ownerPoolStartDelayV1 } from "../🧮️lifecycle/🟦️.ts";
import { ownerPoolProcessStartV1, ownerPoolSurvivorsV1, reapOwnerPoolOrphansV1, readOwnerPoolLeaseV1, writeOwnerPoolLeaseV1 } from "../🪦️reaper/🟦️.ts";
import { OWNER_POOL_STAGING_POLICY_V1, ownerPoolStagingIoV1, ownerPoolStagingSweepV1, writeOwnerPoolStagingOwnerV1 } from "../🧹️staging-sweep/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

const MEGABYTE = 1048576, T = schema.definitions.Texts.const, REASONS = { yielding: "reasonYielding", revision: "reasonRevision", "too-small": "reasonTooSmall", "no-wave": "reasonNoWave", "nothing-fits": "reasonNothingFits", pressure: "reasonPressure", waiting: "reasonWaiting", unshareable: "reasonUnshareable", starting: "reasonStarting", memory: "reasonMemory", cores: "reasonCores", off: "reasonOff", "no-current-wave": "reasonNoCurrentWave", "waves-done": "reasonWavesDone", "waves-backoff": "reasonWavesBackoff" };
const originOf = fleetRoot => { try { return JSON.parse(readFileSync(machinePathV1(fleetRoot, ".cache", "f", "machine.json"), "utf8")).originId; } catch { return undefined; } };
const sleep = (milliseconds, signal) => new Promise(done => { const timer = setTimeout(done, milliseconds); signal?.addEventListener("abort", () => { clearTimeout(timer); done(); }, { once: true }); });
const fill = (text, fields) => text.replace(/\{(\w+)\}/gu, (_all, name) => String(fields[name] ?? ""));

/** 🔔️ One message of the pool texts, English and German in one line. */
export const ownerPoolTextV1 = (key, fields = {}) => ["en", "de"].map(language => fill(T[language][key], fields)).join(" / ");

/** 🏠️ The capacity a machine has when nobody tells the pool more: its cores less one, and the free memory its operating system reports (a standalone puller or a test; the sidecar brings the host view of the capacity gate). */
export const defaultOwnerPoolCapacityV1 = async ({ children, childMegabytes }) => ownerPoolSizingV1({ host: { cores: availableParallelism(), totalMegabytes: Math.round(totalmem() / MEGABYTE), availableMegabytes: Math.round(freemem() / MEGABYTE), pressure: "normal", load: null, swapOutMegabytesPerSecond: null, foreground: { cpu: 1, unrealizedMegabytes: 0, waiters: 0 } }, children, childMegabytes });

/** 🌐️ The supervisor of the pool of this machine: it follows the waves the pointer lists (a new wave is added, none replaces the one a child works on), writes each wave once to a private directory, and keeps as many CHILD PROCESSES running as the machine's capacity allows (`capacity` is asked every `targetPollMilliseconds`; at most two children start per look and one is drained per look, so a change of the host's load moves the pool smoothly). It claims, infers and publishes nothing itself: its memory stays small whatever the owners take, a child that passed its memory limit after an owner is replaced by the child itself, one that passed the hard limit is killed, and a supervisor that ends takes its children with it: it heartbeats a lease (the pool, the waves it works) that every child watches from a thread of its own, so a child ends within seconds of a pool that is gone, replaced or no longer working its wave, and a starting pool ends the children of the pools before it (records `child-<n>.json` with pid and process start time) before it measures the capacity; children that are still alive are counted in the load they cause but never as capacity the pool already uses. It schedules nothing and waits for nobody. */
export async function runOwnerPoolV1({ totalMegabytes, root, environment, signal, say = () => {}, capacity = defaultOwnerPoolCapacityV1, siblings = 0, fleetRoot = root, work, status, pollMilliseconds = P.pointerPollMilliseconds, targetPollMilliseconds = P.targetPollMilliseconds, once = false, driver, childEntry = fileURLToPath(new URL("../🧒️child/🟨️.mjs", import.meta.url)), spawn = Bun.spawn, now = Date.now, priority = P.childPriority, observe, revision, starved, ramp = true }) {
  const settings = environment[P.environmentOff] === "off" ? undefined : graphFleetSettingsV1(fleetRoot, environment);
  const reasonOf = (key, fields = {}, textKey = key) => ({ reason: key, reasonText: textKey === null || REASONS[textKey] === undefined ? null : { en: fill(T.en[REASONS[textKey]], fields), de: fill(T.de[REASONS[textKey]], fields) } });
  if (settings === undefined) { status?.update({ state: "off", ...reasonOf("off") }); return; }
  const total = totalMegabytes ?? Math.round(totalmem() / MEGABYTE);
  status?.update({ state: "idle", siblings, totalMegabytes: total, grantMegabytes: ownerPoolChildLimitsV1(total).hardMegabytes });
  if (total < P.minimumTotalMegabytes) { status?.update({ state: "too-small", ...reasonOf("too-small") }); say(ownerPoolTextV1("tooSmall", { megabytes: total, minimum: P.minimumTotalMegabytes })); return; }
  const client = graphFleetClientV1(settings, { fetch: remoteCacheFetchV1(environment, { root: fleetRoot }) }), limits = ownerPoolChildLimitsV1(total), base = mkdtempSync(join(realpathSync(tmpdir()), OWNER_POOL_STAGING_POLICY_V1.prefix));
  writeOwnerPoolStagingOwnerV1(base);
  ownerPoolStagingSweepV1({ parents: [realpathSync(tmpdir()), machinePathV1(fleetRoot, ".cache", "f", P.stagingFallbackDirectory)], own: base, io: ownerPoolStagingIoV1 });
  const kept = machinePathV1(fleetRoot, ".cache", "f", "owner-pool-waves"), waves = new Map(), children = new Set(), samples = [], finishedAt = [], totals = { claimed: 0, published: 0, stored: 0, held: 0, failed: 0, attempts: 0, recycled: 0 }, failures = {},  portTotals = { diverged: 0, resent: 0, kept: 0, shed: 0 };
  let shareless = { count: 0, until: 0 };
  const shareStep = (event, at = now()) => { const step = ownerPoolSharelessStepV1(shareless, event, at); shareless = step.state; return step.tripped; };
  let exitState = { exits: {}, lastExits: [] }, lastStartMs = -Infinity, lastPointerAt = -Infinity, lastStarvedAt = -Infinity, pointerWaves = [], pointerOk = false, learned, tick = 0;
  const mine = () => learned, wanted = () => learned ?? (typeof revision === "function" ? revision() : revision), judge = wave => { if (wave.state !== "declined" && !ownerPoolAcceptsRevisionV1(wave.head?.revision, mine())) wave.state = "declined"; return wave; };
  const add = stats => { for (const key of ["claimed", "published", "stored", "held", "failed", "attempts"]) totals[key] += stats?.[key] ?? 0; };
  const live = () => { const sum = { ...totals }; for (const child of children) for (const key of ["claimed", "published", "stored", "held", "failed", "attempts"]) sum[key] += child.stats?.[key] ?? 0; return sum; };
  const loadWave = async wave => {
    const manifest = await readOwnerPoolV1(client, wave.id);
    if (manifest === undefined) return false;
    const dir = writeOwnerPoolWaveFilesV1([join(kept, wave.id.slice(0, 16)), join(base, wave.id.slice(0, 16))], { "head.json": JSON.stringify(manifest.head), "session.bin": manifest.session, "units.json": JSON.stringify(manifest.units) }, { mkdir: path => mkdirSync(path, { recursive: true }), write: writeFileSync, remove: path => rmSync(path, { recursive: true, force: true }) });
    Object.assign(wave, { dir, head: manifest.head, units: manifest.units.length, state: "open", children: 0, backoffUntil: 0 });
    judge(wave);
    return true;
  };
  const adopt = () => {
    try {
      const found = readdirSync(kept).map(name => { try { const dir = join(kept, name), head = JSON.parse(readFileSync(join(dir, "head.json"), "utf8")); return existsSync(join(dir, "session.bin")) && existsSync(join(dir, "units.json")) && /^[0-9a-f]{64}$/u.test(head.id) && head.id.startsWith(name) ? { id: head.id, createdMs: head.createdMs, state: "open", dir, head, units: JSON.parse(readFileSync(join(dir, "units.json"), "utf8")).length, children: 0, backoffUntil: 0 } : undefined; } catch { return undefined; } }).filter(Boolean).sort((a, b) => b.createdMs - a.createdMs);
      for (const wave of found.slice(0, P.wavesRetained)) { waves.set(wave.id, judge(wave)); say(ownerPoolTextV1("joined", { id: wave.id.slice(0, 12), units: wave.units, children: 0 })); }
      for (const wave of found.slice(P.wavesRetained)) rmSync(wave.dir, { recursive: true, force: true });
    } catch {}
  };
  const refresh = async () => {
    let pointer = await readOwnerPoolPointerV1(client, now);
    for (let attempt = 0; pointer === "fault" && attempt < P.pointerRetryMilliseconds.length && !signal.aborted; attempt++) { await sleep(Math.max(P.pointerRetryMilliseconds[attempt], Math.min(client.breakerRemainingMilliseconds?.() ?? 0, P.pointerBreakerWaitMaximumMilliseconds)), signal); pointer = await readOwnerPoolPointerV1(client, now); }
    pointerOk = pointer !== "fault";
    if (pointer === "fault") { status?.update({ lastError: ownerPoolTextV1("cacheFault") }); return; }
    pointerWaves = pointer?.waves ?? [];
    const ids = new Set(pointerWaves.map(wave => wave.id));
    for (const wave of pointerWaves) if (!waves.has(wave.id)) { const entry = { id: wave.id, createdMs: wave.createdMs, state: "loading", children: 0, units: wave.units, backoffUntil: 0 }; if (await loadWave(entry)) { waves.set(wave.id, entry); say(ownerPoolTextV1("joined", { id: wave.id.slice(0, 12), units: wave.units, children: children.size })); } }
    for (const [id, wave] of waves) if (!ownerPoolKeepWaveV1({ listed: ids.has(id), anyListed: ids.size > 0, children: wave.children, declined: wave.state === "declined" })) { rmSync(wave.dir, { recursive: true, force: true }); waves.delete(id); }
  };
  const active = () => [...waves.values()].map(wave => { judge(wave); if (wave.state === "done" && now() - wave.doneAt >= ownerPoolReopenMillisecondsV1(wave.createdMs, now())) wave.state = "open"; return wave; }).filter(wave => wave.state === "open" && wave.backoffUntil <= now()).sort((a, b) => b.createdMs - a.createdMs).slice(0, P.wavesActive);
  const childMegabytes = () => ownerPoolChildMegabytesV1(samples, [...children].filter(child => child.ready && child.rss > 0).map(child => child.rss));
  const registry = machinePathV1(fleetRoot, ".cache", "f", P.childRegistryDirectory), leaseFile = join(registry, P.leaseFileName), selfStart = ownerPoolProcessStartV1(process.pid);
  const ownPids = () => new Set([...children].map(child => child.proc.pid));
  const orphans = () => { try { return ownerPoolSurvivorsV1(kept, registry, ownPids()); } catch { return 0; } };
  const reap = async () => { try { return await reapOwnerPoolOrphansV1({ kept, registry, own: ownPids(), self: process.pid }); } catch { return []; } };
  const heartbeat = () => { try { mkdirSync(registry, { recursive: true }); writeOwnerPoolLeaseV1(leaseFile, { version: 1, pid: process.pid, ...(selfStart === undefined ? {} : { startTime: selfStart }), beatMs: Date.now(), waves: [...waves.values()].filter(wave => wave.state !== "declined").map(wave => wave.id) }); } catch {} };
  const launch = wave => {
    const file = join(wave.dir, `child-${++tick}.json`), puller = `${hostname()}:${process.pid}:${tick}:${now()}`;
    writeFileSync(file, JSON.stringify({ store: environment.SEMIO_FLEET_STORE_ROOT, origin: originOf(fleetRoot), dir: wave.dir, root, fleetRoot, puller, recycleMegabytes: limits.recycleMegabytes, grantMegabytes: limits.hardMegabytes, lifetimeMilliseconds: P.childLifetimeMilliseconds, driver: driver ?? null, presence: machinePathV1(fleetRoot, ".cache", "f", P.presenceDirectory, wave.id.slice(0, 16) + ".txt"), registry, waveId: wave.id, parentPid: process.pid, lease: leaseFile, exitFile: file.replace(/\.json$/u, ".exit.json") }));
    lastStartMs = now();
    const proc = spawn([process.execPath, childEntry, file], { detached: true, windowsHide: true, stdin: "pipe", stdout: "pipe", stderr: "inherit", env: { ...environment, ...graphChildEnvironmentV1(), [P.childEnvironment]: "1" } });
    try { setPriority(proc.pid, priority); } catch {}
    const startTime = ownerPoolProcessStartV1(proc.pid), identity = { pid: proc.pid, ...(startTime === undefined ? {} : { startTime }), parentPid: process.pid, ...(selfStart === undefined ? {} : { parentStart: selfStart }) };
    try { mkdirSync(registry, { recursive: true }); writeFileSync(join(registry, proc.pid + ".json"), JSON.stringify({ ...identity, wave: wave.id, startedMs: now() })); } catch {}
    try { const arguments_ = JSON.parse(readFileSync(file, "utf8")); writeFileSync(file + ".tmp", JSON.stringify({ ...arguments_, ...identity })); renameSync(file + ".tmp", file); } catch {}
    const child = { id: tick, proc, wave, startedMs: now(), exitFile: file.replace(/\.json$/u, ".exit.json"), ready: false, owners: 0, rss: 0, running: 0, stats: undefined, port: undefined, drained: false, killed: false };
    children.add(child); wave.children++;
    void (async () => {
      const decoder = new TextDecoder(); let rest = "";
      try { for await (const chunk of proc.stdout) { rest += decoder.decode(chunk, { stream: true }); let at; while ((at = rest.indexOf("\n")) >= 0) { const line = rest.slice(0, at); rest = rest.slice(at + 1); try { onLine(child, JSON.parse(line)); } catch {} } } } catch {}
    })();
    void proc.exited.then(code => { onExit(child, code); });
    return child;
  };
  const onLine = (child, line) => {
    observe?.(child.id, line, now());
    if (typeof line.rss === "number") { child.rss = line.rss; if (line.rss > limits.hardMegabytes && !child.killed) { child.killed = true; try { child.proc.kill("SIGKILL"); } catch {} status?.update({ lastError: `a child held ${line.rss} MB, above ${limits.hardMegabytes} MB, and was stopped` }); } }
    if (typeof line.revision === "string") learned = line.revision;
    if (line.t === "ready") child.ready = true;
    if (line.t === "beat") { child.running = line.running ?? 0; if (line.stats) child.stats = line.stats; if (line.port) child.port = line.port; }
    else if (line.t === "done") { child.owners++; if (line.ok) { finishedAt.push(now()); shareStep("published"); } if (line.rss > 0) samples.push(line.rss); if (samples.length > 64) samples.splice(0, samples.length - 64); }
    else if (line.t === "fail") { if (typeof line.code === "string" && line.code !== "") failures[line.code] = (failures[line.code] ?? 0) + 1; if (line.code === "unshareable" && shareStep("unshareable")) { say(ownerPoolTextV1("reasonUnshareable", { count: shareless.count, minutes: Math.round(P.unshareablePauseMilliseconds / 60000) })); } status?.update({ lastError: `${line.name}: ${line.message}`.slice(0, 300), failures: { ...failures } }); }
    else if (line.t === "end") { child.stats = line.stats ?? child.stats; child.port = line.port ?? child.port; child.reason = line.reason; child.message = line.message; }
  };
  const record = (child, code, target) => {
    let watchdog;
    try { watchdog = JSON.parse(readFileSync(child.exitFile, "utf8")).verdict; rmSync(child.exitFile, { force: true }); } catch {}
    const exit = ownerPoolExitRecordV1({ pid: child.proc.pid, code: code ?? null, signal: child.proc.signalCode ?? null, reason: child.reason, drained: child.drained, killed: child.killed, watchdog, startedMs: child.startedMs, now: now(), owners: child.owners, target: target?.children, why: target?.reason });
    exitState = ownerPoolExitsAddV1(exitState, exit, P.exitsKept);
    status?.update({ exits: exitState.exits, lastExits: exitState.lastExits }); say(exit.text.en + " / " + exit.text.de);
  };
  let lastTarget;
  const onExit = (child, code) => {
    record(child, code, lastTarget);
    children.delete(child); child.wave.children--; add(child.stats); for (const key of Object.keys(portTotals)) portTotals[key] += child.port?.[key] ?? 0;
    if (child.reason === "recycled" || code === P.recycleExitCode) { totals.recycled++; say(ownerPoolTextV1("recycled", { megabytes: child.rss, limit: limits.recycleMegabytes })); }
    else if (code === 0) { if (child.wave.state === "open") child.wave.state = "finishing"; }
    else if (child.reason === "revision" || code === 76) { child.wave.state = "declined"; status?.update(reasonOf("revision")); say(ownerPoolTextV1("declined", { reason: "revision" })); }
    else if (code === 77 || child.reason === "open") { child.wave.backoffUntil = now() + 30000; status?.update({ lastError: (child.message ?? "a child could not open the wave").slice(0, 300) }); }
    else if (!child.drained && !child.killed) { if (now() - child.startedMs < 10000) child.wave.backoffUntil = now() + 30000; status?.update({ lastError: ownerPoolTextV1("childEnded", { reason: `exit ${code}` }).slice(0, 300) }); }
    if (child.wave.state === "finishing" && child.wave.children === 0) { child.wave.state = "done"; child.wave.doneAt = now(); }
  };
  const drain = child => { if (child.drained) return; child.drained = true; try { child.proc.stdin.write("drain\n"); child.proc.stdin.flush?.(); } catch {} };
  const rate = () => { const at = now() - P.rateWindowMilliseconds; while (finishedAt.length > 0 && finishedAt[0] < at) finishedAt.shift(); return Math.round(finishedAt.length * 60000 / P.rateWindowMilliseconds * 10) / 10; };
  const report = ({ children: target, reason, limit, minutes }) => {
    const sum = live(), port = { ...portTotals }, open = active();
    for (const child of children) for (const key of Object.keys(port)) port[key] += child.port?.[key] ?? 0;
    const why = ownerPoolWhyIdleV1({ waves: [...waves.values()].map(wave => ({ state: wave.state, backoffUntil: wave.backoffUntil, acceptable: ownerPoolAcceptsRevisionV1(wave.head?.revision, mine()) })), open: open.length, children: children.size, targetChildren: target, targetReason: reason, now: now() }), newest = open[0] ?? [...waves.values()].sort((a, b) => b.createdMs - a.createdMs)[0];
    const starting = why === null && open.length > 0 && children.size < target, seconds = starting ? Math.ceil(ownerPoolStartDelayV1({ children: [...children], lastStartMs, now: now(), pollMilliseconds: targetPollMilliseconds }) / 1000) : Math.round((limit?.waitedMilliseconds ?? 0) / 1000), shown = starting ? "starting" : why;
    const fields = { count: shareless.count, minutes, waves: waves.size, children: children.size, target, seconds, need: limit?.needMegabytes, free: limit?.freeMegabytes, floor: limit?.floorMegabytes, held: limit?.heldMegabytes, cores: limit?.cores, booked: limit?.bookedCores, idle: limit?.idleCores == null ? "?" : Math.floor(limit.idleCores), swap: Math.round(limit?.swapOutMegabytesPerSecond ?? 0) }, textKey = (shown === "nothing-fits" || shown === "yielding") && limit !== undefined ? (limit.limit === "memory" ? "memory" : "cores") : shown;
    status?.update({ failures: { ...failures }, kept: port.kept, resent: port.resent, diverged: port.diverged, shed: port.shed, state: children.size > 0 ? "pulling" : waves.size > 0 ? "idle" : "idle", wave: newest?.id ?? null, units: [...waves.values()].reduce((count, wave) => count + wave.units, 0), waves: waves.size, children: children.size, target, slots: target, running: [...children].reduce((count, child) => count + child.running, 0), claimed: sum.claimed, published: sum.published, stored: sum.stored, held: sum.held, failed: sum.failed, claimAttempts: sum.attempts, recycled: totals.recycled, ownersPerMinute: rate(), childMegabytes: childMegabytes(), rssMegabytes: Math.round(process.memoryUsage().rss / MEGABYTE) + [...children].reduce((count, child) => count + child.rss, 0), ...reasonOf(shown, fields, textKey) });
    if (work !== undefined) work.busy = [...children].some(child => child.running > 0);
  };
  adopt();
  heartbeat();
  const beating = setInterval(heartbeat, P.leaseHeartbeatMilliseconds);
  try {
    await reap();
    while (!signal.aborted) {
      if (now() - lastPointerAt >= pollMilliseconds) { lastPointerAt = now(); try { await refresh(); } catch (error) { status?.update({ lastError: String(error?.message ?? error).slice(0, 200) }); } }
      if (starved !== undefined && pointerOk && children.size === 0 && now() - lastStarvedAt >= P.starvedRetryMilliseconds && (waves.size === 0 || wanted() !== undefined && ![...waves.values()].some(wave => judge(wave).state !== "declined" && ownerPoolAcceptsRevisionV1(wave.head?.revision, wanted())))) { lastStarvedAt = now(); void Promise.resolve().then(() => starved({ revision: wanted(), waves: waves.size })).catch(() => undefined); }
      await reap();
      const open = active(), passed = orphans();
      let target = { children: 0, reason: null };
      if (work?.restartPending === true) target = { children: 0, reason: null };
      else if (open.length > 0) { try { target = await capacity({ children: children.size + passed, childMegabytes: childMegabytes() }); } catch { target = { children: 1, reason: null }; } }
      shareStep("tick");
      if (shareless.until > 0) target = { children: 0, reason: "unshareable", minutes: Math.ceil((shareless.until - now()) / 60000) };
      lastTarget = target;
      const have = [...children].filter(child => !child.drained);
      if (target.reason === "waiting" || target.reason === "unshareable") {
        for (const child of children) {
          if (!child.drained) drain(child);
          child.yieldedMs ??= now();
          if (now() - child.yieldedMs >= P.yieldKillMilliseconds && !child.killed) { child.killed = true; try { child.proc.kill("SIGKILL"); } catch {} }
        }
      } else if (have.length < target.children) for (let step = 0; step < 2 && have.length + step < target.children && ownerPoolMayStartV1({ children: [...children], survivors: passed, target: target.children, ramp, lastStartMs, now: now(), pollMilliseconds: targetPollMilliseconds }); step++) { const wave = open.sort((a, b) => ownerPoolWaveOrderV1({ priority: a.head?.priority, children: a.children, createdMs: a.createdMs }, { priority: b.head?.priority, children: b.children, createdMs: b.createdMs }))[0]; if (wave === undefined) break; launch(wave); }
      else if (have.length > target.children) { const pick = ownerPoolDrainPickV1({ children: have, reason: target.reason, now: now(), pollMilliseconds: targetPollMilliseconds }); if (pick !== undefined) drain(pick); }
      for (const child of children) if ((child.wave.state === "declined" || !waves.has(child.wave.id)) && !child.drained) drain(child);
      report(target);
      if (once && waves.size > 0 && [...waves.values()].every(wave => wave.state !== "open" || wave.backoffUntil > now()) && children.size === 0) break;
      await sleep(targetPollMilliseconds, signal);
    }
  } finally {
    clearInterval(beating);
    try { if (readOwnerPoolLeaseV1(leaseFile)?.pid === process.pid) rmSync(leaseFile, { force: true }); } catch {}
    for (const child of children) { drain(child); try { child.proc.unref?.(); } catch {} }
    status?.update({ state: "idle", running: 0, children: 0 }); status?.flush?.();
    for (const wave of waves.values()) if (wave.dir !== undefined && !wave.dir.startsWith(kept)) rmSync(wave.dir, { recursive: true, force: true });
    rmSync(base, { recursive: true, force: true });
  }
}
