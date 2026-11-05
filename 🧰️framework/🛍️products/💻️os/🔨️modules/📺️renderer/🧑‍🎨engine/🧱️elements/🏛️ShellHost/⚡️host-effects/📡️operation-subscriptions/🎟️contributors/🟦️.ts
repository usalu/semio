/** 🎟️ Operation listeners follow the exact mounted contributor lease through creation and retirement. */
export interface ContributorOperationLeaseV1 { readonly instance: Promise<number> }
export function subscribeContributorOperationLeasesV1<T extends ContributorOperationLeaseV1>(
  instances: { readonly current: ReadonlyMap<string, T> },
  subscribe: (entry: T, instanceId: number, isCurrent: () => boolean) => (() => void) | undefined,
  onError: (error: unknown) => void,
): () => void {
  let active = true;
  const disposers: (() => void)[] = [];
  for (const [key, entry] of instances.current) {
    const current = () => active && instances.current.get(key) === entry;
    void entry.instance.then(instanceId => {
      if (!current()) return;
      const dispose = subscribe(entry, instanceId, current);
      if (dispose) {
        if (current()) disposers.push(dispose);
        else dispose();
      }
    }).catch(error => { if (current()) onError(error); });
  }
  return () => {
    active = false;
    for (const dispose of disposers.splice(0)) dispose();
  };
}
