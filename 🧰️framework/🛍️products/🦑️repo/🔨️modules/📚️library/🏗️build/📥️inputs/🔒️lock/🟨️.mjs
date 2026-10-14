/** 🔒️ Names a locked package independently of Cargo's abbreviated dependency strings. */
export function cargoLockIdentity(entry) {
  return `${entry.name}@${entry.version}${entry.source === undefined ? "" : ` (${entry.source})`}`;
}

/** 🧬️ Projects exact resolved versions, sources and checksums reachable from the selected local packages. */
export function cargoLockClosure(lock, roots) {
  if (lock.version !== 4 || !Array.isArray(lock.package)) throw Error("Cargo lock version 4 required");
  const entries = lock.package, names = new Map();
  for (const entry of entries) { const group = names.get(entry.name) ?? []; group.push(entry); names.set(entry.name, group); }
  const visited = new Map(), pending = roots.map(root => {
    const matches = (names.get(root.name) ?? []).filter(entry => (root.version === undefined || entry.version === root.version) && entry.source === undefined);
    if (matches.length !== 1) throw Error(`Selected local Cargo lock package refused: ${root.name}`);
    return matches[0];
  });
  while (pending.length) {
    const entry = pending.pop(), identity = cargoLockIdentity(entry);
    if (visited.has(identity)) continue;
    const dependencies = (entry.dependencies ?? []).map(dependency => {
      const match = /^([^ ]+)(?: ([^ ()]+))?(?: \((.+)\))?$/u.exec(dependency);
      if (!match) throw Error(`Invalid locked dependency: ${dependency}`);
      const [, name, version, source] = match, matches = (names.get(name) ?? []).filter(candidate => (version === undefined || candidate.version === version) && (source === undefined || candidate.source === source));
      if (matches.length !== 1) throw Error(`Ambiguous locked dependency: ${dependency}`);
      pending.push(matches[0]);
      return cargoLockIdentity(matches[0]);
    }).sort();
    visited.set(identity, { name: entry.name, version: entry.version, ...(entry.source === undefined ? {} : { source: entry.source }), ...(entry.checksum === undefined ? {} : { checksum: entry.checksum }), dependencies });
  }
  return { version: 1, packages: [...visited].sort(([left], [right]) => left.localeCompare(right, "en")).map(([, entry]) => entry) };
}
