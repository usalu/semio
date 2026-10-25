import { lstatSync, readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { noFollowDirectoryAncestry, workspaceAuthorityPath } from "../🧭️workspace-taxonomy-locator/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

/** 🧾️ Repository-owned TOML boundary for pure Cargo provider manifest projections. */
export interface CargoProviderTomlParser {
  parse(source: string): unknown;
}

/** 🪪️ Raw provider manifest input whose locator was already established by source authority. */
export interface CargoProviderManifestProjectionInput {
  readonly locator: string;
  readonly source: string;
}

/** 🧷️ One dependency declaration retained as unapproved provider evidence. */
export interface CargoProviderDependencyProjection {
  readonly key: string;
  readonly packageOverride?: string;
  readonly localPath?: string;
  readonly workspaceInherited: boolean;
  readonly targetCondition?: string;
  readonly source: "version" | "table";
  readonly version?: string;
  readonly unsupported: Readonly<Record<string, unknown>>;
  readonly unapprovedReasons: readonly string[];
}

/** 📚️ One explicit Cargo library target, never inferred from package or filename identity. */
export interface CargoProviderLibraryProjection {
  readonly name?: string;
  readonly path?: string;
  readonly crateTypes: readonly string[];
  readonly procMacro: boolean;
}

/** 🧩️ Pure manifest facts for a later provider authority resolver. */
export interface CargoProviderManifestProjection {
  readonly locator: string;
  readonly package?: Readonly<{ readonly name: string; readonly version?: string; readonly versionInherited?: true; readonly workspaceLocator?: string }>;
  readonly library?: CargoProviderLibraryProjection;
  readonly workspaceDeclared?: true;
  readonly workspaceDependencies: readonly CargoProviderDependencyProjection[];
  readonly dependencies: readonly CargoProviderDependencyProjection[];
  readonly developmentDependencies: readonly CargoProviderDependencyProjection[];
  readonly buildDependencies: readonly CargoProviderDependencyProjection[];
}

/** 🧯️ Bun's parser is hidden behind repository-owned syntax input/output types. */
export const cargoProviderTomlParser: CargoProviderTomlParser = {
  parse(source: string): unknown {
    return runtimeParseTomlV1(source);
  },
};

function cargoProviderRecord(value: unknown, context: string): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error(`${context} must be a TOML table.`);
  return value as Readonly<Record<string, unknown>>;
}

function cargoProviderString(value: unknown, context: string): string {
  if (typeof value !== "string") throw new Error(`${context} must be a string.`);
  return value;
}

function cargoProviderBoolean(value: unknown, context: string): boolean {
  if (typeof value !== "boolean") throw new Error(`${context} must be a boolean.`);
  return value;
}

function cargoProviderTomlValue(value: unknown, context: string): unknown {
  if (value === null || typeof value === "string" || typeof value === "boolean") return value;
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (Array.isArray(value)) return value.map((entry, index) => cargoProviderTomlValue(entry, `${context}[${index}]`));
  if (typeof value === "object") {
    if (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null) throw new Error(`${context} has an unsupported TOML object.`);
    const record = cargoProviderRecord(value, context);
    return Object.fromEntries(Object.entries(record).map(([key, entry]) => [key, cargoProviderTomlValue(entry, `${context}.${key}`)]));
  }
  throw new Error(`${context} has an unsupported TOML value.`);
}

function cargoProviderMultilineString(source: string): boolean {
  let quote: "basic" | "literal" | undefined;
  for (let index = 0; index < source.length; index++) {
    const character = source[index]!;
    if (quote === "basic") {
      if (character === "\\") { index++; continue; }
      if (character === "\"") quote = undefined;
      continue;
    }
    if (quote === "literal") {
      if (character === "'") quote = undefined;
      continue;
    }
    if (character === "#") {
      while (index < source.length && source[index] !== "\n") index++;
      continue;
    }
    if (character === "\"" && source.slice(index, index + 3) === "\"\"\"") return true;
    if (character === "'" && source.slice(index, index + 3) === "'''") return true;
    if (character === "\"") quote = "basic";
    if (character === "'") quote = "literal";
  }
  return false;
}

