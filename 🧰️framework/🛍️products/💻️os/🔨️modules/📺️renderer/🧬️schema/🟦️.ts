/** 📺️ `os.renderer` schema module — the renderer-owned presentation contracts and their runtime parsers.
 *
 * The JSON Schema document `./🔣️.json` is the authority; every exported type mirrors one of its
 * `$defs` entries and every `parse<Export>` validates against that entry with an owned draft-07
 * subset walker (no runtime dependency on an external validator).
 *
 * @see ./🔣️.json
 */
import document from "./🔣️.json" with { type: "json" };

export const RENDERER_SCHEMA = document as Readonly<Record<string, unknown>>;
export const RENDERER_SCHEMA_ID = document.$id;

type Node = Readonly<Record<string, unknown>>;

/** 🚨️ A structural refusal carrying the instance path and the keyword that refused it. */
export class RendererSchemaError extends Error {
  constructor(
    readonly path: string,
    readonly keyword: string,
    readonly detail: string,
  ) {
    super(`${path || "#"}: ${keyword} — ${detail}`);
    this.name = "RendererSchemaError";
  }
}

const defs = (document as { $defs: Record<string, Node> }).$defs;

function resolve(node: Node): Node {
  const ref = node.$ref;
  if (typeof ref !== "string") return node;
  const name = ref.startsWith("#/$defs/") ? ref.slice("#/$defs/".length) : "";
  const target = defs[name];
  if (!target) throw new RendererSchemaError("", "$ref", `unresolvable ${ref}`);
  return resolve(target);
}

function typeOf(value: unknown): string {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  if (typeof value === "number") return Number.isInteger(value) ? "integer" : "number";
  return typeof value;
}

function matchesType(value: unknown, declared: string): boolean {
  return declared === "number" ? typeOf(value) === "integer" || typeOf(value) === "number" : typeOf(value) === declared;
}

function accepts(node: Node, value: unknown, path: string): string | null {
  try {
    check(node, value, path);
    return null;
  } catch (error) {
    return error instanceof RendererSchemaError ? error.message : String(error);
  }
}

function check(raw: Node, value: unknown, path: string): void {
  const node = resolve(raw);
  const fail = (keyword: string, detail: string): never => {
    throw new RendererSchemaError(path, keyword, detail);
  };
  if (node.const !== undefined && JSON.stringify(node.const) !== JSON.stringify(value)) fail("const", `expected ${JSON.stringify(node.const)}`);
  if (Array.isArray(node.enum) && !node.enum.some((option) => JSON.stringify(option) === JSON.stringify(value))) fail("enum", `expected one of ${JSON.stringify(node.enum)}`);
  if (Array.isArray(node.oneOf)) {
    const matched = node.oneOf.filter((branch) => accepts(branch as Node, value, path) === null);
    if (matched.length !== 1) fail("oneOf", `matched ${matched.length} branches, expected exactly 1`);
  }
  if (Array.isArray(node.anyOf) && !node.anyOf.some((branch) => accepts(branch as Node, value, path) === null)) fail("anyOf", "matched no branch");
  const declared = node.type;
  if (typeof declared === "string" && !matchesType(value, declared)) fail("type", `expected ${declared}, got ${typeOf(value)}`);
  if (Array.isArray(declared) && !declared.some((option) => matchesType(value, String(option)))) fail("type", `expected one of ${declared.join("|")}, got ${typeOf(value)}`);
  if (typeof value === "string") {
    if (typeof node.minLength === "number" && [...value].length < node.minLength) fail("minLength", `shorter than ${node.minLength}`);
    if (typeof node.maxLength === "number" && [...value].length > node.maxLength) fail("maxLength", `longer than ${node.maxLength}`);
    if (typeof node.pattern === "string" && !new RegExp(node.pattern, "u").test(value)) fail("pattern", `does not match ${node.pattern}`);
  }
  if (typeof value === "number") {
    if (typeof node.minimum === "number" && value < node.minimum) fail("minimum", `below ${node.minimum}`);
    if (typeof node.maximum === "number" && value > node.maximum) fail("maximum", `above ${node.maximum}`);
  }
  if (Array.isArray(value)) {
    if (typeof node.minItems === "number" && value.length < node.minItems) fail("minItems", `fewer than ${node.minItems}`);
    if (typeof node.maxItems === "number" && value.length > node.maxItems) fail("maxItems", `more than ${node.maxItems}`);
    if (node.uniqueItems === true && new Set(value.map((item) => JSON.stringify(item))).size !== value.length) fail("uniqueItems", "contains duplicates");
    const items = node.items;
    if (Array.isArray(items)) {
      if (node.additionalItems === false && value.length > items.length) fail("additionalItems", `more than ${items.length} entries`);
      items.forEach((entry, index) => index < value.length && check(entry as Node, value[index], `${path}/${index}`));
    } else if (items && typeof items === "object") {
      value.forEach((entry, index) => check(items as Node, entry, `${path}/${index}`));
    }
  }
  if (value !== null && typeof value === "object" && !Array.isArray(value)) {
    const row = value as Record<string, unknown>;
    const properties = (node.properties ?? {}) as Record<string, Node>;
    for (const key of (node.required ?? []) as string[]) if (!(key in row)) fail("required", `missing ${key}`);
    if (node.additionalProperties === false) for (const key of Object.keys(row)) if (!(key in properties)) fail("additionalProperties", `unexpected ${key}`);
    for (const [key, entry] of Object.entries(properties)) if (key in row) check(entry, row[key], `${path}/${key}`);
  }
}

