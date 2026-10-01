import invocationSchema from "./🧬️schema/🏃️invocation/🔣️.json";
import { existsSync, lstatSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";

export type CargoWorkspaceContributionV1 = Readonly<{ schemaVersion: 1; members: readonly string[]; exclude: readonly string[] }>;
export type CargoWorkspaceScope = Readonly<{ directory: string; manifest: string; lock: string; contribution: CargoWorkspaceContributionV1; memberManifests: readonly string[] }>;
export type CargoWorkspacePackage = Readonly<{ directory: string; manifest: string; name: string; workspace: string }>;
const slash = (value: string): string => value.replaceAll("\\", "/");
const pathPattern = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !isAbsolute(value) && !/^[A-Za-z]:\//u.test(value) && !value.includes("\\") && !value.split("/").includes("..");
const object = (value: unknown): Record<string, any> => { if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Cargo workspace requires an object"); return value as Record<string, any>; };

/** 🧬️ Admits the language-neutral physical membership contract without optional owner identities. */
export function parseCargoWorkspaceContribution(value: unknown): CargoWorkspaceContributionV1 {
  const row = object(value);
  if (Object.keys(row).some(k => !["schemaVersion", "members", "exclude"].includes(k)) || row.schemaVersion !== 1 || !Array.isArray(row.members) || !row.members.length || !Array.isArray(row.exclude)) throw new Error("Invalid Cargo workspace contribution");
  for (const values of [row.members, row.exclude]) if (values.some(v => !pathPattern(v)) || new Set(values).size !== values.length) throw new Error("Invalid Cargo workspace member patterns");
  return { schemaVersion: 1, members: [...row.members], exclude: [...row.exclude] };
}

/** 📁️ Refuses escaped or symlink-backed manifest ancestry before reading owner authority. */
function physical(root: string, path: string): string {
  const full = resolve(root, path), rel = slash(relative(root, full));
  if (rel.startsWith("../") || isAbsolute(rel)) throw new Error(`Cargo workspace path escapes repository: ${path}`);
  let current = resolve(root);
  for (const part of rel.split("/").filter(Boolean)) { current = join(current, part); if (lstatSync(current).isSymbolicLink()) throw new Error(`Cargo workspace path contains a symlink: ${path}`); }
  if (!lstatSync(full).isFile()) throw new Error(`Cargo manifest is not a regular file: ${path}`);
  return full;
}
const read = (root: string, path: string): Record<string, any> => object(Bun.TOML.parse(readFileSync(physical(root, path), "utf8")));
const scope = (directory: string, workspace: Record<string, any>): CargoWorkspaceScope => {
  const admission = workspace.metadata?.semio?.repository;
  if (admission !== undefined && (Object.keys(object(admission)).some(k => !["schema-version", "owner-manifests", "member-manifests"].includes(k)) || admission["schema-version"] !== 1)) throw new Error(`Unknown repository workspace admission at ${directory}`);
  const memberManifests = admission?.["member-manifests"];
  if (!Array.isArray(memberManifests) || !memberManifests.length || memberManifests.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(memberManifests).size !== memberManifests.length) throw new Error(`Invalid native member manifest authority: ${directory}`);
  const prefix = directory === "." ? "" : `${directory}/`;
  return { directory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests, contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: workspace.exclude ?? [] }) };
};

