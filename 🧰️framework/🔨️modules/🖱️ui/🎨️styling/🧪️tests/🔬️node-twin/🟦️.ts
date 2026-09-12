/** 🪞️ Node twin for `🧫️fixtures/🔁️animation-scope/🔣️.json` — the SAME cases with no test framework, so
 * `analyzeCssAnimationScope`/`cssAnimationScopeViolations` are checked by a second runner that shares
 * nothing with the bun suite but the fixture file. Run with `bun ./📜️script.ts twin`. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { analyzeCssAnimationScope, cssAnimationScopeUnclockedPaints, cssAnimationScopeViolations } from "../../📦️packages/🟦️typescript/🟦️.ts";

const fixturePath = resolve(import.meta.dir, "../../🧫️fixtures/🔁️animation-scope/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as {
  cases: readonly { name: string; css: string; expect: Record<string, unknown> }[];
  stylesheet: { path: string; keyframesByProperty: Record<string, readonly string[]>; violations: readonly string[] };
};

let failures = 0;
const check = (label: string, actual: unknown, expected: unknown): void => {
  const left = JSON.stringify(actual);
  const right = JSON.stringify(expected);
  if (left === right) return;
  failures += 1;
  console.error(`FAIL ${label}\n  actual   ${left}\n  expected ${right}`);
};

for (const testCase of fixture.cases) {
  const scope = analyzeCssAnimationScope(testCase.css);
  check(`${testCase.name} · animatedCustomProperties`, scope.animatedCustomProperties, testCase.expect.animatedCustomProperties);
  check(`${testCase.name} · keyframesByProperty`, scope.keyframesByProperty, testCase.expect.keyframesByProperty);
  check(`${testCase.name} · propertyInheritance`, scope.propertyInheritance, testCase.expect.propertyInheritance);
  check(`${testCase.name} · rootClockSelectors`, scope.rootClocks.map((clock) => clock.selector), testCase.expect.rootClockSelectors);
  check(`${testCase.name} · clocks`, scope.clocks, testCase.expect.clocks);
  check(`${testCase.name} · paints`, scope.paints, testCase.expect.paints);
  check(`${testCase.name} · violations`, cssAnimationScopeViolations(scope), testCase.expect.violations);
}

const repoRoot = resolve(import.meta.dir, "../../../../../..");
const sheetScope = analyzeCssAnimationScope(readFileSync(resolve(repoRoot, fixture.stylesheet.path), "utf8"));
check("🖌️ui.css · keyframesByProperty", sheetScope.keyframesByProperty, fixture.stylesheet.keyframesByProperty);
check("🖌️ui.css · violations", cssAnimationScopeViolations(sheetScope), fixture.stylesheet.violations);
check("🖌️ui.css · unclocked paints", cssAnimationScopeUnclockedPaints(sheetScope).map((paint) => `${paint.selector.replace(/\s+/g, " ")} :: ${paint.property}`), []);

console.log(`[twin] 🔁️animation-scope: ${fixture.cases.length} fixture cases + 🖌️ui.css, ${failures} failure(s)`);
if (failures > 0) process.exit(1);
