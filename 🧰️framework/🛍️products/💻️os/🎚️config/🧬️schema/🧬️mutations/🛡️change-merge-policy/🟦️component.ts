/** 🛡️ Authoritative direct TypeScript leaf for the OS-wide conflict policy. */

import type { MergePolicy } from "@semio-tech/framework";
import type { MergePolicyConfigMutation } from "../🟦️component.ts";

//#region 🔖️Schema
/** 🛡️ `os.config.merge-policy` — the authority-local policy choice. */
export interface MergePolicySetting {
  readonly policy: MergePolicy;
}

/** 🪪️ The schema id for the merge-policy config facet. */
export const MERGE_POLICY_CONFIG_SCHEMA = "os.config.merge-policy";
//#endregion 🔖️Schema

//#region 🔖️Mutation
/** 🛡️ Replaces the active OS-wide merge policy. */
export interface ChangeMergePolicy {
  readonly policy: MergePolicy;
}

/** 🏗️ Wraps a change-merge-policy payload in the merge-policy dispatch union. */
export function changeMergePolicy(policy: MergePolicy): MergePolicyConfigMutation {
  return { mutation: "changeMergePolicy", policy };
}

/** 🔺️ Constructs the whole-record post-mutation setting. */
export function diff(payload: ChangeMergePolicy, base: MergePolicySetting): MergePolicySetting {
  return base.policy === payload.policy ? base : { policy: payload.policy };
}

/** ↩️ Restores the prior merge policy. */
export function inverse(_payload: ChangeMergePolicy, base: MergePolicySetting): MergePolicyConfigMutation[] {
  return [changeMergePolicy(base.policy)];
}
//#endregion 🔖️Mutation
