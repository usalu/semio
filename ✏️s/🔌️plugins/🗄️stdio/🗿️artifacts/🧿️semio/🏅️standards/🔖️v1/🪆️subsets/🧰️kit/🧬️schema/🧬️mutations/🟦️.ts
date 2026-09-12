import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactRef, type ArtifactRef } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { parseLinkPin, type LinkPin } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import { parseSemioChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseSemioKitConnection, parseSemioKitPiece, type SemioKitConnection, type SemioKitPiece } from "../📸️snapshot/🟦️.ts";

export interface CreateObject { child_id: string; target: ArtifactRef }
export interface DeleteObject { child_id: string }
export interface CreateModel { child_id: string; target: ArtifactRef }
export interface DeleteModel { child_id: string }
export interface CreateProperties { child_id: string; target: ArtifactRef }
export interface DeleteProperties {}
export interface BindRepresentation { target: ArtifactRef; pin: LinkPin; role: string }
export interface UnbindRepresentation { index: number }
export interface ChangeRepresentationPin { index: number; pin: LinkPin }
export interface AddType { id: string; name: string; category: string }
export interface RemoveType { id: string }
export interface RenameType { id: string; new_name: string }
export interface AddDesign { id: string; name: string }
export interface RemoveDesign { id: string }
export interface EditDesign { id: string; pieces: SemioKitPiece[]; connections: SemioKitConnection[] }

export type SemioKitMutation =
  | { CreateObject: CreateObject }
  | { DeleteObject: DeleteObject }
  | { CreateModel: CreateModel }
  | { DeleteModel: DeleteModel }
  | { CreateProperties: CreateProperties }
  | { DeleteProperties: DeleteProperties }
  | { BindRepresentation: BindRepresentation }
  | { UnbindRepresentation: UnbindRepresentation }
  | { ChangeRepresentationPin: ChangeRepresentationPin }
  | { AddType: AddType }
  | { RemoveType: RemoveType }
  | { RenameType: RenameType }
  | { AddDesign: AddDesign }
  | { RemoveDesign: RemoveDesign }
  | { EditDesign: EditDesign };

function stringField(row: Record<string, unknown>, field: string, at: string): string {
  if (typeof row[field] !== "string") throw new Error(`${at}.${field}: string required`);
  return row[field];
}

function indexField(row: Record<string, unknown>, at: string): number {
  if (!Number.isSafeInteger(row.index) || (row.index as number) < 0) throw new Error(at + ".index: non-negative integer required");
  return row.index as number;
}

function createChild(value: unknown, subset: "object" | "model" | "value", at: string): CreateObject {
  const row = parseSchemaRecord(value, ["child_id", "target"], at);
  const child_id = stringField(row, "child_id", at);
  const target = parseArtifactRef(row.target);
  parseSemioChild({ childId: child_id, target }, subset, at);
  return { child_id, target };
}

function deleteChild(value: unknown, at: string): DeleteObject {
  const row = parseSchemaRecord(value, ["child_id"], at);
  return { child_id: stringField(row, "child_id", at) };
}

function empty(value: unknown, at: string): DeleteProperties {
  parseSchemaRecord(value, [], at);
  return {};
}

function id(value: unknown, at: string): RemoveType {
  const row = parseSchemaRecord(value, ["id"], at);
  return { id: stringField(row, "id", at) };
}

function parseEditDesign(value: unknown, at: string): EditDesign {
  const row = parseSchemaRecord(value, ["id", "pieces", "connections"], at);
  if (!Array.isArray(row.pieces) || !Array.isArray(row.connections)) throw new Error(at + ": piece and connection arrays required");
  const pieces = row.pieces.map((entry, index) => parseSemioKitPiece(entry, `${at}.pieces[${index}]`));
  const connections = row.connections.map((entry, index) => parseSemioKitConnection(entry, `${at}.connections[${index}]`));
  const pieceIds = new Set(pieces.map((piece) => piece.id));
  if (pieceIds.size !== pieces.length || connections.some((connection) => !pieceIds.has(connection.connectingPieceId) || !pieceIds.has(connection.connectedPieceId))) throw new Error(at + ": design identity mismatch");
  return { id: stringField(row, "id", at), pieces, connections };
}

/** 🧬️ Parses exactly one externally tagged native Kit mutation variant. */
export function parseSemioKitMutation(value: unknown, at = "$"): SemioKitMutation {
  const row = parseSchemaRecord(value, ["CreateObject", "DeleteObject", "CreateModel", "DeleteModel", "CreateProperties", "DeleteProperties", "BindRepresentation", "UnbindRepresentation", "ChangeRepresentationPin", "AddType", "RemoveType", "RenameType", "AddDesign", "RemoveDesign", "EditDesign"], at);
  const variants = Object.keys(row);
  if (variants.length !== 1) throw new Error(at + ": exactly one Kit mutation variant required");
  const variant = variants[0]!;
  const payload = row[variant];
  const payloadAt = `${at}.${variant}`;
  if (variant === "CreateObject") return { CreateObject: createChild(payload, "object", payloadAt) };
  if (variant === "DeleteObject") return { DeleteObject: deleteChild(payload, payloadAt) };
  if (variant === "CreateModel") return { CreateModel: createChild(payload, "model", payloadAt) };
  if (variant === "DeleteModel") return { DeleteModel: deleteChild(payload, payloadAt) };
  if (variant === "CreateProperties") return { CreateProperties: createChild(payload, "value", payloadAt) };
  if (variant === "DeleteProperties") return { DeleteProperties: empty(payload, payloadAt) };
  if (variant === "BindRepresentation") {
    const entry = parseSchemaRecord(payload, ["target", "pin", "role"], payloadAt);
    return { BindRepresentation: { target: parseArtifactRef(entry.target), pin: parseLinkPin(entry.pin), role: stringField(entry, "role", payloadAt) } };
  }
  if (variant === "UnbindRepresentation") {
    const entry = parseSchemaRecord(payload, ["index"], payloadAt);
    return { UnbindRepresentation: { index: indexField(entry, payloadAt) } };
  }
  if (variant === "ChangeRepresentationPin") {
    const entry = parseSchemaRecord(payload, ["index", "pin"], payloadAt);
    return { ChangeRepresentationPin: { index: indexField(entry, payloadAt), pin: parseLinkPin(entry.pin) } };
  }
  if (variant === "AddType") {
    const entry = parseSchemaRecord(payload, ["id", "name", "category"], payloadAt);
    return { AddType: { id: stringField(entry, "id", payloadAt), name: stringField(entry, "name", payloadAt), category: stringField(entry, "category", payloadAt) } };
  }
  if (variant === "RemoveType") return { RemoveType: id(payload, payloadAt) };
  if (variant === "RenameType") {
    const entry = parseSchemaRecord(payload, ["id", "new_name"], payloadAt);
    return { RenameType: { id: stringField(entry, "id", payloadAt), new_name: stringField(entry, "new_name", payloadAt) } };
  }
  if (variant === "AddDesign") {
    const entry = parseSchemaRecord(payload, ["id", "name"], payloadAt);
    return { AddDesign: { id: stringField(entry, "id", payloadAt), name: stringField(entry, "name", payloadAt) } };
  }
  if (variant === "RemoveDesign") return { RemoveDesign: id(payload, payloadAt) };
  return { EditDesign: parseEditDesign(payload, payloadAt) };
}
