/** 📥️ The kernel binds the UI input host and the return content seam when its input module loads, and both refuse objects that are not the kernel's. */
import {expect,test} from "vitest";
import "../../🟦️.ts";
import {uiInputHost} from "@semio-tech/ui-contract/retained/host/input";
import {returnContentMatches} from "@semio-tech/ui-contract/retained/host/return-content";

const lifetime={activationGeneration:1n,instanceId:1,guestLifetime:1n},activation={actorId:"seam-actor",activationGeneration:1n,assertActive(){}};

test("the bound kernel input host refuses objects that are not kernel input classes",()=>{
  const host=uiInputHost();
  expect(host.field.matchesOwner({},{},activation,lifetime)).toBe(false);
  expect(host.field.matchesBuilder({},{})).toBe(false);
  expect(host.field.matchesResidentPayload({},{})).toBe(false);
  expect(host.fragment.matches({},{})).toBe(false);
  expect(host.release.matches({},{},{})).toBe(false);
  expect(host.detachment.matches({},{},{})).toBe(false);
});

test("the bound return content seam refuses an object that is not the parser of the return",()=>{
  expect(returnContentMatches({},{},{},activation,lifetime)).toBe(false);
});
