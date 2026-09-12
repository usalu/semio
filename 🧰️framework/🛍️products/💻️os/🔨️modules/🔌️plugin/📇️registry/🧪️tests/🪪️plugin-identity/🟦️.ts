import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { parseModuleDirectories } from "../../📦️deployment/🟦️.ts";
import { parseComponentPackageId, type PluginRegistryEntry } from "../../🔎️discovery/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const registryRoot = resolve(import.meta.dirname, "../..");
const registry = JSON.parse(readFileSync(join(registryRoot, "🤖️generated/🔌️plugins.json"), "utf8")) as readonly PluginRegistryEntry[];
const catalog = parseModuleDirectories(JSON.parse(readFileSync(join(registryRoot, "📦️deployment/🗺️catalog.json"), "utf8")));

/** 🔌️ The one identity a plugin root passes to `Plugin::<…>::builder(…)`, as a quoted literal or a
 * `SCREAMING_SNAKE` const. Doc-comment decoys (`builder(…)`, `builder(...)`) can never match. */
const BUILDER_IDENTITY = /::builder\(\s*(?:"([a-z0-9-]+)"|([A-Z][A-Z0-9_]*))\s*\)/u;
/** 🧩️ The extension analogue — `ExtensionBundle::new(<identity>, <label>, <version>)`. */
const BUNDLE_IDENTITY = /ExtensionBundle::new\(\s*(?:"([a-z0-9-]+)"|([A-Z][A-Z0-9_]*))\s*,/u;
const PACKAGE_ID_CALL = /\.package_id\(\s*"([^"]+)"/u;

/** 🦀️ Resolves the ONE root source file that performs a crate's identity call. Every crate but
 * `demonstrator` declares it in `<pluginDir>/🦀️.rs`; the fallback takes the shallowest `🦀️.rs`
 * under the plugin directory that carries the call, which is how `🪪️manifest/🎪️demonstrator/🦀️.rs`
 * is found without a per-plugin literal here. */
function identityRootSource(cratePath: string, role: PluginRegistryEntry["role"]): { readonly path: string; readonly text: string } {
  const pluginDir = resolve(repoRoot, cratePath, "..", "..");
  const pattern = role === "extension" ? BUNDLE_IDENTITY : BUILDER_IDENTITY;
  const direct = join(pluginDir, "🦀️.rs");
  if (existsSync(direct)) {
    const text = readFileSync(direct, "utf8");
    if (pattern.test(text)) return { path: direct, text };
  }
  const candidates = readdirSync(pluginDir, { recursive: true, encoding: "utf8" })
    .filter((entry) => entry.endsWith("🦀️.rs"))
    .sort((a, b) => a.split("/").length - b.split("/").length || a.localeCompare(b));
  for (const candidate of candidates) {
    const path = join(pluginDir, candidate);
    const text = readFileSync(path, "utf8");
    if (pattern.test(text)) return { path, text };
  }
  throw new Error(`no identity call found for ${cratePath}`);
}

/** 🪪️ Reads the identity literal out of a root source, resolving a `const NAME: &str = "…";` when the
 * call is written against a constant instead of an inline string. */
function declaredIdentity(source: { readonly path: string; readonly text: string }, role: PluginRegistryEntry["role"]): string {
  const match = source.text.match(role === "extension" ? BUNDLE_IDENTITY : BUILDER_IDENTITY);
  if (!match) throw new Error(`no identity call found in ${source.path}`);
  if (match[1]) return match[1];
  const constant = source.text.match(new RegExp(String.raw`const ${match[2]}:\s*&str\s*=\s*"([^"]+)"`, "u"));
  if (!constant) throw new Error(`${source.path} passes ${match[2]} to its identity call but never defines it`);
  return constant[1];
}

