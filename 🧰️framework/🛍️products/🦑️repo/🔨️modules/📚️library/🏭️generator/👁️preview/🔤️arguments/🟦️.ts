import policy from "./🧬️schema/🔣️.json";
import literalPolicy from "./🧬️schema/🔣️literal-command.json";

/** 🔤️ Admits bounded literal argv without interpreting argument data as shell syntax. */
export function parseGeneratorPreviewArgumentsV1(value: unknown): readonly string[] {
  if (!Array.isArray(value) || value.length < policy.minItems || value.length > policy.maxItems || value.some(arg => typeof arg !== "string" || arg.length > policy.items.maxLength || arg !== arg.normalize("NFC") || !new RegExp(policy.items.pattern, "u").test(arg)) || new TextEncoder().encode(JSON.stringify(value)).byteLength > policy["x-semio-admission"].maxBytes) throw new Error("Invalid bounded literal preview arguments");
  return Object.freeze([...value]);
}

/** 🛡️ Reads fixed owner command metadata with finite cross-platform literal tokens and no expansions. */
export function parseGeneratorPreviewLiteralCommandV1(value: unknown): readonly string[] {
  const refuse = (): never => { throw new Error("Invalid literal owner preview command"); };
  if (typeof value !== "string" || !value || value.length > literalPolicy.maxLength || value !== value.normalize("NFC") || !new RegExp(literalPolicy.pattern, "u").test(value)) return refuse();
  const result: string[] = [];
  let cursor = 0;
  while (cursor < value.length) {
    while (value[cursor] === " ") cursor++;
    if (cursor === value.length) break;
    let token = "";
    if (value[cursor] === '"') {
      cursor++;
      while (cursor < value.length && value[cursor] !== '"') token += value[cursor++];
      if (value[cursor++] !== '"' || (cursor < value.length && value[cursor] !== " ")) return refuse();
    } else {
      while (cursor < value.length && value[cursor] !== " ") { if (value[cursor] === '"') return refuse(); token += value[cursor++]; }
    }
    if (token.length > literalPolicy["x-semio-admission"].maxTokenCharacters || result.length >= literalPolicy["x-semio-admission"].maxTokens) return refuse();
    result.push(token);
  }
  if (!result.length) return refuse();
  return Object.freeze(result);
}
