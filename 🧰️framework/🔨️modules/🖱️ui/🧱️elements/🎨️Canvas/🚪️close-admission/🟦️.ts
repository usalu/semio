/** 🚪️ Per-window close admission keeps layout intact until its owner agrees. */
export type WindowCloseRequestV1 = (windowId: string) => boolean | void | Promise<boolean | void>;

/** 🪟️ Deduplicates pending requests and retires their authority when the canvas unmounts. */
export function createWindowCloseAdmissionV1(port: Readonly<{ request?: WindowCloseRequestV1; commit: (windowId: string) => void; report: (error: unknown) => void }>): Readonly<{ close: (windowId: string) => Promise<void>; dispose: () => void }> {
  const pending = new Map<string, Promise<void>>();
  let live = true;
  return {
    close(windowId) {
      if (!live) return Promise.resolve();
      const held = pending.get(windowId);
      if (held !== undefined) return held;
      const work = Promise.resolve().then(async () => {
        if (!live) return;
        try {
          const decision = await port.request?.(windowId);
          if (live && decision !== false) port.commit(windowId);
        } catch (error) {
          port.report(error);
        }
      }).finally(() => { pending.delete(windowId); });
      pending.set(windowId, work);
      return work;
    },
    dispose() { live = false; pending.clear(); },
  };
}