function cargoProviderLocator(locator: string): string {
  if (typeof locator !== "string" || locator.length === 0 || locator.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(locator)) throw new Error("Provider manifest locator must be a safe repository-relative POSIX path.");
  const segments = locator.split("/");
  if (segments.some((segment) => segment.length === 0 || segment === "." || segment === ".." || /^[A-Za-z]:$/u.test(segment) || segment.toLowerCase() === "compose")) throw new Error("Provider manifest locator is unsafe or excluded.");
  if (segments.at(-1) !== "Cargo.toml") throw new Error("Provider manifest locator must name Cargo.toml.");
  return locator;
}

function cargoProviderDependency(key: string, value: unknown, scope: string, targetCondition?: string): CargoProviderDependencyProjection {
  const context = `${scope}.${key}`;
  if (typeof value === "string") return { key, workspaceInherited: false, ...(targetCondition === undefined ? {} : { targetCondition }), source: "version", version: value, unsupported: {}, unapprovedReasons: ["version-only"] };
  const record = cargoProviderRecord(value, context);
  const packageOverride = record.package === undefined ? undefined : cargoProviderString(record.package, `${context}.package`);
  const localPath = record.path === undefined ? undefined : cargoProviderString(record.path, `${context}.path`);
  const workspaceInherited = record.workspace === undefined ? false : cargoProviderBoolean(record.workspace, `${context}.workspace`);
  const version = record.version === undefined ? undefined : cargoProviderString(record.version, `${context}.version`);
  for (const field of ["git", "registry", "branch", "tag", "rev"] as const) if (record[field] !== undefined) cargoProviderString(record[field], `${context}.${field}`);
  if (workspaceInherited && [localPath, version, record.git, record.registry, record.branch, record.tag, record.rev].some((entry) => entry !== undefined)) throw new Error(`${context} mixes workspace inheritance with another source.`);
  if ([localPath, record.git, record.registry].filter((entry) => entry !== undefined).length > 1) throw new Error(`${context} mixes multiple dependency sources.`);
  const unsupported = Object.fromEntries(Object.entries(record).filter(([field]) => !["package", "path", "workspace", "version"].includes(field)).map(([field, entry]) => [field, cargoProviderTomlValue(entry, `${context}.${field}`)]));
  const unapprovedReasons = [
    ...(workspaceInherited ? ["workspace-inheritance"] : []),
    ...(record.workspace === false ? ["workspace-false"] : []),
    ...(localPath === undefined ? ["no-local-path"] : []),
    ...(version === undefined ? [] : ["version-constraint"]),
    ...(["git", "registry", "branch", "tag", "rev"].some((field) => record[field] !== undefined) ? ["external-source"] : []),
    ...(Object.keys(unsupported).length === 0 ? [] : ["unsupported-fields"]),
  ];
  return { key, ...(packageOverride === undefined ? {} : { packageOverride }), ...(localPath === undefined ? {} : { localPath }), workspaceInherited, ...(targetCondition === undefined ? {} : { targetCondition }), source: "table", ...(version === undefined ? {} : { version }), unsupported, unapprovedReasons };
}

function cargoProviderDependencies(value: unknown, scope: string, targetCondition?: string): readonly CargoProviderDependencyProjection[] {
  if (value === undefined) return [];
  const record = cargoProviderRecord(value, scope);
  return Object.entries(record).map(([key, entry]) => cargoProviderDependency(key, entry, scope, targetCondition));
}

function cargoProviderLibrary(value: unknown): CargoProviderLibraryProjection | undefined {
  if (value === undefined) return undefined;
  const record = cargoProviderRecord(value, "lib");
  const crateTypesValue = record["crate-type"];
  if (crateTypesValue !== undefined && (!Array.isArray(crateTypesValue) || crateTypesValue.some((entry) => typeof entry !== "string"))) throw new Error("lib.crate-type must be an array of strings.");
  return {
    ...(record.name === undefined ? {} : { name: cargoProviderString(record.name, "lib.name") }),
    ...(record.path === undefined ? {} : { path: cargoProviderString(record.path, "lib.path") }),
    crateTypes: crateTypesValue === undefined ? [] : [...crateTypesValue],
    procMacro: record["proc-macro"] === undefined ? false : cargoProviderBoolean(record["proc-macro"], "lib.proc-macro"),
  };
}

