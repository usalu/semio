import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { tmpdir } from "node:os";
import { pathToFileURL } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { ESLint } from "eslint";
import postcss from "postcss";
import loadPostcssConfig from "postcss-load-config";
import { build, loadConfigFromFile } from "vite";

type Owner = Readonly<{
  id: string;
  tool: "vite" | "playwright" | "tailwind" | "postcss" | "eslint" | "dependency-cruiser" | "vscode-test";
  previousPath: string;
  ownerPath: string;
  effectiveRoot: string;
  loader: "vite-native" | "module" | "postcss-load-config" | "eslint" | "dependency-cruiser" | "vscode-test";
  mode: "serve" | "build" | "test" | "lint" | "verify";
}>;
type Fixture = Readonly<{
  schemaVersion: 1;
  owners: readonly Owner[];
  removedShims: readonly string[];
  fixedContractIds: readonly string[];
  consumers: readonly Readonly<{ path: string; tokens: readonly string[] }>[];
  projectInputs: readonly Readonly<{ path: string; owners: readonly string[] }>[];
  postcss: Readonly<{
    ownerId: string;
    packageManifestPath: string;
    packageEntryPath: string;
    packageExport: string;
    packageExportTarget: string;
    standaloneLoader: Readonly<{ kind: "explicit-search-place"; searchPlace: string }>;
    viteLoader: Readonly<{ kind: "inline-plugin-array"; exportName: string }>;
    historicalFixturePath: string;
    historicalOwnerPath: string;
    cases: readonly Readonly<{ id: string; source: string; expectedDeclarations: readonly Readonly<{ selector: string; property: string; value: string }>[] }>[];
  }>;
  registration: Readonly<{ name: string; command: string; target: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️tool-configuration-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🎚️tool-configuration-ownership/🔣️.json"), "utf8"));
const ownerById = new Map(fixture.owners.map((owner) => [owner.id, owner]));
const read = (path: string): string => readFileSync(resolve(repoRoot, path), "utf8");

async function importFresh(path: string): Promise<Record<string, any>> {
  return await import(`${pathToFileURL(resolve(repoRoot, path)).href}?ownership=${Date.now()}-${Math.random()}`);
}

describe("Tool configuration ownership", () => {
  test("validates the exact portable owner map", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(fixture.owners).toHaveLength(12);
    expect(new Set(fixture.owners.map(({ id }) => id)).size).toBe(12);
    expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(12);
    expect(fixture.removedShims).toHaveLength(2);
    expect(fixture.consumers).toHaveLength(22);
    expect(fixture.projectInputs).toHaveLength(8);
  });

  test("requires anonymous semantic owners and removes fixed predecessors and shims", () => {
    for (const owner of fixture.owners) {
      expect(owner.ownerPath.includes("/📦️packages/"), owner.ownerPath).toBe(false);
      expect(/\/(?:🟦️\.ts|🟨️\.(?:mjs|cjs))$/u.test(owner.ownerPath), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.ownerPath)), owner.ownerPath).toBe(true);
      expect(existsSync(resolve(repoRoot, owner.previousPath)), owner.previousPath).toBe(false);
    }
    for (const path of fixture.removedShims) expect(existsSync(resolve(repoRoot, path)), path).toBe(false);
  });

  test("binds every selector, import, source-data record and editor setting to the semantic owners", () => {
    for (const consumer of fixture.consumers) {
      const source = read(consumer.path);
      for (const token of consumer.tokens) expect(source, `${consumer.path}: ${token}`).toContain(token);
    }
    const settings = JSON.parse(read(".vscode/settings.json"));
    expect(settings["eslint.options"].overrideConfigFile).toBe(ownerById.get("root-eslint")!.ownerPath);
    expect(settings["tailwindCSS.experimental.configFile"]).toBe(ownerById.get("styling-tailwind")!.ownerPath);
  });

  test("binds each cacheable project graph to its exact external configuration owners", () => {
    for (const binding of fixture.projectInputs) {
      const project = JSON.parse(read(binding.path));
      const inputs = project.namedInputs?.default ?? [];
      for (const id of binding.owners) expect(inputs, `${binding.path}: ${id}`).toContain(`{workspaceRoot}/${ownerById.get(id)!.ownerPath}`);
    }
  });

  test("loads all five Vite owners with the installed native loader and preserves effective roots", async () => {
    const previous = { plugin: process.env.SEMIO_PLUGIN, renderer: process.env.SEMIO_RENDERER };
    process.env.SEMIO_PLUGIN = "s";
    process.env.SEMIO_RENDERER = "react";
    try {
      for (const owner of fixture.owners.filter(({ loader }) => loader === "vite-native")) {
        const command = owner.mode === "build" ? "build" : "serve";
        const loaded = await loadConfigFromFile({ command, mode: command === "build" ? "production" : "development", isSsrBuild: false, isPreview: false }, resolve(repoRoot, owner.ownerPath), undefined, "silent", undefined, "native");
        expect(loaded, owner.id).toBeDefined();
        expect(resolve(loaded!.config.root ?? repoRoot), owner.id).toBe(resolve(repoRoot, owner.effectiveRoot));
      }
    } finally {
      if (previous.plugin === undefined) delete process.env.SEMIO_PLUGIN; else process.env.SEMIO_PLUGIN = previous.plugin;
      if (previous.renderer === undefined) delete process.env.SEMIO_RENDERER; else process.env.SEMIO_RENDERER = previous.renderer;
    }
  }, 30_000);

  test("loads Playwright, Tailwind, ESLint, dependency-cruiser and VS Code configurations through installed tool contracts", async () => {
    const playwright = ownerById.get("demonstrator-playwright")!;
    const listed = spawnSync(process.execPath, [resolve(repoRoot, "node_modules/playwright/cli.js"), "test", "--config", resolve(repoRoot, playwright.ownerPath), "--list"], { cwd: repoRoot, encoding: "utf8", timeout: 30_000, env: { ...process.env, PLAYWRIGHT_BASE_URL: "http://127.0.0.1:6029" } });
    expect(listed.status, listed.stderr).toBe(0);
    expect(listed.stdout).toContain("Total: 7 tests in 1 file");

    const tailwind = ownerById.get("styling-tailwind")!;
    const tailwindModule = await importFresh(tailwind.ownerPath);
    const themeModule = await importFresh("🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts");
    expect(tailwindModule.default.darkMode).toBe("media");
    expect(tailwindModule.default.content).toEqual(["./**/*.{ts,tsx,mdx}"]);
    // 🌐️ The Tailwind config is build tooling and MUST stay behind its declared owner: `🌓️theme/🟦️.ts` is served to
    // the browser through `@semio-tech/ui-styling`, and re-exporting the config from there drags
    // `@tailwindcss/typography` → `@tailwindcss/node` → `@tailwindcss/oxide` (a `.node` binary) into Vite's browser
    // dependency optimizer. `🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts` ("browser entry module graph") guards the closure.
    expect(themeModule.default, "the browser theme barrel must not carry the build-time Tailwind config").toBeUndefined();
    expect(themeModule.tailwindConfig).toBeUndefined();

    const rootEslint = ownerById.get("root-eslint")!;
    const rootLint = new ESLint({ cwd: repoRoot, overrideConfigFile: resolve(repoRoot, rootEslint.ownerPath) });
    const rootConfig = await rootLint.calculateConfigForFile(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts"));
    expect(Object.keys(rootConfig?.rules ?? {}).length).toBeGreaterThan(50);
    const reactEslint = ownerById.get("ui-react-eslint")!;
    const reactRoot = resolve(repoRoot, reactEslint.effectiveRoot);
    const reactLint = new ESLint({ cwd: reactRoot, overrideConfigFile: resolve(repoRoot, reactEslint.ownerPath) });
    const reactConfig = await reactLint.calculateConfigForFile(resolve(reactRoot, "🟦️.tsx"));
    expect(reactConfig?.languageOptions.parser).toBeDefined();

    const dependencyOwner = ownerById.get("dependency-boundaries")!;
    const dependencyConfig = createRequire(import.meta.url)(resolve(repoRoot, dependencyOwner.ownerPath));
    expect(dependencyConfig.forbidden.length).toBeGreaterThan(20);
    expect(dependencyConfig.options).toBeDefined();

    const vscodeOwner = ownerById.get("vscode-extension-test")!;
    const vscodeLoader = await import(pathToFileURL(resolve(repoRoot, "node_modules/@vscode/test-cli/out/cli/config.mjs")).href);
    const vscode = await vscodeLoader.tryLoadConfigFile(resolve(repoRoot, vscodeOwner.ownerPath));
    expect(vscode.path).toBe(resolve(repoRoot, vscodeOwner.ownerPath));
    expect(vscode.tests[0].files).toBe(resolve(repoRoot, vscodeOwner.effectiveRoot, "out/test/**/*.test.js"));
    expect(vscode.tests[0].workspaceFolder).toBe(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode"));
  }, 60_000);

  test("loads the anonymous PostCSS package entry and Vite inline plugins through distinct installed contracts", async () => {
    const owner = ownerById.get(fixture.postcss.ownerId)!;
    const packageRoot = resolve(repoRoot, owner.effectiveRoot);
    const packageManifest = JSON.parse(read(fixture.postcss.packageManifestPath));
    expect(packageManifest.exports[fixture.postcss.packageExport]).toBe(fixture.postcss.packageExportTarget);
    expect(read(fixture.postcss.packageEntryPath).trim()).toBe('export { default } from "../../🛠️build-tooling/🎨️styling/🟦️.ts";');
    const historical = JSON.parse(read(fixture.postcss.historicalFixturePath));
    expect(historical.decisionState).toBe("non-authoritative-concurrent-source-byte-drift");
    expect(historical.mappings.some((row: readonly unknown[]) => row[10] === fixture.postcss.historicalOwnerPath)).toBe(true);
    expect(historical.schemaPrerequisites.exactFixedToolContracts).toContain("postcss-config");
    const loaded = await loadPostcssConfig({ cwd: packageRoot }, packageRoot, { searchPlaces: [fixture.postcss.standaloneLoader.searchPlace] });
    expect(loaded.file).toBe(resolve(repoRoot, fixture.postcss.packageEntryPath));
    for (const row of fixture.postcss.cases) {
      const result = await postcss(loaded.plugins).process(row.source, { from: undefined });
      for (const expected of row.expectedDeclarations) {
        let matched = false;
        result.root.walkRules(expected.selector, (rule) => rule.walkDecls(expected.property, (declaration) => { if (declaration.value === expected.value) matched = true; }));
        expect(matched, `${row.id}: ${expected.selector} ${expected.property}: ${expected.value}`).toBe(true);
      }
      expect(result.css).not.toContain("@apply");
    }

    const ownerModule = await importFresh(owner.ownerPath);
    expect(ownerModule.default).toEqual({ plugins: { "@tailwindcss/postcss": {} } });
    expect(typeof ownerModule[fixture.postcss.viteLoader.exportName]).toBe("function");
    const sandbox = mkdtempSync(resolve(process.env.SEMIO_TEST_OUTPUT_DIR ?? tmpdir(), "semio-postcss-vite-"));
    try {
      writeFileSync(resolve(sandbox, "🟨️.js"), 'import "./🎨️.css";\n');
      writeFileSync(resolve(sandbox, "🎨️.css"), fixture.postcss.cases.map(({ source }) => source).join("\n"));
      const compiled = await build({ configFile: false, root: sandbox, publicDir: false, logLevel: "silent", css: { postcss: { plugins: ownerModule[fixture.postcss.viteLoader.exportName]() } }, build: { write: false, cssCodeSplit: false, rollupOptions: { input: resolve(sandbox, "🟨️.js") } } });
      const outputs = (Array.isArray(compiled) ? compiled : [compiled]).flatMap((result) => result.output);
      const css = outputs.filter((entry) => entry.type === "asset" && entry.fileName.endsWith(".css")).map((entry) => String(entry.source)).join("\n");
      for (const row of fixture.postcss.cases) for (const expected of row.expectedDeclarations) expect(css, row.id).toContain(expected.value);
      expect(css).not.toContain("@apply");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }

    const expression = 'import config from "@semio-tech/ui-react/postcss.config"; process.stdout.write(JSON.stringify(config));';
    for (const executable of [process.execPath, "node"]) {
      const result = spawnSync(executable, ["--input-type=module", "--eval", expression], { cwd: repoRoot, encoding: "utf8", timeout: 30_000 });
      expect(result.status, `${executable}: ${result.stderr}`).toBe(0);
      expect(JSON.parse(result.stdout)).toEqual({ plugins: { "@tailwindcss/postcss": {} } });
    }
  }, 60_000);

  test("keeps frozen PostCSS identity evidence separate from the current configurable owner", () => {
    const history = JSON.parse(read(fixture.postcss.historicalFixturePath));
    expect(history.decisionState).toBe("non-authoritative-concurrent-source-byte-drift");
    expect(history.schemaPrerequisites.exactFixedToolContracts).toContain("postcss-config");
    expect(history.mappings.some((row: readonly unknown[]) => row[10] === fixture.postcss.historicalOwnerPath)).toBe(true);
    expect(ownerById.get(fixture.postcss.ownerId)!.ownerPath).not.toBe(fixture.postcss.historicalOwnerPath);
  });

  test("retires fixed-name exemptions and registers one Bun and Nx ownership route", () => {
    const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
    for (const id of fixture.fixedContractIds) {
      expect(taxonomy.fixedFilenameContracts[id], id).toBeUndefined();
      expect(taxonomy.packageSourceDispositions[id], id).toBeUndefined();
    }
    const librarySource = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts");
    expect(librarySource).not.toContain("resolveViteConfigFileName");
    expect(librarySource).toContain("config: string;");
    expect(taxonomy.semanticDirectoryMemberKinds["tool-configuration-ownership"].memberNames).toEqual(["🎚️tool-configuration-ownership"]);
    expect(taxonomy.semanticDirectoryMemberKinds["react-build-tooling-styling"].ownerKindIds).toEqual(["build-tooling"]);
    expect(taxonomy.semanticDirectoryMemberKinds["react-build-tooling-styling"].memberNames).toEqual(["🎨️styling"]);
    const script = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts");
    expect(script).toContain('segments[0] === "tool-configuration-ownership"');
    const project = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json"));
    expect(project.targets[fixture.registration.target].options.command).toBe("bun ./📜️script.ts test tool-configuration-ownership");
    expect(project.targets[fixture.registration.target].inputs).toContain("{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts");
    for (const path of [ownerById.get(fixture.postcss.ownerId)!.ownerPath, fixture.postcss.packageEntryPath, fixture.postcss.historicalFixturePath]) expect(project.targets[fixture.registration.target].inputs).toContain(`{workspaceRoot}/${path}`);
    const manifest = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json"));
    expect(manifest.scripts[fixture.registration.target]).toBe(`nx run @semio-tech/repo-lib:${fixture.registration.target}`);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) expect(read(path)).toContain(fixture.registration.command);
  });
});
