/** 🧬️ `@generated` — do NOT edit. Regenerate with
 * `bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`, which runs `semio-os-mcp schemas`
 * over `🌉️mcp/🧬️schema/🦀️.rs`'s `schemas()` — the single registry every `os.mcp` schema lives in.
 *
 * This file validates against the committed `./🔣️.json` with its own walker rather than an
 * external validator: a `🔨️modules/*` component file must not take a runtime dependency on a
 * package (CLAUDE.md, "no runtime dependencies on external libraries"). The third-party
 * cross-check lives in `📦️packages/🟦️typescript`, which validates the SAME document with AJV. */
import { readFileSync } from "node:fs";

//#region 🔖️Document
export type JsonValue = null | boolean | number | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };

type SchemaNode = boolean | { readonly [keyword: string]: JsonValue };

/** 📄️ The committed draft-07 document, read once. */
export const OS_MCP_SCHEMA_DOCUMENT = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { readonly $id: string; readonly $defs: Record<string, SchemaNode> };

/** 🆔️ `$id` of the document above — the base every `#/$defs/<ExportId>` pointer resolves against. */
export const OS_MCP_SCHEMA_ID = OS_MCP_SCHEMA_DOCUMENT.$id;

/** 🔍️ One named export's raw schema node, for a consumer that wants to hand it to its own validator. */
export function osMcpSchema(exportId: string): SchemaNode {
  const node = OS_MCP_SCHEMA_DOCUMENT.$defs[exportId];
  if (node === undefined) throw new Error(`os.mcp publishes no ${exportId} schema`);
  return node;
}
//#endregion 🔖️Document

//#region 🔖️Walker
const typeOf = (value: unknown): string => (value === null ? "null" : Array.isArray(value) ? "array" : Number.isInteger(value) ? "integer" : typeof value);

const sameJson = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);

function violations(node: SchemaNode, value: unknown, path: string): string[] {
  if (node === true) return [];
  if (node === false) return [`${path}: no value is valid here`];
  const found: string[] = [];
  const reference = node.$ref;
  if (typeof reference === "string") return violations(osMcpSchema(reference.slice("#/$defs/".length)), value, path);
  const actual = typeOf(value);
  const declared = node.type;
  const admits = (candidate: JsonValue): boolean => candidate === actual || (candidate === "number" && actual === "integer");
  if (typeof declared === "string" && !admits(declared)) found.push(`${path}: expected ${declared}, found ${actual}`);
  if (Array.isArray(declared) && !declared.some(admits)) found.push(`${path}: expected one of ${declared.join("|")}, found ${actual}`);
  if ("const" in node && !sameJson(node.const, value)) found.push(`${path}: must equal ${JSON.stringify(node.const)}`);
  if (Array.isArray(node.enum) && !node.enum.some((member) => sameJson(member, value))) found.push(`${path}: not one of ${JSON.stringify(node.enum)}`);
  if (Array.isArray(node.anyOf) && !node.anyOf.some((branch) => violations(branch as SchemaNode, value, path).length === 0)) found.push(`${path}: matched no anyOf branch`);
  if (Array.isArray(node.oneOf) && (node.oneOf as SchemaNode[]).filter((branch) => violations(branch, value, path).length === 0).length !== 1) found.push(`${path}: must match exactly one oneOf branch`);
  if (Array.isArray(node.allOf)) for (const branch of node.allOf as SchemaNode[]) found.push(...violations(branch, value, path));
  if (node.not !== undefined && violations(node.not as SchemaNode, value, path).length === 0) found.push(`${path}: matched a forbidden schema`);
  if (typeof value === "string") {
    if (typeof node.minLength === "number" && value.length < node.minLength) found.push(`${path}: shorter than ${node.minLength}`);
    if (typeof node.maxLength === "number" && value.length > node.maxLength) found.push(`${path}: longer than ${node.maxLength}`);
    if (typeof node.pattern === "string" && !new RegExp(node.pattern, "u").test(value)) found.push(`${path}: does not match ${node.pattern}`);
  }
  if (typeof value === "number") {
    if (typeof node.minimum === "number" && value < node.minimum) found.push(`${path}: below ${node.minimum}`);
    if (typeof node.maximum === "number" && value > node.maximum) found.push(`${path}: above ${node.maximum}`);
  }
  if (Array.isArray(value)) {
    if (typeof node.minItems === "number" && value.length < node.minItems) found.push(`${path}: fewer than ${node.minItems} items`);
    if (typeof node.maxItems === "number" && value.length > node.maxItems) found.push(`${path}: more than ${node.maxItems} items`);
    if (node.uniqueItems === true && new Set(value.map((item) => JSON.stringify(item))).size !== value.length) found.push(`${path}: items are not unique`);
    const items = node.items;
    if (items !== undefined && !Array.isArray(items)) for (const [index, item] of value.entries()) found.push(...violations(items as SchemaNode, item, `${path}/${index}`));
    if (Array.isArray(items)) for (const [index, member] of items.entries()) if (index < value.length) found.push(...violations(member as SchemaNode, value[index], `${path}/${index}`));
  }
  if (actual === "object") {
    const record = value as Record<string, unknown>;
    const properties = (node.properties ?? {}) as Record<string, SchemaNode>;
    for (const name of (node.required ?? []) as string[]) if (!(name in record)) found.push(`${path}/${name}: required`);
    if (typeof node.minProperties === "number" && Object.keys(record).length < node.minProperties) found.push(`${path}: fewer than ${node.minProperties} properties`);
    if (typeof node.maxProperties === "number" && Object.keys(record).length > node.maxProperties) found.push(`${path}: more than ${node.maxProperties} properties`);
    for (const [name, member] of Object.entries(record)) {
      const declaredMember = properties[name];
      if (declaredMember !== undefined) found.push(...violations(declaredMember, member, `${path}/${name}`));
      else if (node.additionalProperties === false) found.push(`${path}/${name}: not permitted`);
      else if (typeof node.additionalProperties === "object" && node.additionalProperties !== null) found.push(...violations(node.additionalProperties as SchemaNode, member, `${path}/${name}`));
    }
  }
  return found;
}

