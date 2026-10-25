import { WORKSPACE_ROOT } from "../../../🗂️workspaces/📍️checkout-root/🟦️.ts";
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { posix } from "node:path";
import { rustTokens, rustTokenPairs, rustTokenSegments, rustAttributes, rustVisibility } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
/** 📋️ Tool-job coverage inventory: reserved ids, static rows, publication contracts, app-owned rows, dispositions and proofs.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { policyTestModSpans } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";

//#region 🎯️ToolJobCoverage
export type ToolJobCoverageReport = {
  macroHostFiles: number;
  macroInvocations: number;
  commandRows: number;
  uniqueCommandRows: number;
  duplicateRows: { file: string; id: string }[];
  fixtureMacroHostFiles: number;
  fixtureMacroInvocations: number;
  fixtureCommandRows: number;
  boundedRows: number;
  batchOnlyRows: number;
  forbiddenRows: number;
  deletedRows: number;
  productionFactories: number;
  productionRegistrations: number;
  productionDispatches: number;
  aliases: number;
  factoryContracts: { file: string; line: number; factory: string; status: "explicit" | "missing" }[];
  frameworkReservedRoutes: { id: string; status: "failClosedPendingFactory" }[];
  reservedImporterOwners: { owner: string; file: string; id: "import-media"; status: "failClosedPendingFactory" }[];
  globalPayloadStores: GlobalPayloadStoreInventoryRow[];
  globalPayloadStoreExemptions: GlobalPayloadStoreInventoryRow[];
  scanThenMonolithRows: { file: string; id: string; helper: string }[];
  literalRegistrations: number;
  acceptedCommandRows: ToolJobStaticRow[];
  acceptedReservedRoutes: string[];
  remainingCommands: { file: string; id: string; reason: string; source: "literal" | "macro" }[];
  pipelineGates: {
    fullToolOperationJobBounded: boolean;
    durableOneItemPublicationBounded: boolean;
    ephemeralOneItemPublicationBounded: boolean;
    fullToolOperationBounded: boolean;
  };
  selfTests: number;
  failures: string[];
};

/** 🎯️ Finds the matching Rust brace while ignoring quoted strings and comments. */
export function toolJobRustBlock(source: string, open: number): { body: string; end: number } | undefined {
  let depth = 0;
  let string = false;
  let lineComment = false;
  let blockComment = 0;
  let escaped = false;
  for (let index = open; index < source.length; index += 1) {
    const char = source[index]!;
    const next = source[index + 1] ?? "";
    if (lineComment) {
      if (char === "\n") lineComment = false;
      continue;
    }
    if (blockComment > 0) {
      if (char === "/" && next === "*") {
        blockComment += 1;
        index += 1;
      } else if (char === "*" && next === "/") {
        blockComment -= 1;
        index += 1;
      }
      continue;
    }
    if (string) {
      if (escaped) escaped = false;
      else if (char === "\\") escaped = true;
      else if (char === '"') {
        string = false;
      }
      continue;
    }
    if (char === "/" && next === "/") {
      lineComment = true;
      index += 1;
      continue;
    }
    if (char === "/" && next === "*") {
      blockComment = 1;
      index += 1;
      continue;
    }
    if (char === '"') string = true;
    else if (char === "{") depth += 1;
    else if (char === "}" && --depth === 0) return { body: source.slice(open + 1, index), end: index + 1 };
  }
  return undefined;
}

function toolJobRustArray(source: string, open: number): { body: string; end: number } | undefined {
  if (source[open] !== "[") return undefined;
  let depth = 0;
  let string = false;
  let escaped = false;
  let lineComment = false;
  let blockComment = 0;
  for (let index = open; index < source.length; index += 1) {
    const char = source[index]!;
    const next = source[index + 1];
    if (lineComment) {
      if (char === "\n") lineComment = false;
      continue;
    }
    if (blockComment > 0) {
      if (char === "/" && next === "*") {
        blockComment += 1;
        index += 1;
      } else if (char === "*" && next === "/") {
        blockComment -= 1;
        index += 1;
      }
      continue;
    }
    if (string) {
      if (escaped) escaped = false;
      else if (char === "\\") escaped = true;
      else if (char === '"') string = false;
      continue;
    }
    if (char === "/" && next === "/") {
      lineComment = true;
      index += 1;
      continue;
    }
    if (char === "/" && next === "*") {
      blockComment = 1;
      index += 1;
      continue;
    }
    if (char === '"') string = true;
    else if (char === "[") depth += 1;
    else if (char === "]" && --depth === 0) return { body: source.slice(open + 1, index), end: index + 1 };
  }
  return undefined;
}

export type ToolJobStaticRow = { file: string; id: string; source: "literal" | "macro" };

type ToolJobPublicationLane = "HostOnly" | "Artifact" | "Config" | "WindowConfig" | "WindowTransient" | "Draft" | "Presence" | "Transient" | "Child" | "Interaction";
type ToolJobAppOwnedRow = {
  ownerFile: string;
  factoryFile: string;
  ownerTypeName: string;
  factory: string;
  toolId: string;
  publicationLanes: readonly ToolJobPublicationLane[];
  publicationReady: boolean;
};

type GlobalPayloadStoreKind = "child-content-scratch" | "resizable-operation-registry" | "fixed-operation-registry" | "abi-bridge-retention" | "mutable-entropy" | "mutable-payload";
type GlobalPayloadStoreOwner = { name: string; rustType: string; kind: GlobalPayloadStoreKind };
type GlobalPayloadStoreInventoryRow = { file: string; line: number; text: string; owners: GlobalPayloadStoreOwner[] };

/** ⚙️ The host-configuration tools the framework's `dispatch_action` answers with an empty `Emit`: their generic bounded first step is a framework proof, so no plugin-local action row declares them. */
export const TOOL_JOB_HOST_CONFIGURATION_IDS: readonly string[] = ["setActiveTool", "setActiveUtility"];

