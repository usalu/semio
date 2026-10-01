/** 🪟️ Skipped body debt remains visible while contributor retirement yields. */
export async function retireSkippedWindowBodies<T extends { readonly id: string; readonly bodyKey: string }>(instances: readonly T[], read: () => ReadonlyMap<string, string>, publish: (value: ReadonlyMap<string, string>) => void, retire: (instance: T) => Promise<void>): Promise<void> {
  const skipped = new Map(read());
  for (const instance of instances) skipped.set(instance.id, instance.bodyKey);
  publish(skipped);
  for (const instance of instances) await retire(instance);
}
