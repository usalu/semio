export interface SDevMountV1 { dispose(): void; }
export interface SDevProgressV1 { readonly stage: "mount" | "ready" | "dispose"; readonly completed: number; readonly total: number; }
export interface SDevOperationV1 { readonly signal: AbortSignal; readonly progress: (event: SDevProgressV1) => void; }

/** 🧩 Mounts the explicit installed inventory and retires every acquired owner exactly once. */
export async function mountSDevInventoryV1<T>(installed: readonly T[], mount: (contribution: T, operation: SDevOperationV1) => Promise<SDevMountV1>, operation: SDevOperationV1): Promise<SDevMountV1> {
  const retained: SDevMountV1[] = [];
  let disposed = false;
  const dispose = (): void => {
    if (disposed) return;
    disposed = true;
    operation.signal.removeEventListener("abort", dispose);
    const errors: unknown[] = [];
    while (retained.length) {
      try { retained.pop()!.dispose(); } catch (error) { errors.push(error); }
    }
    operation.progress({ stage: "dispose", completed: installed.length, total: installed.length });
    if (errors.length) throw new AggregateError(errors, "s-dev.disposal-failed");
  };
  operation.signal.throwIfAborted();
  operation.signal.addEventListener("abort", dispose, { once: true });
  try {
    operation.progress({ stage: "mount", completed: 0, total: installed.length });
    for (let index = 0; index < installed.length; index++) {
      operation.signal.throwIfAborted();
      const acquired = await mount(installed[index], operation);
      if (disposed || operation.signal.aborted) { acquired.dispose(); operation.signal.throwIfAborted(); throw new Error("s-dev.retired"); }
      retained.push(acquired);
      operation.progress({ stage: "mount", completed: index + 1, total: installed.length });
      operation.signal.throwIfAborted();
    }
    operation.progress({ stage: "ready", completed: installed.length, total: installed.length });
    operation.signal.throwIfAborted();
    return { dispose };
  } catch (error) {
    try { dispose(); } catch (retirement) { throw new AggregateError([error, retirement], "s-dev.mount-and-disposal-failed"); }
    throw error;
  }
}
