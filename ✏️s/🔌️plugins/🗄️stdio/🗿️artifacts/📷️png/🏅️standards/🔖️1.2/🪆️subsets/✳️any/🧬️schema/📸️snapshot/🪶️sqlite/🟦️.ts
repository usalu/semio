/** 📷️ PNG typed ancillary entities, RGBA coordinates and intrinsic unknown-chunk octets. */
import type { PngSnapshot, PngColorType, PngSrgbIntent, PngChunkMarker, PngTextKind } from "../🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { sqliteValueByteLength, type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted sixteen-table schema byte-equal to the adjacent SQL asset. */
export const PNG_SQLITE_SCHEMA = "CREATE TABLE png_document (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295),\n  height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),\n  bit_depth INTEGER NOT NULL CHECK (bit_depth BETWEEN 0 AND 255),\n  color_type INTEGER NOT NULL CHECK (color_type IN (0, 2, 3, 4, 6)),\n  interlace INTEGER NOT NULL CHECK (interlace IN (0, 1))\n);\nCREATE TABLE png_palette (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id)\n);\nCREATE TABLE png_palette_entry (\n  id INTEGER PRIMARY KEY,\n  palette_id INTEGER NOT NULL REFERENCES png_palette(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),\n  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),\n  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)\n);\nCREATE TABLE png_transparency (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  color_model TEXT NOT NULL CHECK (color_model IN ('indexed', 'grayscale', 'rgb')),\n  gray INTEGER CHECK (gray BETWEEN 0 AND 65535),\n  red INTEGER CHECK (red BETWEEN 0 AND 65535),\n  green INTEGER CHECK (green BETWEEN 0 AND 65535),\n  blue INTEGER CHECK (blue BETWEEN 0 AND 65535),\n  CHECK ((color_model = 'indexed' AND gray IS NULL AND red IS NULL AND green IS NULL AND blue IS NULL) OR (color_model = 'grayscale' AND gray IS NOT NULL AND red IS NULL AND green IS NULL AND blue IS NULL) OR (color_model = 'rgb' AND gray IS NULL AND red IS NOT NULL AND green IS NOT NULL AND blue IS NOT NULL))\n);\nCREATE TABLE png_transparency_alpha (\n  id INTEGER PRIMARY KEY,\n  transparency_id INTEGER NOT NULL REFERENCES png_transparency(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)\n);\nCREATE TABLE png_gamma (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  gamma_times_100000 INTEGER NOT NULL CHECK (gamma_times_100000 BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE png_chromaticity (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  white_x INTEGER NOT NULL CHECK (white_x BETWEEN 0 AND 4294967295),\n  white_y INTEGER NOT NULL CHECK (white_y BETWEEN 0 AND 4294967295),\n  red_x INTEGER NOT NULL CHECK (red_x BETWEEN 0 AND 4294967295),\n  red_y INTEGER NOT NULL CHECK (red_y BETWEEN 0 AND 4294967295),\n  green_x INTEGER NOT NULL CHECK (green_x BETWEEN 0 AND 4294967295),\n  green_y INTEGER NOT NULL CHECK (green_y BETWEEN 0 AND 4294967295),\n  blue_x INTEGER NOT NULL CHECK (blue_x BETWEEN 0 AND 4294967295),\n  blue_y INTEGER NOT NULL CHECK (blue_y BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE png_srgb (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  rendering_intent INTEGER NOT NULL CHECK (rendering_intent BETWEEN 0 AND 3)\n);\nCREATE TABLE png_physical_dimensions (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  pixels_per_unit_x INTEGER NOT NULL CHECK (pixels_per_unit_x BETWEEN 0 AND 4294967295),\n  pixels_per_unit_y INTEGER NOT NULL CHECK (pixels_per_unit_y BETWEEN 0 AND 4294967295),\n  unit_is_meter INTEGER NOT NULL CHECK (unit_is_meter IN (0, 1))\n);\nCREATE TABLE png_modification_time (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  year INTEGER NOT NULL CHECK (year BETWEEN 0 AND 65535),\n  month INTEGER NOT NULL CHECK (month BETWEEN 0 AND 255),\n  day INTEGER NOT NULL CHECK (day BETWEEN 0 AND 255),\n  hour INTEGER NOT NULL CHECK (hour BETWEEN 0 AND 255),\n  minute INTEGER NOT NULL CHECK (minute BETWEEN 0 AND 255),\n  second INTEGER NOT NULL CHECK (second BETWEEN 0 AND 255)\n);\nCREATE TABLE png_background (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  color_model TEXT NOT NULL CHECK (color_model IN ('indexed', 'grayscale', 'rgb')),\n  gray INTEGER CHECK (gray BETWEEN 0 AND 65535),\n  red INTEGER CHECK (red BETWEEN 0 AND 65535),\n  green INTEGER CHECK (green BETWEEN 0 AND 65535),\n  blue INTEGER CHECK (blue BETWEEN 0 AND 65535),\n  palette_index INTEGER CHECK (palette_index BETWEEN 0 AND 255),\n  CHECK ((color_model = 'indexed' AND gray IS NULL AND red IS NULL AND green IS NULL AND blue IS NULL AND palette_index IS NOT NULL) OR (color_model = 'grayscale' AND gray IS NOT NULL AND red IS NULL AND green IS NULL AND blue IS NULL AND palette_index IS NULL) OR (color_model = 'rgb' AND gray IS NULL AND red IS NOT NULL AND green IS NOT NULL AND blue IS NOT NULL AND palette_index IS NULL))\n);\nCREATE TABLE png_text (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  keyword TEXT NOT NULL,\n  content TEXT NOT NULL,\n  compressed INTEGER NOT NULL CHECK (compressed IN (0, 1)),\n  chunk_kind TEXT NOT NULL CHECK (chunk_kind IN ('text', 'ztext', 'itext')),\n  language_tag TEXT NOT NULL,\n  translated_keyword TEXT NOT NULL\n);\nCREATE TABLE png_pixel (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  x INTEGER NOT NULL CHECK (x BETWEEN 0 AND 4294967295),\n  y INTEGER NOT NULL CHECK (y BETWEEN 0 AND 4294967295),\n  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),\n  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),\n  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255),\n  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)\n);\nCREATE TABLE png_unknown_chunk (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  kind_octet_1 INTEGER NOT NULL CHECK (kind_octet_1 BETWEEN 0 AND 255),\n  kind_octet_2 INTEGER NOT NULL CHECK (kind_octet_2 BETWEEN 0 AND 255),\n  kind_octet_3 INTEGER NOT NULL CHECK (kind_octet_3 BETWEEN 0 AND 255),\n  kind_octet_4 INTEGER NOT NULL CHECK (kind_octet_4 BETWEEN 0 AND 255)\n);\nCREATE TABLE png_unknown_chunk_byte (\n  id INTEGER PRIMARY KEY,\n  unknown_chunk_id INTEGER NOT NULL REFERENCES png_unknown_chunk(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\nCREATE TABLE png_chunk_sequence (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES png_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  chunk_kind TEXT NOT NULL CHECK (chunk_kind IN ('ihdr', 'plte', 'trns', 'gama', 'chrm', 'srgb', 'phys', 'time', 'bkgd', 'idat', 'iend', 'text', 'unknown')),\n  text_id INTEGER REFERENCES png_text(id),\n  unknown_chunk_id INTEGER REFERENCES png_unknown_chunk(id),\n  CHECK ((chunk_kind = 'text' AND text_id IS NOT NULL AND unknown_chunk_id IS NULL) OR (chunk_kind = 'unknown' AND text_id IS NULL AND unknown_chunk_id IS NOT NULL) OR (chunk_kind NOT IN ('text', 'unknown') AND text_id IS NULL AND unknown_chunk_id IS NULL))\n);\n";
const TABLES = ["png_document","png_palette","png_palette_entry","png_transparency","png_transparency_alpha","png_gamma","png_chromaticity","png_srgb","png_physical_dimensions","png_modification_time","png_background","png_text","png_pixel","png_unknown_chunk","png_unknown_chunk_byte","png_chunk_sequence"] as const;
type Table = typeof TABLES[number];
const COLORS: Readonly<Record<PngColorType,number>> = { grayscale:0,rgb:2,palette:3,grayscaleAlpha:4,rgba:6 };
const INTENTS: Readonly<Record<PngSrgbIntent,number>> = { perceptual:0,relativeColorimetric:1,saturation:2,absoluteColorimetric:3 };
const TEXT: Readonly<Record<PngTextKind,string>> = { text:"text",zText:"ztext",iText:"itext" };
const MARKERS = ["ihdr","plte","trns","gama","chrm","srgb","phys","time","bkgd","idat","iend","text","unknown"] as const;
function integer(value: number, maximum=4294967295): bigint { if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("PNG scalar is outside its declared width"); return BigInt(value); }
function read(row: SqliteRow,column: number,maximum=4294967295): number { const value=artifactSqliteInteger(row,column); if(value<0n||value>BigInt(maximum))throw new Error("PNG scalar is outside its declared width");return Number(value); }
function text(value: string): string { artifactSqliteTextBytes(value); return value; }
function boolean(value: boolean): bigint { if(typeof value!=="boolean")throw new Error("PNG boolean must be typed");return value?1n:0n; }
function grid(width: number,height: number): number { integer(width);integer(height);const count=width*height;if(!Number.isSafeInteger(count)||!Number.isSafeInteger(count*4))throw new Error("PNG pixel count overflow");return count; }
function reference(index: number,length: number): bigint { integer(index,Math.min(length-1,Number.MAX_SAFE_INTEGER));return BigInt(index+1); }
type Emit = (table: Table,cells: SqliteValue[])=>Promise<bigint>;
async function visit(snapshot: PngSnapshot,emit: Emit): Promise<void> {
  const count=grid(snapshot.width,snapshot.height);
  if(snapshot.pixels.length!==count*4)throw new Error("PNG pixels require one RGBA tuple per grid position");
  const color=COLORS[snapshot.colorType];
  if(color===undefined)throw new Error("PNG color type is unknown");
  await emit("png_document",[text(snapshot.schema),integer(snapshot.width),integer(snapshot.height),integer(snapshot.bitDepth,255),BigInt(color),boolean(snapshot.interlace)]);
  if(snapshot.plte!==undefined) {
    await emit("png_palette",[1n]);
    for(let ordinal=0;ordinal<snapshot.plte.length;ordinal++) {const value=snapshot.plte[ordinal]!;await emit("png_palette_entry",[1n,BigInt(ordinal),integer(value.r,255),integer(value.g,255),integer(value.b,255)]);}
  }
  if(snapshot.trns!==undefined) {
    const value=snapshot.trns;
    if(value.colorType==="indexed") {
      await emit("png_transparency",[1n,"indexed",null,null,null,null]);
      for(let ordinal=0;ordinal<value.alpha.length;ordinal++)await emit("png_transparency_alpha",[1n,BigInt(ordinal),integer(value.alpha[ordinal]!,255)]);
    } else if(value.colorType==="grayscale")await emit("png_transparency",[1n,"grayscale",integer(value.gray,65535),null,null,null]);
    else if(value.colorType==="rgb")await emit("png_transparency",[1n,"rgb",null,integer(value.r,65535),integer(value.g,65535),integer(value.b,65535)]);
    else throw new Error("PNG transparency kind is unknown");
  }
  if(snapshot.gama!==undefined)await emit("png_gamma",[1n,integer(snapshot.gama)]);
  if(snapshot.chrm!==undefined) {
    const value=snapshot.chrm;
    await emit("png_chromaticity",[1n,integer(value.whiteX),integer(value.whiteY),integer(value.redX),integer(value.redY),integer(value.greenX),integer(value.greenY),integer(value.blueX),integer(value.blueY)]);
  }
  if(snapshot.srgb!==undefined) {const value=INTENTS[snapshot.srgb];if(value===undefined)throw new Error("PNG rendering intent is unknown");await emit("png_srgb",[1n,BigInt(value)]);}
  if(snapshot.phys!==undefined) {const value=snapshot.phys;await emit("png_physical_dimensions",[1n,integer(value.ppuX),integer(value.ppuY),boolean(value.unitIsMeter)]);}
  if(snapshot.time!==undefined) {const value=snapshot.time;await emit("png_modification_time",[1n,integer(value.year,65535),integer(value.month,255),integer(value.day,255),integer(value.hour,255),integer(value.minute,255),integer(value.second,255)]);}
  if(snapshot.bkgd!==undefined) {
    const value=snapshot.bkgd;
    if(value.colorType==="indexed")await emit("png_background",[1n,"indexed",null,null,null,null,integer(value.index,255)]);
    else if(value.colorType==="grayscale")await emit("png_background",[1n,"grayscale",integer(value.gray,65535),null,null,null,null]);
    else if(value.colorType==="rgb")await emit("png_background",[1n,"rgb",null,integer(value.r,65535),integer(value.g,65535),integer(value.b,65535),null]);
    else throw new Error("PNG background kind is unknown");
  }
  for(let ordinal=0;ordinal<snapshot.textChunks.length;ordinal++) {
    const value=snapshot.textChunks[ordinal]!,kind=TEXT[value.kind];
    if(kind===undefined)throw new Error("PNG text kind is unknown");
    await emit("png_text",[1n,BigInt(ordinal),text(value.keyword),text(value.value),boolean(value.compressed),kind,text(value.languageTag),text(value.translatedKeyword)]);
  }
  for(let ordinal=0;ordinal<count;ordinal++) {
    const offset=ordinal*4;
    await emit("png_pixel",[1n,BigInt(ordinal%snapshot.width),BigInt(Math.floor(ordinal/snapshot.width)),integer(snapshot.pixels[offset]!,255),integer(snapshot.pixels[offset+1]!,255),integer(snapshot.pixels[offset+2]!,255),integer(snapshot.pixels[offset+3]!,255)]);
  }
  for(let ordinal=0;ordinal<snapshot.unknownChunks.length;ordinal++) {
    const value=snapshot.unknownChunks[ordinal]!;
    if(value.kind.length!==4)throw new Error("PNG unknown chunk requires four type octets");
    const id=await emit("png_unknown_chunk",[1n,BigInt(ordinal),...value.kind.map(value=>integer(value,255))]);
    for(let ordinal=0;ordinal<value.data.length;ordinal++)await emit("png_unknown_chunk_byte",[id,BigInt(ordinal),integer(value.data[ordinal]!,255)]);
  }
  for(let ordinal=0;ordinal<snapshot.chunkOrder.length;ordinal++) {
    const marker=snapshot.chunkOrder[ordinal]!;
    if(!MARKERS.includes(marker.chunk))throw new Error("PNG chunk marker is unknown");
    await emit("png_chunk_sequence",[1n,BigInt(ordinal),marker.chunk,marker.chunk==="text"?reference(marker.index,snapshot.textChunks.length):null,marker.chunk==="unknown"?reference(marker.index,snapshot.unknownChunks.length):null]);
  }
}

