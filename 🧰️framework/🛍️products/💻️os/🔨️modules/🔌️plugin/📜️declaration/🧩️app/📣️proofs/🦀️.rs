//! 🧬️ Bounded-first-step proof catalog at the actual proof type owner.
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
        fn bounded_first_step_tool_proofs() -> Vec<$crate::tool_contract::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::tool_contract::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::tool_contract::ArtifactBoundedFirstStepProof::new::<$owner>(
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
        fn bounded_first_step_tool_proofs() -> Vec<$crate::tool_contract::ArtifactBoundedFirstStepProof> {
            let qualify = |proof: $crate::tool_contract::ArtifactBoundedFirstStepProof| {
                $(let proof = proof.with_factory_type::<$owner, $factory_type>();)?
                proof
            };
            vec![$(
                qualify($crate::tool_contract::ArtifactBoundedFirstStepProof::new::<$owner>(
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
