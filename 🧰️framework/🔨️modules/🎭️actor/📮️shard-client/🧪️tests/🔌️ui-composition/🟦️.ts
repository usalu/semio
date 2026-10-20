/** 🏦️ The shard client binds the UI resident composition seam when it loads, and the seam refuses objects that are not its composition or activation. */
import {expect,test} from "vitest";
import "../../🟦️.ts";
import {uiResidentComposition} from "@semio-tech/ui-contract/retained/host/composition";

test("the bound shard composition authority refuses foreign ledgers, clients, activations and pools",()=>{
  const authority=uiResidentComposition();
  expect(authority.matchesResidentLedger({},{})).toBe(false);
  expect(authority.matchesActivation({},{})).toBe(false);
});
