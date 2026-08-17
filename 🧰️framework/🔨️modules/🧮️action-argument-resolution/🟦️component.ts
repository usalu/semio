// #region 🧮️ActionArgumentResolution
/** 🧮️ Resolves staged and default action arguments, then identifies unresolved required arguments. */
import type { ActionArgDef } from "../🛂️manifest/🟦️component.ts";

/** 🌱️ `seed` carries a dialog's pre-seeded context args (e.g. a row-scoped `spaceId` that is never a
 * declared, editable form field) through untouched: declared `defs` still resolve staged-then-default
 * as before, but any `seed` key that is not a declared arg id survives into the result unmodified, and
 * a `seed` value for a declared id that the form hasn't staged yet acts as that field's initial value.
 * A dialog with zero declared `defs` (a plain confirm/cancel) passes `seed`+`staged` through wholesale. */
export function effectiveActionArgs(defs: readonly ActionArgDef[], staged: Readonly<Record<string, unknown>>, seed?: Readonly<Record<string, unknown>>): Record<string, unknown> {
  if (defs.length === 0) return { ...seed, ...staged };
  const effective: Record<string, unknown> = seed ? { ...seed } : {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (!Object.prototype.hasOwnProperty.call(effective, def.id) && def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}

export function missingRequiredArgs(defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): string[] {
  return defs
    .filter((def) => def.required)
    .filter((def) => {
      const value = effective[def.id];
      return value === undefined || value === null || value === "";
    })
    .map((def) => def.id);
}
// #endregion 🧮️ActionArgumentResolution
