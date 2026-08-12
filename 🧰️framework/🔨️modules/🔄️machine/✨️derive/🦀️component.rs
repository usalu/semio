//! 🪄️ Proc macros compiling `statechart!` declarations into `machine` static tables.
//!
//! Entry points delegate to inner `proc_macro2`-typed functions so the
//! parse → analyze → codegen pipeline is unit-testable without `trybuild`.
//!
//! State/region/history names must be unique across the *whole* machine (no dotted
//! relative/absolute target paths) — a deliberate v1 simplification that keeps name
//! resolution a flat map instead of a scoped path resolver.

mod analyze {
    //! 🔬️ Syntax AST → semantic IR — name resolution, dense id assignment, validation.

    use super::parse::{AlwaysAst, EventVariantAst, FinalAst, MachineAst, OnAst, ParallelAst, StateAst, StateItemAst};
    use proc_macro2::Span;
    use std::collections::HashMap;
    use syn::{Ident, Type};

    //#region 🔖️Ir

    pub enum NodeKindIr {
        Atomic,
        Compound,
        Parallel,
        Final,
        HistoryShallow,
        HistoryDeep,
    }

    pub struct NodeIr {
        pub name: String,
        pub kind: NodeKindIr,
        pub parent: Option<usize>,
        pub initial: Option<usize>,
        pub children: Vec<usize>,
        pub entry_actions: Vec<usize>,
        pub exit_actions: Vec<usize>,
        pub invokes: Vec<usize>,
        pub timers: Vec<(usize, u64)>,
    }

    pub enum TriggerIr {
        Event(usize),
        Eventless,
        Done(usize),
        Timer(usize),
    }

    pub struct TransitionIr {
        pub source: usize,
        pub trigger: TriggerIr,
        pub guard: Option<usize>,
        pub targets: Vec<usize>,
        pub internal: bool,
        pub actions: Vec<usize>,
    }

    pub struct Ir {
        pub machine_name: Ident,
        pub nodes: Vec<NodeIr>,
        pub transitions: Vec<TransitionIr>,
        pub event_name: Ident,
        pub event_variants: Vec<EventVariantAst>,
        pub guards: Vec<Ident>,
        pub actions: Vec<Ident>,
        pub context_ty: Type,
        pub input_ty: Type,
        pub output_ty: Type,
        pub effect_ty: Type,
        pub context_from_input: Ident,
        pub make_output: Option<Ident>,
        pub fingerprint: u64,
        pub manifest_json: String,
    }

    //#endregion 🔖️Ir

    //#region 🔖️Analyzer

    struct PendingTransition {
        source: usize,
        trigger: PendingTrigger,
        guard: Option<usize>,
        target_name: String,
        target_span: Span,
        internal: bool,
        action: Option<usize>,
    }

    enum PendingTrigger {
        Event(Ident),
        Eventless,
        Done,
        Timer(usize),
    }

    struct Analyzer {
        nodes: Vec<NodeIr>,
        names: HashMap<String, usize>,
        guards: Vec<Ident>,
        actions: Vec<Ident>,
        outputs: Vec<Ident>,
        pending: Vec<PendingTransition>,
        invoke_counter: usize,
        timer_counter: usize,
    }

    impl Analyzer {
        fn intern_guard(&mut self, ident: Ident) -> usize {
            if let Some(pos) = self.guards.iter().position(|g| g == &ident) {
                pos
            } else {
                self.guards.push(ident);
                self.guards.len() - 1
            }
        }

        fn intern_action(&mut self, ident: Ident) -> usize {
            if let Some(pos) = self.actions.iter().position(|a| a == &ident) {
                pos
            } else {
                self.actions.push(ident);
                self.actions.len() - 1
            }
        }

        fn add_node(&mut self, name: &Ident, kind: NodeKindIr, parent: usize) -> syn::Result<usize> {
            let key = name.to_string();
            if self.names.contains_key(&key) {
                return Err(syn::Error::new(name.span(), format!("duplicate state name `{key}`")));
            }
            let idx = self.nodes.len();
            self.nodes.push(NodeIr { name: key.clone(), kind, parent: Some(parent), initial: None, children: Vec::new(), entry_actions: Vec::new(), exit_actions: Vec::new(), invokes: Vec::new(), timers: Vec::new() });
            self.names.insert(key, idx);
            self.nodes[parent].children.push(idx);
            Ok(idx)
        }

