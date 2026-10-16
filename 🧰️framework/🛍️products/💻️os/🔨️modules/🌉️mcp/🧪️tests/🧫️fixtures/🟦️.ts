/** 🧫 The mcp module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export const channelAppPayloadsFixtureUrl = new URL("../../🐚️channel/🧪️tests/🧫️fixtures/🗿️app-payloads.json", import.meta.url);
export const policyApprovalWithdrawalFixtureUrl = new URL("../../🛡️policy/🧪️tests/🧫️fixtures/🪦️approval-withdrawal.json", import.meta.url);
export const loadMcpSchema = () => import("../../🧬️schema/🔣️.json").then(module => module.default);
