/** 📣️ Selection status updates retain one polite live DOM region in either explicit locale. */
import * as React from "react";
import {render} from "@testing-library/react";
import {expect,it} from "vitest";
import Ajv from "ajv";
import {I18nextProvider} from "react-i18next";
import {Engagement,createShellI18nInstance,disposeShellI18nInstance} from "../../🎯️targets/⚛️react/🟦️.tsx";
import rows from "../../🧫️fixtures/📣️engagement-status/🔣️.json";

it("keeps changing engagement status in one polite atomic accessible region",()=>{
 let transitions=0;
 for(const row of rows.cases){
  const instance=createShellI18nInstance(row.locale==="en"?"en":"de");
  const tree=(content:string)=><I18nextProvider i18n={instance}><Engagement status={[{id:"selection-count",content}]}/></I18nextProvider>;
  const view=render(tree(row.texts[0]!));
  try{
   const live=view.getByRole(rows.role);
   expect(live.getAttribute("aria-live")).toBe(rows.live);
   expect(live.getAttribute("aria-atomic")).toBe(String(rows.atomic));
   for(const content of row.texts){
    view.rerender(tree(content));
    expect(view.getByRole(rows.role)).toBe(live);
    expect(live.textContent).toBe(content);
    transitions++;
   }
  }finally{view.unmount();disposeShellI18nInstance(instance);}
 }
 expect(transitions).toBe(6);
 process.stderr.write(`[DEBUG] Engagement status retained one polite atomic live DOM region across ${transitions} bilingual selection updates\n`);
});
