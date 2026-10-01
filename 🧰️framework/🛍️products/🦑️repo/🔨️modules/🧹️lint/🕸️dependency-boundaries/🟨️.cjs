/** @type {import('dependency-cruiser').IConfiguration} */
const fs = require("fs");
const { builtinModules } = require("module");
const path = require("path");
const REPO_ROOT = path.resolve(__dirname, "../../../../../..");

const TECHNOLOGIES = ["compose", "🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"];
const BOOTSTRAP_TOOLING_ENTRY_PATH = "(^|/)(?:📜️script\\.ts|🏗️builder/🌐️vite/🟦️\\.ts|🧪️tests/🎚️config/🟦️\\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\\.config\\.[cm]?[jt]s)$";
const RENDERER_HOST_ROOT = "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/";
const RESOLVED_NODE_BUILTIN_PATH = `^(?:${[...new Set(builtinModules.map((name) => name.replace(/^node:/, "")))].map(escapeRegex).join("|")})(?:$|/)`;
const RENDERER_HOST_ALLOWED_RESOLVED_PATHS = [
  RENDERER_HOST_ROOT,
  "^🧰️framework/🔨️modules/🖱️ui/",
  "^🧰️framework/📦️packages/",
  "^node_modules/react(?:-dom)?/",
  "^(?:node:|vitest/)",
  "^node_modules/(?:vite|vitest)/",
  RESOLVED_NODE_BUILTIN_PATH,
];

/** 🔌️ Derived from the live `✏️s/🔌️plugins` directory listing rather than hardcoded, so the
 * cross-plugin isolation matrix below self-corrects as plugins are added, renamed, or removed. */
const PLUGINS = presentDirectories("✏️s/🔌️plugins")
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name).sort();

/** 🔣️ `forbiddenPathSegments` (both `⚡️implementations` spellings) read from the M1 shared vocabulary
 * (`26/08/06/MECHANISM-VOCABULARY-AND-DISCOVERY-LIBRARY`) rather than re-hardcoded here, so this config
 * and the registry/root-policy scripts can never drift on which spellings are banned. Plain JSON require —
 * no TS toolchain needed from this plain `.cjs` config. */
