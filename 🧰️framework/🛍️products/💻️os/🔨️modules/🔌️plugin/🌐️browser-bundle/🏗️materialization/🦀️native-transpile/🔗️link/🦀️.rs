//! 🔗️ Split link: appends a thin plugin module to a cached framework image core module and componentizes the result, so a plugin-only edit never relinks or
//! re-codegens the framework closure.
//!
//! The plugin module is linked by `wasm-ld` from the plugin crates' objects only (`--no-entry --import-memory --import-table --allow-undefined --emit-relocs
//! --global-base=1024 --table-base=<image table size>`): its data lies in the image's reservation below the image data (`--global-base` of the image), its
//! table entries after the image's, undefined functions are `env` imports, and undefined data is reached through `GOT.data.internal.<symbol>` globals that
//! `wasm-ld` leaves at 0. The framework image is copied byte for byte; the plugin's imports bind to image exports by symbol name, its stack pointer becomes the
//! image's, and each unresolved GOT global takes the image's address of its symbol. Contract: `🧬️schema/🔣️.json`; design: ticket
//! 26/11/27/SPLIT-PLUGIN-WASM-INTO-SHARED-FRAMEWORK-COMPONENT-AND-THIN-PLUGIN-COMPONENTS.
//!
//! See <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md> and <https://docs.rs/wit-component/0.259.0>.

use anyhow::{anyhow, bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::path::PathBuf;
use std::time::Instant;
use wasm_encoder::reencode::{Error as ReencodeError, Reencode};
use wasm_encoder::{CodeSection, ConstExpr, DataSection, ElementSection, Elements, Encode, ExportKind, ExportSection, Function, FunctionSection, GlobalSection, GlobalType, Instruction, NameMap, NameSection, RawSection, Section, TableSection, TableType, TypeSection, ValType};
use wasmparser::{ExternalKind, KnownCustom, Linking, Name, Operator, Parser, Payload, RelocationType, SymbolInfo, TypeRef};

/// 🚪️ Exit code of a link the framework image cannot serve; stderr then lists the missing symbols, one per line after the header.
pub const EXIT_IMAGE_LACKS_SYMBOLS: i32 = 3;

/// 🧾️ Plugin import symbols the image must export, as `wasm-ld` response lines (`--export=<symbol>`), sorted and unique.
pub fn image_exports(plugin: &[u8]) -> Result<Vec<String>> {
    let module = PluginModule::parse(plugin)?;
    let mut symbols: BTreeSet<String> = module.imports.iter().filter(|i| i.module == "env" && matches!(i.ty, TypeRef::Func(_) | TypeRef::FuncExact(_))).map(|i| i.name.clone()).collect();
    symbols.extend(module.unresolved_data().map(|(_, symbol)| symbol.to_owned()));
    symbols.insert("__stack_pointer".to_owned());
    Ok(symbols.into_iter().map(|s| format!("--export={s}")).collect())
}

/// 🧾️ Rust declarations the image root compiles so the image imports every non-`env` function the plugin imports (the canonical ABI intrinsics of the
/// plugin's WIT exports); the plugin binds to those image imports, so the image's function indices stay fixed. Sorted by module and name.
pub fn image_imports_rust(plugin: &[u8]) -> Result<String> {
    let module = PluginModule::parse(plugin)?;
    let types: Vec<&wasmparser::SubType> = module.types.iter().flat_map(|g| g.types()).collect();
    let mut rows: BTreeMap<(String, String), (Vec<wasmparser::ValType>, Vec<wasmparser::ValType>)> = BTreeMap::new();
    for import in module.imports.iter().filter(|i| i.module != "env") {
        let (TypeRef::Func(ty) | TypeRef::FuncExact(ty)) = import.ty else { continue };
        let wasmparser::CompositeInnerType::Func(func) = &types[ty as usize].composite_type.inner else { bail!("import {}#{} has no function type", import.module, import.name) };
        rows.insert((import.module.clone(), import.name.clone()), (func.params().to_vec(), func.results().to_vec()));
    }
    let rust = |t: &wasmparser::ValType| Ok(match t { wasmparser::ValType::I32 => "i32", wasmparser::ValType::I64 => "i64", wasmparser::ValType::F32 => "f32", wasmparser::ValType::F64 => "f64", other => bail!("import type {other:?} has no Rust spelling") });
    let mut out = String::from("//! 🧾️ Imports of the plugin module that the framework image declares (`semio-native-transpile image-exports --imports`).\n");
    for (k, ((module, name), (params, results))) in rows.iter().enumerate() {
        if results.len() > 1 { bail!("import {module}#{name} returns several values") }
        let args = params.iter().enumerate().map(|(i, t)| Ok(format!("p{i}: {}", rust(t)?))).collect::<Result<Vec<_>>>()?.join(", ");
        let ret = results.first().map(|t| Ok::<_, anyhow::Error>(format!(" -> {}", rust(t)?))).transpose()?.unwrap_or_default();
        out.push_str(&format!("#[link(wasm_import_module = {module:?})]\nunsafe extern \"C\" {{ #[link_name = {name:?}] fn __semio_image_import_{k}({args}){ret}; }}\n"));
    }
    let anchors = (0..rows.len()).map(|k| format!("__semio_image_import_{k} as usize")).collect::<Vec<_>>();
    out.push_str(&format!("#[unsafe(no_mangle)]\npub extern \"C\" fn __semio_image_imports() -> usize {{ {} }}\n", if anchors.is_empty() { "0".to_owned() } else { anchors.join(" ^ ") }));
    Ok(out)
}

/// 🔣️ Prefix `wasm-ld` gives the internal GOT global of a data symbol in a non-shared link.
const GOT_DATA: &str = "GOT.data.internal.";

/// ⚠️ The image does not export every symbol the plugin imports; the image must be relinked exporting `missing`.
#[derive(Debug)]
pub struct ImageLacksSymbols {
    pub missing: Vec<String>,
}

impl std::fmt::Display for ImageLacksSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "framework image lacks {} plugin import(s)", self.missing.len())?;
        for symbol in &self.missing { writeln!(f, "{symbol}")?; }
        Ok(())
    }
}

