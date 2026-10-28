import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { JSDOM } from "jsdom";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { Field } from "../🟦️.tsx";
import cases from "./cases.json";
import schema from "./cases.schema.json";
import { fieldAssociation } from "./🟦️.ts";

test("closed language neutral field association vectors", () => {
  const ajv = new Ajv({ strict: true });
  expect(ajv.validate(schema, cases)).toBe(true);
  expect(ajv.validate(schema, { ...cases, invented: true })).toBe(false);
});

for (const row of cases.cases) test("field association: " + row.id, () => {
  const output = fieldAssociation(row.row, row.control, row.description, row.error);
  expect(output).toEqual(row.expected);
  const dom = new JSDOM("<!doctype html><body></body>");
  const doc = dom.window.document;
  const label = doc.createElement("label");
  const input = doc.createElement("input");
  if (row.expected.target) { label.htmlFor = row.expected.target; input.id = row.expected.target; }
  doc.body.append(label, input);
  expect(output.target).toBe(label.control?.id);
  for (const id of [row.expected.description, row.expected.error].filter(Boolean)) { const paragraph = doc.createElement("p"); paragraph.id = id!; doc.body.append(paragraph); }
  expect((output.describedBy?.split(" ") ?? []).map(id => doc.getElementById(id)?.id)).toEqual([row.expected.description, row.expected.error].filter(Boolean));
  dom.window.close();
});

test("actual Field markup exposes the explicit target, messages and native HTML presentation", () => {
  const id = "window/stage.field.7";
  const target = `${id}/stage.control.7`;
  const markup = renderToStaticMarkup(createElement(Field, { id, controlId: target, label: "Wert / Value", description: "Exakter Wert / Exact value", required: true, error: "Unvollständig / Incomplete", style: { width: "100%" }, children: createElement("input", { id: target, "aria-describedby": `${id}.description ${id}.error`, "aria-invalid": true, "aria-errormessage": `${id}.error` }) }));
  const dom = new JSDOM(markup);
  const doc = dom.window.document;
  const label = doc.querySelector("label")!;
  const input = doc.querySelector("input")!;
  expect(label.control).toBe(input);
  expect(label.id).toBe(`${id}.label`);
  expect(label.querySelector("span")?.getAttribute("aria-hidden")).toBe("true");
  expect(input.getAttribute("aria-describedby")?.split(" ").map(value => doc.getElementById(value)?.textContent)).toEqual(["Exakter Wert / Exact value", "Unvollständig / Incomplete"]);
  expect(doc.getElementById(`${id}.error`)?.getAttribute("role")).toBe("alert");
  expect(doc.getElementById(id)?.style.width).toBe("100%");
  dom.window.close();
});

test("actual read-only Field markup uses a label span with no fabricated input target", () => {
  const markup = renderToStaticMarkup(createElement(Field, { id: "window/preview", controlId: null, label: "Datentyp / Data type", children: "IfcLengthMeasure" }));
  const dom = new JSDOM(markup);
  const label = dom.window.document.querySelector('[data-slot="field-label"]')!;
  expect(label.tagName).toBe("SPAN");
  expect(label.hasAttribute("for")).toBe(false);
  expect(dom.window.document.querySelector("label")).toBeNull();
  dom.window.close();
});