/** ✅️ Every reason `value` fails one named export — empty means it conforms. */
export function osMcpViolations(exportId: string, value: unknown): readonly string[] {
  return violations(osMcpSchema(exportId), value, "");
}

function parseExport(exportId: string, value: unknown): unknown {
  const found = osMcpViolations(exportId, value);
  if (found.length > 0) throw new Error(`${exportId}: ${found.join("; ")}`);
  return value;
}
//#endregion 🔖️Walker

//#region 🔖️Exports
/** 🆔️ Every ExportId this scope publishes, in the document's own (key-sorted) order. */
export const OS_MCP_EXPORT_IDS = ["ActionInvokeInput", "ActionPrepareInput", "ArtifactCreateInput", "ArtifactCreateOutput", "ArtifactCreateTemplateInput", "ArtifactExportInput", "ArtifactExportOutput", "ArtifactOpenInput", "ArtifactOpenOutput", "ArtifactSnapshotInput", "ArtifactSnapshotOutput", "ArtifactValidateInput", "ArtifactValidateOutput", "CallToolResult", "CapabilitiesDescribeInput", "CapabilitiesDescribeOutput", "CapabilitiesSearchInput", "CapabilitiesSearchOutput", "CapabilityActionInput", "CapabilityGenericInput", "CapabilityGenericOutput", "ContentBlock", "ContextResolveInput", "ContextResolveOutput", "ContextSummary", "GatewayError", "GatewayErrorCode", "GisMapInferenceApprovalRequestV1", "HandleInput", "InferenceApproveInput", "InferenceGetInput", "InferenceGetOutput", "InferenceJobHandleInput", "InferenceJobOutput", "InferenceListInput", "InferenceListOutput", "InferenceSubmitInput", "InvocationReport", "InvocationStatus", "JobCancelInput", "JobGetInput", "JobSnapshotOutput", "JobState", "JobStatus", "NullableRevisionStamp", "PreparedActionReport", "Prompt", "PromptArgument", "PromptGetResult", "PromptMessage", "Resource", "ResourceContent", "ResourceTemplate", "RevisionStamp", "SearchHit", "Tool", "TransactionBeginInput", "UiDialogOpenInput", "UiFocusInput", "UiFocusOutput", "UiRevealInput", "UiRevealOutput"] as const;

