/** 🎭️ Keeps every topology's scoped resources until all its consumers have settled. */
export async function runTopologySessionsV1<T, S>(topologies: readonly T[], serial: boolean, withSession: (topology: T, consume: (session: S) => Promise<void>) => Promise<void>, consume: (session: S) => Promise<void>): Promise<void> {
  if (serial) {
    for (const topology of topologies) await withSession(topology, consume);
    return;
  }
  const open = async (index: number, sessions: readonly S[]): Promise<void> => {
    if (index < topologies.length) return withSession(topologies[index], session => open(index + 1, [...sessions, session]));
    const results = await Promise.allSettled(sessions.map(consume));
    const failures = results.filter(result => result.status === "rejected").map(result => result.reason);
    if (failures.length) throw new AggregateError(failures, "Topology consumers failed");
  };
  await open(0, []);
}
