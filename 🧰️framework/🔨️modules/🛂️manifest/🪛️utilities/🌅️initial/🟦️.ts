/** 🌅️ Resolves the first arm of an addressed window from its authored utility declaration. */
export type InitialWindowUtilityInput = Readonly<{
  windowId: string;
  utilityIds: readonly string[];
  initialUtilityId: string | null;
  activeUtilityByWindowId: Readonly<Record<string, string | null>>;
  activeToolId: string | null;
}>;

/** 🪛️ A one-time authority write; an existing nullable register remains authoritative. */
export type InitialWindowUtilityResolution = Readonly<{ write: boolean; utilityId: string | null }>;

/** 🧭️ Preserves resolved arms and clears, and respects a mode tool on first admission. */
export function resolveInitialWindowUtility(input: InitialWindowUtilityInput): InitialWindowUtilityResolution {
  if (input.initialUtilityId !== null && !input.utilityIds.includes(input.initialUtilityId)) throw new Error("initial utility must be accepted by its window kind");
  if (Object.hasOwn(input.activeUtilityByWindowId, input.windowId)) return { write: false, utilityId: input.activeUtilityByWindowId[input.windowId]! };
  return { write: true, utilityId: input.activeToolId === null ? input.initialUtilityId : null };
}