export const TOOL_JOB_FRAMEWORK_RESERVED_IDS = [
  "undo",
  "redo",
  "commitCheckpoint",
  "createAlternative",
  "switchAlternative",
  "checkoutCheckpoint",
  "revertToCommand",
  "copy",
  "cut",
  "paste",
  "noteShellCommand",
  "setHistoryCommandFilter",
  "recordTutorial",
  "clearSelection",
  "interactionHover",
  "interactionSelect",
  "selectAll",
  "setInteractionGranularity",
  "setSelectionMode",
  "configuration-binary",
] as const;
export const TOOL_JOB_PLUGIN_RESERVED_IDS = ["copy", "cut", "paste", "import-media"] as const;
export const TOOL_JOB_RESERVED_IDS = [...TOOL_JOB_FRAMEWORK_RESERVED_IDS, ...TOOL_JOB_PLUGIN_RESERVED_IDS] as const;
export const TOOL_JOB_IMPORTER_INVENTORY = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yj-importer-cohorts.json";
export const TOOL_JOB_CHILD_RETIREMENT_INVENTORY = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yt-child-snapshot-retirement-cohorts.json";

/** 🧹️ Removes in-file test modules, with the attributes that precede them, from the production registration census. */
export function toolJobProductionSource(source: string): string {
  const lines = source.split("\n");
  for (const span of policyTestModSpans(lines)) {
    let first = span.startLine - 1;
    while (first > 0 && /^\s*#\[/.test(lines[first - 1]!)) first -= 1;
    for (let line = first; line < span.endLine; line += 1) lines[line] = "";
  }
  return lines.join("\n");
}

/** 🎭️ Masks only test-only Rust items, preserving neighboring fields and production cfg alternatives. */
export function toolJobProductionProtocolSource(raw: string): string {
  const source = raw, tokens = rustTokens(source), pairs = rustTokenPairs(tokens), spans: [number, number][] = [];
  const production = (start: number, end: number): number => {
    if (end === start + 1 && tokens[start]?.text === "test") return 0;
    const name = tokens[start]?.text, open = start + 1, close = pairs.get(open);
    if (tokens[open]?.text !== "(" || close !== end - 1 || !["all", "any", "not"].includes(name ?? "")) return 2;
    const values = rustTokenSegments(tokens, pairs, open + 1, close, ",").map(([a, b]) => production(a, b));
    if (name === "not") return values.length === 1 && values[0] !== 2 ? 1 - values[0]! : 2;
    if (name === "all") return values.includes(0) ? 0 : values.every(value => value === 1) ? 1 : 2;
    return values.includes(1) ? 1 : values.every(value => value === 0) ? 0 : 2;
  };
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "#" || tokens[index + 1]?.text !== "[") continue;
    const attributes = rustAttributes(tokens, pairs, index, tokens.length);
    const testOnly = attributes.ranges.some(([a, b]) => tokens[a]?.text === "cfg" && tokens[a + 1]?.text === "(" && pairs.get(a + 1) === b - 1 && production(a + 2, b - 1) === 0);
    if (!testOnly) { index = attributes.next - 1; continue; }
    const definition = rustVisibility(tokens, pairs, attributes.next).next, field = tokens[definition]?.kind === "identifier" && tokens[definition + 1]?.text === ":";
    let angle = 0, last = -1, expression = false;
    for (let cursor = attributes.next; cursor < tokens.length; cursor++) {
      const text = tokens[cursor]!.text, paired = pairs.get(cursor);
      if (text === "=") expression = true;
      if (text === "<" && (!expression || tokens[cursor - 1]?.text === "::")) { angle++; continue; }
      if (/^>+$/.test(text)) { angle = Math.max(0, angle - text.length); continue; }
      if (paired !== undefined && paired > cursor) {
        if (text === "{" && !angle && !field && tokens[cursor - 1]?.text !== "!") { last = paired + (tokens[paired + 1]?.text === "," ? 1 : 0); break; }
        cursor = paired;
        continue;
      }
      if (!angle && [",", ";"].includes(text)) { last = cursor; break; }
      if (!angle && [")", "}"].includes(text)) { last = cursor - 1; break; }
    }
    if (last < attributes.next) throw new Error("[production Rust source] test item boundary is not proved");
    spans.push([tokens[index]!.start, tokens[last]!.end]);
    index = last;
  }
  let result = source;
  for (const [start, end] of spans.reverse()) result = result.slice(0, start) + source.slice(start, end).replace(/[^\r\n]/g, " ") + result.slice(end);
  return result;
}

export function toolJobGlobalPayloadStoreInventory(files: ReadonlyMap<string, string>): GlobalPayloadStoreInventoryRow[] {
  const kindOf = (name: string, rustType: string): GlobalPayloadStoreKind => {
    if (/BRIDGE|RETAINED/.test(name)) return "abi-bridge-retention";
    if (/SCRATCH|STAGING|BLOBS|BATCHES/.test(name)) return "child-content-scratch";
    if (/ENTROPY|SEED/.test(name) && /Cells*</.test(rustType)) return "mutable-entropy";
    if (/HashMap|BTreeMap|Vecs*</.test(rustType)) return "resizable-operation-registry";
    if (/\[[^\]]*;[^\]]+\]|Registry|Recovery|ArenaProcessState/.test(rustType)) return "fixed-operation-registry";
    return "mutable-payload";
  };
  const ownersOf = (source: string): GlobalPayloadStoreOwner[] => {
    const owners: GlobalPayloadStoreOwner[] = [];
    for (const owner of source.matchAll(/\bstatic\s+([A-Z][A-Z0-9_]*)\s*:\s*([^=]+?)\s*=/g)) {
      const rustType = owner[2]!.trim().replaceAll(/\s+/g, " ");
      owners.push({ name: owner[1]!, rustType, kind: kindOf(owner[1]!, rustType) });
    }
    return owners;
  };
  const rows: GlobalPayloadStoreInventoryRow[] = [];
  for (const [file, source] of files) {
    const threadLocal = /thread_local!\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = threadLocal.exec(source))) {
      const open = source.indexOf("{", match.index);
      const block = toolJobRustBlock(source, open);
      if (!block) break;
      rows.push({ file, line: source.slice(0, match.index).split("\n").length, text: match[0], owners: ownersOf(block.body) });
      threadLocal.lastIndex = block.end;
    }
    const locked = /(?:OnceLock|LazyLock)\s*<\s*(?:std::sync::)?Mutex\s*</g;
    while ((match = locked.exec(source))) {
      const start = source.lastIndexOf("static ", match.index);
      const initializer = source.indexOf("=", match.index);
      const end = initializer < 0 ? -1 : source.indexOf(";", initializer);
      const declaration = start < 0 || end < 0 ? "" : source.slice(start, end + 1);
      rows.push({ file, line: source.slice(0, match.index).split("\n").length, text: match[0], owners: ownersOf(declaration) });
    }
  }
  return rows;
}

