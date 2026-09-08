import { toolJobFixedOperationRegistryExact } from "../../../../../📜️script.ts";

/** 🧪️ Executes tool job fixed operation registry policy assertions. */
export function toolJobFixedOperationRegistrySelfTests(source: string): number {
  if (!toolJobFixedOperationRegistryExact(source)) throw new Error("[verify interactivity tool-jobs fixed-operation-registry] valid scheduler authority source was rejected.");
  const anchors = [
    "pub operation: OperationId",
    "pub generation: Generation",
    "pub owner: T",
    "slots.try_reserve_exact(CAPACITY)",
    "CAPACITY <= Self::MAXIMUM_SLOTS",
    "self.retained_bytes.checked_add(retained_bytes)",
    "next_retained_bytes > self.maximum_bytes",
    "occupied: usize",
    "self.occupied == CAPACITY",
    "self.occupied == 0",
    "entry.key == key && !entry.closing",
    "entry.owner.cancel()",
    "entry.owner.begin_close()",
    "pub fn cancel_stale_step",
    "entry.key.generation == live_generation",
    "entry.owner.close_step(1, maximum_bytes)",
    "if entry.owner.terminal_is_empty()",
    "assert_eq!(self.occupied, 0, \"fixed operation registry reached Drop before every exact owner was terminal-empty\")",
    "maximum_plus_one_and_saturation_return_the_exact_owner",
    "stale_generation_interrupted_close_and_aba_preserve_exact_authority",
    "assert!(registry.take(stale).is_none())",
    "assert_eq!(byte_rejected.owner.identity, 12)",
    "assert_eq!(capacity_rejected.owner.identity, 16)",
    "assert!(!registry.is_empty(), \"interrupted close must retain the exact owner\")",
    "maximum_registry_backing_initializes_inside_one_interactive_ceiling_under_concurrent_load",
    "median < u128::from(semio_framework_trace::INTERACTIVE_STEP_CEILING_US)",
  ];
  for (const anchor of anchors) {
    const mutated = source.replaceAll(anchor, "HOSTILE_REMOVED_ANCHOR");
    if (mutated === source || toolJobFixedOperationRegistryExact(mutated)) throw new Error(`[verify interactivity tool-jobs fixed-operation-registry] hostile mutation was falsely accepted: ${anchor}`);
  }
  return anchors.length;
}