/** 🗂️ Discovers only present, explicitly authored repository workspaces through physical Cargo documents. */
export function discoverCargoWorkspaces(root: string): readonly CargoWorkspaceScope[] {
  const rootDocument = read(root, "Cargo.toml");
  const patterns = rootDocument.workspace?.metadata?.semio?.repository?.["owner-manifests"] ?? [];
  if (!Array.isArray(patterns) || patterns.some(p => !pathPattern(p) || !p.endsWith("Cargo.toml")) || new Set(patterns).size !== patterns.length) throw new Error("Invalid repository workspace owner patterns");
  const files = [...new Set(["Cargo.toml", ...patterns.flatMap(pattern => [...new Bun.Glob(pattern).scanSync({ cwd: root, dot: true, onlyFiles: true, followSymlinks: false })])])];
  const found: CargoWorkspaceScope[] = [];
  for (const path of files.sort()) {
    if (path.split("/").some(p => p.startsWith(".") || ["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "temp", "compose"].includes(p))) continue;
    const document = read(root, path);
    if (path === "Cargo.toml" || document.workspace?.metadata?.semio?.repository !== undefined) found.push(scope(slash(dirname(path)), object(document.workspace)));
  }
  if (!found.some(row => row.directory === ".")) throw new Error("Root Cargo workspace lacks authored repository admission");
  return found.sort((a, b) => a.manifest.localeCompare(b.manifest));
}

/** 🦀️ Expands each admitted Cargo-native pattern against current physical manifests and rejects absent exact owners. */
export function cargoWorkspaceMembers(root: string, owner: CargoWorkspaceScope): readonly CargoWorkspacePackage[] {
  const cwd = resolve(root, owner.directory), paths = new Set<string>();
  const patterns = owner.memberManifests.map(pattern => new Bun.Glob(pattern));
  const leaves = owner.memberManifests.map(pattern => new Bun.Glob(pattern.slice(0, -"/Cargo.toml".length)));
  const prefixes = owner.memberManifests.map(pattern => pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u, ""));
  const excluded = owner.contribution.exclude.map(pattern => new Bun.Glob(pattern));
  const opaque = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "🗑️generated"]);
  const walk = (directory: string): void => {
    if (directory && !prefixes.some(prefix => !prefix || directory.startsWith(prefix + "/") || prefix === directory || prefix.startsWith(directory + "/"))) return;
    if (directory && leaves.some(pattern => pattern.match(directory))) {
      const manifest = join(cwd, directory, "Cargo.toml");
      if (existsSync(manifest)) { physical(root, slash(relative(root,manifest))); paths.add(slash(relative(root,manifest))); return; }
    }
    for (const name of readdirSync(join(cwd,directory))) {
      const local=directory?`${directory}/${name}`:name;
      if(name.startsWith(".") || opaque.has(name) || excluded.some((pattern,index)=>pattern.match(local) || pattern.match(`${local}/`) || owner.contribution.exclude[index]!.endsWith("/**") && new Bun.Glob(owner.contribution.exclude[index]!.slice(0,-3)).match(local)))continue;
      const entry=lstatSync(join(cwd,local));
      if(entry.isDirectory())walk(local);
      else if(entry.isFile() && name==="Cargo.toml" && local!=="Cargo.toml" && patterns.some(pattern=>pattern.match(local)) && !excluded.some(pattern=>pattern.match(slash(dirname(local)))))paths.add(slash(relative(root,join(cwd,local))));
    }
  };
  walk("");
  if (!paths.size) throw new Error(`Cargo workspace has no current source-bound members: ${owner.manifest}`);
  const names = new Set<string>();
  return [...paths].sort().map(manifest => {
    const document = read(root, manifest), name = document.package?.name;
    if (typeof name !== "string" || !name || names.has(name)) throw new Error(`Cargo workspace member has a missing or duplicate name: ${manifest}`);
    names.add(name);
    const selected = cargoWorkspaceForManifest(root, manifest);
    if (selected.directory !== owner.directory) throw new Error(`Cargo member belongs to another workspace: ${manifest}`);
    return { directory: slash(dirname(manifest)), manifest, name, workspace: owner.directory };
  });
}

/** 🧭️ Selects the actual nearest Cargo workspace for a physical package, including isolated native test workspaces. */
export function cargoWorkspaceForManifest(root: string, manifest: string): CargoWorkspaceScope {
  const full = physical(root, manifest), packageRow = read(root, manifest).package;
  let directory = packageRow?.workspace ? resolve(dirname(full), packageRow.workspace) : dirname(full);
  while (true) {
    const path = join(directory, "Cargo.toml"), relativePath = slash(relative(root, path));
    if (relativePath.startsWith("../") || isAbsolute(relativePath)) throw new Error(`Cargo workspace escapes repository: ${manifest}`);
    if (existsSync(path)) {
      const workspace = read(root, relativePath).workspace;
      if (workspace) {
        const relativeDirectory = slash(relative(root, directory)) || ".", prefix = relativeDirectory === "." ? "" : `${relativeDirectory}/`;
        return workspace.metadata?.semio?.repository !== undefined ? scope(relativeDirectory, object(workspace)) : { directory: relativeDirectory, manifest: `${prefix}Cargo.toml`, lock: `${prefix}Cargo.lock`, memberManifests: (workspace.members ?? ["."]).map((path: string) => `${path}/Cargo.toml`), contribution: parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members ?? ["."], exclude: workspace.exclude ?? [] }) };
      }
    }
    if (directory === resolve(root)) throw new Error(`Cargo package has no workspace: ${manifest}`);
    directory = dirname(directory);
  }
}

/** 📦️ Inventories all current repository member identities without returning stale removed owners. */
export function cargoRepositoryPackages(root: string): readonly CargoWorkspacePackage[] {
  const packages = discoverCargoWorkspaces(root).flatMap(owner => cargoWorkspaceMembers(root, owner));
  const names = new Set<string>();
  for (const row of packages) { if (names.has(row.name)) throw new Error(`Duplicate repository Cargo package: ${row.name}`); names.add(row.name); }
  return packages;
}

