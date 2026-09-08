/** 🧬️ `os.shell` schema module, TypeScript face. `🔣️.json` (draft-07) is the authority; this file
 * is its hand-written consumer: it re-exports the owned type mirror the Rust registry in `🦀️.rs`
 * renders into `../🤖️generated/🟦️.ts`, and adds one `parse<ExportId>()` per `$defs` export that
 * validates an untrusted value against that very document at runtime.
 *
 * The validator below covers exactly the draft-07 vocabulary `🔣️.json` uses (`$ref`, `type`,
 * `enum`, `const`, `properties`, `required`, `additionalProperties`, `items`, `anyOf`, `oneOf`,
 * `allOf`, `minimum`, `maximum`) — no external JSON Schema library is pulled into the runtime, per
 * CLAUDE.md. `bun nx run @semio-tech/framework-os-shell-rs:schema-check` is what keeps this file's
 * export set, `🔣️.json`'s `$defs` and the Rust registry in lockstep.
 *
 * @see ./🔣️.json
 * @see ./🦀️.rs
 */
import schemaDocument from "./🔣️.json" with { type: "json" };

export * from "../🤖️generated/🟦️.js";
import type { ActiveSession, Anchor, AppRole, ArtifactSyncStatus, ByAnchor, Conflict, DialogState, DockUiState, ExtraWindowInstance, IconName, InferencePortPhase, InferencePortStatus, LayoutNode, LoadedPlugin, MergePolicy, NoticeKind, PluginPanelStatus, PluginSupervisorState, ShellCapability, ShellCommand, ShellError, ShellEvent, ShellScope, ShellState, SplitOrientation, SyncCardKind, TransientNotice, UiAppearance, UiChromeLayout, UiDriver, UiLocale, UiTheme } from "../🤖️generated/🟦️.js";

//#region 🪪️Authority
/** 🪪️ The `$id` `🔣️.json` declares — the identity every consumer resolves `#/$defs/<ExportId>` against. */
export const OS_SHELL_SCHEMA_ID = "https://semio.tech/schema/os/shell/component.json";

/** 🧬️ The draft-07 authority document itself, for consumers that want to feed it to a validator. */
export const osShellSchemaDocument: JsonSchemaDocument = schemaDocument as unknown as JsonSchemaDocument;

/** 🧾️ Every `$defs` export id, in document order. */
export const OS_SHELL_SCHEMA_EXPORT_IDS: readonly string[] = Object.keys(osShellSchemaDocument.$defs);

interface JsonSchemaDocument {
  $schema: string;
  $id: string;
  $defs: Record<string, JsonSchemaNode>;
}

interface JsonSchemaNode {
  [keyword: string]: unknown;
}
//#endregion 🪪️Authority

//#region 🔍️Validator
/** 🚨️ Raised by every `parse<ExportId>()` when a value does not satisfy its `$defs` export. */
export class ShellSchemaError extends Error {
  readonly exportId: string;
  readonly problems: readonly string[];

  constructor(exportId: string, problems: readonly string[]) {
    super(`${exportId} does not satisfy ${OS_SHELL_SCHEMA_ID}#/$defs/${exportId}: ${problems.join("; ")}`);
    this.name = "ShellSchemaError";
    this.exportId = exportId;
    this.problems = problems;
  }
}

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

const matchesType = (type: string, value: unknown): boolean => {
  switch (type) {
    case "string":
      return typeof value === "string";
    case "number":
      return typeof value === "number" && Number.isFinite(value);
    case "integer":
      return typeof value === "number" && Number.isInteger(value);
    case "boolean":
      return typeof value === "boolean";
    case "null":
      return value === null;
    case "array":
      return Array.isArray(value);
    case "object":
      return isRecord(value);
    default:
      return false;
  }
};

