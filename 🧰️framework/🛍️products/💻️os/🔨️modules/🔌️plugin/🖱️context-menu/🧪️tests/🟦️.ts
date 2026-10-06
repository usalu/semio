/** 🗣️ The context-menu selection glossary and phrase cases agree with the schema and with ICU's own CLDR plural rules, list conjunctions and number formatting (`Intl`), the independent oracle for the Rust `selection_count_phrase`. */
import {expect,test} from "bun:test";

import fixture from "../🧫️fixtures/🔣️.json";

type Locale="en"|"de";
type Kind=keyof typeof fixture.kinds;
/** 🧮️ One `(count, kind)` pair of a phrase case. */
type Count={readonly count:number;readonly kind:Kind};

const icuPhrase=(locale:Locale,counts:readonly Count[]):string|null=>{
  const plural=new Intl.PluralRules(locale,{type:"cardinal"}),number=new Intl.NumberFormat(locale,{useGrouping:false});
  const parts=counts.filter(entry=>entry.count>0).map(entry=>{const category=plural.select(entry.count);if(category!=="one"&&category!=="other")throw new Error(`${locale} has the unglossed plural category ${category}`);return `${number.format(entry.count)} ${fixture.kinds[entry.kind][locale][category]}`;});
  return parts.length===0?null:new Intl.ListFormat(locale,{type:"conjunction",style:"long"}).format(parts);
};



for(const row of fixture.cases)test(`phrase ${row.locale}: ${row.name}`,()=>{
  expect(icuPhrase(row.locale as Locale,row.counts as readonly Count[])).toBe(row.expected);
});

for(const row of fixture.deleteRows)test(`delete row ${row.locale}: ${row.name}`,()=>{
  const locale=row.locale as Locale,phrase=icuPhrase(locale,[{count:row.nodes,kind:"node"},{count:row.edges,kind:"edge"}]);
  expect(row.label).toBe(phrase===null?fixture.labels.deleteSelection[locale]:`${fixture.labels.deleteSelection[locale]} (${phrase})`);
  expect(row.reason).toBe(phrase===null?fixture.labels.nothingSelected[locale]:null);
});

test("every case names every glossary kind in both locales",()=>{
  for(const locale of ["en","de"] as const)expect(new Set(fixture.cases.filter(row=>row.locale===locale).flatMap(row=>row.counts.map(entry=>entry.kind)))).toEqual(new Set(Object.keys(fixture.kinds)));
});
