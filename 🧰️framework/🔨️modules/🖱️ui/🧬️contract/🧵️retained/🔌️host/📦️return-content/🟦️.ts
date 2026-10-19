import type { UiActivationLease, UiInstanceLifetime } from "../🟦️.ts";

/** 📦️ Return-content seam: how a published shard return accepts the content parser bound to it. The actor holds the content opaquely and asks this seam whether a candidate is the genuine parser of exactly this return, host, activation and lifetime; the kernel binds its brand check once through {@link bindReturnContentMatcher}. A leaf with no runtime imports, so neither side imports the other and the slot is initialised before either evaluates. */
export interface ReturnContentAuthority {
  matches(content: unknown, source: object, host: object, activation: UiActivationLease, lifetime: UiInstanceLifetime): boolean;
}

let authority: ReturnContentAuthority | null = null;
/** 🔗️ Binds, once, the content consumer's brand check; an unbound seam refuses every binding. */
export function bindReturnContentMatcher(bound: ReturnContentAuthority): void {
  if (authority) throw new Error("Return content matcher is already bound");
  authority = bound;
}
/** 🪪️ Whether `content` is the genuine parser of exactly this return, host, activation and lifetime. */
export function returnContentMatches(content: unknown, source: object, host: object, activation: UiActivationLease, lifetime: UiInstanceLifetime): boolean {
  return authority !== null && authority.matches(content, source, host, activation, lifetime);
}
