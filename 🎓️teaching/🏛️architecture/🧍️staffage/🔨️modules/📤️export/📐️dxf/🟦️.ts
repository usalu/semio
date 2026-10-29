/** 📐️ DXF writer, AutoCAD 2000 (AC1015): HEADER ($INSUNITS 6 = metres, $MEASUREMENT 1, extents, $HANDSEED), CLASSES, TABLES (VPORT,
 * LTYPE, LAYER with ACI colour and lineweight, STYLE, VIEW, UCS, APPID, DIMSTYLE, BLOCK_RECORD), BLOCKS, ENTITIES (LWPOLYLINE, TEXT) and
 * OBJECTS (root dictionary, ACAD_GROUP, ACAD_LAYOUT with both layouts, ACAD_MLINESTYLE), every handle unique and every owner pointer resolvable.
 * Conventions follow the in-repo R12 reader/writer (`code\nvalue\n` tag pairs, doubles with a decimal point). Pure, deterministic, browser-compatible.
 * <https://help.autodesk.com/view/OARX/2023/ENU/?guid=GUID-235B22E0-A567-4CF6-92D3-38A2306D73F3> */
import type { ModelScene } from "../../🔗️contracts/🟦️.ts";

//#region 🔖️Values
/** 🧵️ The lineweights (1/100 mm) AutoCAD admits; every other value is snapped to the nearest. */
export const CAD_LINEWEIGHTS = [0, 5, 9, 13, 15, 18, 20, 25, 30, 35, 40, 50, 53, 60, 70, 80, 90, 100, 106, 120, 140, 158, 200, 211] as const;

/** 🧵️ The nearest standard lineweight (1/100 mm) to `hundredthsMm`. */
export const snapLineweight = (hundredthsMm: number): number => CAD_LINEWEIGHTS.reduce((best, weight) => (Math.abs(weight - hundredthsMm) < Math.abs(best - hundredthsMm) ? weight : best));

const real = (value: number): string => {
  if (!Number.isFinite(value)) throw new Error(`DXF coordinate must be finite, got ${value}`);
  const text = (Math.abs(value) < 5e-10 ? 0 : value).toFixed(9).replace(/0+$/u, "");
  return text.endsWith(".") ? `${text}0` : text;
};

const escapeText = (text: string): string => [...text.replace(/\r?\n/gu, " ")].map((character) => { const code = character.codePointAt(0)!; return code >= 0x20 && code < 0x7f ? character : code < 0x20 ? " " : `\\U+${code.toString(16).toUpperCase().padStart(4, "0")}`; }).join("");

