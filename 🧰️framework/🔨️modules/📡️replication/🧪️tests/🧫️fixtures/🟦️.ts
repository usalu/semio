/** 🧫 The replication module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export { default as conflictReplayReportSchema } from "../../⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json" with { type: "json" };
export const codecHistoryLimitsSchemaUrl = fixtureUrl(import.meta.url, "../../⚙️codec/🎟️history-limits/🧬️schema/🔣️.json");
export const sourceAuthorityDomainsFixtureUrl = fixtureUrl(import.meta.url, "../../🎮️mutation/🧪️tests/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json");
export { default as mutationOutcomeCodeFixture } from "../../🎮️mutation/🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json" with { type: "json" };
export { default as causalHistoryTransitionFixture } from "../../🔗️causal/🧪️tests/🧫️fixtures/🧫️history-transition/🔣️.json" with { type: "json" };
export { default as replicationIoBinaryFixture } from "../../🚪️io/💾️binary/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
export { default as replicationIoBinarySchema } from "../../🚪️io/💾️binary/🧬️schema/🔣️.json" with { type: "json" };
export { default as presenceLivenessV1Fixture } from "./💓️presence-liveness-v1/🔣️.json" with { type: "json" };
export const formatVerificationFixtureUrl = fixtureUrl(import.meta.url, "../../📐️format/🔎️verification/🧪️tests/🧫️fixtures/🔣️.json");
export const backboneBatchV1FixtureUrl = fixtureUrl(import.meta.url, "../../🔗️causal/🧪️tests/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json");
export const loadReplicationApplyRefusalFixture = () => import("../../🚧️apply-refusal/🧪️tests/🧫️fixtures/🔣️.json").then(module => module.default);
export const loadReplicationApplyRefusalSchema = () => import("../../🚧️apply-refusal/🧬️schema/🔣️.json").then(module => module.default);
