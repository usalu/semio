import { expect, test } from "bun:test";
import { runTwins } from "../../🟦️.ts";
import { repoRootFromHere } from "../../../../../../🧪️test/🧭️case-discovery/🟦️.ts";

/**
 * 🧮️ Every committed schema twin (TypeScript, Rust, protobuf, GraphQL) of every twin-projecting family equals, byte for byte, what the generator produces from its JSON Schema.
 *
 * This is `schema twins check --under . --strict` as a suite member: a schema edited without `repo ./📜️script.ts schema twins generate`, or a hand-edited generated twin, fails here instead of drifting silently. Families that declare no `🧬️twin-dialect` (or `projection: off`) are not projected and stay outside this gate.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts
 */
test("every committed schema twin equals its generated twin", () => {
  const report = runTwins(repoRootFromHere(), ["check", "--under", ".", "--strict"]);
  expect(report.problems).toEqual([]);
  expect(report.drifted).toEqual([]);
  expect(report.styled).toEqual([]);
}, 280_000);
