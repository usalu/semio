/** 🧺️ The verdict of a presence check shared by every look that asks at the same time: the looks of a take pass ask the same question (are the inventory records my record names stored?) thousands of times, each with a storage lease and a walk of the ancestry of the store. A caller that asks while a batch runs joins the next batch, so one lease answers all of them; a name found present stays answered for the validation round that asked, because the files of a round do not change and every record is content addressed. A batch that fails is asked again caller by caller, so only the caller whose names fail sees the failure. `run(names)` answers the set of names that are missing and throws for a refusal. */
export function createPresenceCoalescerV1({ run, maximumNames = 512, pause = () => new Promise(resolve => setImmediate(resolve)) }) {
  const waiting = [], rounds = new WeakMap(), stats = { asked: 0, remembered: 0, batches: 0, retried: 0, names: 0 };
  let draining = false;
  const known = round => { if (round === undefined || round === null || typeof round !== "object") return new Set(); let set = rounds.get(round); if (set === undefined) { set = new Set(); rounds.set(round, set); } return set; };
  const settle = (callers, missing) => { for (const caller of callers) { if (caller.signal?.aborted) { caller.resolve(false); continue; } for (const name of caller.names) if (!missing.has(name)) caller.known.add(name); caller.resolve(caller.names.every(name => !missing.has(name))); } };
  const drain = async () => {
    if (draining) return;
    draining = true;
    try {
      await pause();
      while (waiting.length > 0) {
        const callers = [], names = new Set(), signal = waiting[0].signal;
        while (waiting.length > 0 && waiting[0].signal === signal && (callers.length === 0 || names.size + waiting[0].names.length <= maximumNames)) {
          const caller = waiting.shift();
          signal?.removeEventListener("abort", caller.withdraw);
          callers.push(caller);
          for (const name of caller.names) names.add(name);
        }
        stats.batches++; stats.names += names.size;
        try { settle(callers, await run([...names].sort(), signal)); }
        catch (error) {
          if (signal?.aborted) { for (const caller of callers) caller.reject(error); continue; }
          if (callers.length === 1) { callers[0].reject(error); continue; }
          for (const caller of callers) {
            if (signal?.aborted) { caller.resolve(false); continue; }
            stats.retried++; try { settle([caller], await run([...caller.names].sort(), signal)); } catch (single) { caller.reject(single); }
          }
        }
      }
    } finally { draining = false; if (waiting.length > 0) void drain(); }
  };
  return Object.freeze({
    /** 🪣️ Whether every name is present: from the memory of `round` when it was found before, else through the next batch. */
    ask(names, round, signal) {
      stats.asked++;
      if (signal?.aborted) return Promise.resolve(false);
      const set = known(round), unknown = names.filter(name => !set.has(name));
      if (unknown.length === 0) { stats.remembered++; return Promise.resolve(true); }
      return new Promise((resolve, reject) => {
        const caller = { names: unknown, known: set, resolve, reject, signal, withdraw: () => {
          const at = waiting.indexOf(caller);
          if (at >= 0) { waiting.splice(at, 1); resolve(false); }
        } };
        waiting.push(caller);
        signal?.addEventListener("abort", caller.withdraw, { once: true });
        void drain();
      });
    },
    stats: () => ({ ...stats }),
  });
}
