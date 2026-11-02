import { describe, expect, it } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "@babel/parser";
import { calc } from "@csstools/css-calc";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { chromeControlItemBaseClass, chromeControlGroupShellClass } from "../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { resolveThemeGeometry, semioTheme } from "../../../🎨️styling/📦️packages/🟦️typescript/🟦️.ts";
import contract from "./🔣️.json";

const uiRoot = resolve(import.meta.dir, "../../..");
const actionSource = readFileSync(resolve(import.meta.dir, "../🟦️.tsx"), "utf8");
const badgeSource = readFileSync(resolve(uiRoot, "🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx"), "utf8");
const nodes = (value: unknown): any[] => {
  if (!value || typeof value !== "object") return [];
  if (Array.isArray(value)) return value.flatMap(nodes);
  return [value, ...Object.values(value).flatMap(nodes)];
};
const ast = (source: string) => nodes(parse(source, { sourceType: "module", plugins: ["typescript", "jsx"] }));
const slotClasses = (source: string, slot: string) => {
  const element = ast(source).find(node => node.type === "JSXOpeningElement" && node.attributes.some((attribute: any) => attribute.name?.name === "data-slot" && attribute.value?.value === slot));
  const attribute = element.attributes.find((attribute: any) => attribute.name?.name === "className");
  return nodes(attribute).filter(node => node.type === "StringLiteral").map(node => node.value).join(" ");
};
const checkClasses = (classes: string, required: string[], forbidden: string[] = []) => {
  const tokens = classes.split(/\s+/);
  for (const token of required) expect(tokens).toContain(token);
  for (const token of forbidden) expect(tokens).not.toContain(token);
};
const oracle = (expression: string) => Number.parseFloat(calc(`calc(${expression})`, { toCanonicalUnits: true }));

describe("Badge and ActionGroup spacing contracts", () => {
  it("gives every badge tone semantic horizontal padding and icon clearance", () => {
    const classes = slotClasses(badgeSource, "badge");
    checkClasses(classes, contract.badge.required, contract.badge.forbidden);
    expect(badgeSource).toContain("size-tiny [&_svg]:size-full");
    expect(oracle("2 * 3.2px")).toBeCloseTo(contract.badge.paddingPx);
    expect(oracle("1 * 3.2px")).toBeCloseTo(contract.badge.gapPx);
  });

  it("contains padded compact icons inside the medium shell without clipping", () => {
    const declaration = ast(actionSource).find(node => node.type === "VariableDeclarator" && node.id.name === "actionGroupItemVariants");
    const overrides = nodes(declaration.init).filter(node => node.type === "StringLiteral").map(node => node.value).join(" ");
    const item = cn(chromeControlItemBaseClass, overrides);
    const shell = cn(chromeControlGroupShellClass, slotClasses(actionSource, "action-group"));
    const vector = contract.actionGroup;
    const theme = semioTheme();
    const geometry = resolveThemeGeometry(theme);
    expect(geometry.scalars.control_height).toBeCloseTo(vector.shellPx);
    expect(oracle(`${theme.metrics.chrome!.sizeTinyUiSpacing} * ${geometry.spacingPx}px`)).toBeCloseTo(vector.iconPx);
    expect(geometry.spacingPx).toBeCloseTo(vector.paddingPx);
    checkClasses(shell, vector.shellRequired, ["h-small", "overflow-hidden"]);
    checkClasses(item, vector.itemRequired, vector.itemForbidden);
    for (const token of ["disabled:pointer-events-none", "disabled:opacity-50", "focus-visible:ring-0"]) expect(item.split(/\s+/)).toContain(token);
    const inner = vector.shellPx - 2 * vector.shellBorderPx;
    expect(inner).toBeCloseTo(vector.itemPx);
    expect(oracle(`${vector.shellPx}px - 2 * ${vector.shellBorderPx}px`)).toBeCloseTo(inner);
    const vertical = (inner - vector.iconPx) / 2;
    const horizontal = (inner - vector.dividerPx - vector.iconPx) / 2;
    expect(vertical).toBeCloseTo(vector.verticalClearancePx);
    expect(horizontal).toBeCloseTo(vector.horizontalClearancePx);
    expect(oracle(`(${inner}px - ${vector.iconPx}px) / 2`)).toBeCloseTo(vertical);
    expect(oracle(`(${inner}px - ${vector.dividerPx}px - ${vector.iconPx}px) / 2`)).toBeCloseTo(horizontal);
    expect(vertical).toBeGreaterThan(vector.paddingPx);
    expect(horizontal).toBeGreaterThan(vector.paddingPx);
  });
});