export type OsMcpExportId = (typeof OS_MCP_EXPORT_IDS)[number];

export type ActionInvokeInput = {
  readonly "approvalHandle"?: string;
  readonly "capabilityId"?: string;
  readonly "expectedRevision"?: { readonly [key: string]: JsonValue };
  readonly "idempotencyKey"?: string;
  readonly "input"?: { readonly [key: string]: JsonValue };
  readonly "preparedActionHandle"?: string;
};

export type ActionPrepareInput = {
  readonly "capabilityId": string;
  readonly "input"?: { readonly [key: string]: JsonValue };
};

export type ArtifactCreateInput = {
  readonly "artifactId": string;
  readonly "initial"?: JsonValue;
  readonly "kind": string;
};

export type ArtifactCreateOutput = {
  readonly "artifactId"?: string;
  readonly "kind"?: string;
  readonly "revision"?: {
    readonly "artifactId"?: string;
    readonly "cursor"?: string;
    readonly "headEditId"?: string;
  } | null;
};

export type ArtifactCreateTemplateInput = {
  readonly "kind": string;
  readonly "template"?: string;
};

export type ArtifactExportInput = {
  readonly "artifactId": string;
  readonly "format"?: string;
};

export type ArtifactExportOutput = {
  readonly "artifactId"?: string;
  readonly "contentBase64"?: string | null;
  readonly "format"?: string;
  readonly "mimeType"?: string | null;
};

export type ArtifactOpenInput = {
  readonly "artifactId": string;
};

export type ArtifactOpenOutput = {
  readonly "artifactId"?: string;
  readonly "kind"?: string | null;
  readonly "revision"?: {
    readonly "artifactId"?: string;
    readonly "cursor"?: string;
    readonly "headEditId"?: string;
  } | null;
  readonly "sizeBytes"?: number | null;
};

export type ArtifactSnapshotInput = {
  readonly "artifactId": string;
  readonly "revision"?: {
    readonly "artifactId"?: string;
    readonly "cursor"?: string;
    readonly "headEditId"?: string;
  } | null;
};

export type ArtifactSnapshotOutput = {
  readonly "artifactId"?: string;
  readonly "packBase64"?: string | null;
  readonly "packBytes"?: number | null;
  readonly "sprBytes"?: number | null;
};

export type ArtifactValidateInput = {
  readonly "artifactId": string;
};

export type ArtifactValidateOutput = { readonly [key: string]: JsonValue };

export type CallToolResult = {
  readonly "content": readonly ContentBlock[];
  readonly "isError": boolean;
  readonly "structuredContent"?: JsonValue;
};

export type CapabilitiesDescribeInput = {
  readonly "capabilityId": string;
};

export type CapabilitiesDescribeOutput = { readonly [key: string]: JsonValue };

export type CapabilitiesSearchInput = {
  readonly "artifactKind"?: string;
  readonly "kind"?: readonly string[];
  readonly "owner"?: string;
  readonly "query": string;
  readonly "requiresScope"?: string;
};

export type CapabilitiesSearchOutput = {
  readonly "results"?: readonly JsonValue[];
};

export type CapabilityActionInput = { readonly [key: string]: JsonValue };

export type CapabilityGenericInput = { readonly [key: string]: JsonValue };

export type CapabilityGenericOutput = { readonly [key: string]: JsonValue };

