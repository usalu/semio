/** 🎬️ The WGPU media reservation consumes the canonical host contract before projecting status. */
import { describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/🎬️media-transport-reservation/🔣️.json";
import { parseMediaTransportProps } from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️";
import contract from "../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🧬️contract/🔣️.json";

describe("🎬️ media transport reservation", () => {
  test("Ajv agrees with every canonical valid and invalid transport vector", () => {
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(contract);
    for (const vector of fixture.cases) expect(validate(vector.props), `${vector.name}: ${JSON.stringify(validate.errors)}`).toBe(vector.schemaValid);
  });

  test("the parser also enforces resource revision authority", () => {
    for (const vector of fixture.cases) {
      const valid = "contractValid" in vector ? vector.contractValid : vector.schemaValid;
      if (valid) expect(() => parseMediaTransportProps(vector.props)).not.toThrow();
      else expect(() => parseMediaTransportProps(vector.props)).toThrow();
    }
  });

  test("only the exact versioned host address is reserved", () => {
    expect(fixture.reservedExtensionId).toBe("framework.media.transport@1");
    const generic = fixture.cases.find((vector) => vector.name === "generic-extension-is-unchanged");
    expect(generic?.extensionId).not.toBe(fixture.reservedExtensionId);
    expect(generic?.expectedHostStatus).toBeNull();
  });
});