const dereference = (node: JsonSchemaNode): JsonSchemaNode => {
  const reference = node.$ref;
  if (typeof reference !== "string") return node;
  const exportId = reference.slice("#/$defs/".length);
  const target = osShellSchemaDocument.$defs[exportId];
  if (target === undefined) throw new Error(`${OS_SHELL_SCHEMA_ID} has no $defs entry for '${exportId}'`);
  return target;
};

const collect = (schema: JsonSchemaNode, value: unknown, path: string): string[] => {
  const problems: string[] = [];
  inspect(schema, value, path, problems);
  return problems;
};

function inspect(schema: JsonSchemaNode, value: unknown, path: string, problems: string[]): void {
  const node = dereference(schema);
  const at = path === "" ? "value" : path;

  if ("const" in node && value !== node.const) problems.push(`${at} must equal ${JSON.stringify(node.const)}`);
  if (Array.isArray(node.enum) && !node.enum.includes(value)) problems.push(`${at} must be one of ${JSON.stringify(node.enum)}`);
  if (node.type !== undefined) {
    const types = Array.isArray(node.type) ? (node.type as string[]) : [node.type as string];
    if (!types.some((type) => matchesType(type, value))) problems.push(`${at} must be ${types.join(" | ")}`);
  }
  if (typeof node.minimum === "number" && typeof value === "number" && value < node.minimum) problems.push(`${at} must be >= ${node.minimum}`);
  if (typeof node.maximum === "number" && typeof value === "number" && value > node.maximum) problems.push(`${at} must be <= ${node.maximum}`);
  if (Array.isArray(node.allOf)) for (const branch of node.allOf as JsonSchemaNode[]) inspect(branch, value, path, problems);
  if (Array.isArray(node.anyOf) && !(node.anyOf as JsonSchemaNode[]).some((branch) => collect(branch, value, path).length === 0)) problems.push(`${at} matches no anyOf branch`);
  if (Array.isArray(node.oneOf)) {
    const matched = (node.oneOf as JsonSchemaNode[]).filter((branch) => collect(branch, value, path).length === 0).length;
    if (matched !== 1) problems.push(`${at} matches ${matched} oneOf branches, expected exactly 1`);
  }
  if (isRecord(value)) {
    const properties = isRecord(node.properties) ? (node.properties as Record<string, JsonSchemaNode>) : undefined;
    const required = Array.isArray(node.required) ? (node.required as string[]) : [];
    for (const key of required) if (!Object.prototype.hasOwnProperty.call(value, key)) problems.push(`${at} is missing required property '${key}'`);
    for (const [key, entry] of Object.entries(value)) {
      const child = properties?.[key];
      if (child !== undefined) inspect(child, entry, `${at}.${key}`, problems);
      else if (node.additionalProperties === false) problems.push(`${at} has unexpected property '${key}'`);
      else if (isRecord(node.additionalProperties)) inspect(node.additionalProperties as JsonSchemaNode, entry, `${at}.${key}`, problems);
    }
  }
  if (Array.isArray(value) && isRecord(node.items)) value.forEach((entry, index) => inspect(node.items as JsonSchemaNode, entry, `${at}[${index}]`, problems));
}

/** 🏭️ Builds the `parse<ExportId>()` for one `$defs` export — validate, then narrow. */
const defineParser =
  <T,>(exportId: string) =>
  (value: unknown): T => {
    const schema = osShellSchemaDocument.$defs[exportId];
    if (schema === undefined) throw new Error(`${OS_SHELL_SCHEMA_ID} has no $defs entry for '${exportId}'`);
    const problems = collect(schema, value, "");
    if (problems.length > 0) throw new ShellSchemaError(exportId, problems);
    return value as T;
  };
//#endregion 🔍️Validator

//#region 🧬️Parsers
/** 🧬️ Validates an untrusted value against `#/$defs/ActiveSession`. */
export const parseActiveSession = defineParser<ActiveSession>("ActiveSession");

/** 🧬️ Validates an untrusted value against `#/$defs/Anchor`. */
export const parseAnchor = defineParser<Anchor>("Anchor");

