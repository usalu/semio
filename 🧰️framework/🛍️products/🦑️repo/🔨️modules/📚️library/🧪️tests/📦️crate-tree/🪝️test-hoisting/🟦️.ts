import { describe } from "bun:test";
import { registerRebasesIncludeArgumentsItemsTests } from "./🔶️rebases-include-arguments-items/🟦️.ts";
import { registerLeavesFixturesFeedLiteralTests } from "./🔸️leaves-fixtures-feed-literal/🟦️.ts";
import { registerInlinesRootFileModulesTests } from "./🟠️inlines-root-file-modules/🟦️.ts";

describe("artifact crate carve", () => {
  registerRebasesIncludeArgumentsItemsTests();
  registerLeavesFixturesFeedLiteralTests();
  registerInlinesRootFileModulesTests();
});
