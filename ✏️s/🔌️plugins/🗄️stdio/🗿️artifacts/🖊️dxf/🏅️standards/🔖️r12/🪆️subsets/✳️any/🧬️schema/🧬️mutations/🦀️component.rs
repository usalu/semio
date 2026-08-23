//! 🧬️ DxfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `DxfDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, name/index-aware, reading the pre-state it needs from `base`.
//! `SetLayer`/`SetStyle`/`SetLinetype`/`SetEntity`/`SetBlock` each set a WHOLE item (not a
//! single sub-field) — their `diff()` still constructs a sparse per-field patch by comparing
//! against `base`'s current value, never a full-item replace.
//!
//! 🧪️ F6: `OpText`/`OpBinary` for `DxfMutation` are **hand-rolled** — `#[derive(dsl::DslOps)]`
//! confirmed rejected by a real `cargo check` (independent of `DxfDiff`'s own rejection above):
//! `SetSnapshot{snapshot:DxfSnapshot}` recursively contains `DxfEntity` (no `DslField`), and
//! `InsertEntity`/`SetEntity`'s `entity: DxfEntity` / `InsertBlock`/`SetBlock`'s `block: DxfBlock`
//! (which itself contains `Vec<DxfEntity>`) carry the same data-carrying-enum payload DIRECTLY as
//! a variant field — `error[E0277]: the trait bound 'DxfEntity: DslField' is not satisfied` at
//! `InsertEntity{entity:DxfEntity}`/`SetEntity{entity:DxfEntity}` (recon report §3a, mutation-side
//! twin of the diff-side blocker: the derive requires `DslField` on every reachable type whether
//! it arrives via a Diff struct field or a Mutation variant field). Grammar: `keyword arg=value
//! ...` (space-separated, same shape the derive's own handcrafted-wrapper convention uses),
//! reusing `🔺️diff`'s `pub(crate)` grammar primitives rather than duplicating them a second time.

