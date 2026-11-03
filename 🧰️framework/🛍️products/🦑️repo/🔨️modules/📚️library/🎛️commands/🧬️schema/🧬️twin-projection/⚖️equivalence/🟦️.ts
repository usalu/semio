/** ⚖️ When two renderings of one twin say the same thing: byte equality, or equality after the normalisations that never change meaning (declaration order, import order, unused imports, doc comments, whitespace, redundant parentheses).
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
import type { Facet } from "../🧩️model/🟦️.ts";

/** 🚦️ `identical` is byte equality, `style` differs only by layout, `drift` differs in content. */
export type Equivalence = "identical" | "style" | "drift";

const collapse = (text: string): string => text.replace(/\s+/gu, " ").replace(/ ?([{};,:()=]) ?/gu, "$1").trim();

const sorted = (parts: readonly string[]): string[] => [...parts].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));

function typescript(text: string): string {
  const code = text.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/^\s*\/\/.*$/gmu, "");
  const body = code.replace(/^import .*$/gmu, "");
  const imports = [...code.matchAll(/^import \{ (.*) \} from ("[^"]+");$/gmu)].map((match) => `import{${sorted(match[1]!.split(", ").filter((name) => new RegExp(`\\b${name.replace("type ", "")}\\b`, "u").test(body))).join(",")}}from ${match[2]}`);
  const statements = body.split(/\n(?=export )/u).map((statement) => collapse(statement.replace(/\(([A-Za-z0-9_[\]]+)\) \| null/gu, "$1 | null"))).filter((statement) => statement !== "");
  return JSON.stringify([sorted(imports), sorted(statements)]);
}

function blocks(text: string, comment: RegExp): string {
  const code = text.replace(comment, "");
  const items = code.split(/\n(?=(?:message|enum|type|input|union|scalar|interface) )/u).map((item) => collapse(item)).filter((item) => item !== "");
  return JSON.stringify(sorted(items));
}

/** 🦀️ A Rust source with every run of consecutive attribute lines put in a fixed order (the derive first), since the order of helper attributes never changes meaning. */
function rust(text: string): string {
  const lines = text.split("\n");
  const result: string[] = [];
  for (let at = 0; at < lines.length; ) {
    if (!lines[at]!.trimStart().startsWith("#[")) {
      result.push(lines[at]!);
      at += 1;
      continue;
    }
    const run: string[] = [];
    for (; at < lines.length && lines[at]!.trimStart().startsWith("#["); at += 1) run.push(lines[at]!.trim());
    result.push(...run.sort((left, right) => Number(right.startsWith("#[derive(")) - Number(left.startsWith("#[derive(")) || left.localeCompare(right, "en")));
  }
  return collapse(result.join("\n"));
}

/** 📏️ The layout-free reading of a twin, equal for every rendering that says the same thing. */
export function normalized(language: Facet, text: string): string {
  if (language === "json") return JSON.stringify(JSON.parse(text));
  if (language === "typescript") return typescript(text);
  if (language === "protobuf") return blocks(text, /^\s*\/\/.*$/gmu);
  if (language === "graphql") return blocks(text, /^\s*#.*$/gmu);
  return rust(text);
}

/** ⚖️ How a committed twin relates to the one the schema produces. */
export function compare(language: Facet, committed: string, generated: string): Equivalence {
  if (committed === generated) return "identical";
  return normalized(language, committed) === normalized(language, generated) ? "style" : "drift";
}

/** 🔎️ The first normalised statement that differs between a committed and a generated twin, for the drift report. */
export function difference(language: Facet, committed: string, generated: string): string {
  if (language === "json") return "the JSON Schema document differs in layout only";
  const parsed = (text: string): unknown[] => {
    const reading = normalized(language, text);
    return language === "rust" ? reading.split(/(?<=[};])(?=\S)/u) : (JSON.parse(reading) as unknown[]);
  };
  const left = parsed(committed);
  const right = parsed(generated);
  const flat = (parts: unknown[]): string[] => parts.flatMap((part) => (Array.isArray(part) ? (part as string[]) : [String(part)]));
  const a = new Set(flat(left));
  const b = new Set(flat(right));
  const only = (from: Set<string>, other: Set<string>): string => [...from].find((item) => !other.has(item)) ?? "";
  return `committed: ${only(a, b).slice(0, 300)} | generated: ${only(b, a).slice(0, 300)}`;
}
