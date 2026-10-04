/** 🔂️ Completes one immediate poll and rejects suspension. */
export function resolveReady<T>(poll: () => {ready: true; value: T} | {ready: false}): T {
  const result = poll();
  if (result.ready) return result.value;
  throw Error("resolveReady: future was not ready on its first poll");
}