/** 🗄️ Exempts only the shared, executable operation/generation registry contract. Bespoke arrays,
 * registries, aliases, and annotation-only claims remain blockers until they use this authority. */
export function toolJobGlobalPayloadStoreIsSharedFixedAuthority(row: GlobalPayloadStoreInventoryRow): boolean {
  return (
    row.owners.length > 0 &&
    row.owners.every(
      (owner) =>
        owner.kind === "fixed-operation-registry" &&
        /(?:OnceLock\s*<\s*(?:std::sync::)?Mutex\s*<|RefCell\s*<)\s*semio_framework_job::FixedOperationRegistry\s*</.test(owner.rustType),
    )
  );
}

/** 🔬️ Enumerates macro and ordinary builder registrations without trusting a manifest disposition. */
/** 🏭️ The tool ids each proof-cataloged `ToolJobFactory` of one owner text classifies through its own `classification()` and `ArtifactOwnedToolJobFactory::TOOL_IDS`. */
function toolJobClassifiedFactories(source: string, constants: ReadonlyMap<string, string>, lists: ReadonlyMap<string, string[]>): { classification: string; ids: string[] }[] {
  const result: { classification: string; ids: string[] }[] = [];
  const proven = new Set([...source.matchAll(/factory_type:\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)\s*,/g)].map((entry) => entry[1]!));
  const factories = /impl\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolJobFactory\s+for\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g;
  let match: RegExpExecArray | null;
  while ((match = factories.exec(source))) {
    const block = toolJobRustBlock(source, source.indexOf("{", match.index));
    if (!block) break;
    factories.lastIndex = block.end;
    const classification = !proven.has(match[1]!) ? undefined : block.body.match(/fn\s+classification\s*\([^)]*\)[^{]*\{[^{}]*?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)/)?.[1];
    const owned = classification ? source.match(new RegExp(`impl\\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory\\s+for\\s+${match[1]}\\s*\\{`)) : undefined;
    const ownedBody = owned ? toolJobRustBlock(source, source.indexOf("{", owned.index))?.body : undefined;
    const expression = ownedBody?.match(/const\s+TOOL_IDS\s*:[^=]+=\s*([^;]+);/s)?.[1]?.trim();
    const ids = expression === undefined ? [] : expression.startsWith("&[")
      ? [...expression.matchAll(/"([^"]+)"|\b([A-Z][A-Z0-9_]*)\b/g)].map((item) => item[1] ?? constants.get(item[2]!)).filter((id): id is string => !!id)
      : lists.get(expression.match(/([A-Z][A-Z0-9_]*)$/)?.[1] ?? "") ?? [];
    if (classification && ids.length > 0) result.push({ classification, ids });
  }
  return result;
}

