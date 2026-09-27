/** 🧪️ SVG export must bypass the reduced semio-drawing style contract. */
import { expect, test } from "bun:test";

test("Draw SVG export uses the typed SVG serializer directly", async () => {
  const source = await Bun.file(new URL("../../../../../../../../🦀️.rs", import.meta.url)).text();
  const body = source.slice(source.indexOf("pub fn drawing_document_to_svg("), source.indexOf("pub fn drawing_document_json_to_svg("));
  expect(body.includes("drawing_document_to_semio_drawing")).toBe(false);
  expect(body.includes("export::svg::v1_1::any::")).toBe(true);
});

import { DOMParser } from "@xmldom/xmldom";
import { drawingSceneToSvg, type DrawingSvgNode } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🔣️.json";

test("SVG fixture preserves gradients, affine matrices, text, image opacity and disabled paint", () => {
  const output = drawingSceneToSvg(fixture.nodes as DrawingSvgNode[], fixture.viewBox as [number,number,number,number]);
  const doc = new DOMParser().parseFromString(output, "image/svg+xml");
  const root = doc.documentElement!;
  expect(root.namespaceURI).toBe("http://www.w3.org/2000/svg");
  expect(root.getAttribute("viewBox")).toBe("-20 -10 200 100");
  const groups = Array.from(doc.getElementsByTagName("g"));
  expect(groups.map(group => group.getAttribute("data-layer-id"))).toEqual(fixture.nodes.map(node => node.id));
  expect(groups[0]!.getAttribute("transform")).toBe("matrix(1 0.5 -0.25 1 7 11)");
  expect(groups[0]!.getAttribute("opacity")).toBe("0.7");
  expect(groups[0]!.getAttribute("style")).toContain("multiply");
  const path = doc.getElementsByTagName("path")[0]!;
  expect(path.getAttribute("fill")).toBe("url(#draw-gradient-0)");
  expect(path.getAttribute("fill-rule")).toBe("evenodd");
  expect(path.getAttribute("stroke-opacity")).toBe("0.4");
  expect(path.getAttribute("stroke-linecap")).toBe("round");
  expect(path.getAttribute("stroke-linejoin")).toBe("bevel");
  expect(path.getAttribute("stroke-dasharray")).toBe("3 2");
  expect(path.getAttribute("d")).toContain("Q 5 9 10 0");
  expect(path.getAttribute("d")).toContain("A 5 3 25 1 0 30 10");
  const gradient = doc.getElementsByTagName("linearGradient")[0]!;
  expect(gradient.getAttribute("gradientUnits")).toBe("userSpaceOnUse");
  expect(gradient.getAttribute("x2")).toBe("30");
  expect(doc.getElementsByTagName("stop")[0]!.getAttribute("stop-opacity")).toBe("0.25");
  expect(doc.getElementsByTagName("radialGradient")[0]!.getAttribute("r")).toBe("12");
  const text = doc.getElementsByTagName("text")[0]!;
  expect(text.getAttribute("font-size")).toBe("20");
  const lines = Array.from(text.getElementsByTagName("tspan"));
  expect(lines.map(line => line.textContent)).toEqual(["A<&>", "Ü 🌍"]);
  expect(lines.map(line => line.getAttribute("y"))).toEqual(["20", "44"]);
  const image = doc.getElementsByTagName("image")[0]!;
  expect(image.getAttribute("href")).toBe(fixture.nodes[2]!.image!.src);
  expect(image.getAttributeNS("http://www.w3.org/1999/xlink", "href")).toBe(fixture.nodes[2]!.image!.src);
  expect(groups[2]!.getAttribute("opacity")).toBe("0.5");
  expect(doc.getElementsByTagName("path")[1]!.getAttribute("fill")).toBe("none");
  expect(doc.getElementsByTagName("path")[1]!.getAttribute("stroke")).toBe("none");
});

test("SVG rejects invalid dimensions and affine coefficients", () => {
  expect(() => drawingSceneToSvg([], [0,0,0,10])).toThrow();
  expect(() => drawingSceneToSvg([], [NaN,0,10,10])).toThrow();
  const nodes = fixture.nodes as DrawingSvgNode[];
  expect(() => drawingSceneToSvg([{...nodes[0]!, transform:[1,0,0,1,Infinity,0]}],[0,0,10,10])).toThrow();
});


test("SVG refuses nonfinite geometry and paint instead of downloading corrupt artwork", () => {
  for (const sample of fixture.invalidNumbers) {
    const nodes = structuredClone(fixture.nodes) as DrawingSvgNode[];
    const node = nodes[sample.node]!;
    const value = Number(sample.value);
    const replacement = sample.field === "opacity" ? {...node, opacity:value}
      : sample.field === "curve" ? {...node, segments:[{kind:"move" as const,to:[value,0] as [number,number]}]}
      : sample.field === "gradient" ? {...node, fill:{kind:"solid" as const,color:[value,0,0,1] as [number,number,number,number]}}
      : sample.field === "stroke" ? {...node, stroke:{...node.stroke!,width:value}}
      : sample.field === "text" ? {...node,text:{...node.text!,size:value}}
      : {...node,image:{...node.image!,width:value}};
    nodes[sample.node] = replacement;
    expect(() => drawingSceneToSvg(nodes, fixture.viewBox as [number,number,number,number])).toThrow("finite");
  }
});
