/** 📊️ `@semio-tech/print-viz-kernel` — the TypeScript twin of the `semio-viz` LaTeX kernel.
 * One barrel over the domain modules so the kernel is consumed from code the way d3 is, with no
 * runtime dependency on anything outside this repository.
 * @see ../../../../🖋️latex/semio-viz.sty
 */

//#region 🔖️Schema
export * from "../../🧬️schema/🟦️.ts";
//#endregion 🔖️Schema

//#region 🔖️Kernel
export * from "../../📐scale/🟦️.ts";
export * from "../../🔢format/🟦️.ts";
export * from "../../🧮transform/🟦️.ts";
export * from "../../✒️mark/🟦️.ts";
export * from "../../🥧shape/🟦️.ts";
export * from "../../🧭coordinate/🟦️.ts";
//#endregion 🔖️Kernel

//#region 🔖️Layouts
export * from "../../🌳hierarchy/🟦️.ts";
export * from "../../🕸️network/🟦️.ts";
export * from "../../🌊flow/🟦️.ts";
export * from "../../🌍geo/🟦️.ts";
export * from "../../📍spatial/🟦️.ts";
//#endregion 🔖️Layouts

//#region 🔖️Presentation
export * from "../../🎨theme/🟦️.ts";
export * from "../../🖼️render/🟦️.ts";
//#endregion 🔖️Presentation
