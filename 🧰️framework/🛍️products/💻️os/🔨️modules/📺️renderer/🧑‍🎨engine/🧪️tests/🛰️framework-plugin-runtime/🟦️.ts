/** 🛰 Framework plugin runtime contract of the renderer. */
import { describe } from "vitest";
import "../🧰️engine-contract-helpers/🟦️.ts";
import { registerAdaptpluginhandleSOwnRefreshuiTests } from "./🔶️adaptpluginhandle-s-own-refreshui/🟦️.ts";
import { registerApplyuipatchtoretainedTests } from "./🔸️applyuipatchtoretained/🟦️.ts";
import { registerSerializesCompleteMultiTurnTests } from "./🟠️serializes-complete-multi-turn/🟦️.ts";
import { registerAdaptpluginhandleExposesSetmergepolicyResolveconflictTests } from "./🟢️adaptpluginhandle-exposes-setmergepolicy/🟦️.ts";

describe("framework plugin runtime", () => {
  registerAdaptpluginhandleSOwnRefreshuiTests();
  registerApplyuipatchtoretainedTests();
  registerSerializesCompleteMultiTurnTests();
  registerAdaptpluginhandleExposesSetmergepolicyResolveconflictTests();
});
