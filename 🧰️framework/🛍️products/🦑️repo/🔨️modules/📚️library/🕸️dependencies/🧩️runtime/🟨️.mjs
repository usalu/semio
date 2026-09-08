/** 🧩️ Runtime membership is closed over dependencies and every selected component's consumed topics.
 * @param {readonly {pluginId: string, dependsOn?: readonly string[], consumes?: readonly string[], contributes?: readonly string[], host?: unknown}[]} components
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
  const selected = new Set(), pending = [...roots];
  while (pending.length) {
    const id = pending.pop();
    if (selected.has(id)) continue;
    const component = byId.get(id);
    if (!component) throw new Error(`Unknown runtime component ${id}`);
    selected.add(id);
    pending.push(...(component.dependsOn ?? []));
    for (const topic of component.consumes ?? []) pending.push(...(contributors.get(topic) ?? []));
    if (component.host) pending.push(...byId.keys());
  }
  return [...selected].sort();
}
