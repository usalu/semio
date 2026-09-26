/** 🎨️ Fill edits follow shared cases and an independent Immer document update. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { produce } from "immer";
import { Vector4 } from "three";
import { editFill, type Fill, type FillEdit } from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import cases from "../../🧫️fixtures/🔣️.json";
test("fill editing preserves stops, geometry, alpha and source on rejection", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  for (const entry of cases) {
    expect(validate(entry.edit)).toBe(true);
    const source = structuredClone(entry.before) as Fill | null;
    if (entry.error) expect(() => editFill(source, entry.edit as FillEdit)).toThrow();
    else {
      const result = editFill(source, entry.edit as FillEdit);
      expect(result).toEqual(entry.after);
      if (entry.edit.kind === "addStop" && source && "stops" in source && result && "stops" in result) {
        const [left,right] = source.stops;
        const color = new Vector4(...left!.color).lerp(new Vector4(...right!.color), (entry.edit.offset!-left!.offset)/(right!.offset-left!.offset)).toArray();
        expect(result.stops.find(stop => stop.offset === entry.edit.offset)!.color).toEqual(color);
      }
      if (["alpha","offset","removeStop"].includes(entry.edit.kind)) {
        const oracle = produce({fill:source}, draft => {
          const fill=draft.fill!;
          if(entry.edit.kind==="alpha") {
            const color=fill.kind==="solid" ? fill.color : fill.stops[entry.edit.index!]!.color;
            color[3]=entry.edit.value as number;
          } else if("stops" in fill && entry.edit.kind==="offset") {
            fill.stops[entry.edit.index!]!.offset=entry.edit.value as number;
            fill.stops.sort((a,b)=>a.offset-b.offset);
          } else if("stops" in fill) fill.stops.splice(entry.edit.index!,1);
        });
        expect(result).toEqual(oracle.fill);
      }
    }
    expect(source).toEqual(entry.before);
  }
});
