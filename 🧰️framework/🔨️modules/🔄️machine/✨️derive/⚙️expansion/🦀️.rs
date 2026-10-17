//! 🪄️ Proc macros compiling `statechart!` declarations into `machine` static tables.
//!
//! Entry points delegate to inner `proc_macro2`-typed functions so the
//! parse → analyze → codegen pipeline is unit-testable without `trybuild`.
//!
//! State/region/history names must be unique across the *whole* machine (no dotted
//! relative/absolute target paths) — a deliberate v1 simplification that keeps name
//! resolution a flat map instead of a scoped path resolver.

#[path = "🔬️analyze/🦀️.rs"]
mod analyze;

#[path = "🏗️codegen/🦀️.rs"]
mod codegen;

#[path = "✂️parse/🦀️.rs"]
mod parse;

use quote::quote;
use syn::DeriveInput;

//#region 🔖️Entry

/// 🪄️ Compiles a `machine <name> { .. }` declaration into a `pub mod <name> { .. }`
/// with a dense static [`machine::MachineDefinition`], a generated event enum, and a
/// `states` module of `NodeId` consts.
pub fn expand_statechart(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ast: parse::MachineAst = syn::parse2(input)?;
    let ir = analyze::analyze(ast)?;
    Ok(codegen::emit(&ir))
}

