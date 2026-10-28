import { createHash } from "node:crypto";
import { refuseBuildV1 } from "../../../../📖️texts/🟦️.ts";
import { buildSchedulingNativePrefixesV1, buildSchedulingPrefixesV1 } from "../../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import table from "./🔣️.json" with { type: "json" };

if (validateJsonSchemaSubset(schema, table).length) throw refuseBuildV1("invalid", "closure.walk-views-required");

/** 👓️ The admitted view policy: the views a project may read a command walk through and the view of each closure recipe. See `./🔣️.json` (the policy), `../../../../📥️source/🔏️closure/🗓️scheduling` (the prefixes) and `./🧪️tests/🟦️.ts` (the law). */
export const WALK_VIEW_POLICY_V1 = Object.freeze(structuredClone(table));

const PREFIX_SETS_V1 = Object.freeze({ scheduling: () => buildSchedulingPrefixesV1(), "scheduling-native": () => buildSchedulingNativePrefixesV1() });

/** 🎯️ The view every consumer of one closure recipe reads its walks through, or undefined for the whole walk: a `closure-of` recipe reads through the view its declared role names, a shared closure (`script-provider`, `tool-closure`) through its own view, any other recipe through the view the policy names for it (none: the whole walk). */
export function walkViewOfRecipeV1(recipe) {
  if (recipe.fn === "closure-of") return closureRoleViewV1(recipe.role);
  if (recipe.fn === "script-provider" || recipe.fn === "tool-closure") { walkViewPrefixesV1(recipe.view); return recipe.view; }
  return Object.hasOwn(WALK_VIEW_POLICY_V1.recipes, recipe.fn) ? WALK_VIEW_POLICY_V1.recipes[recipe.fn] : undefined;
}

/** 🎭️ The view one declared closure role reads its walk through, or undefined for a role the policy keeps whole; a role the policy does not declare is refused (every `closure-of` names one). */
export function closureRoleViewV1(role) {
  if (!Object.hasOwn(WALK_VIEW_POLICY_V1.closures, role)) throw refuseBuildV1("invalid", "closure.walk-views-required");
  return WALK_VIEW_POLICY_V1.closures[role] === "whole" ? undefined : WALK_VIEW_POLICY_V1.closures[role];
}

/** 📏️ The workspace-relative prefixes one view leaves out of a roster; a name the policy does not carry is refused. */
export function walkViewPrefixesV1(view) {
  if (!Object.hasOwn(WALK_VIEW_POLICY_V1.views, view)) throw refuseBuildV1("invalid", "closure.walk-views-required");
  return PREFIX_SETS_V1[WALK_VIEW_POLICY_V1.views[view].prefixes]();
}

/** 🔏️ The answer digest of one walk read through a view: the whole walk (roster, compiled modules, generated outputs, the script provider ports it names) when no view is named, otherwise the view's name with the roster and the compiled modules outside its prefixes, the generated outputs of every compiled module (a left-out module that imports a generated output still makes the target depend on its producer) and the ports (a project names each provider by reference, `../🔌️provider-closures`). */
export function walkViewAnswerV1(value, compiled, generated, view, ports = []) {
  if (view === undefined) return createHash("sha256").update(JSON.stringify([value, compiled, generated, ports])).digest("hex");
  const prefixes = walkViewPrefixesV1(view), kept = name => typeof name !== "string" || !prefixes.some(prefix => (name.startsWith("{workspaceRoot}/") ? name.slice(16) : name).startsWith(prefix));
  return createHash("sha256").update(JSON.stringify([view, value.filter(kept), compiled.filter(kept), generated.filter(([name, rows]) => rows.length || kept(name)), ports])).digest("hex");
}
