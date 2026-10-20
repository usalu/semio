/** 🔮️ Independent Buffer and ownership observations for plain shared examples. */
import { test } from "bun:test";
import { fixedListStorageSelfTests } from "../🔬️fixed-list-storage/🟦️.ts";
import { testBuiltTreeRetirementFixture } from "../../🏗️builder/♻️retirement/🧪️tests/🔬️built-tree-retirement/🟦️.ts";
import { surfaceOwnershipSelfTests } from "@semio-tech/ui/runtime/testing/surface-ownership";
import { testRuntimeTreeRetirement } from "@semio-tech/ui/runtime/retirement/tree/testing/runtime-tree-retirement";

test("fixed lists preserve independent byte arithmetic, copying and owner masks", () => { fixedListStorageSelfTests(); });
test("built tree retirement preserves independent ownership observations", testBuiltTreeRetirementFixture);
test("surface ownership preserves independent byte arithmetic", () => { surfaceOwnershipSelfTests(); });
test("runtime tree retirement preserves independent ownership observations", testRuntimeTreeRetirement);
