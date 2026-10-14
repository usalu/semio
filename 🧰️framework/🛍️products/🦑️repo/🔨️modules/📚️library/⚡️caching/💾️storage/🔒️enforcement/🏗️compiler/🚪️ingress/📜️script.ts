#!/usr/bin/env bun
/** 🚪️ Refuses unissued default Cargo compiler ingress before its version probe creates outputs. */
import policy from "./🔣️policy.json" with {type:"json"};
throw Error(policy.refusal);
