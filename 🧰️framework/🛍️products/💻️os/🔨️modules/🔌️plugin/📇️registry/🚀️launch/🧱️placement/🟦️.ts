/** 🧱️ Admits launcher containers under the owned source placement schema. */
import schema from "../🧬️schema/🧱️placement/🔣️.json";

type Row = Record<string, unknown>;
const isRow = (value: unknown): value is Row => typeof value === "object" && value !== null && !Array.isArray(value);
const isText = (value: unknown): value is string => typeof value === "string";
const isName = (value: unknown): value is string => isText(value) && value.length > 0;
/** 🛑️ Reports the exact container that refused source admission. */
function reject(path: string): never { throw new Error(`invalid launch seed at ${path}`); }
const inputSchema = schema.properties.inputs.items;
const inputKeys = new Set(Object.keys(inputSchema.properties));
const inputTypes = new Set<string>(inputSchema.properties.type.enum);
const requestTypes = new Set<string>(schema.properties.configurations.items.oneOf[0]!.properties!.request.enum!);
const placeholder = new RegExp(schema.properties.configurations.items.oneOf[1]!.pattern!);

/** 🚦️ Rejects executable rows in input containers before the source producer renders anything. */
export function assertLaunchSeedPlacement(document: unknown): void {
  if (!isRow(document)) reject("$");
  const configurations = document.configurations;
  if (!Array.isArray(configurations) || configurations.length > schema.properties.configurations.maxItems) reject("$.configurations");
  configurations.forEach((row, index) => {
    const path = `$.configurations[${index}]`;
    if (isText(row)) { if (!placeholder.test(row)) reject(path); return; }
    if (!isRow(row) || !isName(row.name) || !isName(row.type) || inputTypes.has(row.type) || !isText(row.request) || !requestTypes.has(row.request)) reject(path);
  });
  if (!Object.hasOwn(document, "inputs")) return;
  const inputs = document.inputs;
  if (!Array.isArray(inputs) || inputs.length > schema.properties.inputs.maxItems) reject("$.inputs");
  inputs.forEach((row, index) => {
    const path = `$.inputs[${index}]`;
    if (!isRow(row) || Object.keys(row).some(key => !inputKeys.has(key)) || !isName(row.id) || !isText(row.type) || !inputTypes.has(row.type)) reject(path);
    for (const key of ["description", "default"]) if (Object.hasOwn(row, key) && !isText(row[key])) reject(`${path}.${key}`);
    if (Object.hasOwn(row, "password") && typeof row.password !== "boolean") reject(`${path}.password`);
    if (Object.hasOwn(row, "command") && !isName(row.command)) reject(`${path}.command`);
    if (row.type === "command" && !isName(row.command)) reject(`${path}.command`);
    if (row.type === "pickString" && !Array.isArray(row.options)) reject(`${path}.options`);
    if (Object.hasOwn(row, "options")) {
      if (!Array.isArray(row.options)) reject(`${path}.options`);
      row.options.forEach((option, optionIndex) => {
        if (isText(option)) return;
        if (!isRow(option) || Object.keys(option).some(key => key !== "label" && key !== "value") || !isText(option.label) || !isText(option.value)) reject(`${path}.options[${optionIndex}]`);
      });
    }
  });
}