        fn build_state(&mut self, ast: StateAst, parent: usize) -> syn::Result<usize> {
            let name = ast.name.clone().expect("nested/top-level states always carry a name");
            let kind = if ast.children.is_empty() { NodeKindIr::Atomic } else { NodeKindIr::Compound };
            let idx = self.add_node(&name, kind, parent)?;

            for entry in &ast.entry {
                let action_idx = self.intern_action(entry.clone());
                self.nodes[idx].entry_actions.push(action_idx);
            }
            for exit in &ast.exit {
                let action_idx = self.intern_action(exit.clone());
                self.nodes[idx].exit_actions.push(action_idx);
            }
            for _invoke in &ast.invokes {
                let invoke_id = self.invoke_counter;
                self.invoke_counter += 1;
                self.nodes[idx].invokes.push(invoke_id);
            }
            for after in &ast.afters {
                let timer_id = self.timer_counter;
                self.timer_counter += 1;
                self.nodes[idx].timers.push((timer_id, after.delay_ms));
                let action = after.action.clone().map(|a| self.intern_action(a));
                self.pending.push(PendingTransition { source: idx, trigger: PendingTrigger::Timer(timer_id), guard: None, target_name: after.target.to_string(), target_span: after.target.span(), internal: false, action });
            }
            self.push_on_transitions(idx, &ast.transitions);
            self.push_always_transitions(idx, &ast.always);

            let history_idx = if let Some(h) = &ast.history {
                let kind = if h.deep { NodeKindIr::HistoryDeep } else { NodeKindIr::HistoryShallow };
                Some(self.add_node(&h.name, kind, idx)?)
            } else {
                None
            };

            for child in ast.children {
                match child {
                    StateItemAst::State(s) => {
                        self.build_state(s, idx)?;
                    }
                    StateItemAst::Parallel(p) => {
                        self.build_parallel(p, idx)?;
                    }
                    StateItemAst::Final(f) => {
                        self.build_final(f, idx)?;
                    }
                }
            }

            if matches!(self.nodes[idx].kind, NodeKindIr::Compound) {
                let initial_name = ast.initial.clone().ok_or_else(|| syn::Error::new(name.span(), format!("compound state `{name}` needs `initial: <child>;`")))?;
                let initial_idx = *self.names.get(&initial_name.to_string()).ok_or_else(|| syn::Error::new(initial_name.span(), format!("unknown initial state `{initial_name}`")))?;
                self.nodes[idx].initial = Some(initial_idx);
            }
            if let Some(h_idx) = history_idx {
                self.nodes[h_idx].initial = self.nodes[idx].initial;
            }

            Ok(idx)
        }

        fn build_parallel(&mut self, ast: ParallelAst, parent: usize) -> syn::Result<usize> {
            if ast.regions.len() < 2 {
                return Err(syn::Error::new(ast.name.span(), format!("parallel state `{}` needs at least two `state` regions", ast.name)));
            }
            let idx = self.add_node(&ast.name, NodeKindIr::Parallel, parent)?;
            for region in ast.regions {
                self.build_state(region, idx)?;
            }
            if let Some((target, action)) = ast.on_done {
                let action_idx = action.map(|a| self.intern_action(a));
                self.pending.push(PendingTransition { source: idx, trigger: PendingTrigger::Done, guard: None, target_name: target.to_string(), target_span: target.span(), internal: false, action: action_idx });
            }
            Ok(idx)
        }

        fn build_final(&mut self, ast: FinalAst, parent: usize) -> syn::Result<usize> {
            let idx = self.add_node(&ast.name, NodeKindIr::Final, parent)?;
            if let Some(output_fn) = ast.output {
                self.outputs.push(output_fn);
            }
            Ok(idx)
        }

        fn push_on_transitions(&mut self, source: usize, transitions: &[OnAst]) {
            for on in transitions {
                let guard = on.guard.clone().map(|g| self.intern_guard(g));
                let action = on.action.clone().map(|a| self.intern_action(a));
                self.pending.push(PendingTransition { source, trigger: PendingTrigger::Event(on.event.clone()), guard, target_name: on.target.to_string(), target_span: on.target.span(), internal: on.internal, action });
            }
        }

        fn push_always_transitions(&mut self, source: usize, always: &[AlwaysAst]) {
            for al in always {
                let guard = al.guard.clone().map(|g| self.intern_guard(g));
                let action = al.action.clone().map(|a| self.intern_action(a));
                self.pending.push(PendingTransition { source, trigger: PendingTrigger::Eventless, guard, target_name: al.target.to_string(), target_span: al.target.span(), internal: false, action });
            }
        }
    }

    //#endregion 🔖️Analyzer

    //#region 🔖️Fingerprint

    fn kind_tag(kind: &NodeKindIr) -> u8 {
        match kind {
            NodeKindIr::Atomic => 0,
            NodeKindIr::Compound => 1,
            NodeKindIr::Parallel => 2,
            NodeKindIr::Final => 3,
            NodeKindIr::HistoryShallow => 4,
            NodeKindIr::HistoryDeep => 5,
        }
    }

    fn fnv_feed(state: &mut u64, bytes: &[u8]) {
        for &b in bytes {
            *state ^= b as u64;
            *state = state.wrapping_mul(0x100000001b3);
        }
    }

    /// 🔢️ Hand-rolled FNV-1a over the compiled structure — no hashing crate dependency.
    fn compute_fingerprint(nodes: &[NodeIr], transitions: &[TransitionIr], events: &[EventVariantAst]) -> u64 {
        let mut state: u64 = 0xcbf29ce484222325;
        for node in nodes {
            fnv_feed(&mut state, node.name.as_bytes());
            fnv_feed(&mut state, &[kind_tag(&node.kind)]);
            fnv_feed(&mut state, &(node.parent.map_or(u64::MAX, |p| p as u64)).to_le_bytes());
        }
        for t in transitions {
            fnv_feed(&mut state, &(t.source as u64).to_le_bytes());
            for target in &t.targets {
                fnv_feed(&mut state, &(*target as u64).to_le_bytes());
            }
        }
        for v in events {
            fnv_feed(&mut state, v.name().to_string().as_bytes());
        }
        state
    }

