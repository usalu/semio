import "../🔎️source-discovery/🟦️.ts";
import "../🧭️journeys/🧩️sources/🟦️.ts";
import "../../../📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests/🌿️runtime-selection/🟦️.ts";
import "../🎛️dispatch/🟦️.ts";
import "../🧊️execution/🚀️launch/🟦️.ts";
import "../🧊️execution/🛠️bootstrap/🟦️.ts";
import "../♻️reuse/🟦️.ts";
import { expect, test } from "bun:test";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv2020 from "ajv/dist/2020";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { JSDOM } from "jsdom";
import clamp from "lodash/clamp";
import glob from "fast-glob";
import stringWidth from "string-width";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
import { installedDashboard } from "../../../📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts";
import { spawnSync } from "node:child_process";

const root = resolve(import.meta.dir, "../../../../../../.."), dashboard = resolve(import.meta.dir, "../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const fixture = read(join(dashboard, "🧪️tests/🧫️fixtures/🧭️authority/🔣️.json"));
const authority = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")), oracle = new Ajv2020({ strict: false, allErrors: true });
const validate = oracle.compile(authority), neutralAlias = new RegExp(authority.properties.scripts.additionalProperties.pattern);
const accepts = (scripts: Record<string, unknown>) => Object.entries(fixture.scripts).every(([name, command]) => scripts[name] === command) && Object.entries(scripts).every(([name, command]) => Object.hasOwn(fixture.scripts, name) || typeof command === "string" && neutralAlias.test(command));
const allowedPath = (path: string) => {
  const normalized = path.replaceAll("\\", "/").toLowerCase();
  const parts = normalized.split("/");
  const file = parts.at(-1)!;
  return !parts.some(part => fixture.forbiddenDirectories.includes(part)) && !fixture.forbiddenExtensions.some((extension: string) => file.endsWith(extension)) && !fixture.forbiddenFiles.includes(file) && !fixture.forbiddenPaths.includes(parts.slice(-2).join("/")) && !parts.some((part, index) => part === ".cursor" && parts[index + 1] !== undefined && parts[index + 1] !== "plans");
};

const object = (value: unknown): value is Record<string, any> => typeof value === "object" && value !== null && !Array.isArray(value);
const ownerCommand = /^bun (?:--watch )?(?:"((?:[^"\r\n]*[/\\])?📜️script\.ts)"|((?:[^\s"\r\n]*[/\\])?📜️script\.ts)) [a-z][a-z0-9-]*(?: (?:[^\s"';&|<>\r\n]+|"[^"\r\n]*"|'[^'\r\n]*')+)*$/;
const acceptsOwnerCommand = (command: unknown) => typeof command === "string" && ownerCommand.test(command) && !/[$`]/.test(command);
const acceptsOwnerOptions = (options: unknown) => object(options) && (!Object.hasOwn(options, "command") || acceptsOwnerCommand(options.command)) && (!Object.hasOwn(options, "commands") || Array.isArray(options.commands) && options.commands.length > 0 && options.commands.every((command: unknown) => acceptsOwnerCommand(object(command) ? command.command : command)));
const acceptsOwnerTarget = (target: unknown) => object(target) && (!Object.hasOwn(target, "executor") || ["repo:script", "repo:noop"].includes(target.executor)) && (!Object.hasOwn(target, "command") || acceptsOwnerCommand(target.command)) && (!Object.hasOwn(target, "options") || acceptsOwnerOptions(target.options)) && (!Object.hasOwn(target, "configurations") || object(target.configurations) && Object.values(target.configurations).every(acceptsOwnerOptions)) && (target.executor === "repo:noop" || Object.hasOwn(target, "command") || object(target.options) && (Object.hasOwn(target.options, "command") || Object.hasOwn(target.options, "commands")));
type InventoryEntry = { path: string; directory: boolean };
const inventoryPolicy = oracle.getSchema(`${authority.$id}#/$defs/InventoryPolicy`)!;
const inventory = (workspace: string, policy: unknown, list = (path: string) => readdirSync(path, { withFileTypes: true })) => {
  let cached: InventoryEntry[] | undefined;
  return () => {
    if (cached) return cached;
    if (!inventoryPolicy(policy)) throw Error(`Invalid active inventory policy: ${oracle.errorsText(inventoryPolicy.errors)}`);
    const rules = policy as { boundaryDirectories: string[]; excludedRoots: string[] }, paths: InventoryEntry[] = [], pending = [workspace];
    const excluded = new Set(rules.excludedRoots), boundaries = new Set(rules.boundaryDirectories);
    let visited = 0;
    while (pending.length) {
      const directory = pending.pop()!;
      if (++visited % 10000 === 0) console.log(`[DEBUG] authority gate inspected ${visited} source directories`);
      for (const entry of list(directory)) {
        const absolute = join(directory, entry.name), path = relative(workspace, absolute).replaceAll("\\", "/");
        if (entry.isDirectory() && (boundaries.has(entry.name) || excluded.has(path))) continue;
        paths.push({ path, directory: entry.isDirectory() });
        if (entry.isDirectory() && !entry.isSymbolicLink()) pending.push(absolute);
      }
    }
    cached = paths;
    return cached;
  };
};
const activeRepositoryPaths = inventory(root, fixture.inventoryPolicy);
const acceptsInstructions = (text: string) => !/(?:\.(?:vscode(?:-insiders)?|idea|fleet|zed|vs|run|nova|settings|metadata|windsurf|kiro|atom|brackets)[/\\]|\.cursor[/\\](?!plans(?:[/\\]|$))|\.(?:cursorrules|windsurfrules)\b|\.sublime-(?:project|workspace|build)\b|cmakesettings\.json|launch\.json|🧩️launch\.seed\.jsonc|ide startup|reopen in vs code|all required vs code extensions|dev containers: reopen in container|vs code.*with.*dev containers extension)/.test(text.toLowerCase());

test("active inventory fails closed and only excludes declared generated roots and dependency boundaries", () => {
  expect(inventoryPolicy(fixture.inventoryPolicy)).toBe(true);
  for (const policy of [{}, { excludedRoots: [] }, { ...fixture.inventoryPolicy, excludedDirectories: [] }, { ...fixture.inventoryPolicy, boundaryDirectories: [...fixture.inventoryPolicy.boundaryDirectories, "build"] }, { ...fixture.inventoryPolicy, excludedRoots: [...fixture.inventoryPolicy.excludedRoots, "domain/dist"] }]) {
    expect(inventoryPolicy(policy)).toBe(false);
    const scan = inventory(root, policy);
    expect(scan).toThrow("Invalid active inventory policy");
    expect(scan).toThrow("Invalid active inventory policy");
  }
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!parent) throw Error("Caller-owned authority inventory artifacts required");
  mkdirSync(parent, { recursive: true });
  const workspace = mkdtempSync(join(parent, "authority-"));
  try {
    for (const row of fixture.inventoryCases) {
      const path = join(workspace, row.path);
      if (path.length > 256) throw Error("Portable authority fixture path capacity exceeded");
      mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, "{}");
    }
    let fail = true;
    const scan = inventory(workspace, fixture.inventoryPolicy, path => {
      if (path === join(workspace, "domain") && fail) { fail = false; throw Error("[DEBUG] fixture traversal denied"); }
      return readdirSync(path, { withFileTypes: true });
    });
    expect(scan).toThrow("fixture traversal denied");
    const paths = scan();
    expect(paths.length).toBeGreaterThan(0);
    expect(scan()).toBe(paths);
    const independent = glob.sync("**/*", { cwd: workspace, dot: true, onlyFiles: true, followSymbolicLinks: false, ignore: [...fixture.inventoryPolicy.boundaryDirectories.map((name: string) => `**/${name}/**`), ...fixture.inventoryPolicy.excludedRoots.map((path: string) => `${path}/**`)] });
    expect(paths.filter(entry => !entry.directory).map(entry => entry.path).sort()).toEqual(independent.sort());
    for (const row of fixture.inventoryCases) expect(paths.some(entry => entry.path === row.path), row.path).toBe(row.inspected);
    expect(paths.filter(entry => !allowedPath(entry.path)).length).toBeGreaterThan(0);
    console.log(`[DEBUG] authority inventory recovered after failed traversal and checked ${fixture.inventoryCases.length} path boundaries twice`);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
});

test("root agent instructions name the dashboard as the human surface and need no editor exemption", () => {
  const validateInstruction = oracle.getSchema(`${authority.$id}#/$defs/DashboardInstruction`)!;
  for (const row of fixture.dashboardInstructionCases) {
    const modeled = row.path === fixture.dashboardInstruction.path && row.text === fixture.dashboardInstruction.text;
    expect(modeled, row.path).toBe(row.accepted);
    expect(validateInstruction({ path: row.path, text: row.text }), row.path).toBe(row.accepted);
  }
  expect(validateInstruction(fixture.dashboardInstruction)).toBe(true);
  const text = readFileSync(join(root, fixture.dashboardInstruction.path), "utf8").replaceAll("\r\n", "\n");
  expect(text.split(fixture.dashboardInstruction.text).length).toBe(2);
  expect(acceptsInstructions(text), fixture.dashboardInstruction.path).toBe(true);
});

test("every owner command and target agrees with the independent schema validator", () => {
  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const validateCommand = oracle.getSchema(`${schema.$id}#/$defs/OwnerCommand`)!, validateTarget = oracle.getSchema(`${schema.$id}#/$defs/OwnerTarget`)!;
  for (const row of fixture.ownerCommandCases) {
    expect(acceptsOwnerCommand(row.command), row.command).toBe(row.accepted);
    expect(validateCommand(row.command), row.command).toBe(row.accepted);
  }
  for (const row of fixture.ownerTargetCases) {
    expect(acceptsOwnerTarget(row.target), JSON.stringify(row.target)).toBe(row.accepted);
    expect(validateTarget(row.target), JSON.stringify(row.target)).toBe(row.accepted);
  }
  console.log(`[DEBUG] authority checked ${fixture.ownerCommandCases.length} command vectors and ${fixture.ownerTargetCases.length} owner target vectors`);
});

test("runtime devcontainer lifecycle uses build owners and provisions no IDE customization", () => {
  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
  const validateRuntime = new Ajv2020({ strict: false, allErrors: true }).compile(schema.$defs.DevcontainerRuntime);
  const acceptsRuntime = (value: Record<string, any>) => ["postCreateCommand", "postStartCommand", "postAttachCommand"].every(key => JSON.stringify(value[key]) === JSON.stringify(fixture.devcontainerRuntime[key])) && !["customizations", "settings", "extensions"].some(key => Object.hasOwn(value, key)) && (!Object.hasOwn(value, "mounts") || Array.isArray(value.mounts) && value.mounts.every((mount: unknown) => typeof mount === "string" && !/target=[^,]*\/\.(?:codeium|kiro|vscode|vscode-insiders|cursor|windsurf|idea|fleet|zed|atom|brackets)(?:\/|,|$)/.test(mount)));
  for (const row of fixture.devcontainerCases) {
    const value = { ...fixture.devcontainerRuntime, ...row.add };
    expect(acceptsRuntime(value), row.name).toBe(row.accepted);
    expect(validateRuntime(value), row.name).toBe(row.accepted);
  }
  const runtime = parseJsonc(readFileSync(join(root, ".devcontainer/devcontainer.json"), "utf8"));
  expect(acceptsRuntime(runtime)).toBe(true);
  expect(validateRuntime(runtime), JSON.stringify(validateRuntime.errors)).toBe(true);
  const bootstrap = readFileSync(join(root, "📜️script.ts"), "utf8"), lifecycle = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🟦️.ts"), "utf8");
  expect(bootstrap).not.toContain(".codeium");
  expect(lifecycle).not.toContain('".codeium"');
  const owner = read(join(root, "📋️project.json"));
  for (const phase of ["start", "attach"]) expect(owner.targets[`setup-devcontainer-${phase}`]?.options.command).toBe(`bun ./📜️script.ts setup devcontainer ${phase}`);
  console.log(`[DEBUG] authority checked ${fixture.devcontainerCases.length} runtime container vectors and all three build lifecycle owners`);
});
test("exclusive dashboard paths agree with the independent schema validator", () => {
  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
  const validatePath = new Ajv2020({ strict: false }).compile(schema.$defs.ActiveRepositoryPath);
  for (const row of fixture.pathCases) {
    expect(allowedPath(row.path), row.path).toBe(row.accepted);
    expect(validatePath(row.path.toLowerCase()), row.path).toBe(row.accepted);
    expect(allowedPath(row.path.toUpperCase().replaceAll("/", "\\")), row.path).toBe(row.accepted);
  }
});

test("developer instructions agree with the independent schema validator and retire IDE setup", () => {
  const schema = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json"));
  const validateInstructions = new Ajv2020({ strict: false }).compile(schema.$defs.DeveloperInstructions);
  for (const row of fixture.instructionCases) {
    expect(acceptsInstructions(row.text), row.text).toBe(row.accepted);
    expect(validateInstructions(row.text.toLowerCase()), row.text).toBe(row.accepted);
  }
  const documents = new Set<string>(fixture.developerDocuments);
  for (const { path, directory } of activeRepositoryPaths()) if (!directory && /(?:^|\/)(?:README|CONTRIBUTING|DEVELOPMENT|AGENTS)\.md$/i.test(path) && !/(?:🧫️fixtures|🧪️tests|🧬️schema|\.cursor\/plans)(?:\/|$)/.test(path)) documents.add(path);
  for (const path of documents) {
    const text = readFileSync(join(root, path), "utf8").replaceAll("\r\n", "\n");
    expect(acceptsInstructions(text), path).toBe(true);
    expect(validateInstructions(text.toLowerCase()), path).toBe(true);
  }
  console.log(`[DEBUG] authority checked ${documents.size} developer documents and ${fixture.instructionCases.length} shared instruction vectors`);
}, 300_000);

test("localized command metadata follows shared language-neutral vectors and independent schema validation", () => {
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const vectors = read(join(dashboard, "🧫️fixtures/🌐️metadata/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const validText = (value: unknown) => typeof value === "object" && value !== null && !Array.isArray(value) && Object.keys(value).sort().join(",") === "de,en" && Object.values(value).every(text => typeof text === "string" && text.trim().length > 0 && [...text].length <= 1024 && !/[\u0000-\u001f\u007f]/.test(text));
  for (const row of vectors.cases) {
    expect(Object.values(row.metadata).every(validText), row.name).toBe(row.accepted);
    for (const [definition, base] of [
      ["Tool", { id: "source-open", command: ["bun", "--version"] }],
      ["TargetDeclaration", {}],
      ["Parameter", { id: "path", kind: "text" }],
      ["Compound", { id: "source-open", members: ["workspace:test"] }],
      ["Group", { id: "source-open", targets: ["test"], projects: ["workspace"] }],
      ["ParameterInfo", { id: "path", kind: "text", values: [], required: true, origin: "tool" }],
      ["RegistryEntry", { id: "tool:workspace/source-open", kind: "tool", verb: "task", label: "source-open", owner: ["workspace"], longRunning: false, listed: true, mutating: false, parameters: [], source: "📋️project.json" }]
    ] as const) {
      const validate = oracle.getSchema(`${schema.$id}#/$defs/${definition}`)!;
      expect(validate({ ...base, ...row.metadata }), `${row.name}: ${definition}`).toBe(row.accepted);
    }
  }
  console.log(`[DEBUG] authority checked ${vectors.cases.length} localized metadata vectors across seven declarations and outputs`);
});

test("launcher input batches and viewport bounds match independent DOM and clamp oracles", () => {
  const vectors = read(join(dashboard, "🧫️fixtures/⌨️batch/🔣️.json"));
  const schema = read(join(dashboard, "🧬️schema/⌨️batch/🔣️.json"));
  const validate = new Ajv2020({ strict: false }).compile(schema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  const fields = ["parameter", "args", "env"];
  for (const row of vectors.cases) {
    let focus = -1; const model = ["", "", ""];
    const dom = new JSDOM("<form><input id='parameter'><input id='args'><input id='env'></form>");
    const inputs = fields.map(field => dom.window.document.getElementById(field) as HTMLInputElement);
    for (const event of row.events) {
      if (event === "enter") { focus = 0; inputs[0]!.focus(); }
      else if (event === "down") { focus = Math.min(2, focus + 1); inputs[focus]!.focus(); }
      else {
        const value = event.slice(event.indexOf(":") + 1);
        model[focus] += value;
        const input = dom.window.document.activeElement as HTMLInputElement;
        input.setSelectionRange(input.value.length, input.value.length);
        input.setRangeText(value, input.selectionStart!, input.selectionEnd!, "end");
      }
    }
    expect(fields[focus], row.name).toBe(row.field);
    expect(model[focus], row.name).toBe(row.expected);
    expect(inputs[fields.indexOf(row.field)]!.value, row.name).toBe(row.expected);
    dom.window.close();
  }
  for (const row of vectors.defaultTextCases) {
    const dom = new JSDOM("<input>"); const input = dom.window.document.querySelector("input")!;
    input.defaultValue = row.default ?? "";
    expect(row.default ?? "").toBe(row.expected);
    expect(input.value).toBe(row.expected);
    dom.window.close();
  }
  for (const row of vectors.viewportCases) {
    const oldTop = Math.max(0, 3 - row.initialPage + 1), maximum = Math.max(0, 12 - row.page);
    expect(Math.max(0, Math.min(oldTop, maximum))).toBe(row.expectedTop);
    expect(clamp(oldTop, 0, maximum)).toBe(row.expectedTop);
  }
  const metadata = read(join(dashboard, "🧫️fixtures/🌐️metadata/🔣️.json")).workspace.projects[0].dashboard.tools[0];
  for (const row of vectors.pointerCases) {
    const dom = new JSDOM("<form><h1></h1><p></p><label for='parameter'></label><input id='parameter'><label for='args'>Arguments</label><input id='args'></form>");
    const document = dom.window.document, input = document.querySelector<HTMLInputElement>("#parameter")!, args = document.querySelector<HTMLInputElement>("#args")!, label = document.querySelector("label")!;
    document.querySelector("h1")!.textContent = metadata.title[row.locale];
    document.querySelector("p")!.textContent = metadata.description[row.locale];
    input.placeholder = metadata.parameters[0].title[row.locale]; label.textContent = input.placeholder;
    input.value = row.initialParameter; args.value = row.initialArgs; args.focus();
    label.control!.focus(); input.setSelectionRange(input.value.length, input.value.length); input.setRangeText(row.typed, input.selectionStart!, input.selectionEnd!, "end");
    expect(document.activeElement).toBe(input);
    expect(input.placeholder).toBe(row.expectedPlaceholder);
    expect(row.initialParameter + row.typed).toBe(row.expectedText);
    expect(input.value).toBe(row.expectedText);
    expect(args.value).toBe(row.initialArgs);
    dom.window.close();
  }
  console.log(`[DEBUG] launcher batch oracle checked ${vectors.cases.length} input sequences, ${vectors.defaultTextCases.length} defaults, ${vectors.viewportCases.length} growing viewports and ${vectors.pointerCases.length} described pointer fields`);
});

test("typed repo actions agree with shared vectors and independent schema validation", () => {
  const vectors = read(join(dashboard, "🧫️fixtures/🏛️repo-action/🔣️.json"));
  const validateAction = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(dashboard, "🧬️schema/🏛️repo-action/🔣️.json")));
  const registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json")), tree = read(join(dashboard, "🧬️schema/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(registry); oracle.addSchema(tree);
  const text = (value: unknown) => typeof value === "string" && value.trim().length > 0 && [...value].length <= 1024 && !/[\u0000-\u001f\u007f]/.test(value);
  const ticket = (value: unknown) => typeof value === "string" && /^\d{2}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*(?:\/[A-Z0-9]+(?:-[A-Z0-9]+)*)*$/.test(value);
  const object = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
  const keys = (value: Record<string, unknown>, expected: string[]) => Object.keys(value).sort().join(",") === expected.sort().join(",");
  const accepts = (action: unknown) => {
    if (typeof action === "string") return ["GoalsList", "GoalsTree", "TreeMonorepo", "TreeGoal", "TreeStatute", "TreeTerritory", "StatutesCatalog"].includes(action);
    if (!object(action) || Object.keys(action).length !== 1) return false;
    const [kind, value] = Object.entries(action)[0]!; if (!object(value)) return false;
    if (["TicketShow", "TicketFiles", "TicketClose"].includes(kind)) return keys(value, ["id"]) && ticket(value.id);
    if (kind === "TicketReopen") return keys(value, ["id", "prompt"]) && ticket(value.id) && text(value.prompt);
    if (kind === "DraftStart") return keys(value, ["title"]) && text(value.title);
    if (kind === "DraftDiscard") return keys(value, ["slug"]) && typeof value.slug === "string" && value.slug.length <= 1024 && /^[A-Z0-9]+(?:-[A-Z0-9]+)*$/.test(value.slug);
    return kind === "Analyze" && keys(value, ["scope", "policy"]) && ["all", "rust", "typescript", "go", "python", "markdown"].includes(value.scope as string) && (value.policy === null || value.policy === "code");
  };
  for (const row of vectors.valid) {
    expect(accepts(row.action), row.key).toBe(true);
    expect(validateAction(row.action), `${row.key}: ${JSON.stringify(validateAction.errors)}`).toBe(true);
    expect(oracle.getSchema(`${registry.$id}#/$defs/CommandId`)!(`repo:${row.key}`), row.key).toBe(true);
    expect(oracle.getSchema(`${registry.$id}#/$defs/Reference`)!(`repo:${row.key}`), row.key).toBe(false);
    expect(oracle.getSchema(`${tree.$id}#/$defs/RepoLeaf`)!({ kind: "repo", action: row.key, goArgv: row.argv }), row.key).toBe(true);
  }
  for (const action of vectors.invalid) {
    expect(accepts(action), JSON.stringify(action)).toBe(false);
    expect(validateAction(action), JSON.stringify(action)).toBe(false);
  }
  expect(oracle.getSchema(`${registry.$id}#/$defs/ParameterInfo`)!({ id: "title", kind: "text", values: [], required: true, origin: "repo" })).toBe(true);
  console.log(`[DEBUG] repo action schema oracle checked ${vectors.valid.length} accepted and ${vectors.invalid.length} refused shared actions`);
});

test("native console width profiles match the shared schema and independent cursor model", () => {
  const ui = join(root, "🧰️framework/🔨️modules/🖱️ui/⌨️tui");
  const vectors = read(join(ui, "🧫️fixtures/📏️text-width/🪟️console/🔣️.json"));
  const validate = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(ui, "🧬️schema/📏️text-width/🪟️console/🔣️.json")));
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  const dependency = createRequire(createRequire(import.meta.url).resolve("string-width")), { eastAsianWidth } = dependency("get-east-asian-width");
  const windowsCells = (text: string) => [...text].reduce((cells, char) => cells + (char.codePointAt(0)! > 0xffff ? 2 : eastAsianWidth(char.codePointAt(0)!)), 0);
  const sampleIds = new Set<string>();
  for (const row of vectors.cases) {
    expect(sampleIds.has(row.id), row.id).toBe(false); sampleIds.add(row.id);
    expect(row.text.replace(/[\ufe0f\u200d]/gu, ""), row.id).toBe(row.scalarText);
    expect(windowsCells(row.scalarText), `${row.id}: scalar console cells`).toBe(row.windowsScalarCells);
    expect(windowsCells(row.text), `${row.id}: unprojected console cells`).toBe(row.windowsClusterCells);
    expect(stringWidth(row.text), `${row.id}: independent string-width cells`).toBe(row.stringWidthCells);
  }
  expect(vectors.cases.find((row: { id: string }) => row.id === "canonical-argument")).toMatchObject({ scalarCells: 107, clusterCells: 110, windowsScalarCells: 110, windowsClusterCells: 119 });
  console.log(`[DEBUG] independent string-width and East-Asian-width/UTF16 oracles checked ${vectors.cases.length} measured native cursor vectors; formal widths retain their native unicode-width 0.2.2 oracle`);
});

test("the Go repository implementation has a dashboard-owned source build", () => {
  const cli = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli");
  const vector = read(join(cli, "🧫️fixtures/🏗️build/🔣️.json"));
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const validate = oracle.compile(read(join(cli, "🧬️schema/🏗️build/🔣️.json")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const owner = read(join(cli, "📦️packages/🐹️go/📋️project.json"));
  expect(owner.name).toBe(vector.project);
  expect(owner.targets.build?.options.command).toBe(vector.command);
  expect(owner.targets.build?.metadata.semio.dashboard).toEqual(vector.declaration);
  expect(owner.targets.build?.outputs).toEqual(Object.values(vector.artifacts));
  const entry = readFileSync(join(cli, "📦️packages/🐹️go/📜️script.ts"), "utf8");
  expect(entry).toContain(`resolve(import.meta.dir, "${vector.module}")`);
  expect(entry).toContain(`"-o", binary, "${vector.entryPoint}"`);
  expect(oracle.getSchema(`${schema.$id}#/$defs/TargetDeclaration`)!(owner.targets.build?.metadata.semio.dashboard)).toBe(true);
  console.log("[DEBUG] canonical Go repository build target agrees with its shared build/registry schemas");
});

test("the bootstrap decision matches the independent schema validator for every shared vector", () => {
  const aliases: string[] = [];
  for (const row of fixture.cases) {
    const scripts = { ...fixture.scripts, ...row.add };
    for (const name of row.remove) delete scripts[name];
    expect(accepts(scripts), row.name).toBe(row.accepted);
    expect(validate({ scripts }), row.name).toBe(row.accepted);
    if (row.accepted) for (const [name, command] of Object.entries(scripts)) if (!Object.hasOwn(fixture.scripts, name)) {
      aliases.push(command as string);
    }
  }
  const parsed = spawnSync(testPythonV1() ?? Bun.which("python") ?? "python3", ["-c", "import json,shlex,sys; print(json.dumps([shlex.split(value) for value in json.load(sys.stdin)]))"], { input: JSON.stringify(aliases), encoding: "utf8", timeout: 5000, maxBuffer: 65536 });
  expect(parsed.status, parsed.stderr).toBe(0);
  expect(parsed.signal).toBeNull();
  expect(JSON.parse(parsed.stdout)).toEqual(aliases.map(command => command.split(" ")));
});

test("the root manifest keeps canonical bootstraps and neutral task aliases", () => {
  const manifest = read(join(root, "package.json"));
  expect(validate(manifest), JSON.stringify(validate.errors?.slice(0, 8))).toBe(true);
  expect(accepts(manifest.scripts)).toBe(true);
});

test("active editor launch projections are absent and owners retain dashboard command declarations", () => {
  for (const path of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) expect(existsSync(join(root, path)), path).toBe(false);
  const manifest = read(join(root, "📋️project.json"));
  const registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(registry);
  const validateManifest = oracle.getSchema(`${registry.$id}#/$defs/ProjectManifest`)!;
  expect(validateManifest(manifest), oracle.errorsText(validateManifest.errors)).toBe(true);
  for (const name of ["generate-launch", "generate-launch-check"]) expect(manifest.targets[name], name).toBeUndefined();
});

test("root runnable commands remain owned by build targets and dashboard declarations", () => {
  const manifest = read(join(root, "📋️project.json"));
  expect(Object.keys(manifest.targets).length).toBeGreaterThan(50);
  expect(manifest.targets.setup).toBeDefined();
  expect(manifest.metadata.semio.dashboard.parameters.length).toBeGreaterThan(0);
  const registry = readFileSync(join(dashboard, "🎮️registry/🧾️declarations/🦀️.rs"), "utf8");
  expect(registry).not.toContain("workspace_scripts");
});

test("the dashboard has no editor projection route or implementation", () => {
  const manifest = read(join(dashboard, "📦️packages/🦀️rust/📋️project.json"));
  expect(manifest.targets.launch).toBeUndefined();
  const script = readFileSync(join(dashboard, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
  for (const name of ["LaunchScript", "projectDashboardLaunch", "launchParameterArguments", 'register("launch"']) expect(script).not.toContain(name);
  expect(existsSync(join(dashboard, "🌳️command-tree/🚀️launch/🟦️.ts"))).toBe(false);
  expect(readFileSync(join(dashboard, "README.md"), "utf8")).not.toContain("VSCode Launch Projection");
});

test("the editor extension exposes source navigation without developer operations", () => {
  const extension = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode");
  const contract = read(join(extension, "🧫️fixtures/🎛️dashboard-only/🔣️.json"));
  const manifest = read(join(extension, "📦️packages/🟦️typescript/package.json"));
  const source = readFileSync(join(extension, "🟦️.ts"), "utf8");
  const contributions = JSON.stringify(manifest.contributes);
  for (const command of contract.forbiddenCommands) {
    expect(contributions, command).not.toContain(JSON.stringify(command));
    expect(source, command).not.toContain(JSON.stringify(command));
    expect(source, command).not.toContain(`'${command}'`);
  }
  for (const symbol of contract.forbiddenSymbols) expect(source, symbol).not.toContain(symbol);
});

test("authored scripts use one declarative build entry", () => {
  const manifest = read(join(root, "📋️project.json"));
  const tool = manifest.metadata.semio.dashboard.tools.find((tool: { id: string }) => tool.id === "run-script");
  expect(tool).toBeDefined();
  expect(tool.command).toEqual(["bun", "repo", "exec", "--projects={project}", "--", "bun", "{workspace}/{script}"]);
  expect(tool.cwd).toBe("{directory}");
  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "script")).toMatchObject({ id: "script", kind: "text", required: true });
  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "project")).toMatchObject({ required: true, default: "", suggestions: { source: "projects" } });
  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "script").suggestions).toEqual({ source: "scripts", projectParameter: "project", directoryParameter: "directory" });
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const declaration = oracle.getSchema(`${schema.$id}#/$defs/ProjectManifest`)!;
  expect(declaration(manifest), oracle.errorsText(declaration.errors)).toBe(true);
});