impl std::error::Error for ImageLacksSymbols {}

fn leb_len(bytes: &[u8]) -> usize {
    bytes.iter().position(|b| b & 0x80 == 0).map_or(0, |p| p + 1)
}

fn span(range: Range<u64>) -> Range<usize> {
    range.start as usize..range.end as usize
}

fn entries_after_count(bytes: &[u8], range: Range<u64>) -> Range<usize> {
    let range = span(range);
    range.start + leb_len(&bytes[range.start..])..range.end
}

fn const_value(expr: &wasmparser::ConstExpr, global: &dyn Fn(u32) -> Option<i64>) -> Result<i64> {
    let mut stack: Vec<i64> = Vec::new();
    let binary = |stack: &mut Vec<i64>, op: fn(i32, i32) -> i32| -> Result<()> {
        let (b, a) = (stack.pop().ok_or_else(|| anyhow!("const expr underflow"))?, stack.pop().ok_or_else(|| anyhow!("const expr underflow"))?);
        stack.push(op(a as i32, b as i32) as i64);
        Ok(())
    };
    for op in expr.get_operators_reader() {
        match op? {
            Operator::I32Const { value } => stack.push(value as i64),
            Operator::I64Const { value } => stack.push(value),
            Operator::GlobalGet { global_index } => stack.push(global(global_index).ok_or_else(|| anyhow!("const expr reads non-constant global {global_index}"))?),
            Operator::I32Add => binary(&mut stack, i32::wrapping_add)?,
            Operator::I32Sub => binary(&mut stack, i32::wrapping_sub)?,
            Operator::I32Mul => binary(&mut stack, i32::wrapping_mul)?,
            Operator::End => {}
            other => bail!("unsupported const operator {other:?}"),
        }
    }
    stack.pop().ok_or_else(|| anyhow!("empty const expr"))
}

/// 🖼️ The framework image as copied ranges plus the few facts the link needs.
#[derive(Default)]
struct FrameworkImage<'a> {
    bytes: &'a [u8],
    types: u32,
    type_groups: (Range<usize>, u32),
    imports: Option<Range<usize>>,
    func_imports: u32,
    imported_functions: Vec<(String, String)>,
    global_imports: u32,
    functions: (Range<usize>, u32),
    table_minimum: Option<u64>,
    memories: Option<Range<usize>>,
    globals: (Range<usize>, u32),
    global_values: Vec<Option<i64>>,
    exports: Vec<(String, ExternalKind, u32)>,
    start: Option<u32>,
    elements: (Range<usize>, u32),
    data_count: Option<u32>,
    code: (Range<usize>, u32),
    data: (Range<usize>, u32),
    data_floor: Option<i64>,
    customs: Vec<(String, &'a [u8])>,
    names: BTreeMap<u32, String>,
}

