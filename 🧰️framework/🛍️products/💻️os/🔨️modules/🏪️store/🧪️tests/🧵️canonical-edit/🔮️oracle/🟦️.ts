import { test } from "bun:test";
import { testCanonicalEditFixtures } from "../🟦️.ts";
import { storeCanonicalEditSealerSelfTests } from "../../../🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts";

test("canonical edit payloads, bytes and retained source ownership agree with independent oracles", () => {
  testCanonicalEditFixtures();
  const observations = storeCanonicalEditSealerSelfTests();
}, 30_000);
