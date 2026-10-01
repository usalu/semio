//! 🗒️ Handcrafted note entities, recursive block containment and composed text relationships.
use super::NoteSnapshot;
use crate::{NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextChild, NoteTextParagraph, NoteTextRun};
use semio_framework::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C}};
use std::collections::{BTreeMap, BTreeSet};
#[path="🔢️number/🦀️.rs"]
mod number;
use number::{Row,Projection};

fn text(value: &str) -> V { V::Text(value.into()) }
fn bool_at(row:Row<'_>, index: usize) -> Result<bool, String> { match row.integer(index)? { 0 => Ok(false), 1 => Ok(true), _ => Err("note boolean must be zero or one".into()) } }
fn opt_bool(row:Row<'_>, index: usize) -> Result<Option<bool>, String> { if matches!(row.values.get(index), Some(V::Null)) { Ok(None) } else { bool_at(row, index).map(Some) } }
fn opt_real(row:Row<'_>, index: usize) -> Result<Option<f64>, String> { if row.is_null(index)? { Ok(None) } else { row.real(index).map(Some) } }
fn opt_int(row:Row<'_>, index: usize) -> Result<Option<i64>, String> { if matches!(row.values.get(index), Some(V::Null)) { Ok(None) } else { row.integer(index).map(Some) } }
fn optional_text_cell(value: &Option<String>) -> C<'_> { value.as_deref().map_or(C::Null, C::Text) }
fn bool_cell(value: bool) -> C<'static> { C::Integer(i64::from(value)) }
fn optional_bool_cell(value: Option<bool>) -> C<'static> { value.map_or(C::Null, bool_cell) }
fn optional_real_cell(value: Option<f64>) -> C<'static> { value.map_or(C::Null, C::Real) }

fn common(block: &NoteBlockNode) -> (&str, &str, [f64; 5], bool, bool) {
    match block {
        NoteBlockNode::Text { id, name, x, y, width, height, rotation, visible, locked, .. } |
        NoteBlockNode::Image { id, name, x, y, width, height, rotation, visible, locked, .. } |
        NoteBlockNode::Table { id, name, x, y, width, height, rotation, visible, locked, .. } |
        NoteBlockNode::Math { id, name, x, y, width, height, rotation, visible, locked, .. } |
        NoteBlockNode::Ink { id, name, x, y, width, height, rotation, visible, locked, .. } |
        NoteBlockNode::Group { id, name, x, y, width, height, rotation, visible, locked, .. } => (id, name, [*x, *y, *width, *height, *rotation], *visible, *locked),
    }
}

fn kind(block: &NoteBlockNode) -> &'static str { match block { NoteBlockNode::Text {..} => "text", NoteBlockNode::Image {..} => "image", NoteBlockNode::Table {..} => "table", NoteBlockNode::Math {..} => "math", NoteBlockNode::Ink {..} => "ink", NoteBlockNode::Group {..} => "group" } }

struct ProjectionBudget<'c, 'p> { control: &'c mut Control<'p>, rows: usize, bytes: usize, work: usize }

impl ProjectionBudget<'_, '_> {
    fn text(&mut self, value: &str) -> Result<(), String> {
        self.bytes = self.bytes.checked_add(value.len()).ok_or("note text byte count overflow")?;
        self.control.check_value_bytes(self.bytes)?;
        if value.len() > 65_536 { self.control.checkpoint(Phase::ProjectSnapshot, self.rows, 0)?; }
        self.work += 1;
        if self.work % 256 == 0 { self.control.checkpoint(Phase::ProjectSnapshot, self.rows, 0)?; }
        Ok(())
    }
    fn scalars(&mut self, count: usize) -> Result<(), String> {
        self.bytes = self.bytes.checked_add(count.checked_mul(8).ok_or("note scalar byte count overflow")?).ok_or("note scalar byte count overflow")?;
        self.control.check_value_bytes(self.bytes)
    }
    fn rows(&mut self, count: usize) -> Result<(), String> {
        self.rows = self.rows.checked_add(count).ok_or("note row count overflow")?;
        self.control.check_rows(self.rows.checked_add(1).ok_or("note row count overflow")?)?;
        self.control.checkpoint(Phase::ProjectSnapshot, self.rows, 0)
    }
}

fn preflight(snapshot: &NoteSnapshot, control: &mut Control<'_>) -> Result<usize, String> {
    control.check_rows(snapshot.blocks.len().checked_add(snapshot.assets.len()).and_then(|count| count.checked_add(2)).ok_or("note row count overflow")?)?;
    let mut budget = ProjectionBudget { control, rows: 1, bytes: 0, work: 0 };
    budget.scalars(1 + [snapshot.grid_visible.is_some(), snapshot.grid_spacing.is_some(), snapshot.grid_subdivisions.is_some(), snapshot.grid_opacity.is_some(), snapshot.snap_enabled.is_some(), snapshot.snap_grid_spacing.is_some(), snapshot.pencil_width.is_some(), snapshot.eraser_radius.is_some()].into_iter().filter(|present| *present).count())?;
    budget.text(&snapshot.schema)?; budget.text(&snapshot.id)?; if let Some(title) = &snapshot.title { budget.text(title)?; }
    budget.rows(snapshot.assets.len())?;
    for (key, asset) in &snapshot.assets { budget.scalars(2 + usize::from(asset.width.is_some()) + usize::from(asset.height.is_some()))?; budget.text(key)?; budget.text(&asset.mime)?; budget.text(&asset.data)?; }
    if let Some(link) = &snapshot.linked_artifact {
        budget.rows(1)?; budget.scalars(2)?; budget.text(&link.role)?; budget.text(&link.target.artifact_id)?; budget.text(&link.target.dialect.artifact_kind)?; budget.text(&link.target.dialect.standard)?; budget.text(&link.target.dialect.subset)?;
        match &link.pin { store::LinkPin::Head => budget.text("head")?, store::LinkPin::Checkpoint { id } => { budget.text("checkpoint")?; budget.text(id)?; }, store::LinkPin::Snapshot { blob } => { budget.text("snapshot")?; budget.scalars(2)?; budget.text(&blob.hash)?; budget.text(&blob.media_type)?; } }
    }
    let mut stack = snapshot.blocks.iter().map(|block| (block, 0usize)).collect::<Vec<_>>();
    while let Some((block, depth)) = stack.pop() {
        budget.rows(1)?; budget.scalars(10 + usize::from(depth > 0))?; budget.text(kind(block))?;
        let (id, name, _, _, _) = common(block); budget.text(id)?; budget.text(name)?;
        match block {
            NoteBlockNode::Text { content, font_weight, align, .. } => {
                budget.rows(1usize.checked_add(content.paragraphs.len()).ok_or("note row count overflow")?)?;
                budget.scalars(2 + content.paragraphs.len().checked_mul(3).ok_or("note scalar count overflow")?)?;
                for value in [content.handle.child_id.as_str(), content.handle.target.artifact_id.as_str(), content.handle.target.dialect.artifact_kind.as_str(), content.handle.target.dialect.standard.as_str(), content.handle.target.dialect.subset.as_str(), font_weight, align] { budget.text(value)?; }
                for paragraph in &content.paragraphs { budget.rows(paragraph.runs.len())?; for run in &paragraph.runs { budget.scalars(3 + usize::from(run.bold.is_some()) + usize::from(run.italic.is_some()) + usize::from(run.underline.is_some()))?; budget.text(&run.text)?; if let Some(link) = &run.link { budget.text(link)?; } } }
            }
            NoteBlockNode::Image { image_key, .. } => { budget.rows(1)?; budget.scalars(1 + usize::from(snapshot.assets.contains_key(image_key)))?; budget.text(image_key)?; }
            NoteBlockNode::Table { columns, rows, .. } => { budget.rows(columns.len().checked_add(rows.len()).ok_or("note row count overflow")?)?; budget.scalars(columns.len().checked_add(rows.len()).and_then(|count| count.checked_mul(3)).ok_or("note scalar count overflow")?)?; for column in columns { budget.text(column)?; } for row in rows { budget.rows(row.len())?; for cell in row { budget.scalars(3)?; budget.text(&cell.content)?; } } }
            NoteBlockNode::Math { tex, .. } => { budget.rows(1)?; budget.scalars(2)?; budget.text(tex)?; }
            NoteBlockNode::Ink { points, .. } => { budget.rows(1usize.checked_add(points.len()).ok_or("note row count overflow")?)?; budget.scalars(points.len().checked_mul(5).and_then(|count| count.checked_add(6)).ok_or("note scalar count overflow")?)?; }
            NoteBlockNode::Group { children, .. } => { budget.control.check_rows(budget.rows.checked_add(stack.len()).and_then(|count| count.checked_add(children.len())).and_then(|count| count.checked_add(1)).ok_or("note row count overflow")?)?; stack.extend(children.iter().map(|child| (child, depth + 1))); }
        }
    }
    Ok(budget.rows)
}

impl store::ArtifactSqliteSnapshot for NoteSnapshot {
    fn preflight_sqlite_snapshot_encoding(&self, _: sqlite_snapshot::SnapshotEncoding, control: &mut Control<'_>) -> Result<(), String> {
        control.checkpoint(Phase::EncodeNative, 0, 0)?;
        let database = self.to_sqlite_database(control)?;
        let mut bound = sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(16_384)?;
        for table in &database.tables { for row in &table.rows {
            bound.add(4096)?;
            for value in &row.values { match value {
                V::Text(value) => { bound.repeated(value.len(), 16)?; bound.add(256)?; },
                V::Blob(value) => { bound.repeated(value.len(), 64)?; bound.add(256)?; },
                V::Integer(_) | V::Real(_) | V::Null => bound.add(2048)?,
            } }
        } }
        bound.finish()
    }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&Db,control:&mut Control<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(Phase::ProjectSnapshot,0,0)?;
        if dialect.artifact_kind!="s.note.note"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("snapshot has no owned semantic validator for this exact dialect").into());}
        let row=database.table("note_document")?.single_row()?;if row.rowid!=1||row.text(1)?!=self.schema{return Err(String::from("snapshot document identity differs from semantic projection").into());}
        control.checkpoint(Phase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics:Vec::new()})
    }
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut Control<'_>) -> Result<Db, String> {
        control.checkpoint(Phase::ProjectSnapshot, 0, 0)?;
        preflight(self, control)?;
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert_key("note_document", 1, &[C::Text(&self.schema), C::Text(&self.id), optional_text_cell(&self.title), optional_bool_cell(self.grid_visible), optional_real_cell(self.grid_spacing), optional_real_cell(self.grid_subdivisions), optional_real_cell(self.grid_opacity), optional_bool_cell(self.snap_enabled), optional_real_cell(self.snap_grid_spacing), optional_real_cell(self.pencil_width), optional_real_cell(self.eraser_radius)])?;
        let mut assets = BTreeMap::new();
        for (key, asset) in &self.assets { let id = out.insert("note_asset", &[C::Integer(1), C::Text(key), C::Text(&asset.mime), C::Text(&asset.data), optional_real_cell(asset.width), optional_real_cell(asset.height)])?; assets.insert(key.as_str(), id); }
        if let Some(link) = &self.linked_artifact {
            let (pin, checkpoint, hash, high, low, media) = match &link.pin {
                store::LinkPin::Head => ("head", C::Null, C::Null, C::Null, C::Null, C::Null),
                store::LinkPin::Checkpoint { id } => ("checkpoint", C::Text(id), C::Null, C::Null, C::Null, C::Null),
                store::LinkPin::Snapshot { blob } => ("snapshot", C::Null, C::Text(&blob.hash), C::Integer((blob.size >> 32) as i64), C::Integer((blob.size & 0xffff_ffff) as i64), C::Text(&blob.media_type)),
            };
            out.insert_key("note_link", 1, &[C::Integer(1), C::Text(&link.target.artifact_id), C::Text(&link.target.dialect.artifact_kind), C::Text(&link.target.dialect.standard), C::Text(&link.target.dialect.subset), C::Text(&link.role), C::Text(pin), checkpoint, hash, high, low, media])?;
        }
        let mut stack = self.blocks.iter().enumerate().rev().map(|(ordinal, block)| (block, None, ordinal)).collect::<Vec<_>>();
        while let Some((block, parent, ordinal)) = stack.pop() {
            let (id, name, geometry, visible, locked) = common(block);
            let mut cells = vec![C::Integer(1), parent.map_or(C::Null, C::Integer), C::Integer(ordinal as i64), C::Text(kind(block)), C::Text(id), C::Text(name)];
            cells.extend(geometry.into_iter().map(C::Real)); cells.extend([bool_cell(visible), bool_cell(locked)]);
            let block_id = out.insert("note_block", &cells)?;
            match block {
                NoteBlockNode::Text { content, font_size, font_weight, align, .. } => {
                    out.insert_key("note_text", block_id, &[C::Text(&content.handle.child_id), C::Text(&content.handle.target.artifact_id), C::Text(&content.handle.target.dialect.artifact_kind), C::Text(&content.handle.target.dialect.standard), C::Text(&content.handle.target.dialect.subset), C::Real(*font_size), C::Text(font_weight), C::Text(align)])?;
                    for (ordinal, paragraph) in content.paragraphs.iter().enumerate() { let paragraph_id = out.insert("note_paragraph", &[C::Integer(block_id), C::Integer(ordinal as i64)])?; for (ordinal, run) in paragraph.runs.iter().enumerate() { out.insert("note_run", &[C::Integer(paragraph_id), C::Integer(ordinal as i64), C::Text(&run.text), optional_bool_cell(run.bold), optional_bool_cell(run.italic), optional_bool_cell(run.underline), optional_text_cell(&run.link)])?; } }
                }
                NoteBlockNode::Image { image_key, .. } => { out.insert_key("note_image", block_id, &[C::Text(image_key), assets.get(image_key.as_str()).copied().map_or(C::Null, C::Integer)])?; }
                NoteBlockNode::Table { columns, rows, .. } => {
                    for (ordinal, column) in columns.iter().enumerate() { out.insert("note_column", &[C::Integer(block_id), C::Integer(ordinal as i64), C::Text(column)])?; }
                    for (ordinal, row) in rows.iter().enumerate() { let row_id = out.insert("note_table_row", &[C::Integer(block_id), C::Integer(ordinal as i64)])?; for (ordinal, cell) in row.iter().enumerate() { out.insert("note_cell", &[C::Integer(row_id), C::Integer(ordinal as i64), C::Text(&cell.content)])?; } }
                }
                NoteBlockNode::Math { tex, display_mode, .. } => { out.insert_key("note_math", block_id, &[C::Text(tex), bool_cell(*display_mode)])?; }
                NoteBlockNode::Ink { points, stroke_width, color, .. } => { let mut cells = vec![C::Real(*stroke_width)]; cells.extend(color.iter().copied().map(C::Real)); out.insert_key("note_ink", block_id, &cells)?; for (ordinal, point) in points.iter().enumerate() { out.insert("note_point", &[C::Integer(block_id), C::Integer(ordinal as i64), C::Real(point[0]), C::Real(point[1])])?; } }
                NoteBlockNode::Group { children, .. } => { stack.extend(children.iter().enumerate().rev().map(|(ordinal, child)| (child, Some(block_id), ordinal))); }
            }
        }
        out.finish()
    }

    fn from_sqlite_database(db: &Db, control: &mut Control<'_>) -> Result<Self, String> {
        control.checkpoint(Phase::ReconstructSnapshot, 0, 0)?;
        sqlite_snapshot::validate_sqlite_database_schema(db, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(db, Phase::ReconstructSnapshot)?;
        let mut reader = Reader::new(db, control)?;
        reader.snapshot()
    }
}

