import {compileDocumentSchemaV1} from "../../📇️directory/🔌️client/🌐️document-http/🧬️schema/🟦️.ts";
import {boundedServicePayloadV1, admittedServiceSchemaRecordV1, type ServiceSchemaRecordV1} from "./🧬️schema/🧺️value/🟦️.ts";
export {boundedServicePayloadV1} from "./🧬️schema/🧺️value/🟦️.ts";

export const DOCUMENT_SERVICE_TOPIC_V1 = "semio.os.document-http-port/v1";

/** 📨 Owner-addressed bounded operation with an opaque owner-validated payload. */
export type InstalledServiceOperationV1 = Readonly<{ owner: string; serviceId: string; action: string; operationEpoch: number; payload: unknown }>;
export type InstalledServiceStatusV1 = Readonly<{ owner: string; serviceId: string; payload: unknown }>;
export type InstalledServiceHistoryStatusV1 = Readonly<{ phase: "unavailable" | "available" | "submitting" | "applied" | "failed"; canUndo: boolean; code: string | null }>;
export type AdmittedDocumentServiceOperationV1 = Readonly<{ action: string; method: "GET" | "POST"; route: readonly string[]; sendBody: boolean; cursorField: string | null; requestMaxBytes: number; responseMaxBytes: number; inputSchema: ServiceSchemaRecordV1; outputSchema: ServiceSchemaRecordV1 }>;
export type AdmittedDocumentServiceDeclarationV1 = Readonly<{ schema: typeof DOCUMENT_SERVICE_TOPIC_V1; owner: string; serviceId: string; operations: readonly AdmittedDocumentServiceOperationV1[] }>;

/** 🛂 Parses one exact operation envelope before admitting its owner payload. */
export function parseInstalledServiceOperationV1(value: unknown): InstalledServiceOperationV1 & { readonly kind: "service-operation" } {
  const row = record(value);
  if (Object.keys(row).sort().join(",") !== "action,kind,operationEpoch,owner,payload,serviceId" || row.kind !== "service-operation" || typeof row.operationEpoch !== "number" || !Number.isSafeInteger(row.operationEpoch) || row.operationEpoch < 1) throw new Error("installed-service.invalid");
  return { kind: "service-operation", owner: text(row.owner, 128), serviceId: text(row.serviceId, 256), action: text(row.action, 64), operationEpoch: row.operationEpoch, payload: boundedServicePayloadV1(row.payload) };
}

function record(value: unknown): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value) || Object.getPrototypeOf(value) !== Object.prototype || Object.getOwnPropertySymbols(value).length || Object.values(Object.getOwnPropertyDescriptors(value)).some(descriptor => !Object.hasOwn(descriptor,"value") || !descriptor.enumerable)) throw new Error("installed-service.invalid");
  return value as Readonly<Record<string, unknown>>;
}

function text(value: unknown, maximum: number): string {
  if (typeof value !== "string" || value.length === 0 || value.length > maximum || /\p{Cc}/u.test(value)) throw new Error("installed-service.invalid");
  return value;
}

/** 🪪 Parses an owner status without interpreting any owner's lifecycle or geometry. */
export function parseInstalledServiceStatusV1(value: unknown): InstalledServiceStatusV1 {
  const row = record(value);
  if (Object.keys(row).sort().join(",") !== "owner,payload,serviceId") throw new Error("installed-service.invalid");
  return { owner: text(row.owner, 128), serviceId: text(row.serviceId, 256), payload: boundedServicePayloadV1(row.payload) };
}

/** ↩️ Framework history selection carries availability while its receipt remains owner-private. */
export function parseInstalledServiceHistoryStatusV1(value: unknown): InstalledServiceHistoryStatusV1 {
  const row = record(value);
  if (Object.keys(row).sort().join(",") !== "canUndo,code,phase" || !["unavailable", "available", "submitting", "applied", "failed"].includes(String(row.phase))) throw new Error("installed-service.invalid");
  if (typeof row.canUndo !== "boolean" || (row.phase !== "available" && row.phase !== "failed" && row.canUndo) || (row.phase === "available" && !row.canUndo) || ((row.phase === "failed") !== (row.code !== null))) throw new Error("installed-service.invalid");
  return { phase: row.phase as InstalledServiceHistoryStatusV1["phase"], canUndo: row.canUndo, code: row.code === null ? null : text(row.code, 128) };
}

