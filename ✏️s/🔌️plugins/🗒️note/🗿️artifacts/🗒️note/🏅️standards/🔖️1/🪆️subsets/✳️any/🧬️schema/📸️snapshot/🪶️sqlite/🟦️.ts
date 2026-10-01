/** 🗒️ Handcrafted Note relational entities and exact owned scalar identities. */
import type { NoteSnapshot, NoteBlockNode, NoteImageAsset, ArtifactLink } from "../🟦️.ts";
import type { NoteTextParagraph, NoteTextRun } from "../../🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteTables, artifactSqliteInteger as integer, artifactSqliteText as text, artifactSqliteBoolean as boolean, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { binary64, encodeIeee754Cells, readBinary64, ieee754IsNull, type Binary64, type Ieee754Cell, type Ieee754Column } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type { SqliteDatabase, SqliteRow, SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Language-neutral authored Note schema, shared with its native implementation. */
export const NOTE_SQLITE_SCHEMA = `CREATE TABLE note_document (id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL, artifact_id TEXT NOT NULL, title TEXT, grid_visible INTEGER CHECK (grid_visible IN (0,1)), grid_spacing REAL, grid_subdivisions REAL, grid_opacity REAL, snap_enabled INTEGER CHECK (snap_enabled IN (0,1)), snap_grid_spacing REAL, pencil_width REAL, eraser_radius REAL, grid_spacing_bits INTEGER, grid_spacing_class TEXT CHECK (grid_spacing_class IN ('finite','positiveInfinity','negativeInfinity','nan')), grid_subdivisions_bits INTEGER, grid_subdivisions_class TEXT CHECK (grid_subdivisions_class IN ('finite','positiveInfinity','negativeInfinity','nan')), grid_opacity_bits INTEGER, grid_opacity_class TEXT CHECK (grid_opacity_class IN ('finite','positiveInfinity','negativeInfinity','nan')), snap_grid_spacing_bits INTEGER, snap_grid_spacing_class TEXT CHECK (snap_grid_spacing_class IN ('finite','positiveInfinity','negativeInfinity','nan')), pencil_width_bits INTEGER, pencil_width_class TEXT CHECK (pencil_width_class IN ('finite','positiveInfinity','negativeInfinity','nan')), eraser_radius_bits INTEGER, eraser_radius_class TEXT CHECK (eraser_radius_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE note_asset (id INTEGER PRIMARY KEY, note_id INTEGER NOT NULL REFERENCES note_document(id), asset_key TEXT NOT NULL, mime TEXT NOT NULL, data TEXT NOT NULL, width REAL, height REAL, width_bits INTEGER, width_class TEXT CHECK (width_class IN ('finite','positiveInfinity','negativeInfinity','nan')), height_bits INTEGER, height_class TEXT CHECK (height_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE note_link (id INTEGER PRIMARY KEY CHECK (id = 1), note_id INTEGER NOT NULL REFERENCES note_document(id), artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL, role TEXT NOT NULL, pin_kind TEXT NOT NULL CHECK (pin_kind IN ('head','checkpoint','snapshot')), checkpoint_id TEXT, blob_hash TEXT, blob_size_high INTEGER CHECK (blob_size_high BETWEEN 0 AND 4294967295), blob_size_low INTEGER CHECK (blob_size_low BETWEEN 0 AND 4294967295), blob_media_type TEXT);
CREATE TABLE note_block (id INTEGER PRIMARY KEY, note_id INTEGER NOT NULL REFERENCES note_document(id), parent_id INTEGER REFERENCES note_block(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), kind TEXT NOT NULL CHECK (kind IN ('text','image','table','math','ink','group')), block_id TEXT NOT NULL, name TEXT NOT NULL, x REAL, y REAL, width REAL, height REAL, rotation REAL, visible INTEGER NOT NULL CHECK (visible IN (0,1)), locked INTEGER NOT NULL CHECK (locked IN (0,1)), x_bits INTEGER, x_class TEXT CHECK (x_class IN ('finite','positiveInfinity','negativeInfinity','nan')), y_bits INTEGER, y_class TEXT CHECK (y_class IN ('finite','positiveInfinity','negativeInfinity','nan')), width_bits INTEGER, width_class TEXT CHECK (width_class IN ('finite','positiveInfinity','negativeInfinity','nan')), height_bits INTEGER, height_class TEXT CHECK (height_class IN ('finite','positiveInfinity','negativeInfinity','nan')), rotation_bits INTEGER, rotation_class TEXT CHECK (rotation_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE note_text (id INTEGER PRIMARY KEY REFERENCES note_block(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL, font_size REAL, font_weight TEXT NOT NULL, align TEXT NOT NULL, font_size_bits INTEGER, font_size_class TEXT CHECK (font_size_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE note_paragraph (id INTEGER PRIMARY KEY, text_id INTEGER NOT NULL REFERENCES note_text(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0));
CREATE TABLE note_run (id INTEGER PRIMARY KEY, paragraph_id INTEGER NOT NULL REFERENCES note_paragraph(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), text TEXT NOT NULL, bold INTEGER CHECK (bold IN (0,1)), italic INTEGER CHECK (italic IN (0,1)), underline INTEGER CHECK (underline IN (0,1)), link TEXT);
CREATE TABLE note_image (id INTEGER PRIMARY KEY REFERENCES note_block(id), image_key TEXT NOT NULL, asset_id INTEGER REFERENCES note_asset(id));
CREATE TABLE note_column (id INTEGER PRIMARY KEY, block_id INTEGER NOT NULL REFERENCES note_block(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), label TEXT NOT NULL);
CREATE TABLE note_table_row (id INTEGER PRIMARY KEY, block_id INTEGER NOT NULL REFERENCES note_block(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0));
CREATE TABLE note_cell (id INTEGER PRIMARY KEY, table_row_id INTEGER NOT NULL REFERENCES note_table_row(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), content TEXT NOT NULL);
CREATE TABLE note_math (id INTEGER PRIMARY KEY REFERENCES note_block(id), tex TEXT NOT NULL, display_mode INTEGER NOT NULL CHECK (display_mode IN (0,1)));
CREATE TABLE note_ink (id INTEGER PRIMARY KEY REFERENCES note_block(id), stroke_width REAL, red REAL, green REAL, blue REAL, alpha REAL, stroke_width_bits INTEGER, stroke_width_class TEXT CHECK (stroke_width_class IN ('finite','positiveInfinity','negativeInfinity','nan')), red_bits INTEGER, red_class TEXT CHECK (red_class IN ('finite','positiveInfinity','negativeInfinity','nan')), green_bits INTEGER, green_class TEXT CHECK (green_class IN ('finite','positiveInfinity','negativeInfinity','nan')), blue_bits INTEGER, blue_class TEXT CHECK (blue_class IN ('finite','positiveInfinity','negativeInfinity','nan')), alpha_bits INTEGER, alpha_class TEXT CHECK (alpha_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE note_point (id INTEGER PRIMARY KEY, ink_id INTEGER NOT NULL REFERENCES note_ink(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), x REAL, y REAL, x_bits INTEGER, x_class TEXT CHECK (x_class IN ('finite','positiveInfinity','negativeInfinity','nan')), y_bits INTEGER, y_class TEXT CHECK (y_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
`;

