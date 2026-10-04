//#region 🧱️BoxedFixedSlots
/** 🧱️ One fixed-capacity slot table's budget row, mirroring `FixedSlotTableBudget` in `🦀️.rs` and
 * one entry of `🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — the language-agnostic record of which
 * registries build their slots straight on the heap through `boxed_fixed_slots`.
 *
 * 🇩🇪 Eine Zeile des Budgets für feste Slot-Tabellen. */
export type FixedSlotTableBudget = {
  readonly guard: string;
  readonly crate: string;
  readonly owner: string;
  readonly capacityConstant: string;
  readonly capacity: number;
  readonly elementType: string;
  readonly elementSizeBytes: number;
  readonly ownerSizeBytes: number;
};

/** 🧱️ The whole committed budget: the bounded thread stack every listed table's constructor must fit,
 * the size above which a slot table has to leave the caller's frame, and the tables themselves. */
export type BoxedFixedSlotsBudget = {
  readonly law: string;
  readonly provenance: string;
  readonly boundedThreadStackBytes: number;
  readonly workerPoolStackBytes: number;
  readonly conversionThresholdBytes: number;
  readonly tables: readonly FixedSlotTableBudget[];
};

/** 📏️ What one slot table would cost in the constructing frame if its array were built inline —
 * `capacity * size_of::<T>()`, the number `Box::new([const { None }; N])` actually reserves at
 * `opt-level = 0`. */
export const inlineSlotTableBytes = (table: FixedSlotTableBudget): number => table.capacity * table.elementSizeBytes;

/** 🧪️ Re-checks the committed budget's arithmetic without linking any Rust — the independent half of
 * the law the per-crate Rust guards assert against live `size_of`. Returns one human-readable fault
 * per violation, empty when the record is sound.
 *
 * 🇩🇪 Prüft die Arithmetik des Budgets unabhängig von Rust und liefert je Verletzung einen Befund. */
export const fixedSlotTableFaults = (budget: BoxedFixedSlotsBudget): readonly string[] => {
  const faults: string[] = [];
  if (budget.tables.length === 0) faults.push("the budget lists no slot table");
  if (budget.boundedThreadStackBytes > budget.workerPoolStackBytes) faults.push(`the guarded stack ${budget.boundedThreadStackBytes} B exceeds the worker-pool stack ${budget.workerPoolStackBytes} B it stands in for`);
  const seen = new Set<string>();
  for (const table of budget.tables) {
    const inline = inlineSlotTableBytes(table);
    if (seen.has(table.owner)) faults.push(`${table.owner} is listed twice`);
    seen.add(table.owner);
    if (!Number.isSafeInteger(table.capacity) || table.capacity <= 0) faults.push(`${table.owner} declares a non-positive capacity ${table.capacity}`);
    if (!Number.isSafeInteger(table.elementSizeBytes) || table.elementSizeBytes <= 0) faults.push(`${table.owner} declares a non-positive slot size ${table.elementSizeBytes}`);
    if (inline <= budget.conversionThresholdBytes) faults.push(`${table.owner} holds ${inline} B inline, at or under the ${budget.conversionThresholdBytes} B threshold — it does not need the heap-first helper`);
    if (table.ownerSizeBytes >= inline) faults.push(`${table.owner} is ${table.ownerSizeBytes} B against a ${inline} B slot table, so the table is still inline`);
    if (table.ownerSizeBytes > budget.boundedThreadStackBytes) faults.push(`${table.owner} is ${table.ownerSizeBytes} B, over the ${budget.boundedThreadStackBytes} B bounded stack on its own`);
  }
  return faults;
};
//#endregion 🧱️BoxedFixedSlots
