import { refuseBuildV1 } from "../../../../📖️texts/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import table from "./🔣️.json" with { type: "json" };

if (validateJsonSchemaSubset(schema, table).length) throw refuseBuildV1("invalid", "closure.provider-closures-required");

/** 🔌️ The admitted shared closure policy: group names, the views a provider closure is published in, the declared tool commands and how the providers project is recognized. See `./🔣️.json`, `../👓️walk-views` (the views) and `./🧪️tests/🟦️.ts` (the law). */
export const PROVIDER_CLOSURE_POLICY_V1 = Object.freeze(structuredClone(table));

const PORT_V1 = /^[a-z0-9-]+$/u, refused = () => refuseBuildV1("invalid", "closure.provider-closures-required");

/** 🏷️ The named input of the providers project that holds the closure of `port` read through `view`; an unknown view or a port the designation grammar does not admit is refused. */
export function providerGroupNameV1(port, view) {
  if (typeof port !== "string" || !PORT_V1.test(port) || !Object.hasOwn(table.views, view)) throw refused();
  return `${table.group.prefix}${port}${table.group.separator}${view}`;
}

/** 🧰️ The named input of the providers project that holds the closure of the declared tool command `name`, read through the view the policy declares for it; an undeclared tool is refused. */
export function toolGroupNameV1(name) {
  if (!Object.hasOwn(table.tools.closures, name)) throw refuseBuildV1("invalid", "closure.tool-unknown", { name: String(name) });
  return `${table.tools.prefix}${name}${table.group.separator}${table.tools.closures[name].view}`;
}

/** 🔍️ The recipe of the shared group a name stands for (`script-provider` with port and view, `tool-closure` with name and view), or undefined for any other name. */
export function sharedGroupOfV1(name) {
  if (typeof name !== "string") return undefined;
  if (name.startsWith(table.group.prefix)) {
    const [port, view, ...rest] = name.slice(table.group.prefix.length).split(table.group.separator);
    return rest.length === 0 && PORT_V1.test(port ?? "") && Object.hasOwn(table.views, view) ? { fn: "script-provider", port, view } : undefined;
  }
  if (name.startsWith(table.tools.prefix)) {
    const [tool, view, ...rest] = name.slice(table.tools.prefix.length).split(table.group.separator);
    return rest.length === 0 && Object.hasOwn(table.tools.closures, tool) && table.tools.closures[tool].view === view ? { fn: "tool-closure", name: tool, view } : undefined;
  }
  return undefined;
}

/** 🔗️ The reference a command closure holds for one provider closure: the provider group of the providers project, as a build named input reference. */
export function providerReferenceV1(project, port, view) {
  return { input: providerGroupNameV1(port, view), projects: [project] };
}

/** 🕸️ The reference a command closure holds for one declared tool command. */
export function toolReferenceV1(project, name) {
  return { input: toolGroupNameV1(name), projects: [project] };
}

/** 🧲️ Reads one closure item as a shared reference (`{input, projects:[project]}` naming a shared group): the project and the recipe of the group, or undefined for any other item. */
export function sharedReferenceOfV1(item) {
  if (item === null || typeof item !== "object" || Array.isArray(item) || Object.keys(item).length !== 2 || !Array.isArray(item.projects) || item.projects.length !== 1 || typeof item.projects[0] !== "string") return undefined;
  const recipe = sharedGroupOfV1(item.input);
  return recipe && { project: item.projects[0], recipe };
}

/** 📋️ The shared references of a closure value: a list, or a target-sources value whose `closure` is the list. */
export function sharedReferencesOfV1(value) {
  return (Array.isArray(value) ? value : Array.isArray(value?.closure) ? value.closure : []).flatMap(item => { const reference = sharedReferenceOfV1(item); return reference ? [reference] : []; });
}

/** 🔀️ Joins command input lists, every shared reference once after the other items, provider references read through `view` (a Rust command joins the target's own closure, read through `scheduling`, with the native command inputs). */
export function joinWithProviderViewV1(lists, view) {
  const items = [], references = new Map();
  for (const list of lists) for (const item of list) {
    const reference = sharedReferenceOfV1(item);
    if (!reference) { items.push(item); continue; }
    const row = reference.recipe.fn === "script-provider" ? providerReferenceV1(reference.project, reference.recipe.port, view) : item;
    references.set(JSON.stringify(row), row);
  }
  return [...new Set(items), ...[...references.keys()].sort().map(key => references.get(key))];
}

/** 🧩️ A closure value whose shared references are replaced by the files of the shared closures they name, as the queries over a command closure (`some`, `excludedPaths`, `workspaceKeys`) see it. */
export function expandClosureValueV1(value, files) {
  const expand = list => [...new Set([...list.filter(item => !sharedReferenceOfV1(item)), ...files])];
  return Array.isArray(value) ? expand(value) : value !== null && typeof value === "object" && Array.isArray(value.closure) ? { ...value, closure: expand(value.closure) } : value;
}

/** 🪪️ The name of the providers project from its configuration: it must declare the designation by its file name under the policy's metadata key; anything else is refused. */
export function providerProjectNameV1(json, designation) {
  if (json === null || typeof json !== "object" || typeof json.name !== "string" || !json.name || json.metadata?.semio?.[table.project.metadata] !== designation) throw refuseBuildV1("invalid", "closure.provider-project-required", { designation: String(designation) });
  return json.name;
}
