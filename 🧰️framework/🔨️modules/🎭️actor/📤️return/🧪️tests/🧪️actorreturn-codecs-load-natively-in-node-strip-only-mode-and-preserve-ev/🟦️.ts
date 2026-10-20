import { registerActorReturnCodecsLoadTests } from "./🔶️actor-return-codecs-load/🟦️.ts";
import { registerActorReturnResultMatchesTests } from "./🔸️actor-return-result-matches/🟦️.ts";
import { registerActorReturnResultFramingTests } from "./🟠️actor-return-result-framing/🟦️.ts";
import type { TestSource } from "./🟢️shared/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../📃️page/🟦️.ts"), "createActorBytePage" | "readActorBytePage"> & Pick<typeof import("../../🟦️.ts"), "ACTOR_RETURN_CONTROL_MAXIMUM_BYTES" | "ACTOR_RETURN_DRIVE_MAXIMUM_BYTES" | "ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES" | "ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES" | "ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES" | "ACTOR_RETURN_RESULT_MAXIMUM_BYTES" | "ActorReturnResultFraming" | "decodeActorReturnDrive" | "decodeActorReturnResult" | "encodeActorReturnDrive" | "encodeActorReturnResult">, testSource: TestSource): Promise<void> {
  const { it, resultOracle, hydrateResult, encodeActorReturnResult, expect, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, createActorBytePage, readActorBytePage, decodeActorReturnDrive, vi } = registerActorReturnCodecsLoadTests({ dependencies, vitest, testSource });
  registerActorReturnResultMatchesTests({ it, resultOracle, hydrateResult, encodeActorReturnResult, expect, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, createActorBytePage, readActorBytePage, decodeActorReturnDrive, vi });
  registerActorReturnResultFramingTests({ it, resultOracle, hydrateResult, expect, vi, encodeActorReturnResult, decodeActorReturnResult });
}
