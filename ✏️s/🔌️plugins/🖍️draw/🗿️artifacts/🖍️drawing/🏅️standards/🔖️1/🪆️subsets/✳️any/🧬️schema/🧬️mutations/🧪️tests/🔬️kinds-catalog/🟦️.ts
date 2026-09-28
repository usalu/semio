/** 🧬️ Canonical mutation fixtures must agree with the public tagged wire schema. */
import { expect, it } from "bun:test";
import Ajv from "ajv";
import { fileURLToPath } from "node:url";
import aggregate from "../../🔣️.json";
import documentSchema from "../../../🔣️.json";
import fixture from "../../../../../🎨️style/🧬️schema/🧬️mutations/📝️update-text/🧫️fixtures/🔣️.json";

it("validates every canonical tagged mutation through the aggregate schema", async () => {
  const ajv = new Ajv({strict: false, validateFormats: false}).addSchema(documentSchema);
  const cwd = fileURLToPath(new URL("../../../../../", import.meta.url));
  for await (const path of new Bun.Glob("*/🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json").scan({cwd, absolute: true})) ajv.addSchema(await Bun.file(path).json());
  const validate = ajv.compile(aggregate);
  for (const edit of fixture.edits) expect(validate({mutation: "updateText", layerId: "text", ...edit})).toBe(true);
  for (const size of fixture.invalidSizes) expect(validate({mutation: "updateText", layerId: "text", content: "changed", size})).toBe(false);
  let count = 0;
  for await (const path of new Bun.Glob("*/🧫️fixtures/🧬️mutations/*/*/🦠️mutation/🔣️.json").scan({cwd, absolute: true})) {
    const mutation = await Bun.file(path).json();
    expect(validate(mutation), `${mutation.mutation}: ${ajv.errorsText(validate.errors)}`).toBe(true);
    expect(validate({...mutation, mutation: "unknown"})).toBe(false);
    expect(validate({...mutation, unexpected: true})).toBe(false);
    count += 1;
  }
  expect(count).toBe(aggregate.oneOf.length);
});

import diffSchema from "../../../🔺️diff/🔣️.json";
import fieldCases from "../../🧫️fixtures/🎛️field-patch/🔣️.json";
import blendSchema from "../../../../../🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🧬️schema/🔣️.json";
import {diff as blendDiff} from "../../../../../🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🔺️diff/🟦️.ts";
import {parseDrawingLayerNode} from "../../../🟦️.ts";
import {parseDrawingLayerPatch} from "../../../🔺️diff/🟦️.ts";

it("refuses invalid blend vocabulary at mutation, patch and nested document boundaries", () => {
  const ajv = new Ajv({strict:false,validateFormats:false});
  const validateMutation = ajv.compile(blendSchema);
  const validateDocument = ajv.compile(documentSchema);
  ajv.addSchema(diffSchema);
  const validatePatch = ajv.compile({$ref:`${diffSchema.$id}#/$defs/DrawingLayerPatch`});
  expect(validatePatch({blendMode:null})).toBe(true);
  expect(parseDrawingLayerPatch({blendMode:null}).blendMode).toBeUndefined();
  for (const {patch,accepted} of fieldCases.filter(({patch}) => patch.field === "blendMode")) {
    const mutation = {layerId:"shape",blendMode:patch.value};
    const leaf = {kind:"shape",blendMode:patch.value};
    const root = {kind:"group",children:[{kind:"group",children:[leaf]}]};
    const document = {schema:"drawing.document",id:"blend",assets:{},layers:[root]};
    expect(validateMutation(mutation)).toBe(accepted);
    expect(validatePatch({blendMode:patch.value})).toBe(accepted);
    expect(validateDocument(document)).toBe(accepted);
    if (accepted) {
      expect(parseDrawingLayerNode(root)).toEqual(root);
      expect(parseDrawingLayerPatch({blendMode:patch.value}).blendMode).toBe(patch.value);
      expect(blendDiff(mutation as never).layers.patched[0]!.patch.blendMode).toBe(patch.value);
    } else {
      expect(() => parseDrawingLayerNode(root)).toThrow();
      expect(() => parseDrawingLayerPatch({blendMode:patch.value})).toThrow();
      expect(() => blendDiff(mutation as never)).toThrow();
    }
  }
});