/** 🧭️ Projects one caller-supplied Cargo manifest without resolving paths, aliases, or graph authority. */
export function projectCargoProviderManifest(input: CargoProviderManifestProjectionInput, parser: CargoProviderTomlParser = cargoProviderTomlParser): CargoProviderManifestProjection {
  const locator = cargoProviderLocator(input.locator);
  if (typeof input.source !== "string") throw new Error("Provider manifest source must be a string.");
  if (cargoProviderMultilineString(input.source)) throw new Error("Provider manifest multiline strings are unsupported.");
  let parsed: unknown;
  try {
    parsed = parser.parse(input.source);
  } catch (error) {
    throw new Error(`Provider manifest TOML is invalid: ${error instanceof Error ? error.message : String(error)}`);
  }
  const root = cargoProviderRecord(parsed, "manifest");
  const packageProjection = root.package === undefined ? undefined : (() => {
    const table = cargoProviderRecord(root.package, "package");
    const version = table.version;
    const versionProjection = version === undefined ? {} : typeof version === "string"
      ? { version }
      : (() => {
          const inheritance = cargoProviderRecord(version, "package.version");
          if (Object.keys(inheritance).length !== 1 || inheritance.workspace !== true) throw new Error("package.version must be a string or exact workspace inheritance.");
          return { versionInherited: true as const };
        })();
    const workspaceLocator = table.workspace === undefined ? undefined : cargoProviderString(table.workspace, "package.workspace");
    return { name: cargoProviderString(table.name, "package.name"), ...versionProjection, ...(workspaceLocator === undefined ? {} : { workspaceLocator }) };
  })();
  const target = root.target === undefined ? {} : cargoProviderRecord(root.target, "target");
  const targetTables = Object.entries(target).map(([condition, entry]) => [condition, cargoProviderRecord(entry, `target.${condition}`)] as const);
  const targetDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table.dependencies, `target.${condition}.dependencies`, condition));
  const targetDevelopmentDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table["dev-dependencies"], `target.${condition}.dev-dependencies`, condition));
  const targetBuildDependencies = targetTables.flatMap(([condition, table]) => cargoProviderDependencies(table["build-dependencies"], `target.${condition}.build-dependencies`, condition));
  const workspace = root.workspace === undefined ? {} : cargoProviderRecord(root.workspace, "workspace");
  const library = cargoProviderLibrary(root.lib);
  return {
    locator,
    ...(packageProjection === undefined ? {} : { package: packageProjection }),
    ...(library === undefined ? {} : { library }),
    ...(root.workspace === undefined ? {} : { workspaceDeclared: true as const }),
    workspaceDependencies: cargoProviderDependencies(workspace.dependencies, "workspace.dependencies"),
    dependencies: [...cargoProviderDependencies(root.dependencies, "dependencies"), ...targetDependencies],
    developmentDependencies: [...cargoProviderDependencies(root["dev-dependencies"], "dev-dependencies"), ...targetDevelopmentDependencies],
    buildDependencies: [...cargoProviderDependencies(root["build-dependencies"], "build-dependencies"), ...targetBuildDependencies],
  };
}

/** 🔗️ Exact selected dependency input for a local Cargo provider binding proof. */
export interface CargoProviderBindingInput {
  readonly workspaceRoot: string;
  readonly consumerManifestLocator: string;
  readonly dependencyKey: string;
}

/** 🔑️ Bounded provider identity evidence for one selected local normal dependency. */
export interface CargoProviderBinding {
  readonly consumerManifestLocator: string;
  readonly workspaceManifestLocator: string;
  readonly providerManifestLocator: string;
  readonly librarySourceLocator: string;
  readonly packageName: string;
  readonly libraryName: string;
  readonly procMacro: boolean;
  readonly dependencyKey: string;
  readonly workspaceInherited: boolean;
  readonly pathAuthority: "consumer" | "workspace";
  readonly externName: string;
}

function cargoProviderBindingLocator(locator: string, finalName?: string): string {
  if (typeof locator !== "string" || locator.length === 0 || locator !== locator.normalize("NFC") || locator.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(locator)) throw new Error("Cargo provider binding locator is unsafe.");
  const segments = locator.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose") || finalName !== undefined && segments.at(-1) !== finalName) throw new Error("Cargo provider binding locator is excluded or malformed.");
  return locator;
}