/** 🧬️ Validates an untrusted value against `#/$defs/AppRole`. */
export const parseAppRole = defineParser<AppRole>("AppRole");

/** 🧬️ Validates an untrusted value against `#/$defs/ArtifactSyncStatus`. */
export const parseArtifactSyncStatus = defineParser<ArtifactSyncStatus>("ArtifactSyncStatus");

/** 🧬️ Validates an untrusted value against `#/$defs/ByAnchor`. */
export const parseByAnchor = defineParser<ByAnchor<unknown>>("ByAnchor");

/** 🧬️ Validates an untrusted value against `#/$defs/Conflict`. */
export const parseConflict = defineParser<Conflict>("Conflict");

/** 🧬️ Validates an untrusted value against `#/$defs/DialogState`. */
export const parseDialogState = defineParser<DialogState>("DialogState");

/** 🧬️ Validates an untrusted value against `#/$defs/DockUiState`. */
export const parseDockUiState = defineParser<DockUiState>("DockUiState");

/** 🧬️ Validates an untrusted value against `#/$defs/ExtraWindowInstance`. */
export const parseExtraWindowInstance = defineParser<ExtraWindowInstance>("ExtraWindowInstance");

/** 🧬️ Validates an untrusted value against `#/$defs/IconName`. */
export const parseIconName = defineParser<IconName>("IconName");

/** 🧬️ Validates an untrusted value against `#/$defs/InferencePortPhase`. */
export const parseInferencePortPhase = defineParser<InferencePortPhase>("InferencePortPhase");

/** 🧬️ Validates an untrusted value against `#/$defs/InferencePortStatus`. */
export const parseInferencePortStatus = defineParser<InferencePortStatus>("InferencePortStatus");

/** 🧬️ Validates an untrusted value against `#/$defs/LayoutNode`. */
export const parseLayoutNode = defineParser<LayoutNode>("LayoutNode");

/** 🧬️ Validates an untrusted value against `#/$defs/LoadedPlugin`. */
export const parseLoadedPlugin = defineParser<LoadedPlugin>("LoadedPlugin");

/** 🧬️ Validates an untrusted value against `#/$defs/MergePolicy`. */
export const parseMergePolicy = defineParser<MergePolicy>("MergePolicy");

/** 🧬️ Validates an untrusted value against `#/$defs/NoticeKind`. */
export const parseNoticeKind = defineParser<NoticeKind>("NoticeKind");

/** 🧬️ Validates an untrusted value against `#/$defs/PluginPanelStatus`. */
export const parsePluginPanelStatus = defineParser<PluginPanelStatus>("PluginPanelStatus");

/** 🧬️ Validates an untrusted value against `#/$defs/PluginSupervisorState`. */
export const parsePluginSupervisorState = defineParser<PluginSupervisorState>("PluginSupervisorState");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellCapability`. */
export const parseShellCapability = defineParser<ShellCapability>("ShellCapability");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellCommand`. */
export const parseShellCommand = defineParser<ShellCommand>("ShellCommand");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellError`. */
export const parseShellError = defineParser<ShellError>("ShellError");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellEvent`. */
export const parseShellEvent = defineParser<ShellEvent>("ShellEvent");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellScope`. */
export const parseShellScope = defineParser<ShellScope>("ShellScope");

/** 🧬️ Validates an untrusted value against `#/$defs/ShellState`. */
export const parseShellState = defineParser<ShellState>("ShellState");

/** 🧬️ Validates an untrusted value against `#/$defs/SplitOrientation`. */
export const parseSplitOrientation = defineParser<SplitOrientation>("SplitOrientation");

/** 🧬️ Validates an untrusted value against `#/$defs/SyncCardKind`. */
export const parseSyncCardKind = defineParser<SyncCardKind>("SyncCardKind");

/** 🧬️ Validates an untrusted value against `#/$defs/TransientNotice`. */
export const parseTransientNotice = defineParser<TransientNotice>("TransientNotice");

