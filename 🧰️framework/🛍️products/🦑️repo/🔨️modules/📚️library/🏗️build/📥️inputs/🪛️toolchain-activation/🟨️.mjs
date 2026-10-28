/** 🪛️ Names the toolchains of the cache policy that the provisioning targets a project depends on activate: a project whose targets depend on `deps-tectonic` or `deps-tex` hashes the TeX pins even when no authored input lists them. */
export function toolchainsActivatedByV1(targets, toolchains) {
  const provisioners = new Set();
  for (const target of Object.values(targets ?? {})) for (const dependency of target?.dependsOn ?? []) {
    const name = typeof dependency === "string" ? dependency : dependency?.target;
    if (typeof name === "string") provisioners.add(name.slice(name.lastIndexOf(":") + 1));
  }
  return Object.entries(toolchains ?? {}).filter(([, contract]) => (contract.activatedBy ?? []).some((name) => provisioners.has(name))).map(([tool]) => tool).sort();
}
