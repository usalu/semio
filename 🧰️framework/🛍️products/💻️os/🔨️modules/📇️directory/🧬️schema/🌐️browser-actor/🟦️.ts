/** 🌐️ Path-free actor identities; schema and Rust twin are adjacent. */
export const DOCUMENT_BROWSER_ACTOR_MAX_BYTES = 67_108_864;
export const DOCUMENT_BROWSER_ACTOR_INTERFACES = Object.freeze([
  "semio:framework/host-async@1.0.0", "semio:framework/pure@1.0.0",
  "wasi:cli/environment@0.2.0", "wasi:cli/exit@0.2.0", "wasi:cli/stderr@0.2.0", "wasi:cli/stdin@0.2.0", "wasi:cli/stdout@0.2.0",
  "wasi:cli/terminal-input@0.2.0", "wasi:cli/terminal-output@0.2.0", "wasi:cli/terminal-stderr@0.2.0", "wasi:cli/terminal-stdin@0.2.0", "wasi:cli/terminal-stdout@0.2.0",
  "wasi:clocks/monotonic-clock@0.2.0", "wasi:io/error@0.2.0", "wasi:io/poll@0.2.0", "wasi:io/streams@0.2.0",
]);

/** 🧷️ Exact captured package bytes that an actor derivation must reference. */
export type DocumentBrowserActorSourceV1 = Readonly<{ componentSha256: string; descriptorByteSha256: string }>;

/** 🪪️ A closed actor identity never carries a path, byte owner, or policy source. */
export type DocumentClosedBrowserActorV1 = Readonly<{
  kind: "closed-browser-actor";
  schema: "semio.os.closed-browser-actor.v1";
  codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1";
  sha256: string;
  sourceComponentSha256: string;
  sourceDescriptorByteSha256: string;
  policySha256: string;
  importInterfaces: readonly string[];
}>;

/** 🧭️ Plan identity has an explicit absence state and deliberately no byte length. */
export type DocumentOpenBrowserActorV1 = Readonly<{ kind: "none" }> | DocumentClosedBrowserActorV1;

/** 📏️ Lease identity adds the independently bounded selected byte length. */
export type DocumentExecutionTargetBrowserActorV1 = Readonly<{ kind: "none" }> | (DocumentClosedBrowserActorV1 & Readonly<{ byteLength: number }>);

function deny(): never { throw new Error("document-browser-actor.invalid-identity"); }

function data(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value) || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) return deny();
  const output = Object.create(null) as Record<string, unknown>;
  const keys = Reflect.ownKeys(value);
  if (keys.length > 9) return deny();
  for (const key of keys) {
    const field = Object.getOwnPropertyDescriptor(value, key)!;
    if (typeof key !== "string" || !field.enumerable || !("value" in field)) return deny();
    output[key] = field.value;
  }
  return output;
}

function digest(value: unknown): string {
  return typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value) ? value : deny();
}

function interfaces(value: unknown): readonly string[] {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype || value.length > DOCUMENT_BROWSER_ACTOR_INTERFACES.length || Reflect.ownKeys(value).length !== value.length + 1) return deny();
  const output: string[] = [];
  for (let index = 0; index < value.length; index++) {
    const field = Object.getOwnPropertyDescriptor(value, String(index));
    if (!field || !("value" in field) || typeof field.value !== "string" || !DOCUMENT_BROWSER_ACTOR_INTERFACES.includes(field.value) || (index > 0 && output[index - 1] >= field.value)) return deny();
    output.push(field.value);
  }
  return Object.freeze(output);
}

function length(value: unknown): number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES ? value : deny();
}

function parse(value: unknown, source: DocumentBrowserActorSourceV1, renderer: unknown, lease: boolean): DocumentOpenBrowserActorV1 | DocumentExecutionTargetBrowserActorV1 {
  const row = data(value);
  if (row.kind === "none") {
    if (Object.keys(row).length !== 1 || (renderer !== "react" && renderer !== "wgpu")) return deny();
    return Object.freeze({ kind: "none" });
  }
  const keys = ["kind", "schema", "codegenPolicy", "sha256", "sourceComponentSha256", "sourceDescriptorByteSha256", "policySha256", "importInterfaces", ...(lease ? ["byteLength"] : [])];
  if (row.kind !== "closed-browser-actor" || renderer !== "wasm" || row.schema !== "semio.os.closed-browser-actor.v1" || row.codegenPolicy !== "semio.os.browser-jco-1.27.0-jspi.v1" || Object.keys(row).length !== keys.length || !keys.every(key => Object.hasOwn(row, key))) return deny();
  const component = digest(row.sourceComponentSha256), descriptor = digest(row.sourceDescriptorByteSha256);
  if (component !== source.componentSha256 || descriptor !== source.descriptorByteSha256) return deny();
  return Object.freeze({ kind: row.kind, schema: row.schema, codegenPolicy: row.codegenPolicy, sha256: digest(row.sha256), sourceComponentSha256: component, sourceDescriptorByteSha256: descriptor, policySha256: digest(row.policySha256), importInterfaces: interfaces(row.importInterfaces), ...(lease ? { byteLength: length(row.byteLength) } : {}) });
}

/** 🛂️ Admits an exact owned plan identity bound to package bytes and renderer. */
export function parseDocumentOpenBrowserActorV1(value: unknown, source: DocumentBrowserActorSourceV1, renderer: unknown): DocumentOpenBrowserActorV1 {
  return parse(value, source, renderer, false);
}

/** 🔏️ Admits an exact owned lease identity with a nonzero bounded length. */
export function parseDocumentExecutionTargetBrowserActorV1(value: unknown, source: DocumentBrowserActorSourceV1, renderer: unknown): DocumentExecutionTargetBrowserActorV1 {
  return parse(value, source, renderer, true) as DocumentExecutionTargetBrowserActorV1;
}

/** 🧾️ Projects a validated plan identity using only the selected actor length. */
export function documentBrowserActorLeaseFromPlanV1(plan: DocumentOpenBrowserActorV1, source: DocumentBrowserActorSourceV1, renderer: unknown, byteLength: number | undefined): DocumentExecutionTargetBrowserActorV1 {
  const identity = parseDocumentOpenBrowserActorV1(plan, source, renderer);
  if (identity.kind === "none") {
    if (byteLength !== undefined) return deny();
    return identity;
  }
  return parseDocumentExecutionTargetBrowserActorV1({ ...identity, byteLength }, source, renderer);
}

/** ⚖️ Compares every field of parsed identities, including lease length presence. */
export function sameDocumentBrowserActorV1(left: DocumentOpenBrowserActorV1 | DocumentExecutionTargetBrowserActorV1, right: DocumentOpenBrowserActorV1 | DocumentExecutionTargetBrowserActorV1): boolean {
  if (left.kind !== right.kind) return false;
  if (left.kind === "none" || right.kind === "none") return true;
  return left.schema === right.schema && left.codegenPolicy === right.codegenPolicy && left.sha256 === right.sha256
    && left.sourceComponentSha256 === right.sourceComponentSha256 && left.sourceDescriptorByteSha256 === right.sourceDescriptorByteSha256 && left.policySha256 === right.policySha256
    && left.importInterfaces.length === right.importInterfaces.length && left.importInterfaces.every((entry, index) => entry === right.importInterfaces[index])
    && ("byteLength" in left ? "byteLength" in right && left.byteLength === right.byteLength : !("byteLength" in right));
}
