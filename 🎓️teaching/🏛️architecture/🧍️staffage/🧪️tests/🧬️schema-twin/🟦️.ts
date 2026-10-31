/** 🧬️ Smoke case of the schema twin: the closed vocabularies the contract and the catalog layout agree on.
 * @see ../../🧬️schema/🟦️.ts */
import { describe, expect, it } from "vitest";
import { CATEGORIES, LANGUAGES, SCHEMA_VERSION } from "../../📦️packages/🟦️typescript/🟦️.ts";

describe("schema twin vocabularies", () => {
  it("names the catalog categories in contract order", () => {
    expect(CATEGORIES).toEqual(["humans", "animals", "vegetation", "furniture", "objects", "vehicles", "machinery", "components"]);
  });
  it("offers English first and German second without a default", () => {
    expect(LANGUAGES).toEqual(["en", "de"]);
  });
  it("versions the contract", () => {
    expect(SCHEMA_VERSION).toBe("semio.staffage/v2");
  });
});
