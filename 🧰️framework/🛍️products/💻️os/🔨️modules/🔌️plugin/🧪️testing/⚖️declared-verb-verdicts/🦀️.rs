//! ⚖️ Immutable declared action observations, representative inputs and verdict policies.

    /// 🧬️ Preserves declared defaults and stages schema-validated representatives for required bridge inputs.
    pub fn declared_bridge_arguments(args: &[semio_framework::ActionArgDef]) -> Result<semio_framework::DslValue, String> {
        use semio_framework::{ArgSchema, DslValue};
        let mut representatives = Vec::new();
        for argument in args.iter().filter(|argument| argument.required && argument.default.is_none()) {
            let candidates = match &argument.schema {
                ArgSchema::String { options, .. } if options.len() == 1 => vec![DslValue::String(options[0].value.clone())],
                _ => declared_argument_alternatives(argument).map(|(first, second)| vec![first, second]).unwrap_or_default(),
            };
            let schema = argument.json_schema(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En);
            let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&semio_framework_pack_json::to_json_string(&schema)).map_err(|error| format!("required argument {} schema cannot be validated: {error}", argument.id))?;
            let value = candidates
                .into_iter()
                .find(|candidate| validator.validate_json(&semio_framework_pack_json::to_json_string(candidate)).is_ok())
                .ok_or_else(|| format!("required argument {} has no supported schema-valid representative: {:?}", argument.id, argument.schema))?;
            representatives.push((argument.id.clone(), value));
        }
        Ok(semio_framework::effective_action_args(args, &DslValue::Object(representatives), None))
    }


    /// 🎯️ What one dispatch of a declared verb did, observed exactly where the shell observes it: the
    /// host action entry (`PluginApp::handle_action`), the retained publication settle, and the state
    /// read back afterwards. The declaration (`ActionKind`, audience, `effects.destructive`, args) is
    /// what users, collaborators and agents rely on; these laws hold the handler to it
    /// (ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP, slice P8).
    #[derive(Debug)]
    pub enum DeclaredVerbOutcome {
        /// 🚪️ The dispatch infrastructure refused the verb before any handler ran (`interactive-job.*`,
        /// `app.command.unsupported`) — the declared verb cannot be invoked at all.
        Unreachable { code: String, detail: String, failure:Option<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> },
        /// 🛑️ The handler refused this invocation by name.
        Refused { code: String, detail: String, failure:Option<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> },
        /// ✅️ The operation settled; see [`DeclaredVerbEffect`].
        Settled(DeclaredVerbEffect),
    }

    impl PartialEq for DeclaredVerbOutcome{
        fn eq(&self,other:&Self)->bool{match(self,other){(Self::Unreachable{code:a,detail:b,..},Self::Unreachable{code:c,detail:d,..})|(Self::Refused{code:a,detail:b,..},Self::Refused{code:c,detail:d,..})=>a==c&&b==d,(Self::Settled(a),Self::Settled(b))=>a==b,_=>false}}
    }
    impl Eq for DeclaredVerbOutcome{}

    /// 📤️ The observable effect of one settled dispatch — published lanes, requested host effects and
    /// the document/config change read back from the stores.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct DeclaredVerbEffect {
        pub lanes: std::collections::BTreeSet<&'static str>,
        pub document_changed: bool,
        pub document_replaced: bool,
        pub config_changed: bool,
        pub user_path_written: bool,
        pub host_effects: usize,
        pub fingerprint: u64,
        /// 🪆️ `slot/child_id` of every composed child the parent names after the verb and no store holds,
        /// that it did not already name unheld before the verb.
        pub orphaned_children: Vec<String>,
    }

    impl DeclaredVerbEffect {
        /// 🔇️ Nothing observable happened: no store lane beyond the terminal/UI witnesses, no host
        /// effect or event, no document or config change.
        pub fn is_silent(&self) -> bool {
            !self.touches_document() && !self.config_changed && !self.user_path_written && self.host_effects == 0 && self.lanes.iter().all(|lane| matches!(*lane, "terminal" | "ui"))
        }

        /// 📝️ Wrote the document (its own ops or an owned child's) or asked the host to replace it.
        pub fn touches_document(&self) -> bool {
            self.document_changed || self.document_replaced || self.lanes.contains("artifact") || self.lanes.contains("child")
        }
    }

    /// 🧪️ One declared verb probed on its app's own boot fixture, from every window kind that presents
    /// it (a window-claimed verb from its own window, an app-level verb from each window a shell can
    /// dispatch it from — a window-config verb only acts from its own window).
    #[derive(Debug, PartialEq)]
    pub struct DeclaredVerbProbe {
        pub verb: String,
        pub kind: semio_framework::ActionKind,
        pub audience: semio_framework::CapabilityAudience,
        pub destructive: bool,
        pub bridge: Result<(), String>,
        pub windows: Vec<DeclaredVerbWindowProbe>,
        /// 🤖️ The same staged invocation through the agent lane (`dispatch_command_frame`, the MCP
        /// `action_prepare` phase) at the address the capability catalog publishes; `None` for a
        /// verb agents never see (Input/Chrome audience).
        pub agent: Option<DeclaredVerbOutcome>,
    }

    /// 🪟️ The verb dispatched from one window kind: once with the staged declared defaults, then twice
    /// per perturbable declared argument with two distinct valid values.
    #[derive(Debug, PartialEq)]
    pub struct DeclaredVerbWindowProbe {
        pub window: String,
        pub staged: DeclaredVerbOutcome,
        pub arguments: Vec<DeclaredArgumentProbe>,
    }

    /// 🔀️ The same verb dispatched with two distinct values of one declared argument. `others_specified`
    /// says every OTHER declared argument carried a value (a declared default or the surface's example),
    /// so a refusal cannot be blamed on a missing sibling.
    #[derive(Debug, PartialEq)]
    pub struct DeclaredArgumentProbe {
        pub argument: String,
        pub others_specified: bool,
        pub first: DeclaredVerbOutcome,
        pub second: DeclaredVerbOutcome,
    }

    /// ⚖️ One way a handler breaks its verb's declaration.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum DeclaredVerbFinding {
        /// 📝️ A `View`/`🐚️Shell` verb wrote or replaced the document (law a).
        DocumentWriteFromNonMutation { verb: String, kind: String },
        /// 🔇️ An agent-facing `Mutation` verb settled without any observable effect for every probe (law b).
        SilentMutation { verb: String },
        /// 🔀️ Two distinct values of one declared argument produced the identical outcome (law c).
        IgnoredArgument { verb: String, argument: String },
        /// ⚠️ A verb declared destructive settled without writing the document or a user file.
        DestructiveWithoutDiscard { verb: String },
        /// 🚪️ The dispatch infrastructure refused the declared verb before its handler ran.
        Unreachable { verb: String, code: String },
        /// 🌉️ The declared verb does not bridge to a command with its staged declared defaults.
        Unbridged { verb: String, detail: String },
        /// 🤖️ An agent invoking the verb gets a different effect than a human pressing it: refused or
        /// silent where the shell acts, or writing the document where the shell does not (and vice versa).
        AgentLaneDiverges { verb: String, shell: String, agent: String },
        /// 🪆️ The verb left the parent naming a composed child no store holds — every later verb, window and
        /// reload that reads the child finds nothing (flow's parent-lane edits re-minted its content child).
        ComposedChildOrphaned { verb: String, children: String },
    }

    impl std::fmt::Display for DeclaredVerbFinding {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DeclaredVerbFinding::DocumentWriteFromNonMutation { verb, kind } => write!(formatter, "{verb}: declared {kind} but wrote or replaced the document"),
                DeclaredVerbFinding::SilentMutation { verb } => write!(formatter, "{verb}: declared Mutation but every probe settled silently — refuse by name or act"),
                DeclaredVerbFinding::IgnoredArgument { verb, argument } => write!(formatter, "{verb}: two distinct values of declared argument `{argument}` produced the identical outcome"),
                DeclaredVerbFinding::DestructiveWithoutDiscard { verb } => write!(formatter, "{verb}: declared destructive but settled without writing the document or a user file"),
                DeclaredVerbFinding::Unreachable { verb, code } => write!(formatter, "{verb}: the dispatch infrastructure refused it before its handler ran ({code})"),
                DeclaredVerbFinding::Unbridged { verb, detail } => write!(formatter, "{verb}: does not bridge with its staged declared defaults ({detail})"),
                DeclaredVerbFinding::AgentLaneDiverges { verb, shell, agent } => write!(formatter, "{verb}: the agent lane diverges from the shell lane (shell {shell}; agent {agent})"),
                DeclaredVerbFinding::ComposedChildOrphaned { verb, children } => write!(formatter, "{verb}: leaves the parent naming composed children no store holds ({children})"),
            }
        }
    }

    /// ⚖️ The declared-verb verdict — a pure function of one probe, stated once for every plugin and
    /// held against the language-agnostic fixture `🧫️fixtures/⚖️declared-verb-verdicts.json`.
    /// (a) no `View`/`🐚️Shell` verb touches the document from any window; (b) an agent-facing
    /// `Mutation` verb never settles silently on every probe of every window; (c) no agent- or
    /// chrome-facing verb ignores a declared argument in every window that probed it; a destructive
    /// verb discards something whenever it settles; every declared agent or chrome verb bridges with
    /// its staged declared defaults (an Input gesture's payload is the host's) and every declared verb
    /// is reachable from at least one window; an agent-facing verb's agent lane writes the document
    /// exactly when the shell lane does and acts wherever the shell acts (never refused or silent there).
    pub fn declared_verb_findings(probe: &DeclaredVerbProbe) -> Vec<DeclaredVerbFinding> {
        use semio_framework::{ActionKind, CapabilityAudience};
        let verb = probe.verb.clone();
        if let Err(detail) = &probe.bridge {
            return match probe.audience {
                CapabilityAudience::Input => Vec::new(),
                _ => vec![DeclaredVerbFinding::Unbridged { verb, detail: detail.clone() }],
            };
        }
        if let Some(DeclaredVerbOutcome::Unreachable { code, .. }) =
            probe.windows.iter().map(|window| &window.staged).find(|staged| matches!(staged, DeclaredVerbOutcome::Unreachable { .. })).filter(|_| probe.windows.iter().all(|window| matches!(window.staged, DeclaredVerbOutcome::Unreachable { .. })))
        {
            return vec![DeclaredVerbFinding::Unreachable { verb, code: code.clone() }];
        }
        let outcomes = || probe.windows.iter().flat_map(|window| std::iter::once(&window.staged).chain(window.arguments.iter().flat_map(|argument| [&argument.first, &argument.second])));
        let settled = || outcomes().filter_map(|outcome| if let DeclaredVerbOutcome::Settled(effect) = outcome { Some(effect) } else { None });
        let mut findings = Vec::new();
        if !matches!(probe.kind, ActionKind::Mutation) && settled().any(DeclaredVerbEffect::touches_document) {
            findings.push(DeclaredVerbFinding::DocumentWriteFromNonMutation { verb: verb.clone(), kind: format!("{:?}", probe.kind) });
        }
        if matches!(probe.kind, ActionKind::Mutation) && probe.audience == CapabilityAudience::Agent && outcomes().all(|outcome| matches!(outcome, DeclaredVerbOutcome::Settled(effect) if effect.is_silent())) {
            findings.push(DeclaredVerbFinding::SilentMutation { verb: verb.clone() });
        }
        if probe.destructive && settled().next().is_some() && settled().all(|effect| !effect.touches_document() && !effect.user_path_written) {
            findings.push(DeclaredVerbFinding::DestructiveWithoutDiscard { verb: verb.clone() });
        }
        if let Some(effect) = settled().find(|effect| !effect.orphaned_children.is_empty()) {
            findings.push(DeclaredVerbFinding::ComposedChildOrphaned { verb: verb.clone(), children: effect.orphaned_children.join(", ") });
        }
        if let Some(agent) = &probe.agent {
            let shell_writes = probe.windows.iter().any(|window| declared_verb_wrote_document(&window.staged));
            let shell_acts = probe.windows.iter().any(|window| matches!(&window.staged, DeclaredVerbOutcome::Settled(effect) if !effect.is_silent()));
            let agent_writes = declared_verb_wrote_document(agent);
            let agent_acts = matches!(agent, DeclaredVerbOutcome::Settled(effect) if !effect.is_silent());
            if shell_writes != agent_writes || (shell_acts && !agent_acts) {
                let describe = |outcome: &DeclaredVerbOutcome| match outcome {
                    DeclaredVerbOutcome::Unreachable { code, .. } => format!("unreachable {code}"),
                    DeclaredVerbOutcome::Refused { code, .. } => format!("refused {code}"),
                    DeclaredVerbOutcome::Settled(effect) if effect.touches_document() => "writes the document".to_string(),
                    DeclaredVerbOutcome::Settled(effect) if effect.is_silent() => "changes nothing".to_string(),
                    DeclaredVerbOutcome::Settled(_) => "acts without writing the document".to_string(),
                };
                let shell = probe.windows.iter().map(|window| format!("{}: {}", window.window, describe(&window.staged))).collect::<Vec<_>>().join(", ");
                findings.push(DeclaredVerbFinding::AgentLaneDiverges { verb: verb.clone(), shell, agent: describe(agent) });
            }
        }
        if probe.audience != CapabilityAudience::Input {
            let mut names: Vec<&str> = Vec::new();
            for argument in probe.windows.iter().flat_map(|window| window.arguments.iter()) {
                if !names.contains(&argument.argument.as_str()) {
                    names.push(argument.argument.as_str());
                }
            }
            for name in names {
                let mut probes = probe.windows.iter().flat_map(|window| window.arguments.iter().map(move |argument| (&window.staged, argument))).filter(|(_, argument)| argument.argument == name).peekable();
                if probes.peek().is_some() && probes.all(|(staged, argument)| declared_verb_outcomes_agree(staged, &argument.first, &argument.second, argument.others_specified)) {
                    findings.push(DeclaredVerbFinding::IgnoredArgument { verb: verb.clone(), argument: name.to_string() });
                }
            }
        }
        findings
    }

    /// 🟰️ Two outcomes an observer cannot tell apart. Two equal settles always agree. Two infrastructure
    /// refusals prove nothing about the argument. Two equal handler refusals agree only when every
    /// sibling argument carried a value (otherwise the missing sibling may be what was refused) and the
    /// staged invocation, which leaves this argument at its default, was refused identically too — a
    /// body both perturbations fail to parse the same way was still read, since its default did not.
    fn declared_verb_outcomes_agree(staged: &DeclaredVerbOutcome, first: &DeclaredVerbOutcome, second: &DeclaredVerbOutcome, others_specified: bool) -> bool {
        match (first, second) {
            (DeclaredVerbOutcome::Settled(first), DeclaredVerbOutcome::Settled(second)) => first == second,
            (DeclaredVerbOutcome::Refused { .. }, DeclaredVerbOutcome::Refused { .. }) => others_specified && first == second && staged == first,
            _ => false,
        }
    }

    /// 🔀️ Two distinct valid values for one declared argument, or `None` when its schema admits no
    /// generic pair (a single-option choice, a record, a host-resolved choice, `Any`).
    pub fn declared_argument_alternatives(argument: &semio_framework::ActionArgDef) -> Option<(semio_framework::DslValue, semio_framework::DslValue)> {
        use semio_framework::{ArgFormat, ArgSchema, DslValue};
        let text = |value: &str| DslValue::String(value.to_string());
        match &argument.schema {
            ArgSchema::String { options, .. } if options.len() >= 2 => Some((text(&options[0].value), text(&options[1].value))),
            ArgSchema::String { options, .. } if !options.is_empty() => None,
            ArgSchema::String { format: Some(ArgFormat::ArtifactKind { .. } | ArgFormat::SurfaceApp { .. }), .. } => None,
            ArgSchema::String { format: Some(ArgFormat::Json), .. } => Some((text("{}"), text("[]"))),
            ArgSchema::String { .. } => Some((text("p8-alpha"), text("p8-beta"))),
            ArgSchema::Number { min, max, integer, .. } => {
                let (low, high) = match (min, max) {
                    (Some(low), Some(high)) if high > low => (*low, *high),
                    (Some(low), _) => (*low, *low + 1.0),
                    (None, Some(high)) => (*high - 1.0, *high),
                    (None, None) => (1.0, 2.0),
                };
                let number = |value: f64| {
                    if *integer && value >= 0.0 {
                        DslValue::uint(value as u64)
                    } else if *integer {
                        DslValue::int(value as i64)
                    } else {
                        DslValue::float(value)
                    }
                };
                Some((number(low), number(high)))
            }
            ArgSchema::Boolean => Some((DslValue::Bool(false), DslValue::Bool(true))),
            ArgSchema::Vector { dims, .. } => {
                let vector = |start: f64, direction: f64| DslValue::Array((0..*dims).map(|index| DslValue::float(start + direction * f64::from(index))).collect());
                Some((vector(1.0, 1.0), vector(f64::from(*dims), -1.0)))
            }
            ArgSchema::Reference { many: false, .. } => Some((text("p8-alpha"), text("p8-beta"))),
            ArgSchema::Reference { many: true, .. } => Some((DslValue::Array(vec![text("p8-alpha")]), DslValue::Array(vec![text("p8-beta")]))),
            ArgSchema::Array { items, .. } => match items.as_ref() {
                ArgSchema::String { options, .. } if options.len() >= 2 => Some((DslValue::Array(vec![text(&options[0].value)]), DslValue::Array(vec![text(&options[1].value)]))),
                ArgSchema::String { options, .. } if options.is_empty() => Some((DslValue::Array(vec![text("p8-alpha")]), DslValue::Array(vec![text("p8-beta")]))),
                ArgSchema::Number { .. } => Some((DslValue::Array(vec![DslValue::uint(1)]), DslValue::Array(vec![DslValue::uint(2)]))),
                _ => None,
            },
            ArgSchema::String { .. } | ArgSchema::Object { .. } | ArgSchema::Any => None,
        }
    }


    /// 🤖️ The verbs whose agent lane diverges from their shell lane, in probe order — a surface pins
    /// this list so every divergence is named in its own law until the agent lane runs the shell
    /// lane's code (ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP, slice P8), and a new one
    /// turns the law red.
    pub fn declared_verb_agent_divergences(probes: &[DeclaredVerbProbe]) -> Vec<String> {
        probes.iter().filter(|probe| declared_verb_findings(probe).iter().any(|finding| matches!(finding, DeclaredVerbFinding::AgentLaneDiverges { .. }))).map(|probe| probe.verb.clone()).collect()
    }

    /// 🔎️ The probe of one declared verb, for a caller pinning what its examples made that verb do.
    pub fn declared_verb_probe<'a>(probes: &'a [DeclaredVerbProbe], verb: &str) -> &'a DeclaredVerbProbe {
        probes.iter().find(|probe| probe.verb == verb).unwrap_or_else(|| panic!("verb {verb} was not probed"))
    }

    /// 📝️ Whether this outcome settled and wrote or replaced the document.
    pub fn declared_verb_wrote_document(outcome: &DeclaredVerbOutcome) -> bool {
        matches!(outcome, DeclaredVerbOutcome::Settled(effect) if effect.touches_document())
    }

    /// 📝️ Whether the verb's staged invocation wrote or replaced the document from any window.
    pub fn declared_verb_staged_wrote_document(probe: &DeclaredVerbProbe) -> bool {
        probe.windows.iter().any(|window| declared_verb_wrote_document(&window.staged))
    }

    /// 📝️ Whether any invocation of the verb, staged or perturbed, wrote or replaced the document.
    pub fn declared_verb_ever_wrote_document(probe: &DeclaredVerbProbe) -> bool {
        probe.windows.iter().any(|window| declared_verb_wrote_document(&window.staged) || window.arguments.iter().any(|argument| declared_verb_wrote_document(&argument.first) || declared_verb_wrote_document(&argument.second)))
    }


#[cfg(test)]
include!("../../🧪️tests/⚖️declared-verb-verdicts/🦀️.rs");