/** 📜 Validates the exact installed declaration and owner schema identities before routing. */
export function admitDocumentServiceDeclarationV1(owner: string, value: unknown): AdmittedDocumentServiceDeclarationV1 {
  const row = record(value);
  if (Object.keys(row).sort().join(",") !== "operations,owner,schema,serviceId" || row.schema !== DOCUMENT_SERVICE_TOPIC_V1 || row.owner !== owner || !Array.isArray(row.operations) || row.operations.length < 1 || row.operations.length > 8) throw new Error("installed-service.invalid");
  text(owner, 128);
  const actions = new Set<string>();
  const operations = row.operations.map((value): AdmittedDocumentServiceOperationV1 => {
    const op = record(value);
    if (Object.keys(op).sort().join(",") !== "action,cursorField,inputSchema,method,outputSchema,requestMaxBytes,responseMaxBytes,route,sendBody") throw new Error("installed-service.invalid");
    const action = text(op.action, 64);
    if (actions.has(action) || (op.method !== "GET" && op.method !== "POST") || typeof op.sendBody !== "boolean" || (op.method === "GET" && op.sendBody) || !Array.isArray(op.route) || op.route.length < 1 || op.route.length > 8) throw new Error("installed-service.invalid");
    actions.add(action);
    const route = op.route.map((value) => { const segment = text(value, 256); if (segment === "." || segment === ".." || /[/?#]/u.test(segment) || (/[{}]/u.test(segment) && !/^\{[A-Za-z][A-Za-z0-9]*\}$/u.test(segment))) throw new Error("installed-service.invalid"); return segment; });
    const limit = (value: unknown, maximum: number): number => { if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 1 || value > maximum) throw new Error("installed-service.bounds"); return value; };
    const schema = (source: unknown): ServiceSchemaRecordV1 => { const value = admittedServiceSchemaRecordV1(source); if (typeof value.$id !== "string" || !value.$id.startsWith(`${owner}:`)) throw new Error("installed-service.foreign-schema"); compileDocumentSchemaV1(value); return value; };
    return Object.freeze({ action, method: op.method, route: Object.freeze(route), sendBody: op.sendBody, cursorField: op.cursorField === null ? null : text(op.cursorField, 64), requestMaxBytes: limit(op.requestMaxBytes, 64 * 1024), responseMaxBytes: limit(op.responseMaxBytes, 64 * 1024), inputSchema: schema(op.inputSchema), outputSchema: schema(op.outputSchema) });
  });
  return Object.freeze({ schema: DOCUMENT_SERVICE_TOPIC_V1, owner, serviceId: text(row.serviceId, 256), operations: Object.freeze(operations) });
}

export interface InstalledServiceDriverV1<Context = unknown> {
  readonly owner: string;
  readonly serviceId: string;
  dispatch(operation: InstalledServiceOperationV1): void;
  retire(): void;
  sessionRetired(): void;
  documentClosed(runtimeKey: string): void;
  documentRebootstrapped(context: Context): void;
  documentMounted(context: Context): void;
}

/** 🔌 A finite executable contribution roster; removal revokes every retained owner callback first. */
export class InstalledServiceRegistryV1<Context = unknown> {
  private readonly drivers = new Map<string, InstalledServiceDriverV1<Context>>();

  private key(owner: string, serviceId: string): string { return `${text(owner, 128)}\0${text(serviceId, 256)}`; }

  install(driver: InstalledServiceDriverV1<Context>): () => void {
    const key = this.key(driver.owner, driver.serviceId);
    if (this.drivers.has(key) || this.drivers.size >= 64) throw new Error("installed-service.capacity");
    this.drivers.set(key, driver);
    return () => { if (this.drivers.get(key) !== driver) return; this.drivers.delete(key); driver.retire(); };
  }

  dispatch(operation: InstalledServiceOperationV1): boolean {
    if (!Number.isSafeInteger(operation.operationEpoch) || operation.operationEpoch < 1) throw new Error("installed-service.invalid-epoch");
    text(operation.action, 64);
    const driver = this.drivers.get(this.key(operation.owner, operation.serviceId));
    if (!driver) return false;
    driver.dispatch({ ...operation, payload: boundedServicePayloadV1(operation.payload) });
    return true;
  }

  sessionRetired(): void { for (const driver of this.drivers.values()) driver.sessionRetired(); }
  documentClosed(runtimeKey: string): void { for (const driver of this.drivers.values()) driver.documentClosed(runtimeKey); }
  documentRebootstrapped(context: Context): void { for (const driver of this.drivers.values()) driver.documentRebootstrapped(context); }
  documentMounted(context: Context): void { for (const driver of this.drivers.values()) driver.documentMounted(context); }
}

/** 🧩 Validates the finite installed execution contribution set selected by the Shell registry. */
export function parseInstalledServiceContributionsV1(value: unknown): Readonly<{ kind: "service-contributions"; services: readonly Readonly<{ owner: string; serviceId: string }>[] }> {
  const row = record(value);
  if (Object.keys(row).sort().join(",") !== "kind,services" || row.kind !== "service-contributions" || !Array.isArray(row.services) || row.services.length > 64) throw new Error("installed-service.invalid");
  const seen = new Set<string>();
  const services = row.services.map((value) => { const row = record(value); if (Object.keys(row).sort().join(",") !== "owner,serviceId") throw new Error("installed-service.invalid"); const owner = text(row.owner, 128), serviceId = text(row.serviceId, 256), key = `${owner}\0${serviceId}`; if (seen.has(key)) throw new Error("installed-service.invalid"); seen.add(key); return { owner, serviceId }; });
  return { kind: "service-contributions", services };
}
