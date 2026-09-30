/** 🔌️ Concrete catalogue handle spelling laws share the implementation-owned fixture with Rust. */
import { describe, it, expect } from "bun:test";
import portSidesFixture from "../../🧫️fixtures/🔌️port-sides/🔣️.json";
//#region 🔌️PortSideTwin
/** 🔌️ The renderer half of the catalogue port-side law. A wire endpoint has exactly ONE public name
 * here — `${nodeId}@${portId}`, the string `🕸️NodeGraph/🟦️.tsx` builds for every hit target — so an
 * operator that declares one port id on both sides gives two handles one key and a press on the
 * output resolves to the input. The Rust half
 * (`✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs`, `no_operator_in_the_catalogue_declares_one_port_id_on_both_sides`)
 * drives the LIVE first-party catalogue; this half drives the same fixture rows through the
 * renderer's own handle spelling. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
describe("🔌️ operator port sides", () => {
  const fixture = portSidesFixture as unknown as {
    suffix: string;
    wildcardMarker: string;
    correctedInputs: { operator: string; was: string; now: string }[];
    rows: { operator: string; inputs: string[]; outputs: string[] }[];
  };

  it("gives every port of an operator its own {nodeId}@{portId} handle", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(12);
    for (const row of fixture.rows) {
      const ports = [...row.inputs, ...row.outputs].filter((port) => port !== fixture.wildcardMarker);
      const handles = ports.map((port) => `${row.operator}@${port}`);
      expect([row.operator, new Set(handles).size]).toEqual([row.operator, handles.length]);
    }
  });

  it("spells a produced channel with the suffix only when its own operator is given the same noun", () => {
    for (const row of fixture.rows) {
      for (const output of row.outputs) {
        if (!output.endsWith(fixture.suffix)) continue;
        const plain = output.slice(0, -fixture.suffix.length);
        expect([row.operator, output, row.inputs.includes(plain)]).toEqual([row.operator, output, true]);
      }
      for (const input of row.inputs) {
        if (input === fixture.wildcardMarker) continue;
        expect([row.operator, input, row.outputs.includes(input)]).toEqual([row.operator, input, false]);
      }
    }
  });

  it("keeps the wildcard marker and the corrected inputs the fixture names", () => {
    expect(fixture.wildcardMarker).toBe("*");
    const variable = fixture.rows.find((row) => row.operator === "core.variable")!;
    expect(variable.inputs).toEqual([fixture.wildcardMarker]);
    expect(variable.outputs).toEqual([fixture.wildcardMarker]);
    const move = fixture.rows.find((row) => row.operator === "math.move")!;
    for (const corrected of fixture.correctedInputs.filter((entry) => entry.operator === "math.move")) {
      expect(move.inputs).toContain(corrected.now);
      expect(move.inputs).not.toContain(corrected.was);
    }
  });
});
//#endregion 🔌️PortSideTwin