const tables = ["note_document", "note_asset", "note_link", "note_block", "note_text", "note_paragraph", "note_run", "note_image", "note_column", "note_table_row", "note_cell", "note_math", "note_ink", "note_point"] as const;
type Table = typeof tables[number];
const widths: Readonly<Record<Table, number>> = { note_document: 12, note_asset: 7, note_link: 13, note_block: 14, note_text: 9, note_paragraph: 3, note_run: 8, note_image: 3, note_column: 4, note_table_row: 3, note_cell: 4, note_math: 3, note_ink: 6, note_point: 5 };
const columns: Readonly<Partial<Record<Table, readonly Ieee754Column[]>>> = {
  note_document: [{ index: 5, width: 64 }, { index: 6, width: 64 }, { index: 7, width: 64 }, { index: 9, width: 64 }, { index: 10, width: 64 }, { index: 11, width: 64 }],
  note_asset: [{ index: 5, width: 64 }, { index: 6, width: 64 }],
  note_block: [{ index: 7, width: 64 }, { index: 8, width: 64 }, { index: 9, width: 64 }, { index: 10, width: 64 }, { index: 11, width: 64 }],
  note_text: [{ index: 6, width: 64 }],
  note_ink: [{ index: 1, width: 64 }, { index: 2, width: 64 }, { index: 3, width: 64 }, { index: 4, width: 64 }, { index: 5, width: 64 }],
  note_point: [{ index: 3, width: 64 }, { index: 4, width: 64 }],
};
function flag(value: boolean): bigint { if (typeof value !== "boolean") throw new Error("Note boolean required"); return value ? 1n : 0n; }
function optionalFlag(value: boolean | null | undefined): bigint | null { return value == null ? null : flag(value); }
function size(value: bigint): bigint { if (typeof value !== "bigint" || value < 0n || value > 18446744073709551615n) throw new Error("Note blob size exceeds u64"); return value; }
function rowLimit(count: number, options: ArtifactSqliteOptions): void { if (!Number.isSafeInteger(count) || count + 1 > (options.maxRows ?? 1_000_000)) throw new Error("Note SQLite row limit"); }