impl<'a> FrameworkImage<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self> {
        let mut image = FrameworkImage { bytes, ..Default::default() };
        for payload in Parser::new(0).parse_all(bytes) {
            match payload? {
                Payload::TypeSection(r) => {
                    image.type_groups = (entries_after_count(bytes, r.range()), r.count());
                    for group in r { image.types += group?.types().len() as u32; }
                }
                Payload::ImportSection(r) => {
                    image.imports = Some(span(r.range()));
                    for import in r.into_imports() {
                        let import = import?;
                        match import.ty {
                            TypeRef::Func(_) | TypeRef::FuncExact(_) => { image.func_imports += 1; image.imported_functions.push((import.module.to_owned(), import.name.to_owned())); }
                            TypeRef::Global(_) => { image.global_imports += 1; image.global_values.push(None); }
                            TypeRef::Table(_) => bail!("the framework image must define its table (--export-table --growable-table), not import it"),
                            _ => {}
                        }
                    }
                }
                Payload::FunctionSection(r) => image.functions = (entries_after_count(bytes, r.range()), r.count()),
                Payload::TableSection(r) => {
                    if r.count() != 1 { bail!("the framework image must define exactly one table") }
                    for table in r {
                        let table = table?;
                        if table.ty.maximum.is_some() { bail!("the framework image table must be growable (--growable-table)") }
                        image.table_minimum = Some(table.ty.initial);
                    }
                }
                Payload::MemorySection(r) => image.memories = Some(span(r.range())),
                Payload::GlobalSection(r) => {
                    image.globals = (entries_after_count(bytes, r.range()), r.count());
                    for global in r {
                        let global = global?;
                        let known = image.global_values.clone();
                        image.global_values.push(const_value(&global.init_expr, &|i| known.get(i as usize).copied().flatten()).ok());
                    }
                }
                Payload::ExportSection(r) => for export in r { let export = export?; image.exports.push((export.name.to_owned(), export.kind, export.index)); },
                Payload::StartSection { func, .. } => image.start = Some(func),
                Payload::ElementSection(r) => image.elements = (entries_after_count(bytes, r.range()), r.count()),
                Payload::DataCountSection { count, .. } => image.data_count = Some(count),
                Payload::CodeSectionStart { count, range, .. } => image.code = (entries_after_count(bytes, range), count),
                Payload::DataSection(r) => {
                    image.data = (entries_after_count(bytes, r.range()), r.count());
                    for data in r {
                        if let wasmparser::DataKind::Active { offset_expr, .. } = data?.kind {
                            let offset = const_value(&offset_expr, &|_| None)?;
                            image.data_floor = Some(image.data_floor.map_or(offset, |floor| floor.min(offset)));
                        }
                    }
                }
                Payload::CustomSection(c) => match c.as_known() {
                    KnownCustom::Name(names) => for name in names { if let Name::Function(map) = name? { for naming in map { let naming = naming?; image.names.insert(naming.index, naming.name.to_owned()); } } },
                    KnownCustom::Dylink0(_) => bail!("the framework image must be a static module, not a shared library"),
                    _ => image.customs.push((c.name().to_owned(), c.data())),
                },
                _ => {}
            }
        }
        Ok(image)
    }

    fn export(&self, name: &str, kind: ExternalKind) -> Option<u32> {
        self.exports.iter().find(|(n, k, _)| n == name && *k == kind).map(|(_, _, i)| *i)
    }
}

struct Import {
    module: String,
    name: String,
    ty: TypeRef,
}

/// 🧩️ The plugin module: a static `wasm-ld` link of the plugin crates that imports its memory, its table and every framework function.
struct PluginModule<'a> {
    types: Vec<wasmparser::RecGroup>,
    imports: Vec<Import>,
    functions: Vec<u32>,
    globals: Vec<wasmparser::Global<'a>>,
    global_names: BTreeMap<u32, String>,
    exports: Vec<(String, ExternalKind, u32)>,
    elements: Vec<wasmparser::Element<'a>>,
    data: Vec<wasmparser::Data<'a>>,
    data_count: bool,
    bodies: Vec<wasmparser::FunctionBody<'a>>,
    component_types: Vec<(String, &'a [u8])>,
    names: BTreeMap<u32, String>,
    unresolved_relocations: BTreeSet<String>,
}