export function toolJobStaticRows(files: ReadonlyMap<string, string>): ToolJobStaticRow[] {
  const rows: ToolJobStaticRow[] = [];
  const constants = new Map<string, string>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)) constants.set(match[1]!, match[2]!);
  }
  const lists = new Map<string, string[]>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?\[\s*&(?:'static\s+)?str\s*\]\s*=\s*&\[([^\]]*)\]\s*;/gs)) {
      lists.set(match[1]!, [...match[2]!.matchAll(/"([^"]+)"|\b([A-Z][A-Z0-9_]*)\b/g)].map((item) => item[1] ?? constants.get(item[2]!)).filter((id): id is string => !!id));
    }
  }
  for (const [file, source] of files) {
    const invocation = /^\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*app_commands!\s*\{/gm;
    let match: RegExpExecArray | null;
    while ((match = invocation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      const row = /^\s*"([^"]+)"(?:\s+as\s+"[^"]+")?\s*=>/gm;
      let item: RegExpExecArray | null;
      while ((item = row.exec(block.body))) rows.push({ file, id: item[1]!, source: "macro" });
      invocation.lastIndex = block.end;
    }
    const typedVariants = /^\s*[A-Za-z_][A-Za-z0-9_]*_command_variants!\s*\{/gm;
    while ((match = typedVariants.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      const row = /^\s*[A-Za-z_][A-Za-z0-9_]*\s*=\s*(?:"([^"]+)"|(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*))\s*,?/gm;
      let item: RegExpExecArray | null;
      while ((item = row.exec(block.body))) {
        const id = item[1] ?? constants.get(item[2] ?? "");
        if (id) rows.push({ file, id, source: "literal" });
      }
      typedVariants.lastIndex = block.end;
    }
    const literal = /\.(?:mutation|operation|view_action|shell_action)\(\s*"([^"]+)"|\.action_with\(\s*ActionDefinition(?:\s*\{[\s\S]{0,300}?\.\.)?::(?:new_catalog|bounded_catalog|new)\(\s*"([^"]+)"|\.(?:app_command|command_with)\(\s*CommandDefinition::(?:new_catalog|bounded_catalog|new)\(\s*"([^"]+)"/g;
    while ((match = literal.exec(source))) rows.push({ file, id: match[1] ?? match[2] ?? match[3]!, source: "literal" });
    for (const factory of toolJobClassifiedFactories(source, constants, lists)) for (const id of factory.ids) rows.push({ file, id, source: "literal" });
  }
  const unique = new Map<string, ToolJobStaticRow>();
  for (const row of rows) unique.set(`${row.file}\0${row.id}`, row);
  return [...unique.values()].sort((left, right) => left.file.localeCompare(right.file) || left.id.localeCompare(right.id));
}

export function toolJobPublicationContracts(
  source: string,
  factoryBody: string,
  strings: ReadonlyMap<string, string>,
): Map<string, ToolJobPublicationLane[]> | undefined {
  const header = factoryBody.match(/const\s+PUBLICATION_CONTRACTS\s*:\s*&'static\s*\[\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\]\s*=\s*/s);
  if (!header) return undefined;
  const valueAt = header.index! + header[0].length;
  const literal = factoryBody.startsWith("&[", valueAt) ? toolJobRustArray(factoryBody, valueAt + 1) : undefined;
  const expression = literal ? factoryBody.slice(valueAt, literal.end).trim() : factoryBody.slice(valueAt).match(/^([^;]+);/s)?.[1]?.trim();
  if (!expression) return undefined;
  const constant = expression.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)$/)?.[1];
  const declaration = constant ? source.search(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?const\\s+${constant}\\s*:\\s*&(?:'static\\s+)?\\[\\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\\s*\\]\\s*=`)) : -1;
  const open = declaration < 0 ? -1 : source.indexOf("[", source.indexOf("=", declaration));
  const array = open < 0 ? undefined : toolJobRustArray(source, open);
  const contracts = constant ? array?.body : literal?.body;
  if (!contracts) return undefined;
  const result = new Map<string, ToolJobPublicationLane[]>();
  const row = /(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\{\s*tool_id:\s*(?:"([^"]+)"|(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*))\s*,\s*lanes:\s*&\[([^\]]*)\]\s*\}/g;
  let match: RegExpExecArray | null;
  while ((match = row.exec(contracts))) {
    const toolId = match[1] ?? strings.get(match[2] ?? "");
    const names = match[3]!.split(",").map((lane) => lane.trim()).filter(Boolean);
    const lanes = names.map((lane) => lane.match(/^(?:semio_framework_plugin::)?ArtifactToolPublicationLane::(HostOnly|Artifact|Config|WindowConfig|WindowTransient|Draft|Presence|Transient|Child|Interaction)$/)?.[1] as ToolJobPublicationLane | undefined);
    if (lanes.some((lane) => lane === undefined)) return undefined;
    if (!toolId || result.has(toolId) || lanes.length === 0 || new Set(lanes).size !== lanes.length || (lanes.includes("HostOnly") && lanes.length !== 1)) return undefined;
    result.set(toolId, lanes as ToolJobPublicationLane[]);
  }
  return result.size > 0 ? result : undefined;
}

export function toolJobPublicationAuthorityReady(ownerBody: string, lanes: readonly ToolJobPublicationLane[], files: ReadonlyMap<string, string> = new Map(), ownerFile: string = ""): boolean {
  const factory = (name: string) => {
    const start = ownerBody.search(new RegExp(`fn\\s+${name}\\s*\\(`));
    const open = start < 0 ? -1 : ownerBody.indexOf("{", start);
    const body = open < 0 ? undefined : toolJobRustBlock(ownerBody, open);
    return !!body && /\bSome\s*\(/.test(body.body);
  };
  const registration = (name: string) => {
    const start = ownerBody.search(new RegExp(`fn\\s+${name}\\s*\\(`));
    const open = start < 0 ? -1 : ownerBody.indexOf("{", start);
    const body = open < 0 ? undefined : toolJobRustBlock(ownerBody, open);
    return !!body && (/\bregistry\s*\.register::</.test(body.body) || /::register\(\s*registry\s*\)/.test(body.body));
  };
  return lanes.every((lane) => {
    if (lane === "HostOnly") return true;
    if (lane === "Child") return false;
    if (lane === "Interaction") return true;
    if (lane === "WindowConfig") return registration("register_window_config_owners");
    if (lane === "WindowTransient") return registration("register_window_transient_owners");
    if (lane === "Artifact") return factory("build_artifact_store_one_item_preparation_factory");
    if (lane === "Config") return factory("build_config_store_one_item_preparation_factory");
    if (lane === "Draft") return factory("build_draft_store_one_item_preparation_factory");
    if (lane === "Presence") return factory("build_presence_store_one_item_preparation_factory") && factory("build_presence_local_root_retirement_factory");
    return factory("build_transient_store_one_item_preparation_factory") && factory("build_transient_local_root_retirement_factory");
  });
}

/** 🏷️ Resolves a one-identifier type expression through a `type NAME = …;` alias declared in the same text, leaving every other expression as written. */
export function toolJobResolveTypeAlias(source: string, expression: string): string {
  const name = expression.trim();
  return /^[A-Za-z_][A-Za-z0-9_]*$/.test(name) ? (source.match(new RegExp(`\\btype\\s+${name}\\s*=\\s*([^;]+);`))?.[1]?.trim() ?? name) : expression;
}

/** 🧩️ Enumerates only command rows backed by an explicitly registered app-owned factory and builder. */
export function toolJobAppOwnedRows(files: ReadonlyMap<string, string>): ToolJobAppOwnedRow[] {
  const stringConstants = new Map<string, Map<string, string>>();
  const listConstants = new Map<string, Map<string, string[]>>();
  for (const [file, source] of files) {
    const strings = new Map<string, string>();
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)) strings.set(match[1]!, match[2]!);
    stringConstants.set(file, strings);
    const lists = new Map<string, string[]>();
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?\[\s*&(?:'static\s+)?str\s*\]\s*=\s*&\[([^\]]*)\]\s*;/gs)) {
      const ids: string[] = [];
      for (const item of match[2]!.split(",")) {
        const literal = item.match(/"([^"]+)"/)?.[1];
        const constant = item.trim().match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)$/)?.[1];
        const id = literal ?? strings.get(constant ?? "");
        if (id) ids.push(id);
      }
      lists.set(match[1]!, ids);
    }
    listConstants.set(file, lists);
  }
  const ownerFiles = new Map<string, { file: string; body: string }>();
  for (const [file, source] of files) {
    const owner = /impl\s+(?:semio_framework_plugin::)?(ArtifactEditor|ArtifactApp|ArtifactViewer)\s+for\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = owner.exec(source))) {
      const open = source.indexOf("{", match.index);
      const block = toolJobRustBlock(source, open);
      if (!block) break;
      ownerFiles.set(`${match[1]}\0${match[2]}`, { file, body: block.body });
      owner.lastIndex = block.end;
    }
  }
  const production = [...files.values()].join("\n");
  const rows: ToolJobAppOwnedRow[] = [];
  for (const [factoryFile, source] of files) {
    const implementation = /impl\s+(?:semio_framework_plugin::)?ArtifactOwnedToolJobFactory\s+for\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = implementation.exec(source))) {
      const factory = match[1]!;
      const open = source.indexOf("{", match.index);
      const block = toolJobRustBlock(source, open);
      if (!block) break;
      implementation.lastIndex = block.end;
      const ownerExpression = toolJobResolveTypeAlias(source, block.body.match(/type\s+Owner\s*=\s*([^;]+)\s*;/)?.[1] ?? "").replaceAll(/\s+/g, "") || undefined;
      const editorOwner = ownerExpression?.match(/^(?:semio_framework_plugin::)?EditorApp<([A-Za-z_][A-Za-z0-9_]*)>$/)?.[1];
      const viewerOwner = ownerExpression?.match(/^(?:semio_framework_plugin::)?ViewerApp<([A-Za-z_][A-Za-z0-9_]*)>$/)?.[1];
      const directOwner = editorOwner || viewerOwner ? undefined : ownerExpression?.match(/^(?:semio_framework_plugin::)?([A-Za-z_][A-Za-z0-9_]*)$/)?.[1];
      const ownerTypeName = editorOwner ?? viewerOwner ?? directOwner;
      const toolExpression = block.body.match(/const\s+TOOL_IDS\s*:\s*&'static\s*\[\s*&'static\s+str\s*\]\s*=\s*([^;]+);/s)?.[1]?.trim();
      const owner = editorOwner ? ownerFiles.get(`ArtifactEditor\0${editorOwner}`) : viewerOwner ? ownerFiles.get(`ArtifactViewer\0${viewerOwner}`) : directOwner ? ownerFiles.get(`ArtifactApp\0${directOwner}`) : undefined;
      const registered = new RegExp(`registry\\.register\\(\\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*${factory}::new\\(`).test(production);
      const concrete = new RegExp(`impl\\s+(?:semio_framework::)?ToolJobFactory\\s+for\\s+${factory}\\s*\\{`).test(source) && source.includes("fn create_job") && source.includes("fn execution_contract");
      const builder = !!owner && /fn\s+build_tool_job\s*\([^)]*request/s.test(owner.body) && owner.body.includes("request");
      if (!owner || !ownerTypeName || !registered || !concrete || !builder || !toolExpression) continue;
      const localStrings = stringConstants.get(factoryFile) ?? new Map<string, string>();
      const localLists = listConstants.get(factoryFile) ?? new Map<string, string[]>();
      const ids = toolExpression.startsWith("&[")
        ? toolExpression
            .slice(2, toolExpression.lastIndexOf("]"))
            .split(",")
            .map((item) => item.match(/"([^"]+)"/)?.[1] ?? localStrings.get(item.trim().match(/(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)/)?.[1] ?? ""))
            .filter((id): id is string => !!id)
        : localLists.get(toolExpression.match(/(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)/)?.[1] ?? "") ?? [];
      const publication = toolJobPublicationContracts(source, block.body, localStrings);
      const exactPublication = !!publication && publication.size === ids.length && ids.every((toolId) => publication.has(toolId));
      for (const toolId of ids) {
        const publicationLanes = exactPublication ? publication!.get(toolId)! : [];
        rows.push({
          ownerFile: owner.file,
          factoryFile,
          ownerTypeName,
          factory,
          toolId,
          publicationLanes,
          publicationReady: exactPublication && toolJobPublicationAuthorityReady(owner.body, publicationLanes, files, owner.file),
        });
      }
    }
  }
  const unique = new Map<string, ToolJobAppOwnedRow>();
  for (const row of rows) unique.set(`${row.ownerFile}\0${row.toolId}`, row);
  return [...unique.values()].sort((left, right) => left.ownerFile.localeCompare(right.ownerFile) || left.toolId.localeCompare(right.toolId));
}

