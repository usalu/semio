/** 🖌️ The depiction draws a frame of the core by the drawing rules: one `<svg class="pet">` per actor, one group per
 * run of parts on the same bone carrying the matrix of that bone, the face above the part the species names, colours
 * through classes and palette custom properties. Its matrix strings are held to gl-matrix's `mat2d`, which composes the
 * same rig and carries the same points; a frame that changes nothing writes nothing, a move nobody could see writes
 * nothing either, and nothing is ever written in a way a Content-Security-Policy without inline styles would block.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://glmatrix.net/docs/module-mat2d.html — the third-party oracle
 */
import { glMatrix } from "gl-matrix";
import { describe } from "vitest";
import { registerDrawsOneSvgPerTests } from "./🔶️draws-one-svg-per/🟦️.tsx";
import { registerWritesNothingFrameChangesTests } from "./🔸️writes-nothing-frame-changes/🟦️.tsx";

glMatrix.setMatrixArrayType(Array);

describe("pet depiction", () => {
  registerDrawsOneSvgPerTests();
  registerWritesNothingFrameChangesTests();
});