/** 📤️ Project the canonical Note snapshot through explicit entity and relation rows. */
export async function noteSnapshotToSqliteDatabase(snapshot: NoteSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  rowLimit(snapshot.blocks.length + Object.keys(snapshot.assets ?? {}).length + 1, options);
  const out = await ArtifactSqliteProjection.create(NOTE_SQLITE_SCHEMA, options);
  let count = 0;
  const insert = async (table: Table, cells: readonly Ieee754Cell[], key?: bigint): Promise<bigint> => {
    rowLimit(++count, options);
    const identity = key ?? BigInt(counts.get(table) ?? 1);
    const values = encodeIeee754Cells([identity, ...cells], columns[table] ?? [], options.maxColumns);
    const result = await out.insert(table, values.slice(1), identity);
    counts.set(table, Number(identity) + 1);
    return result;
  };
  const counts = new Map<Table, number>();
  await insert("note_document", [snapshot.schema, snapshot.id, snapshot.title ?? null, optionalFlag(snapshot.gridVisible), snapshot.gridSpacing ?? null, snapshot.gridSubdivisions ?? null, snapshot.gridOpacity ?? null, optionalFlag(snapshot.snapEnabled), snapshot.snapGridSpacing ?? null, snapshot.pencilWidth ?? null, snapshot.eraserRadius ?? null], 1n);
  const assets = new Map<string, bigint>();
  for (const key of Object.keys(snapshot.assets ?? {}).sort()) {
    const asset = snapshot.assets![key]!;
    assets.set(key, await insert("note_asset", [1n, key, asset.mime, asset.data, asset.width ?? null, asset.height ?? null]));
  }
  const link = snapshot.linkedArtifact;
  if (link) {
    const pin = link.pin;
    const bits = pin.kind === "snapshot" ? size(pin.blob.size) : 0n;
    await insert("note_link", [1n, link.target.artifactId, link.target.dialect.artifactKind, link.target.dialect.standard, link.target.dialect.subset, link.role, pin.kind, pin.kind === "checkpoint" ? pin.id : null, pin.kind === "snapshot" ? pin.blob.hash : null, pin.kind === "snapshot" ? bits >> 32n : null, pin.kind === "snapshot" ? bits & 0xffffffffn : null, pin.kind === "snapshot" ? pin.blob.mediaType : null], 1n);
  }
  const stack = snapshot.blocks.map((block, ordinal) => ({ block, ordinal, parent: null as bigint | null })).reverse();
  while (stack.length) {
    const { block, ordinal, parent } = stack.pop()!;
    const key = await insert("note_block", [1n, parent, BigInt(ordinal), block.kind === "stroke" ? "ink" : block.kind, block.id, block.name, block.x, block.y, block.width, block.height, block.rotation ?? binary64(0), flag(block.visible ?? true), flag(block.locked ?? false)]);
    switch (block.kind) {
      case "text": {
        const { handle } = block.content;
        await insert("note_text", [handle.childId, handle.target.artifactId, handle.target.dialect.artifactKind, handle.target.dialect.standard, handle.target.dialect.subset, block.fontSize, block.fontWeight, block.align], key);
        for (const [ordinal, paragraph] of block.content.paragraphs.entries()) {
          const paragraphId = await insert("note_paragraph", [key, BigInt(ordinal)]);
          for (const [ordinal, run] of paragraph.runs.entries()) await insert("note_run", [paragraphId, BigInt(ordinal), run.text, optionalFlag(run.bold), optionalFlag(run.italic), optionalFlag(run.underline), run.link ?? null]);
        }
        break;
      }
      case "image": await insert("note_image", [block.imageKey, assets.get(block.imageKey) ?? null], key); break;
      case "table":
        for (const [ordinal, label] of block.columns.entries()) await insert("note_column", [key, BigInt(ordinal), label]);
        for (const [ordinal, row] of block.rows.entries()) { const rowId = await insert("note_table_row", [key, BigInt(ordinal)]); for (const [ordinal, cell] of row.entries()) await insert("note_cell", [rowId, BigInt(ordinal), cell.content]); }
        break;
      case "math": await insert("note_math", [block.tex, flag(block.displayMode)], key); break;
      case "stroke":
        await insert("note_ink", [block.strokeWidth, ...block.color], key);
        for (const [ordinal, point] of block.points.entries()) await insert("note_point", [key, BigInt(ordinal), point[0], point[1]]);
        break;
      case "group":
        rowLimit(count + stack.length + block.children.length, options);
        for (let ordinal = block.children.length - 1; ordinal >= 0; ordinal--) stack.push({ block: block.children[ordinal]!, ordinal, parent: key });
        break;
      default: throw new Error("Unknown Note block kind");
    }
  }
  return out.finish();
}

