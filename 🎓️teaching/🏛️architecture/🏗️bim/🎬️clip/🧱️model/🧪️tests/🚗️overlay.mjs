/** 🚗 Exact batched polygon-set overlay behind an owned JSON test interface. */
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
import {resolve} from "node:path";

const clipping = createRequire(import.meta.url)("polygon-clipping");

function polygons(value) {
  if (!Array.isArray(value)) throw new TypeError("Expected polygon array");
  for (const polygon of value) {
    if (!Array.isArray(polygon) || !polygon.length) throw new TypeError("Expected polygon rings");
    for (const ring of polygon) {
      if (!Array.isArray(ring) || ring.length < 4 || ring.some(point => !Array.isArray(point) || point.length !== 2 || point.some(v => typeof v !== "number" || !Number.isFinite(v)))) throw new TypeError("Expected finite closed coordinate rings");
      if (ring[0][0] !== ring.at(-1)[0] || ring[0][1] !== ring.at(-1)[1]) throw new TypeError("Expected closed ring");
    }
  }
  return value;
}

function area(multi) {
  const ringArea = ring => {
    const [x, y] = ring[0];
    return Math.abs(ring.slice(1).reduce((sum, point, i) => sum + (ring[i][0] - x) * (point[1] - y) - (point[0] - x) * (ring[i][1] - y), 0) / 2);
  };
  return multi.reduce((sum, polygon) => sum + ringArea(polygon[0]) - polygon.slice(1).reduce((holes, ring) => holes + ringArea(ring), 0), 0);
}

/** 🧩 Consume and return ordinary JSON; no dependency type crosses the boundary. */
export function overlayJson(input) {
  const request = JSON.parse(input);
  if (!request || Object.keys(request).length !== 1 || !Array.isArray(request.cases)) throw new TypeError("Expected overlay cases");
  const ids = new Set();
  const cases = request.cases.map(value => {
    if (!value || Object.keys(value).sort().join() !== "actual,expected,id" || typeof value.id !== "string" || !value.id || ids.has(value.id)) throw new TypeError("Expected distinct case identities");
    ids.add(value.id);
    const actual = polygons(value.actual), expected = polygons(value.expected);
    const left = actual.length ? clipping.union(...actual) : [];
    const right = expected.length ? clipping.union(...expected) : [];
    const xor = left.length && right.length ? clipping.xor(left, right) : left.length ? left : right;
    const xor_area = area(xor);
    if (!Number.isFinite(xor_area) || xor_area < 0) throw new Error("Invalid overlay area");
    return {id:value.id, xor_area};
  });
  return JSON.stringify({engine:"polygon-clipping", cases});
}

if (typeof process !== "undefined" && process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  try {
    process.stdin.setEncoding("utf8");
    let input = "";
    for await (const chunk of process.stdin) input += chunk;
    process.stdout.write(overlayJson(input));
  } catch (error) {
    process.stderr.write(String(error));
    process.exitCode = 1;
  }
}