/** 🧬️ Validates an untrusted value against `#/$defs/UiAppearance`. */
export const parseUiAppearance = defineParser<UiAppearance>("UiAppearance");

/** 🧬️ Validates an untrusted value against `#/$defs/UiChromeLayout`. */
export const parseUiChromeLayout = defineParser<UiChromeLayout>("UiChromeLayout");

/** 🧬️ Validates an untrusted value against `#/$defs/UiDriver`. */
export const parseUiDriver = defineParser<UiDriver>("UiDriver");

/** 🧬️ Validates an untrusted value against `#/$defs/UiLocale`. */
export const parseUiLocale = defineParser<UiLocale>("UiLocale");

/** 🧬️ Validates an untrusted value against `#/$defs/UiTheme`. */
export const parseUiTheme = defineParser<UiTheme>("UiTheme");
//#endregion 🧬️Parsers

//#region 🧪️tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("os.shell schema module", () => {
    it("declares draft-07 and the owned $id", () => {
      expect(osShellSchemaDocument.$schema).toBe("http://json-schema.org/draft-07/schema#");
      expect(osShellSchemaDocument.$id).toBe(OS_SHELL_SCHEMA_ID);
    });

    it("has one $defs export per exported type of the rendered mirror", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const mirror = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "..", "🤖️generated", "🟦️.ts"), "utf8");
      const rendered = [...mirror.matchAll(/^export type ([A-Za-z0-9_]+)/gmu)].map((match) => match[1]).sort();
      expect([...OS_SHELL_SCHEMA_EXPORT_IDS].sort()).toEqual(rendered);
    });

    it("accepts every committed fixture's state, command and expectation", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const fixturesDir = join(dirname(fileURLToPath(import.meta.url)), "..", "🧫️fixtures");
      const files = readdirSync(fixturesDir).filter((name) => name.endsWith(".json"));
      expect(files.length).toBeGreaterThan(0);
      for (const file of files) {
        const fixture = JSON.parse(readFileSync(join(fixturesDir, file), "utf8")) as { state: unknown; command: unknown; expected: { state?: unknown; events?: unknown[]; error?: unknown } };
        expect(() => parseShellState(fixture.state), file).not.toThrow();
        expect(() => parseShellCommand(fixture.command), file).not.toThrow();
        if (fixture.expected.state !== undefined) expect(() => parseShellState(fixture.expected.state), file).not.toThrow();
        for (const event of fixture.expected.events ?? []) expect(() => parseShellEvent(event), file).not.toThrow();
        if (fixture.expected.error !== undefined) expect(() => parseShellError(fixture.expected.error), file).not.toThrow();
      }
    });

    it("rejects values outside the declared value space", () => {
      expect(() => parseAnchor("middle")).toThrow(ShellSchemaError);
      expect(() => parseUiLocale("fr")).toThrow(ShellSchemaError);
      expect(parseUiLocale("de")).toBe("de");
      expect(() => parseLoadedPlugin({ pluginId: "p", moduleUrl: "u" })).toThrow(/missing required property 'label'/u);
      expect(() => parseLoadedPlugin({ pluginId: "p", moduleUrl: "u", label: null, extra: 1 })).toThrow(/unexpected property 'extra'/u);
      expect(() => parseLayoutNode({ kind: "split", orientation: "sideways", children: [], sizes: [] })).toThrow(ShellSchemaError);
      expect(parseLayoutNode({ kind: "split", orientation: "horizontal", children: [{ kind: "leaf", windowId: "w1" }], sizes: [1] })).toEqual({ kind: "split", orientation: "horizontal", children: [{ kind: "leaf", windowId: "w1" }], sizes: [1] });
      expect(() => parseShellCommand({ type: "setPanelVisible", anchor: "left" })).toThrow(ShellSchemaError);
      expect(() => parseByAnchor({ left: 1, right: 2, top: 3 })).toThrow(ShellSchemaError);
    });
  });
}
//#endregion 🧪️tests
