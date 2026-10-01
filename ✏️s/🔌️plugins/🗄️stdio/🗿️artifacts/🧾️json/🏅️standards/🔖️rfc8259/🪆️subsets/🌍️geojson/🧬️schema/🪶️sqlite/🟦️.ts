/** 🌍️ Borrowed RFC7946 semantic admission preserves the shared JSON syntax snapshot. */
import type { JsonSnapshot, JsonValue } from "../../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteInteger, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
export interface GeoJsonSqliteDiagnostic { readonly code: string; readonly severity: "error" | "warning"; readonly message: string }
class GeoJsonError extends Error { constructor(path: string, message: string) { super(`GeoJSON ${path || "/"}: ${message}`); } }
function fail(path: string, message: string): never { throw new GeoJsonError(path, message); }
function text(value: JsonValue | undefined): string | undefined { return value?.kind === "string" ? value.value : undefined; }
function items(value: JsonValue | undefined): readonly JsonValue[] | undefined { return value?.kind === "array" ? value.items : undefined; }
function number(value: JsonValue): number | undefined { if (value.kind !== "number") return; const result = Number(value.lexeme); return Number.isFinite(result) ? result : undefined; }
async function sourceCrs(root: JsonValue, readMember: (value: JsonValue | undefined, key: string) => Promise<JsonValue | undefined>): Promise< "rfc7946" | "declaredCrs84" | "webMercator"> {
 const crs = await readMember(root, "crs"); if (!crs) return "rfc7946";
 if (text(await readMember(crs, "type")) !== "name") fail("/crs", "only a GJ2008 named crs can be honoured; linked or untyped CRS objects are refused (RFC 7946 §4 is WGS 84)");
 const name = text(await readMember(await readMember(crs, "properties"), "name")); if (name === undefined) fail("/crs/properties/name", "a named crs carries its name as a string");
 const code = name.trim().replace(/[a-z]/g, value => value.toUpperCase()).replaceAll("URN:OGC:DEF:CRS:", "").replaceAll("::", ":").replaceAll(":1.3:", ":");
 if (["OGC:CRS84", "CRS84", "EPSG:4326"].includes(code)) return "declaredCrs84";
 if (["EPSG:3857", "EPSG:900913", "EPSG:102100", "EPSG:102113"].includes(code)) return "webMercator";
 return fail("/crs/properties/name", `coordinate reference system \`${name}\` is not WGS 84 and has no exact reprojection here; reproject to WGS 84 (RFC 7946 §4) first`);
}
/** 🛡️ Cancellable actual-tree conformance without a GeoJSON wire model or properties clone. */
export async function checkGeoJsonConformanceControlled(snapshot: JsonSnapshot, options: ArtifactSqliteOptions = {}): Promise<readonly GeoJsonSqliteDiagnostic[]> {
 let steps = 0;
 const tick = async (): Promise<void> => { if (++steps % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", steps, 0); };
 await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0, false);
 const stack: { value: JsonValue; index: number }[] = [{ value: snapshot.value, index: -1 }], active = new Set<JsonValue>();
 while (stack.length) {
  const frame = stack.at(-1)!; if (frame.index === -1) { if (active.has(frame.value)) throw Error("GeoJSON typed JSON contains a cycle"); active.add(frame.value); if (steps >= (options.maxRows ?? 1_000_000)) throw Error("GeoJSON conformance row limit"); await tick(); frame.index = 0; }
  const child = frame.value.kind === "array" ? frame.value.items[frame.index] : frame.value.kind === "object" ? frame.value.members[frame.index]?.value : undefined;
  if (child) { frame.index++; stack.push({ value: child, index: -1 }); } else { active.delete(frame.value); stack.pop(); }
 }
 let rings = 0;
 try {
  const readMember = async (value: JsonValue | undefined, key: string): Promise<JsonValue | undefined> => { if (value?.kind === "object") for (let i = value.members.length - 1; i >= 0; i--) { await tick(); if (value.members[i]!.key === key) return value.members[i]!.value; } };
  const crs = await sourceCrs(snapshot.value, readMember);
  const position = async (value: JsonValue, path: string): Promise<readonly [number, number]> => {
   const coordinates = items(value); if (!coordinates) fail(path, "a position is an array of numbers"); if (coordinates.length < 2) fail(path, "a position has at least longitude and latitude");
   for (let i = 0; i < coordinates.length; i++) { await tick(); if (number(coordinates[i]!) === undefined) fail(`${path}/${i}`, "a coordinate is a finite number"); }
   let x = number(coordinates[0]!)!, y = number(coordinates[1]!)!; if (crs === "webMercator") { x = x / 6378137 * 180 / Math.PI; y = (2 * Math.atan(Math.exp(y / 6378137)) - Math.PI / 2) * 180 / Math.PI; }
   if (!(x >= -180 && x <= 180 && y >= -90 && y <= 90)) fail(path, `[${x}, ${y}] is not a WGS 84 longitude/latitude (lon ∈ [−180, 180], lat ∈ [−90, 90])`); return [x, y];
  };
  const positions = async (value: JsonValue, path: string, minimum: number, what: string): Promise<void> => { const list = items(value); if (!list) fail(path, `${what} coordinates are an array of positions`); if (list.length < minimum) fail(path, `${what} needs at least ${minimum} positions`); for (let i = 0; i < list.length; i++) await position(list[i]!, `${path}/${i}`); };
  const polygon = async (value: JsonValue, path: string): Promise<void> => {
   const list = items(value); if (!list) fail(path, "Polygon coordinates are an array of linear rings"); if (!list.length) fail(path, "a Polygon has an exterior ring");
   for (let i = 0; i < list.length; i++) {
    const ringPath = `${path}/${i}`, ring = items(list[i]!); if (!ring) fail(ringPath, "a linear ring coordinates are an array of positions"); if (ring.length < 4) fail(ringPath, "a linear ring needs at least 4 positions");
    let area = 0, first: readonly [number, number] | undefined, previous: readonly [number, number] | undefined;
    for (let j = 0; j < ring.length; j++) { const point = await position(ring[j]!, `${ringPath}/${j}`); first ??= point; if (previous) area += previous[0] * point[1] - point[0] * previous[1]; previous = point; }
    const begin = items(ring[0]!)!, end = items(ring.at(-1)!)!; let closed = begin.length === end.length && first![0] === previous![0] && first![1] === previous![1]; for (let j = 2; closed && j < begin.length; j++) { await tick(); closed = number(begin[j]!) === number(end[j]!); }
    if (!closed) fail(ringPath, "a linear ring is closed: its first and last positions are identical (RFC 7946 §3.1.6)"); if ((area > 0) !== (i === 0)) rings++;
   }
  };
  const geometry = async (value: JsonValue, path: string, depth: number): Promise<void> => {
   await tick(); if (depth > 32) fail(path, "GeometryCollections nest deeper than 32"); const kind = text(await readMember(value, "type")); if (kind === undefined) fail(path, "a geometry has a string `type`");
   if (kind === "GeometryCollection") { const children = items(await readMember(value, "geometries")); if (!children) fail(`${path}/geometries`, "a GeometryCollection has a `geometries` array"); for (let i = 0; i < children.length; i++) await geometry(children[i]!, `${path}/geometries/${i}`, depth + 1); return; }
   const at = `${path}/coordinates`, coordinates = await readMember(value, "coordinates"); if (!coordinates) fail(at, `a ${kind} has \`coordinates\``);
   if (kind === "Point") await position(coordinates, at); else if (kind === "MultiPoint" || kind === "LineString") await positions(coordinates, at, kind === "LineString" ? 2 : 0, `a ${kind}`); else if (kind === "Polygon") await polygon(coordinates, at); else if (kind === "MultiLineString" || kind === "MultiPolygon") { const parts = items(coordinates); if (!parts) fail(at, `a ${kind} coordinates are an array`); for (let i = 0; i < parts.length; i++) if (kind === "MultiPolygon") await polygon(parts[i]!, `${at}/${i}`); else await positions(parts[i]!, `${at}/${i}`, 2, "a LineString"); } else fail(`${path}/type`, `\`${kind}\` is not an RFC 7946 geometry type`);
  };
  const feature = async (value: JsonValue, path: string): Promise<void> => {
   if (text(await readMember(value, "type")) !== "Feature") fail(`${path}/type`, 'a feature has `"type": "Feature"`'); const id = await readMember(value, "id"); if (id && id.kind !== "string") { if (id.kind !== "number") fail(`${path}/id`, "a feature id is a string or a number (RFC 7946 §3.2)"); if (number(id) === undefined) fail(`${path}/id`, "a numeric feature id is a finite number"); }
   const part = await readMember(value, "geometry"); if (!part) fail(`${path}/geometry`, "a feature has a `geometry` member (an object or null)"); if (part.kind !== "null") await geometry(part, `${path}/geometry`, 0);
   const properties = await readMember(value, "properties"); if (!properties) fail(`${path}/properties`, "a feature has a `properties` member (an object or null)"); if (properties.kind !== "null" && properties.kind !== "object") fail(`${path}/properties`, "feature properties are an object or null");
  };
  const kind = text(await readMember(snapshot.value, "type")); if (kind === "FeatureCollection") { const features = items(await readMember(snapshot.value, "features")); if (!features) fail("/features", "a FeatureCollection has a `features` array"); for (let i = 0; i < features.length; i++) await feature(features[i]!, `/features/${i}`); } else if (kind === "Feature") await feature(snapshot.value, ""); else if (kind === undefined) fail("/type", "a GeoJSON object has a string `type`"); else await geometry(snapshot.value, "", 0);
  await artifactSqliteCheckpoint(options, "projectSnapshot", steps, steps, false);
  const diagnostics: GeoJsonSqliteDiagnostic[] = []; if (crs !== "rfc7946") diagnostics.push({ code: "stdio.json.geojson.legacy-crs", severity: "warning", message: "the GJ2008 `crs` member was removed by RFC 7946 §4; coordinates are WGS 84 by definition" }); if (rings) diagnostics.push({ code: "stdio.json.geojson.left-handed-ring", severity: "warning", message: `${rings} linear ring(s) do not follow the right-hand rule (RFC 7946 §3.1.6)` }); return diagnostics;
 } catch (error) { if (!(error instanceof GeoJsonError)) throw error; return [{ code: "stdio.json.geojson.not-rfc7946", severity: "error", message: error.message }]; }
}
/** 🧭️ Exact GeoJSON coordinate and owned projection identity. */
export async function validateGeoJsonSnapshotSqliteDialect(snapshot: JsonSnapshot, dialect: ArtifactDialect, database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<readonly GeoJsonSqliteDiagnostic[]> {
 await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0, false);
 if (dialect.artifactKind !== "s.stdio.json" || dialect.standard !== "rfc8259" || dialect.subset !== "geojson") throw Error("GeoJSON does not own this semantic subset");
 const table = database.tables.find(table => table.name.toLowerCase() === "json_document"); if (table?.rows.length !== 1 || table.rows[0]!.rowid !== 1n || artifactSqliteInteger(table.rows[0]!, 0) !== 1n || artifactSqliteText(table.rows[0]!, 1) !== snapshot.schema) throw Error("GeoJSON document identity disagrees with its snapshot");
 return checkGeoJsonConformanceControlled(snapshot, options);
}