impl<'a> PluginModule<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self> {
        let mut module = PluginModule { types: Vec::new(), imports: Vec::new(), functions: Vec::new(), globals: Vec::new(), global_names: BTreeMap::new(), exports: Vec::new(), elements: Vec::new(), data: Vec::new(), data_count: false, bodies: Vec::new(), component_types: Vec::new(), names: BTreeMap::new(), unresolved_relocations: BTreeSet::new() };
        let mut undefined_data: BTreeMap<u32, String> = BTreeMap::new();
        let mut relocations = Vec::new();
        for payload in Parser::new(0).parse_all(bytes) {
            match payload? {
                Payload::TypeSection(r) => for group in r { module.types.push(group?); },
                Payload::ImportSection(r) => for import in r.into_imports() {
                    let import = import?;
                    match import.ty {
                        TypeRef::Func(_) | TypeRef::FuncExact(_) | TypeRef::Memory(_) | TypeRef::Table(_) => {}
                        other => bail!("the plugin imports {other:?} {}#{}: link it as a static module (no -shared)", import.module, import.name),
                    }
                    module.imports.push(Import { module: import.module.to_owned(), name: import.name.to_owned(), ty: import.ty });
                },
                Payload::FunctionSection(r) => for ty in r { module.functions.push(ty?); },
                Payload::GlobalSection(r) => for global in r { module.globals.push(global?); },
                Payload::ExportSection(r) => for export in r { let export = export?; module.exports.push((export.name.to_owned(), export.kind, export.index)); },
                Payload::ElementSection(r) => for element in r { module.elements.push(element?); },
                Payload::DataSection(r) => for data in r { module.data.push(data?); },
                Payload::DataCountSection { .. } => module.data_count = true,
                Payload::CodeSectionEntry(body) => module.bodies.push(body),
                Payload::StartSection { .. } => bail!("the plugin module must not have a start function"),
                Payload::MemorySection(_) | Payload::TableSection(_) => bail!("the plugin module must import its memory and table (--import-memory --import-table)"),
                Payload::TagSection(_) => bail!("the plugin module must not define exception tags"),
                Payload::CustomSection(c) => match c.as_known() {
                    KnownCustom::Dylink0(_) => bail!("the plugin module must be a static link, not a shared library"),
                    KnownCustom::Name(names) => for name in names {
                        match name? {
                            Name::Function(map) => for naming in map { let naming = naming?; module.names.insert(naming.index, naming.name.to_owned()); },
                            Name::Global(map) => for naming in map { let naming = naming?; module.global_names.insert(naming.index, naming.name.to_owned()); },
                            _ => {}
                        }
                    },
                    KnownCustom::Linking(r) => for sub in r.subsections() {
                        if let Linking::SymbolTable(symbols) = sub? {
                            for (index, symbol) in symbols.into_iter().enumerate() {
                                if let SymbolInfo::Data { flags, name, symbol: None } = symbol? { if flags.contains(wasmparser::SymbolFlags::UNDEFINED) { undefined_data.insert(index as u32, name.to_owned()); } }
                            }
                        }
                    },
                    KnownCustom::Reloc(r) => for entry in r.entries() { relocations.push(entry?); },
                    _ if c.name().starts_with("component-type") => module.component_types.push((c.name().to_owned(), c.data())),
                    _ => {}
                },
                _ => {}
            }
        }
        for entry in relocations {
            let direct = matches!(entry.ty, RelocationType::MemoryAddrLeb | RelocationType::MemoryAddrSleb | RelocationType::MemoryAddrI32 | RelocationType::MemoryAddrLeb64 | RelocationType::MemoryAddrSleb64 | RelocationType::MemoryAddrI64 | RelocationType::MemoryAddrLocrelI32);
            if let (true, Some(symbol)) = (direct, undefined_data.get(&entry.index)) { module.unresolved_relocations.insert(symbol.clone()); }
        }
        if !module.unresolved_relocations.is_empty() {
            bail!("the plugin module addresses framework data directly ({}): only GOT access can bind to the framework image", module.unresolved_relocations.iter().cloned().collect::<Vec<_>>().join(", "));
        }
        Ok(module)
    }

    fn export(&self, name: &str, kind: ExternalKind) -> Option<u32> {
        self.exports.iter().find(|(n, k, _)| n == name && *k == kind).map(|(_, _, i)| *i)
    }

    fn func_imports(&self) -> usize {
        self.imports.iter().filter(|i| matches!(i.ty, TypeRef::Func(_) | TypeRef::FuncExact(_))).count()
    }

    fn unresolved_data(&self) -> impl Iterator<Item = (u32, &str)> + '_ {
        self.global_names.iter().filter_map(|(index, name)| {
            let symbol = name.strip_prefix(GOT_DATA)?;
            let unresolved = matches!(self.globals.get(*index as usize).map(|g| const_value(&g.init_expr, &|_| None)), Some(Ok(0)));
            (unresolved && symbol != "__memory_base" && symbol != "__table_base").then_some((*index, symbol))
        })
    }
}