test("only the two retained agent clients can own derived MCP configuration", () => {
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const validateClient = new Ajv2020({ strict: false }).compile(schema.$defs.AgentClientId);
  const native = readFileSync(join(dashboard, "🎮️registry/🧾️declarations/🦀️.rs"), "utf8");
  const clients = JSON.parse(native.match(/pub const AGENT_CLIENTS: \[&str; \d+\] = (\[[^;]+\]);/)![1]!);
  for (const row of fixture.agentCases) {
    expect(validateClient(row.client), row.client).toBe(row.accepted);
    expect(clients.includes(row.client), row.client).toBe(row.accepted);
  }
});

test("course and proctor compound belongs to the course project and retains readiness and cancellation", () => {
  const course = read(join(root, "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"));
  const proctor = read(join(root, "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"));
  expect(course.metadata.semio.dashboard.compounds.find((compound: { id: string }) => compound.id === "course-with-proctor")).toEqual({ id: "course-with-proctor", verb: "dev", stop: "together", members: [{ run: "@teaching/proctor:dev" }, { run: "@teaching/architecture-course:dev" }] });
  expect(proctor.targets.dev.metadata.semio.dashboard.ready).toEqual({ port: 8791, portEnv: "PROCTOR_PORT" });
  expect(course.targets.dev.metadata.semio.dashboard.ready.port).toBe(6061);
});