use crate::artifacts::dxf::schema::diff::{
    block_diff_between,
    dec_block,
    // 🧪️ P2-FG1: real recursive binary twins backing the upgraded `OpBinary` impl below (see
    // `🔺️diff/🦀️component.rs`'s `#region 🔖️ItemBinaryCodecs`/`#region 🔖️BinaryPrimitives`).
    dec_block_bin,
    dec_dxf_entity,
    dec_dxf_entity_bin,
    dec_dxf_snapshot,
    dec_dxf_snapshot_bin,
    dec_header_var,
    dec_header_var_bin,
    dec_layer,
    dec_layer_bin,
    dec_linetype,
    dec_linetype_bin,
    dec_str,
    dec_style,
    dec_style_bin,
    diff_insert_block,
    diff_insert_entity,
    diff_insert_layer,
    diff_insert_linetype,
    diff_insert_style,
    diff_remove_block,
    diff_remove_entity,
    diff_remove_header_var,
    diff_remove_layer,
    diff_remove_linetype,
    diff_remove_style,
    diff_set_block,
    diff_set_entity,
    diff_set_header_var,
    diff_set_layer,
    diff_set_linetype,
    diff_set_snapshot,
    diff_set_style,
    enc_block,
    enc_block_bin,
    enc_dxf_entity,
    enc_dxf_entity_bin,
    enc_dxf_snapshot,
    enc_dxf_snapshot_bin,
    enc_header_var,
    enc_header_var_bin,
    enc_layer,
    enc_layer_bin,
    enc_linetype,
    enc_linetype_bin,
    enc_str,
    enc_style,
    enc_style_bin,
    entity_diff_between_pub,
    layer_diff_between,
    linetype_diff_between,
    read_str_lp,
    style_diff_between,
    write_str_lp,
    DxfDiff,
};
use crate::artifacts::dxf::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle};
use crate::artifacts::dxf::DxfSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.dxf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DxfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DxfSnapshot,
    },

    /// 🏷️ Creates or replaces a `$VAR` header entry.
    SetHeaderVar {
        name: String,
        header_var: DxfHeaderVar,
    },
    /// ➖️ Removes a `$VAR` header entry.
    RemoveHeaderVar {
        name: String,
    },

    /// ➕️ Inserts a whole `LAYER` table entry at `index`.
    InsertLayer {
        index: usize,
        layer: DxfLayer,
    },
    /// ➖️ Removes a `LAYER` table entry by name.
    RemoveLayer {
        name: String,
    },
    /// ✏️ Replaces the WHOLE `LAYER` entry named `name` (diff is still a sparse per-field patch).
    SetLayer {
        name: String,
        layer: DxfLayer,
    },

    /// ➕️ Inserts a whole `STYLE` table entry at `index`.
    InsertStyle {
        index: usize,
        style: DxfStyle,
    },
    /// ➖️ Removes a `STYLE` table entry by name.
    RemoveStyle {
        name: String,
    },
    /// ✏️ Replaces the WHOLE `STYLE` entry named `name`.
    SetStyle {
        name: String,
        style: DxfStyle,
    },

    /// ➕️ Inserts a whole `LTYPE` table entry at `index`.
    InsertLinetype {
        index: usize,
        linetype: DxfLinetype,
    },
    /// ➖️ Removes an `LTYPE` table entry by name.
    RemoveLinetype {
        name: String,
    },
    /// ✏️ Replaces the WHOLE `LTYPE` entry named `name`.
    SetLinetype {
        name: String,
        linetype: DxfLinetype,
    },

    /// ➕️ Inserts a whole entity at `index` in the top-level `ENTITIES` list.
    InsertEntity {
        index: usize,
        entity: DxfEntity,
    },
    /// ➖️ Removes the entity at `index`.
    RemoveEntity {
        index: usize,
    },
    /// ✏️ Replaces the WHOLE entity at `index` (diff is `Replace` on kind change, else a
    /// sparse kind-specific patch).
    SetEntity {
        index: usize,
        entity: DxfEntity,
    },

    /// ➕️ Inserts a whole `BLOCK` at `index`.
    InsertBlock {
        index: usize,
        block: DxfBlock,
    },
    /// ➖️ Removes the `BLOCK` at `index`.
    RemoveBlock {
        index: usize,
    },
    /// ✏️ Replaces the WHOLE `BLOCK` at `index`.
    SetBlock {
        index: usize,
        block: DxfBlock,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DxfMutation` variant, declaration order — the single source of
/// truth `../../🧪️oracle/🔣️component.json`'s `mutationCatalogs[].kinds` and every test-case adapter
/// duplicate against (per ticket 26/08/23/END-TO-END-TESTING-REFACTOR wave 7's registration rule:
/// the framework never parses Rust, so this constant plus `kinds_const_matches_enum_variants` below
/// is what keeps the manifest honest).
pub const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "set-header-var",
    "remove-header-var",
    "insert-layer",
    "remove-layer",
    "set-layer",
    "insert-style",
    "remove-style",
    "set-style",
    "insert-linetype",
    "remove-linetype",
    "set-linetype",
    "insert-entity",
    "remove-entity",
    "set-entity",
    "insert-block",
    "remove-block",
    "set-block",
];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_dxf_mutation(snapshot: &mut DxfSnapshot, mutation: &DxfMutation) -> protocol::MutationOutcome<DxfDiff> {
    let outcome = <DxfMutation as Mutation<DxfSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DxfSnapshot> for DxfMutation {
    type Diff = DxfDiff;

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            DxfMutation::NoMutation => DxfDiff::default(),
            DxfMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            DxfMutation::SetHeaderVar { name, header_var } => {
                let existed = base.header_vars.iter().any(|v| &v.name == name);
                diff_set_header_var(base.header_vars.len(), name, header_var.clone(), existed)
            }
            DxfMutation::RemoveHeaderVar { name } => diff_remove_header_var(name),

            DxfMutation::InsertLayer { index, layer } => diff_insert_layer(*index, layer.clone()),
            DxfMutation::RemoveLayer { name } => diff_remove_layer(name),
            DxfMutation::SetLayer { name, layer } => {
                let old = base.tables.layers.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
                diff_set_layer(name, layer_diff_between(&old, layer))
            }

            DxfMutation::InsertStyle { index, style } => diff_insert_style(*index, style.clone()),
            DxfMutation::RemoveStyle { name } => diff_remove_style(name),
            DxfMutation::SetStyle { name, style } => {
                let old = base.tables.styles.iter().find(|s| &s.name == name).cloned().unwrap_or_default();
                diff_set_style(name, style_diff_between(&old, style))
            }

            DxfMutation::InsertLinetype { index, linetype } => diff_insert_linetype(*index, linetype.clone()),
            DxfMutation::RemoveLinetype { name } => diff_remove_linetype(name),
            DxfMutation::SetLinetype { name, linetype } => {
                let old = base.tables.linetypes.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
                diff_set_linetype(name, linetype_diff_between(&old, linetype))
            }

            DxfMutation::InsertEntity { index, entity } => diff_insert_entity(*index, entity.clone()),
            DxfMutation::RemoveEntity { index } => diff_remove_entity(*index),
            DxfMutation::SetEntity { index, entity } => match base.entities.get(*index) {
                Some(old) => diff_set_entity(*index, entity_diff_between_pub(old, entity)),
                None => diff_insert_entity(*index, entity.clone()),
            },

            DxfMutation::InsertBlock { index, block } => diff_insert_block(*index, block.clone()),
            DxfMutation::RemoveBlock { index } => diff_remove_block(*index),
            DxfMutation::SetBlock { index, block } => match base.blocks.get(*index) {
                Some(old) => diff_set_block(*index, block_diff_between(old, block)),
                None => diff_insert_block(*index, block.clone()),
            },
        })
    }

    fn inverse(&self, base: &DxfSnapshot) -> Vec<Self> {
        match self {
            DxfMutation::NoMutation => vec![DxfMutation::NoMutation],
            DxfMutation::SetSnapshot { .. } => vec![DxfMutation::SetSnapshot { snapshot: base.clone() }],

            DxfMutation::SetHeaderVar { name, .. } => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => vec![DxfMutation::SetHeaderVar { name: name.clone(), header_var: v.clone() }],
                None => vec![DxfMutation::RemoveHeaderVar { name: name.clone() }],
            },
            DxfMutation::RemoveHeaderVar { name } => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => vec![DxfMutation::SetHeaderVar { name: name.clone(), header_var: v.clone() }],
                None => vec![DxfMutation::NoMutation],
            },

            // 🧭️ Reads the name off the mutation's OWN payload, not `base` at `index` — `base`
            // is pre-insertion state, so `base.tables.layers[index]` (if any) is whatever WAS
            // there before, never the layer this mutation is about to insert.
            DxfMutation::InsertLayer { layer, .. } => vec![DxfMutation::RemoveLayer { name: layer.name.clone() }],
            DxfMutation::RemoveLayer { name } => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::InsertLayer { index: base.tables.layers.iter().position(|x| &x.name == name).unwrap_or(base.tables.layers.len()), layer: l.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
            DxfMutation::SetLayer { name, .. } => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::SetLayer { name: name.clone(), layer: l.clone() }],
                None => vec![DxfMutation::RemoveLayer { name: name.clone() }],
            },

            DxfMutation::InsertStyle { style, .. } => vec![DxfMutation::RemoveStyle { name: style.name.clone() }],
            DxfMutation::RemoveStyle { name } => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => vec![DxfMutation::InsertStyle { index: base.tables.styles.iter().position(|x| &x.name == name).unwrap_or(base.tables.styles.len()), style: s.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
            DxfMutation::SetStyle { name, .. } => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => vec![DxfMutation::SetStyle { name: name.clone(), style: s.clone() }],
                None => vec![DxfMutation::RemoveStyle { name: name.clone() }],
            },

            DxfMutation::InsertLinetype { linetype, .. } => vec![DxfMutation::RemoveLinetype { name: linetype.name.clone() }],
            DxfMutation::RemoveLinetype { name } => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::InsertLinetype { index: base.tables.linetypes.iter().position(|x| &x.name == name).unwrap_or(base.tables.linetypes.len()), linetype: l.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
            DxfMutation::SetLinetype { name, .. } => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::SetLinetype { name: name.clone(), linetype: l.clone() }],
                None => vec![DxfMutation::RemoveLinetype { name: name.clone() }],
            },

            DxfMutation::InsertEntity { index, .. } => vec![DxfMutation::RemoveEntity { index: *index }],
            DxfMutation::RemoveEntity { index } => match base.entities.get(*index) {
                Some(e) => vec![DxfMutation::InsertEntity { index: *index, entity: e.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
            DxfMutation::SetEntity { index, .. } => match base.entities.get(*index) {
                Some(e) => vec![DxfMutation::SetEntity { index: *index, entity: e.clone() }],
                None => vec![DxfMutation::NoMutation],
            },

            DxfMutation::InsertBlock { index, .. } => vec![DxfMutation::RemoveBlock { index: *index }],
            DxfMutation::RemoveBlock { index } => match base.blocks.get(*index) {
                Some(b) => vec![DxfMutation::InsertBlock { index: *index, block: b.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
            DxfMutation::SetBlock { index, .. } => match base.blocks.get(*index) {
                Some(b) => vec![DxfMutation::SetBlock { index: *index, block: b.clone() }],
                None => vec![DxfMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `print_op`/`parse_op` — one match arm per variant (no `DslVariants` scaffolding
/// available since nothing here derives it, see module doc). Every arg value is either hex
/// (strings), decimal (indices), or a `🔺️diff` positional/tagged payload — never a literal
/// space or `=`, so top-level tokenizing is a trivial `line.split(' ')` / `tok.split_once('=')`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_dxf_mutation(m: &DxfMutation) -> String {
    match m {
        DxfMutation::NoMutation => "no-mutation".to_string(),
        DxfMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_dxf_snapshot(snapshot)),

        DxfMutation::SetHeaderVar { name, header_var } => format!("set-header-var name={} header-var={}", enc_str(name), enc_header_var(header_var)),
        DxfMutation::RemoveHeaderVar { name } => format!("remove-header-var name={}", enc_str(name)),

        DxfMutation::InsertLayer { index, layer } => format!("insert-layer index={index} layer={}", enc_layer(layer)),
        DxfMutation::RemoveLayer { name } => format!("remove-layer name={}", enc_str(name)),
        DxfMutation::SetLayer { name, layer } => format!("set-layer name={} layer={}", enc_str(name), enc_layer(layer)),

        DxfMutation::InsertStyle { index, style } => format!("insert-style index={index} style={}", enc_style(style)),
        DxfMutation::RemoveStyle { name } => format!("remove-style name={}", enc_str(name)),
        DxfMutation::SetStyle { name, style } => format!("set-style name={} style={}", enc_str(name), enc_style(style)),

        DxfMutation::InsertLinetype { index, linetype } => format!("insert-linetype index={index} linetype={}", enc_linetype(linetype)),
        DxfMutation::RemoveLinetype { name } => format!("remove-linetype name={}", enc_str(name)),
        DxfMutation::SetLinetype { name, linetype } => format!("set-linetype name={} linetype={}", enc_str(name), enc_linetype(linetype)),

        DxfMutation::InsertEntity { index, entity } => format!("insert-entity index={index} entity={}", enc_dxf_entity(entity)),
        DxfMutation::RemoveEntity { index } => format!("remove-entity index={index}"),
        DxfMutation::SetEntity { index, entity } => format!("set-entity index={index} entity={}", enc_dxf_entity(entity)),

        DxfMutation::InsertBlock { index, block } => format!("insert-block index={index} block={}", enc_block(block)),
        DxfMutation::RemoveBlock { index } => format!("remove-block index={index}"),
        DxfMutation::SetBlock { index, block } => format!("set-block index={index} block={}", enc_block(block)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_dxf_mutation(line: &str) -> Result<DxfMutation, String> {
    if line == "no-mutation" {
        return Ok(DxfMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("dxf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("dxf mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(DxfMutation::SetSnapshot { snapshot: dec_dxf_snapshot(arg("snapshot")?)? }),

        "set-header-var" => Ok(DxfMutation::SetHeaderVar { name: dec_str(arg("name")?)?, header_var: dec_header_var(arg("header-var")?)? }),
        "remove-header-var" => Ok(DxfMutation::RemoveHeaderVar { name: dec_str(arg("name")?)? }),

        "insert-layer" => Ok(DxfMutation::InsertLayer { index: usize_arg("index")?, layer: dec_layer(arg("layer")?)? }),
        "remove-layer" => Ok(DxfMutation::RemoveLayer { name: dec_str(arg("name")?)? }),
        "set-layer" => Ok(DxfMutation::SetLayer { name: dec_str(arg("name")?)?, layer: dec_layer(arg("layer")?)? }),

        "insert-style" => Ok(DxfMutation::InsertStyle { index: usize_arg("index")?, style: dec_style(arg("style")?)? }),
        "remove-style" => Ok(DxfMutation::RemoveStyle { name: dec_str(arg("name")?)? }),
        "set-style" => Ok(DxfMutation::SetStyle { name: dec_str(arg("name")?)?, style: dec_style(arg("style")?)? }),

        "insert-linetype" => Ok(DxfMutation::InsertLinetype { index: usize_arg("index")?, linetype: dec_linetype(arg("linetype")?)? }),
        "remove-linetype" => Ok(DxfMutation::RemoveLinetype { name: dec_str(arg("name")?)? }),
        "set-linetype" => Ok(DxfMutation::SetLinetype { name: dec_str(arg("name")?)?, linetype: dec_linetype(arg("linetype")?)? }),

        "insert-entity" => Ok(DxfMutation::InsertEntity { index: usize_arg("index")?, entity: dec_dxf_entity(arg("entity")?)? }),
        "remove-entity" => Ok(DxfMutation::RemoveEntity { index: usize_arg("index")? }),
        "set-entity" => Ok(DxfMutation::SetEntity { index: usize_arg("index")?, entity: dec_dxf_entity(arg("entity")?)? }),

        "insert-block" => Ok(DxfMutation::InsertBlock { index: usize_arg("index")?, block: dec_block(arg("block")?)? }),
        "remove-block" => Ok(DxfMutation::RemoveBlock { index: usize_arg("index")? }),
        "set-block" => Ok(DxfMutation::SetBlock { index: usize_arg("index")?, block: dec_block(arg("block")?)? }),

        other => Err(format!("dxf mutation: unknown keyword {other:?}")),
    }
}

impl OpText for DxfMutation {
    fn print_op(&self) -> String {
        print_dxf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_dxf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `DxfMutation` variant ordinal, in the SAME 0-18 order `parse_dxf_mutation`'s own keyword match
/// uses. Every variant payload reuses `🔺️diff/🦀️component.rs`'s real recursive binary item
/// codecs (`enc_dxf_snapshot_bin`/`enc_dxf_entity_bin`/`enc_block_bin`/…) — genuinely structured,
/// varint/length-prefixed binary, never text-as-bytes.
impl OpBinary for DxfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            DxfMutation::NoMutation => 0,
            DxfMutation::SetSnapshot { .. } => 1,
            DxfMutation::SetHeaderVar { .. } => 2,
            DxfMutation::RemoveHeaderVar { .. } => 3,
            DxfMutation::InsertLayer { .. } => 4,
            DxfMutation::RemoveLayer { .. } => 5,
            DxfMutation::SetLayer { .. } => 6,
            DxfMutation::InsertStyle { .. } => 7,
            DxfMutation::RemoveStyle { .. } => 8,
            DxfMutation::SetStyle { .. } => 9,
            DxfMutation::InsertLinetype { .. } => 10,
            DxfMutation::RemoveLinetype { .. } => 11,
            DxfMutation::SetLinetype { .. } => 12,
            DxfMutation::InsertEntity { .. } => 13,
            DxfMutation::RemoveEntity { .. } => 14,
            DxfMutation::SetEntity { .. } => 15,
            DxfMutation::InsertBlock { .. } => 16,
            DxfMutation::RemoveBlock { .. } => 17,
            DxfMutation::SetBlock { .. } => 18,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            DxfMutation::NoMutation => {}
            DxfMutation::SetSnapshot { snapshot } => enc_dxf_snapshot_bin(snapshot, &mut out),
            DxfMutation::SetHeaderVar { name, header_var } => {
                write_str_lp(&mut out, name);
                enc_header_var_bin(header_var, &mut out);
            }
            DxfMutation::RemoveHeaderVar { name } => write_str_lp(&mut out, name),
            DxfMutation::InsertLayer { index, layer } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_layer_bin(layer, &mut out);
            }
            DxfMutation::RemoveLayer { name } => write_str_lp(&mut out, name),
            DxfMutation::SetLayer { name, layer } => {
                write_str_lp(&mut out, name);
                enc_layer_bin(layer, &mut out);
            }
            DxfMutation::InsertStyle { index, style } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_style_bin(style, &mut out);
            }
            DxfMutation::RemoveStyle { name } => write_str_lp(&mut out, name),
            DxfMutation::SetStyle { name, style } => {
                write_str_lp(&mut out, name);
                enc_style_bin(style, &mut out);
            }
            DxfMutation::InsertLinetype { index, linetype } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_linetype_bin(linetype, &mut out);
            }
            DxfMutation::RemoveLinetype { name } => write_str_lp(&mut out, name),
            DxfMutation::SetLinetype { name, linetype } => {
                write_str_lp(&mut out, name);
                enc_linetype_bin(linetype, &mut out);
            }
            DxfMutation::InsertEntity { index, entity } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_dxf_entity_bin(entity, &mut out);
            }
            DxfMutation::RemoveEntity { index } => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            DxfMutation::SetEntity { index, entity } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_dxf_entity_bin(entity, &mut out);
            }
            DxfMutation::InsertBlock { index, block } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            DxfMutation::RemoveBlock { index } => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            DxfMutation::SetBlock { index, block } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(DxfMutation::NoMutation),
            1 => Ok(DxfMutation::SetSnapshot { snapshot: dec_dxf_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))? }),
            2 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let header_var = dec_header_var_bin(&mut reader).map_err(|e| malformed("op header_var", reader.position(), e))?;
                Ok(DxfMutation::SetHeaderVar { name, header_var })
            }
            3 => Ok(DxfMutation::RemoveHeaderVar { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? }),
            4 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let layer = dec_layer_bin(&mut reader).map_err(|e| malformed("op layer", reader.position(), e))?;
                Ok(DxfMutation::InsertLayer { index, layer })
            }
            5 => Ok(DxfMutation::RemoveLayer { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? }),
            6 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let layer = dec_layer_bin(&mut reader).map_err(|e| malformed("op layer", reader.position(), e))?;
                Ok(DxfMutation::SetLayer { name, layer })
            }
            7 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DxfMutation::InsertStyle { index, style })
            }
            8 => Ok(DxfMutation::RemoveStyle { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? }),
            9 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DxfMutation::SetStyle { name, style })
            }
            10 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let linetype = dec_linetype_bin(&mut reader).map_err(|e| malformed("op linetype", reader.position(), e))?;
                Ok(DxfMutation::InsertLinetype { index, linetype })
            }
            11 => Ok(DxfMutation::RemoveLinetype { name: read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))? }),
            12 => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let linetype = dec_linetype_bin(&mut reader).map_err(|e| malformed("op linetype", reader.position(), e))?;
                Ok(DxfMutation::SetLinetype { name, linetype })
            }
            13 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_dxf_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(DxfMutation::InsertEntity { index, entity })
            }
            14 => Ok(DxfMutation::RemoveEntity { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize }),
            15 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_dxf_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(DxfMutation::SetEntity { index, entity })
            }
            16 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DxfMutation::InsertBlock { index, block })
            }
            17 => Ok(DxfMutation::RemoveBlock { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize }),
            18 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DxfMutation::SetBlock { index, block })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `DxfMutation` values — one instance per variant (19 total, plus four
/// extra `InsertEntity` cases exercising the Polyline/Other/Solid/Insert entity kinds the base
/// `variants()` fixture didn't reach), incl. a `SetSnapshot` payload nesting a raw-retained
/// `other_tables` entry and a block with a nested entity — exercises the WHOLE grammar/protocol
/// tree end-to-end. The single source of truth reused by `op_text_binary_roundtrip_law` below AND
/// by `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law` conformance
/// tests, so a new variant only needs adding here once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DxfMutation> {
    use crate::artifacts::dxf::schema::snapshot::{DxfOtherTable, DxfTables, DxfTag, DxfValue, DxfVertex};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn demo_snapshot_for_set() -> DxfSnapshot {
        DxfSnapshot {
            schema: "stdio.dxf".into(),
            header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
            tables: DxfTables { layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }], styles: vec![], linetypes: vec![] },
            other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
            blocks: vec![DxfBlock { name: "B1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
        }
    }

    vec![
        DxfMutation::NoMutation,
        DxfMutation::SetSnapshot { snapshot: demo_snapshot_for_set() },
        DxfMutation::SetHeaderVar { name: "$ACADVER".into(), header_var: DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1015".into() }, extra_group_codes: vec![] } },
        DxfMutation::SetHeaderVar { name: "$NEWVAR".into(), header_var: DxfHeaderVar { name: "$NEWVAR".into(), group_code: 70, value: DxfValue::Int { value: 3 }, extra_group_codes: vec![(999, DxfValue::Str { value: "note".into() })] } },
        DxfMutation::RemoveHeaderVar { name: "$ACADVER".into() },
        DxfMutation::InsertLayer { index: 1, layer: DxfLayer { name: "L2".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } },
        DxfMutation::RemoveLayer { name: "0".into() },
        DxfMutation::SetLayer { name: "0".into(), layer: DxfLayer { name: "0".into(), color: 3, linetype: "DASHED".into(), flags: 1, unknown_group_codes: vec![] } },
        DxfMutation::InsertStyle { index: 1, style: DxfStyle { name: "S2".into(), flags: 0, font_name: "arial".into(), unknown_group_codes: vec![] } },
        DxfMutation::RemoveStyle { name: "STANDARD".into() },
        DxfMutation::SetStyle { name: "STANDARD".into(), style: DxfStyle { name: "STANDARD".into(), flags: 1, font_name: "romans".into(), unknown_group_codes: vec![] } },
        DxfMutation::InsertLinetype { index: 1, linetype: DxfLinetype { name: "DASHED".into(), flags: 0, description: "Dashed".into(), unknown_group_codes: vec![] } },
        DxfMutation::RemoveLinetype { name: "CONTINUOUS".into() },
        DxfMutation::SetLinetype { name: "CONTINUOUS".into(), linetype: DxfLinetype { name: "CONTINUOUS".into(), flags: 1, description: "Solid line".into(), unknown_group_codes: vec![] } },
        DxfMutation::InsertEntity { index: 0, entity: DxfEntity::Arc { center: [1.0, 1.0, 0.0], radius: 2.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] } },
        DxfMutation::RemoveEntity { index: 0 },
        DxfMutation::SetEntity { index: 0, entity: DxfEntity::Line { start: [9.0, 9.0, 0.0], end: [8.0, 8.0, 0.0], layer: "L2".into(), unknown_group_codes: vec![] } },
        DxfMutation::SetEntity { index: 1, entity: DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 1.0, value: "hi".into(), layer: "0".into(), unknown_group_codes: vec![] } },
        DxfMutation::InsertBlock { index: 0, block: DxfBlock { name: "B2".into(), base_point: [1.0, 1.0, 0.0], entities: vec![], unknown_group_codes: vec![] } },
        DxfMutation::RemoveBlock { index: 0 },
        DxfMutation::SetBlock {
            index: 0,
            block: DxfBlock { name: "B1".into(), base_point: [5.0, 5.0, 0.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] },
        },
        DxfMutation::InsertEntity {
            index: 1,
            entity: DxfEntity::Polyline {
                vertices: vec![DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] }, DxfVertex { x: 1.0, y: 0.0, z: 0.0, bulge: 0.5, unknown_group_codes: vec![(8, DxfValue::Str { value: "0".into() })] }],
                closed: true,
                layer: "0".into(),
                unknown_group_codes: vec![],
            },
        },
        DxfMutation::InsertEntity { index: 2, entity: DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] } },
        // 🧭️ `index: 2` (not 3/4): `base_snapshot()` (used by `mutation_diff_law`/`inverse_law`,
        // the two OTHER generic property tests sharing this fixture) has exactly 2 entities —
        // `Mutation::inverse`'s `InsertEntity` arm reads the index literally off the mutation's
        // own payload (comment above, `#region 🔖️MutationTrait`), which only round-trips when the
        // requested index is `<= base.len()` (no clamping inside `generic_apply`'s `idx.min(len)`
        // needed); an out-of-range literal index here would desync `inverse_law` for a fixture
        // this demo list is also reused by, not a grammar/protocol concern.
        DxfMutation::InsertEntity { index: 2, entity: DxfEntity::Solid { points: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], layer: "0".into(), unknown_group_codes: vec![] } },
        DxfMutation::InsertEntity { index: 2, entity: DxfEntity::Insert { block_name: "B1".into(), position: [1.0, 2.0, 3.0], scale: [1.0, 1.0, 1.0], rotation: 0.0, layer: "0".into(), unknown_group_codes: vec![] } },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dxf::schema::diff::DxfEntitiesDiff;
    use crate::artifacts::dxf::schema::snapshot::{DxfOtherTable, DxfTables, DxfTag, DxfValue, DxfVertex};
    use protocol::command::DiffAlgebra;

    #[semio_framework_async_macros::async_test]
    async fn missing_entity_target_is_rejected_before_mutation() {
        let base = DxfSnapshot::default();
        let diff = DxfDiff { entities: Some(DxfEntitiesDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
        let error = diff.apply(&base).expect_err("missing entity target must be rejected");
        assert_eq!(error.code, "invalid-remove-index");
        assert_eq!(error.target, vec!["entities", "0"]);
        assert_eq!(base, DxfSnapshot::default());
    }

    //#region 🔖️Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> DxfSnapshot {
        DxfSnapshot {
            schema: "stdio.dxf".into(),
            header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
            tables: DxfTables {
                layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
                styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
                linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
            },
            other_tables: vec![],
            blocks: vec![DxfBlock { name: "B1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] }],
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 5.0, layer: "0".into(), unknown_group_codes: vec![] }],
        }
    }

    /// 🔁️ Every DxfMutation-generic property test below (`mutation_diff_law`/`inverse_law`/
    /// `op_text_binary_roundtrip_law`) shares ONE fixture with `⚙️engine/🦀️component.rs`'s
    /// conformance laws — `demo_mutation_cases()` (`#region 🔖️DemoCases` above).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn variants() -> Vec<DxfMutation> {
        demo_mutation_cases()
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ Canonical "differs in every mutable field" snapshot A — every collection carries a
    /// stable-prefix item plus one that will be modified (index-keyed) or removed (name-keyed).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> DxfSnapshot {
        DxfSnapshot {
            schema: "stdio.dxf".into(),
            header_vars: vec![
                DxfHeaderVar { name: "$KEEP".into(), group_code: 1, value: DxfValue::Str { value: "same".into() }, extra_group_codes: vec![] },
                DxfHeaderVar { name: "$DROP".into(), group_code: 70, value: DxfValue::Int { value: 1 }, extra_group_codes: vec![] },
                DxfHeaderVar { name: "$MOD".into(), group_code: 40, value: DxfValue::Double { value: 1.0 }, extra_group_codes: vec![] },
            ],
            tables: DxfTables {
                layers: vec![
                    DxfLayer { name: "KEEP".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                    DxfLayer { name: "DROP".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                    DxfLayer { name: "MOD".into(), color: 2, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                ],
                styles: vec![DxfStyle { name: "S".into(), flags: 0, font_name: "a".into(), unknown_group_codes: vec![] }],
                linetypes: vec![DxfLinetype { name: "L".into(), flags: 0, description: "d".into(), unknown_group_codes: vec![] }],
            },
            other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
            blocks: vec![
                DxfBlock { name: "B0".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] },
                DxfBlock { name: "B1".into(), base_point: [1.0, 1.0, 1.0], entities: vec![DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] },
            ],
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }],
        }
    }

    /// 🧬️ Sweep B: every index-keyed collection's index-0 item is UNCHANGED, index-1 is MODIFIED
    /// in every field, and a brand-new item appears at the end — proven ADDED via `between(a,b)`
    /// (`b` is longer) and REMOVED via `between(b,a)`. Name-keyed collections show removed +
    /// modified + added simultaneously from ONE `between(a,b)` call. `entities[1]` changes KIND
    /// (Circle → Text), proving `DxfEntityDiff::Replace`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> DxfSnapshot {
        DxfSnapshot {
            schema: "stdio.dxf".into(),
            header_vars: vec![
                DxfHeaderVar { name: "$KEEP".into(), group_code: 1, value: DxfValue::Str { value: "same".into() }, extra_group_codes: vec![] },
                DxfHeaderVar { name: "$MOD".into(), group_code: 41, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![(999, DxfValue::Str { value: "note".into() })] },
                DxfHeaderVar { name: "$NEW".into(), group_code: 70, value: DxfValue::Int { value: 9 }, extra_group_codes: vec![] },
            ],
            tables: DxfTables {
                layers: vec![
                    DxfLayer { name: "KEEP".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                    DxfLayer { name: "MOD".into(), color: 5, linetype: "DASHED".into(), flags: 1, unknown_group_codes: vec![(1, DxfValue::Str { value: "x".into() })] },
                    DxfLayer { name: "NEW".into(), color: 3, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] },
                ],
                styles: vec![DxfStyle { name: "S".into(), flags: 1, font_name: "b".into(), unknown_group_codes: vec![] }],
                linetypes: vec![DxfLinetype { name: "L".into(), flags: 1, description: "e".into(), unknown_group_codes: vec![] }],
            },
            other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
            blocks: vec![
                DxfBlock { name: "B0".into(), base_point: [0.0, 0.0, 0.0], entities: vec![], unknown_group_codes: vec![] },
                DxfBlock { name: "B1renamed".into(), base_point: [9.0, 9.0, 9.0], entities: vec![], unknown_group_codes: vec![(5, DxfValue::Str { value: "h".into() })] },
                DxfBlock { name: "B2".into(), base_point: [2.0, 2.0, 2.0], entities: vec![], unknown_group_codes: vec![] },
            ],
            entities: vec![
                DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Text { position: [1.0, 1.0, 1.0], height: 2.0, value: "swapped-kind".into(), layer: "T".into(), unknown_group_codes: vec![] },
                DxfEntity::Arc { center: [0.0, 0.0, 0.0], radius: 3.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] },
            ],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for m in variants() {
            let diff = m.diff(&base);
            let expected = diff.diff().apply(&base).expect("valid mutation diff");

            let mut via_apply = base.clone();
            let returned_diff = apply_dxf_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_dxf_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        for m in variants() {
            let mut forward = base.clone();
            apply_dxf_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_dxf_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            let d = m.diff(&base);
            let mid = d.diff().apply(&base).expect("valid forward diff");
            let back = d.diff().inverse(&base).apply(&mid).expect("valid inverse diff");
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // 🧩 Insert(2)+Remove(0) on entities: the two-op sequence base → mid → after.
        let new_entity = DxfEntity::Arc { center: [0.0, 0.0, 0.0], radius: 1.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] };
        let d1 = DxfMutation::InsertEntity { index: 2, entity: new_entity.clone() }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = DxfMutation::RemoveEntity { index: 0 }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f)+Insert(2,g): both must survive.
        let d1 = DxfMutation::InsertEntity { index: 2, entity: new_entity.clone() }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let other_entity = DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 1.0, value: "g".into(), layer: "0".into(), unknown_group_codes: vec![] };
        let d2 = DxfMutation::InsertEntity { index: 2, entity: other_entity }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.entities.len(), base.entities.len() + 2, "both inserts must survive");

        // 🧩 Add+SetField (kind-preserving): patch into the added payload.
        let d1 = DxfMutation::InsertEntity { index: 1, entity: new_entity.clone() }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let patched = DxfEntity::Arc { center: [9.0, 9.0, 9.0], radius: 1.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] };
        let d2 = DxfMutation::SetEntity { index: 1, entity: patched }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+SetField absorb mismatch");
        match &after.entities[1] {
            DxfEntity::Arc { center, .. } => assert_eq!(*center, [9.0, 9.0, 9.0]),
            other => panic!("expected Arc, got {other:?}"),
        }

        // 🧩 Add+SetField ACROSS a kind change: patch-into-Replace (the entity-diff-specific
        // canonical case — SetEntity with a different kind produces `Replace`, which must still
        // absorb cleanly into a preceding Insert's carried payload).
        let d1 = DxfMutation::InsertEntity { index: 1, entity: new_entity.clone() }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let swapped = DxfEntity::Text { position: [0.0, 0.0, 0.0], height: 3.0, value: "swap".into(), layer: "0".into(), unknown_group_codes: vec![] };
        let d2 = DxfMutation::SetEntity { index: 1, entity: swapped.clone() }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Replace(kind-change) absorb mismatch");
        assert_eq!(after.entities[1], swapped);

        // 🧩 Modify+Remove: modifying then removing the same entity collapses to a removal.
        let d1 = DxfMutation::SetEntity { index: 1, entity: DxfEntity::Circle { center: [0.0, 0.0, 0.0], radius: 9.0, layer: "0".into(), unknown_group_codes: vec![] } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = DxfMutation::RemoveEntity { index: 1 }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Modify+Remove absorb mismatch");

        // 🧩 Name-keyed: Add layer + remove-of-added annihilates the add.
        let d1 = DxfMutation::InsertLayer { index: 2, layer: DxfLayer { name: "Fresh".into(), color: 1, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] } }.diff(&base);
        let mid = d1.diff().apply(&base).expect("valid first diff");
        let d2 = DxfMutation::RemoveLayer { name: "Fresh".into() }.diff(&mid);
        let after = d2.diff().apply(&mid).expect("valid second diff");
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).expect("valid absorbed diff"), after, "Add+Remove(name-keyed) absorb mismatch");
        assert_eq!(after.tables.layers, base.tables.layers, "add-then-remove of the same name must be a full no-op");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = DxfMutation::InsertEntity { index: 0, entity: new_entity.clone() }.diff(&base);
        let s1 = d1.diff().apply(&base).expect("valid first diff");
        let d2 = DxfMutation::SetEntity { index: 0, entity: DxfEntity::Circle { center: [2.0, 2.0, 2.0], radius: 4.0, layer: "0".into(), unknown_group_codes: vec![] } }.diff(&s1);
        let s2 = d2.diff().apply(&s1).expect("valid second diff");
        let d3 = DxfMutation::RemoveEntity { index: 2 }.diff(&s2);
        let s3 = d3.diff().apply(&s2).expect("valid third diff");

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("valid left diff"), s3);
        assert_eq!(right.apply(&base).expect("valid right diff"), s3);
        assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(DxfDiff::between(&a, &b).apply(&a).expect("valid forward diff"), b);
        assert_eq!(DxfDiff::between(&b, &a).apply(&b).expect("valid backward diff"), a);
        assert!(DxfDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();

        let d_ab = DxfDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) == b");
        let d_ba = DxfDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) == a");
        assert!(DxfDiff::between(&a, &a).is_empty());

        // 🔍 header_vars (name-keyed): removed + modified + added from ONE between(a,b) call.
        let hv = d_ab.header_vars.as_ref().expect("header_vars diff populated");
        assert_eq!(hv.removed, vec!["$DROP".to_string()]);
        assert!(!hv.modified.is_empty() && !hv.added.is_empty());
        let hvm = &hv.modified.iter().find(|m| m.name == "$MOD").expect("$MOD modified").diff;
        assert!(hvm.group_code.is_some() && hvm.value.is_some() && hvm.extra_group_codes.is_some(), "every DxfHeaderVarDiff field must be patched");

        // 🔍 layers (name-keyed).
        let ld = d_ab.tables.as_ref().and_then(|t| t.layers.as_ref()).expect("layers diff populated");
        assert_eq!(ld.removed, vec!["DROP".to_string()]);
        assert!(!ld.modified.is_empty() && !ld.added.is_empty());
        let lm = &ld.modified.iter().find(|m| m.name == "MOD").expect("MOD layer modified").diff;
        assert!(lm.color.is_some() && lm.linetype.is_some() && lm.flags.is_some() && lm.unknown_group_codes.is_some());

        // 🔍 styles/linetypes (name-keyed, single-entry modify).
        let sd = d_ab.tables.as_ref().and_then(|t| t.styles.as_ref()).expect("styles diff populated");
        assert!(!sd.modified.is_empty());
        assert!(sd.modified[0].diff.flags.is_some() && sd.modified[0].diff.font_name.is_some());
        let ltd = d_ab.tables.as_ref().and_then(|t| t.linetypes.as_ref()).expect("linetypes diff populated");
        assert!(!ltd.modified.is_empty());
        assert!(ltd.modified[0].diff.flags.is_some() && ltd.modified[0].diff.description.is_some());

        // 🔍 blocks (index-keyed): modified+added from between(a,b); modified+removed from between(b,a).
        let bd_ab = d_ab.blocks.as_ref().expect("blocks diff populated (a->b)");
        assert!(bd_ab.removed.is_empty() && !bd_ab.modified.is_empty() && !bd_ab.added.is_empty());
        let bm = &bd_ab.modified[0].diff;
        assert!(bm.name.is_some() && bm.base_point.is_some() && bm.unknown_group_codes.is_some(), "every DxfBlockDiff scalar field must be patched");
        let bd_ba = d_ba.blocks.as_ref().expect("blocks diff populated (b->a)");
        assert!(!bd_ba.removed.is_empty() && !bd_ba.modified.is_empty() && bd_ba.added.is_empty());

        // 🔍 entities (index-keyed): modified (kind-preserving Line stays Line) + added from
        // between(a,b); Text(index 1) proves the kind-change `Replace` path.
        let ed_ab = d_ab.entities.as_ref().expect("entities diff populated (a->b)");
        assert!(ed_ab.removed.is_empty() && !ed_ab.modified.is_empty() && !ed_ab.added.is_empty());
        let em1 = &ed_ab.modified.iter().find(|m| m.index == 1).expect("entities[1] modified").diff;
        assert!(matches!(em1, crate::artifacts::dxf::schema::diff::DxfEntityDiff::Replace { .. }), "kind change (Circle->Text) must be a Replace");
        let ed_ba = d_ba.entities.as_ref().expect("entities diff populated (b->a)");
        assert!(!ed_ba.removed.is_empty() && !ed_ba.modified.is_empty() && ed_ba.added.is_empty());
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️VertexUnknownGroupCodesRetained
    /// 🕳️ `DxfVertex.unknown_group_codes` participates in equality (weak leaf, whole-vec
    /// replaced by the parent `Polyline` diff — confirms it isn't silently dropped by the
    /// snapshot type even though no dedicated mutation targets it directly).
    #[semio_framework_async_macros::async_test]
    async fn vertex_unknown_group_codes_are_part_of_equality() {
        let v1 = DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] };
        let v2 = DxfVertex { x: 0.0, y: 0.0, z: 0.0, bulge: 0.0, unknown_group_codes: vec![(40, DxfValue::Double { value: 1.0 })] };
        assert_ne!(v1, v2);
    }
    //#endregion 🔖️VertexUnknownGroupCodesRetained

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ `OpText`/`OpBinary` round-trip laws over the hand-rolled `DxfMutation` grammar — every
    /// variant from the existing `variants()` fixture, including `SetSnapshot` (exercises the
    /// whole-snapshot grammar, incl. `other_tables` raw retention and a nested block's own
    /// entities) and every typed-entity/table Insert/Set/Remove keyword.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for m in variants() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must never contain a newline, for {m:?}");
            let parsed = DxfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e:?}, for {m:?}"));
            assert_eq!(parsed, m, "parse_op(print_op(m)) == m");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e:?}, for {m:?}"));
            let decoded = DxfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e:?}, for {m:?}"));
            assert_eq!(decoded, m, "decode_op(encode_op(m)) == m");

            let printed2 = m.print_op();
            assert_eq!(printed, printed2, "print_op must be deterministic, for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️KindsCatalogLaw
    /// 🧾️ `KINDS` matches the enum's own variant set (via `demo_mutation_cases`' one-instance-per-
    /// variant coverage) and every entry parses/prints as its own keyword -- what keeps
    /// `../../🧪️oracle/🔣️component.json`'s `mutationCatalogs[].kinds` honest against Rust, per the
    /// wave 7 fleet brief's registration rule ("the framework never parses Rust").
    #[semio_framework_async_macros::async_test]
    async fn kinds_const_matches_enum_variants_in_declaration_order() {
        assert_eq!(KINDS.len(), 19, "DxfMutation has 19 variants");
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for m in demo_mutation_cases() {
            let printed = m.print_op();
            let keyword = printed.split(' ').next().unwrap_or_default();
            assert!(KINDS.contains(&keyword), "KINDS is missing {keyword:?} for {m:?}");
            seen.insert(keyword.to_string());
        }
        assert_eq!(seen.len(), KINDS.len(), "every KINDS entry must be exercised by demo_mutation_cases()");
        for kind in KINDS {
            assert!(seen.contains(*kind), "KINDS entry {kind:?} has no demo_mutation_cases() coverage");
        }
    }
    //#endregion 🔖️KindsCatalogLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/widens-the-circle-entity-radius/🦀️component.rs"]
mod set_snapshot_widens_the_circle_entity_radius;
//#endregion 🧪️FixtureCases
