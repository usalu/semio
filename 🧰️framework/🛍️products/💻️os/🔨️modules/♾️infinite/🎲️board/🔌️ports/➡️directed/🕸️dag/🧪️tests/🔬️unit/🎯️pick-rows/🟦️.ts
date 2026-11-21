import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Box2, Vector2 } from "three";

type Case = {
  lod: string;
  widget: string;
  port: string;
  side: "input" | "output";
  probe: "row" | "anchor" | "title";
  handle: string | null;
  rowAddition: boolean;
};

const { reference, cases }=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎯️pick-rows.json", import.meta.url), "utf8")) as {
  reference: {row:[number,number,number,number];anchor:[number,number];anchorHitRadius:number;rowFractionFromAnchor:number};
  cases: Case[];
};

test("Three hit geometry confirms connector priority and the detail row fallback", () => {
  const [x0,y0,x1,y1]=reference.row;
  const bounds=new Box2(new Vector2(x0,y0),new Vector2(x1,y1));
  const row=bounds.getCenter(new Vector2());
  for(const item of cases){
    const anchor=new Vector2(item.side==="input"?reference.anchor[0]:x1-reference.anchor[0],reference.anchor[1]);
    const title=new Vector2(row.x,y0-1);
    const ratio=item.side==="input"?reference.rowFractionFromAnchor:1-reference.rowFractionFromAnchor;
    const rowProbe=new Vector2(x0+(x1-x0)*ratio,row.y);
    const probe=item.probe==="row"?rowProbe:item.probe==="anchor"?anchor:title;
    const connector=item.lod!=="minimap"&&anchor.distanceTo(probe)<=reference.anchorHitRadius;
    const rowAddition=!connector&&["detail","micro"].includes(item.lod)&&bounds.containsPoint(probe);
    expect(rowAddition,item.lod).toBe(item.rowAddition);
    expect(connector||rowAddition?`${item.widget}@${item.port}`:null,item.lod).toBe(item.handle);
  }
});