struct IndexMap {
    types: u32,
    functions: Vec<u32>,
    globals: Vec<u32>,
    data: u32,
    elements: u32,
}

impl Reencode for IndexMap {
    type Error = anyhow::Error;
    fn type_index(&mut self, ty: u32) -> Result<u32, ReencodeError<anyhow::Error>> { Ok(self.types + ty) }
    fn function_index(&mut self, func: u32) -> Result<u32, ReencodeError<anyhow::Error>> { Ok(self.functions[func as usize]) }
    fn global_index(&mut self, global: u32) -> Result<u32, ReencodeError<anyhow::Error>> { Ok(self.globals[global as usize]) }
    fn data_index(&mut self, data: u32) -> Result<u32, ReencodeError<anyhow::Error>> { Ok(self.data + data) }
    fn element_index(&mut self, element: u32) -> Result<u32, ReencodeError<anyhow::Error>> { Ok(self.elements + element) }
}

fn reencode_error(error: ReencodeError<anyhow::Error>) -> anyhow::Error {
    anyhow!("{error:?}")
}

fn entries(section: &impl Encode) -> Vec<u8> {
    let mut out = Vec::new();
    section.encode(&mut out);
    let size = leb_len(&out);
    let count = leb_len(&out[size..]);
    out.split_off(size + count)
}

fn put(out: &mut Vec<u8>, section: &impl Section) {
    out.push(section.id());
    section.encode(out);
}

fn put_joined(out: &mut Vec<u8>, id: u8, count: u32, parts: &[&[u8]]) {
    let mut payload = Vec::new();
    count.encode(&mut payload);
    for part in parts { payload.extend_from_slice(part); }
    put(out, &RawSection { id, data: &payload });
}

fn is_component_export(name: &str) -> bool {
    !(name.starts_with("_R") || name.starts_with("_ZN") || name.starts_with("__") || name.starts_with("GOT."))
}

/// 📐️ Layout facts of one link, reported in the receipt.
pub struct Layout {
    pub data_start: i64,
    pub data_end: i64,
    pub reserve_end: i64,
    pub table_base: i64,
    pub table_end: i64,
    pub got_data: usize,
    pub bound_functions: usize,
    pub plugin_functions: usize,
}