/** 🎯️ Binds a named current repository package to its physical manifest and selected native workspace. */
export function cargoRepositoryPackage(root: string, name: string): CargoWorkspacePackage {
  const row = cargoRepositoryPackages(root).find(row => row.name === name);
  if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`);
  return row;
}

/** 📐️ Checks declared native member patterns independently of physical package discovery. */
export function cargoWorkspaceDeclaresMemberV1(source: string, directory: string): boolean {
  const workspace = object(object(Bun.TOML.parse(source)).workspace);
  const contribution = parseCargoWorkspaceContribution({ schemaVersion: 1, members: workspace.members, exclude: workspace.exclude ?? [] });
  return contribution.members.some(pattern => new Bun.Glob(pattern).match(directory)) && !contribution.exclude.some(pattern => new Bun.Glob(pattern).match(directory));
}

/** 🎛️ Binds package selectors to one current physical workspace and preserves explicit Cargo manifest control. */
export function selectedCargoArguments(root: string, args: readonly string[]): string[] {
  if (args.some(arg => arg === "--manifest-path" || arg.startsWith("--manifest-path="))) return [...args];
  const names: string[] = [];
  for (let index = 1; index < args.length; index++) {
    const arg = args[index]!;
    if (arg === "--") break;
    if (arg === "-p" || arg === "--package") { const value = args[++index]; if (!value) throw new Error("Cargo package selector requires a current name"); names.push(value); }
    else if (arg.startsWith("--package=")) names.push(arg.slice(10));
  }
  if (!names.length) return [...args];
  const packages = cargoRepositoryPackages(root), rows = names.map(name => { const row = packages.find(row => row.name === name); if (!row) throw new Error(`Unknown current repository Cargo package: ${name}`); return row; });
  if (new Set(rows.map(row => row.workspace)).size !== 1) throw new Error("Cargo command selects packages from different workspaces; invoke each exact owner separately");
  return [args[0]!, "--manifest-path", join(root, rows[0]!.manifest), ...args.slice(1)];
}

/** 📣️ Publishes only the admitted current member array, preserving every other Cargo declaration. */
export function publishCargoWorkspaceMembership(root: string, owner: CargoWorkspaceScope, mode: "check" | "write"): boolean {
  const path = physical(root, owner.manifest), source = readFileSync(path, "utf8");
  const members = cargoWorkspaceMembers(root, owner).map(row => slash(relative(resolve(root, owner.directory), resolve(root, row.directory))));
  const document = object(Bun.TOML.parse(source)), actual = document.workspace?.members;
  if (JSON.stringify(actual) === JSON.stringify(members)) return false;
  if (mode === "check") throw new Error(`Cargo source membership is stale: ${owner.manifest}`);
  const heading = /^\[workspace\]\r?$/m.exec(source);
  if (!heading) throw new Error(`Cargo workspace table is absent: ${owner.manifest}`);
  const bodyStart = heading.index + heading[0].length, next = /^\[/m.exec(source.slice(bodyStart));
  const section = source.slice(bodyStart, next ? bodyStart + next.index : source.length), field = /^members\s*=\s*\[/m.exec(section);
  if (!field) throw new Error(`Cargo workspace member field is absent: ${owner.manifest}`);
  const start = bodyStart + field.index;
  let end = bodyStart + field.index + field[0].length, quote = "", escaped = false;
  for (; end < source.length; end++) {
    const character = source[end]!;
    if (quote) { if (escaped) escaped = false; else if (quote === '"' && character === "\\") escaped = true; else if (character === quote) quote = ""; }
    else if (character === '"' || character === "'") quote = character;
    else if (character === "]") { end++; break; }
  }
  if (end >= source.length) throw new Error(`Unterminated Cargo workspace member array: ${owner.manifest}`);
  const replacement = `members = [\n${members.map(path => `    ${JSON.stringify(path)},`).join("\n")}\n]`;
  if (readFileSync(physical(root, owner.manifest), "utf8") !== source) throw new Error(`Cargo workspace changed during member discovery: ${owner.manifest}`);
  writeFileSync(path, source.slice(0,start) + replacement + source.slice(end));
  return true;
}

export type CargoPreparationV1 = Readonly<{ script: string; command: readonly string[] }>;
/** 🧬️ Admits an owner-authored executable recipe with literal argv and no implicit producer identity. */
export function parseCargoPreparation(value: unknown): CargoPreparationV1 {
 const row=object(value);if(Object.keys(row).some(k=>!["script","command"].includes(k)) || (typeof row.script!=="string" || !row.script || isAbsolute(row.script) || /^[A-Za-z]:\//.test(row.script) || row.script.includes("\\")) || row.script.split("/").at(-1)!=="📜️script.ts" || !Array.isArray(row.command) || !row.command.length || row.command.some((v:unknown)=>typeof v!=="string" || !v || v.includes("\0")))throw new Error("Invalid Cargo preparation recipe");return {script:row.script,command:[...row.command]};
}
/** 🔗️ Selects only native scopes reached by the current workspace's authored local dependency paths. */
export function prepareCargoOwners(root: string, selected: CargoWorkspaceScope): void {
 const scopes=discoverCargoWorkspaces(root), reachable=new Set([selected.directory]), executed=new Set<string>();
 for(const directory of reachable){const owner=scopes.find(s=>s.directory===directory);if(!owner)throw new Error(`Unknown selected Cargo scope: ${directory}`);const authority=read(root,owner.manifest).workspace;
 for(const pkg of cargoWorkspaceMembers(root,owner)){const document=read(root,pkg.manifest), groups=[document,...Object.values(document.target??{})] as Record<string,any>[];
 for(const group of groups)for(const kind of ["dependencies","dev-dependencies","build-dependencies"])for(const [alias,value] of Object.entries(group[kind]??{})){const entry=value as any;if(!entry || typeof entry!=="object")continue;const dependency=entry.workspace?authority.dependencies?.[alias]:entry;if(!dependency)throw new Error(`Missing inherited Cargo dependency: ${pkg.manifest} ${alias}`);if(typeof dependency.path!=="string")continue;const path=resolve(root,entry.workspace?owner.directory:pkg.directory,dependency.path), local=slash(relative(root,path));if(local.startsWith("../") || isAbsolute(local))throw new Error(`Cargo dependency escapes preparation authority: ${alias}`);const child=scopes.filter(s=>s.directory==="." || local===s.directory || local.startsWith(s.directory+"/")).sort((a,b)=>b.directory.length-a.directory.length)[0];if(child)reachable.add(child.directory);}
 const value=document.package?.metadata?.semio?.preparation;if(value===undefined)continue;const recipe=parseCargoPreparation(value), script=physical(root,slash(relative(root,resolve(root,pkg.directory,recipe.script)))), key=JSON.stringify([script,...recipe.command]);if(slash(relative(resolve(root,owner.directory),script)).startsWith("../"))throw new Error(`Cargo preparation leaves its owner workspace: ${pkg.manifest}`);if(executed.has(key))continue;executed.add(key);
 console.log(`[cargo-preparation] ${pkg.manifest} ${recipe.command.join(" ")}`);
 const result=Bun.spawnSync([process.execPath,script,...recipe.command],{cwd:resolve(root,pkg.directory),env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_ACTIVE:script},stdout:"pipe",stderr:"inherit",timeout:30_000});if(result.stdout.byteLength)process.stderr.write(result.stdout);if(result.exitCode!==0)throw new Error(`Cargo owner preparation failed: ${pkg.manifest} (${result.exitCode})`);
 }
 }
}

/** 🦀️ Uses the owned schema authority for native operations that consume current package inputs. */
export function cargoCommandRequiresOwnerPreparationV1(command: string): boolean {
  return invocationSchema.allOf[0]!.if.properties.command.enum.includes(command);
}

/** 🛠️ Refreshes the selected native workspace before each repository Cargo operation. */
export function prepareCargoWorkspaceInvocation(root: string, args: readonly string[], cwd: string): void {
  if (!cargoCommandRequiresOwnerPreparationV1(args[0] ?? "")) return;
  const index = args.indexOf("--manifest-path"), inline = args.find(arg => arg.startsWith("--manifest-path="));
  const selected = index >= 0 ? args[index+1] : inline?.slice(16);
  const path = selected ? resolve(cwd, selected) : join(cwd, "Cargo.toml");
  if (process.env.SEMIO_CARGO_PREPARATION_ACTIVE) throw new Error(`Cargo recursion in owner preparation: ${process.env.SEMIO_CARGO_PREPARATION_ACTIVE}`);
  const owner = cargoWorkspaceForManifest(root, slash(relative(root, path))), source = read(root, owner.manifest);
  if (source.workspace?.metadata?.semio?.repository !== undefined) {
    const result=Bun.spawnSync([process.execPath,fileURLToPath(new URL("./📜️script.ts",import.meta.url)),"prepare","--manifest",owner.manifest],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"inherit"});
    if (result.stdout.byteLength) process.stderr.write(result.stdout);
    if(result.exitCode!==0)throw new Error(`Selected Cargo preparation failed: ${owner.manifest} (${result.exitCode})`);
  }
}
