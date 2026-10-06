/** 🧮️ Explicit-locale status transitions agree with an independent i18next projection. */
import {expect,test} from "bun:test";
import {createInstance} from "i18next";
import rows from "../../🧫️fixtures/🔣️.json";
import {drawingSelectionStatus} from "../../🟦️.ts";

test("drawing selection status projects bilingual live counts with an independent localization oracle",async()=>{
 expect(Object.hasOwn(rows,"ambientLocale")).toBe(false);
 expect(new Set(rows.labels.map(labels=>labels.locale))).toEqual(new Set(["en","de"]));
 let witnesses=0;
 for(const labels of rows.labels){
  const locale=labels.locale==="en"?"en":"de",oracle=createInstance();
  await oracle.init({lng:locale,fallbackLng:false,resources:{[locale]:{translation:{layers_one:labels.layerOne,layers_other:labels.layerMany,selected:labels.selected}}},interpolation:{prefix:"{",suffix:"}",escapeValue:false}});
  for(const row of rows.cases){
   expect(row.expected[locale].length).toBe(row.selections.length);
   for(const [step,selection]of row.selections.entries()){
    expect(selection.every(index=>index<row.layers)).toBe(true);
    const expected=row.expected[locale][step]!;
    expect(oracle.t("layers",{count:row.layers})+" · "+oracle.t("selected",{count:selection.length})).toBe(expected);
    expect(drawingSelectionStatus(row.layers,selection.length,labels)).toBe(expected);
    witnesses++;
   }
  }
 }
 expect(witnesses).toBe(16);
 process.stderr.write(`[DEBUG] Draw live count text matched ${witnesses} neutral bilingual transitions and independent i18next plural/interpolation output\n`);
});

test("drawing status refuses invalid counts without choosing an ambient locale",()=>{
 for(const count of [-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1]){
  expect(()=>drawingSelectionStatus(count,0,rows.labels[0]!)).toThrow(/count/);
  expect(()=>drawingSelectionStatus(0,count,rows.labels[1]!)).toThrow(/count/);
 }
});