/// 🔗️ Merges `plugin` into `framework` and returns the validated core module.
pub fn merge(framework: &[u8], plugin: &[u8]) -> Result<(Vec<u8>, Layout)> {
    let image = FrameworkImage::parse(framework)?;
    let plugin = PluginModule::parse(plugin)?;
    let table_base = image.table_minimum.ok_or_else(|| anyhow!("the framework image defines no table: link it with --export-table --growable-table"))? as i64;
    let reserve_end = image.data_floor.unwrap_or(0);
    let image_functions = image.func_imports + image.functions.1;
    let first_new_global = image.global_imports + image.globals.1;
    let mut missing = BTreeSet::new();
    let mut functions: Vec<u32> = plugin.imports.iter().filter(|i| matches!(i.ty, TypeRef::Func(_) | TypeRef::FuncExact(_))).map(|i| {
        let bound = if i.module == "env" { image.export(&i.name, ExternalKind::Func) } else { image.imported_functions.iter().position(|(m, n)| *m == i.module && *n == i.name).map(|k| k as u32) };
        bound.unwrap_or_else(|| { missing.insert(if i.module == "env" { i.name.clone() } else { format!("{}#{}", i.module, i.name) }); 0 })
    }).collect();
    let bound_functions = functions.len();
    functions.extend((0..plugin.functions.len() as u32).map(|k| image_functions + k));
    let initialize = image_functions + plugin.functions.len() as u32;
    let stack_pointer = plugin.export("__stack_pointer", ExternalKind::Global).ok_or_else(|| anyhow!("the plugin module must export __stack_pointer (--export=__stack_pointer)"))?;
    let image_stack_pointer = image.export("__stack_pointer", ExternalKind::Global).ok_or_else(|| anyhow!("the framework image must export __stack_pointer"))?;
    let mut addresses: BTreeMap<u32, i64> = BTreeMap::new();
    for (index, symbol) in plugin.unresolved_data() {
        match image.export(symbol, ExternalKind::Global).map(|g| image.global_values[g as usize]) {
            Some(Some(address)) => { addresses.insert(index, address); }
            Some(None) => bail!("framework data symbol {symbol} is not a constant address"),
            None => { missing.insert(symbol.to_owned()); }
        }
    }
    if !missing.is_empty() { return Err(ImageLacksSymbols { missing: missing.into_iter().collect() }.into()); }
    let mut kept = Vec::new();
    let mut globals = Vec::with_capacity(plugin.globals.len());
    for index in 0..plugin.globals.len() as u32 {
        if index == stack_pointer { globals.push(image_stack_pointer); } else { globals.push(first_new_global + kept.len() as u32); kept.push(index); }
    }
    let values: Vec<Option<i64>> = (0..plugin.globals.len() as u32).map(|i| addresses.get(&i).copied().or_else(|| const_value(&plugin.globals[i as usize].init_expr, &|_| None).ok())).collect();
    let resolve = |index: u32| values.get(index as usize).copied().flatten();
    let mut data_start = i64::MAX;
    let mut data_end = 0i64;
    for data in &plugin.data {
        if let wasmparser::DataKind::Active { offset_expr, .. } = &data.kind {
            let offset = const_value(offset_expr, &|_| None)?;
            data_start = data_start.min(offset);
            data_end = data_end.max(offset + data.data.len() as i64);
        }
    }
    if data_end > 0 && (data_start < 1024 || data_end > reserve_end) {
        bail!("the plugin data spans [{data_start}, {data_end}) but the framework image reserves [1024, {reserve_end}): link the plugin with --global-base=1024 and the image with a larger --global-base");
    }
    let mut table_end = table_base;
    for element in &plugin.elements {
        let wasmparser::ElementKind::Active { offset_expr, .. } = &element.kind else { bail!("the plugin module has a passive or declared element segment") };
        let offset = const_value(offset_expr, &|_| None)?;
        if offset < table_base { bail!("the plugin table entries start at {offset} inside the framework image table: link the plugin with --table-base={table_base}") }
        let count = match element.items.clone() { wasmparser::ElementItems::Functions(r) => r.count(), wasmparser::ElementItems::Expressions(_, r) => r.count() };
        table_end = table_end.max(offset + count as i64);
    }
    let mut map = IndexMap { types: image.types, functions, globals, data: image.data.1, elements: image.elements.1 };

    let mut types = TypeSection::new();
    for group in &plugin.types { map.parse_recursive_type_group(types.ty(), group.clone()).map_err(reencode_error)?; }
    let initialize_type = image.types + plugin.types.iter().map(|g| g.types().len() as u32).sum::<u32>();
    types.ty().function([], []);
    let mut function_section = FunctionSection::new();
    for ty in &plugin.functions { function_section.function(image.types + ty); }
    function_section.function(initialize_type);
    let mut global_section = GlobalSection::new();
    for index in &kept {
        let global = &plugin.globals[*index as usize];
        let ty = map.global_type(global.ty).map_err(reencode_error)?;
        let value = resolve(*index).ok_or_else(|| anyhow!("plugin global {index} has no constant initializer"))?;
        let init = if ty.val_type == ValType::I64 { ConstExpr::i64_const(value) } else { ConstExpr::i32_const(value as i32) };
        global_section.global(ty, &init);
    }
    let mut element_section = ElementSection::new();
    for element in &plugin.elements {
        let wasmparser::ElementKind::Active { offset_expr, .. } = &element.kind else { unreachable!() };
        let wasmparser::ElementItems::Functions(items) = element.items.clone() else { bail!("the plugin module has an expression element segment") };
        let items = items.into_iter().map(|f| Ok(map.functions[f? as usize])).collect::<Result<Vec<u32>>>()?;
        element_section.active(None, &ConstExpr::i32_const(const_value(offset_expr, &|_| None)? as i32), Elements::Functions(items.into()));
    }
    let mut data_section = DataSection::new();
    for data in &plugin.data {
        match &data.kind {
            wasmparser::DataKind::Active { offset_expr, .. } => { data_section.active(0, &ConstExpr::i32_const(const_value(offset_expr, &|_| None)? as i32), data.data.iter().copied()); }
            wasmparser::DataKind::Passive => { data_section.passive(data.data.iter().copied()); }
        }
    }
    let mut code = CodeSection::new();
    for body in &plugin.bodies { map.parse_function_body(&mut code, body.clone()).map_err(reencode_error)?; }
    let mut init = Function::new([]);
    if let Some(index) = image.export("_initialize", ExternalKind::Func) { init.instruction(&Instruction::Call(index)); }
    if let Some(index) = plugin.export("__wasm_call_ctors", ExternalKind::Func) { init.instruction(&Instruction::Call(map.functions[index as usize])); }
    init.instruction(&Instruction::End);
    code.function(&init);

    let mut bound: std::collections::HashSet<&str> = plugin.imports.iter().map(|i| i.name.as_str()).collect();
    bound.extend(plugin.unresolved_data().map(|(_, symbol)| symbol));
    let mut exports = ExportSection::new();
    let mut exported = std::collections::HashSet::new();
    for (name, kind, index) in &image.exports {
        let kept = match kind {
            ExternalKind::Func => (is_component_export(name) || !bound.contains(name.as_str())) && name != "_initialize",
            ExternalKind::Global => false,
            _ => true,
        };
        if kept { exports.export(name, map.export_kind(*kind).map_err(reencode_error)?, *index); exported.insert(name.clone()); }
    }
    for (name, kind, index) in &plugin.exports {
        if *kind != ExternalKind::Func || !is_component_export(name) { continue; }
        if !exported.insert(name.clone()) { bail!("plugin export {name} collides with a framework image export") }
        exports.export(name, ExportKind::Func, map.functions[*index as usize]);
    }
    exports.export("_initialize", ExportKind::Func, initialize);

    let bytes = image.bytes;
    let mut out = Vec::with_capacity(bytes.len() + plugin.bodies.iter().map(|b| span(b.range()).len()).sum::<usize>() + (1 << 20));
    out.extend_from_slice(&bytes[..8]);
    put_joined(&mut out, 1, image.type_groups.1 + plugin.types.len() as u32 + 1, &[&bytes[image.type_groups.0.clone()], &entries(&types)]);
    if let Some(range) = &image.imports { put(&mut out, &RawSection { id: 2, data: &bytes[range.clone()] }); }
    put_joined(&mut out, 3, image.functions.1 + plugin.functions.len() as u32 + 1, &[&bytes[image.functions.0.clone()], &entries(&function_section)]);
    let mut tables = TableSection::new();
    tables.table(TableType { element_type: wasm_encoder::RefType::FUNCREF, table64: false, minimum: table_end as u64, maximum: None, shared: false });
    put(&mut out, &tables);
    if let Some(range) = &image.memories { put(&mut out, &RawSection { id: 5, data: &bytes[range.clone()] }); }
    put_joined(&mut out, 6, image.globals.1 + kept.len() as u32, &[&bytes[image.globals.0.clone()], &entries(&global_section)]);
    put(&mut out, &exports);
    if let Some(function_index) = image.start { put(&mut out, &wasm_encoder::StartSection { function_index }); }
    put_joined(&mut out, 9, image.elements.1 + element_section.len(), &[&bytes[image.elements.0.clone()], &entries(&element_section)]);
    if image.data_count.is_some() || plugin.data_count { put(&mut out, &wasm_encoder::DataCountSection { count: image.data.1 + plugin.data.len() as u32 }); }
    put_joined(&mut out, 10, image.code.1 + plugin.bodies.len() as u32 + 1, &[&bytes[image.code.0.clone()], &entries(&code)]);
    put_joined(&mut out, 11, image.data.1 + plugin.data.len() as u32, &[&bytes[image.data.0.clone()], &entries(&data_section)]);
    for (name, data) in image.customs.iter().chain(plugin.component_types.iter()) { put(&mut out, &wasm_encoder::CustomSection { name: name.into(), data: (*data).into() }); }
    let mut names = NameMap::new();
    for (index, name) in &image.names { names.append(*index, name); }
    let first_defined = plugin.func_imports() as u32;
    for (index, name) in plugin.names.range(first_defined..) { names.append(map.functions[*index as usize], name); }
    names.append(initialize, "__semio_split_initialize");
    let mut name_section = NameSection::new();
    name_section.functions(&names);
    put(&mut out, &name_section);
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all()).validate_all(&out).context("the merged module does not validate")?;
    let layout = Layout { data_start: if data_end > 0 { data_start } else { 0 }, data_end, reserve_end, table_base, table_end, got_data: addresses.len(), bound_functions, plugin_functions: plugin.functions.len() };
    Ok((out, layout))
}

