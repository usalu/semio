// #region 🧲️Header
// 💻️ framework/ui/modules/🏷️style-variants/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cn, type ClassNameInput } from "../🏷️class-name-composition/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧬️StyleVariantSchema
type StyleVariantPrimitive = string | number | boolean;

/** @emoji 🧬️ Declares named variants and their finite CSS-class choices. */
export type StyleVariantSchema = Readonly<Record<string, Readonly<Record<string, ClassNameInput>>>>;

type StyleVariantLiteral<Key extends string> = Key extends "true" | "false" ? boolean : Key;

/** @emoji 🎚️ Selects zero or one choice from each declared style variant. */
export type StyleVariantSelection<Schema extends StyleVariantSchema> = {
  readonly [Name in keyof Schema]?: StyleVariantLiteral<Extract<keyof Schema[Name], string>> | null;
};

type StyleVariantCondition<Schema extends StyleVariantSchema> = {
  readonly [Name in keyof Schema]?:
    | StyleVariantLiteral<Extract<keyof Schema[Name], string>>
    | readonly StyleVariantLiteral<Extract<keyof Schema[Name], string>>[];
};

/** @emoji 🧩️ Declares classes selected by a conjunction of variant choices. */
export type StyleCompoundVariant<Schema extends StyleVariantSchema> = StyleVariantCondition<Schema> & {
  readonly class?: ClassNameInput;
  readonly className?: ClassNameInput;
};

/** @emoji 📜️ Defines defaults and compound rules for one finite variant schema. */
export interface StyleVariantConfiguration<Schema extends StyleVariantSchema> {
  readonly variants: Schema;
  readonly defaultVariants?: StyleVariantSelection<Schema>;
  readonly compoundVariants?: readonly StyleCompoundVariant<Schema>[];
}

type StyleVariantInvocation<Schema extends StyleVariantSchema> = StyleVariantSelection<Schema> & {
  readonly class?: ClassNameInput;
  readonly className?: ClassNameInput;
};

/** @emoji 🧵️ Compiles a typed variant selection into one owned class-name string. */
export type StyleVariantCompiler<Schema extends StyleVariantSchema> = (selection?: StyleVariantInvocation<Schema>) => string;

/** @emoji 🏷️ Extracts the public variant selection accepted by an owned compiler. */
export type StyleVariantProps<Compiler> = Compiler extends StyleVariantCompiler<infer Schema> ? StyleVariantSelection<Schema> : never;
// #endregion 🧬️StyleVariantSchema

// #region 🧵️StyleVariantCompilation
function styleVariantValueEquals(actual: StyleVariantPrimitive | null | undefined, expected: unknown): boolean {
  return Array.isArray(expected) ? expected.some((candidate) => candidate === actual) : expected === actual;
}

/** @emoji 🪡️ Compiles base, selected, default, compound, and caller classes in stable schema order. */
export function styleVariants<const Schema extends StyleVariantSchema = {}>(
  base?: ClassNameInput,
  configuration?: StyleVariantConfiguration<Schema>,
): StyleVariantCompiler<Schema> {
  const variants = configuration?.variants ?? ({} as Schema);
  const names = Object.keys(variants);

  return (selection) => {
    const resolved: Record<string, StyleVariantPrimitive | null | undefined> = {};
    const selectedClasses = names.map((name) => {
      const selected = selection?.[name];
      const value = selected === null ? null : (selected ?? configuration?.defaultVariants?.[name]);
      resolved[name] = value as StyleVariantPrimitive | null | undefined;
      return value === null || value === undefined ? undefined : variants[name]?.[String(value)];
    });
    const compoundClasses = configuration?.compoundVariants?.flatMap((compound) => {
      const conditions = Object.entries(compound).filter(([name]) => name !== "class" && name !== "className");
      const matches = conditions.every(([name, expected]) => styleVariantValueEquals(resolved[name], expected));
      return matches ? [compound.class, compound.className] : [];
    });

    return cn(base, selectedClasses, compoundClasses, selection?.class, selection?.className);
  };
}
// #endregion 🧵️StyleVariantCompilation
