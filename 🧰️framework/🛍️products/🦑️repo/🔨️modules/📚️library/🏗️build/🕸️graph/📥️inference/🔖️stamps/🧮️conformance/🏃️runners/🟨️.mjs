import { createHash } from "node:crypto";
import { dirname } from "node:path";
import { commandWordsV1, crateNamesV1, deriveInputsV1, dispatchBranchesV1, entryFilesV1, excludesV1, globMatchesV1, inputGlobRegExpV1, includeRootsV1, innerCommandWordsV1, inputPrecisionLanguageV1, inputPrecisionTextV1, packageNameV1, parseJsoncV1, programInputsV1, readInputPrecisionV1, selectRuleV1 } from "../../../../../📥️inputs/🎯️precision/🟦️.ts";
import { packageManagerRegistryV1, selectPackageManagerV1 } from "../../../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

const SPEC = /(?:from\s*|import\s*\(\s*|import\s*)["'](\.{1,2}\/[^"']+)["']/gu;
const normalize = (path) => path.split("/").reduce((parts, part) => (part === ".." ? (parts.pop(), parts) : part === "." || part === "" ? parts : [...parts, part]), []).join("/");
const plain = (value) => JSON.parse(JSON.stringify(value, (_key, item) => (item instanceof Map ? [...item.entries()] : item instanceof Set ? [...item] : item)));
const attempt = async (body) => { try { return { value: plain(await body()) }; } catch (error) { return { error: String(error?.message ?? error) }; } };
const sha = (text) => (text === undefined ? null : createHash("sha256").update(text).digest("hex"));

/** 🧰️ The fixture workspace as an engine host: an in-memory tree, a regular-expression relative-import closure with explicit resolution probes and the declared Cargo graph. */
function fixtureHostV1(fixture, tree = fixture.workspace.files) {
  const closure = (entries) => {
    const seen = new Set(), pending = [...entries];
    while (pending.length) {
      const file = pending.pop();
      if (seen.has(file) || !(file in tree)) continue;
      seen.add(file);
      for (const match of (tree[file] ?? "").matchAll(SPEC)) {
        const base = normalize(`${dirname(file)}/${match[1]}`), hit = [base, `${base}.ts`, `${base}/🟦️.ts`].find((candidate) => candidate in tree);
        if (hit) pending.push(hit);
      }
    }
    return [...seen].sort();
  };
  const directories = new Set(Object.keys(tree).flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
  return {
    exists: (path) => path in tree || directories.has(path),
    readText: (path) => tree[path],
    listDirectory: (path) => {
      const prefix = path ? `${path}/` : "", names = new Map();
      for (const file of Object.keys(tree)) if (file.startsWith(prefix)) { const rest = file.slice(prefix.length), [name] = rest.split("/"); names.set(name, rest.includes("/")); }
      return [...names].map(([name, directory]) => ({ name, directory }));
    },
    kinds: fixture.workspace.kinds,
    importClosure: async (entries) => closure(entries),
    program: {
      compiler: "fixture-compiler-1",
      imports: async (file) => {
        const facts = [], resolved = [];
        for (const match of (tree[file] ?? "").matchAll(SPEC)) {
          const base = normalize(`${dirname(file)}/${match[1]}`);
          for (const candidate of [base, `${base}.ts`, `${base}/🟦️.ts`]) { facts.push({ kind: "file", path: candidate, value: candidate in tree }); if (candidate in tree) { resolved.push(candidate); break; } }
        }
        return { resolved: [...new Set(resolved)].sort(), facts };
      },
      fact: (kind, path) => (kind === "digest" ? sha(tree[path]) : kind === "file" ? path in tree : null),
      check: () => {},
      report: () => {},
    },
    cargoManifests: () => Object.keys(tree).filter((path) => path.endsWith("Cargo.toml")).map((path) => path.slice(0, -"Cargo.toml".length).replace(/\/$/u, "")),
    cargoRoots: (crate) => fixture.workspace.cargoRoots[crate] ?? [crate],
    cargoSources: (root, named) => (named ? undefined : fixture.workspace.cargoDependencySources[root]) ?? fixture.workspace.cargoSources[root],
    cargoToolchain: fixture.workspace.cargoToolchain,
  };
}

/** 🛤️ The host of the router fixture: its own tree, Cargo graph and import closure. */
function routerHostV1(corpus) {
  const tree = corpus.tree, base = fixtureHostV1({ workspace: { files: tree, kinds: {}, cargoRoots: corpus.cargo.roots, cargoDependencySources: {}, cargoSources: corpus.cargo.sources, cargoToolchain: corpus.cargo.toolchain } }, tree);
  return { ...base, cargoSources: (root) => corpus.cargo.sources[root] };
}

const readKindsOfV1 = (document, rule, into) => {
  for (const read of rule.reads) for (const key of Object.keys(read)) {
    if (key === "profile") for (const inner of document.profiles?.[read.profile]?.reads ?? []) for (const innerKey of Object.keys(inner)) into.add(innerKey);
    else into.add(key);
  }
  into.add("profile");
};

/** 🎯️ Evaluates the input precision corpus: the engine on the authored fixtures (globs and glob dialects, command words, JSONC, Cargo package names, dispatchers, every derivation, the program walks, the router, the manager roles, the texts) and the read kinds the derivations exercised. */
export async function runInputPrecisionV1({ main, dialects, router }) {
  const cases = {}, covered = new Set(), document = readInputPrecisionV1(main.document), host = fixtureHostV1(main);
  main.globs.forEach((row, index) => { cases[`glob/${index}`] = globMatchesV1(row.pattern, row.path); });
  dialects.rows.filter((row) => row.engines.includes("input")).forEach((row, index) => { const expression = inputGlobRegExpV1(row.pattern); cases[`dialect/${index}`] = [...row.matches, ...row.rejects].map((path) => expression.test(path)); });
  for (const [index, row] of dialects.errors.entries()) if (row.engine === "input") cases[`dialect-error/${index}`] = await attempt(() => inputGlobRegExpV1(row.pattern).source);
  main.commands.forEach((row, index) => { cases[`command/${index}`] = commandWordsV1(row.command) ?? null; });
  main.jsonc.forEach((row, index) => { cases[`jsonc/${index}`] = parseJsoncV1(row.text); });
  main.packageNames.forEach((row, index) => { cases[`package-name/${index}`] = packageNameV1(row.manifest) ?? null; });
  main.dispatch.forEach((row, index) => { cases[`dispatch/${index}`] = dispatchBranchesV1(row.source); });
  cases["crate-names"] = [...crateNamesV1(host.cargoManifests(), host.readText)];
  for (const [index, row] of main.derive.entries()) {
    const rule = selectRuleV1(document, row.identity);
    if (rule) readKindsOfV1(document, rule, covered);
    cases[`derive/${index}`] = rule === undefined ? null : rule.evidence === "assumed" ? { rule: rule.id, assumed: true } : await attempt(() => deriveInputsV1(document, rule, row.identity, host));
  }
  main.unselected.forEach((identity, index) => { cases[`unselected/${index}`] = selectRuleV1(document, identity)?.id ?? null; });
  const exporter = selectRuleV1(document, main.cargoExport.identity);
  cases["cargo-export"] = await attempt(() => deriveInputsV1(document, exporter, main.cargoExport.identity, host));
  cases["cargo-export-unwritten"] = await attempt(() => deriveInputsV1(document, exporter, { ...main.cargoExport.identity, outputs: [] }, host));
  cases["program"] = await attempt(() => programInputsV1(main.program.tsconfig, host));
  cases["program-entries"] = await attempt(() => programInputsV1(main.program.tsconfig, host, ["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"]));
  cases["entry-files"] = entryFilesV1(["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"], host);
  cases["include-roots"] = includeRootsV1(["app/src/**/*.ts"], ["app/src/skip/**"], host);
  cases["excludes"] = ["✏️s/x/node_modules/y", "a/dist/b", "src/skip/z", "x/y.ts", "keep/me"].map((path) => excludesV1(["✏️s/**/node_modules/**", "**/dist/**", "src/skip/**", "x/*.ts"])(path));
  for (const [index, row] of main.messages.entries()) cases[`language/${index}`] = await attempt(() => inputPrecisionLanguageV1(document, row.environment, () => []));
  const event = { phase: "progress", tsconfig: "app/tsconfig.json", files: 10, derived: 4, reused: 6, seconds: 1.5 };
  for (const language of document.messages.languages) for (const phase of ["derive", "progress", "done"]) cases[`text/${language}/${phase}`] = await attempt(() => inputPrecisionTextV1(document, { ...event, phase }, language));
  for (const [index, row] of main.refusals.entries()) {
    cases[`refusal/${index}`] = await attempt(async () => {
      if (row.kind === "document") {
        const mutated = JSON.parse(JSON.stringify(main.document));
        if (row.mutate === "wholeRepository") mutated.rules[2].reads.push({ globs: ["{workspaceRoot}/**/*"] });
        if (row.mutate === "wholeRepositoryDot") mutated.rules[2].reads.push({ globs: ["{workspaceRoot}/./**/*"] });
        if (row.mutate === "unknownProfile") mutated.rules[0].reads = [{ profile: "no-such-profile" }];
        if (row.mutate === "duplicateRule") mutated.rules[1].id = mutated.rules[0].id;
        if (row.mutate === "unknownKey") mutated.surprise = true;
        return readInputPrecisionV1(mutated).rules.length;
      }
      const narrow = { ...document, rules: [{ id: row.rule, evidence: "static", description: "refusal", select: { project: row.identity.project }, replaces: [], reads: row.reads }] };
      return deriveInputsV1(narrow, narrow.rules[0], row.identity, host);
    });
  }
  const managerRule = { id: "neutral-manager", description: "Selected manager configuration inputs", select: { project: "fixture", targets: ["check"] }, evidence: "static", replaces: [], reads: [{ manager: { ecosystem: "javascript", role: "scope" } }] };
  const managerDocument = readInputPrecisionV1({ ...main.document, profiles: {}, rules: [managerRule] }), managerHost = { ...host, managerFiles: (_ecosystem, role) => (role === "locks" ? ["bun.lock", "bun.lockb"] : ["bunfig.toml"]) };
  cases["manager-read"] = await attempt(() => deriveInputsV1(managerDocument, managerRule, { project: "fixture", target: "check", command: "repo check" }, managerHost));
  readKindsOfV1(managerDocument, managerRule, covered);
  const routerHost = routerHostV1(router), routerRule = { id: "router-fixture", description: "fixture", select: { project: "p" }, evidence: "static", replaces: ["default", "^default", "reads"], reads: [{ router: { source: "rt/📜️script.ts" } }] };
  const branchRule = { ...routerRule, id: "router-branches", reads: [{ router: { source: "rt2/📜️script.ts", classes: "rt2/routes/🟦️.ts" } }] };
  const routed = (using, command, word) => { const routedDocument = readInputPrecisionV1({ ...main.document, rules: [...main.document.rules, using] }); return deriveInputsV1(routedDocument, routedDocument.rules.at(-1), { project: "p", target: word, command }, routerHost); };
  readKindsOfV1(document, routerRule, covered);
  router.words.forEach((row, index) => { cases[`router-words/${index}`] = innerCommandWordsV1(row.command) ?? null; });
  for (const row of router.derived) cases[`router/${row.word}`] = await attempt(() => routed(routerRule, `repo ./📜️script.ts ${row.word}`, row.word));
  for (const row of router.refused) cases[`router-refused/${row.word}`] = await attempt(() => routed(routerRule, `repo ./📜️script.ts ${row.word}`, row.word));
  for (const [index, row] of router.branches.entries()) cases[`router-branch/${index}`] = await attempt(() => routed(branchRule, row.command, "t"));
  cases["router-branch/whole"] = await attempt(() => routed(branchRule, "repo ./📜️script.ts test other", "t"));
  return { cases, covered: { read: [...covered].sort() } };
}

/** 📦️ Evaluates the package manager corpus: the manager of every ecosystem the policy selects, every registered manager with the file names inference consumes, the environment override and its refusals. */
export async function runPackageManagersV1({ roles, policy }) {
  const cases = {}, selection = Object.keys(policy.selection).sort(), covered = { manager: new Set(), ecosystem: new Set() };
  for (const manager of packageManagerRegistryV1.all()) {
    covered.manager.add(manager.id);
    cases[`manager/${manager.id}`] = { ecosystem: manager.ecosystem, primary: manager.primaryLockName(), locks: manager.lockFileNames(), manifests: manager.manifestFileNames(), scope: manager.configurationNames("scope"), home: manager.configurationNames("home"), config: manager.configurationNames("config") };
  }
  for (const ecosystem of selection) {
    covered.ecosystem.add(ecosystem);
    cases[`select/${ecosystem}`] = await attempt(() => { const manager = selectPackageManagerV1(ecosystem, {}); return { id: manager.id, locks: manager.lockFileNames(), scope: manager.configurationNames("scope") }; });
  }
  for (const row of roles.cases) cases[`override/${row.manager}`] = await attempt(() => { const manager = selectPackageManagerV1("javascript", { SEMIO_PACKAGE_MANAGER_JAVASCRIPT: row.manager }); return { id: manager.id, locks: manager.lockFileNames(), scope: manager.configurationNames("scope") }; });
  cases["select/unknown"] = await attempt(() => selectPackageManagerV1("no-such-ecosystem", {}).id);
  cases["select/mismatch"] = await attempt(() => selectPackageManagerV1("javascript", { SEMIO_PACKAGE_MANAGER_JAVASCRIPT: "cargo" }).id);
  cases["ecosystems"] = [...packageManagerRegistryV1.ecosystems()];
  cases["lock-names"] = [...packageManagerRegistryV1.lockFileNames()];
  cases["manifest-names"] = [...packageManagerRegistryV1.manifestFileNames()];
  return { cases, covered: { manager: [...covered.manager].sort(), ecosystem: [...covered.ecosystem].sort() } };
}

/** 🗂️ The runners by the name the corpus manifest gives them. */
export const RUNNERS_V1 = Object.freeze({ "input-precision": runInputPrecisionV1, "package-managers": runPackageManagersV1 });