describe("plugin identity is the same in every authority", () => {
  it("joins the Cargo component package, the root builder/bundle identity, the deployment row and the generated row for all 59 crates", () => {
    expect(registry).toHaveLength(59);
    expect(catalog).toHaveLength(registry.length);
    const byId = new Map(catalog.map((row) => [row.pluginId, row.directoryName]));
    for (const entry of registry) {
      const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
      const packageId = parseComponentPackageId(readFileSync(manifestPath, "utf8"), manifestPath);
      expect(packageId, entry.pluginId).toBe(`semio:${entry.pluginId}`);
      expect(entry.packageId, entry.pluginId).toBe(packageId);

      const source = identityRootSource(entry.cratePath, entry.role);
      expect(declaredIdentity(source, entry.role), `${entry.pluginId} declares its identity in ${source.path}`).toBe(entry.pluginId);

      const declaredPackageId = source.text.match(PACKAGE_ID_CALL)?.[1];
      if (declaredPackageId === undefined) {
        // 🗄️ `stdio` is the one crate that computes its package id at runtime from its own embedded
        // Cargo.toml, and no extension bundle declares one at all (`ExtensionBundle::package_id` is
        // unused repo-wide) — both are recorded here rather than silently skipped.
        expect(entry.role === "extension" || source.text.includes("component_package_id"), `${entry.pluginId} declares neither a package_id literal nor a runtime derivation`).toBe(true);
      } else {
        expect(declaredPackageId, entry.pluginId).toBe(packageId);
      }

      expect(byId.get(entry.pluginId), `deployment catalog has no row for ${entry.pluginId}`).toBeDefined();
    }
  });

  it("keeps every hand-authored plugin-identity fixture valid against its own schema and equal to the derived tuple", () => {
    const ajv = new Ajv({ strict: true });
    let checked = 0;
    for (const entry of registry) {
      const fixtureDir = resolve(repoRoot, entry.cratePath, "..", "..", "🧫️fixtures", "🧫️plugin-identity");
      if (!existsSync(join(fixtureDir, "🔣️.json"))) continue;
      const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8")) as Record<string, string>;
      const schema = JSON.parse(readFileSync(join(fixtureDir, "../../🧬️schema/🔣️.json"), "utf8"));
      const definitions = Object.entries(schema.$defs).filter(([, value]) => (value as { properties?: { schema?: { const?: string } } }).properties?.schema?.const === fixture.schema);
      expect(definitions, `${entry.pluginId} identity schema`).toHaveLength(1);
      const validate = ajv.addSchema(schema).getSchema(`${schema.$id}#/$defs/${definitions[0]![0]}`)!;
      expect(validate(fixture), `${entry.pluginId} fixture: ${ajv.errorsText(validate.errors)}`).toBe(true);
      expect(fixture.pluginId).toBe(entry.pluginId);
      expect(fixture.packageId).toBe(entry.packageId);
      expect(fixture.packageName).toBe(entry.packageName);
      expect(fixture.artifactKindPrefix).toBe(`s.${entry.pluginId}.`);
      expect(fixture.moduleDirectoryName).toBe(catalog.find((row) => row.pluginId === entry.pluginId)?.directoryName);
      const manifest = readFileSync(join(repoRoot, entry.cratePath, "Cargo.toml"), "utf8");
      expect(manifest, `${entry.pluginId} playground variant`).toContain(`variant = "${fixture.playgroundVariant}"`);
      checked += 1;
    }
    expect(checked, "at least the space and reasoning identity tuples are pinned").toBeGreaterThanOrEqual(2);
  });

  it("reads the real identity rather than a doc-comment decoy, and reports drift instead of passing vacuously", () => {
    const decoyed = { path: "synthetic", text: '/// `Plugin::<X>::builder(…)` — see the doc\npub fn plugin() { Plugin::<X>::builder("energy").package_id("semio:energy") }' };
    expect(declaredIdentity(decoyed, "plugin")).toBe("energy");
    const viaConst = { path: "synthetic", text: 'const EXTENSION_ID: &str = "imperative-extension-core";\nExtensionBundle::new(EXTENSION_ID, "Imperative Core", V)' };
    expect(declaredIdentity(viaConst, "extension")).toBe("imperative-extension-core");
    expect(declaredIdentity(viaConst, "extension")).not.toBe("imperative-extension-effect");
    expect(() => declaredIdentity({ path: "synthetic", text: "ExtensionBundle::new(MISSING_ID, \"x\", V)" }, "extension")).toThrow(/never defines it/u);
    expect(() => declaredIdentity({ path: "synthetic", text: "// nothing here" }, "plugin")).toThrow(/no identity call/u);
  });
});
