import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export const LOOK_POLICY_V1 = Object.freeze({ ...schema.definitions.Policy.const });
const TEXTS = schema.definitions.Texts.const;
const fill = (text, values) => text.replace(/\{(\w+)\}/gu, (_all, name) => String(values[name] ?? 0));

/** 🔭️ How many owners one look job holds: enough batches for every child to be fed `jobsPerChild` times, never more than `maximumOwnersPerJob`, never fewer than one.
 * @see ./🧪️tests/🟦️.ts */
export function lookBatchSizeV1(owners, children, policy = LOOK_POLICY_V1) {
  return Math.max(1, Math.min(policy.maximumOwnersPerJob, Math.ceil(owners / (Math.max(1, children) * policy.jobsPerChild))));
}

/** 🔭️ The batches of the lanes of a take pass in the order the farm serves them: one batch of each lane in turn, so the owners of a short lane never wait behind thousands of the long one. Every name is in exactly one batch, in its lane's order.
 * @see ./🧪️tests/🟦️.ts */
export function lookBatchesV1(lanes, size) {
  const rest = lanes.map(lane => [...lane]), batches = [];
  for (let live = rest.filter(lane => lane.length > 0); live.length > 0; live = live.filter(lane => lane.length > 0)) for (const lane of live) batches.push(lane.splice(0, size));
  return batches;
}

/** 🔭️ The line a read says when it queues its looks, in one language. */
export const lookQueuedTextV1 = (language, fields) => fill(TEXTS[language === "de" ? "de" : "en"].queued, fields);

/**
 * 🔭️ The queue of owner looks of a read: every look is a job of a farm child (the assembly authority, the verdict on the stored record and the closure references of up to `maximumOwnersPerJob` owners; `shared` is what every job carries, read in the flow of the caller of `start` or `ask` when it is a function), the parent only dispatches the jobs and hands each owner its answer. At most as many jobs run at once as children are ready (re-read at every settlement and whenever the farm grows through `kick`), a job is dispatched only while the answers nobody asked for yet stay below a lead of `leadJobsPerChild` jobs per child, and an owner that asks before its job was dispatched moves it to the front (it is then also exempt from the lead). `ask(name)` answers the reply of the owner once (a second ask looks again, as does an answer older than `freshMilliseconds`), undefined when the farm is gone, and rejects with `refuse(name, reason)` when the farm could not look at it.
 * @param {{ farm: { width(): number, observe(job: object): Promise<{ looks?: object[] } | undefined> }, jobOf: (name: string) => object, shared?: object | (() => object), refuse: (name: string, reason: string) => Error, say?: (text: (language: string) => string) => void, policy?: typeof LOOK_POLICY_V1, now?: () => number }} options
 * @see ./🧪️tests/🟦️.ts
 */
export function createLookQueueV1({ farm, jobOf, shared = {}, refuse, say = () => {}, policy = LOOK_POLICY_V1, now = () => performance.now() }) {
  const entries = new Map(), queue = [], counts = { jobs: 0, owners: 0, asked: 0, again: 0, refused: 0 };
  let running = 0, held = 0, flying = 0, size = 1;
  const children = () => Math.max(1, farm.width());
  const entryOf = (name, batch) => {
    let resolve, reject;
    const promise = new Promise((done, fail) => { resolve = done; reject = fail; });
    promise.catch(() => {});
    return { name, batch, at: 0, ready: false, promise, resolve, reject };
  };
  const finish = (entry, reply, error) => {
    if (entry.ready) return;
    entry.ready = true; entry.at = now(); held++;
    if (error === undefined) entry.resolve(reply); else entry.reject(error);
  };
  const run = batch => {
    batch.state = "flying"; running++; flying += batch.names.length; counts.jobs++;
    const job = { ...batch.shared, name: batch.names[0] + (batch.names.length > 1 ? " +" + (batch.names.length - 1) : ""), look: true, weight: 0.01 * batch.names.length, owners: batch.names.map(jobOf) };
    Promise.resolve().then(() => farm.observe(job)).then(
      reply => {
        const byName = new Map((reply?.looks ?? []).map(look => [look.name, look]));
        for (const name of batch.names) {
          const entry = entries.get(name);
          if (entry === undefined || entry.batch !== batch) continue;
          const look = byName.get(name);
          if (reply === undefined) finish(entry, undefined);
          else if (look === undefined || typeof look.failed === "string") { counts.refused++; finish(entry, undefined, refuse(name, look?.failed ?? "the child gave no answer for the owner")); }
          else finish(entry, look);
        }
      },
      error => { for (const name of batch.names) { const entry = entries.get(name); if (entry !== undefined && entry.batch === batch) { counts.refused++; finish(entry, undefined, refuse(name, String(error?.message ?? error).slice(0, 300))); } } },
    ).finally(() => { running--; flying -= batch.names.length; pump(); });
  };
  const pump = () => {
    while (queue.length > 0 && running < children() && (queue[0].urgent === true || held + flying < children() * policy.leadJobsPerChild * size)) run(queue.shift());
  };
  const enqueue = (names, front) => {
    const batch = { names, state: "queued", urgent: front, shared: typeof shared === "function" ? shared() : shared };
    for (const name of names) entries.set(name, entryOf(name, batch));
    if (front) queue.unshift(batch); else queue.push(batch);
    return batch;
  };
  return Object.freeze({
    /** 🔭️ Queues the looks of all lanes of a take pass, in batches. */
    start(lanes) {
      const names = lanes.reduce((total, lane) => total + lane.length, 0);
      if (names === 0) return;
      size = lookBatchSizeV1(names, farm.width() || policy.assumedChildren, policy);
      const batches = lookBatchesV1(lanes, size);
      for (const names of batches) enqueue(names, false);
      counts.owners += names.length;
      say(language => lookQueuedTextV1(language, { owners: names, jobs: batches.length, size, children: farm.width() || policy.assumedChildren }));
      pump();
    },
    /** 🔔️ Tells the queue that a child became ready. */
    kick() { pump(); },
    /** 🔭️ The look of one owner: the answer of its job, queued or flown already. */
    ask(name) {
      counts.asked++;
      let entry = entries.get(name);
      if (entry !== undefined && entry.ready && now() - entry.at > policy.freshMilliseconds) { held--; entries.delete(name); entry = undefined; counts.again++; }
      if (entry === undefined) { enqueue([name], true); entry = entries.get(name); }
      if (entry.batch.state === "queued") { queue.splice(queue.indexOf(entry.batch), 1); entry.batch.urgent = true; queue.unshift(entry.batch); pump(); }
      const settled = entry;
      return settled.promise.finally(() => { if (entries.get(name) === settled) { entries.delete(name); if (settled.ready) held--; } });
    },
    stats() { return Object.freeze({ ...counts, queued: queue.reduce((total, batch) => total + batch.names.length, 0), flying, running, held }); },
  });
}