class Reader {
  private readonly rows = new Map<Table, Map<bigint, SqliteRow>>();
  private readonly children = new Map<Table, Map<bigint | null, SqliteRow[]>>();
  private readonly used = new Map<Table, Set<bigint>>();
  private completed = 0;
  private total = 0;
  private constructor(private readonly options: ArtifactSqliteOptions) {}

  static async create(database: SqliteDatabase, options: ArtifactSqliteOptions): Promise<Reader> {
    await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, 0);
    const data = await artifactSqliteTables(database, NOTE_SQLITE_SCHEMA, options);
    const reader = new Reader(options);
    for (const [index, table] of tables.entries()) {
      const map = new Map<bigint, SqliteRow>();
      reader.rows.set(table, map);
      reader.used.set(table, new Set());
      for (const row of data[index]!) {
        if (row.rowid <= 0n || integer(row, 0) !== row.rowid || map.has(row.rowid) || row.values.length !== widths[table] + 2 * (columns[table]?.length ?? 0)) throw new Error("Invalid Note row identity or width");
        map.set(row.rowid, row);
        rowLimit(++reader.total, options);
        if (reader.total % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, reader.total);
      }
      const ordinal = table === "note_block" ? 3 : ["note_paragraph", "note_run", "note_column", "note_table_row", "note_cell", "note_point"].includes(table) ? 2 : undefined;
      if (ordinal !== undefined) {
        const groups = new Map<bigint | null, SqliteRow[]>();
        for (const row of map.values()) {
          const parent = table === "note_block" && row.values[2] === null ? null : integer(row, table === "note_block" ? 2 : 1);
          const group = groups.get(parent) ?? [];
          group.push(row);
          groups.set(parent, group);
        }
        for (const group of groups.values()) {
          const ordered = new Array<SqliteRow>(group.length);
          for (const row of group) { const position = integer(row, ordinal); if (position < 0n || position >= BigInt(group.length) || ordered[Number(position)]) throw new Error("Note ordinals must be contiguous and unique"); ordered[Number(position)] = row; }
          for (let i = 0; i < group.length; i++) { group[i] = ordered[i]!; if (i % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, reader.total); }
        }
        reader.children.set(table, groups);
      }
    }
    return reader;
  }

  async take(table: Table, key: bigint): Promise<SqliteRow> {
    const row = this.rows.get(table)!.get(key);
    if (!row || this.used.get(table)!.has(key)) throw new Error("Note row missing, cyclic or referenced twice");
    this.used.get(table)!.add(key);
    if (++this.completed % 256 === 0) await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.completed, this.total);
    return row;
  }
  ordered(table: Table, parent: bigint | null): readonly SqliteRow[] { const groups = this.children.get(table); const rows = groups?.get(parent) ?? []; groups?.delete(parent); return rows; }
  real(table: Table, row: SqliteRow, index: number): Binary64 { return readBinary64(row, index, columns[table] ?? []); }
  optionalReal(table: Table, row: SqliteRow, index: number): Binary64 | null { return ieee754IsNull(row, index, columns[table] ?? []) ? null : this.real(table, row, index); }
  optionalBool(row: SqliteRow, index: number): boolean | null { return row.values[index] === null ? null : boolean(row, index); }
  optionalText(row: SqliteRow, index: number): string | null { return row.values[index] === null ? null : text(row, index); }

  async snapshot(): Promise<NoteSnapshot> {
    const row = await this.take("note_document", 1n);
    const assets: Record<string, NoteImageAsset> = Object.create(null);
    const snapshot: NoteSnapshot = { schema: text(row, 1), id: text(row, 2), title: this.optionalText(row, 3), gridVisible: this.optionalBool(row, 4), gridSpacing: this.optionalReal("note_document", row, 5), gridSubdivisions: this.optionalReal("note_document", row, 6), gridOpacity: this.optionalReal("note_document", row, 7), snapEnabled: this.optionalBool(row, 8), snapGridSpacing: this.optionalReal("note_document", row, 9), pencilWidth: this.optionalReal("note_document", row, 10), eraserRadius: this.optionalReal("note_document", row, 11), blocks: [], assets, linkedArtifact: null };
    for (const key of this.rows.get("note_asset")!.keys()) {
      const asset = await this.take("note_asset", key);
      const name = text(asset, 2);
      if (integer(asset, 1) !== 1n || Object.hasOwn(assets, name)) throw new Error("Note asset identity or document owner differs");
      assets[name] = { mime: text(asset, 3), data: text(asset, 4), width: this.optionalReal("note_asset", asset, 5), height: this.optionalReal("note_asset", asset, 6) };
    }
    if (this.rows.get("note_link")!.has(1n)) {
      const link = await this.take("note_link", 1n);
      if (integer(link, 1) !== 1n) throw new Error("Note link belongs to another document");
      let pin: ArtifactLink["pin"];
      switch (text(link, 7)) {
        case "head": if (link.values.slice(8).some(value => value !== null)) throw new Error("Note head pin has unexpected fields"); pin = { kind: "head" }; break;
        case "checkpoint": if (link.values.slice(9).some(value => value !== null)) throw new Error("Note checkpoint pin has unexpected fields"); pin = { kind: "checkpoint", id: text(link, 8) }; break;
        case "snapshot": {
          if (link.values[8] !== null) throw new Error("Note snapshot pin has checkpoint fields");
          const high = integer(link, 10), low = integer(link, 11);
          if (high < 0n || high > 0xffffffffn || low < 0n || low > 0xffffffffn) throw new Error("Note blob size word exceeds u32");
          pin = { kind: "snapshot", blob: { hash: text(link, 9), size: high << 32n | low, mediaType: text(link, 12) } };
          break;
        }
        default: throw new Error("Unknown Note history pin");
      }
      snapshot.linkedArtifact = { target: { artifactId: text(link, 2), dialect: { artifactKind: text(link, 3), standard: text(link, 4), subset: text(link, 5) } }, role: text(link, 6), pin };
    }
    for (const block of this.ordered("note_block", null)) snapshot.blocks.push(await this.block(block.rowid));
    if (this.completed !== this.total) throw new Error("Note database contains orphaned or mismatched entities");
    await artifactSqliteCheckpoint(this.options, "reconstructSnapshot", this.total, this.total);
    return snapshot;
  }

  async block(key: bigint): Promise<NoteBlockNode> {
    const node = await this.shallow(key);
    const pending = [{ node, children: node.kind === "group" ? this.ordered("note_block", key)[Symbol.iterator]() : [][Symbol.iterator]() }];
    while (pending.length) {
      const frame = pending[pending.length - 1]!;
      const child = frame.children.next();
      if (!child.done) { const node = await this.shallow(child.value.rowid); pending.push({ node, children: node.kind === "group" ? this.ordered("note_block", child.value.rowid)[Symbol.iterator]() : [][Symbol.iterator]() }); }
      else { const completed = pending.pop()!.node; const parent = pending[pending.length - 1]; if (!parent) return completed; if (parent.node.kind !== "group") throw new Error("Note child belongs to a non-group block"); parent.node.children.push(completed); }
    }
    throw new Error("Empty Note block reconstruction");
  }

  async shallow(key: bigint): Promise<NoteBlockNode> {
    const row = await this.take("note_block", key);
    if (integer(row, 1) !== 1n) throw new Error("Note block belongs to another document");
    const frame = { id: text(row, 5), name: text(row, 6), x: this.real("note_block", row, 7), y: this.real("note_block", row, 8), width: this.real("note_block", row, 9), height: this.real("note_block", row, 10), rotation: this.real("note_block", row, 11), visible: boolean(row, 12), locked: boolean(row, 13) };
    switch (text(row, 4)) {
      case "text": {
        const detail = await this.take("note_text", key);
        const paragraphs: NoteTextParagraph[] = [];
        for (const paragraph of this.ordered("note_paragraph", key)) {
          await this.take("note_paragraph", paragraph.rowid);
          const runs: NoteTextRun[] = [];
          for (const run of this.ordered("note_run", paragraph.rowid)) { await this.take("note_run", run.rowid); runs.push({ text: text(run, 3), bold: this.optionalBool(run, 4), italic: this.optionalBool(run, 5), underline: this.optionalBool(run, 6), link: this.optionalText(run, 7) }); }
          paragraphs.push({ runs });
        }
        return { ...frame, kind: "text", fontSize: this.real("note_text", detail, 6), fontWeight: text(detail, 7), align: text(detail, 8), content: { handle: { childId: text(detail, 1), target: { artifactId: text(detail, 2), dialect: { artifactKind: text(detail, 3), standard: text(detail, 4), subset: text(detail, 5) } } }, paragraphs } };
      }
      case "image": {
        const detail = await this.take("note_image", key);
        if (detail.values[2] !== null) { const asset = this.rows.get("note_asset")!.get(integer(detail, 2)); if (!asset || text(asset, 2) !== text(detail, 1)) throw new Error("Note image asset relation differs from its key"); }
        return { ...frame, kind: "image", imageKey: text(detail, 1) };
      }
      case "table": {
        const columns: string[] = [], rows: { content: string }[][] = [];
        for (const column of this.ordered("note_column", key)) { await this.take("note_column", column.rowid); columns.push(text(column, 3)); }
        for (const row of this.ordered("note_table_row", key)) { await this.take("note_table_row", row.rowid); const cells: { content: string }[] = []; for (const cell of this.ordered("note_cell", row.rowid)) { await this.take("note_cell", cell.rowid); cells.push({ content: text(cell, 3) }); } rows.push(cells); }
        return { ...frame, kind: "table", columns, rows };
      }
      case "math": { const detail = await this.take("note_math", key); return { ...frame, kind: "math", tex: text(detail, 1), displayMode: boolean(detail, 2) }; }
      case "ink": {
        const detail = await this.take("note_ink", key);
        const points: [Binary64, Binary64][] = [];
        for (const point of this.ordered("note_point", key)) { await this.take("note_point", point.rowid); points.push([this.real("note_point", point, 3), this.real("note_point", point, 4)]); }
        return { ...frame, kind: "stroke", points, strokeWidth: this.real("note_ink", detail, 1), color: [this.real("note_ink", detail, 2), this.real("note_ink", detail, 3), this.real("note_ink", detail, 4), this.real("note_ink", detail, 5)] };
      }
      case "group": return { ...frame, kind: "group", children: [] };
      default: throw new Error("Unknown Note block kind");
    }
  }
}

/** 📥️ Reconstruct the canonical typed Snapshot without a native wire codec. */
export async function noteSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<NoteSnapshot> { return (await Reader.create(database, options)).snapshot(); }
