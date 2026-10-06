/** 🌲️ One caller-owned directory entry. */
export interface DiscoveryEntry<P> { readonly value: P; readonly directory: boolean }
/** 🧭️ Complete or cancelled discovery with retained work. */
export interface Discovery<P> { readonly files: readonly P[]; readonly directories: number; readonly cancelled: boolean }
/** 🌳️ Visits physical directory identities once and checks control before each expansion. */
export function discover<P, K>(root: P, identity: (path: P) => K, entries: (path: P) => readonly DiscoveryEntry<P>[], matches: (path: P) => boolean, control: (directories: number, path: P) => boolean): Discovery<P> {
  const pending = [root], visited = new Set<K>(), files: P[] = [];
  while (pending.length) {
    const directory = pending.pop()!;
    if (!control(visited.size, directory)) return { files, directories: visited.size, cancelled: true };
    const key = identity(directory);
    if (visited.has(key)) continue;
    visited.add(key);
    for (const entry of entries(directory)) {
      if (entry.directory) pending.push(entry.value);
      else if (matches(entry.value)) files.push(entry.value);
    }
  }
  return { files, directories: visited.size, cancelled: false };
}
