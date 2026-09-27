/** 🧾️ Shared parity verdict evidence checked independently by JSON Schema. */
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it } from "vitest";
import { parityVerdict } from "../../⚖️parity/✅️verdict/🟦️.ts";
import type { ParityPlaygroundReport } from "../../⚖️parity/🏗️structure/🟦️.ts";
import fixture from "../../⚖️parity/🧫️fixtures/✅️verdict/🔣️.json";
import schema from "../../⚖️parity/🧬️schema/✅️verdict/🔣️.json";

describe("complete renderer parity evidence", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  for (const row of fixture.cases) {
    it(row.name, () => {
      expect(validate(row.report)).toBe(row.expected === "PASS");
      expect(parityVerdict(row.report as ParityPlaygroundReport)).toBe(row.expected);
    });
  }
});