const FORBIDDEN_NAME = /[<>/\\":;?*|=`\n\r]/u;
//#endregion 🔖️Values

/** 📐️ Writes `scene` (metres) as a DXF R2000 document. */
export function writeDxf(scene: ModelScene): string {
  if (scene.units !== "m") throw new Error(`ModelScene units must be "m", got ${JSON.stringify(scene.units)}`);
  const names = new Set<string>();
  for (const layer of scene.layers) {
    if (!layer.name || FORBIDDEN_NAME.test(layer.name) || layer.name.length > 255) throw new Error(`Layer name ${JSON.stringify(layer.name)} is not a valid DXF table name`);
    if (names.has(layer.name.toUpperCase())) throw new Error(`Layer name ${JSON.stringify(layer.name)} is duplicated`);
    names.add(layer.name.toUpperCase());
    if (!Number.isInteger(layer.aci) || layer.aci < 1 || layer.aci > 255) throw new Error(`Layer ${layer.name}: ACI colour must be 1..255, got ${layer.aci}`);
  }
  const zero = scene.layers.find((layer) => layer.name === "0");
  const declared = scene.layers.filter((layer) => layer.name !== "0");
  let next = 1;
  const fresh = (): string => (next++).toString(16).toUpperCase();
  const h = { root: fresh(), groups: fresh(), layouts: fresh(), mlines: fresh(), mlineStandard: fresh(), layoutModel: fresh(), layoutPaper: fresh(), vport: fresh(), vportRecord: fresh(), ltype: fresh(), ltypeByBlock: fresh(), ltypeByLayer: fresh(), ltypeContinuous: fresh(), ltypeDashed: fresh(), layer: fresh(), layerZero: fresh(), style: fresh(), styleStandard: fresh(), view: fresh(), ucs: fresh(), appid: fresh(), appidAcad: fresh(), dimstyle: fresh(), dimstyleStandard: fresh(), blockRecords: fresh(), recordModel: fresh(), recordPaper: fresh(), blockModel: fresh(), endModel: fresh(), blockPaper: fresh(), endPaper: fresh() };
  const layerHandles = declared.map(() => fresh());

  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  const touch = (x: number, y: number): void => { minX = Math.min(minX, x); minY = Math.min(minY, y); maxX = Math.max(maxX, x); maxY = Math.max(maxY, y); };
  for (const layer of scene.layers) {
    for (const line of layer.polylines) for (const point of line.points) touch(point.x, point.y);
    for (const text of layer.texts) touch(text.at.x, text.at.y);
  }
  const empty = !Number.isFinite(minX);
  const [x0, y0, x1, y1] = empty ? [0, 0, 1, 1] : [minX, minY, maxX, maxY];

  const out: string[] = [];
  const g = (code: number, value: string | number): void => { out.push(String(code).padStart(3), typeof value === "number" ? String(value) : value); };
  const r = (code: number, value: number): void => g(code, real(value));
  const pt = (code: number, x: number, y: number, z = 0): void => { r(code, x); r(code + 10, y); r(code + 20, z); };
  const begin = (name: string): void => { g(0, "SECTION"); g(2, name); };
  const end = (): void => g(0, "ENDSEC");
  const owned = (kind: string, handle: string, owner: string, ...markers: string[]): void => { g(0, kind); g(5, handle); g(330, owner); for (const marker of markers) g(100, marker); };
  const table = (name: string, handle: string, count: number, body: () => void): void => { g(0, "TABLE"); g(2, name); g(5, handle); g(330, "0"); g(100, "AcDbSymbolTable"); g(70, count); body(); g(0, "ENDTAB"); };
  const record = (kind: string, handle: string, owner: string, marker: string): void => { g(0, kind); g(5, handle); g(330, owner); g(100, "AcDbSymbolTableRecord"); g(100, marker); };

  begin("HEADER");
  const variable = (name: string, ...tags: readonly (readonly [number, string | number])[]): void => { g(9, name); for (const [code, value] of tags) g(code, value); };
  variable("$ACADVER", [1, "AC1015"]);
  variable("$ACADMAINTVER", [70, 6]);
  variable("$DWGCODEPAGE", [3, "ANSI_1252"]);
  variable("$INSBASE", [10, "0.0"], [20, "0.0"], [30, "0.0"]);
  variable("$EXTMIN", [10, empty ? "1e20" : real(x0)], [20, empty ? "1e20" : real(y0)], [30, empty ? "1e20" : "0.0"]);
  variable("$EXTMAX", [10, empty ? "-1e20" : real(x1)], [20, empty ? "-1e20" : real(y1)], [30, empty ? "-1e20" : "0.0"]);
  variable("$LIMMIN", [10, real(x0)], [20, real(y0)]);
  variable("$LIMMAX", [10, real(x1)], [20, real(y1)]);
  variable("$ORTHOMODE", [70, 0]);
  variable("$LTSCALE", [40, "1.0"]);
  variable("$TEXTSIZE", [40, "0.1"]);
  variable("$TEXTSTYLE", [7, "Standard"]);
  variable("$CLAYER", [8, "0"]);
  variable("$CELTYPE", [6, "ByLayer"]);
  variable("$CECOLOR", [62, 256]);
  variable("$CELWEIGHT", [370, -1]);
  variable("$LWDISPLAY", [290, 1]);
  variable("$LUNITS", [70, 2]);
  variable("$LUPREC", [70, 4]);
  variable("$AUNITS", [70, 0]);
  variable("$AUPREC", [70, 0]);
  variable("$MEASUREMENT", [70, 1]);
  variable("$INSUNITS", [70, 6]);
  variable("$PDMODE", [70, 0]);
  variable("$DIMSTYLE", [2, "Standard"]);
  variable("$CMLSTYLE", [2, "Standard"]);
  variable("$HANDSEED", [5, "FFFFFFFF"]);
  const seedAt = out.length - 1;
  end();

  begin("CLASSES");
  g(0, "CLASS"); g(1, "LAYOUT"); g(2, "AcDbLayout"); g(3, "ObjectDBX Classes"); g(90, 0); g(91, 2); g(280, 0); g(281, 0);
  end();

  begin("TABLES");
  table("VPORT", h.vport, 1, () => {
    record("VPORT", h.vportRecord, h.vport, "AcDbViewportTableRecord");
    g(2, "*Active"); g(70, 0); pt(10, 0, 0); pt(11, 1, 1); g(12, real((x0 + x1) / 2)); g(22, real((y0 + y1) / 2)); g(13, "0.0"); g(23, "0.0"); g(14, "1.0"); g(24, "1.0"); g(15, "1.0"); g(25, "1.0"); pt(16, 0, 0, 1); pt(17, 0, 0); r(40, Math.max(y1 - y0, (x1 - x0) / 1.5, 1e-6) * 1.1); g(41, "1.5"); g(42, "50.0"); g(43, "0.0"); g(44, "0.0"); g(50, "0.0"); g(51, "0.0"); g(71, 0); g(72, 100); g(73, 1); g(74, 3); g(75, 0); g(76, 0); g(77, 0); g(78, 0); g(281, 0); g(65, 1); pt(110, 0, 0); pt(111, 1, 0); pt(112, 0, 1); g(79, 0); g(146, "0.0");
  });
  table("LTYPE", h.ltype, 4, () => {
    for (const [handle, name, text, pattern] of [[h.ltypeByBlock, "ByBlock", "", []], [h.ltypeByLayer, "ByLayer", "", []], [h.ltypeContinuous, "Continuous", "Solid line", []], [h.ltypeDashed, "DASHED", "Dashed __ __ __ __ __ __ __ __ __ __ __ __ __", [0.5, -0.25]]] as const) {
      record("LTYPE", handle, h.ltype, "AcDbLinetypeTableRecord");
      g(2, name); g(70, 0); g(3, text); g(72, 65); g(73, pattern.length); r(40, pattern.reduce((sum, part) => sum + Math.abs(part), 0));
      for (const part of pattern) { r(49, part); g(74, 0); }
    }
  });
  table("LAYER", h.layer, declared.length + 1, () => {
    const emit = (handle: string, name: string, aci: number, lineweight: number): void => {
      record("LAYER", handle, h.layer, "AcDbLayerTableRecord");
      g(2, name); g(70, 0); g(62, aci); g(6, "Continuous"); g(290, 1); g(370, lineweight);
    };
    emit(h.layerZero, "0", zero?.aci ?? 7, zero ? snapLineweight(zero.lineweight) : -3);
    declared.forEach((layer, index) => emit(layerHandles[index]!, layer.name, layer.aci, snapLineweight(layer.lineweight)));
  });
  table("STYLE", h.style, 1, () => {
    record("STYLE", h.styleStandard, h.style, "AcDbTextStyleTableRecord");
    g(2, "Standard"); g(70, 0); g(40, "0.0"); g(41, "1.0"); g(50, "0.0"); g(71, 0); g(42, "0.2"); g(3, "txt"); g(4, "");
  });
  table("VIEW", h.view, 0, () => undefined);
  table("UCS", h.ucs, 0, () => undefined);
  table("APPID", h.appid, 1, () => {
    record("APPID", h.appidAcad, h.appid, "AcDbRegAppTableRecord");
    g(2, "ACAD"); g(70, 0);
  });
  table("DIMSTYLE", h.dimstyle, 1, () => {
    g(0, "DIMSTYLE"); g(105, h.dimstyleStandard); g(330, h.dimstyle); g(100, "AcDbSymbolTableRecord"); g(100, "AcDbDimStyleTableRecord");
    g(2, "Standard"); g(70, 0); g(40, "1.0"); g(41, "0.18"); g(42, "0.0625"); g(43, "0.38"); g(44, "0.18"); g(140, "0.18"); g(141, "0.09"); g(147, "0.09"); g(77, 1); g(78, 8); g(271, 4); g(272, 4); g(273, 2); g(274, 2); g(340, h.styleStandard);
  });
  table("BLOCK_RECORD", h.blockRecords, 2, () => {
    for (const [handle, name, layout] of [[h.recordModel, "*Model_Space", h.layoutModel], [h.recordPaper, "*Paper_Space", h.layoutPaper]] as const) {
      record("BLOCK_RECORD", handle, h.blockRecords, "AcDbBlockTableRecord");
      g(2, name); g(340, layout);
    }
  });
  end();

  begin("BLOCKS");
  for (const [opening, closing, owner, name, paper] of [[h.blockModel, h.endModel, h.recordModel, "*Model_Space", false], [h.blockPaper, h.endPaper, h.recordPaper, "*Paper_Space", true]] as const) {
    owned("BLOCK", opening, owner, "AcDbEntity"); if (paper) g(67, 1); g(8, "0"); g(100, "AcDbBlockBegin"); g(2, name); g(70, 0); pt(10, 0, 0); g(3, name); g(1, "");
    owned("ENDBLK", closing, owner, "AcDbEntity"); if (paper) g(67, 1); g(8, "0"); g(100, "AcDbBlockEnd");
  }
  end();

  begin("ENTITIES");
  for (const layer of scene.layers) {
    for (const line of layer.polylines) {
      if (line.points.length < 2) throw new Error(`Layer ${layer.name}: a polyline needs at least two points`);
      owned("LWPOLYLINE", fresh(), h.recordModel, "AcDbEntity"); g(8, layer.name); g(100, "AcDbPolyline"); g(90, line.points.length); g(70, line.closed ? 1 : 0); g(43, "0.0");
      for (const point of line.points) { r(10, point.x); r(20, point.y); }
    }
    for (const text of layer.texts) {
      if (!(text.heightM > 0)) throw new Error(`Layer ${layer.name}: text height must be positive`);
      owned("TEXT", fresh(), h.recordModel, "AcDbEntity"); g(8, layer.name); g(100, "AcDbText"); pt(10, text.at.x, text.at.y); r(40, text.heightM); g(1, escapeText(text.text)); g(7, "Standard"); g(100, "AcDbText");
    }
  }
  end();

  begin("OBJECTS");
  const dictionary = (handle: string, owner: string, entries: readonly (readonly [string, string])[]): void => { owned("DICTIONARY", handle, owner, "AcDbDictionary"); g(281, 1); for (const [key, value] of entries) { g(3, key); g(350, value); } };
  g(0, "DICTIONARY"); g(5, h.root); g(330, "0"); g(100, "AcDbDictionary"); g(281, 1);
  for (const [key, value] of [["ACAD_GROUP", h.groups], ["ACAD_LAYOUT", h.layouts], ["ACAD_MLINESTYLE", h.mlines]] as const) { g(3, key); g(350, value); }
  dictionary(h.groups, h.root, []);
  dictionary(h.layouts, h.root, [["Model", h.layoutModel], ["Layout1", h.layoutPaper]]);
  dictionary(h.mlines, h.root, [["Standard", h.mlineStandard]]);
  owned("MLINESTYLE", h.mlineStandard, h.mlines, "AcDbMlineStyle"); g(2, "STANDARD"); g(70, 0); g(3, ""); g(62, 256); r(51, 90); r(52, 90); g(71, 2);
  for (const offset of [0.5, -0.5]) { r(49, offset); g(62, 256); g(6, "BYLAYER"); }
  for (const [handle, name, tab, record, paper] of [[h.layoutModel, "Model", 0, h.recordModel, false], [h.layoutPaper, "Layout1", 1, h.recordPaper, true]] as const) {
    g(0, "LAYOUT"); g(5, handle); g(102, "{ACAD_REACTORS"); g(330, h.layouts); g(102, "}"); g(330, h.layouts);
    g(100, "AcDbPlotSettings"); g(1, ""); g(2, "none_device"); g(4, ""); g(6, ""); r(40, 0); r(41, 0); r(42, 0); r(43, 0); r(44, 0); r(45, 0); r(46, 0); r(47, 0); r(48, 0); r(49, 0); r(140, 0); r(141, 0); r(142, 1); r(143, 1);
    g(70, paper ? 0 : 1024); g(72, 0); g(73, 0); g(74, 0); g(7, ""); g(75, 16); g(147, "1.0"); pt(148, 0, 0); g(76, 0); g(77, 2); g(78, 300); g(149, "0.0");
    g(100, "AcDbLayout"); g(1, name); g(70, 1); g(71, tab); pt(10, 0, 0); pt(11, 12, 9); pt(12, 0, 0); pt(14, 1e20, 1e20, 1e20); pt(15, -1e20, -1e20, -1e20); g(146, "0.0"); pt(13, 0, 0); pt(16, 1, 0); pt(17, 0, 1); g(76, 0); g(330, record);
  }
  end();
  g(0, "EOF");

  out[seedAt] = next.toString(16).toUpperCase();
  return out.join("\n") + "\n";
}