/** 🧪️ Validates `value` against the named export and returns it, or throws {@link RendererSchemaError}. */
export function parseRendererExport<T>(exportId: string, value: unknown): T {
  const node = defs[exportId];
  if (!node) throw new RendererSchemaError("", "$defs", `unknown export ${exportId}`);
  check(node, value, "");
  return value as T;
}

/** 🛡️ Reports whether `value` satisfies the named export without throwing. */
export function rendererExportAccepts(exportId: string, value: unknown): boolean {
  const node = defs[exportId];
  if (!node) throw new RendererSchemaError("", "$defs", `unknown export ${exportId}`);
  return accepts(node, value, "") === null;
}

/** 📇️ Every export id this module publishes. */
export function rendererExportIds(): readonly string[] {
  return Object.keys(defs);
}

export interface ShellScopeV1 { readonly spaceId: string; readonly documentId: string }
export interface ShellDialogDocumentV1 { readonly runtimeKey: string; readonly clientInstanceId: string; readonly scope: ShellScopeV1 | null }
export interface ShellDialogOriginV1 { readonly pluginId: string; readonly appId: string; readonly controllerId: string; readonly sessionInstanceId: number; readonly document: ShellDialogDocumentV1 | null }
export interface AdmittedShellInstanceTransitionV1 { readonly id: string; readonly before: boolean; readonly after: boolean; readonly creates: number; readonly retires: number; readonly accepted: boolean }
export type DocumentOpeningStepV1 = "socket" | "attach" | "close" | "detach" | "commit" | "retire";
export interface DocumentOpeningTransitionV1 { readonly id: string; readonly fail: "none" | "socket" | "attach" | "deadline"; readonly replace: "none" | "socket" | "attach"; readonly outcome: "failed" | "retired" | "ready"; readonly sequence: readonly DocumentOpeningStepV1[]; readonly timers: number }
export interface DocumentOpeningAdmissionV1 { readonly background: boolean; readonly runtimeKey: string; readonly instanceId: number; readonly admitted: boolean; readonly closed: readonly string[] }
export type DocumentOpeningBackgroundSequenceV1 = readonly ("create-1" | "create-2" | "create-failed" | "visit-1" | "visit-2" | "release-1" | "release-2")[];
export type DocumentOpeningAttachmentStepV1 = "attach-a" | "attached-a" | "detach" | "attach-b";
export interface DocumentOpeningCloseFailureV1 { readonly attached: boolean; readonly sequence: readonly ("socket" | "attach" | "close" | "detach" | "retire")[] }
export interface DocumentOpeningRefV1 { readonly documentId: string; readonly schema: string; readonly spaceId?: string }
export interface DocumentOpeningContextV1 { readonly currentSpaceId: string | null; readonly identity: { readonly hubBaseUrl: string } | null; readonly dataDir?: string; readonly surface?: string }
export type DocumentOpeningTargetV1 = { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly requestedSurfaceId: string } | { readonly kind: "folder"; readonly path: string };
export type DocumentOpeningErrorCodeV1 = "opening.identity-required" | "opening.surface-required";
export interface DocumentOpeningScopeResolutionV1 { readonly id: string; readonly ref: DocumentOpeningRefV1; readonly context: DocumentOpeningContextV1; readonly expected?: readonly DocumentOpeningTargetV1[]; readonly error?: DocumentOpeningErrorCodeV1 }
export type DocumentOpeningRequestStageV1 = "open-plan" | "manifest" | "component" | "descriptor" | "socket-grants";
export interface DocumentFirstOpenV1 { readonly requestedSurfaceOnly: boolean; readonly requestStages: readonly DocumentOpeningRequestStageV1[]; readonly socketCount: number; readonly localSocketFailures: number; readonly unselectedSocketFailures: number; readonly hostile: readonly { readonly id: string; readonly requestStages: readonly DocumentOpeningRequestStageV1[] }[] }
export type TutorialRunEventV1 = "resolve" | "stop" | "switch" | "replace";
export interface TutorialRunTransitionV1 { readonly id: string; readonly events: readonly TutorialRunEventV1[]; readonly started: boolean; readonly restores: number }
export type TutorialDriveEventV1 = "claimA" | "claimB" | "releaseA" | "releaseB" | "retire";
export interface TutorialDriveTransitionV1 { readonly id: string; readonly events: readonly TutorialDriveEventV1[]; readonly active: readonly boolean[] }
export interface PausedTutorialSeekTransitionV1 { readonly id: string; readonly playing: boolean; readonly requested: boolean; readonly interrupt: "none" | "pause" | "close" | "play"; readonly mutations: number; readonly playhead: number; readonly resumed: boolean }
export interface SerialTutorialDriveTransitionV1 { readonly id: string; readonly close: boolean; readonly replace: boolean; readonly writes: readonly ("M" | "inverse-M")[]; readonly cursor: number }
export interface HostAppAliasesV1 { readonly landingAppId: string; readonly hostAppId: string }
export interface HostAppV1 { readonly id: string; readonly role: "editor" | "viewer"; readonly dialect: { readonly artifactKind: string } }
export interface HostIdentityResolutionV1 { readonly aliases: HostAppAliasesV1; readonly apps: readonly HostAppV1[]; readonly expected: { readonly landingAppId: string; readonly hostAppId: string } }
export interface ArtifactBootstrapProgressV1 { readonly kind: "artifact-bootstrap-progress"; readonly documentId: string; readonly receivedBytes: number; readonly totalBytes: number; readonly receivedChunks: number; readonly totalChunks: number }
export interface ArtifactBootstrapFailedV1 { readonly kind: "artifact-bootstrap-failed"; readonly documentId: string; readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure"; readonly message: string; readonly retryable: boolean }
export interface ArtifactRebootstrapRequiredV1 { readonly kind: "artifact-rebootstrap-required"; readonly documentId: string; readonly message: string; readonly retryable: boolean }
export type BootstrapStatusV1 = ArtifactBootstrapProgressV1 | ArtifactBootstrapFailedV1 | ArtifactRebootstrapRequiredV1;
export interface LocalizedNoticeTextV1 { readonly en: string; readonly de: string }
export interface ExtensionInvocationRequestV1 { readonly capability: string; readonly request: Readonly<Record<string, unknown>> }
export interface ExtensionInvocationCompletionV1 { readonly surface: string; readonly revision: number; readonly notification: string; readonly uiScope: { readonly kind: "full" | "partial" }; readonly historyPatch: { readonly cursor: number; readonly upserts: readonly Readonly<Record<string, unknown>>[]; readonly canUndo: boolean; readonly canRedo: boolean } }
export interface ExtensionCompletionRefusalV1 { readonly kind: "missing-lease" | "missing-complete" | "missing-guard" | "foreign-request" | "foreign-instance"; readonly code: "extension.completion-unavailable" | "extension.completion-owner-mismatch" }
export interface ExtensionInvocationMissV1 { readonly phase: "extension" | "invoke"; readonly code: string }
export interface ExtensionInvocationFaultV1 { readonly origin: "extension" | "plugin" | "host"; readonly code: string; readonly severity: "error" | "warning"; readonly message: string; readonly scope: Readonly<Record<string, unknown>>; readonly retryable: boolean }
export interface MountedGisMapProbeSourceV1 { readonly scope: ShellScopeV1; readonly clientInstanceId: string; readonly activationGeneration: string; readonly catalogGenerationId: string; readonly componentSha256: string; readonly descriptorSha256: string; readonly browserActorSha256: string; readonly uiRevision: number; readonly verifiedSurfaceId: string; readonly surface: string; readonly regions: readonly { readonly id: string }[] }
export interface MountedGisMapProbeV1 { readonly scope: ShellScopeV1; readonly clientInstanceId: string; readonly activationGeneration: string; readonly catalogGenerationId: string; readonly componentSha256: string; readonly descriptorSha256: string; readonly browserActorSha256: string; readonly uiRevision: number; readonly rootKind: "tiled-map"; readonly regionIds: readonly string[] }
export type MountedGisMapProbeRefusalV1 = "unacknowledged-identity" | "stale-ui-revision" | "non-map-root" | "malformed-map-pack" | "duplicate-region-id";

export const parseShellDialogOriginV1 = (value: unknown): ShellDialogOriginV1 => parseRendererExport("ShellDialogOriginV1", value);
export const parseAdmittedShellInstanceTransitionV1 = (value: unknown): AdmittedShellInstanceTransitionV1 => parseRendererExport("AdmittedShellInstanceTransitionV1", value);
export const parseDocumentOpeningTransitionV1 = (value: unknown): DocumentOpeningTransitionV1 => parseRendererExport("DocumentOpeningTransitionV1", value);
export const parseDocumentOpeningAdmissionV1 = (value: unknown): DocumentOpeningAdmissionV1 => parseRendererExport("DocumentOpeningAdmissionV1", value);
export const parseDocumentOpeningBackgroundSequenceV1 = (value: unknown): DocumentOpeningBackgroundSequenceV1 => parseRendererExport("DocumentOpeningBackgroundSequenceV1", value);
export const parseDocumentOpeningCloseFailureV1 = (value: unknown): DocumentOpeningCloseFailureV1 => parseRendererExport("DocumentOpeningCloseFailureV1", value);
export const parseDocumentOpeningScopeResolutionV1 = (value: unknown): DocumentOpeningScopeResolutionV1 => parseRendererExport("DocumentOpeningScopeResolutionV1", value);
export const parseDocumentFirstOpenV1 = (value: unknown): DocumentFirstOpenV1 => parseRendererExport("DocumentFirstOpenV1", value);
export const parseTutorialRunTransitionV1 = (value: unknown): TutorialRunTransitionV1 => parseRendererExport("TutorialRunTransitionV1", value);
export const parseTutorialDriveTransitionV1 = (value: unknown): TutorialDriveTransitionV1 => parseRendererExport("TutorialDriveTransitionV1", value);
export const parsePausedTutorialSeekTransitionV1 = (value: unknown): PausedTutorialSeekTransitionV1 => parseRendererExport("PausedTutorialSeekTransitionV1", value);
export const parseSerialTutorialDriveTransitionV1 = (value: unknown): SerialTutorialDriveTransitionV1 => parseRendererExport("SerialTutorialDriveTransitionV1", value);
export const parseHostIdentityResolutionV1 = (value: unknown): HostIdentityResolutionV1 => parseRendererExport("HostIdentityResolutionV1", value);
export const parseBootstrapStatusV1 = (value: unknown): BootstrapStatusV1 => parseRendererExport("BootstrapStatusV1", value);
export const parseLocalizedNoticeTextV1 = (value: unknown): LocalizedNoticeTextV1 => parseRendererExport("LocalizedNoticeTextV1", value);
export const parseExtensionInvocationRequestV1 = (value: unknown): ExtensionInvocationRequestV1 => parseRendererExport("ExtensionInvocationRequestV1", value);
export const parseExtensionInvocationCompletionV1 = (value: unknown): ExtensionInvocationCompletionV1 => parseRendererExport("ExtensionInvocationCompletionV1", value);
export const parseExtensionCompletionRefusalV1 = (value: unknown): ExtensionCompletionRefusalV1 => parseRendererExport("ExtensionCompletionRefusalV1", value);
export const parseExtensionInvocationMissV1 = (value: unknown): ExtensionInvocationMissV1 => parseRendererExport("ExtensionInvocationMissV1", value);
export const parseExtensionInvocationFaultV1 = (value: unknown): ExtensionInvocationFaultV1 => parseRendererExport("ExtensionInvocationFaultV1", value);
export const parseMountedGisMapProbeSourceV1 = (value: unknown): MountedGisMapProbeSourceV1 => parseRendererExport("MountedGisMapProbeSourceV1", value);
export const parseMountedGisMapProbeV1 = (value: unknown): MountedGisMapProbeV1 => parseRendererExport("MountedGisMapProbeV1", value);
