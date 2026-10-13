import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { observedFs } from "../👁️observation/🟨️.mjs";

const { existsSync, readdirSync } = observedFs;

/** 📥️ Projects source and compiler input ownership independently of target orchestration. */
export function inferProjectInputs(json, root, workspaceRoot, facts, scripts, context) {
  const { POLICY, TAXONOMY, LIBRARY_ROOT, SCRIPT_BASENAME, cargoSourceInputs, goManifest, nxPath, relativeScriptInputs, resolveOutputPath, exampleCollectionInputs, workspaceNegationClosed, declaredSourceInputs, owned } = context;
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  const tools = ["javascript"];
  if (existsSync(join(workspaceRoot, root, "Cargo.toml")) || json.targets?.wasm) tools.push("cargo");
  if (json.targets?.wasm || json.targets?.["extension-package"] || (tools.includes("cargo") && json.targets?.package)) tools.push("wasm");
  if (goManifest(root, workspaceRoot)) tools.push("go");
  if (existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");
  if (readdirSync(join(workspaceRoot, root)).some((file) => /\.[cfv]sproj$/.test(file))) tools.push("dotnet");
  if (existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");
  const nativeSources = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, false) : undefined;
  const nativeTests = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, true) : undefined;
  const runner = nxPath(relative(workspaceRoot, LIBRARY_ROOT));
  const inputs = [
    "{projectRoot}/**/*",
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
  const deliveryOffsets = TAXONOMY.testDeliveryScopeDirectoryNames.map((name) => root.indexOf(`/${name}/`)).filter((index) => index >= 0);
  const owner = deliveryOffsets.length > 0 ? root.slice(0, Math.min(...deliveryOffsets)) : root;
  if (owner !== root) {
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit,sql}";
    const native = nativeTests;
    inputs.push(...(native ? [...native, `{workspaceRoot}/${owner}/**/*.{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}`] : [`{workspaceRoot}/${owner}/**/*.${extensions}`]));
  }
  for (const tool of tools) {
    const contract = POLICY.toolchains[tool];
    inputs.push(...contract.files.map((file) => `{workspaceRoot}/${file}`));
    inputs.push(...contract.environment.map((env) => ({ env })));
    inputs.push(...contract.commands.map((runtime) => ({ runtime })));
  }
  if (tools.includes("python")) inputs.push({ runtime: `uv run --locked --no-sync --project ${JSON.stringify(root)} python --version` });
  for (const directory of POLICY.generatedDirectories) {
    inputs.push(`!{projectRoot}/**/${directory}/**/*`);
    if (owner !== root) inputs.push(`!{workspaceRoot}/${owner}/**/${directory}/**/*`);
  }
  if (owner !== runner) inputs.push(`!{workspaceRoot}/${runner}/**/🧪️tests/**/*`);
  const examples = [...exampleCollectionInputs("{projectRoot}"), ...exampleCollectionInputs("{workspaceRoot}")];
  const production = ["!{projectRoot}/**/🧪️tests/**/*", ...examples, "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];
  if (owner !== root) production.push(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`);
  const declarations = json.namedInputs ?? {};
  const exclusions = [];
  for (const target of Object.values(json.targets ?? {})) for (const output of target.outputs ?? []) {
    const absolute = resolveOutputPath(output, root, workspaceRoot);
    if (!absolute) continue;
    const path = nxPath(relative(workspaceRoot, absolute));
    if (path === "" || path.startsWith("../")) throw new Error(`Invalid output ownership for ${json.name}: ${output}`);
    exclusions.push(`!{workspaceRoot}/${path}`, `!{workspaceRoot}/${path}/**/*`);
    if (owned(path, root)) {
      const local = nxPath(relative(join(workspaceRoot, root), absolute));
      exclusions.push(`!{projectRoot}/${local}`, `!{projectRoot}/${local}/**/*`);
    }
  }
  if (owner !== root && tools.includes("cargo")) {
    for (const glob of POLICY.ephemeralOwnerGlobs ?? []) {
      exclusions.push(`!{workspaceRoot}/${owner}/${glob}`, `!{workspaceRoot}/${owner}/${glob}/**/*`);
    }
  }
  const native = (sources, productionOnly = false) => !tools.includes("cargo") ? ["production"] : [...new Set(sources ? [`{workspaceRoot}/${nativeRoot}/Cargo.toml`, ...sources] : ["{projectRoot}/**/*", ...(owner !== root ? [`{workspaceRoot}/${owner}/**/*`] : [])]), ...(productionOnly ? ["!{projectRoot}/**/🧪️tests/**/*", ...examples, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`] : [])] : []), ...POLICY.generatedDirectories.flatMap((directory) => [`!{projectRoot}/**/${directory}/**/*`, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/${directory}/**/*`] : [])]), ...exclusions];
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript") && owner !== root && existsSync(join(workspaceRoot, root, SCRIPT_BASENAME));
  const artifactSource = join(workspaceRoot, owner, "🟦️.ts");
  const artifactSources = artifactTypeScript ? [...relativeScriptInputs([artifactSource], workspaceRoot, scripts), "{projectRoot}/package.json", ...exclusions] : [];
  const javascript = POLICY.toolchains.javascript;
  const artifactCommandSources = artifactTypeScript ? [...relativeScriptInputs([join(workspaceRoot, root, SCRIPT_BASENAME)], workspaceRoot, scripts), `{workspaceRoot}/bunfig.toml`, { externalDependencies: ["typescript"] }, ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime })), { runtime: 'node -p "process.platform.concat(process.arch)"' }] : [];
  return { ...declarations, ...declaredSourceInputs(json, workspaceRoot), default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...inputs, ...(declarations.default ?? []), ...exclusions, ...production, ...(declarations.production ?? [])], nativeSources: workspaceNegationClosed([...native(nativeSources, true), ...(declarations.nativeSources ?? [])]), nativeTestSources: workspaceNegationClosed([...native(nativeTests), ...(declarations.nativeSources ?? []), ...(declarations.nativeTestSources ?? [])]), ...(artifactTypeScript ? { artifactSources: [...artifactSources, ...(declarations.artifactSources ?? [])], artifactCommandSources: [...artifactCommandSources, ...(declarations.artifactCommandSources ?? [])] } : {}) };
}