export type ContentBlock = {
  readonly "text": string;
  readonly "type": "text";
} | {
  readonly "data": string;
  readonly "mimeType": string;
  readonly "type": "image";
} | {
  readonly "data": string;
  readonly "mimeType": string;
  readonly "type": "audio";
} | {
  readonly "mimeType"?: string | null;
  readonly "name"?: string | null;
  readonly "type": "resource_link";
  readonly "uri": string;
} | {
  readonly "blob"?: string | null;
  readonly "mimeType"?: string | null;
  readonly "text"?: string | null;
  readonly "type": "resource";
  readonly "uri": string;
};

export type ContextResolveInput = {
  readonly "locale"?: string;
  readonly "principal"?: string;
};

export type ContextResolveOutput = { readonly [key: string]: JsonValue };

export type ContextSummary = {
  readonly "activeArtifactId"?: string | null;
  readonly "catalogHash": string;
  readonly "locale": string;
  readonly "principal": string;
  readonly "scopes": readonly string[];
  readonly "sessionId": string;
};

export type GatewayError = {
  readonly "code": GatewayErrorCode;
  readonly "details"?: JsonValue;
  readonly "message": string;
  readonly "retryable": boolean;
};

export type GatewayErrorCode = "INPUT_INVALID" | "PRECONDITION_FAILED" | "REVISION_CONFLICT" | "PERMISSION_DENIED" | "APPROVAL_REQUIRED" | "PLUGIN_UNAVAILABLE" | "SIDE_EFFECT_REJECTED" | "CANCELLED" | "COMPENSATION_FAILED" | "NOT_FOUND" | "BUDGET_EXCEEDED" | "INTERNAL";

export type GisMapInferenceApprovalRequestV1 = {
  readonly "jobId": string;
  readonly "proposalHash": string;
  readonly "schema": "semio.hub.inference-approval/v1";
  readonly "version": 1;
};

export type HandleInput = {
  readonly "preparedActionHandle": string;
} | {
  readonly "transactionHandle": string;
} | {
  readonly "undoToken": string;
};

export type InferenceApproveInput = {
  readonly "jobHandle": string;
  readonly "proposalHash": string;
};

export type InferenceGetInput = {
  readonly "artifactId": string;
  readonly "inferenceSchema": string;
};

export type InferenceGetOutput = { readonly [key: string]: JsonValue };

export type InferenceJobHandleInput = {
  readonly "after"?: number;
  readonly "jobHandle": string;
};

export type InferenceJobOutput = { readonly [key: string]: JsonValue };

export type InferenceListInput = {
  readonly "artifactId"?: string;
};

export type InferenceListOutput = {
  readonly "artifactId"?: JsonValue;
  readonly "artifactKind"?: JsonValue;
  readonly "declared"?: readonly JsonValue[];
};

export type InferenceSubmitInput = {
  readonly "documentId": string;
  readonly "lifetimeMs"?: number;
  readonly "requestId"?: string;
};

export type InvocationReport = {
  readonly "affectedResources": readonly string[];
  readonly "capabilityId": string;
  readonly "diffUri"?: string | null;
  readonly "invocationId": string;
  readonly "postconditions": readonly string[];
  readonly "replayed": boolean;
  readonly "revisionAfter"?: RevisionStamp | null;
  readonly "revisionBefore"?: RevisionStamp | null;
  readonly "status": InvocationStatus;
  readonly "undoToken"?: string | null;
  readonly "warnings": readonly string[];
};

export type InvocationStatus = "SUCCEEDED" | "FAILED" | "CANCELLED";

export type JobCancelInput = {
  readonly "jobId": string;
};

export type JobGetInput = {
  readonly "jobId": string;
};

export type JobSnapshotOutput = {
  readonly "cancelRequested"?: boolean;
  readonly "error"?: JsonValue;
  readonly "jobId"?: string;
  readonly "kind"?: string;
  readonly "message"?: JsonValue;
  readonly "progress"?: JsonValue;
  readonly "result"?: JsonValue;
  readonly "status"?: string;
};

export type JobState = "PENDING" | "RUNNING" | "SUCCEEDED" | "FAILED" | "CANCELLED";

export type JobStatus = {
  readonly "error"?: GatewayError | null;
  readonly "jobId": string;
  readonly "progress"?: number | null;
  readonly "result"?: JsonValue;
  readonly "state": JobState;
};

