/** 🧫 The mcp module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
import { fixtureUrl } from "@semio-tech/framework/test/source-text";
export const channelAppPayloadsFixtureUrl = fixtureUrl(import.meta.url, "../../🐚️channel/🧪️tests/🧫️fixtures/🗿️app-payloads.json");
export const policyApprovalWithdrawalFixtureUrl = fixtureUrl(import.meta.url, "../../🛡️policy/🧪️tests/🧫️fixtures/🪦️approval-withdrawal.json");
export const loadMcpSchema = () => import("../../🧬️schema/🔣️.json").then(module => module.default);
