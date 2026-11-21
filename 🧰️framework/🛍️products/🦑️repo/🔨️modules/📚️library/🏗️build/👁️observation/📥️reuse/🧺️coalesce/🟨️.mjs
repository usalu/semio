function bindBatchWithdrawal(callers) {
  const signals = new Set(callers.map(caller => caller.signal));
  if (signals.has(undefined)) return {};
  if (signals.size === 1) return { signal: signals.values().next().value };
  const controller = new AbortController();
  const withdraw = event => {
    const signal = event.target;
    if (signals.delete(signal) && signals.size === 0) controller.abort(signal.reason);
  };
  for (const signal of signals) {
    if (signal.aborted) signals.delete(signal);
    else signal.addEventListener("abort", withdraw, { once: true });
  }
  if (signals.size === 0) controller.abort();
  return { signal: controller.signal, close: () => { for (const signal of signals) signal.removeEventListener("abort", withdraw); } };
}

/** 🧺️ The verdict of a presence check shared by every look that asks at the same time: the looks of a take pass ask the same question (are the inventory records my record names stored?) thousands of times, each with a storage lease and a walk of the ancestry of the store. A caller that asks while a batch runs joins the next batch, so one lease answers them even when their withdrawal signals differ. Mixed withdrawal stops the shared operation after every observer has left; a name found present stays answered for the validation round that asked, because the files of a round do not change and every record is content addressed. A batch that fails is asked again caller by caller, so only the caller whose names fail sees the failure. `run(names)` answers the set of names that are missing and throws for a refusal. */
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
        const callers = [], names = new Set();
        while (waiting.length > 0) {
          const caller = waiting[0], unknown = caller.names.filter(name => !caller.known.has(name));
          if (callers.length > 0 && names.size + unknown.length > maximumNames
            && names.size + new Set(unknown.filter(name => !names.has(name))).size > maximumNames) break;
          waiting.shift();
          caller.signal?.removeEventListener("abort", caller.withdraw);
          if (unknown.length === 0) { stats.remembered++; caller.resolve(true); continue; }
          caller.names = unknown;
          callers.push(caller);
          for (const name of unknown) names.add(name);
        }
        if (callers.length === 0) continue;
        const { signal, close } = bindBatchWithdrawal(callers);
        stats.batches++; stats.names += names.size;
        try { settle(callers, await run([...names].sort(), signal)); }
        catch (error) {
          if (signal?.aborted) { for (const caller of callers) caller.reject(error); continue; }
          if (callers.length === 1) { callers[0].reject(error); continue; }
          for (const caller of callers) {
            if (caller.signal?.aborted) { caller.resolve(false); continue; }
            stats.retried++; try { settle([caller], await run([...caller.names].sort(), caller.signal)); } catch (single) { caller.reject(single); }
          }
        } finally { close?.(); }
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
