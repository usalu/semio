/** 🧵️ Caller-owned construction and explicit admission, scoped by worker identity. */
export function createWorkerCell<T>(initialize: () => T): { read(owner: string): T } {
  const states = new Map<string, T>();
  return { read(owner) { if (!states.has(owner)) states.set(owner, initialize()); return states.get(owner)!; } };
}

/** 🚪️ Reads require a prior caller-supplied admission; repeated admission updates existing state. */
export function createAdmittedWorkerCell<T>(): { read(owner: string): T | undefined; admit(owner: string, initialize: () => T, update: (state: T) => void): T } {
  const states = new Map<string, T>();
  return {
    read: owner => states.get(owner),
    admit(owner, initialize, update) {
      if (!states.has(owner)) states.set(owner, initialize());
      const state = states.get(owner)!;
      update(state);
      return state;
    },
  };
}
