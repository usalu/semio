import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
/** 🎒️ Individually authored ZIP archive, member, header and decoded byte relations. */
import type { ZipSnapshot, ZipExtraField } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteDocument, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Literal copy of this snapshot's adjacent handcrafted SQL asset. */
export const ZIP_SQLITE_SCHEMA = "CREATE TABLE zip_archive (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  comment TEXT NOT NULL,\n  comment_utf8 INTEGER NOT NULL CHECK (comment_utf8 IN (0, 1))\n);\nCREATE TABLE zip_entry (\n  id INTEGER PRIMARY KEY,\n  archive_id INTEGER NOT NULL REFERENCES zip_archive(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL,\n  compression_method INTEGER NOT NULL CHECK (compression_method BETWEEN 0 AND 65535),\n  data_descriptor_signature INTEGER NOT NULL CHECK (data_descriptor_signature IN (0, 1))\n);\nCREATE TABLE zip_entry_byte (\n  id INTEGER PRIMARY KEY,\n  entry_id INTEGER NOT NULL REFERENCES zip_entry(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE zip_local_header (\n  id INTEGER PRIMARY KEY REFERENCES zip_entry(id),\n  version_needed INTEGER NOT NULL CHECK (version_needed BETWEEN 0 AND 65535),\n  flags INTEGER NOT NULL CHECK (flags BETWEEN 0 AND 65535),\n  modified_time INTEGER NOT NULL CHECK (modified_time BETWEEN 0 AND 65535),\n  modified_date INTEGER NOT NULL CHECK (modified_date BETWEEN 0 AND 65535),\n  legacy_name_present INTEGER NOT NULL CHECK (legacy_name_present IN (0, 1))\n);\nCREATE TABLE zip_local_legacy_name_byte (\n  id INTEGER PRIMARY KEY,\n  header_id INTEGER NOT NULL REFERENCES zip_local_header(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE zip_central_header (\n  id INTEGER PRIMARY KEY REFERENCES zip_entry(id),\n  version_made_by INTEGER NOT NULL CHECK (version_made_by BETWEEN 0 AND 65535),\n  version_needed INTEGER NOT NULL CHECK (version_needed BETWEEN 0 AND 65535),\n  flags INTEGER NOT NULL CHECK (flags BETWEEN 0 AND 65535),\n  modified_time INTEGER NOT NULL CHECK (modified_time BETWEEN 0 AND 65535),\n  modified_date INTEGER NOT NULL CHECK (modified_date BETWEEN 0 AND 65535),\n  legacy_name_present INTEGER NOT NULL CHECK (legacy_name_present IN (0, 1)),\n  comment TEXT NOT NULL,\n  legacy_comment_present INTEGER NOT NULL CHECK (legacy_comment_present IN (0, 1)),\n  internal_attributes INTEGER NOT NULL CHECK (internal_attributes BETWEEN 0 AND 65535),\n  external_attributes INTEGER NOT NULL CHECK (external_attributes BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE zip_central_legacy_name_byte (\n  id INTEGER PRIMARY KEY,\n  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE zip_central_legacy_comment_byte (\n  id INTEGER PRIMARY KEY,\n  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE zip_local_extra_field (\n  id INTEGER PRIMARY KEY,\n  header_id INTEGER NOT NULL REFERENCES zip_local_header(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  tag INTEGER NOT NULL CHECK (tag BETWEEN 0 AND 65535)\n);\nCREATE TABLE zip_local_extra_field_byte (\n  id INTEGER PRIMARY KEY,\n  extra_field_id INTEGER NOT NULL REFERENCES zip_local_extra_field(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE zip_central_extra_field (\n  id INTEGER PRIMARY KEY,\n  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  tag INTEGER NOT NULL CHECK (tag BETWEEN 0 AND 65535)\n);\nCREATE TABLE zip_central_extra_field_byte (\n  id INTEGER PRIMARY KEY,\n  extra_field_id INTEGER NOT NULL REFERENCES zip_central_extra_field(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\n";
function integer(value: number, maximum = 65535): bigint { if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("ZIP integer exceeds its declared width"); return BigInt(value); }
function read(row: SqliteRow, index: number, maximum = 65535): number { const value = artifactSqliteInteger(row,index); if (value < 0n || value > BigInt(maximum)) throw new Error("ZIP integer exceeds its declared width"); return Number(value); }
function flag(value: boolean): bigint { if (typeof value !== "boolean") throw new Error("ZIP policy flag must be boolean"); return value ? 1n : 0n; }
async function emitBytes(out: ArtifactSqliteProjection, name: string, parent: bigint, bytes: readonly number[]): Promise<void> { for (let index = 0; index < bytes.length; index++) await out.insert(name,[parent,BigInt(index),integer(bytes[index]!,255)]); }
async function emitFields(out: ArtifactSqliteProjection, name: string, byteName: string, parent: bigint, fields: readonly ZipExtraField[]): Promise<void> { for (let index = 0; index < fields.length; index++) { const field = fields[index]!; const id = await out.insert(name,[parent,BigInt(index),integer(field.id)]); await emitBytes(out,byteName,id,field.data); } }

