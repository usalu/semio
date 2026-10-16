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

type ToolJobPublicationLane = "HostOnly" | "Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child";
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

/** 🧹️ Removes in-file test modules from the production registration census. */
export function toolJobProductionSource(source: string): string {
  const lines = source.split("\n");
  for (const span of policyTestModSpans(lines)) for (let line = span.startLine - 1; line < span.endLine; line += 1) lines[line] = "";
  return lines.join("\n");
}

export function toolJobProductionProtocolSource(raw: string): string {
  let source = toolJobProductionSource(raw);
  const marker = /#\[cfg\((?:all\([^\]]*\btest\b[^\]]*\)|test)\)\]/g;
  for (;;) {
    const match = marker.exec(source);
    if (!match) return source;
    const item = match.index + match[0].length;
    let parentheses = 0;
    let brackets = 0;
    let end = source.length;
    for (let index = item; index < source.length; index += 1) {
      const char = source[index]!;
      if (char === "(") parentheses += 1;
      else if (char === ")") parentheses -= 1;
      else if (char === "[") brackets += 1;
      else if (char === "]") brackets -= 1;
      else if (char === ";" && parentheses === 0 && brackets === 0) {
        end = index + 1;
        break;
      } else if (char === "{" && parentheses === 0 && brackets === 0) {
        end = toolJobRustBlock(source, index)?.end ?? source.length;
        break;
      }
    }
    source = `${source.slice(0, match.index)}${" ".repeat(end - match.index)}${source.slice(end)}`;
    marker.lastIndex = 0;
  }
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

/** 🎯️ Enumerates macro and ordinary builder registrations without trusting a manifest disposition. */
export function toolJobStaticRows(files: ReadonlyMap<string, string>): ToolJobStaticRow[] {
  const rows: ToolJobStaticRow[] = [];
  const constants = new Map<string, string>();
  for (const source of files.values()) {
    for (const match of source.matchAll(/(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*"([^"]+)"/g)) constants.set(match[1]!, match[2]!);
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
  const expression = factoryBody.match(/const\s+PUBLICATION_CONTRACTS\s*:\s*&'static\s*\[\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\]\s*=\s*([^;]+);/s)?.[1]?.trim();
  if (!expression) return undefined;
  const constant = expression.match(/^(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*)$/)?.[1];
  const declaration = constant ? source.search(new RegExp(`(?:pub(?:\\([^)]*\\))?\\s+)?const\\s+${constant}\\s*:\\s*&(?:'static\\s+)?\\[\\s*(?:semio_framework_plugin::)?ArtifactToolPublicationContract\\s*\\]\\s*=`)) : -1;
  const open = declaration < 0 ? -1 : source.indexOf("[", source.indexOf("=", declaration));
  const array = open < 0 ? undefined : toolJobRustArray(source, open);
  const contracts = constant ? array?.body : expression;
  if (!contracts) return undefined;
  const result = new Map<string, ToolJobPublicationLane[]>();
  const row = /(?:semio_framework_plugin::)?ArtifactToolPublicationContract\s*\{\s*tool_id:\s*(?:"([^"]+)"|(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*))\s*,\s*lanes:\s*&\[([^\]]*)\]\s*\}/g;
  let match: RegExpExecArray | null;
  while ((match = row.exec(contracts))) {
    const toolId = match[1] ?? strings.get(match[2] ?? "");
    const lanes = [...match[3]!.matchAll(/(?:semio_framework_plugin::)?ArtifactToolPublicationLane::(HostOnly|Artifact|Config|Draft|Presence|Transient|Child)/g)].map((lane) => lane[1] as ToolJobPublicationLane);
    if (!toolId || result.has(toolId) || lanes.length === 0 || new Set(lanes).size !== lanes.length || (lanes.includes("HostOnly") && lanes.length !== 1)) return undefined;
    result.set(toolId, lanes);
  }
  return result.size > 0 ? result : undefined;
}

export function toolJobPublicationAuthorityReady(ownerBody: string, lanes: readonly ToolJobPublicationLane[]): boolean {
  const factory = (name: string) => {
    const start = ownerBody.search(new RegExp(`fn\\s+${name}\\s*\\(`));
    const open = start < 0 ? -1 : ownerBody.indexOf("{", start);
    const body = open < 0 ? undefined : toolJobRustBlock(ownerBody, open);
    return !!body && /\bSome\s*\(/.test(body.body);
  };
  return lanes.every((lane) => {
    if (lane === "HostOnly") return true;
    if (lane === "Child") return false;
    if (lane === "Artifact") return factory("build_artifact_store_one_item_preparation_factory");
    if (lane === "Config") return factory("build_config_store_one_item_preparation_factory");
    if (lane === "Draft") return factory("build_draft_store_one_item_preparation_factory");
    if (lane === "Presence") return factory("build_presence_store_one_item_preparation_factory") && factory("build_presence_local_root_retirement_factory");
    return factory("build_transient_store_one_item_preparation_factory") && factory("build_transient_local_root_retirement_factory");
  });
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
    const owner = /impl\s+(?:semio_framework_plugin::)?(ArtifactEditor|ArtifactApp)\s+for\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g;
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
      const ownerExpression = block.body.match(/type\s+Owner\s*=\s*([^;]+)\s*;/)?.[1]?.replaceAll(/\s+/g, "");
      const editorOwner = ownerExpression?.match(/^(?:semio_framework_plugin::)?EditorApp<([A-Za-z_][A-Za-z0-9_]*)>$/)?.[1];
      const directOwner = editorOwner ? undefined : ownerExpression?.match(/^(?:semio_framework_plugin::)?([A-Za-z_][A-Za-z0-9_]*)$/)?.[1];
      const ownerTypeName = editorOwner ?? directOwner;
      const toolExpression = block.body.match(/const\s+TOOL_IDS\s*:\s*&'static\s*\[\s*&'static\s+str\s*\]\s*=\s*([^;]+);/s)?.[1]?.trim();
      const owner = editorOwner ? ownerFiles.get(`ArtifactEditor\0${editorOwner}`) : directOwner ? ownerFiles.get(`ArtifactApp\0${directOwner}`) : undefined;
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
          publicationReady: exactPublication && toolJobPublicationAuthorityReady(owner.body, publicationLanes),
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
  for (const [file, source] of files) {
    const explicit = /\.action_interactive_job\(\s*(?:"([^"]+)"|(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Z][A-Z0-9_]*))\s*,\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite|ForbiddenFromUi|Deleted)\s*\)/g;
    let match: RegExpExecArray | null;
    while ((match = explicit.exec(source))) {
      const id = match[1] ?? constants.get(match[2] ?? "");
      if (id) result.set(`${file}\0${id}`, match[3]!);
    }
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

/** 📐️ Reads only owner-local typed bounded-first-step proof catalogs. */
export function toolJobProofs(files: ReadonlyMap<string, string>): ToolJobProof[] {
  const proofs: ToolJobProof[] = [];
  const valuesOf = (source: string): number[] => source.split(",").map((value) => Number(value.trim().replaceAll("_", "")));
  const validValues = (values: readonly number[]) => (values.length === 5 || values.length === 7) && values.every((value) => Number.isFinite(value) && value > 0) && values[4]! < 8_000;
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
        const mappedRows = [...block.body.matchAll(/"([^"]+)"\s*=>\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^)]*)\)/g)];
        for (const row of mappedRows) {
          const values = valuesOf(row[3]!);
          if (validValues(values)) proofs.push({ ...common, toolId: row[1]!, execution: row[2] === "resumable" ? "resumable" : "bounded", values });
        }
        if (mappedRows.length === 0) {
          const sharedContract = block.body.match(/contract:\s*(?:[A-Za-z_][A-Za-z0-9_]*::)*ToolExecutionContract::(bounded_first_step|resumable)\(([^)]*)\)/);
          const sharedTools = block.body.match(/tools:\s*\[([^\]]*)\]/s);
          const values = valuesOf(sharedContract?.[2] ?? "");
          if (sharedTools && validValues(values)) {
            for (const tool of sharedTools[1]!.matchAll(/"([^"]+)"/g)) proofs.push({ ...common, toolId: tool[1]!, execution: sharedContract?.[1] === "resumable" ? "resumable" : "bounded", values });
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