/** 🔒️ Reads exact explicit dispositions; constructors never count as proof. */
export function toolJobDispositions(files: ReadonlyMap<string, string>): Map<string, string> {
  const result = new Map<string, string>();
  const constants = new Map<string, string>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&str\s*=\s*"([^"]+)"/g)) constants.set(match[1]!, match[2]!);
  }
  const lists = new Map<string, string[]>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?\[\s*&(?:'static\s+)?str\s*\]\s*=\s*&\[([^\]]*)\]\s*;/gs)) {
      lists.set(match[1]!, [...match[2]!.matchAll(/"([^"]+)"|\b([A-Z][A-Z0-9_]*)\b/g)].map((item) => item[1] ?? constants.get(item[2]!)).filter((id): id is string => !!id));
    }
  }
  for (const [file, source] of files) {
    const explicit = /\.action_interactive_job\(\s*(?:"([^"]+)"|(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*))\s*,\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)\s*\)/g;
    let match: RegExpExecArray | null;
    while ((match = explicit.exec(source))) {
      const id = match[1] ?? constants.get(match[2] ?? "");
      if (id) result.set(`${file}\0${id}`, match[3]!);
    }
    const looped = /for\s+([a-z_][a-z0-9_]*)\s+in\s+&?(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)(?:\.iter\(\))?\s*\{[^{}]*?\.action_interactive_job\(\s*\*?\1\s*,\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)\s*\)/g;
    while ((match = looped.exec(source))) for (const id of lists.get(match[2]!) ?? []) result.set(`${file}\0${id}`, match[3]!);
    const assigned = new Set([...source.matchAll(/\.semantics\.execution\.interactive_job\s*=\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)\s*;/g)].map((entry) => entry[1]!));
    const bulk = source.match(/\.interactive_jobs\(\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)\s*\)/)?.[1] ?? (assigned.size === 1 ? [...assigned][0] : undefined);
    for (const factory of toolJobClassifiedFactories(source, constants, lists)) for (const id of factory.ids) if (!result.has(`${file}\0${id}`)) result.set(`${file}\0${id}`, factory.classification);
    if (bulk) for (const row of toolJobStaticRows(new Map([[file, source]]))) if (!result.has(`${file}\0${row.id}`)) result.set(`${file}\0${row.id}`, bulk);
  }
  return result;
}

