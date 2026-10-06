/** 🔮️ Independent Buffer and ownership observations for plain shared examples. */
import { test } from "bun:test";
import { fixedListStorageSelfTests } from "../🔬️fixed-list-storage/🟦️.ts";
import { testBuiltTreeRetirementFixture } from "../../♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts";
import { surfaceOwnershipSelfTests } from "../../../🧠️runtime/🧪️tests/🔬️surface-ownership/🟦️.ts";
import { testRuntimeTreeRetirement } from "../../../🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts";

test("fixed lists preserve independent byte arithmetic, copying and owner masks", () => { fixedListStorageSelfTests(); });
test("built tree retirement preserves independent ownership observations", testBuiltTreeRetirementFixture);
test("surface ownership preserves independent byte arithmetic", () => { surfaceOwnershipSelfTests(); });
test("runtime tree retirement preserves independent ownership observations", testRuntimeTreeRetirement);
