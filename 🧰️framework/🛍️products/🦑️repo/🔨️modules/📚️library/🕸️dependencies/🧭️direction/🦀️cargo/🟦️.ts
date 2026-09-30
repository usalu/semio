import { readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type CargoDirectionDependency = Readonly<{ name: string; alias: string; owner: string | null; kind: "normal" | "dev" | "build"; optional: boolean; platform: string | null }>;
export type CargoDirectionPackage = Readonly<{ name: string; owner: string; role: string | null; dependencies: readonly CargoDirectionDependency[] }>;
export type CargoDirectionPolicy = Readonly<{ areaLayers: Readonly<Record<string, "framework" | "implementation" | "repo-wide">>; roles: readonly string[]; rules: Readonly<Record<string, Readonly<{ fromRoles: readonly string[]; toRoles: readonly string[] }>>> }>;
export type CargoDirectionViolation = Readonly<{ rule: string; from: string; to: string; alias: string; kind: CargoDirectionDependency["kind"]; optional: boolean; platform: string | null }>;
export type CargoDirectionProblem = Readonly<{ code: "missing-role" | "unknown-role" | "unclassified-owner"; owner: string }>;
export type CargoDirectionReport = Readonly<{ packages: number; localDependencies: number; violations: readonly CargoDirectionViolation[]; problems: readonly CargoDirectionProblem[] }>;

const object = (value: unknown): Record<string, any> => {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Cargo direction requires readable object metadata");
  return value as Record<string, any>;
};
const text = (value: unknown): string => {
  if (typeof value !== "string" || !value) throw new Error("Cargo direction requires a nonempty name or owner path");
  return value;
};
const array = (value: unknown): unknown[] => {
  if (!Array.isArray(value)) throw new Error("Cargo direction requires complete array metadata");
  return value;
};
const ownerPath = (root: string, path: string): string => {
  const owner = relative(root, path).replaceAll("\\", "/");
  if (!owner || owner.startsWith("../") || isAbsolute(owner)) throw new Error(`Cargo direction owner escapes the authored workspace: ${path}`);
  return owner;
};
const role = (manifest: Record<string, any>): string | null => manifest.package?.metadata?.semio?.role ?? null;

/** 🧾️ Inventories all authored Cargo members and declaration kinds independently of the Cargo graph. */
export function cargoDirectionInventory(root: string): readonly CargoDirectionPackage[] {
  const read = (path: string): Record<string, any> => object(Bun.TOML.parse(readFileSync(path, "utf8")));
  const workspace = object(read(join(root, "Cargo.toml")).workspace);
  const members = array(workspace.members).map(text);
  if (!members.length || members.some((path) => /[*?\[\]]/u.test(path))) throw new Error("Cargo direction requires explicit authored workspace members");
  const found: CargoDirectionPackage[] = [], pending = [...members], visited = new Set<string>();
  for (let index = 0; index < pending.length; index++) {
    const member = pending[index]!;
    const owner = ownerPath(root, resolve(root, member));
    if (visited.has(owner)) continue;
    visited.add(owner);
    const manifest = read(join(root, owner, "Cargo.toml"));
    const dependencies: CargoDirectionDependency[] = [];
    const collect = (tables: Record<string, any>, platform: string | null): void => {
      for (const [table, kind] of [["dependencies", "normal"], ["dev-dependencies", "dev"], ["build-dependencies", "build"]] as const) {
        for (const [alias, declaration] of Object.entries(tables[table] ?? {})) {
          const local = typeof declaration === "string" ? { version: declaration } : object(declaration);
          const inherited = local.workspace === true;
          const source = inherited ? object(workspace.dependencies ?? {})[alias] : local;
          if (inherited && source === undefined) throw new Error(`Cargo direction cannot read inherited dependency: ${owner} → ${alias}`);
          const base = typeof source === "string" ? { version: source } : object(source);
          const value = { ...base, ...local };
          if (value.optional !== undefined && typeof value.optional !== "boolean") throw new Error(`Cargo direction requires a boolean optional declaration: ${owner} → ${alias}`);
          dependencies.push({ name: text(value.package ?? alias), alias, owner: value.path === undefined ? null : ownerPath(root, resolve(inherited ? root : join(root, owner), text(value.path))), kind, optional: value.optional ?? false, platform });
        }
      }
    };
    collect(manifest, null);
    for (const [platform, tables] of Object.entries(manifest.target ?? {})) collect(object(tables), platform);
    found.push({ name: text(object(manifest.package).name), owner, role: role(manifest), dependencies });
    for (const dep of dependencies) if (dep.owner !== null && !visited.has(dep.owner)) pending.push(dep.owner);
  }
  return found;
}

/** 🦀️ Adapts Cargo's declared metadata behind a repository-owned language-neutral contract. */
export function cargoDirectionMetadata(value: unknown, root: string): readonly CargoDirectionPackage[] {
  const metadata = object(value), packages = array(metadata.packages), members = array(metadata.workspace_members).map(text);
  if (resolve(text(metadata.workspace_root)) !== resolve(root)) throw new Error("Cargo direction metadata belongs to a different workspace");
  if (metadata.version !== 1 || !packages.length || members.length !== packages.length || new Set(members).size !== members.length) throw new Error("Cargo direction metadata is empty or incomplete");
  const ids = packages.map((value) => text(object(value).id));
  if (new Set(ids).size !== ids.length || JSON.stringify([...ids].sort()) !== JSON.stringify([...members].sort())) throw new Error("Cargo direction metadata omits authored workspace members");
  return packages.map((value) => {
    const pkg = object(value), owner = ownerPath(root, dirname(text(pkg.manifest_path)));
    return { name: text(pkg.name), owner, role: pkg.metadata?.semio?.role ?? null, dependencies: array(pkg.dependencies).map((value) => {
      const dep = object(value), kind = dep.kind === null ? "normal" : dep.kind;
      if (kind !== "normal" && kind !== "dev" && kind !== "build") throw new Error("Cargo direction metadata has an unknown dependency kind");
      if (typeof dep.optional !== "boolean" || (dep.target !== null && typeof dep.target !== "string")) throw new Error("Cargo direction metadata omits dependency activation declarations");
      return { name: text(dep.name), alias: text(dep.rename ?? dep.name), owner: dep.path === undefined || dep.path === null ? null : ownerPath(root, text(dep.path)), kind, optional: dep.optional, platform: dep.target };
    }) };
  });
}

/** 🧱️ Enforces physical and semantic direction for every local declaration without role omissions or exceptions. */
export function cargoDependencyDirectionReport(graph: readonly CargoDirectionPackage[], inventory: readonly CargoDirectionPackage[], policy: CargoDirectionPolicy): CargoDirectionReport {
  const validate = (packages: readonly CargoDirectionPackage[]): void => {
    if (!Array.isArray(packages) || !packages.length) throw new Error("Cargo direction requires a complete authored package inventory");
    if (new Set(packages.map((pkg) => pkg.owner)).size !== packages.length || new Set(packages.map((pkg) => pkg.name)).size !== packages.length) throw new Error("Cargo direction repeats a package owner or name");
    for (const pkg of packages) {
      text(pkg.name); text(pkg.owner);
      if (pkg.role !== null && typeof pkg.role !== "string") throw new Error("Cargo direction requires readable semantic role metadata");
      for (const value of array(pkg.dependencies)) {
        const dep = object(value);
        text(dep.name); text(dep.alias);
        if (!["normal", "dev", "build"].includes(dep.kind) || typeof dep.optional !== "boolean" || (dep.platform !== null && typeof dep.platform !== "string")) throw new Error("Cargo direction requires complete declaration activation metadata");
        if (dep.owner !== null) {
          const target = packages.find((pkg) => pkg.owner === text(dep.owner));
          if (!target || target.name !== dep.name) throw new Error(`Cargo direction omits a declared local target: ${pkg.owner} → ${dep.owner}`);
        }
      }
    }
  };
  validate(graph); validate(inventory);
  const signature = (packages: readonly CargoDirectionPackage[]): string => JSON.stringify(packages.map((pkg) => [pkg.name, pkg.owner, pkg.role, pkg.dependencies.map((dep) => [dep.name, dep.alias, dep.owner, dep.kind, dep.optional, dep.platform]).sort()]).sort());
  if (signature(graph) !== signature(inventory)) throw new Error("Cargo direction metadata disagrees with the independently inventoried authored declarations");
  const areas = Object.entries(object(policy.areaLayers)).sort(([a], [b]) => b.length - a.length);
  if (!areas.length || areas.some(([, layer]) => !["framework", "implementation", "repo-wide"].includes(layer)) || !Array.isArray(policy.roles) || !policy.roles.length || new Set(policy.roles).size !== policy.roles.length) throw new Error("Cargo direction requires declared physical areas and semantic roles");
  const layer = (owner: string): string | undefined => areas.find(([area]) => owner === area || owner.startsWith(`${area}/`))?.[1];
  const rules = Object.entries(object(policy.rules));
  if (!rules.length || rules.some(([, rule]) => [rule.fromRoles, rule.toRoles].some((roles) => !Array.isArray(roles) || !roles.length || roles.some((role: unknown) => typeof role !== "string" || !policy.roles.includes(role))))) throw new Error("Cargo direction policy references an unknown semantic role");
  const problems: CargoDirectionProblem[] = [], violations: CargoDirectionViolation[] = [];
  let localDependencies = 0;
  for (const pkg of graph) {
    if (!layer(pkg.owner)) problems.push({ code: "unclassified-owner", owner: pkg.owner });
    if (!pkg.role) problems.push({ code: "missing-role", owner: pkg.owner });
    else if (!policy.roles.includes(pkg.role)) problems.push({ code: "unknown-role", owner: pkg.owner });
    for (const dep of pkg.dependencies) {
      if (dep.owner === null) continue;
      const target = graph.find((pkg) => pkg.owner === dep.owner)!;
      localDependencies++;
      const forbidden = layer(pkg.owner) === "framework" && layer(target.owner) === "implementation" ? ["cargo-framework-no-implementation"] : [];
      for (const [name, rule] of rules) if (rule.fromRoles.includes(pkg.role) && rule.toRoles.includes(target.role)) forbidden.push(name);
      for (const rule of forbidden) violations.push({ rule, from: pkg.owner, to: target.owner, alias: dep.alias, kind: dep.kind, optional: dep.optional, platform: dep.platform });
    }
  }
  violations.sort((a, b) => {
    const first = JSON.stringify([a.rule, a.from, a.to, a.alias, a.kind, a.platform]), second = JSON.stringify([b.rule, b.from, b.to, b.alias, b.kind, b.platform]);
    return first < second ? -1 : first > second ? 1 : 0;
  });
  return { packages: graph.length, localDependencies, violations, problems };
}