    fn build_manifest_json(name: &Ident, nodes: &[NodeIr], transitions: &[TransitionIr], events: &[EventVariantAst]) -> String {
        let mut json = format!("{{\"id\":\"{name}\",\"states\":[");
        for (i, node) in nodes.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            let parent = node.parent.map_or_else(|| "null".to_string(), |p| p.to_string());
            json.push_str(&format!("{{\"id\":\"{}\",\"parent\":{}}}", node.name, parent));
        }
        json.push_str("],\"events\":[");
        for (i, event) in events.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!("\"{}\"", event.name()));
        }
        json.push_str("],\"transitionCount\":");
        json.push_str(&transitions.len().to_string());
        json.push('}');
        json
    }

    //#endregion 🔖️Fingerprint

pub fn analyze(ast: MachineAst) -> syn::Result<Ir> {
        let mut az = Analyzer { nodes: Vec::new(), names: HashMap::new(), guards: Vec::new(), actions: Vec::new(), outputs: Vec::new(), pending: Vec::new(), invoke_counter: 0, timer_counter: 0 };
        az.nodes.push(NodeIr { name: "root".to_string(), kind: NodeKindIr::Compound, parent: None, initial: None, children: Vec::new(), entry_actions: Vec::new(), exit_actions: Vec::new(), invokes: Vec::new(), timers: Vec::new() });
        az.names.insert("root".to_string(), 0);

        for child in ast.children {
            match child {
                StateItemAst::State(s) => {
                    az.build_state(s, 0)?;
                }
                StateItemAst::Parallel(p) => {
                    az.build_parallel(p, 0)?;
                }
                StateItemAst::Final(f) => {
                    az.build_final(f, 0)?;
                }
            }
        }

        let initial_idx = *az.names.get(&ast.initial.to_string()).ok_or_else(|| syn::Error::new(ast.initial.span(), format!("unknown initial state `{}`", ast.initial)))?;
        az.nodes[0].initial = Some(initial_idx);

        let mut transitions = Vec::new();
        for p in az.pending {
            let target_idx = *az.names.get(&p.target_name).ok_or_else(|| syn::Error::new(p.target_span, format!("unknown transition target `{}`", p.target_name)))?;
            let trigger = match p.trigger {
                PendingTrigger::Event(ident) => {
                    let event_idx = ast.event_variants.iter().position(|v| v.name() == &ident).ok_or_else(|| syn::Error::new(ident.span(), format!("unknown event variant `{ident}`")))?;
                    TriggerIr::Event(event_idx)
                }
                PendingTrigger::Eventless => TriggerIr::Eventless,
                PendingTrigger::Done => TriggerIr::Done(p.source),
                PendingTrigger::Timer(id) => TriggerIr::Timer(id),
            };
            transitions.push(TransitionIr { source: p.source, trigger, guard: p.guard, targets: vec![target_idx], internal: p.internal, actions: p.action.into_iter().collect() });
        }

        let make_output = match (ast.output_from_context, az.outputs.len()) {
            (Some(ident), 0) => Some(ident),
            (None, 0) => None,
            (None, 1) => Some(az.outputs[0].clone()),
            (None, _) => return Err(syn::Error::new(ast.name.span(), "at most one `final` state may declare `output:`; use machine-level `output_from_context:` for more complex cases")),
            (Some(_), _) => return Err(syn::Error::new(ast.name.span(), "declare output via either machine-level `output_from_context:` or a single `final { output: .. }`, not both")),
        };

        let fingerprint = compute_fingerprint(&az.nodes, &transitions, &ast.event_variants);
        let manifest_json = build_manifest_json(&ast.name, &az.nodes, &transitions, &ast.event_variants);

        Ok(Ir {
            machine_name: ast.name,
            nodes: az.nodes,
            transitions,
            event_name: ast.event_name,
            event_variants: ast.event_variants,
            guards: az.guards,
            actions: az.actions,
            context_ty: ast.context_ty,
            input_ty: ast.input_ty,
            output_ty: ast.output_ty,
            effect_ty: ast.effect_ty,
            context_from_input: ast.context_from_input,
            make_output,
            fingerprint,
            manifest_json,
        })
    }

    //#endregion 🔖️Entry
}
mod codegen {
    //! 🏗️ Semantic IR → Rust code — dense static tables, a `Machine` impl, and a `states` module.

    use super::analyze::{Ir, NodeIr, NodeKindIr, TransitionIr, TriggerIr};
    use super::parse::EventVariantAst;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    //#region 🔖️Helpers

    fn to_pascal_case(input: &str) -> String {
        input
            .split('_')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect()
    }