export type NullableRevisionStamp = {
  readonly "artifactId"?: string;
  readonly "cursor"?: string;
  readonly "headEditId"?: string;
} | null;

export type PreparedActionReport = {
  readonly "capabilityId": string;
  readonly "expectedRevision"?: RevisionStamp | null;
  readonly "expiresAtMs": number;
  readonly "preparedHandle": string;
  readonly "preview": JsonValue;
};

export type Prompt = {
  readonly "arguments"?: readonly PromptArgument[];
  readonly "description"?: string | null;
  readonly "name": string;
  readonly "title"?: string | null;
};

export type PromptArgument = {
  readonly "description"?: string | null;
  readonly "name": string;
  readonly "required": boolean;
};

export type PromptGetResult = {
  readonly "description"?: string | null;
  readonly "messages": readonly PromptMessage[];
};

export type PromptMessage = {
  readonly "content": ContentBlock;
  readonly "role": string;
};

export type Resource = {
  readonly "description"?: string | null;
  readonly "mimeType"?: string | null;
  readonly "name": string;
  readonly "size"?: number | null;
  readonly "title"?: string | null;
  readonly "uri": string;
};

export type ResourceContent = {
  readonly "blob"?: string | null;
  readonly "mimeType"?: string | null;
  readonly "text"?: string | null;
  readonly "uri": string;
};

export type ResourceTemplate = {
  readonly "description"?: string | null;
  readonly "mimeType"?: string | null;
  readonly "name": string;
  readonly "title"?: string | null;
  readonly "uriTemplate": string;
};

export type RevisionStamp = {
  readonly "artifactId": string;
  readonly "cursor": string;
  readonly "headEditId": string;
};

export type SearchHit = {
  readonly "appId": string;
  readonly "capabilityId": string;
  readonly "description": string;
  readonly "pluginId": string;
  readonly "score": number;
  readonly "title": string;
};

export type Tool = {
  readonly "_meta"?: JsonValue;
  readonly "annotations"?: JsonValue;
  readonly "description"?: string | null;
  readonly "inputSchema": JsonValue;
  readonly "name": string;
  readonly "outputSchema"?: JsonValue;
  readonly "title"?: string | null;
};

export type TransactionBeginInput = {
  readonly "preparedHandles": readonly string[];
};

export type UiDialogOpenInput = {
  readonly "args"?: { readonly [key: string]: JsonValue };
  readonly "dialogId": string;
};

export type UiFocusInput = {
  readonly "windowId"?: string;
};

export type UiFocusOutput = {
  readonly "ok"?: boolean;
  readonly "windowId"?: JsonValue;
};

export type UiRevealInput = {
  readonly "anchor": "left" | "right" | "top" | "bottom";
  readonly "path": readonly string[];
};

export type UiRevealOutput = {
  readonly "anchor"?: string;
  readonly "ok"?: boolean;
  readonly "path"?: readonly string[];
};
//#endregion 🔖️Exports

