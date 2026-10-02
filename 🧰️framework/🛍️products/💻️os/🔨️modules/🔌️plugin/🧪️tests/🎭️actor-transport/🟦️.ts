import * as vitest from "vitest";
import { fileURLToPath } from "node:url";
import * as shard from "../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import * as response from "../../../../../../🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts";
import { createActorBytePage } from "../../../../../../🔨️modules/🎭️actor/📃️page/🟦️.ts";
import { registerTests1 as registerResponse } from "../🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts";
import { registerTests1 as registerShard } from "../🧪️shardclient-reserved-response-settlement/🟦️.ts";
import { registerRetryableLifecycleDeadlineTests } from "../⏱️retryable-lifecycle-deadline/🟦️.ts";
import { registerCancelJobReplyTests } from "../🛑️cancel-job-reply/🟦️.ts";
import { registerComponentCodecReplyTests } from "../🧬️component-codec-reply/🟦️.ts";

const shardUrl = new URL("../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts", import.meta.url);
const responseUrl = new URL("../../../../../../🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts", import.meta.url);
const source = (url: URL) => ({ directory: fileURLToPath(new URL(".", url)), url: url.href });
await registerResponse(vitest, { ...response, createActorBytePage }, source(responseUrl));
await registerShard(vitest, shard.shardClientTestDependenciesV1(), source(shardUrl));
await registerRetryableLifecycleDeadlineTests(vitest, shard.shardClientRetryableLifecycleTestDependenciesV1(), source(shardUrl));
await registerCancelJobReplyTests(vitest, shard.shardClientTestDependenciesV1(), source(shardUrl));
await registerComponentCodecReplyTests(vitest, source(shardUrl));

