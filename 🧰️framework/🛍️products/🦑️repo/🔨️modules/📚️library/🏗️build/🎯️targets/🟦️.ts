import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🎯️ Selects one canonical tool-neutral build leaf from its closed authored contract. */
export function canonicalBuildTargetV1(slice: string): string {
  const targets = schema.$defs.Targets.properties;
  if (!Object.hasOwn(targets, slice)) throw new Error(`Unknown build slice: ${slice}`);
  return targets[slice as keyof typeof targets].const;
}
