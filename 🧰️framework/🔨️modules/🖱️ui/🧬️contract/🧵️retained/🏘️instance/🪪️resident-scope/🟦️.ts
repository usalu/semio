import type { UiIndexGrant } from "../../🔌️host/🟦️.ts";
import type { RetainedUiWireStep } from "../../📦️wire/🟦️.ts";

/** 🪪️ The resident-scope side of the instance seam: what an instance needs to close a scope attached to it. The resident module implements it and binds its brand check through {@link bindResidentScopeMatcher}; the instance never imports the resident module. A leaf with no runtime imports, so the binding slot is initialised before either side evaluates. */
export interface OwnedUiResidentScope {
  beginClose(): void;
  closeStep(grant: UiIndexGrant): RetainedUiWireStep;
  terminalIsEmpty(): boolean;
}
export type ResidentScopeMatcher = (scope: unknown, owner: object, activation: object, lifetime: object) => boolean;
let residentScopeMatcher: ResidentScopeMatcher | null = null;
/** 🔗️ Binds, once, the resident module's brand check for attached scopes; an unbound seam refuses every attachment. */
export function bindResidentScopeMatcher(matcher: ResidentScopeMatcher): void {
  if (residentScopeMatcher) throw new Error("Resident scope matcher is already bound");
  residentScopeMatcher = matcher;
}
/** 🪪️ Whether `scope` is a genuine resident scope of exactly this owner, activation and lifetime. */
export function residentScopeMatches(scope: unknown, owner: object, activation: object, lifetime: object): scope is OwnedUiResidentScope {
  return residentScopeMatcher !== null && residentScopeMatcher(scope, owner, activation, lifetime);
}
