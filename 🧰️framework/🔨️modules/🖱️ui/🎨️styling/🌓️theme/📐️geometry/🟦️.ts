import contract from "./🧬️contract/🔣️.json";

type Quantity = { value: number; length: boolean };
type Token = { text: string; start: number; end: number };
const rules = contract["x-semio-resolution"];
const whitespace = /[\t\n\r\f ]/;
const number = new RegExp(`^(?:${rules.numberPattern})`);

/** 📏️ Resolves bounded CSS length arithmetic against the authored theme root. */
export function resolveThemeSpacingPx(compact: string, rootRemPx: number): number {
  if (compact.length > contract.properties.compact.maxLength || !Number.isFinite(rootRemPx) || rootRemPx <= 0) throw new Error("Invalid compact spacing/root size");
  const tokens: Token[] = [];
  let offset = 0;
  while (offset < compact.length) {
    if (whitespace.test(compact[offset]!)) { offset++; continue; }
    const start = offset;
    const numeric = number.exec(compact.slice(offset));
    if (numeric) {
      offset += numeric[0].length;
      while (/[a-z]/i.test(compact[offset] ?? "")) offset++;
    } else if (/[a-z]/i.test(compact[offset]!)) {
      while (/[a-z]/i.test(compact[offset] ?? "")) offset++;
    } else if ("()+-*/,".includes(compact[offset]!)) offset++;
    else throw new Error("Unsupported CSS length token");
    tokens.push({ text: compact.slice(start, offset).toLowerCase(), start, end: offset });
    if (tokens.length > rules.maxTokens) throw new Error("CSS length token limit");
  }
  let cursor = 0;
  const peek = () => tokens[cursor]?.text;
  const take = (text: string) => {
    if (peek() !== text) throw new Error(`Expected ${text}`);
    cursor++;
  };
  const quantity = (value: number, length: boolean): Quantity => {
    if (!Number.isFinite(value)) throw new Error("Nonfinite CSS length");
    return { value, length };
  };
  const expression = (depth: number): Quantity => {
    let left = product(depth);
    while (peek() === "+" || peek() === "-") {
      const operator = tokens[cursor++]!;
      if (!whitespace.test(compact[operator.start - 1] ?? "") || !whitespace.test(compact[operator.end] ?? "")) throw new Error("CSS addition requires surrounding whitespace");
      const right = product(depth);
      if (left.length !== right.length) throw new Error("CSS addition dimensions differ");
      left = quantity(left.value + (operator.text === "+" ? right.value : -right.value), left.length);
    }
    return left;
  };
  const product = (depth: number): Quantity => {
    let left = atom(depth);
    while (peek() === "*" || peek() === "/") {
      const operator = tokens[cursor++]!.text;
      const right = atom(depth);
      if (operator === "*") {
        if (left.length && right.length) throw new Error("CSS product dimensions unsupported");
        left = quantity(left.value * right.value, left.length || right.length);
      } else {
        if (right.value === 0 || (!left.length && right.length)) throw new Error("CSS division dimensions/zero invalid");
        left = quantity(left.value / right.value, left.length && !right.length);
      }
    }
    return left;
  };
  const atom = (depth: number): Quantity => {
    if (depth > rules.maxDepth) throw new Error("CSS length nesting limit");
    const token = tokens[cursor++];
    if (!token) throw new Error("Missing CSS length operand");
    if (token.text === "+" || token.text === "-") {
      const next = tokens[cursor];
      if (!next || token.end !== next.start || !number.test(next.text)) throw new Error("A CSS numeric sign must directly prefix its number");
      const operand = atom(depth + 1);
      return quantity((token.text === "-" ? -1 : 1) * operand.value, operand.length);
    }
    if (token.text === "(") {
      const nested = expression(depth + 1);
      take(")");
      return nested;
    }
    const numeric = number.exec(token.text);
    if (numeric) {
      const unit = token.text.slice(numeric[0].length);
      const multiplier = unit === "rem" ? rootRemPx : rules.absoluteUnits[unit as keyof typeof rules.absoluteUnits];
      if (unit && multiplier === undefined) throw new Error("Context-dependent CSS length unit");
      return quantity(Number(numeric[0]) * (multiplier ?? 1), unit !== "");
    }
    if (!rules.functions.includes(token.text)) throw new Error("Unsupported CSS length function");
    if (rules.functionNameAdjacency && token.end !== tokens[cursor]?.start) throw new Error("CSS function name must touch its opening parenthesis");
    take("(");
    const arguments_: Quantity[] = [expression(depth + 1)];
    while (peek() === ",") {
      cursor++;
      arguments_.push(expression(depth + 1));
      if (arguments_.length > rules.maxArguments) throw new Error("CSS function argument limit");
    }
    take(")");
    if (arguments_.some(item => item.length !== arguments_[0]!.length)) throw new Error("CSS function dimensions differ");
    const values = arguments_.map(item => item.value);
    const value = token.text === "calc" && values.length === 1 ? values[0]!
      : token.text === "min" ? Math.min(...values)
      : token.text === "max" ? Math.max(...values)
      : token.text === "clamp" && values.length === 3 ? Math.max(values[0]!, Math.min(values[1]!, values[2]!)) : NaN;
    return quantity(value, arguments_[0]!.length);
  };
  if (peek() === "(" || ((peek() === "+" || peek() === "-") && tokens[0]!.end !== tokens[1]?.start)) throw new Error("Invalid outer CSS length");
  const result = atom(0);
  if (cursor !== tokens.length || result.value < 0 || result.value > rules.maxMagnitudePx || (!result.length && result.value !== 0)) throw new Error("Compact spacing must resolve to a nonnegative length");
  return result.value;
}

/** 🏷️ The localized authoring guidance from the shared spacing contract. */
export function themeCompactSpacingUi(locale: "en" | "de"): { label: string; description: string } {
  const ui = contract.properties.compact["x-semio-ui"];
  return { label: ui.label[locale], description: ui.description[locale] };
}
