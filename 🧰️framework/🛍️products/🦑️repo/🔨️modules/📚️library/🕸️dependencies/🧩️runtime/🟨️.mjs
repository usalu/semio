/** 🧩️ Runtime membership is closed over dependencies and every selected component's consumed topics.
 * Host plugins reached only because an extension `extends` them are linked shallowly: the host is
 * materialized, but its own `depends-on` / `consumes` closure is not expanded from that edge.
 * @param {readonly {pluginId: string, dependsOn?: readonly string[], consumes?: readonly string[], contributes?: readonly string[], extends?: string, host?: unknown}[]} components
 * @param {readonly string[]} roots
 * @returns {string[]}
 */
export function runtimeComponentClosure(components, roots) {
  const byId = new Map(), contributors = new Map();
  for (const component of components) {
    if (!component.pluginId || byId.has(component.pluginId)) throw new Error(`Invalid or duplicate runtime component ${component.pluginId}`);
    byId.set(component.pluginId, component);
    for (const topic of component.contributes ?? []) {
      const ids = contributors.get(topic) ?? [];
      ids.push(component.pluginId);
      contributors.set(topic, ids);
    }
  }
  const selected = new Set(), pending = roots.map(id => ({ id, shallow: false }));
  while (pending.length) {
    const { id, shallow } = pending.pop();
    if (selected.has(id)) continue;
    const component = byId.get(id);
    if (!component) throw new Error(`Unknown runtime component ${id}`);
    selected.add(id);
    if (shallow) continue;
    for (const dep of component.dependsOn ?? []) pending.push({ id: dep, shallow: component.extends === dep });
    for (const topic of component.consumes ?? []) for (const contributor of contributors.get(topic) ?? []) pending.push({ id: contributor, shallow: false });
    if (component.host) for (const hostId of byId.keys()) pending.push({ id: hostId, shallow: false });
  }
  return [...selected].sort();
}
