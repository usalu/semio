/** 🧪️ Dash input matches the language-neutral cases and independent Ajv grammar. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { parseStrokeCap, parseStrokeJoin, parseStrokeDash } from "../../🟦️.ts";
import enumCases from "../../🧫️fixtures/🎚️enums/🔣️.json";
import strokeSchema from "../../🧬️schema/🔣️.json";
import artifactSchema from "../../../🔣️.json";
import cases from "../../🧫️fixtures/🔣️.json";

test("stroke dashes accept editable patterns and reject invalid input", () => {
  
  for (const entry of cases) {
    
    if (entry.error) expect(() => parseStrokeDash(entry.value)).toThrow();
    else expect(parseStrokeDash(entry.value)).toEqual(entry.dash);
  }
});

test("stroke caps and joins match the schema and reject unsupported values", () => {
  const validate = new Ajv({strict:true}).compile(strokeSchema);
  for (const entry of enumCases) {
    const stroke = {color:[0,0,0,1],width:1,cap:"butt",join:"miter",[entry.kind]:entry.value};
    expect(validate(stroke)).toBe(entry.valid);
    const parse = entry.kind === "cap" ? parseStrokeCap : parseStrokeJoin;
    if (entry.valid) expect(parse(entry.value)).toBe(entry.value);
    else expect(() => parse(entry.value)).toThrow();
  }
});

test("artifact schema validates stroke enums inside groups and preserves boolean references", () => {
  const validate = new Ajv({strict:false,validateFormats:false}).compile(artifactSchema);
  for (const entry of enumCases) {
    const stroke = {color:[0,0,0,1],width:1,cap:"butt",join:"miter",[entry.kind]:entry.value};
    const path = {kind:"path",id:"outline",attributes:{stroke}};
    const layers = [{kind:"group",children:[path]},{kind:"boolean",children:["outline"]}];
    expect(validate({schema:"drawing.document",id:"stroke-fixture",assets:{},layers})).toBe(entry.valid);
  }
});