/// 🧱️ Wraps a merged core module into a component the way `wasm-component-ld` does for `wasm32-wasip2` (no adapter, semver-merged imports).
pub fn componentize(core: &[u8]) -> Result<Vec<u8>> {
    wit_component::ComponentEncoder::default().merge_imports_based_on_semver(true).module(core)?.validate(true).encode()
}

/// ⌨️ Arguments of `link` and `image-exports`.
pub struct LinkCli {
    pub framework: Option<PathBuf>,
    pub plugin: PathBuf,
    pub out: PathBuf,
    pub core: Option<PathBuf>,
    pub receipt: Option<PathBuf>,
    pub imports: Option<PathBuf>,
}

/// 🏃️ `link`: merge, componentize, write the component, the optional core module and the receipt (`🧬️schema/🔣️.json#/definitions/SplitLinkReceipt`).
pub fn run_link(cli: &LinkCli) -> Result<()> {
    let framework_path = cli.framework.as_ref().ok_or_else(|| anyhow!("--framework <image.wasm> required"))?;
    let framework = std::fs::read(framework_path).with_context(|| format!("read {}", framework_path.display()))?;
    let plugin = std::fs::read(&cli.plugin).with_context(|| format!("read {}", cli.plugin.display()))?;
    let started = Instant::now();
    let (core, layout) = merge(&framework, &plugin)?;
    let merged_ms = started.elapsed().as_millis();
    let component = componentize(&core)?;
    let componentized_ms = started.elapsed().as_millis() - merged_ms;
    std::fs::write(&cli.out, &component).with_context(|| format!("write {}", cli.out.display()))?;
    if let Some(path) = &cli.core { std::fs::write(path, &core).with_context(|| format!("write {}", path.display()))?; }
    if let Some(path) = &cli.receipt {
        let receipt = serde_json::json!({
            "version": 1,
            "framework": { "path": framework_path, "bytes": framework.len() },
            "plugin": { "path": cli.plugin, "bytes": plugin.len(), "functions": layout.plugin_functions, "boundFunctions": layout.bound_functions },
            "layout": { "dataStart": layout.data_start, "dataEnd": layout.data_end, "reserveEnd": layout.reserve_end, "tableBase": layout.table_base, "tableEnd": layout.table_end, "gotData": layout.got_data },
            "component": { "path": cli.out, "bytes": component.len(), "coreBytes": core.len() },
            "durationsMs": { "merge": merged_ms, "componentize": componentized_ms },
        });
        std::fs::write(path, format!("{receipt:#}\n")).with_context(|| format!("write {}", path.display()))?;
    }
    Ok(())
}