const TAXONOMY = JSON.parse(
  fs.readFileSync(path.join(REPO_ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"),
);

/** 🗂️ Optional owner directories disappear cleanly; malformed present owners remain errors. */
function presentDirectories(owner) {
  try { return fs.readdirSync(path.join(REPO_ROOT, owner), { withFileTypes: true }); }
  catch (error) { if (error.code === "ENOENT") return []; throw error; }
}

/** 📁️ Deletion removes a workspace contribution without hiding a damaged present manifest. */
function presentWorkspace(owner) {
  let entry;
  try { entry = fs.lstatSync(path.join(REPO_ROOT, owner)); }
  catch (error) { if (error.code === "ENOENT") return false; throw error; }
  if (!entry.isDirectory()) throw new Error(`Workspace owner must be a directory: ${owner}`);
  return true;
}

/** ⚙️ Escapes a literal string for embedding inside a `RegExp` alternation. */
function escapeRegex(literal) {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 🏷️ Physical source ownership ends before installed package storage. */
function sourceOwnerPattern(owner) {
  return `^${escapeRegex(owner)}/(?!node_modules(?:/|$)|.*/node_modules(?:/|$))`;
}

/** 🏷️ Groups installed aliases that share the same direction while retaining exact package boundaries. */
function packageOwnerPatterns(names) {
  const packages = [...new Set(names)].sort();
  return packages.length ? [`(?:^(?:node_modules/)?|/node_modules/)(?:${packages.map(escapeRegex).join("|")})(?:$|/)`] : [];
}

/** 🗂️ Removes only redundant descendants already covered by an identical physical owner direction. */
function sourceOwnerPatterns(owners) {
  const roots = [...new Set(owners)];
  return roots.filter((owner) => !roots.some((root) => owner !== root && owner.startsWith(`${root}/`))).map(sourceOwnerPattern);
}

/** 🧪️ Fails config loading if the two resolver-boundary allowlists regress into broad source or vendor exclusions. */
function assertFocusedBoundarySemantics() {
  const bootstrap = new RegExp(BOOTSTRAP_TOOLING_ENTRY_PATH, "u");
  const approvedBootstrap = [
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts",
    "compose/client/lib/sketchpad/doc/js/vite.config.ts",
    "♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts",
    "♻️mit-bestand/🎤️präsentation/📦️packages/🟦️typescript/🧪️vitest.config.ts",
  ];
  const runtimeSources = [
    "compose/client/lib/sketchpad/js/boot.tsx",
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts",
    "🌎️hub/🔨️modules/example/🟦️.ts",
    "♻️mit-bestand/🧺️demonstrator/🤖️generated/🟦️plugins.ts",
    "♻️mit-bestand/🧺️demonstrator/runtime.config.ts",
  ];
  if (approvedBootstrap.some((candidate) => !bootstrap.test(candidate)) || runtimeSources.some((candidate) => bootstrap.test(candidate))) {
    throw new Error("dependency-cruiser bootstrap-tooling boundary is broader or narrower than its approved entry points");
  }
  const crossRules = crossTechnologyRules();
  if (crossRules.length !== TECHNOLOGIES.length * (TECHNOLOGIES.length - 1) || crossRules.some((rule) => rule.from.pathNot !== BOOTSTRAP_TOOLING_ENTRY_PATH)) {
    throw new Error("dependency-cruiser bootstrap-tooling boundary is not applied to every cross-technology direction");
  }

  const rendererRule = rendererHostsOnlyUiRule();
  if (rendererRule.from.path !== RENDERER_HOST_ROOT || rendererRule.to.pathNot !== RENDERER_HOST_ALLOWED_RESOLVED_PATHS) {
    throw new Error("dependency-cruiser renderer-host rule is not wired to its resolved-path allowlist");
  }
  const rendererAllows = rendererRule.to.pathNot.map((pattern) => new RegExp(pattern, "u"));
  const allowedRendererTargets = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/📦️packages/🟦️typescript/🟦️.ts",
    "node_modules/react/index.js",
    "node_modules/react-dom/client.js",
    "node_modules/vitest/dist/index.js",
    "vitest/importMeta",
    "fs",
    "node:path",
  ];
  const forbiddenRendererTargets = [
    "node_modules/three/build/three.module.js",
    "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts",
    "♻️mit-bestand/🧺️demonstrator/🪧️brand.ts",
  ];
  const allowed = (candidate) => rendererAllows.some((pattern) => pattern.test(candidate));
  if (allowedRendererTargets.some((candidate) => !allowed(candidate)) || forbiddenRendererTargets.some(allowed)) {
    throw new Error("dependency-cruiser renderer-host allowlist no longer distinguishes presentation dependencies from runtime ownership breaches");
  }
}

assertFocusedBoundarySemantics();

/** 🗺️ Classifies package ownership from the taxonomy and authored workspace manifests without reading opaque areas. */
const AREA_LAYERS = Object.entries(TAXONOMY.areaLayers);
const OPAQUE_PATHS = Object.values(TAXONOMY.pathExclusions).map(({ path: prefix }) => prefix.replace(/\/$/u, ""));
const { dependencyDirectionWorkspacePackages } = require("../../📚️library/🕸️dependencies/🧭️direction/🟦️.ts");
const WORKSPACES = dependencyDirectionWorkspacePackages(REPO_ROOT, OPAQUE_PATHS.map((prefix) => `^${escapeRegex(prefix)}(?:/|$)`)).map((pkg) => pkg.owner);
const WORKSPACE_PACKAGES = WORKSPACES
  .filter((dir) => !OPAQUE_PATHS.some((prefix) => dir === prefix || dir.startsWith(`${prefix}/`)))
  .filter(presentWorkspace)
  .map((dir) => { const manifest = JSON.parse(fs.readFileSync(path.join(REPO_ROOT, dir, "package.json"), "utf8")); if (typeof manifest.name !== "string" || !manifest.name) throw new Error(`Workspace owner requires an authored package name: ${dir}`); return { dir, name: manifest.name, dependencyRole: manifest.semio?.dependencyRole }; });
const S_PACKAGES = WORKSPACE_PACKAGES.filter(({ dir }) => dir.startsWith("✏️s/"));
const FRAMEWORK_PACKAGES = WORKSPACE_PACKAGES.filter(({ dir }) => dir.startsWith("🧰️framework/"));
const IGNORED_GRAPH_PATHS = TAXONOMY.implementationLeafPolicy.ignoredPathPatterns.filter((pattern) => pattern !== "**/node_modules").map((pattern) => `(^|/)${escapeRegex(pattern.replace(/^\*\*\//u, ""))}(/|$)`)
  .concat(OPAQUE_PATHS.map((prefix) => `^${escapeRegex(prefix)}(/|$)`));

/** 🧱️ Matches framework and implementation owners identically through resolved paths and public package names. */
function frameworkNoImplementationRule() {
  const frameworkAreas = AREA_LAYERS.filter(([, layer]) => layer === "framework").map(([area]) => area);
  const implementationAreas = AREA_LAYERS.filter(([, layer]) => layer === "implementation").map(([area]) => area);
  const targets = sourceOwnerPatterns(implementationAreas).concat(packageOwnerPatterns(WORKSPACE_PACKAGES.filter((pkg) => implementationAreas.some((area) => pkg.dir === area || pkg.dir.startsWith(`${area}/`))).map((pkg) => pkg.name)));
  return {
    name: "framework-no-implementation",
    severity: "error",
    comment: "Taxonomy framework areas must remain independent of implementation areas, including type imports, dynamic imports, tests, tooling and package subpaths",
    from: { path: frameworkAreas.map(sourceOwnerPattern) },
    to: { path: targets },
  };
}

/** 🧬️ Builds declared semantic role rules from taxonomy owners and workspace package contributions. */
function declaredDependencyDirectionRules() {
  const { roles, rules } = TAXONOMY.dependencyDirections;
  for (const pkg of WORKSPACE_PACKAGES) if (pkg.dependencyRole && !roles[pkg.dependencyRole]) throw new Error(`Unknown dependency role ${pkg.dependencyRole} in ${pkg.dir}`);
  const patterns = (roleIds) => roleIds.flatMap((id) => {
    const role = roles[id];
    if (!role) throw new Error(`Unknown dependency direction role ${id}`);
    const packages = WORKSPACE_PACKAGES.filter((pkg) => pkg.dependencyRole === id || role.ownerPaths.some((owner) => pkg.dir === owner || pkg.dir.startsWith(`${owner}/`)));
    return sourceOwnerPatterns(role.ownerPaths.concat(packages.map((pkg) => pkg.dir))).concat(packageOwnerPatterns(packages.map((pkg) => pkg.name).concat(role.externalPackages)));
  });
  return Object.entries(rules).map(([name, rule]) => ({ name, severity: "error", comment: "Semantic owners must follow their declared dependency direction through resolved paths, package aliases and subpaths", from: { path: patterns(rule.fromRoles) }, to: { path: patterns(rule.toRoles) } }));
}

/** 🥾️ Keeps product-runtime technology edges forbidden while exempting only executable bootstrap and Vite/Vitest configuration entry points. */
function crossTechnologyRules() {
  const rules = [];
  for (const from of TECHNOLOGIES) {
    for (const to of TECHNOLOGIES) {
      if (from === to) continue;
      rules.push({
        name: `no-cross-technology-${from}-to-${to}`,
        severity: "error",
        comment: "Runtime relative imports must not cross top-level technology folders; bootstrap scripts and approved Vite/Vitest config entry points may consume repo tooling",
        from: { path: `^${from}/`, pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH },
        to: {
          path: `^${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

/** 🔌️ Plugins must not relative-import a sibling plugin's implementation — cross-plugin sharing goes
 * through @semio-tech packages or a framework/s module. `flow` is exempt (media-graph canvas embed). */
function crossPluginRules() {
  const rules = [];
  for (const from of PLUGINS) {
    for (const to of PLUGINS) {
      if (from === to || to === "🌊️flow") continue;
      rules.push({
        name: `no-cross-plugin-${from}-to-${to}`,
        severity: "error",
        comment: "Relative imports must not cross plugin folders; use @semio-tech packages",
        from: { path: `^✏️s/🔌️plugins/${from}/` },
        to: {
          path: `^✏️s/🔌️plugins/${to}/`,
          dependencyTypes: ["local"],
        },
      });
    }
  }
  return rules;
}

/** 🌳️ Step 7 of the spicy-umbrella mechanism wave (`26/08/06/DEPENDENCY-CRUISER-CONFIG-MODERNIZATION-FOR-TAXONOMY-SHAPE`):
 * forbids any dependency whose RESOLVED path still carries a `⚡️implementations`/`⚡️implementation`
 * segment (Shape V2 tree purity, both spellings, from `🔣️taxonomy.json`'s `forbiddenPathSegments`).
 * WARN, not error — plugins are fully retrofitted but framework/hub/mit-bestand haven't been touched by
 * this initiative yet, so real hits are EXPECTED here; promotion to error is W10 finalization's job, not
 * this ticket's. */
function noImplSegmentRule() {
  const alternation = TAXONOMY.forbiddenPathSegments.map((segment) => segment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
  return {
    name: "no-impl-segment",
    severity: "warn",
    comment: "Shape V2 tree purity: no dependency path may carry a ⚡️implementations/⚡️implementation segment — WARN until the W10 finalization flip promotes this to error",
    from: {},
    to: { path: `(^|/)(${alternation})(/|$)` },
  };
}

/** 🚫️ Bans dependency paths whose emoji-stripped segment is a banned stem (`core`, `shared`, …). */
function noCorePathRule() {
  const stems = (TAXONOMY.bannedNameStems || ["core"]).map((segment) => segment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|");
  return {
    name: "no-core-path",
    severity: "error",
    comment: "Clean mechanism: no dependency may resolve through a banned name stem folder (core/shared/util/…) — ERROR after Wave 4 core dissolve",
    from: {
      path: "^(✏️s/|🧰️framework/|🌎️hub/|♻️mit-bestand/)",
    },
    to: {
      path: `(^|/)([^/]*?)(${stems})(/|$)`,
      pathNot: "node_modules|target|/pkg/",
    },
  };
}

/** 📦️ Step 7's "`$1`-capture rule": a relative (`local`) import may freely reach anywhere inside its OWN
 * package/module family — same directory tree, any depth — but must not resolve into a SIBLING family via
 * a deep relative path; cross-family reuse goes through a `@semio-tech/…` package-name import instead.
 * "Family" is approximated with a path-segment heuristic (chosen over wiring up M1's `discoverPackages()`
 * here: that library is an ESM/TS module meant for the registry/root-policy TS scripts, and importing it
 * into this plain `.cjs` config would need a build step for no real gain — the taxonomy's actual package
 * unit, `<owner>/📦️packages/<lang>/`, is Shape V2 end-state and most of these areas are still legacy
 * sandwiches today, so a `📦️packages`-anchored capture would simply fail to match almost anything yet;
 * the directory-family heuristic below already covers the real, present-day gap: `✏️s/🔌️plugins/*` cross
 * imports are already an ERROR via `crossPluginRules`, so plugins are deliberately left out here to avoid
 * a redundant WARN — the gap this rule actually closes is *within* 🧰️framework (product-to-product,
 * module-to-module), ✏️s/🔨️modules (s-module-to-s-module), 🌎️hub/🔨️modules, and ♻️mit-bestand
 * (item-to-item), none of which any existing rule reaches).
 * `📜️script.ts` itself is exempt (`pathNot` below): every such bootstrap script across the repo already
 * relative-imports repo-lib (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/…`) across family boundaries —
 * the SAME sanctioned pattern `no-escaping-relative-imports` above already carves out ("do not fix it into
 * a specifier-depth rule, that would break every script.ts in the repo") — flagging it here would just be
 * ~30 files of noise on an already-litigated non-issue, not a new real finding. */
function crossPackageRelativeRule() {
  const familyPattern = "🧰️framework/(?:🛍️products|🔨️modules)/[^/]+|✏️s/🔨️modules/[^/]+|🌎️hub/🔨️modules/[^/]+|♻️mit-bestand/[^/]+";
  return {
    name: "no-cross-package-relative",
    severity: "warn",
    comment:
      "Deep relative imports must not cross package/module family boundaries in favor of @semio-tech/… package-name imports — WARN until package-name imports are the norm repo-wide, then promote at finalization",
    from: { path: `^(${familyPattern})/`, pathNot: "(^|/)📜️script\\.ts$" },
    to: {
      dependencyTypes: ["local"],
      pathNot: "^$1/",
    },
  };
}

/** 🧱️ `framework-no-s` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `🧰️framework` must not
 * import `✏️s` app/plugin/module code — framework is the substrate `✏️s` builds on, never the reverse.
 * ERROR (W7 of the same ticket): a full `bunx dependency-cruiser … --output-type err` sweep across
 * `compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand` found zero real hits for this rule — the pre-existing
 * violations it was staged to wait out never materialized on this (TS/JS import graph) surface, so there
 * is nothing left to clear before promoting. */
function frameworkNoSRule() {
  return {
    name: "framework-no-s",
    severity: "error",
    comment: "🧰️framework must not import ✏️s app/plugin/module code — apps consume framework, never the reverse",
    from: { path: "^🧰️framework/" },
    to: {
      path: [sourceOwnerPattern("✏️s")].concat(S_PACKAGES.map((p) => `^${escapeRegex(p.name)}$`)),
    },
  };
}

/** 🧱️ `s-modules-no-plugins` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `✏️s/🔨️modules`
 * are shared building blocks and must not depend on `✏️s/🔌️plugins`, the app layer built from them — the
 * audit for this ticket found zero real violations of this direction, so it's safe to enforce as ERROR
 * immediately rather than staging it through `warn`. */
function sModulesNoPluginsRule() {
  const pluginPackageNames = packageOwnerPatterns(S_PACKAGES.filter((p) => p.dir.startsWith("✏️s/🔌️plugins/")).map((p) => p.name));
  return {
    name: "s-modules-no-plugins",
    severity: "error",
    comment: "✏️s/🔨️modules must not import ✏️s/🔌️plugins — modules are shared substrate for plugins, not the reverse",
    from: { path: "^✏️s/🔨️modules/" },
    to: {
      path: [sourceOwnerPattern("✏️s/🔌️plugins")].concat(pluginPackageNames),
    },
  };
}

/** 🧩️ Optional extensions and artifacts consume their plugin owner, never the reverse. */
function pluginNoExtensionOrArtifactRules() {
  const ownerPaths = ["🧩️extensions", "🗿️artifacts"];
  const packagePatterns = packageOwnerPatterns(S_PACKAGES.filter((pkg) => ownerPaths.some((owner) => pkg.dir.includes(`/${owner}/`))).map((pkg) => pkg.name));
  return PLUGINS.map((plugin) => ({
    name: `plugin-no-extension-or-artifact-${plugin}`,
    severity: "error",
    comment: "Plugin owners must remain independent of optional extensions and artifacts through every import form",
    from: { path: `^✏️s/🔌️plugins/${escapeRegex(plugin)}/`, pathNot: `^✏️s/🔌️plugins/${escapeRegex(plugin)}/(?:${ownerPaths.join("|")})/` },
    to: { path: ["^✏️s/(?!node_modules(?:/|$)|.*/node_modules(?:/|$))🔌️plugins/[^/]+/(?:🧩️extensions|🗿️artifacts)/"].concat(packagePatterns) },
  }));
}

/** 🔌️ `plugins-framework-sdk-only` (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`): TS/JS mirror of
 * `PluginCapabilityLintScript`'s Cargo-side `semio-framework-os` ban (framework-os-dev's `📜️script.ts`,
 * `depRules`) — a plugin may depend on the plugin SDK (`@semio-tech/framework`, the product-neutral
 * package rooted at `🧰️framework/📦️packages/`) but must not depend on any OTHER `🧰️framework` package: the
 * OS host (`@semio-tech/framework-os`), a renderer target, or anything else product-specific that isn't
 * the SDK. `from` excludes `📜️script.ts` for the exact reason `crossPackageRelativeRule` above already
 * does: every plugin's own build/dev script relative-imports repo-lib
 * (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/…`), a sanctioned dev-tooling pattern, not a runtime
 * coupling — leaving it in would have drowned the real findings below in 108 identical script.ts hits. WARN,
 * not error, for the same reason this ticket's other policy rules stay report-mode (see
 * `noImplSegmentRule`/`crossPackageRelativeRule` above): do not shift the shared verify gate under the two
 * other sessions concurrently editing this tree. A dry sweep at seed time DID find 7 real runtime hits after
 * the `📜️script.ts` exclusion (`bunx dependency-cruiser --config .dependency-cruiser.cjs --output-type
 * err-long ✏️s`, grep `plugins-framework-sdk-only` minus `📜️script\.ts →` lines) — `📐️cad`'s renderer/brepjs
 * components reaching `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f`,
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg` (a wasm build output), and three plugins'
 * `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript` direct-renderer imports — real,
 * pre-existing, left as WARN backlog for a future wave to triage rather than silenced or hastily excepted. */
function pluginsFrameworkSdkOnlyRule() {
  const otherFrameworkPackageNames = FRAMEWORK_PACKAGES.filter((p) => /^@semio-tech\/framework($|-)/.test(p.name) && p.name !== "@semio-tech/framework").map(
    (p) => `^${escapeRegex(p.name)}$`,
  );
  return {
    name: "plugins-framework-sdk-only",
    severity: "warn",
    comment: "✏️s/🔌️plugins/** may depend on the plugin SDK (@semio-tech/framework) but not any other 🧰️framework package — mirrors the Cargo capability lint's semio-framework-os ban",
    from: { path: "^✏️s/🔌️plugins/", pathNot: "(^|/)📜️script\\.ts$" },
    to: {
      path: ["^🧰️framework/"].concat(otherFrameworkPackageNames),
      pathNot: ["^🧰️framework/📦️packages/", "^@semio-tech/framework$"],
    },
  };
}

/** 🖥️ Enforces the renderer host's presentation-only dependency boundary against dependency-cruiser's canonical resolved paths. */
function rendererHostsOnlyUiRule() {
  return {
    name: "renderer-hosts-only-ui",
    severity: "error",
    comment: "the react renderer host may depend only on ui/styling, framework-core protocol types, react, and itself — never os-shell, ui-interpreter, or app packages",
    from: { path: RENDERER_HOST_ROOT },
    to: { pathNot: RENDERER_HOST_ALLOWED_RESOLVED_PATHS },
  };
}

module.exports = {
  forbidden: [
    {
      name: "no-circular",
      severity: "error",
      comment: "No circular dependencies",
      from: {},
      to: { circular: true },
    },
    {
      name: "not-to-unlisted",
      severity: "error",
      comment: "Only depend on packages declared in the nearest package.json",
      from: {},
      to: {
        dependencyTypes: ["npm-no-pkg", "npm-unknown"],
      },
    },
    {
      // 🧭️ Matches the RESOLVED dependency path, not the raw import specifier — dependency-cruiser is
      // invoked from the repo root, so this only fires when a resolved local import lands 4+ levels
      // above the repo root, i.e. actually escapes the checkout. A `📜️script.ts`'s own relative
      // specifier (`../../../../../../../🧰️framework/…`, 6-8 `../` segments to reach repo-lib) resolves
      // to an ordinary in-repo path and never trips this — do not "fix" it into a specifier-depth rule,
      // that would break every script.ts in the repo (see ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).
      name: "no-escaping-relative-imports",
      severity: "error",
      comment: "Relative imports must not resolve to a path outside the repo checkout",
      from: {},
      to: {
        dependencyTypes: ["local"],
        path: "^\\.\\./\\.\\./\\.\\./\\.\\./",
      },
    },
    {
      name: "framework-no-plugin-packages",
      severity: "error",
      comment: "🧰️framework must not import plugin app packages — shells derive from app contributions",
      from: { path: "^🧰️framework/" },
      to: {
        path: ["^✏️s/🔌️plugins/"].concat(S_PACKAGES.filter((pkg) => pkg.dir.startsWith("✏️s/🔌️plugins/")).map((pkg) => `(?:^(?:node_modules/)?|/node_modules/)${escapeRegex(pkg.name)}(?:$|/)`)),
      },
    },
    {
      name: "ui-no-framework-packages",
      severity: "error",
      comment: "🧰️framework/🔨️modules/🖱️ui must stay presentational and business-logic free — no OS/plugin/framework coupling",
      from: { path: "^🧰️framework/🔨️modules/🖱️ui/" },
      to: {
        path: ["^🧰️framework/", "^@semio-tech/framework-"],
        pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-"],
      },
    },
    rendererHostsOnlyUiRule(),
    {
      name: "no-generated-edits-upstream",
      severity: "error",
      comment: "only the plugin registry itself may import its generated plugin catalog directly — other consumers must go through generated/🟦️plugins.ts",
      from: { pathNot: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/" },
      to: { path: "^🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🔣️plugins\\.json$" },
    },
    {
      name: "no-state-outside-os",
      severity: "error",
      comment:
        "OS-exclusive state authority: ✏️s and non-OS framework must not import OS host DocumentStore/session internals via deep relative paths — go through @semio-tech packages / public host APIs",
      from: {
        path: "^(✏️s/|🧰️framework/)",
        pathNot: "^🧰️framework/🛍️products/💻️os/",
      },
      to: {
        path: "^🧰️framework/🛍️products/💻️os/.*/(🏪️store|🖥️host)/",
        dependencyTypes: ["local"],
      },
    },
    ...crossTechnologyRules(),
    ...crossPluginRules(),
    noImplSegmentRule(),
    noCorePathRule(),
    crossPackageRelativeRule(),
    frameworkNoSRule(),
    frameworkNoImplementationRule(),
    { ...frameworkNoImplementationRule(), name: "repo-no-implementation", comment: "Repository-wide source must remain independent of deletable implementation owners", from: { path: ["^[^/]+$"] } },
    ...declaredDependencyDirectionRules(),
    sModulesNoPluginsRule(),
    ...pluginNoExtensionOrArtifactRules(),
    pluginsFrameworkSdkOnlyRule(),
  ],
  options: {
    exclude: { path: IGNORED_GRAPH_PATHS },
    doNotFollow: {
      path: IGNORED_GRAPH_PATHS.concat("(^|/)node_modules(/|$)").join("|"),
    },
    tsPreCompilationDeps: true,
    combinedDependencies: true,
    enhancedResolveOptions: {
      exportsFields: ["exports"],
      conditionNames: ["semio-source", "import", "require", "node", "default"],
    },
  },
};
