import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactLink, type ArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import { parseSemioTransform, type SemioTransform } from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";
export type { ArtifactLink } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
export type { SemioTransform } from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
export type { ArtifactChild } from "../../../✉️base/🧬️schema/🪆️child/🟦️.ts";

export interface SemioKitType { id: string; name: string; category: string }
export interface SemioKitPiece { id: string; typeId: string; transform: SemioTransform }
export interface SemioKitConnection { id: string; connectingPieceId: string; connectingPort: string; connectedPieceId: string; connectedPort: string }
export interface SemioKitDesign { id: string; name: string; pieces: SemioKitPiece[]; connections: SemioKitConnection[] }

export interface SemioKitSnapshot {
  /** @state artifact */ schema: "stdio.semio.kit";
  /** @state artifact */ types: SemioKitType[];
  /** @state artifact */ designs: SemioKitDesign[];
  /** @state artifact @child kind=s.stdio.semio many */ objects: ArtifactChild[];
  /** @state artifact @child kind=s.stdio.semio many */ models: ArtifactChild[];
  /** @state artifact @child kind=s.stdio.semio */ properties?: ArtifactChild;
  /** @state artifact @link_slot roles=representation many */ representations: ArtifactLink[];
}

function stringField(row: Record<string, unknown>, field: string, at: string): string {
  if (typeof row[field] !== "string") throw new Error(`${at}.${field}: string required`);
  return row[field];
}

function arrayField(row: Record<string, unknown>, field: string, at: string): unknown[] {
  if (!Array.isArray(row[field])) throw new Error(`${at}.${field}: array required`);
  return row[field];
}

function uniqueIds(values: readonly { id: string }[], at: string): void {
  const ids = new Set<string>();
  for (const value of values) {
    if (ids.has(value.id)) throw new Error(`${at}: duplicate id ${JSON.stringify(value.id)}`);
    ids.add(value.id);
  }
}

/** 🏷️ Parses one exact Kit catalog entry. */
export function parseSemioKitType(value: unknown, at = "$"): SemioKitType {
  const row = parseSchemaRecord(value, ["id", "name", "category"], at);
  return { id: stringField(row, "id", at), name: stringField(row, "name", at), category: stringField(row, "category", at) };
}

/** 📐️ Parses one design piece through the shared transform owner. */
export function parseSemioKitPiece(value: unknown, at = "$"): SemioKitPiece {
  const row = parseSchemaRecord(value, ["id", "typeId", "transform"], at);
  return { id: stringField(row, "id", at), typeId: stringField(row, "typeId", at), transform: parseSemioTransform(row.transform, at + ".transform") };
}

/** 🔌️ Parses one exact connection between two design pieces. */
export function parseSemioKitConnection(value: unknown, at = "$"): SemioKitConnection {
  const row = parseSchemaRecord(value, ["id", "connectingPieceId", "connectingPort", "connectedPieceId", "connectedPort"], at);
  return {
    id: stringField(row, "id", at),
    connectingPieceId: stringField(row, "connectingPieceId", at),
    connectingPort: stringField(row, "connectingPort", at),
    connectedPieceId: stringField(row, "connectedPieceId", at),
    connectedPort: stringField(row, "connectedPort", at),
  };
}

/** 📋️ Parses one design and verifies its internal piece connections. */
export function parseSemioKitDesign(value: unknown, at = "$"): SemioKitDesign {
  const row = parseSchemaRecord(value, ["id", "name", "pieces", "connections"], at);
  const pieces = arrayField(row, "pieces", at).map((entry, index) => parseSemioKitPiece(entry, `${at}.pieces[${index}]`));
  const connections = arrayField(row, "connections", at).map((entry, index) => parseSemioKitConnection(entry, `${at}.connections[${index}]`));
  uniqueIds(pieces, at + ".pieces");
  uniqueIds(connections, at + ".connections");
  const pieceIds = new Set(pieces.map((piece) => piece.id));
  for (const connection of connections) if (!pieceIds.has(connection.connectingPieceId) || !pieceIds.has(connection.connectedPieceId)) throw new Error(`${at}.connections: referenced piece required`);
  return { id: stringField(row, "id", at), name: stringField(row, "name", at), pieces, connections };
}

/** 🧬️ Parses the complete Kit document with exact catalog, child and link identities. */
export function parseSemioKitSnapshot(value: unknown, at = "$"): SemioKitSnapshot {
  const row = parseSchemaRecord(value, ["schema", "types", "designs", "objects", "models", "properties", "representations"], at);
  if (row.schema !== "stdio.semio.kit") throw new Error(at + ".schema: Kit schema required");
  const types = arrayField(row, "types", at).map((entry, index) => parseSemioKitType(entry, `${at}.types[${index}]`));
  const designs = arrayField(row, "designs", at).map((entry, index) => parseSemioKitDesign(entry, `${at}.designs[${index}]`));
  const objects = arrayField(row, "objects", at).map((entry, index) => parseSemioChild(entry, "object", `${at}.objects[${index}]`));
  const models = arrayField(row, "models", at).map((entry, index) => parseSemioChild(entry, "model", `${at}.models[${index}]`));
  const representations = arrayField(row, "representations", at).map((entry) => parseArtifactLink(entry));
  uniqueIds(types, at + ".types");
  uniqueIds(designs, at + ".designs");
  const typeIds = new Set(types.map((entry) => entry.id));
  for (const design of designs) for (const piece of design.pieces) if (!typeIds.has(piece.typeId)) throw new Error(`${at}.designs: referenced type required`);
  for (const representation of representations) if (!typeIds.has(representation.role)) throw new Error(`${at}.representations: role must name a Kit type`);
  const result: SemioKitSnapshot = { schema: row.schema, types, designs, objects, models, representations };
  if (Object.hasOwn(row, "properties")) result.properties = parseSemioChild(row.properties, "value", at + ".properties");
  return result;
}