async function groups(rows: readonly SqliteRow[], parents: ReadonlySet<bigint>, options: ArtifactSqliteOptions): Promise<Map<bigint,SqliteRow[]>> {
  const result = new Map<bigint,SqliteRow[]>();
  for (let index = 0; index < rows.length; index++) { const row = rows[index]!; const parent = artifactSqliteInteger(row,1); if (row.rowid <= 0n || !parents.has(parent)) throw new Error("ZIP entity has invalid identity or parent"); const siblings = result.get(parent) ?? []; siblings.push(row); result.set(parent,siblings); if ((index+1)%256===0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,rows.length); }
  for (const [parent,siblings] of result) result.set(parent,artifactSqliteOrderedRows(siblings,2));
  return result;
}
async function readBytes(rows: readonly SqliteRow[], parents: ReadonlySet<bigint>, options: ArtifactSqliteOptions): Promise<Map<bigint,number[]>> {
  const result = new Map<bigint,number[]>();
  for (const [parent,siblings] of await groups(rows,parents,options)) { const bytes: number[] = []; for (let index = 0; index < siblings.length; index++) { bytes.push(read(siblings[index]!,3,255)); if ((index+1)%256===0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,siblings.length); } result.set(parent,bytes); }
  return result;
}
async function readFields(rows: readonly SqliteRow[], byteRows: readonly SqliteRow[], parents: ReadonlySet<bigint>, options: ArtifactSqliteOptions): Promise<Map<bigint,ZipExtraField[]>> {
  const grouped = await groups(rows,parents,options);
  const keys = new Set(rows.map(row=>row.rowid));
  const bytes = await readBytes(byteRows,keys,options);
  const result = new Map<bigint,ZipExtraField[]>();
  for (const [parent,siblings] of grouped) { const fields: ZipExtraField[] = []; for (let index = 0; index < siblings.length; index++) { const row = siblings[index]!; fields.push({id:read(row,3),data:bytes.get(row.rowid)??[]}); if ((index+1)%256===0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,siblings.length); } result.set(parent,fields); }
  return result;
}
async function headers(rows: readonly SqliteRow[], parents: ReadonlySet<bigint>, options: ArtifactSqliteOptions): Promise<Map<bigint,SqliteRow>> {
  const result = new Map<bigint,SqliteRow>();
  for (let index = 0; index < rows.length; index++) { const row = rows[index]!; if (!parents.has(row.rowid) || result.has(row.rowid)) throw new Error("ZIP header has an unknown or repeated member"); result.set(row.rowid,row); if ((index+1)%256===0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,rows.length); }
  if (result.size !== parents.size) throw new Error("ZIP must contain exactly one header per member");
  return result;
}
function optionalBytes(present: boolean, values: ReadonlyMap<bigint,number[]>, id: bigint): number[]|undefined { const bytes = values.get(id)??[]; if (present) return bytes; if (bytes.length) throw new Error("ZIP absent legacy text cannot own bytes"); return undefined; }

/** 📤️ Projects complete ordered member content and both independent header policies. */
export async function zipSnapshotToSqliteDatabase(snapshot: ZipSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const out = await ArtifactSqliteProjection.create(ZIP_SQLITE_SCHEMA,options);
  await out.insert("zip_archive",[snapshot.schema,snapshot.comment,flag(snapshot.commentUtf8)]);
  for (let index = 0; index < snapshot.entries.length; index++) {
    const entry = snapshot.entries[index]!;
    const id = await out.insert("zip_entry",[1n,BigInt(index),entry.name,integer(entry.metadata.compressionMethod),flag(entry.metadata.dataDescriptorSignature)]);
    await emitBytes(out,"zip_entry_byte",id,entry.data);
    const local = entry.metadata.local;
    await out.insert("zip_local_header",[integer(local.versionNeeded),integer(local.flags),integer(local.modifiedTime),integer(local.modifiedDate),flag(local.unicodePathLegacyName!==undefined)],id);
    if (local.unicodePathLegacyName!==undefined) await emitBytes(out,"zip_local_legacy_name_byte",id,local.unicodePathLegacyName);
    await emitFields(out,"zip_local_extra_field","zip_local_extra_field_byte",id,local.extraFields);
    const central = entry.metadata.central;
    await out.insert("zip_central_header",[integer(central.versionMadeBy),integer(central.versionNeeded),integer(central.flags),integer(central.modifiedTime),integer(central.modifiedDate),flag(central.unicodePathLegacyName!==undefined),central.comment,flag(central.unicodeCommentLegacy!==undefined),integer(central.internalAttributes),integer(central.externalAttributes,4294967295)],id);
    if (central.unicodePathLegacyName!==undefined) await emitBytes(out,"zip_central_legacy_name_byte",id,central.unicodePathLegacyName);
    if (central.unicodeCommentLegacy!==undefined) await emitBytes(out,"zip_central_legacy_comment_byte",id,central.unicodeCommentLegacy);
    await emitFields(out,"zip_central_extra_field","zip_central_extra_field_byte",id,central.extraFields);
  }
  return out.finish();
}

