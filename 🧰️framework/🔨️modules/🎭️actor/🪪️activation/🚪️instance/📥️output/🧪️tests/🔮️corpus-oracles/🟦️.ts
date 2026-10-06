/** 🔮️ Actual actor output ownership laws with independent immutable mutation oracles. */
import { describe, expect, spyOn, test } from "bun:test";
import { OwnedResidentLedger } from "../../../../../../🌱️value/💾️resident/🟦️.ts";
import { OwnedActorTurnOutput, OwnedActorTurnOutputs, cancelEmpty } from "../../🟦️.ts";
import { registerTests1 } from "../🧪️ownedactorturnoutput/🟦️.ts";

await registerTests1({ describe, expect, it: test, vi: { spyOn } } as unknown as NonNullable<ImportMeta["vitest"]>, { OwnedResidentLedger, OwnedActorTurnOutput, OwnedActorTurnOutputs, cancelEmpty }, { directory: new URL("../..", import.meta.url).pathname, url: new URL("../../🟦️.ts", import.meta.url).href });