export type ToolJobProof = {
  sourceFile: string;
  ownerFile: string;
  ownerTypeName: string;
  controllerId: string;
  factory: string;
  factoryType?: string;
  toolId: string;
  documentSchema: string;
  execution: "bounded" | "resumable";
  values: readonly number[];
};

/** 🧮️ Builds an evaluator for the integer constant expressions Rust execution contracts spell: decimal and hex literals with `_` and type suffixes, constant names (a module path reduces to its last segment), `as` casts, parentheses, `+ - * /` and `<<`. A name that is declared with two different values, an unknown name or a call or method access yields `NaN`, never a guess. */
export function toolJobConstantEvaluator(sources: Iterable<string>): (expression: string) => number {
  const items = new Map<string, Set<string>>();
  for (const source of sources) for (const match of source.matchAll(/\bconst\s+([A-Z][A-Z0-9_]*)\s*:\s*(?:usize|u8|u16|u32|u64|i32|i64)\s*=\s*([^;]+);/gu)) items.set(match[1]!, (items.get(match[1]!) ?? new Set()).add(match[2]!.trim()));
  const evaluate = (text: string, seen: readonly string[]): number => {
    const tokens = text.replace(/\s+as\s+[A-Za-z0-9_]+/gu, "").match(/0x[0-9a-fA-F_]+(?:u\d+|usize)?|\d[\d_]*(?:u\d+|usize|i\d+)?|[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*|<<|[-+*/()]|\S/gu) ?? [];
    let at = 0;
    const atom = (): number => {
      const token = tokens[at++];
      if (token === undefined) return Number.NaN;
      if (token === "(") { const inner = shift(); return tokens[at++] === ")" ? inner : Number.NaN; }
      if (/^0x/iu.test(token)) return Number.parseInt(token.slice(2).replace(/_/gu, "").replace(/(?:u\d+|usize)$/u, ""), 16);
      if (/^\d/u.test(token)) return Number(token.replace(/_/gu, "").replace(/(?:u\d+|usize|i\d+)$/u, ""));
      if (!/^[A-Za-z_]/u.test(token) || tokens[at] === "(" || tokens[at] === ".") return Number.NaN;
      const name = token.split("::").at(-1)!;
      const values = items.get(name);
      if (values === undefined || values.size !== 1 || seen.includes(name)) return Number.NaN;
      return evaluate([...values][0]!, [...seen, name]);
    };
    const product = (): number => { let value = atom(); while (tokens[at] === "*" || tokens[at] === "/") value = tokens[at++] === "*" ? value * atom() : Math.floor(value / atom()); return value; };
    const sum = (): number => { let value = product(); while (tokens[at] === "+" || tokens[at] === "-") value = tokens[at++] === "+" ? value + product() : value - product(); return value; };
    const shift = (): number => { let value = sum(); while (tokens[at] === "<<") { at += 1; value *= 2 ** sum(); } return value; };
    const result = shift();
    return at === tokens.length ? result : Number.NaN;
  };
  return (expression) => evaluate(expression, []);
}

/** ☎️ The text between the parenthesis that opens at `open` and its match, ignoring nested parentheses. */
function toolJobCallArguments(source: string, open: number): string | undefined {
  let depth = 0;
  for (let index = open; index < source.length; index += 1) {
    if (source[index] === "(") depth += 1;
    else if (source[index] === ")" && --depth === 0) return source.slice(open + 1, index);
  }
  return undefined;
}

/** ✂️ Splits call arguments at their top-level commas. */
function toolJobSplitArguments(text: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let start = 0;
  for (let index = 0; index < text.length; index += 1) {
    if ("([{".includes(text[index]!)) depth += 1;
    else if (")]}".includes(text[index]!)) depth -= 1;
    else if (text[index] === "," && depth === 0) { parts.push(text.slice(start, index)); start = index + 1; }
  }
  parts.push(text.slice(start));
  return parts.map((part) => part.trim()).filter((part) => part !== "");
}

/** 📐️ Reads only owner-local typed bounded-first-step proof catalogs. A contract is spelled inline (`ToolExecutionContract::bounded_first_step(..)`) or as a call of an owner-local `fn name() -> ToolExecutionContract` whose body is that inline form; its arguments are literals or integer constants, evaluated from the constants of the same sources. */
/** 📜️ Resolves static contracts through exact constants and side-effect-free zero-argument helpers. */
export function toolJobContractSource(source: string, expression: string): { execution: "bounded" | "resumable"; values: number[] } | undefined {
  const external = new Map([
    ["semio_framework::kernel::COMMAND_MAXIMUM_BYTES", ["🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs", "COMMAND_MAXIMUM_BYTES"]],
    ["semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES", ["🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs", "GUEST_HOST_ANSWER_CEILING_BYTES"]],
  ]);
  const number = (text: string, owner: string, seen: Set<string>): number | undefined => {
    const value = text.trim().replace(/\s+as\s+(?:u64|usize)$/, "");
    if (/^[0-9][0-9_]*$/.test(value)) { const result = Number(value.replaceAll("_", "")); return Number.isSafeInteger(result) ? result : undefined; }
    if (seen.has(value) || seen.size > 32) return undefined;
    const next = new Set(seen).add(value);
    const imported = external.get(value);
    if (imported) return number(imported[1]!, policyReadFileSafe(WORKSPACE_ROOT, imported[0]!), next);
    const work = value.match(/^([A-Z][A-Z0-9_]*)\.work_items\(\)$/);
    if (work) {
      const rows = [...owner.matchAll(new RegExp(`const\\s+${work[1]}\\s*:\\s*ArtifactRetainedWorkCapacity\\s*=\\s*ArtifactRetainedWorkCapacity::for_invertible_items\\(([^()]*)\\)\\s*;`, "g"))];
      const retained = policyReadFileSafe(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs").replace(/\s+/g, "");
      if (rows.length !== 1 || !retained.includes("pubconstfnwork_items(self)->usize{self.rows(self.invertible_items)}") || !retained.includes("items.saturating_mul(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)")) return undefined;
      const items = number(rows[0]![1]!, owner, next);
      const itemSource = policyReadFileSafe(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🦀️.rs");
      const multiplier = number("ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS", itemSource, next);
      const result = items === undefined || multiplier === undefined ? undefined : items * multiplier;
      return result !== undefined && Number.isSafeInteger(result) ? result : undefined;
    }
    if (!/^[A-Z][A-Z0-9_]*$/.test(value)) return undefined;
    const constants = [...owner.matchAll(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?const\\s+${value}\\s*:\\s*(?:usize|u64|u32)\\s*=\\s*([^;]+);`, "g"))];
    return constants.length === 1 ? number(constants[0]![1]!, owner, next) : undefined;
  };
  const resolve = (value: string, seen: Set<string>): { execution: "bounded" | "resumable"; values: number[] } | undefined => {
    const constructor = value.trim().match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^()]*)\)$/);
    if (constructor) {
      const values = constructor[2]!.split(",").filter((part) => part.trim()).map((part) => number(part, source, new Set()));
      return values.every((part) => part !== undefined) ? { execution: constructor[1] === "resumable" ? "resumable" : "bounded", values: values as number[] } : undefined;
    }
    const call = value.trim().match(/^([a-z_][a-z0-9_]*)\(\)$/);
    if (!call || seen.has(call[1]!) || seen.size > 32) return undefined;
    const declarations = [...source.matchAll(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?(?:const\\s+)?fn\\s+${call[1]}\\(\\)\\s*->\\s*ToolExecutionContract\\s*\\{`, "g"))];
    if (declarations.length !== 1) return undefined;
    const body = toolJobRustBlock(source, source.indexOf("{", declarations[0]!.index))?.body.trim().replace(/^return\s+/, "").replace(/;$/, "");
    return body ? resolve(body, new Set(seen).add(call[1]!)) : undefined;
  };
  return resolve(expression, new Set());
}

/** ✅️ Reads only owner-local typed bounded-first-step proof catalogs. */
export function toolJobProofs(files: ReadonlyMap<string, string>): ToolJobProof[] {
  const proofs: ToolJobProof[] = [];
  const evaluate = toolJobConstantEvaluator(files.values());
  const contractFunctions = new Map<string, { kind: "bounded_first_step" | "resumable"; values: readonly number[] }>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/\bfn\s+([a-z_][a-z0-9_]*)\s*\(\s*\)\s*->\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract\s*\{\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(/gu)) {
      const open = match.index + match[0].length - 1;
      const args = toolJobCallArguments(source, open);
      if (args !== undefined) contractFunctions.set(match[1]!, { kind: match[2] as "bounded_first_step" | "resumable", values: toolJobSplitArguments(args).map(evaluate) });
    }
  }
  const validValues = (values: readonly number[]) => (values.length === 5 || values.length === 7) && values.every((value) => Number.isNaN(value) || (Number.isFinite(value) && value > 0)) && Number.isFinite(values[4]) && values[4]! < 8_000;
  const resolve = (expression: string): { execution: "bounded" | "resumable"; values: readonly number[] } | undefined => {
    const inline = expression.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(/u);
    if (inline) {
      const args = toolJobCallArguments(expression, inline[0].length - 1);
      return args === undefined ? undefined : { execution: inline[1] === "resumable" ? "resumable" : "bounded", values: toolJobSplitArguments(args).map(evaluate) };
    }
    const named = expression.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([a-z_][a-z0-9_]*)\(\s*\)$/u);
    const function_ = named ? contractFunctions.get(named[1]!) : undefined;
    return function_ ? { execution: function_.kind === "resumable" ? "resumable" : "bounded", values: function_.values } : undefined;
  };
  for (const [sourceFile, source] of files) {
    const invocation = /(?:semio_framework_plugin::)?bounded_first_step_tool_proofs!\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = invocation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      const header = block.body.match(/owner:\s*([^,]+),\s*owner_file:\s*"([^"]+)",\s*controller:\s*"([^"]+)",\s*artifact_schema:\s*"([^"]+)",\s*factory:\s*"([^"]+)"/s);
      if (header) {
        const factoryType = block.body.match(/factory_type:\s*([^,]+),/)?.[1]?.replaceAll(/\s+/g, "");
        const common = { sourceFile, ownerFile: header[2]!, ownerTypeName: header[1]!.replaceAll(/\s+/g, ""), controllerId: header[3]!, documentSchema: header[4]!, factory: header[5]!, factoryType };
        const mappedRows = [...block.body.matchAll(/"([^"]+)"\s*=>\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*(?:ToolExecutionContract::(?:bounded_first_step|resumable)\([^)]*\)|[a-z_][a-z0-9_]*\(\s*\)))/g)];
        for (const row of mappedRows) {
          const inherited = resolve(row[2]!);
          const contract = inherited && inherited.values.every(Number.isFinite) ? inherited : toolJobContractSource(source, row[2]!) ?? inherited;
          if (contract && validValues(contract.values)) proofs.push({ ...common, toolId: row[1]!, execution: contract.execution, values: contract.values });
        }
        if (mappedRows.length === 0) {
          const sharedAt = block.body.search(/\bcontract:/);
          const shared = sharedAt < 0 ? undefined : toolJobSplitArguments(block.body.slice(sharedAt + "contract:".length))[0];
          const sharedTools = block.body.match(/tools:\s*\[([^\]]*)\]/s);
          const inherited = shared === undefined ? undefined : resolve(shared.trim());
          const contract = inherited && inherited.values.every(Number.isFinite) ? inherited : shared === undefined ? undefined : toolJobContractSource(source, shared.trim()) ?? inherited;
          if (sharedTools && contract && validValues(contract.values)) {
            for (const tool of sharedTools[1]!.matchAll(/"([^"]+)"/g)) proofs.push({ ...common, toolId: tool[1]!, execution: contract.execution, values: contract.values });
          }
        }
      }
      invocation.lastIndex = block.end;
    }
  }
  return proofs;
}

