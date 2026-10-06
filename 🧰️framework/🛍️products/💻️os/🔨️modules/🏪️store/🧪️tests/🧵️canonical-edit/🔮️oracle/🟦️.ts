import { test } from "bun:test";
import { testCanonicalEditFixtures } from "../🟦️.ts";
import { storeCanonicalEditSealerSelfTests } from "../../../🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts";

test("canonical edit payloads, bytes and retained source ownership agree with independent oracles", () => {
  testCanonicalEditFixtures();
  const observations = storeCanonicalEditSealerSelfTests();
  console.log("[DEBUG] canonical edit domain observations " + JSON.stringify(observations));
}, 30_000);