    fn node_id_lit(index: usize) -> TokenStream {
        let lit = index as u16;
        quote! { machine::NodeId(#lit) }
    }

    fn node_kind_tokens(kind: &NodeKindIr) -> TokenStream {
        match kind {
            NodeKindIr::Atomic => quote! { machine::NodeKind::Atomic },
            NodeKindIr::Compound => quote! { machine::NodeKind::Compound },
            NodeKindIr::Parallel => quote! { machine::NodeKind::Parallel },
            NodeKindIr::Final => quote! { machine::NodeKind::Final },
            NodeKindIr::HistoryShallow => quote! { machine::NodeKind::HistoryShallow },
            NodeKindIr::HistoryDeep => quote! { machine::NodeKind::HistoryDeep },
        }
    }

    fn trigger_tokens(trigger: &TriggerIr) -> TokenStream {
        match trigger {
            TriggerIr::Event(idx) => {
                let lit = *idx as u16;
                quote! { machine::Trigger::Event(machine::EventId(#lit)) }
            }
            TriggerIr::Eventless => quote! { machine::Trigger::Eventless },
            TriggerIr::Done(node_idx) => {
                let target = node_id_lit(*node_idx);
                quote! { machine::Trigger::Done(#target) }
            }
            TriggerIr::Timer(id) => {
                let lit = *id as u16;
                quote! { machine::Trigger::Timer(machine::TimerId(#lit)) }
            }
        }
    }

    //#endregion 🔖️Helpers

    //#region 🔖️NodeDef

    fn node_def_tokens(node: &NodeIr, doc_index: usize) -> TokenStream {
        let stable_id = &node.name;
        let kind = node_kind_tokens(&node.kind);
        let parent = match node.parent {
            Some(p) => {
                let id = node_id_lit(p);
                quote! { Some(#id) }
            }
            None => quote! { None },
        };
        let initial = match node.initial {
            Some(i) => {
                let id = node_id_lit(i);
                quote! { Some(#id) }
            }
            None => quote! { None },
        };
        let children = node.children.iter().map(|c| node_id_lit(*c));
        let entry_actions = node.entry_actions.iter().map(|a| {
            let lit = *a as u16;
            quote! { machine::ActionId(#lit) }
        });
        let exit_actions = node.exit_actions.iter().map(|a| {
            let lit = *a as u16;
            quote! { machine::ActionId(#lit) }
        });
        let invokes = node.invokes.iter().map(|i| {
            let lit = *i as u16;
            quote! { machine::InvokeId(#lit) }
        });
        let timers = node.timers.iter().map(|(id, ms)| {
            let idl = *id as u16;
            let msl = *ms;
            quote! { (machine::TimerId(#idl), #msl) }
        });
        let doc_index = doc_index as u16;
        quote! {
            machine::NodeDef {
                stable_id: #stable_id,
                kind: #kind,
                parent: #parent,
                initial: #initial,
                children: &[ #(#children),* ],
                entry_actions: &[ #(#entry_actions),* ],
                exit_actions: &[ #(#exit_actions),* ],
                invokes: &[ #(#invokes),* ],
                timers: &[ #(#timers),* ],
                doc_index: #doc_index,
            }
        }
    }

    //#endregion 🔖️NodeDef

    //#region 🔖️TransitionDef

    fn transition_def_tokens(transition: &TransitionIr, doc_index: usize) -> TokenStream {
        let source = node_id_lit(transition.source);
        let trigger = trigger_tokens(&transition.trigger);
        let guard = match transition.guard {
            Some(g) => {
                let lit = g as u16;
                quote! { Some(machine::GuardId(#lit)) }
            }
            None => quote! { None },
        };
        let targets = transition.targets.iter().map(|t| node_id_lit(*t));
        let kind = if transition.internal {
            quote! { machine::TransitionKind::Internal }
        } else {
            quote! { machine::TransitionKind::External }
        };
        let actions = transition.actions.iter().map(|a| {
            let lit = *a as u16;
            quote! { machine::ActionId(#lit) }
        });
        let doc_index = doc_index as u16;
        quote! {
            machine::TransitionDef {
                source: #source,
                trigger: #trigger,
                guard: #guard,
                targets: &[ #(#targets),* ],
                kind: #kind,
                actions: &[ #(#actions),* ],
                doc_index: #doc_index,
            }
        }
    }

    //#endregion 🔖️TransitionDef

    //#region 🔖️Event

    fn event_variant_def_tokens(variant: &EventVariantAst) -> TokenStream {
        match variant {
            EventVariantAst::Unit(name) => quote! { #name },
            EventVariantAst::Struct(name, fields) => {
                let field_defs = fields.iter().map(|(fname, fty)| quote! { #fname: #fty });
                quote! { #name { #(#field_defs),* } }
            }
            EventVariantAst::Tuple(name, types) => quote! { #name(#(#types),*) },
        }
    }

    fn event_id_arm_tokens(event_name: &syn::Ident, index: usize, variant: &EventVariantAst) -> TokenStream {
        let lit = index as u16;
        match variant {
            EventVariantAst::Unit(name) => quote! { #event_name::#name => machine::EventId(#lit) },
            EventVariantAst::Struct(name, _) => quote! { #event_name::#name { .. } => machine::EventId(#lit) },
            EventVariantAst::Tuple(name, types) => {
                let underscores = types.iter().map(|_| quote! { _ });
                quote! { #event_name::#name(#(#underscores),*) => machine::EventId(#lit) }
            }
        }
    }

    //#endregion 🔖️Event

    //#region 🔖️Emit

    pub fn emit(ir: &Ir) -> TokenStream {
        let mod_name = &ir.machine_name;
        let marker_name = format_ident!("{}", to_pascal_case(&ir.machine_name.to_string()));
        let event_name = &ir.event_name;
        let context_ty = &ir.context_ty;
        let input_ty = &ir.input_ty;
        let output_ty = &ir.output_ty;
        let effect_ty = &ir.effect_ty;
        let context_from_input = &ir.context_from_input;
        let machine_name_str = ir.machine_name.to_string();
        let manifest_json = &ir.manifest_json;
        let fingerprint = ir.fingerprint;

        let word_count = ir.nodes.len().div_ceil(64).max(1);

        let event_variant_defs = ir.event_variants.iter().map(event_variant_def_tokens);
        let event_id_arms = ir.event_variants.iter().enumerate().map(|(i, v)| event_id_arm_tokens(event_name, i, v));
        let event_name_arms = ir.event_variants.iter().enumerate().map(|(i, v)| {
            let idx = i as u16;
            let name_str = v.name().to_string();
            quote! { #idx => #name_str }
        });
        let event_count = ir.event_variants.len() as u16;

        let node_defs = ir.nodes.iter().enumerate().map(|(i, n)| node_def_tokens(n, i));
        let transition_defs = ir.transitions.iter().enumerate().map(|(i, t)| transition_def_tokens(t, i));

        let guard_idents = &ir.guards;
        let action_idents = &ir.actions;

        let make_output = match &ir.make_output {
            Some(ident) => quote! { Some(super::#ident) },
            None => quote! { None },
        };

        let state_consts = ir.nodes.iter().enumerate().map(|(i, n)| {
            let const_name = format_ident!("{}", n.name.to_uppercase());
            let id = node_id_lit(i);
            quote! { pub const #const_name: machine::NodeId = #id; }
        });

        quote! {
            pub mod #mod_name {
                use super::*;

                //#region 🔖️Event
                #[derive(Clone, Debug)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub enum #event_name {
                    #(#event_variant_defs),*
                }

                impl machine::StatechartEvent for #event_name {
                    const EVENT_COUNT: u16 = #event_count;

                    fn event_id(&self) -> machine::EventId {
                        match self {
                            #(#event_id_arms),*
                        }
                    }

                    fn event_name(id: machine::EventId) -> &'static str {
                        match id.0 {
                            #(#event_name_arms,)*
                            _ => "?",
                        }
                    }
                }
                //#endregion 🔖️Event

                //#region 🔖️Machine
                pub struct #marker_name;

                impl machine::Machine for #marker_name {
                    type Context = #context_ty;
                    type Event = #event_name;
                    type Input = #input_ty;
                    type Output = #output_ty;
                    type Effect = #effect_ty;
                    type Config = machine::BitSet<#word_count>;

                    fn definition() -> &'static machine::MachineDefinition<Self> {
                        static DEF: machine::MachineDefinition<#marker_name> = machine::MachineDefinition {
                            id: #machine_name_str,
                            nodes: &[ #(#node_defs),* ],
                            transitions: &[ #(#transition_defs),* ],
                            context_from_input: super::#context_from_input,
                            make_output: #make_output,
                            guards: &[ #(super::#guard_idents),* ],
                            actions: &[ #(super::#action_idents),* ],
                            fingerprint: #fingerprint,
                            manifest_json: #manifest_json,
                        };
                        &DEF
                    }
                }
                //#endregion 🔖️Machine

                //#region 🔖️States
                pub mod states {
                    use super::super::*;
                    #(#state_consts)*
                }
                //#endregion 🔖️States

                pub type Snapshot = machine::Snapshot<#marker_name>;
                pub const MANIFEST_JSON: &str = #manifest_json;
            }
        }
    }

    //#endregion 🔖️Emit
}
mod parse {
    //! ✂️ Token stream → syntax AST for the `statechart!` DSL. Spanned errors only.

    use syn::parse::{Parse, ParseStream};
    use syn::{braced, Ident, LitInt, Token, Type};

    //#region 🔖️Keywords

    mod kw {
        syn::custom_keyword!(machine);
        syn::custom_keyword!(context);
        syn::custom_keyword!(event);
        syn::custom_keyword!(input);
        syn::custom_keyword!(output);
        syn::custom_keyword!(effect);
        syn::custom_keyword!(initial);
        syn::custom_keyword!(state);
        syn::custom_keyword!(parallel);
        syn::custom_keyword!(history);
        syn::custom_keyword!(shallow);
        syn::custom_keyword!(deep);
        syn::custom_keyword!(on);
        syn::custom_keyword!(on_done);
        syn::custom_keyword!(after);
        syn::custom_keyword!(always);
        syn::custom_keyword!(invoke);
        syn::custom_keyword!(entry);
        syn::custom_keyword!(exit);
        syn::custom_keyword!(internal);
        syn::custom_keyword!(context_from_input);
        syn::custom_keyword!(output_from_context);
    }

    //#endregion 🔖️Keywords

    //#region 🔖️Ast

    pub struct MachineAst {
        pub name: Ident,
        pub context_ty: Type,
        pub event_name: Ident,
        pub event_variants: Vec<EventVariantAst>,
        pub input_ty: Type,
        pub output_ty: Type,
        pub effect_ty: Type,
        pub context_from_input: Ident,
        pub output_from_context: Option<Ident>,
        pub initial: Ident,
        pub children: Vec<StateItemAst>,
    }

    pub enum EventVariantAst {
        Unit(Ident),
        Struct(Ident, Vec<(Ident, Type)>),
        Tuple(Ident, Vec<Type>),
    }

    impl EventVariantAst {
        pub fn name(&self) -> &Ident {
            match self {
                EventVariantAst::Unit(name) | EventVariantAst::Struct(name, _) | EventVariantAst::Tuple(name, _) => name,
            }
        }
    }

    pub enum StateItemAst {
        State(StateAst),
        Parallel(ParallelAst),
        Final(FinalAst),
    }

    #[derive(Default)]
    pub struct StateAst {
        pub name: Option<Ident>,
        pub initial: Option<Ident>,
        pub entry: Vec<Ident>,
        pub exit: Vec<Ident>,
        pub invokes: Vec<Ident>,
        pub afters: Vec<AfterAst>,
        pub transitions: Vec<OnAst>,
        pub always: Vec<AlwaysAst>,
        pub history: Option<HistoryAst>,
        pub children: Vec<StateItemAst>,
    }

    pub struct FinalAst {
        pub name: Ident,
        pub output: Option<Ident>,
    }

    pub struct ParallelAst {
        pub name: Ident,
        pub regions: Vec<StateAst>,
        pub on_done: Option<(Ident, Option<Ident>)>,
    }

    pub struct AfterAst {
        pub delay_ms: u64,
        pub target: Ident,
        pub action: Option<Ident>,
    }

    pub struct OnAst {
        pub event: Ident,
        pub guard: Option<Ident>,
        pub target: Ident,
        pub action: Option<Ident>,
        pub internal: bool,
    }

    pub struct AlwaysAst {
        pub guard: Option<Ident>,
        pub target: Ident,
        pub action: Option<Ident>,
    }

    pub struct HistoryAst {
        pub name: Ident,
        pub deep: bool,
    }

    //#endregion 🔖️Ast

    //#region 🔖️Parse

    fn parse_typed_item(input: ParseStream<'_>) -> syn::Result<Type> {
        input.parse::<Token![:]>()?;
        let ty = input.parse::<Type>()?;
        input.parse::<Token![;]>()?;
        Ok(ty)
    }

    fn parse_ident_item(input: ParseStream<'_>) -> syn::Result<Ident> {
        input.parse::<Token![:]>()?;
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![;]>()?;
        Ok(ident)
    }

    fn parse_event_block(input: ParseStream<'_>) -> syn::Result<(Ident, Vec<EventVariantAst>)> {
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut variants = Vec::new();
        while !content.is_empty() {
            let variant_name = content.parse::<Ident>()?;
            if content.peek(syn::token::Brace) {
                let fields_content;
                syn::braced!(fields_content in content);
                let mut fields = Vec::new();
                while !fields_content.is_empty() {
                    let field_name = fields_content.parse::<Ident>()?;
                    fields_content.parse::<Token![:]>()?;
                    let field_ty = fields_content.parse::<Type>()?;
                    fields.push((field_name, field_ty));
                    if fields_content.peek(Token![,]) {
                        fields_content.parse::<Token![,]>()?;
                    }
                }
                variants.push(EventVariantAst::Struct(variant_name, fields));
            } else if content.peek(syn::token::Paren) {
                let tuple_content;
                syn::parenthesized!(tuple_content in content);
                let mut types = Vec::new();
                while !tuple_content.is_empty() {
                    types.push(tuple_content.parse::<Type>()?);
                    if tuple_content.peek(Token![,]) {
                        tuple_content.parse::<Token![,]>()?;
                    }
                }
                variants.push(EventVariantAst::Tuple(variant_name, types));
            } else {
                variants.push(EventVariantAst::Unit(variant_name));
            }
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        Ok((name, variants))
    }

    fn parse_after(input: ParseStream<'_>) -> syn::Result<AfterAst> {
        let delay_lit = input.parse::<LitInt>()?;
        let delay_ms: u64 = delay_lit.base10_parse()?;
        input.parse::<Token![=>]>()?;
        let target = input.parse::<Ident>()?;
        let action = if input.peek(Token![do]) {
            input.parse::<Token![do]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        input.parse::<Token![;]>()?;
        Ok(AfterAst { delay_ms, target, action })
    }

    fn parse_on(input: ParseStream<'_>) -> syn::Result<OnAst> {
        let event = input.parse::<Ident>()?;
        let guard = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        input.parse::<Token![=>]>()?;
        let target = input.parse::<Ident>()?;
        let action = if input.peek(Token![do]) {
            input.parse::<Token![do]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        let internal = if input.peek(kw::internal) {
            input.parse::<kw::internal>()?;
            true
        } else {
            false
        };
        input.parse::<Token![;]>()?;
        Ok(OnAst { event, guard, target, action, internal })
    }

    fn parse_always(input: ParseStream<'_>) -> syn::Result<AlwaysAst> {
        let guard = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        input.parse::<Token![=>]>()?;
        let target = input.parse::<Ident>()?;
        let action = if input.peek(Token![do]) {
            input.parse::<Token![do]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        input.parse::<Token![;]>()?;
        Ok(AlwaysAst { guard, target, action })
    }

    fn parse_history(input: ParseStream<'_>) -> syn::Result<HistoryAst> {
        let name = input.parse::<Ident>()?;
        let deep = if input.peek(kw::deep) {
            input.parse::<kw::deep>()?;
            true
        } else {
            input.parse::<kw::shallow>()?;
            false
        };
        input.parse::<Token![;]>()?;
        Ok(HistoryAst { name, deep })
    }

    fn parse_state_body(input: ParseStream<'_>, name: Option<Ident>) -> syn::Result<StateAst> {
        let content;
        braced!(content in input);
        let mut state = StateAst { name, ..StateAst::default() };
        while !content.is_empty() {
            if content.peek(kw::initial) {
                content.parse::<kw::initial>()?;
                state.initial = Some(parse_ident_item(&content)?);
            } else if content.peek(kw::entry) {
                content.parse::<kw::entry>()?;
                state.entry.push(content.parse::<Ident>()?);
                content.parse::<Token![;]>()?;
            } else if content.peek(kw::exit) {
                content.parse::<kw::exit>()?;
                state.exit.push(content.parse::<Ident>()?);
                content.parse::<Token![;]>()?;
            } else if content.peek(kw::invoke) {
                content.parse::<kw::invoke>()?;
                state.invokes.push(content.parse::<Ident>()?);
                content.parse::<Token![;]>()?;
            } else if content.peek(kw::after) {
                content.parse::<kw::after>()?;
                state.afters.push(parse_after(&content)?);
            } else if content.peek(kw::on) {
                content.parse::<kw::on>()?;
                state.transitions.push(parse_on(&content)?);
            } else if content.peek(kw::always) {
                content.parse::<kw::always>()?;
                state.always.push(parse_always(&content)?);
            } else if content.peek(kw::history) {
                content.parse::<kw::history>()?;
                state.history = Some(parse_history(&content)?);
            } else if content.peek(kw::state) {
                content.parse::<kw::state>()?;
                let child_name = content.parse::<Ident>()?;
                state.children.push(StateItemAst::State(parse_state_body(&content, Some(child_name))?));
            } else if content.peek(kw::parallel) {
                content.parse::<kw::parallel>()?;
                state.children.push(StateItemAst::Parallel(parse_parallel(&content)?));
            } else if content.peek(Token![final]) {
                content.parse::<Token![final]>()?;
                state.children.push(StateItemAst::Final(parse_final(&content)?));
            } else {
                return Err(content.error("expected initial/entry/exit/invoke/after/on/always/history/state/parallel/final"));
            }
        }
        Ok(state)
    }

    fn parse_final(input: ParseStream<'_>) -> syn::Result<FinalAst> {
        let name = input.parse::<Ident>()?;
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            return Ok(FinalAst { name, output: None });
        }
        let content;
        braced!(content in input);
        let mut output = None;
        while !content.is_empty() {
            if content.peek(kw::output) {
                content.parse::<kw::output>()?;
                output = Some(parse_ident_item(&content)?);
            } else {
                return Err(content.error("expected `output` inside a `final` block"));
            }
        }
        Ok(FinalAst { name, output })
    }

    fn parse_parallel(input: ParseStream<'_>) -> syn::Result<ParallelAst> {
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut regions = Vec::new();
        let mut on_done = None;
        while !content.is_empty() {
            if content.peek(kw::state) {
                content.parse::<kw::state>()?;
                let region_name = content.parse::<Ident>()?;
                regions.push(parse_state_body(&content, Some(region_name))?);
            } else if content.peek(kw::on_done) {
                content.parse::<kw::on_done>()?;
                content.parse::<Token![=>]>()?;
                let target = content.parse::<Ident>()?;
                let action = if content.peek(Token![do]) {
                    content.parse::<Token![do]>()?;
                    Some(content.parse::<Ident>()?)
                } else {
                    None
                };
                content.parse::<Token![;]>()?;
                on_done = Some((target, action));
            } else {
                return Err(content.error("expected `state` region or `on_done` inside a `parallel` block"));
            }
        }
        Ok(ParallelAst { name, regions, on_done })
    }

    impl Parse for MachineAst {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            input.parse::<kw::machine>()?;
            let name = input.parse::<Ident>()?;
            let content;
            braced!(content in input);

            let mut context_ty = None;
            let mut event_decl = None;
            let mut input_ty = None;
            let mut output_ty = None;
            let mut effect_ty = None;
            let mut context_from_input = None;
            let mut output_from_context = None;
            let mut initial = None;
            let mut children = Vec::new();

            while !content.is_empty() {
                if content.peek(kw::context_from_input) {
                    content.parse::<kw::context_from_input>()?;
                    context_from_input = Some(parse_ident_item(&content)?);
                } else if content.peek(kw::context) {
                    content.parse::<kw::context>()?;
                    context_ty = Some(parse_typed_item(&content)?);
                } else if content.peek(kw::event) {
                    content.parse::<kw::event>()?;
                    event_decl = Some(parse_event_block(&content)?);
                } else if content.peek(kw::input) {
                    content.parse::<kw::input>()?;
                    input_ty = Some(parse_typed_item(&content)?);
                } else if content.peek(kw::output_from_context) {
                    content.parse::<kw::output_from_context>()?;
                    output_from_context = Some(parse_ident_item(&content)?);
                } else if content.peek(kw::output) {
                    content.parse::<kw::output>()?;
                    output_ty = Some(parse_typed_item(&content)?);
                } else if content.peek(kw::effect) {
                    content.parse::<kw::effect>()?;
                    effect_ty = Some(parse_typed_item(&content)?);
                } else if content.peek(kw::initial) {
                    content.parse::<kw::initial>()?;
                    initial = Some(parse_ident_item(&content)?);
                } else if content.peek(kw::state) {
                    content.parse::<kw::state>()?;
                    let child_name = content.parse::<Ident>()?;
                    children.push(StateItemAst::State(parse_state_body(&content, Some(child_name))?));
                } else if content.peek(kw::parallel) {
                    content.parse::<kw::parallel>()?;
                    children.push(StateItemAst::Parallel(parse_parallel(&content)?));
                } else if content.peek(Token![final]) {
                    content.parse::<Token![final]>()?;
                    children.push(StateItemAst::Final(parse_final(&content)?));
                } else {
                    return Err(content.error("expected context/event/input/output/effect/context_from_input/output_from_context/initial/state/parallel/final"));
                }
            }

            let (event_name, event_variants) = event_decl.ok_or_else(|| input.error("machine must declare `event <Name> { .. }`"))?;

            Ok(MachineAst {
                name,
                context_ty: context_ty.ok_or_else(|| input.error("machine must declare `context: Type;`"))?,
                event_name,
                event_variants,
                input_ty: input_ty.ok_or_else(|| input.error("machine must declare `input: Type;`"))?,
                output_ty: output_ty.ok_or_else(|| input.error("machine must declare `output: Type;`"))?,
                effect_ty: effect_ty.ok_or_else(|| input.error("machine must declare `effect: Type;`"))?,
                context_from_input: context_from_input.ok_or_else(|| input.error("machine must declare `context_from_input: ident;`"))?,
                output_from_context,
                initial: initial.ok_or_else(|| input.error("machine must declare `initial: ident;`"))?,
                children,
            })
        }
    }

    //#endregion 🔖️Parse
}

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
                let snapshot = machine::restore::<#machine_path>(&persisted, context, &[])
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

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn statechart_expands_minimal_machine_to_valid_rust() {
        let input = quote! {
            machine toggle {
                context: ToggleContext;
                event ToggleEvent { Flip }
                input: ();
                output: ();
                effect: ();
                context_from_input: build_context;
                initial: off;
                state off {
                    on Flip => on;
                }
                state on {
                    on Flip => off;
                }
            }
        };
        let expanded = expand_statechart(input).expect("expansion should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
    }

    #[test]
    fn statechart_rejects_duplicate_state_names() {
        let input = quote! {
            machine dup {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a { on Go => b; }
                state a { on Go => b; }
            }
        };
        let err = expand_statechart(input).expect_err("duplicate state names must be rejected");
        assert!(err.to_string().contains("duplicate state name"));
    }

    #[test]
    fn statechart_rejects_unknown_initial_state() {
        let input = quote! {
            machine bad_initial {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: nonexistent;
                state a { on Go => a; }
            }
        };
        let err = expand_statechart(input).expect_err("unknown initial state must be rejected");
        assert!(err.to_string().contains("unknown initial state"));
    }

    #[test]
    fn statechart_rejects_unknown_transition_target() {
        let input = quote! {
            machine bad_target {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a { on Go => nowhere; }
            }
        };
        let err = expand_statechart(input).expect_err("unknown transition target must be rejected");
        assert!(err.to_string().contains("unknown transition target"));
    }

    #[test]
    fn statechart_rejects_compound_without_initial() {
        let input = quote! {
            machine bad_compound {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a {
                    state nested { on Go => nested; }
                }
            }
        };
        let err = expand_statechart(input).expect_err("compound state without `initial:` must be rejected");
        assert!(err.to_string().contains("needs `initial:"));
    }

    #[test]
    fn statechart_expands_hierarchical_parallel_history_machine() {
        let input = quote! {
            machine media {
                context: MediaContext;
                event MediaEvent { Open, Pause, Play, Stop, Start, AudioDone, VideoDone }
                input: ();
                output: Receipt;
                effect: ();
                context_from_input: build_media;
                output_from_context: make_receipt;
                initial: closed;
                state closed {
                    on Open => open;
                }
                state open {
                    initial: playing;
                    entry on_open_enter;
                    exit on_open_exit;
                    invoke ticker;
                    after 30000 => closed do on_timeout;
                    history opened_history shallow;
                    state playing {
                        on Pause => paused;
                    }
                    state paused {
                        on Play if can_resume => playing;
                    }
                    on Stop => closed;
                }
                parallel recording {
                    state audio {
                        initial: audio_capturing;
                        state audio_capturing { on AudioDone => audio_done; }
                        final audio_done;
                    }
                    state video {
                        initial: video_capturing;
                        state video_capturing { on VideoDone => video_done; }
                        final video_done;
                    }
                    on_done => closed do on_recording_done;
                }
            }
        };
        let expanded = expand_statechart(input).expect("expansion of a hierarchical/parallel/history machine should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
    }

    #[test]
    fn derive_statechart_event_expands_mixed_variant_kinds() {
        let input: DeriveInput = syn::parse_quote! {
            enum Ev {
                Unit,
                Struct { field: i32 },
                Tuple(i32, i32),
            }
        };
        let expanded = expand_statechart_event(&input).expect("derive should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
        let text = expanded.to_string();
        assert!(text.contains("EVENT_COUNT"));
        assert!(text.contains("3u16") || text.contains("3"));
    }

    #[test]
    fn derive_statechart_event_rejects_non_enum() {
        let input: DeriveInput = syn::parse_quote! {
            struct NotAnEnum;
        };
        let err = expand_statechart_event(&input).expect_err("deriving on a struct must be rejected");
        assert!(err.to_string().contains("enums"));
    }

    #[test]
    fn derive_statechart_schema_expands_named_fields() {
        let input: DeriveInput = syn::parse_quote! {
            struct Ctx {
                order_id: String,
                attempts: u32,
            }
        };
        let expanded = expand_statechart_schema(&input).expect("derive should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
        let text = expanded.to_string();
        assert!(text.contains("SCHEMA_JSON"));
        assert!(text.contains("order_id"));
    }

    #[test]
    fn export_wasm_machine_expands_to_valid_rust() {
        let input = quote! { checkout::Checkout, "CheckoutMachine" };
        let expanded = expand_export_wasm_machine(input).expect("expansion should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
        let text = expanded.to_string();
        assert!(text.contains("CheckoutMachine"));
        assert!(text.contains("sendJson") || text.contains("send_json"));
    }
}

//#endregion 🧪️Tests
