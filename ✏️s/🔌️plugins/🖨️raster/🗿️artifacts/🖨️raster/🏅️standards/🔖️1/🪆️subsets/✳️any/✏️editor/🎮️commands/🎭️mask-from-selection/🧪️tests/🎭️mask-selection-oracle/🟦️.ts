import { expect, test } from "bun:test";
import sharp from "sharp";
import maskFixture from "../../🧫️fixtures/🔣️.json";
test("selection mask coverage matches the Sharp alpha-channel oracle",async()=>{
  const {width,height,selection,expectedRgba}=maskFixture;
  const alpha=Buffer.alloc(width*height);
  for(const {start,length,coverage} of selection) alpha.fill(coverage!,start!,start!+length!);
  const rgba=await sharp(Buffer.alloc(width*height*3,255),{raw:{width,height,channels:3}}).joinChannel(alpha,{raw:{width,height,channels:1}}).raw().toBuffer();
  expect([...rgba]).toEqual(expectedRgba);
});
