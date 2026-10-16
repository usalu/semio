//! 📜️ Facade-owned exported macros cut out of the WP2 trees: `app_commands!`, `app_action_enum!`, `view_commands!` (`📜️declaration/📨️emission`) and `bounded_first_step_tool_proofs!` (`📜️declaration/👷️job`).
//!
//! 🧭️ WP2 pre-created this empty module (PLAN-PLUGIN-CRATES.md, section 12.5.6): when the crate that receives the defining file is carved, WP2 moves the `#[macro_export]` definitions here verbatim, so `$crate::` keeps naming the facade.

/// 🏭️ Generates a closed per-app action enum plus its `AppAction` impl from a list of
/// `Variant = "actionId"` pairs — the ids should match what's passed to `.mutation()/.view_action()/
/// .shell_action()` on the app's `AppBuilder` so the declared action registry and the dispatch match
/// can't drift apart silently.
#[macro_export]
macro_rules! app_action_enum {
($vis:vis enum $Name:ident { $($Variant:ident = $id:literal),* $(,)? }) => {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    $vis enum $Name {
        $($Variant),*
    }

    impl $crate::app_emission::AppAction for $Name {
        async fn from_action_id(id: &str) -> Result<Self, String> {
            match id {
                $($id => Ok(Self::$Variant),)*
                other => Err(Fault::from(format!("unknown action id {other}"))),
            }
        }
    }
};
}

#[macro_export]
macro_rules! app_commands {
($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty, ctx = $Ctx:ty { $($id:literal as $key:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
    $(#[$meta])*
    #[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, semio_framework_dsl_record_derive::DslEnum)]
    $vis enum $Name {
        $(
            #[dsl(key = $key)]
            $Payload($module::$Payload),
        )*
    }

    impl $Name {
        /// 🎯️ Exact manifest ids owned by this generated typed command schema.
        pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];

        /// 🏷️ Each row's `$id` — the manifest action id, distinct from the `$key` wire keyword.
        pub fn command_id(&self) -> &'static str {
            match self {
                $(Self::$Payload(_) => $id,)*
            }
        }

        /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg, ctx)` — see
        /// `🔖️KeyedAndContextualForms` for why `ctx` exists.
        pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>, ctx: &mut $Ctx) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
            match self {
                $(Self::$Payload(payload) => $module::handle(payload, doc, cfg, ctx),)*
            }
        }
    }

    impl $crate::ToolCommandCatalog for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
    }

    impl ::protocol::OpText for $Name {
        fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = semio_framework_dsl_record::parse(
                        body,
                        &(spec_fn.ordinary)(),
                        &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline },
                    )?;
                    return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::semio_framework_diagnostic::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), ::semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
};

($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty { $($id:literal as $key:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
    $(#[$meta])*
    #[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, semio_framework_dsl_record_derive::DslEnum)]
    $vis enum $Name {
        $(
            #[dsl(key = $key)]
            $Payload($module::$Payload),
        )*
    }

    impl $Name {
        /// 🎯️ Exact manifest ids owned by this generated typed command schema.
        pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];

        /// 🏷️ Each row's `$id` — the manifest action id, distinct from the `$key` wire keyword.
        pub fn command_id(&self) -> &'static str {
            match self {
                $(Self::$Payload(_) => $id,)*
            }
        }

        /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg)`.
        pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
            match self {
                $(Self::$Payload(payload) => $module::handle(payload, doc, cfg),)*
            }
        }
    }

    impl $crate::ToolCommandCatalog for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
    }

    impl ::protocol::OpText for $Name {
        fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = semio_framework_dsl_record::parse(
                        body,
                        &(spec_fn.ordinary)(),
                        &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline },
                    )?;
                    return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::semio_framework_diagnostic::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), ::semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
};

($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Mutation:ty, $Config:ty, $ConfigMutation:ty { $($id:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
    $(#[$meta])*
    #[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, semio_framework_dsl_record_derive::DslEnum)]
    $vis enum $Name {
        $(
            #[dsl(key = $id)]
            $Payload($module::$Payload),
        )*
    }

    impl $Name {
        /// 🎯️ Exact manifest ids owned by this generated typed command schema.
        pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];

        /// 🏷️ Each row's `$id` — mirrors `ArtifactApp::command_id`'s per-command labeling need.
        pub fn command_id(&self) -> &'static str {
            match self {
                $(Self::$Payload(_) => $id,)*
            }
        }

        /// 🎯️ Matches each variant to `$module::handle(payload, doc, cfg)` — see `🔖️DispatchDesignChoice`
        /// above for why this is concrete (not generic) in the app's own associated types.
        pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>) -> Result<$crate::Emit<$Mutation, $ConfigMutation>, $crate::Fault> {
            match self {
                $(Self::$Payload(payload) => $module::handle(payload, doc, cfg),)*
            }
        }
    }

    impl $crate::ToolCommandCatalog for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
    }

    impl ::protocol::OpText for $Name {
        fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = semio_framework_dsl_record::parse(
                        body,
                        &(spec_fn.ordinary)(),
                        &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline },
                    )?;
                    return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::semio_framework_diagnostic::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), ::semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
};
}