/** 🚧 Rejects resumable cursors that spend their yields only scanning inputs and then call the
 * original one-shot reducer. Such a route moves the latency spike but does not split it. */
export function toolJobScanThenMonolithRows(files: ReadonlyMap<string, string>, proofs: readonly ToolJobProof[]): { file: string; id: string; helper: string }[] {
  const rows: { file: string; id: string; helper: string }[] = [];
  for (const [file, source] of files) {
    const resumable = proofs.filter((proof) => proof.sourceFile === file && proof.execution === "resumable");
    if (resumable.length === 0) continue;
    const implementation = /impl(?:<[^>]+>)?\s+(?:semio_framework_plugin::retained_command::)?ArtifactCommandWork<[^{}]+>\s+for\s+[^\s<{]+(?:<[^>{]+>)?\s*\{/g;
    let match: RegExpExecArray | null;
    while ((match = implementation.exec(source))) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block) break;
      implementation.lastIndex = block.end;
      if (!block.body.includes("ArtifactCommandWorkStep::Progress") || !block.body.includes("command-scan")) continue;
      const called = [...block.body.matchAll(/\b([a-z_][a-z0-9_]*)\s*\(/g)].map((call) => call[1]!);
      const helper = called.find((name) => {
        const declaration = new RegExp(`(?:^|\\n)\\s*fn\\s+${name}\\s*\\(`, "m").exec(source);
        if (!declaration) return false;
        const open = source.indexOf("{", declaration.index);
        const body = open < 0 ? undefined : toolJobRustBlock(source, open);
        return !!body && /(?:command\s*)?\.dispatch\s*\(/.test(body.body);
      });
      if (!helper && !/(?:command\s*)?\.dispatch\s*\(/.test(block.body)) continue;
      for (const proof of resumable) rows.push({ file, id: proof.toolId, helper: helper ?? "inline-dispatch" });
      break;
    }
  }
  return rows.sort((left, right) => left.file.localeCompare(right.file) || left.id.localeCompare(right.id));
}

export function toolJobProofIdentity(proof: ToolJobProof): string {
  return `${proof.ownerFile}\0${proof.ownerTypeName}\0${proof.controllerId}\0${proof.factory}\0${proof.factoryType ?? ""}\0${proof.toolId}\0${proof.documentSchema}.tool-command.v1`;
}
//#endregion 🎯️ToolJobCoverage

/** 🗿️ An artifact crate's own lib root (`🗿️artifacts/<artifact>/🦀️.rs`, declared by that artifact's
 * `📦️packages/🦀️rust/Cargo.toml` `[lib] path = "../../🦀️.rs"`), which owns the `#[path]` module tree
 * since the plugin package root became a re-export shell. */
function toolJobArtifactCrateRoot(file: string): string | undefined {
  const match = file.match(/^(.*\/🗿️artifacts\/[^/]+)\/🦀️\.rs$/);
  return match?.[1];
}
/** 🧭️ Resolves one crate module through its package's explicit taxonomy declarations. */
export function toolJobFactoryModuleFile(files: ReadonlyMap<string, string>, ownerFile: string, modulePath: string): string | undefined {
  const candidates = new Set<string>();
  const visit = (source: string, directory: string, prefix: string): void => {
    const declaration = /#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])/g;
    let match: RegExpExecArray | null;
    while ((match = declaration.exec(source))) {
      const target = posix.normalize(posix.join(directory, match[1]!));
      const name = `${prefix}::${match[2]}`;
      if (match[3] === ";") {
        const exportedComponent = prefix === modulePath && match[2] === "component" && /pub\s+use\s+component::\*\s*;/.test(source);
        if ((name === modulePath || exportedComponent) && files.has(target)) candidates.add(target);
        else if (modulePath.startsWith(`${name}::`) && files.has(target)) visit(files.get(target)!, posix.dirname(target), name);
      } else {
        const block = toolJobRustBlock(source, source.indexOf("{", match.index));
        if (!block) return;
        visit(block.body, target, name);
        declaration.lastIndex = block.end;
      }
    }
  };
  for (const [file, source] of files) {
    if (!file.endsWith("/🦀️.rs")) continue;
    const packageBoundary = file.lastIndexOf("/📦️packages/");
    const owningRoot = packageBoundary >= 0 ? file.slice(0, packageBoundary) : toolJobArtifactCrateRoot(file);
    if (owningRoot === undefined || !ownerFile.startsWith(`${owningRoot}/`)) continue;
    visit(source, posix.dirname(file), "crate");
  }
  return candidates.size === 1 ? [...candidates][0] : undefined;
}
