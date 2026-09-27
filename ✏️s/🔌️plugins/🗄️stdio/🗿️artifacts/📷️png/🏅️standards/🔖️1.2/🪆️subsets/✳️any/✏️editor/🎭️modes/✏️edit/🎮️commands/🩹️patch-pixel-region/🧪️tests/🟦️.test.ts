import { PNG } from "pngjs";
import { describe, expect, it } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { applyPngPixelRegionRgba8 } from "../🟦️.ts";

describe("PNG pixel region fixture", () => {
  it.each(fixture.cases)("matches pngjs bitblt for $name", ({ image, command, expectedPixels }) => {
    const actual = applyPngPixelRegionRgba8(Uint8Array.from(image.pixels), image.width, image.height, command);
    expect(Array.from(actual)).toEqual(expectedPixels);

    const destination = new PNG({ width: image.width, height: image.height });
    destination.data = Buffer.from(image.pixels);
    const source = new PNG({ width: command.width, height: command.height });
    for (let index = 0; index < source.data.length; index += 4) {
      source.data[index] = command.red;
      source.data[index + 1] = command.green;
      source.data[index + 2] = command.blue;
      source.data[index + 3] = command.alpha;
    }
    PNG.bitblt(source, destination, 0, 0, command.width, command.height, command.x, command.y);
    expect(Array.from(destination.data)).toEqual(expectedPixels);

    const reopened = PNG.sync.read(PNG.sync.write(destination));
    expect({ width: reopened.width, height: reopened.height, pixels: Array.from(reopened.data) }).toEqual({ width: image.width, height: image.height, pixels: expectedPixels });
  });
});
