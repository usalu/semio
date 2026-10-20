/** 🩹️ The actor binds the UI host patch seam when its ui-patch module loads, and the UI instance then refuses a forged patch source. */
import {expect,test} from "vitest";
import "../🟦️.ts";
import {OwnedUiInstance} from "@semio-tech/ui-contract/retained/instance";
import {uiPatchAuthority} from "@semio-tech/ui-contract/retained/host";

const lifetime={activationGeneration:1n,instanceId:1,guestLifetime:1n},activation={actorId:"seam-actor",activationGeneration:1n,assertActive(){}};

test("the bound actor authority refuses objects that are not its patch sources or receipts",()=>{
  const authority=uiPatchAuthority();
  expect(authority.matches({},activation,lifetime)).toBe(false);
  expect(authority.matchesOwner({},{})).toBe(false);
  expect(authority.matchesReceipt({},{},{})).toBe(false);
});

test("an instance refuses a forged patch source once the actor has bound its brand checks",()=>{
  const instance=new OwnedUiInstance(activation,lifetime,{maxNodes:8,maxDepth:4,maxChildren:4,maxTextBytes:64,maxPatchOps:4,maxPatchBytes:256} as never,{usizeBits:64} as never);
  const forged={value:{activation,lifetime,receipt:{lifetime,patchSequence:1n},surface:"s",baseRevision:0,revision:1,operationCount:0},inputRetired:true,operation(){return undefined;},acceptInput(){return true;},releaseInput(){return true;}};
  expect(()=>instance.beginPatch(forged,{} as never)).toThrow("Foreign native instance patch owner");
});