function cargoProviderBindingWorkspaceRoot(root: string): string {
  if (typeof root !== "string" || root.length === 0 || root !== root.normalize("NFC") || !isAbsolute(root) || /[\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(root)) throw new Error("Cargo provider workspace root is unsafe.");
  const windows = /^[A-Za-z]:[\\/]/u.test(root), posixRoot = root.startsWith("/");
  if ((!windows && !posixRoot) || root.includes(":") && (!windows || root.indexOf(":") !== 1 || root.lastIndexOf(":") !== 1) || posixRoot && root.includes("\\")) throw new Error("Cargo provider workspace root is unsafe.");
  const segments = root.split(/[\\/]/u);
  if (segments.some((segment) => segment === "." || segment === ".." || segment.toLocaleLowerCase("en-US") === "compose")) throw new Error("Cargo provider workspace root is excluded.");
  const authority = workspaceAuthorityPath(root, "Cargo provider workspace root");
  noFollowDirectoryAncestry(authority, "Cargo provider workspace root ancestry");
  return authority;
}

function cargoProviderBindingRead(root: string, locator: string): string {
  const safe = cargoProviderBindingLocator(locator);
  let current = root;
  for (const [index, segment] of safe.split("/").entries()) {
    current = join(current, segment);
    const state = lstatSync(current);
    if (state.isSymbolicLink() || (index + 1 === safe.split("/").length ? !state.isFile() : !state.isDirectory())) throw new Error("Cargo provider binding source is not a no-follow regular file.");
  }
  return readFileSync(current, "utf8");
}

function cargoProviderBindingDirectory(root: string, segments: readonly string[]): void {
  const path = join(root, ...segments);
  const state = lstatSync(path);
  if (state.isSymbolicLink() || !state.isDirectory()) throw new Error("Cargo provider dependency path is not a no-follow directory.");
}

function cargoProviderBindingPath(root: string, baseLocator: string, path: string, finalName: string, pathPointsToFile = false): string {
  if (typeof path !== "string" || path.length === 0 || path !== path.normalize("NFC") || path.includes("\\") || /[:\u0000-\u001F\u007F\u0085\u2028\u2029]/u.test(path) || path.startsWith("/")) throw new Error("Cargo provider dependency path is unsafe.");
  const stack = baseLocator.split("/").slice(0, -1);
  const parts = path.split("/");
  for (const [index, segment] of parts.entries()) {
    if (!segment || segment === "." || segment.toLocaleLowerCase("en-US") === "compose") throw new Error("Cargo provider dependency path is excluded.");
    if (segment === "..") { if (stack.length === 0) throw new Error("Cargo provider dependency path escapes the workspace."); stack.pop(); }
    else {
      stack.push(segment);
      if (!pathPointsToFile || index + 1 !== parts.length) cargoProviderBindingDirectory(root, stack);
    }
  }
  return cargoProviderBindingLocator([...(pathPointsToFile ? stack : [...stack, finalName])].join("/"), finalName);
}

function cargoProviderBindingDependencySettings(dependency: CargoProviderDependencyProjection): boolean {
  const unsupported = dependency.unsupported;
  if (unsupported.features !== undefined && (!Array.isArray(unsupported.features) || unsupported.features.some((feature) => typeof feature !== "string"))) return false;
  if (unsupported["default-features"] !== undefined && typeof unsupported["default-features"] !== "boolean") return false;
  return Object.keys(unsupported).every((field) => field === "features" || field === "default-features");
}

function cargoProviderBindingNearestWorkspace(root: string, manifestLocator: string, manifest: CargoProviderManifestProjection): string {
  if (manifest.workspaceDeclared) return manifestLocator;
  const directory = manifestLocator.split("/").slice(0, -1);
  for (; directory.length >= 0; directory.pop()) {
    const locator = [...directory, "Cargo.toml"].join("/");
    try {
      const ancestor = projectCargoProviderManifest({ locator, source: cargoProviderBindingRead(root, locator) });
      if (ancestor.workspaceDeclared) return locator;
    } catch (error) {
      if (!(error instanceof Error) || !/ENOENT/u.test(error.message)) throw error;
    }
    if (directory.length === 0) break;
  }
  throw new Error("Cargo manifest has no governing workspace on its bounded ancestor chain.");
}

/** 🔎️ Resolves one selected normal local dependency without following a Cargo graph. */
export function resolveCargoProviderBinding(input: CargoProviderBindingInput): CargoProviderBinding {
  const consumerManifestLocator = cargoProviderBindingLocator(input.consumerManifestLocator, "Cargo.toml");
  if (typeof input.dependencyKey !== "string" || input.dependencyKey.length === 0) throw new Error("Cargo provider dependency key is required.");
  const root = cargoProviderBindingWorkspaceRoot(input.workspaceRoot);
  const workspaceManifestLocator = "Cargo.toml", workspace = projectCargoProviderManifest({ locator: workspaceManifestLocator, source: cargoProviderBindingRead(root, workspaceManifestLocator) });
  if (!workspace.workspaceDeclared) throw new Error("Cargo provider workspace manifest must declare [workspace].");
  const consumer = projectCargoProviderManifest({ locator: consumerManifestLocator, source: cargoProviderBindingRead(root, consumerManifestLocator) });
  if (!consumer.package) throw new Error("Cargo consumer manifest requires [package].");
  if (cargoProviderBindingNearestWorkspace(root, consumerManifestLocator, consumer) !== workspaceManifestLocator) throw new Error("Cargo consumer nearest workspace disagrees with the supplied workspace.");
  if (consumer.package.workspaceLocator !== undefined && cargoProviderBindingPath(root, consumerManifestLocator, consumer.package.workspaceLocator, "Cargo.toml") !== workspaceManifestLocator) throw new Error("Cargo consumer explicit workspace locator disagrees with the supplied workspace.");
  const selected = consumer.dependencies.filter((dependency) => dependency.key === input.dependencyKey);
  if (selected.length !== 1 || selected[0]!.targetCondition !== undefined) throw new Error("Cargo selected dependency must occur exactly once in unconditional normal dependencies.");
  const consumerDependency = selected[0]!;
  if (!cargoProviderBindingDependencySettings(consumerDependency) || consumerDependency.unapprovedReasons.includes("workspace-false") || consumerDependency.workspaceInherited && consumerDependency.packageOverride !== undefined) throw new Error("Cargo selected consumer dependency settings are unsupported.");
  const dependency = consumerDependency.workspaceInherited ? (() => {
    const inherited = workspace.workspaceDependencies.filter((entry) => entry.key === input.dependencyKey);
    if (inherited.length !== 1) throw new Error("Cargo workspace dependency inheritance is unresolved or ambiguous.");
    return inherited[0]!;
  })() : consumerDependency;
  if (dependency.localPath === undefined || dependency.version !== undefined || !cargoProviderBindingDependencySettings(dependency) || dependency.unapprovedReasons.includes("workspace-false")) throw new Error("Cargo selected dependency is not a supported local path authority.");
  const pathAuthority = consumerDependency.workspaceInherited ? "workspace" as const : "consumer" as const;
  const providerManifestLocator = cargoProviderBindingPath(root, pathAuthority === "workspace" ? workspaceManifestLocator : consumerManifestLocator, dependency.localPath, "Cargo.toml");
  const provider = projectCargoProviderManifest({ locator: providerManifestLocator, source: cargoProviderBindingRead(root, providerManifestLocator) });
  if (cargoProviderBindingNearestWorkspace(root, providerManifestLocator, provider) !== workspaceManifestLocator) throw new Error("Cargo provider nearest workspace disagrees with the supplied workspace.");
  if (provider.package?.workspaceLocator !== undefined && cargoProviderBindingPath(root, providerManifestLocator, provider.package.workspaceLocator, "Cargo.toml") !== workspaceManifestLocator) throw new Error("Cargo provider explicit workspace locator disagrees with the supplied workspace.");
  const packageName = dependency.packageOverride ?? dependency.key;
  if (provider.package?.name !== packageName) throw new Error("Cargo selected dependency package identity disagrees with the provider manifest.");
  if (!provider.library?.name || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(provider.library.name) || !provider.library.path) throw new Error("Cargo provider requires an explicit valid library name and source path.");
  const librarySourceLocator = cargoProviderBindingPath(root, providerManifestLocator, provider.library.path, provider.library.path.split("/").at(-1)!, true);
  cargoProviderBindingRead(root, librarySourceLocator);
  return { consumerManifestLocator, workspaceManifestLocator, providerManifestLocator, librarySourceLocator, packageName, libraryName: provider.library.name, procMacro: provider.library.procMacro, dependencyKey: dependency.key, workspaceInherited: consumerDependency.workspaceInherited, pathAuthority, externName: dependency.packageOverride === undefined ? provider.library.name : dependency.key.replaceAll("-", "_") };
}
