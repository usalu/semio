/** 🧺️ The verdict of a presence check shared by every look that asks at the same time: the looks of a take pass ask the same question (are the inventory records my record names stored?) thousands of times, each with a storage lease and a walk of the ancestry of the store. A caller that asks while a batch runs joins the next batch, so one lease answers all of them; a name found present stays answered for the validation round that asked, because the files of a round do not change and every record is content addressed. A batch that fails is asked again caller by caller, so only the caller whose names fail sees the failure. `run(names)` answers the set of names that are missing and throws for a refusal. */
export function createPresenceCoalescerV1({ run, maximumNames = 512, pause = () => new Promise(resolve => setImmediate(resolve)) }) {
  const waiting = [], rounds = new WeakMap(), stats = { asked: 0, remembered: 0, batches: 0, retried: 0, names: 0 };
  let draining = false;
  const known = round => { if (round === undefined || round === null || typeof round !== "object") return new Set(); let set = rounds.get(round); if (set === undefined) { set = new Set(); rounds.set(round, set); } return set; };
  const settle = (callers, missing) => { for (const caller of callers) { for (const name of caller.names) if (!missing.has(name)) caller.known.add(name); caller.resolve(caller.names.every(name => !missing.has(name))); } };
  const drain = async () => {
    if (draining) return;
    draining = true;
    try {
      await pause();
      while (waiting.length > 0) {
        const callers = [], names = new Set();
        while (waiting.length > 0 && (callers.length === 0 || names.size + waiting[0].names.length <= maximumNames)) { const caller = waiting.shift(); callers.push(caller); for (const name of caller.names) names.add(name); }
        stats.batches++; stats.names += names.size;
        try { settle(callers, await run([...names].sort())); }
        catch (error) {
          if (callers.length === 1) { callers[0].reject(error); continue; }
          for (const caller of callers) { stats.retried++; try { settle([caller], await run([...caller.names].sort())); } catch (single) { caller.reject(single); } }
        }
      }
    } finally { draining = false; if (waiting.length > 0) void drain(); }
  };
  return Object.freeze({
    /** 🪣️ Whether every name is present: from the memory of `round` when it was found before, else through the next batch. */
    ask(names, round) {
      stats.asked++;
      const set = known(round), unknown = names.filter(name => !set.has(name));
      if (unknown.length === 0) { stats.remembered++; return Promise.resolve(true); }
      return new Promise((resolve, reject) => { waiting.push({ names: unknown, known: set, resolve, reject }); void drain(); });
    },
    stats: () => ({ ...stats }),
  });
}
