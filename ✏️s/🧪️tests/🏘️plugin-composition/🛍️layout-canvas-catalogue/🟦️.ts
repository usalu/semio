import { beforeEach, afterEach, describe, expect, it, vi } from "vitest";
import { cleanup } from "@semio-tech/ui-react/test";
import { compileJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import catalogueTransportSchema from "../../../🔌️plugins/📏️layout/🧬️schema/🛍️canvas-catalogue/🔣️.json" with { type: "json" };
import transport from "../../../🔌️plugins/📏️layout/🧪️tests/🧫️fixtures/🛍️canvas-catalogue/transport.json" with { type: "json" };
import layoutCatalogue from "../../../🔌️plugins/📏️layout/🧪️tests/🧫️fixtures/🛍️canvas-catalogue/🔣️.json" with { type: "json" };
import { mountedHost, canvasInputSessions, settle } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🔮️oracles/🖱️mounted-input/🟦️.tsx";
import fixture from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🧫️fixtures/🖱️input-contract/🔣️.json" with { type: "json" };

const validateTransport = semioSchemaAjvV1({ allErrors: true }).compile(catalogueTransportSchema);
const ownedTransport = compileJsonSchemaSubset(catalogueTransportSchema);

beforeEach(() => { canvasInputSessions().length = 0; vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} }); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

describe("Layout catalogue host integration", () => {
  it.each(transport.cases)("validates the language-neutral transport $id contract", row => {
    expect(ownedTransport.validate(row.envelope).length === 0).toBe(row.valid);
    expect(validateTransport(row.envelope), JSON.stringify(validateTransport.errors)).toBe(row.valid);
  });

  it("validates actual Layout catalogue envelopes with the independent JSON Schema oracle", () => {
    for (const row of layoutCatalogue.cases.filter(row => row.kind !== null)) {
      const envelope = { action: row.action, args: row.args };
      expect(ownedTransport.validate(envelope)).toEqual([]);
      expect(validateTransport(envelope), JSON.stringify(validateTransport.errors)).toBe(true);
    }
    expect(new Set(layoutCatalogue.cases.map(row => row.id)).size).toBe(layoutCatalogue.cases.length);
    expect(layoutCatalogue.cases.some(row => row.action === "canvasDragOver")).toBe(true);
    expect(layoutCatalogue.cases.some(row => row.action === "canvasDrop")).toBe(true);
  });

  it.each(layoutCatalogue.cases.filter(row => row.kind !== null))("forwards the actual Layout catalogue $id envelope", async row => {
    const { actions, host } = mountedHost({ ...fixture.surface, id: row.args.surfaceId, controllerId: "layout-play" });
    const args = row.args as typeof row.args & { types?: string[]; dragData?: string };
    const dropping = row.action === "canvasDrop";
    const event = new MouseEvent(dropping ? "drop" : "dragover", { bubbles: true, cancelable: true, clientX: args.x, clientY: args.y });
    Object.defineProperty(event, "dataTransfer", { value: {
      types: args.types ?? [layoutCatalogue.mime, `${layoutCatalogue.kindMimePrefix}${row.kind}`],
      getData: (mime: string) => {
        if (!dropping) throw new Error("drag-over cannot read protected source data");
        return mime === layoutCatalogue.mime ? args.dragData! : "";
      },
    } });
    host.dispatchEvent(event);
    await settle();
    expect(actions).toEqual(dropping
      ? [
          { controllerId: "layout-play", action: "canvasDragLeave", args: { surfaceId: row.args.surfaceId } },
          { controllerId: "layout-play", action: row.action, args: row.args },
        ]
      : [{ controllerId: "layout-play", action: row.action, args: row.args }]);
    for (const action of actions.filter(action => action.action !== "canvasDragLeave")) {
      const envelope = { action: action.action, args: action.args };
      expect(ownedTransport.validate(envelope)).toEqual([]);
      expect(validateTransport(envelope), JSON.stringify(validateTransport.errors)).toBe(true);
    }
    if (dropping) expect(JSON.parse(args.dragData!)).toEqual({ kind: row.kind });
  });

});