/** 📤️ Project all persisted PNG semantic fields after bounded cancellable entity preflight. */
export async function pngSnapshotToSqliteDatabase(snapshot: PngSnapshot,options: ArtifactSqliteOptions={}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  const counts=new Map<Table,number>(TABLES.map(name=>[name,0]));
  let total=0,bytes=0;
  await visit(snapshot,async(name,cells)=>{
    if(++total>(options.maxRows??1_000_000))throw new Error("PNG SQLite row limit");
    bytes+=8;for(const value of cells){bytes+=sqliteValueByteLength(value);artifactSqliteValueBudget(bytes,options);}
    const count=counts.get(name)!+1;counts.set(name,count);
    if(total%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",0,total);
    return BigInt(count);
  });
  const rows=new Map<Table,SqliteRow[]>(TABLES.map(name=>[name,[]]));
  let completed=0;
  await visit(snapshot,async(name,cells)=>{
    const table=rows.get(name)!,id=BigInt(table.length+1);table.push({rowid:id,values:[id,...cells]});
    if(++completed%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total);
    return id;
  });
  const database=artifactSqliteDatabase(PNG_SQLITE_SCHEMA,TABLES.map(name=>rows.get(name)!),options);
  await artifactSqliteCheckpoint(options,"projectSnapshot",total,total);return database;
}
function nulls(row: SqliteRow,columns: readonly number[]): void {if(columns.some(column=>row.values[column]!==null))throw new Error("PNG optional component has conflicting payloads");}

/** 📥️ Restore typed PNG optional components, exact chunk references and a complete canonical pixel grid. */
export async function pngSnapshotFromSqliteDatabase(database: SqliteDatabase,options: ArtifactSqliteOptions={}): Promise<PngSnapshot> {
  const total=database.tables.reduce((sum,table)=>sum+table.rows.length,0);
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,total);
  const tables=await artifactSqliteTables(database,PNG_SQLITE_SCHEMA,options);
  const rows=new Map<Table,readonly SqliteRow[]>(TABLES.map((name,index)=>[name,tables[index]!]));
  const document=artifactSqliteDocument(rows.get("png_document")!);
  let checked=0;
  const tick=async():Promise<void>=>{if(++checked%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",Math.min(checked,total),total);};
  const one=(name:Table):SqliteRow|undefined=>{
    const result=rows.get(name)!;if(!result.length)return undefined;
    const row=artifactSqliteDocument(result);artifactSqliteDocumentReference(row,1);return row;
  };
  const ordered=async(name:Table):Promise<SqliteRow[]>=>{
    const result=rows.get(name)!;for(const row of result){artifactSqliteDocumentReference(row,1);await tick();}
    return artifactSqliteOrderedRows(result,2);
  };
  const width=read(document,2),height=read(document,3),count=grid(width,height);
  const pixelRows=rows.get("png_pixel")!;
  if(count!==pixelRows.length)throw new Error("PNG grid requires exactly one pixel per position");
  const colorType=(Object.keys(COLORS) as PngColorType[]).find(key=>COLORS[key]===read(document,5,6));
  if(colorType===undefined)throw new Error("PNG color type is unknown");
  const snapshot:PngSnapshot={schema:artifactSqliteText(document,1),width,height,bitDepth:read(document,4,255),colorType,interlace:artifactSqliteBoolean(document,6),textChunks:[],pixels:new Array<number>(count*4),chunkOrder:[],unknownChunks:[]};
  const palette=one("png_palette");
  if(!palette&&rows.get("png_palette_entry")!.length)throw new Error("PNG palette entries lack their palette");
  if(palette) {
    snapshot.plte=[];
    for(const row of artifactSqliteOrderedRows(rows.get("png_palette_entry")!,2)){artifactSqliteDocumentReference(row,1);snapshot.plte.push({r:read(row,3,255),g:read(row,4,255),b:read(row,5,255)});await tick();}
  }
  const transparency=one("png_transparency"),alphas=rows.get("png_transparency_alpha")!;
  if(transparency) {
    const model=artifactSqliteText(transparency,2);
    if(model==="indexed") {
      nulls(transparency,[3,4,5,6]);snapshot.trns={colorType:model,alpha:[]};
      for(const row of artifactSqliteOrderedRows(alphas,2)){artifactSqliteDocumentReference(row,1);snapshot.trns.alpha.push(read(row,3,255));await tick();}
    } else if(model==="grayscale"){nulls(transparency,[4,5,6]);snapshot.trns={colorType:model,gray:read(transparency,3,65535)};}
    else if(model==="rgb"){nulls(transparency,[3]);snapshot.trns={colorType:model,r:read(transparency,4,65535),g:read(transparency,5,65535),b:read(transparency,6,65535)};}
    else throw new Error("PNG transparency kind is unknown");
  }
  if(alphas.length&&snapshot.trns?.colorType!=="indexed")throw new Error("PNG transparency alpha rows lack indexed ownership");
  const gamma=one("png_gamma");if(gamma)snapshot.gama=read(gamma,2);
  const chromaticity=one("png_chromaticity");if(chromaticity)snapshot.chrm={whiteX:read(chromaticity,2),whiteY:read(chromaticity,3),redX:read(chromaticity,4),redY:read(chromaticity,5),greenX:read(chromaticity,6),greenY:read(chromaticity,7),blueX:read(chromaticity,8),blueY:read(chromaticity,9)};
  const srgb=one("png_srgb");if(srgb)snapshot.srgb=(Object.keys(INTENTS) as PngSrgbIntent[]).find(key=>INTENTS[key]===read(srgb,2,3))!;
  const physical=one("png_physical_dimensions");if(physical)snapshot.phys={ppuX:read(physical,2),ppuY:read(physical,3),unitIsMeter:artifactSqliteBoolean(physical,4)};
  const time=one("png_modification_time");if(time)snapshot.time={year:read(time,2,65535),month:read(time,3,255),day:read(time,4,255),hour:read(time,5,255),minute:read(time,6,255),second:read(time,7,255)};
  const background=one("png_background");
  if(background) {
    const model=artifactSqliteText(background,2);
    if(model==="indexed"){nulls(background,[3,4,5,6]);snapshot.bkgd={colorType:model,index:read(background,7,255)};}
    else if(model==="grayscale"){nulls(background,[4,5,6,7]);snapshot.bkgd={colorType:model,gray:read(background,3,65535)};}
    else if(model==="rgb"){nulls(background,[3,7]);snapshot.bkgd={colorType:model,r:read(background,4,65535),g:read(background,5,65535),b:read(background,6,65535)};}
    else throw new Error("PNG background kind is unknown");
  }
  const textRows=await ordered("png_text"),textIndices=new Map<bigint,number>();
  for(const row of textRows) {
    const kind=(Object.keys(TEXT) as PngTextKind[]).find(key=>TEXT[key]===artifactSqliteText(row,6));
    if(kind===undefined)throw new Error("PNG text kind is unknown");
    textIndices.set(row.rowid,snapshot.textChunks.length);
    snapshot.textChunks.push({keyword:artifactSqliteText(row,3),value:artifactSqliteText(row,4),compressed:artifactSqliteBoolean(row,5),kind,languageTag:artifactSqliteText(row,7),translatedKeyword:artifactSqliteText(row,8)});await tick();
  }
  const seen=new Uint8Array(count);
  for(const row of pixelRows) {
    artifactSqliteDocumentReference(row,1);const x=read(row,2),y=read(row,3);
    if(x>=width||y>=height)throw new Error("PNG pixel coordinates exceed the grid");
    const position=y*width+x;if(seen[position])throw new Error("PNG pixel coordinates must be unique");seen[position]=1;
    for(let channel=0;channel<4;channel++)snapshot.pixels[position*4+channel]=read(row,4+channel,255);await tick();
  }
  const unknownRows=await ordered("png_unknown_chunk"),unknownIndices=new Map<bigint,number>(),byteGroups=new Map<bigint,SqliteRow[]>();
  const unknownIds=new Set(unknownRows.map(row=>row.rowid));
  for(const row of rows.get("png_unknown_chunk_byte")!) {
    const owner=artifactSqliteInteger(row,1);if(!unknownIds.has(owner))throw new Error("PNG unknown byte has a dangling owner");
    const list=byteGroups.get(owner)??[];list.push(row);byteGroups.set(owner,list);await tick();
  }
  for(const row of unknownRows) {
    unknownIndices.set(row.rowid,snapshot.unknownChunks.length);
    const data:number[]=[];
    for(const byte of artifactSqliteOrderedRows(byteGroups.get(row.rowid)??[],2)){data.push(read(byte,3,255));await tick();}
    snapshot.unknownChunks.push({kind:[read(row,3,255),read(row,4,255),read(row,5,255),read(row,6,255)],data});await tick();
  }
  for(const row of await ordered("png_chunk_sequence")) {
    const kind=artifactSqliteText(row,3);let marker:PngChunkMarker;
    if(kind==="text"){nulls(row,[5]);const index=textIndices.get(artifactSqliteInteger(row,4));if(index===undefined)throw new Error("PNG text marker is dangling");marker={chunk:kind,index};}
    else if(kind==="unknown"){nulls(row,[4]);const index=unknownIndices.get(artifactSqliteInteger(row,5));if(index===undefined)throw new Error("PNG unknown marker is dangling");marker={chunk:kind,index};}
    else {nulls(row,[4,5]);if(!MARKERS.includes(kind as PngChunkMarker["chunk"]))throw new Error("PNG chunk marker is unknown");marker={chunk:kind as Exclude<PngChunkMarker["chunk"],"text"|"unknown">};}
    snapshot.chunkOrder.push(marker);await tick();
  }
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",total,total);return snapshot;
}

