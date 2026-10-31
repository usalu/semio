/** 🔢️ PDF real numbers preserve exact numeric values without forbidden exponent tokens. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {pdfNumber} from "./🟦️.ts";
import rows from "./📐️.json";
import schema from "./🔣️.json";
test("PDF scalar decimals preserve every original value and reject nonfinite numbers",()=>{expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);for(const row of rows.cases){const actual=pdfNumber(row.value);expect(actual).toBe(row.expected);expect(actual).toMatch(/^-?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)$/);expect(JSON.parse(actual)).toBe(row.value===0?0:row.value);}for(const value of [NaN,Infinity,-Infinity])expect(()=>pdfNumber(value)).toThrow("PDF requires a finite number");});