test("Bazel runtime controls retain typed dashboard phases and both locales", async () => {
  const bazel = read(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🧫️fixtures/🔣️.json"));
  const registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json")), oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(registry);
  const target = read(join(root, "📋️project.json")).targets["test-build-tool-bazel"], declaration = target.metadata.semio.dashboard;
  const validate = oracle.getSchema(`${registry.$id}#/$defs/TargetDeclaration`)!;
  expect(validate(declaration), oracle.errorsText(validate.errors)).toBe(true);
  expect(target.options.forwardAllArgs).toBe(true);
  const parameter = declaration.parameters.find((value: { id: string }) => value.id === bazel.testDispatch.dashboardParameter.id);
  expect(parameter.kind).toBe("choice"); expect(parameter.default).toBeUndefined();
  expect(parameter.values).toEqual(bazel.integrationPhases.map((phase: string) => ({ id: phase, args: [phase] })));
  for (const field of ["title", "description"]) expect(Object.keys(parameter[field]).sort()).toEqual(["de", "en"]);
  expect(fixture.liveControls.filter((row: { id: string }) => row.id === "workspace:test-build-tool-bazel").map((row: { parameters: { phase: string } }) => row.parameters.phase)).toEqual(bazel.integrationPhases);
  const build = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
  const dispatchSchema = read(join(build, "🔧️tools/🏗️bazel/🧪️tests/🧬️schema/📋️dispatch.json"));
  const ts = await import("typescript"), { compileJsonSchemaSubset } = await import(join(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"));
  const source = readFileSync(join(build, "📜️script.ts"), "utf8"), tree = ts.createSourceFile("build.ts", source, ts.ScriptTarget.Latest, true);
  const definition = tree.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "bazelTestCommandsV1");
  expect(definition).toBeDefined();
  const body = ts.transpileModule(definition!.getText(tree).replace(/^export /u, "") + ";return bazelTestCommandsV1;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  const dispatch = new Function("bazelTestSchema", "bazelTestContract", body)(dispatchSchema, compileJsonSchemaSubset(dispatchSchema));
  const independent = new Ajv({ strict: true }).compile(dispatchSchema);
  for (const row of [...bazel.testDispatch.cases, ...parameter.values.map((value: { id: string; args: string[] }) => ({ arguments: value.args, phases: [value.id] }))]) {
    expect(independent({ phase: row.arguments[0] ?? null })).toBe(true);
    expect(dispatch(row.arguments).map((value: { phase: string | null }) => value.phase)).toEqual(row.phases);
  }
  for (const args of bazel.testDispatch.rejected) {
    expect(independent({ phase: args[0] ?? null }) && args.length < 2).toBe(false);
    expect(() => dispatch(args)).toThrow();
  }
  console.log(`[DEBUG] authority validated ${bazel.integrationPhases.length} Bazel phase declarations, bilingual metadata and native dry-run vectors`);
});

test("the native dashboard resolves retained live controls with typed inputs and explicit argument vectors", () => {
  const binary = process.env.SEMIO_DASHBOARD_BIN ?? installedDashboard(root, "prototype", root);
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const validateLaunch = oracle.getSchema(`${schema.$id}#/$defs/Launch`)!;
  for (const row of fixture.liveControls) {
    const expand = (value: string) => value.replaceAll("{workspace}", root);
    const args = ["run", row.id, "--root", root, "--dry-run", ...Object.entries(row.parameters).flatMap(([key, value]) => ["--param", `${key}=${expand(String(value))}`]), ...Object.entries(row.env).flatMap(([key, value]) => ["--env", `${key}=${value}`]), ...(row.args.length ? ["--", ...row.args] : [])];
    const run = Bun.spawnSync([binary, ...args], { cwd: root, env: { ...process.env, SEMIO_DASHBOARD_GRAPH: "fresh" }, stdout: "pipe", stderr: "pipe" });
    expect(run.exitCode, `${row.id}: ${run.stderr}`).toBe(0);
    const launch = JSON.parse(run.stdout.toString());
    expect(validateLaunch(launch), `${row.id}: ${oracle.errorsText(validateLaunch.errors)}`).toBe(true);
    expect(launch.processes.map((process: { commandId: string }) => process.commandId), row.id).toEqual(row.commandIds);
    if (row.words) expect(launch.processes[0].args, row.id).toEqual(row.words.map(expand));
    if (row.stop) expect(launch.stop, row.id).toBe(row.stop);
    if (row.ports) expect(launch.processes.map((process: { ready: { port: number } }) => process.ready.port), row.id).toEqual(row.ports);
    for (const process of launch.processes) for (const [key, value] of Object.entries(row.env)) expect(Object.fromEntries(process.env)[key], `${row.id}: ${key}`).toBe(value);
    console.log(`[DEBUG] native dashboard resolved ${row.id}: ${launch.processes.map((process: { commandId: string }) => process.commandId).join(", ")}`);
  }
}, 120_000);

test("workspace startup never waits for Nx analytics consent", () => {
  const config = read(join(root, "nx.json"));
  expect(config.analytics).toBe(false);
  const schema = read(join(root, "node_modules/nx/schemas/nx-schema.json"));
  const validateAnalytics = new Ajv2020({ strict: false }).compile(schema.properties.analytics);
  expect(validateAnalytics(config.analytics)).toBe(true);
});


test("ready-address opening has a customizable binding and independent schema vectors", () => {
  const vectors = read(join(dashboard, "🧫️fixtures/🌐️open/🔣️.json"));
  const schema = read(join(dashboard, "🧬️schema/🌐️open/🔣️.json"));
  const oracle = new Ajv2020({ strict: false });
  const validateOpener = oracle.compile(schema.$defs.Opener), validateTask = oracle.compile(schema.$defs.ReadyTask);
  const clean = (value: unknown): value is string => typeof value === "string" && value.length > 0 && !/[\u0000\r\n]/.test(value);
  for (const row of vectors.openers) {
    expect(Array.isArray(row.value) && row.value.length > 0 && row.value.every(clean)).toBe(row.accepted);
    expect(validateOpener(row.value)).toBe(row.accepted);
  }
  for (const row of vectors.tasks) {
    expect(row.status === "running" && clean(row.url) && /^https?:\/\/.+/.test(row.url)).toBe(row.accepted);
    expect(validateTask(row)).toBe(row.accepted);
  }
  const keymap = read(join(dashboard, "⚙️preferences/⌨️keymap/🔣️.json"));
  expect(keymap.bindings.find((binding: { action: string }) => binding.action === vectors.action)).toEqual({ scope: "prefix", action: vectors.action, keys: [vectors.key] });
});

test("every active source directory is free of IDE configuration and workspace files", () => {
  const failures = activeRepositoryPaths().filter(({ path }) => !allowedPath(path)).map(({ path }) => path);
  expect(failures.sort(), "active IDE configurations compete with the dashboard registry").toEqual([]);
}, 300_000);


test("all active owner manifests retain executable canonical scripts and valid dashboard declarations", () => {
  const authority = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")), registry = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(authority); oracle.addSchema(registry);
  const validateTarget = oracle.getSchema(`${authority.$id}#/$defs/OwnerTarget`)!, validateManifest = oracle.getSchema(`${registry.$id}#/$defs/ProjectManifest`)!;
  const failures: string[] = [], owners = new Set<string>();
  let targets = 0, commands = 0;
  for (const { path, directory } of activeRepositoryPaths()) {
    if (directory || !/(?:^|\/)📋️project\.json$/.test(path) || /(?:^|\/)(?:🧫️fixtures|fixtures)(?:\/|$)/.test(path)) continue;
    const manifest = read(join(root, path)), projectRoot = dirname(join(root, path));
    if (owners.has(manifest.name)) failures.push(`${path}: duplicate owner ${manifest.name}`);
    owners.add(manifest.name);
    if (!validateManifest(manifest)) failures.push(`${path}: ${oracle.errorsText(validateManifest.errors)}`);
    for (const [name, target] of Object.entries(manifest.targets ?? {}) as [string, Record<string, any>][]) {
      targets++;
      const modeled = acceptsOwnerTarget(target), validated = validateTarget(target);
      if (!modeled || !validated) failures.push(`${path}#${name}: model=${modeled}, schema=${validated}, ${oracle.errorsText(validateTarget.errors)}`);
      for (const options of [target.options ?? {}, ...Object.values(target.configurations ?? {})] as Record<string, any>[]) {
        const cwd = (options.cwd ?? target.options?.cwd ?? "{projectRoot}").replaceAll("{workspaceRoot}", root).replaceAll("{projectRoot}", projectRoot);
        for (const raw of [target.command, options.command, ...(options.commands ?? [])].filter(value => value !== undefined)) {
          commands++;
          const command = object(raw) ? raw.command : raw, match = typeof command === "string" ? ownerCommand.exec(command) : null;
          if (!match || !existsSync(resolve(root, cwd, match[1] ?? match[2]!))) failures.push(`${path}#${name}: missing canonical script for ${JSON.stringify(command)}`);
        }
      }
    }
  }
  expect(owners.size).toBeGreaterThan(100);
  expect(targets).toBeGreaterThan(3000);
  expect(failures.sort(), "every owner must resolve commands through its permanent scripts and dashboard declarations").toEqual([]);
  console.log(`[DEBUG] authority checked ${owners.size} owners, ${targets} targets and ${commands} existing canonical script invocations`);
}, 300_000);

