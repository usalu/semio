import {DOCUMENT_SERVICE_TOPIC_V1,admitDocumentServiceDeclarationV1,boundedServicePayloadV1,type AdmittedDocumentServiceDeclarationV1} from "../🟦️.ts";
import {admittedServiceSchemaRecordV1,type ServiceSchemaRecordV1,type ServiceValueV1} from "../🧬️schema/🧺️value/🟦️.ts";
import {compileDocumentSchemaV1} from "../../../📇️directory/🔌️client/🌐️document-http/🧬️schema/🟦️.ts";

export type DocumentServiceWireOperationV1 = Readonly<{action:string;method:"GET"|"POST";route:readonly string[];sendBody:boolean;cursorField:string|null;requestMaxBytes:number;responseMaxBytes:number;inputSchema:string;outputSchema:string}>;
export type DocumentServiceWireDeclarationV1 = Readonly<{schema:typeof DOCUMENT_SERVICE_TOPIC_V1;owner:string;serviceId:string;operations:readonly DocumentServiceWireOperationV1[]}>;
const SCHEMA_WIRE_MAX_BYTES_V1 = 32768;

function record(value: unknown): Readonly<Record<string,unknown>> {
  if(typeof value!=="object"||value===null||Array.isArray(value))throw new Error("installed-service.invalid");
  return value as Readonly<Record<string,unknown>>;
}
function text(value: unknown,maximum:number):string {
  if(typeof value!=="string"||value.length===0||new TextEncoder().encode(value).length>maximum||/\p{Cc}/u.test(value))throw new Error("installed-service.invalid");
  return value;
}

function wireMetadata(value:AdmittedDocumentServiceDeclarationV1):void {
  text(value.owner,128);text(value.serviceId,256);
  for(const operation of value.operations){text(operation.action,64);for(const segment of operation.route)text(segment,256);if(operation.cursorField!==null)text(operation.cursorField,64);}
}

/** 📜 Admits schema JSON text only at its physical byte boundary. */
export function decodeDocumentServiceSchemaV1(source: string): ServiceSchemaRecordV1 {
  if(typeof source!=="string"||source.length===0||new TextEncoder().encode(source).length>SCHEMA_WIRE_MAX_BYTES_V1)throw new Error("installed-service.bounds");
  const value=admittedServiceSchemaRecordV1(JSON.parse(source));
  compileDocumentSchemaV1(value);
  return value;
}

/** 🛂 Admits the native DocumentHttpPort declaration into owned semantic schema records. */
export function admitDocumentServiceWireDeclarationV1(owner:string,value:unknown):AdmittedDocumentServiceDeclarationV1 {
  const row=record(value);
  if(!Array.isArray(row.operations))throw new Error("installed-service.invalid");
  const operations=row.operations.map(value=>{const operation=record(value);if(typeof operation.inputSchema!=="string"||typeof operation.outputSchema!=="string")throw new Error("installed-service.invalid-schema");return {...operation,inputSchema:decodeDocumentServiceSchemaV1(operation.inputSchema),outputSchema:decodeDocumentServiceSchemaV1(operation.outputSchema)};});
  const admitted=admitDocumentServiceDeclarationV1(owner,{...row,operations});
  wireMetadata(admitted);
  return admitted;
}

/** 📤 Emits the existing native DocumentHttpPort string schema grammar from admitted records. */
export function encodeDocumentServiceWireDeclarationV1(value:AdmittedDocumentServiceDeclarationV1):DocumentServiceWireDeclarationV1 {
  wireMetadata(value);
  const encode=(schema:ServiceSchemaRecordV1):string=>{const output=JSON.stringify(schema);text(output,SCHEMA_WIRE_MAX_BYTES_V1);return output;};
  return Object.freeze({...value,operations:Object.freeze(value.operations.map(operation=>Object.freeze({...operation,inputSchema:encode(operation.inputSchema),outputSchema:encode(operation.outputSchema)})))});
}

/** 🌐 Encodes only an admitted operation inside its exact document scope. */
export function documentServiceRequestV1(declaration: AdmittedDocumentServiceDeclarationV1, scope: Readonly<{ spaceId: string; documentId: string }>, action: string, payload: unknown): Readonly<{ path: string; method: "GET" | "POST"; body?: string; responseMaxBytes: number }> {
  const operation = declaration.operations.find((entry) => entry.action === action);
  if (!operation) throw new Error("installed-service.unavailable");
  const value = boundedServicePayloadV1(payload);
  if (!compileDocumentSchemaV1(operation.inputSchema)(value)) throw new Error("installed-service.invalid-payload");
  const json = JSON.stringify(value);
  if (new TextEncoder().encode(json).length > operation.requestMaxBytes) throw new Error("installed-service.bounds");
  const fields = record(value);
  const segments = operation.route.map((segment) => segment.startsWith("{") ? text(fields[segment.slice(1, -1)], 256) : segment);
  let path = `/spaces/${encodeURIComponent(text(scope.spaceId, 256))}/documents/${encodeURIComponent(text(scope.documentId, 256))}/${segments.map(encodeURIComponent).join("/")}`;
  if (operation.cursorField !== null) { const cursor = fields[operation.cursorField]; if (typeof cursor !== "number" || !Number.isSafeInteger(cursor) || cursor < 0) throw new Error("installed-service.invalid"); path += `?after=${cursor}`; }
  return { path, method: operation.method, ...(operation.sendBody ? { body: json } : {}), responseMaxBytes: operation.responseMaxBytes };
}

/** 📥 Admits one response body against the operation's declared wire and semantic boundaries. */
export function admitDocumentServiceResponseV1(declaration:AdmittedDocumentServiceDeclarationV1,action:string,body:string):ServiceValueV1 {
  const operation=declaration.operations.find(entry=>entry.action===action);
  if(!operation)throw new Error("installed-service.unavailable");
  if(body.length===0||new TextEncoder().encode(body).length>operation.responseMaxBytes)throw new Error("installed-service.bounds");
  const value=boundedServicePayloadV1(JSON.parse(body));
  if(!compileDocumentSchemaV1(operation.outputSchema)(value))throw new Error("installed-service.invalid-payload");
  return value;
}