/// 👁️ The read-only twin of [`app_commands!`] — same generated shape (enum, `TOOL_JOB_IDS`,
/// `command_id`, `ToolCommandCatalog`, `OpText`/`OpBinary`), but `dispatch` returns
/// [`ViewEmit`], so a viewer's typed command channel is STRUCTURALLY incapable of carrying an
/// artifact or draft mutation (contract §2.2) — the read-only guarantee is a type property of
/// the command enum itself, not a convention each payload module has to keep.
#[macro_export]
macro_rules! view_commands {
($(#[$meta:meta])* $vis:vis enum $Name:ident for $Snapshot:ty, $Config:ty, $ConfigMutation:ty { $($id:literal as $key:literal => $module:ident :: $Payload:ident),* $(,)? }) => {
    $(#[$meta])*
    #[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, semio_framework_dsl_record_derive::DslEnum)]
    $vis enum $Name {
        $(
            #[dsl(key = $key)]
            $Payload($module::$Payload),
        )*
    }

    impl $Name {
        /// 🎯️ Exact manifest ids owned by this generated typed command schema.
        pub const TOOL_JOB_IDS: &'static [&'static str] = &[$($id),*];

        /// 🏷️ Each row's `$id` — the manifest action id, distinct from the `$key` wire keyword.
        pub fn command_id(&self) -> &'static str {
            match self {
                $(Self::$Payload(_) => $id,)*
            }
        }

        /// 👁️ Matches each variant to `$module::handle(payload, doc, cfg)`, whose return type is
        /// `ViewEmit` — no artifact/draft lane exists to route.
        pub fn dispatch(&self, doc: &$crate::ArtifactView<'_, $Snapshot>, cfg: &$crate::ConfigView<'_, $Config>) -> Result<$crate::ViewEmit<$ConfigMutation>, $crate::Fault> {
            match self {
                $(Self::$Payload(payload) => $module::handle(payload, doc, cfg),)*
            }
        }
    }

    impl $crate::ToolCommandCatalog for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
    }

    impl ::protocol::OpText for $Name {
        fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            for (keyword, spec_fn) in &variants {
                let probe = format!("{keyword} ");
                if line == keyword.as_str() || line.starts_with(&probe) {
                    let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                    let record = semio_framework_dsl_record::parse(
                        body,
                        &(spec_fn.ordinary)(),
                        &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline },
                    )?;
                    return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
                }
            }
            Err(::semio_framework_diagnostic::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), ::semio_framework_diagnostic::TextSpan::at(1, 1)))
        }
        fn print_op(&self) -> String {
            let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
            let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
            let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
            let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
            if body.is_empty() {
                keyword
            } else {
                format!("{keyword} {body}")
            }
        }
    }

    impl ::protocol::OpBinary for $Name {
        const TOOL_JOB_IDS: &'static [&'static str] = Self::TOOL_JOB_IDS;
        fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
            ::dsl::variants_binary::encode_op(self)
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
            ::dsl::variants_binary::decode_op(bytes)
        }
    }
};
}

/// 🧬️ Declares an exact bounded-first-step proof catalog in the owning plugin source.
#[macro_export]
macro_rules! bounded_first_step_tool_proofs {
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        contract: $contract:expr,
        tools: [$($tool:literal),+ $(,)?]
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<$crate::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+]
        }
    };
    (
        owner: $owner:ty,
        owner_file: $owner_file:literal,
        controller: $controller:literal,
        artifact_schema: $artifact_schema:literal,
        factory: $factory:literal,
        $(factory_type: $factory_type:ty,)?
        tools: { $($tool:literal => $contract:expr),+ $(,)? }
    ) => {
        fn bounded_first_step_tool_proofs() -> Vec<$crate::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::ArtifactBoundedFirstStepProof::new::<$owner>(
                    $owner_file,
                    $controller,
                    $factory,
                    $tool,
                    $artifact_schema,
                    $contract,
                ))
            ),+]
        }
    };
}
