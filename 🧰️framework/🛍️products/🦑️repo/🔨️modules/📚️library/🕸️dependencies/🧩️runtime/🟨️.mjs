/** 🧩️ Runtime membership is closed over dependencies and every selected component's consumed topics.
 * Host plugins reached only because an extension `extends` them are linked shallowly: the host is
 * materialized, but its own `depends-on` / `consumes` closure is not expanded from that edge.
 *
 * 🏠️ A host component (`host` set) pulls EVERY component in, because the OS shell's launcher opens
 * any artifact kind. That is a property of the host SESSION, not of the crate: the same crate also
 * ships ordinary artifact apps (`🪐️space`'s Home and Space), and a playground row that names one
 * `app` boots that app standalone — exactly like every other single-app row. Such a root is passed
 * `appScoped: true` and keeps its own `depends-on` / `consumes` closure instead of the whole catalog
 * (which no page can link anyway while `stdio`'s component is over the wasm function ceiling).
 * @param {readonly {pluginId: string, dependsOn?: readonly string[], consumes?: readonly string[], contributes?: readonly string[], extends?: string, host?: unknown}[]} components
 * @param {readonly (string | {readonly id: string, readonly appScoped?: boolean})[]} roots
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
  const selected = new Set(), pending = roots.map(root => (typeof root === "string" ? { id: root, shallow: false, appScoped: false } : { id: root.id, shallow: false, appScoped: root.appScoped === true }));
  while (pending.length) {
    const { id, shallow, appScoped } = pending.pop();
    if (selected.has(id)) continue;
    const component = byId.get(id);
    if (!component) { const error=new Error(`Unknown runtime component ${id}`);error.code="RUNTIME_INPUT_ADMISSION";error.componentId=id;throw error; }
    selected.add(id);
    if (shallow) continue;
    for (const dep of component.dependsOn ?? []) pending.push({ id: dep, shallow: component.extends === dep, appScoped: false });
    for (const topic of component.consumes ?? []) for (const contributor of contributors.get(topic) ?? []) pending.push({ id: contributor, shallow: false, appScoped: false });
    if (component.host && !appScoped) for (const hostId of byId.keys()) pending.push({ id: hostId, shallow: false, appScoped: false });
  }
  return [...selected].sort();
}

/** 📥️ Retains refused source admission independently of runtime closure facts. */
export function runtimeInputAdmissionV1(components, roots, sourcePaths, sourceExists) {
 try { const selected=runtimeComponentClosure(components,roots);for(const path of sourcePaths)if(!sourceExists(path))return {schemaVersion:1,status:"refused",missing:{kind:"source",value:path}};return {schemaVersion:1,status:"admitted",selected}; }
 catch(error){if(error.code!=="RUNTIME_INPUT_ADMISSION")throw error;return {schemaVersion:1,status:"refused",missing:{kind:"component",value:error.componentId}};}
}