struct Reader<'a, 'c, 'p> {
    rows: BTreeMap<(&'static str, i64), Row<'a>>,
    children: BTreeMap<(&'static str, Option<i64>), Vec<Row<'a>>>,
    used: BTreeSet<(&'static str, i64)>,
    control: &'c mut Control<'p>,
    total: usize,
}

const TABLES: [&str; 14] = ["note_document", "note_asset", "note_link", "note_block", "note_text", "note_paragraph", "note_run", "note_image", "note_column", "note_table_row", "note_cell", "note_math", "note_ink", "note_point"];

impl<'a, 'c, 'p> Reader<'a, 'c, 'p> {
    fn new(db: &'a Db, control: &'c mut Control<'p>) -> Result<Self, String> {
        let total = db.tables.iter().try_fold(0usize, |count, table| count.checked_add(table.rows.len()).ok_or("note row count overflow"))?;
        control.check_rows(total.checked_add(1).ok_or("note row count overflow")?)?;
        let mut reader = Self { rows: BTreeMap::new(), children: BTreeMap::new(), used: BTreeSet::new(), control, total };
        let mut bytes = 0usize;
        for name in TABLES {
            let width = match name { "note_document" => 12, "note_asset" => 7, "note_link" => 13, "note_block" => 14, "note_text" => 9, "note_paragraph" => 3, "note_run" => 8, "note_image" => 3, "note_column" => 4, "note_table_row" => 3, "note_cell" => 4, "note_math" => 3, "note_ink" => 6, "note_point" => 5, _ => unreachable!() };
            for row in &db.table(name)?.rows {
                let row=sqlite_snapshot::artifact::FloatRow::new(row,number::columns(name))?;
                if row.values.len() != width { return Err(format!("{name} row has the wrong width")); }
                if row.rowid <= 0 || row.integer(0)? != row.rowid || reader.rows.insert((name, row.rowid), row).is_some() { return Err("duplicate or invalid note row identity".into()); }
                for value in row.values { if let V::Text(value) = value { bytes = bytes.checked_add(value.len()).ok_or("note text byte count overflow")?; reader.control.check_value_bytes(bytes)?; } }
                let parent = match name { "note_block" => Some(opt_int(row, 2)?), "note_paragraph" | "note_run" | "note_column" | "note_table_row" | "note_cell" | "note_point" => Some(Some(row.integer(1)?)), _ => None };
                if let Some(parent) = parent { reader.children.entry((name, parent)).or_default().push(row); }
                if reader.rows.len() % 256 == 0 { reader.control.checkpoint(Phase::ReconstructSnapshot, 0, total)?; }
            }
        }
        for ((table, _), rows) in &mut reader.children {
            let ordinal = if *table == "note_block" { 3 } else { 2 };
            let mut comparisons = 0usize;
            let mut cancelled = None;
            rows.sort_by_key(|row| { comparisons += 1; if comparisons % 1024 == 0 && cancelled.is_none() { cancelled = reader.control.checkpoint(Phase::ReconstructSnapshot, 0, total).err(); } row.integer(ordinal).unwrap_or(i64::MIN) });
            if let Some(error) = cancelled { return Err(error); }
            for (position, row) in rows.iter().enumerate() { if row.integer(ordinal)? != position as i64 { return Err(format!("{table} ordinals must be contiguous and unique")); } }
        }
        Ok(reader)
    }

    fn take(&mut self, table: &'static str, id: i64) -> Result<Row<'a>, String> {
        let row = self.rows.get(&(table, id)).copied().ok_or_else(|| format!("missing {table} row {id}"))?;
        if !self.used.insert((table, id)) { return Err(format!("{table} row {id} was referenced twice or cyclically")); }
        if self.used.len() % 256 == 0 { self.control.checkpoint(Phase::ReconstructSnapshot, self.used.len(), self.total)?; }
        Ok(row)
    }

    fn text(&mut self,row:Row<'_>,column:usize)->Result<String,String>{store::sqlite_snapshot::artifact::Reconstruction::new(self.control)?.text(row.text(column)?)}
    fn optional_text(&mut self,row:Row<'_>,column:usize)->Result<Option<String>,String>{row.optional_text(column)?.map(|value|store::sqlite_snapshot::artifact::Reconstruction::new(self.control)?.text(value)).transpose()}
    fn ordered(&mut self, table: &'static str, parent: Option<i64>) -> Vec<Row<'a>> { self.children.remove(&(table, parent)).unwrap_or_default() }

    fn snapshot(&mut self) -> Result<NoteSnapshot, String> {
        let row = self.take("note_document", 1)?;
        let mut snapshot = NoteSnapshot { schema: self.text(row,1)?, id: self.text(row,2)?, title: self.optional_text(row,3)?, grid_visible: opt_bool(row, 4)?, grid_spacing: opt_real(row, 5)?, grid_subdivisions: opt_real(row, 6)?, grid_opacity: opt_real(row, 7)?, snap_enabled: opt_bool(row, 8)?, snap_grid_spacing: opt_real(row, 9)?, pencil_width: opt_real(row, 10)?, eraser_radius: opt_real(row, 11)?, blocks: Vec::new(), assets: BTreeMap::new(), linked_artifact: None };
        let assets = self.rows.range(("note_asset", i64::MIN)..=("note_asset", i64::MAX)).map(|(_, row)| row.rowid).collect::<Vec<_>>();
        for id in assets { let row = self.take("note_asset", id)?; if row.integer(1)? != 1 { return Err("note asset owner must be its document".into()); } if snapshot.assets.insert(self.text(row,2)?, NoteImageAsset { mime: self.text(row,3)?, data: self.text(row,4)?, width: opt_real(row, 5)?, height: opt_real(row, 6)? }).is_some() { return Err("duplicate note asset key".into()); } }
        if self.rows.contains_key(&("note_link", 1)) {
            let row = self.take("note_link", 1)?;
            if row.integer(1)? != 1 { return Err("note link owner must be its document".into()); }
            let pin = match row.text(7)? {
                "head" if row.values[8..].iter().all(|value| *value == V::Null) => store::LinkPin::Head,
                "checkpoint" if row.values[9..].iter().all(|value| *value == V::Null) => store::LinkPin::Checkpoint { id: self.text(row,8)? },
                "snapshot" if row.values.get(8) == Some(&V::Null) => { let high = u32::try_from(row.integer(10)?).map_err(|_| "note blob size high word exceeds u32")?; let low = u32::try_from(row.integer(11)?).map_err(|_| "note blob size low word exceeds u32")?; store::LinkPin::Snapshot { blob: store::BlobRef { hash: self.text(row,9)?, size: (u64::from(high) << 32) | u64::from(low), media_type: self.text(row,12)? } } },
                _ => return Err("note link pin fields disagree with their kind".into()),
            };
            snapshot.linked_artifact = Some(store::ArtifactLink { target: store::os_io::ArtifactRef { artifact_id: self.text(row,2)?, dialect: store::os_io::ArtifactDialect { artifact_kind: self.text(row,3)?, standard: self.text(row,4)?, subset: self.text(row,5)? } }, role: self.text(row,6)?, pin });
        }
        for row in self.ordered("note_block", None) { snapshot.blocks.push(self.block(row.rowid)?); }
        if self.used.len() != self.total { return Err("note SQLite contains orphaned or mismatched entity rows".into()); }
        self.control.checkpoint(Phase::ReconstructSnapshot, self.total, self.total)?;
        Ok(snapshot)
    }

    fn block(&mut self,key:i64)->Result<NoteBlockNode,String>{
        let node=self.block_shallow(key)?;
        let children=if matches!(node,NoteBlockNode::Group{..}){self.ordered("note_block",Some(key))}else{Vec::new()};
        let mut pending=vec![(node,children.into_iter())];
        loop{
            if let Some(child)=pending.last_mut().and_then(|(_,children)|children.next()){
                let node=self.block_shallow(child.rowid)?;
                let children=if matches!(node,NoteBlockNode::Group{..}){self.ordered("note_block",Some(child.rowid))}else{Vec::new()};
                pending.push((node,children.into_iter()));
            }else{
                let(node,_)=pending.pop().ok_or("note reconstruction stack is empty")?;
                if let Some((NoteBlockNode::Group{children,..},_))=pending.last_mut(){children.push(node);}else if pending.is_empty(){return Ok(node);}else{return Err("note child belongs to a non-group block".into());}
            }
        }
    }

    fn block_shallow(&mut self, key: i64) -> Result<NoteBlockNode, String> {
        let row = self.take("note_block", key)?;
        if row.integer(1)? != 1 { return Err("note block owner must be its document".into()); }
        let id = self.text(row,5)?; let name = self.text(row,6)?;
        let x = row.real(7)?; let y = row.real(8)?; let width = row.real(9)?; let height = row.real(10)?; let rotation = row.real(11)?;
        let visible = bool_at(row, 12)?; let locked = bool_at(row, 13)?;
        Ok(match row.text(4)? {
            "text" => {
                let detail = self.take("note_text", key)?;
                let handle = store::ArtifactChild::new(self.text(detail,1)?, store::os_io::ArtifactRef { artifact_id: self.text(detail,2)?, dialect: store::os_io::ArtifactDialect { artifact_kind: self.text(detail,3)?, standard: self.text(detail,4)?, subset: self.text(detail,5)? } });
                let mut paragraphs = Vec::new();
                for paragraph in self.ordered("note_paragraph", Some(key)) { self.take("note_paragraph", paragraph.rowid)?; let mut runs = Vec::new(); for run in self.ordered("note_run", Some(paragraph.rowid)) { self.take("note_run", run.rowid)?; runs.push(NoteTextRun { text: self.text(run,3)?, bold: opt_bool(run, 4)?, italic: opt_bool(run, 5)?, underline: opt_bool(run, 6)?, link: self.optional_text(run,7)? }); } paragraphs.push(NoteTextParagraph { runs }); }
                NoteBlockNode::Text { id, name, x, y, width, height, rotation, visible, locked, content: NoteTextChild { handle, paragraphs }, font_size: detail.real(6)?, font_weight: self.text(detail,7)?, align: self.text(detail,8)? }
            }
            "image" => {
                let detail = self.take("note_image", key)?;
                if let Some(asset) = opt_int(detail, 2)? { let asset = self.rows.get(&("note_asset", asset)).ok_or("note image references a missing asset")?; if asset.text(2)? != detail.text(1)? { return Err("note image key and asset relation disagree".into()); } }
                NoteBlockNode::Image { id, name, x, y, width, height, rotation, visible, locked, image_key: self.text(detail,1)? }
            }
            "table" => {
                let mut columns = Vec::new(); for column in self.ordered("note_column", Some(key)) { self.take("note_column", column.rowid)?; columns.push(self.text(column,3)?); }
                let mut rows = Vec::new(); for row in self.ordered("note_table_row", Some(key)) { self.take("note_table_row", row.rowid)?; let mut cells = Vec::new(); for cell in self.ordered("note_cell", Some(row.rowid)) { self.take("note_cell", cell.rowid)?; cells.push(NoteTableCell { content: self.text(cell,3)? }); } rows.push(cells); }
                NoteBlockNode::Table { id, name, x, y, width, height, rotation, visible, locked, columns, rows }
            }
            "math" => { let detail = self.take("note_math", key)?; NoteBlockNode::Math { id, name, x, y, width, height, rotation, visible, locked, tex: self.text(detail,1)?, display_mode: bool_at(detail, 2)? } }
            "ink" => { let detail = self.take("note_ink", key)?; let mut points = Vec::new(); for point in self.ordered("note_point", Some(key)) { self.take("note_point", point.rowid)?; points.push([point.real(3)?, point.real(4)?]); } NoteBlockNode::Ink { id, name, x, y, width, height, rotation, visible, locked, points, stroke_width: detail.real(1)?, color: [detail.real(2)?, detail.real(3)?, detail.real(4)?, detail.real(5)?] } }
            "group" => NoteBlockNode::Group { id, name, x, y, width, height, rotation, visible, locked, children:Vec::new() },
            _ => return Err("unknown note block kind".into()),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sqlite_snapshot_note_owned_guard_checks_exact_identity_and_projection(){
        let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🎯️dialect.json")).unwrap();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let dialect=|value:&serde_json::Value|semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};let valid=dialect(&plan["valid"]);assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|true,limits)).unwrap().diagnostics.is_empty());for invalid in plan["invalid"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut Control::new(&mut |_|true,limits)).is_err());}let mut mismatched=database.clone();mismatched.table_mut("note_document").unwrap().rows[0].values[1]=V::Text("other.schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&mismatched,&mut Control::new(&mut |_|true,limits)).is_err());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|false,limits)).is_err());
    }
    use super::*;
    use store::{ArtifactSqliteSnapshot, FromValue};

    fn fixture() -> NoteSnapshot { NoteSnapshot::from_value(serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()).unwrap() }

    #[test]
    fn sqlite_snapshot_note_preserves_ieee_settings_geometry_and_presence(){
        use std::{io::Write,process::{Command,Stdio}};let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢️ieee.json")).unwrap();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for case in cases["binary64"].as_array().unwrap(){
            let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut snapshot=fixture();snapshot.grid_spacing=Some(value);snapshot.grid_subdivisions=None;snapshot.grid_opacity=Some(value);snapshot.pencil_width=Some(value);snapshot.assets.get_mut("image-1").unwrap().width=Some(value);let NoteBlockNode::Group{x,rotation,..}=&mut snapshot.blocks[0]else{panic!("group");};*x=value;*rotation=value;
            let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();let restored=NoteSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();assert_eq!(restored.grid_spacing.unwrap().to_bits(),bits);assert!(restored.grid_subdivisions.is_none());assert_eq!(restored.grid_opacity.unwrap().to_bits(),bits);assert_eq!(restored.pencil_width.unwrap().to_bits(),bits);assert_eq!(restored.assets["image-1"].width.unwrap().to_bits(),bits);assert!(restored.assets["image-1"].height.is_none());let NoteBlockNode::Group{x,rotation,..}=&restored.blocks[0]else{panic!("group");};assert_eq!(x.to_bits(),bits);assert_eq!(rotation.to_bits(),bits);
            let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT CAST(grid_spacing_bits AS TEXT) AS bits,grid_spacing_class AS class,grid_spacing IS NULL AS nullQuery FROM note_document').get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(actual["class"],case["class"]);assert_eq!(actual["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));
        }
    }

    #[test]
    fn sqlite_snapshot_note_iterative_groups_exceed512_and_sqlite_can_walk_them(){
        use std::{io::Write,process::{Command,Stdio}};let plan:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌲️deep.json")).unwrap();let depth=plan["depth"].as_u64().unwrap() as usize;let mut node=NoteBlockNode::Math{id:plan["leafId"].as_str().unwrap().into(),name:"deep leaf".into(),x:0.0,y:0.0,width:10.0,height:10.0,rotation:0.0,visible:true,locked:false,tex:plan["leafText"].as_str().unwrap().into(),display_mode:true};
        for level in 0..depth{node=NoteBlockNode::Group{id:format!("group-{level}"),name:"nested group".into(),x:0.0,y:0.0,width:10.0,height:10.0,rotation:0.0,visible:true,locked:false,children:vec![node]};}
        let mut snapshot=fixture();snapshot.blocks=vec![node];let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();assert_eq!(db.table("note_block").unwrap().rows.len(),plan["expectedBlocks"].as_u64().unwrap() as usize);let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const row=db.query('WITH RECURSIVE groups(id,depth) AS (SELECT id,0 FROM note_block WHERE parent_id IS NULL UNION ALL SELECT b.id,g.depth+1 FROM note_block b JOIN groups g ON b.parent_id=g.id) SELECT COUNT(*) AS count,MAX(depth) AS depth FROM groups').get();await Bun.write(Bun.stdout,JSON.stringify(row));db.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["count"],plan["expectedBlocks"]);assert_eq!(actual["depth"],plan["depth"]);
        let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let restored=NoteSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();let mut node=&restored.blocks[0];for _ in 0..depth{let NoteBlockNode::Group{children,..}=node else{panic!("group expected");};assert_eq!(children.len(),1);node=&children[0];}let NoteBlockNode::Math{id,tex,..}=node else{panic!("leaf expected");};assert_eq!(id,plan["leafId"].as_str().unwrap());assert_eq!(tex,plan["leafText"].as_str().unwrap());
    }

    #[test]
    fn sqlite_snapshot_note_preserves_every_entity_and_relationship() {
        let snapshot = fixture();
        let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
        let mut progress = |_| true;
        let mut control = Control::new(&mut progress, limits);
        let db = snapshot.to_sqlite_database(&mut control).unwrap();
        assert_eq!(db.table("note_block").unwrap().rows.len(), 8);
        assert_eq!(db.table("note_paragraph").unwrap().rows.len(), 2);
        assert_eq!(db.table("note_table_row").unwrap().rows.len(), 3);
        assert_eq!(db.table("note_cell").unwrap().rows.len(), 3);
        assert_eq!(db.table("note_point").unwrap().rows.len(), 2);
        assert_eq!(NoteSnapshot::from_sqlite_database(&db, &mut control).unwrap(), snapshot);
        let file = sqlite_snapshot::export_sqlite_database(&db, limits, &mut |_| true).unwrap();
        let loaded = sqlite_snapshot::import_sqlite_database(&file, limits, &mut |_| true).unwrap();
        assert_eq!(NoteSnapshot::from_sqlite_database(&loaded, &mut control).unwrap(), snapshot);
        let mut changed = db.clone();
        changed.table_mut("note_run").unwrap().rows[0].values[3] = text("Edited through SQL");
        let changed = NoteSnapshot::from_sqlite_database(&changed, &mut control).unwrap();
        let NoteBlockNode::Group { children, .. } = &changed.blocks[0] else { panic!("group expected") };
        let NoteBlockNode::Text { content, .. } = &children[0] else { panic!("text expected") };
        assert_eq!(content.paragraphs[1].runs[0].text, "Edited through SQL");
        for pin in [store::LinkPin::Head, store::LinkPin::Checkpoint { id: "checkpoint-1".into() }, store::LinkPin::Snapshot { blob: store::BlobRef { hash: "max-size".into(), size: u64::MAX, media_type: "application/octet-stream".into() } }] {
            let mut variant = snapshot.clone(); variant.linked_artifact.as_mut().unwrap().pin = pin;
            let db = variant.to_sqlite_database(&mut control).unwrap();
            assert_eq!(NoteSnapshot::from_sqlite_database(&db, &mut control).unwrap(), variant);
        }
    }

    #[test]
    fn sqlite_snapshot_note_rejects_invalid_contracts_and_honors_control() {
        let snapshot = fixture(); let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_| false, limits)).is_err());
        let mut too_small = limits; too_small.max_rows = 2;
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_| true, too_small)).is_err());
        too_small = limits; too_small.max_value_bytes = 1;
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_| true, too_small)).is_err());
        let db = snapshot.to_sqlite_database(&mut Control::new(&mut |_| true, limits)).unwrap();
        assert!(NoteSnapshot::from_sqlite_database(&db, &mut Control::new(&mut |_| false, limits)).is_err());
        let mut invalid = db.clone(); invalid.table_mut("note_block").unwrap().rows[0].values[2] = V::Integer(999);
        assert!(NoteSnapshot::from_sqlite_database(&invalid, &mut Control::new(&mut |_| true, limits)).is_err());
        let mut invalid = db.clone(); invalid.table_mut("note_run").unwrap().rows[0].values[2] = V::Integer(8);
        assert!(NoteSnapshot::from_sqlite_database(&invalid, &mut Control::new(&mut |_| true, limits)).is_err());
        let mut invalid = db.clone(); invalid.table_mut("note_block").unwrap().sql = invalid.table("note_block").unwrap().sql.replace("name TEXT NOT NULL", "name TEXT \"NOT NULL\"");
        assert!(NoteSnapshot::from_sqlite_database(&invalid, &mut Control::new(&mut |_| true, limits)).is_err());
        let mut large = snapshot; let NoteBlockNode::Group { children, .. } = &mut large.blocks[0] else { panic!("group expected") }; let NoteBlockNode::Text { content, .. } = &mut children[0] else { panic!("text expected") };
        content.paragraphs[1].runs = vec![content.paragraphs[1].runs[0].clone(); 1000];
        let mut calls = 0;
        assert!(large.to_sqlite_database(&mut Control::new(&mut |_| { calls += 1; calls < 8 }, limits)).is_err());
    }
}