/// 🪄️ Implements `machine::StatechartEvent` for a consumer-authored enum, for use with
/// hand-authored `MachineDefinition` tables (i.e. without the `statechart!` DSL).
pub fn expand_statechart_event(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(e) => e,
        _ => return Err(syn::Error::new_spanned(input, "StatechartEvent can only be derived for enums")),
    };
    let count = data.variants.len() as u16;
    let mut id_arms = Vec::new();
    let mut name_arms = Vec::new();
    for (i, variant) in data.variants.iter().enumerate() {
        let lit = i as u16;
        let vname = &variant.ident;
        let pattern = match &variant.fields {
            syn::Fields::Named(_) => quote! { #name::#vname { .. } },
            syn::Fields::Unnamed(_) => quote! { #name::#vname(..) },
            syn::Fields::Unit => quote! { #name::#vname },
        };
        id_arms.push(quote! { #pattern => machine::EventId(#lit) });
        let vname_str = vname.to_string();
        name_arms.push(quote! { #lit => #vname_str });
    }
    Ok(quote! {
        impl machine::StatechartEvent for #name {
            const EVENT_COUNT: u16 = #count;

            fn event_id(&self) -> machine::EventId {
                match self {
                    #(#id_arms),*
                }
            }

            fn event_name(id: machine::EventId) -> &'static str {
                match id.0 {
                    #(#name_arms,)*
                    _ => "?",
                }
            }
        }
    })
}

/// 🪄️ Implements `machine::StatechartSchema` for a consumer-authored context struct —
/// embeds a field name/type manifest consumed by TypeScript generation tooling.
pub fn expand_statechart_schema(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return Err(syn::Error::new_spanned(input, "StatechartSchema can only be derived for structs")),
    };
    let mut fields_json = Vec::new();
    if let syn::Fields::Named(named) = &data.fields {
        for field in &named.named {
            let fname = field.ident.as_ref().expect("named field").to_string();
            let ty = &field.ty;
            let fty = quote! { #ty }.to_string().replace('"', "\\\"");
            fields_json.push(format!("{{\"name\":\"{fname}\",\"type\":\"{fty}\"}}"));
        }
    }
    let json = format!("{{\"fields\":[{}]}}", fields_json.join(","));
    Ok(quote! {
        impl machine::StatechartSchema for #name {
            const SCHEMA_JSON: &'static str = #json;
        }
    })
}

struct ExportWasmMachineAst {
    machine_path: syn::Path,
    js_name: syn::LitStr,
}

impl syn::parse::Parse for ExportWasmMachineAst {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let machine_path = input.parse::<syn::Path>()?;
        input.parse::<syn::Token![,]>()?;
        let js_name = input.parse::<syn::LitStr>()?;
        Ok(Self { machine_path, js_name })
    }
}

/// 🪄️ wasm-bindgen can't export generics, so a concrete consumer machine invokes this
/// in its own `🔖️WasmBridge` region to emit a `#[wasm_bindgen]`-exported class wrapping
/// one `<machine_path>` instance: `new/sendJson/tick/snapshotJson/restoreJson/manifestJson/onEffect`.
pub fn expand_export_wasm_machine(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ast: ExportWasmMachineAst = syn::parse2(input)?;
    let machine_path = &ast.machine_path;
    let class_name = syn::Ident::new(&ast.js_name.value(), ast.js_name.span());
    Ok(quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct #class_name {
            snapshot: std::cell::RefCell<machine::Snapshot<#machine_path>>,
            host: std::cell::RefCell<machine::WasmHost<#machine_path>>,
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #class_name {
            #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
            pub fn new(input_json: &str) -> Result<#class_name, wasm_bindgen::JsValue>
            where
                <#machine_path as machine::Machine>::Input: for<'de> serde::Deserialize<'de>,
            {
                let input: <#machine_path as machine::Machine>::Input =
                    serde_json::from_str(input_json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let mut sink: Vec<machine::Command<#machine_path>> = Vec::new();
                let snapshot = machine::init::<#machine_path>(input, &mut sink);
                let instance = Self {
                    snapshot: std::cell::RefCell::new(snapshot),
                    host: std::cell::RefCell::new(machine::WasmHost::new()),
                };
                for command in sink {
                    let mut snap = instance.snapshot.borrow_mut();
                    let mut host = instance.host.borrow_mut();
                    machine::route_command(&mut *host, &mut snap, machine::ActorId(0), command);
                }
                Ok(instance)
            }

            #[wasm_bindgen::prelude::wasm_bindgen(js_name = sendJson)]
            pub fn send_json(&self, event_json: &str) -> Result<(), wasm_bindgen::JsValue>
            where
                <#machine_path as machine::Machine>::Event: for<'de> serde::Deserialize<'de>,
                <#machine_path as machine::Machine>::Effect: serde::Serialize,
            {
                let event: <#machine_path as machine::Machine>::Event =
                    serde_json::from_str(event_json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let mut snapshot = self.snapshot.borrow_mut();
                let mut host = self.host.borrow_mut();
                let mut buffer: Vec<machine::Command<#machine_path>> = Vec::new();
                let mut inspector = machine::NullInspector;
                machine::macrostep(&mut snapshot, event, &mut buffer, &mut inspector);
                for command in buffer {
                    machine::route_command(&mut *host, &mut snapshot, machine::ActorId(0), command);
                }
                Ok(())
            }

            /// Fires any `after`-timers/invokes whose deadline has passed. Call this
            /// from a JS-side `setInterval`/`requestAnimationFrame` loop.
            #[wasm_bindgen::prelude::wasm_bindgen(js_name = tick)]
            pub fn tick(&self) {
                let due = self.host.borrow_mut().due_timers();
                for (_actor, timer) in due {
                    let mut snapshot = self.snapshot.borrow_mut();
                    let mut host = self.host.borrow_mut();
                    let mut buffer: Vec<machine::Command<#machine_path>> = Vec::new();
                    let mut inspector = machine::NullInspector;
                    machine::timer_elapsed(&mut snapshot, timer, &mut buffer, &mut inspector);
                    for command in buffer {
                        machine::route_command(&mut *host, &mut snapshot, machine::ActorId(0), command);
                    }
                }
            }

            #[wasm_bindgen::prelude::wasm_bindgen(js_name = snapshotJson)]
            pub fn snapshot_json(&self) -> Result<String, wasm_bindgen::JsValue>
            where
                <#machine_path as machine::Machine>::Context: serde::Serialize,
            {
                let snapshot = self.snapshot.borrow();
                let persisted = machine::persist(&*snapshot);
                let persisted_json = serde_json::to_string(&persisted).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let context_json = serde_json::to_string(&snapshot.context).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                Ok(format!("{{\"persisted\":{persisted_json},\"context\":{context_json}}}"))
            }

            #[wasm_bindgen::prelude::wasm_bindgen(js_name = restoreJson)]
            pub fn restore_json(snapshot_json: &str) -> Result<#class_name, wasm_bindgen::JsValue>
            where
                <#machine_path as machine::Machine>::Context: for<'de> serde::Deserialize<'de>,
            {
                let value: serde_json::Value =
                    serde_json::from_str(snapshot_json).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let persisted: machine::PersistedSnapshot =
                    serde_json::from_value(value["persisted"].clone()).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let context: <#machine_path as machine::Machine>::Context =
                    serde_json::from_value(value["context"].clone()).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
                let snapshot = machine::restore::<#machine_path, machine::NoMigrations>(&persisted, context, &[])
                    .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("{:?}", e)))?;
                Ok(Self {
                    snapshot: std::cell::RefCell::new(snapshot),
                    host: std::cell::RefCell::new(machine::WasmHost::new()),
                })
            }

            /// Static — the manifest doesn't depend on any instance, so tooling can
            /// read it (for TypeScript generation) without constructing a machine.
            #[wasm_bindgen::prelude::wasm_bindgen(js_name = manifestJson)]
            pub fn manifest_json() -> String {
                <#machine_path as machine::Machine>::definition().manifest_json.to_string()
            }

            #[wasm_bindgen::prelude::wasm_bindgen(js_name = onEffect)]
            pub fn on_effect(&self, callback: js_sys::Function) {
                self.host.borrow_mut().set_effect_callback(callback);
            }
        }
    })
}

//#endregion 🔖️Entry

//#region 🧪️Tests

#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