/// 🏃️ `merge`: write only the validated merged core module, without componentizing it.
pub fn run_merge(cli: &LinkCli) -> Result<()> {
    let framework_path = cli.framework.as_ref().ok_or_else(|| anyhow!("--framework <image.wasm> required"))?;
    let framework = std::fs::read(framework_path).with_context(|| format!("read {}", framework_path.display()))?;
    let plugin = std::fs::read(&cli.plugin).with_context(|| format!("read {}", cli.plugin.display()))?;
    let (core, _) = merge(&framework, &plugin)?;
    std::fs::write(&cli.out, core).with_context(|| format!("write {}", cli.out.display()))
}

/// 🏃️ `image-exports`: write the `wasm-ld` response file that makes the framework image export every plugin import and, with `--imports`, the image root
/// declarations of the plugin's non-`env` imports.
pub fn run_image_exports(cli: &LinkCli) -> Result<()> {
    let plugin = std::fs::read(&cli.plugin).with_context(|| format!("read {}", cli.plugin.display()))?;
    let mut lines = image_exports(&plugin)?.join("\n");
    lines.push('\n');
    std::fs::write(&cli.out, lines).with_context(|| format!("write {}", cli.out.display()))?;
    if let Some(path) = &cli.imports { std::fs::write(path, image_imports_rust(&plugin)?).with_context(|| format!("write {}", path.display()))?; }
    Ok(())
}

/// ⌨️ Parses `link|merge --framework <image.wasm> --plugin <plugin.wasm> -o <component.wasm> [--core <core.wasm>] [--receipt <receipt.json>]` and
/// `image-exports --plugin <plugin.wasm> -o <exports.rsp> [--imports <imports.rs>]`.
pub fn parse_link(args: impl Iterator<Item = String>) -> Result<LinkCli> {
    let mut iter = args;
    let (mut framework, mut plugin, mut out, mut core, mut receipt, mut imports) = (None, None, None, None, None, None);
    while let Some(arg) = iter.next() {
        let mut value = || iter.next().map(PathBuf::from).ok_or_else(|| anyhow!("{arg} needs a value"));
        match arg.as_str() {
            "--framework" => framework = Some(value()?),
            "--plugin" => plugin = Some(value()?),
            "-o" | "--out" => out = Some(value()?),
            "--core" => core = Some(value()?),
            "--receipt" => receipt = Some(value()?),
            "--imports" => imports = Some(value()?),
            other => bail!("unsupported link option {other}"),
        }
    }
    Ok(LinkCli { framework, plugin: plugin.ok_or_else(|| anyhow!("--plugin <plugin.wasm> required"))?, out: out.ok_or_else(|| anyhow!("-o <path> required"))?, core, receipt, imports })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
