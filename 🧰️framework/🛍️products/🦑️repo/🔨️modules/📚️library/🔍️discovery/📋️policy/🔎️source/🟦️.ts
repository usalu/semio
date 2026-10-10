/** 🧪️Brace-spans of any free item guarded by a `cfg` expression containing `test`. */
/** 🧩️ What a `#[cfg(test)]` attribute's same-line remainder starts: a parameter, field or argument (`hook: T,`, `hook: T) -> R {`)
 * ends at a top-level `,` or an unmatched `)`/`]` and leaves its enclosing item in production; anything else is the item itself. */
export function interactivityCfgTestSameLineKind(item: string): "member" | "item" {
  let depth = 0;
  for (const char of item) {
    if (char === "(" || char === "[") depth++;
    else if (char === ")" || char === "]") {
      if (depth === 0) return "member";
      depth--;
    } else if (depth === 0 && char === ",") return "member";
    else if (depth === 0 && (char === "{" || char === ";")) return "item";
  }
  return "item";
}

/** 🧪️ The line spans of `#[cfg(test)]` items — an attribute that names `test` without negating it (`cfg(not(test))` is production) —
 * whether the item starts on the attribute's own line (`#[cfg(test)] hook: T,`, `#[cfg(test)] if armed {`) or on a later one. A
 * test-only parameter or field (`,`) keeps its enclosing production item. */
export function interactivityCfgTestItemSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { startLine: number; depth: number }[] = [];
  let pendingStart: number | undefined;
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const attribute = /#\[cfg\((?![^\]]*\bnot\s*\()[^\]]*\btest\b[^\]]*\)\]/.exec(codeOnly);
    if (attribute) pendingStart = i + 1;
    const sameLine = pendingStart === i + 1 && attribute ? codeOnly.slice(attribute.index + attribute[0].length) : "";
    if (sameLine.trim().length > 0 && !/^\s*#\[/.test(sameLine) && interactivityCfgTestSameLineKind(sameLine) === "member") pendingStart = undefined;
    const item = pendingStart === undefined ? "" : i + 1 > pendingStart ? codeOnly : sameLine;
    if (pendingStart !== undefined && item.trim().length > 0 && !/^\s*#\[/.test(item)) {
      const openCount = (item.match(/\{/g) ?? []).length;
      if (openCount > 0) {
        stack.push({ startLine: pendingStart, depth });
        pendingStart = undefined;
      } else if (/;\s*$/.test(item)) {
        spans.push({ name: "cfg(test)", startLine: pendingStart, endLine: i + 1 });
        pendingStart = undefined;
      } else if (/,\s*$/.test(item)) {
        pendingStart = undefined;
      }
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      spans.push({ name: "cfg(test)", startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

export type PolicyModSpan = { name: string; startLine: number; endLine: number };

/**
 * 🧹️Masks `"..."` string-literal contents and `'x'` char-literal contents (same length, so indices stay
 * aligned) — line-bounded (`\n` excluded from both classes) and the char-literal form requires exactly one
 * char/escape, so a Rust lifetime apostrophe (`&'static`, `'a`) never greedily pairs with an unrelated
 * quote elsewhere in the file (which would otherwise corrupt brace-counting across huge, unrelated spans).
 */
export function policyMaskLiterals(line: string): string {
  return line.replace(/"(?:[^"\\\n]|\\.)*"/g, (m) => `"${" ".repeat(Math.max(0, m.length - 2))}"`).replace(/'(?:\\.|[^'\\\n])'/g, (m) => `'${" ".repeat(Math.max(0, m.length - 2))}'`);
}

const POLICY_MOD_ANY_OPEN_RE = /^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$/;

/** 🧪️Brace-spans of `#[cfg(test)] mod … { … }` blocks — synthetic test fixtures (e.g. `App::builder` in a unit test) aren't real app registrations. */
export function policyTestModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number; isTest: boolean }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_ANY_OPEN_RE);
    if (modMatch) {
      const isTest = lines.slice(Math.max(0, i - 2), i).some((l) => /#\[cfg\([^\]]*\btest\b[^\]]*\)\]/.test(l)) || modMatch[1] === "tests";
      stack.push({ name: modMatch[1]!, startLine: i + 1, depth, isTest });
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      if (top.isTest) spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

/** 🏷️True when `lineNo` sits inside a `#[cfg(test)] mod …` / `mod tests` brace span. */
export function policyLineInTestMod(testSpans: readonly PolicyModSpan[], lineNo: number): boolean {
  return testSpans.some((s) => s.startLine <= lineNo && lineNo <= s.endLine);
}
