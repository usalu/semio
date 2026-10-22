#!/usr/bin/env repo
/** 🚪️ Refuses unissued default Cargo compiler ingress before its version probe creates outputs. */
import policy from "./🔣️policy.json" with {type:"json"};
throw new Error(policy.refusal);