//#region 🔖️Parsers
export const parseActionInvokeInput = (value: unknown): ActionInvokeInput => parseExport("ActionInvokeInput", value) as ActionInvokeInput;
export const parseActionPrepareInput = (value: unknown): ActionPrepareInput => parseExport("ActionPrepareInput", value) as ActionPrepareInput;
export const parseArtifactCreateInput = (value: unknown): ArtifactCreateInput => parseExport("ArtifactCreateInput", value) as ArtifactCreateInput;
export const parseArtifactCreateOutput = (value: unknown): ArtifactCreateOutput => parseExport("ArtifactCreateOutput", value) as ArtifactCreateOutput;
export const parseArtifactCreateTemplateInput = (value: unknown): ArtifactCreateTemplateInput => parseExport("ArtifactCreateTemplateInput", value) as ArtifactCreateTemplateInput;
export const parseArtifactExportInput = (value: unknown): ArtifactExportInput => parseExport("ArtifactExportInput", value) as ArtifactExportInput;
export const parseArtifactExportOutput = (value: unknown): ArtifactExportOutput => parseExport("ArtifactExportOutput", value) as ArtifactExportOutput;
export const parseArtifactOpenInput = (value: unknown): ArtifactOpenInput => parseExport("ArtifactOpenInput", value) as ArtifactOpenInput;
export const parseArtifactOpenOutput = (value: unknown): ArtifactOpenOutput => parseExport("ArtifactOpenOutput", value) as ArtifactOpenOutput;
export const parseArtifactSnapshotInput = (value: unknown): ArtifactSnapshotInput => parseExport("ArtifactSnapshotInput", value) as ArtifactSnapshotInput;
export const parseArtifactSnapshotOutput = (value: unknown): ArtifactSnapshotOutput => parseExport("ArtifactSnapshotOutput", value) as ArtifactSnapshotOutput;
export const parseArtifactValidateInput = (value: unknown): ArtifactValidateInput => parseExport("ArtifactValidateInput", value) as ArtifactValidateInput;
export const parseArtifactValidateOutput = (value: unknown): ArtifactValidateOutput => parseExport("ArtifactValidateOutput", value) as ArtifactValidateOutput;
export const parseCallToolResult = (value: unknown): CallToolResult => parseExport("CallToolResult", value) as CallToolResult;
export const parseCapabilitiesDescribeInput = (value: unknown): CapabilitiesDescribeInput => parseExport("CapabilitiesDescribeInput", value) as CapabilitiesDescribeInput;
export const parseCapabilitiesDescribeOutput = (value: unknown): CapabilitiesDescribeOutput => parseExport("CapabilitiesDescribeOutput", value) as CapabilitiesDescribeOutput;
export const parseCapabilitiesSearchInput = (value: unknown): CapabilitiesSearchInput => parseExport("CapabilitiesSearchInput", value) as CapabilitiesSearchInput;
export const parseCapabilitiesSearchOutput = (value: unknown): CapabilitiesSearchOutput => parseExport("CapabilitiesSearchOutput", value) as CapabilitiesSearchOutput;
export const parseCapabilityActionInput = (value: unknown): CapabilityActionInput => parseExport("CapabilityActionInput", value) as CapabilityActionInput;
export const parseCapabilityGenericInput = (value: unknown): CapabilityGenericInput => parseExport("CapabilityGenericInput", value) as CapabilityGenericInput;
export const parseCapabilityGenericOutput = (value: unknown): CapabilityGenericOutput => parseExport("CapabilityGenericOutput", value) as CapabilityGenericOutput;
export const parseContentBlock = (value: unknown): ContentBlock => parseExport("ContentBlock", value) as ContentBlock;
export const parseContextResolveInput = (value: unknown): ContextResolveInput => parseExport("ContextResolveInput", value) as ContextResolveInput;
export const parseContextResolveOutput = (value: unknown): ContextResolveOutput => parseExport("ContextResolveOutput", value) as ContextResolveOutput;
export const parseContextSummary = (value: unknown): ContextSummary => parseExport("ContextSummary", value) as ContextSummary;
export const parseGatewayError = (value: unknown): GatewayError => parseExport("GatewayError", value) as GatewayError;
export const parseGatewayErrorCode = (value: unknown): GatewayErrorCode => parseExport("GatewayErrorCode", value) as GatewayErrorCode;
export const parseGisMapInferenceApprovalRequestV1 = (value: unknown): GisMapInferenceApprovalRequestV1 => parseExport("GisMapInferenceApprovalRequestV1", value) as GisMapInferenceApprovalRequestV1;
export const parseHandleInput = (value: unknown): HandleInput => parseExport("HandleInput", value) as HandleInput;
export const parseInferenceApproveInput = (value: unknown): InferenceApproveInput => parseExport("InferenceApproveInput", value) as InferenceApproveInput;
export const parseInferenceGetInput = (value: unknown): InferenceGetInput => parseExport("InferenceGetInput", value) as InferenceGetInput;
export const parseInferenceGetOutput = (value: unknown): InferenceGetOutput => parseExport("InferenceGetOutput", value) as InferenceGetOutput;
export const parseInferenceJobHandleInput = (value: unknown): InferenceJobHandleInput => parseExport("InferenceJobHandleInput", value) as InferenceJobHandleInput;
export const parseInferenceJobOutput = (value: unknown): InferenceJobOutput => parseExport("InferenceJobOutput", value) as InferenceJobOutput;
export const parseInferenceListInput = (value: unknown): InferenceListInput => parseExport("InferenceListInput", value) as InferenceListInput;
export const parseInferenceListOutput = (value: unknown): InferenceListOutput => parseExport("InferenceListOutput", value) as InferenceListOutput;
export const parseInferenceSubmitInput = (value: unknown): InferenceSubmitInput => parseExport("InferenceSubmitInput", value) as InferenceSubmitInput;
export const parseInvocationReport = (value: unknown): InvocationReport => parseExport("InvocationReport", value) as InvocationReport;
export const parseInvocationStatus = (value: unknown): InvocationStatus => parseExport("InvocationStatus", value) as InvocationStatus;
export const parseJobCancelInput = (value: unknown): JobCancelInput => parseExport("JobCancelInput", value) as JobCancelInput;
export const parseJobGetInput = (value: unknown): JobGetInput => parseExport("JobGetInput", value) as JobGetInput;
export const parseJobSnapshotOutput = (value: unknown): JobSnapshotOutput => parseExport("JobSnapshotOutput", value) as JobSnapshotOutput;
export const parseJobState = (value: unknown): JobState => parseExport("JobState", value) as JobState;
export const parseJobStatus = (value: unknown): JobStatus => parseExport("JobStatus", value) as JobStatus;
export const parseNullableRevisionStamp = (value: unknown): NullableRevisionStamp => parseExport("NullableRevisionStamp", value) as NullableRevisionStamp;
export const parsePreparedActionReport = (value: unknown): PreparedActionReport => parseExport("PreparedActionReport", value) as PreparedActionReport;
export const parsePrompt = (value: unknown): Prompt => parseExport("Prompt", value) as Prompt;
export const parsePromptArgument = (value: unknown): PromptArgument => parseExport("PromptArgument", value) as PromptArgument;
export const parsePromptGetResult = (value: unknown): PromptGetResult => parseExport("PromptGetResult", value) as PromptGetResult;
export const parsePromptMessage = (value: unknown): PromptMessage => parseExport("PromptMessage", value) as PromptMessage;
export const parseResource = (value: unknown): Resource => parseExport("Resource", value) as Resource;
export const parseResourceContent = (value: unknown): ResourceContent => parseExport("ResourceContent", value) as ResourceContent;
export const parseResourceTemplate = (value: unknown): ResourceTemplate => parseExport("ResourceTemplate", value) as ResourceTemplate;
export const parseRevisionStamp = (value: unknown): RevisionStamp => parseExport("RevisionStamp", value) as RevisionStamp;
export const parseSearchHit = (value: unknown): SearchHit => parseExport("SearchHit", value) as SearchHit;
export const parseTool = (value: unknown): Tool => parseExport("Tool", value) as Tool;
export const parseTransactionBeginInput = (value: unknown): TransactionBeginInput => parseExport("TransactionBeginInput", value) as TransactionBeginInput;
export const parseUiDialogOpenInput = (value: unknown): UiDialogOpenInput => parseExport("UiDialogOpenInput", value) as UiDialogOpenInput;
export const parseUiFocusInput = (value: unknown): UiFocusInput => parseExport("UiFocusInput", value) as UiFocusInput;
export const parseUiFocusOutput = (value: unknown): UiFocusOutput => parseExport("UiFocusOutput", value) as UiFocusOutput;
export const parseUiRevealInput = (value: unknown): UiRevealInput => parseExport("UiRevealInput", value) as UiRevealInput;
export const parseUiRevealOutput = (value: unknown): UiRevealOutput => parseExport("UiRevealOutput", value) as UiRevealOutput;
//#endregion 🔖️Parsers