/** 📥️ Reconstructs exact header fields, optional legacy bytes and ordered duplicate-named members. */
export async function zipSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<ZipSnapshot> {
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
  const tables = await artifactSqliteTables(database,ZIP_SQLITE_SCHEMA,options);
  const archive = artifactSqliteDocument(tables[0]!);
  const entries = (await groups(tables[1]!,new Set([1n]),options)).get(1n)??[];
  const keys = new Set(entries.map(row=>row.rowid));
  const data = await readBytes(tables[2]!,keys,options);
  const localHeaders = await headers(tables[3]!,keys,options);
  const localNames = await readBytes(tables[4]!,keys,options);
  const centralHeaders = await headers(tables[5]!,keys,options);
  const centralNames = await readBytes(tables[6]!,keys,options);
  const centralComments = await readBytes(tables[7]!,keys,options);
  const localFields = await readFields(tables[8]!,tables[9]!,keys,options);
  const centralFields = await readFields(tables[10]!,tables[11]!,keys,options);
  const result: ZipSnapshot = {schema:artifactSqliteText(archive,1),comment:artifactSqliteText(archive,2),commentUtf8:artifactSqliteBoolean(archive,3),entries:[]};
  for (let index = 0; index < entries.length; index++) {
    const row = entries[index]!;
    const id = row.rowid;
    const local = localHeaders.get(id)!;
    const central = centralHeaders.get(id)!;
    const compressionMethod = read(row,4);
    result.entries.push({name:artifactSqliteText(row,3),data:data.get(id)??[],metadata:{
      compressionMethod,dataDescriptorSignature:artifactSqliteBoolean(row,5),
      local:{versionNeeded:read(local,1),flags:read(local,2),modifiedTime:read(local,3),modifiedDate:read(local,4),extraFields:localFields.get(id)??[],unicodePathLegacyName:optionalBytes(artifactSqliteBoolean(local,5),localNames,id)},
      central:{versionMadeBy:read(central,1),versionNeeded:read(central,2),flags:read(central,3),modifiedTime:read(central,4),modifiedDate:read(central,5),extraFields:centralFields.get(id)??[],unicodePathLegacyName:optionalBytes(artifactSqliteBoolean(central,6),centralNames,id),comment:artifactSqliteText(central,7),unicodeCommentLegacy:optionalBytes(artifactSqliteBoolean(central,8),centralComments,id),internalAttributes:read(central,9),externalAttributes:read(central,10,4294967295)}
    }});
    if ((index+1)%256===0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,entries.length);
  }
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",entries.length,entries.length);
  return result;
}

import type {ArtifactDialect} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import {checkZipIso21320Conformance,type ZipIso21320Diagnostic} from "../../../../🌐️iso21320/🧬️schema/🟦️.ts";

/** 🛡️ Validates the exact owned coordinate and returns its typed semantic diagnostics. */
export async function zipSnapshotValidateSqliteSubset(snapshot:ZipSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly ZipIso21320Diagnostic[]>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  if(dialect.artifactKind!=="s.stdio.zip"||dialect.standard!=="2.0"||(dialect.subset!=="*"&&dialect.subset!=="iso21320"))throw new Error("ZIP snapshot dialect is not owned");
  const tables=await artifactSqliteTables(database,ZIP_SQLITE_SCHEMA,options);
  const archive=artifactSqliteDocument(tables[0]!);
  if(artifactSqliteText(archive,1)!==snapshot.schema)throw new Error("ZIP semantic subset document identity disagrees with its snapshot");
  for(let index=0;index<snapshot.entries.length;index++){
    integer(snapshot.entries[index]!.metadata.compressionMethod);
    if((index+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",index+1,snapshot.entries.length);
  }
  return dialect.subset==="iso21320"?checkZipIso21320Conformance(snapshot,new NativeDecodeControl(0,event=>{options.onProgress?.({phase:"projectSnapshot",completed:event.completed,total:event.total});return true;},options.signal)):[];
}
