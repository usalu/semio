/** 🐾️ `@semio-tech/pets-react` — the web render target of `@semio-tech/pets`.
 *
 * Domain-neutral: it draws whatever menagerie it is given. The depiction turns a frame of the core into inline SVG; the
 * paints live in `🎨️.css`, which this module loads. Everything is decoration: nothing here takes focus or pointer
 * events, and nothing depends on a design system — React and the pets core only.
 *
 * @see ../../🧬️schema/🔣️.json — the contract every menagerie, stage and frame follows
 * @see ./🔨️modules/🖌️depiction/🟦️.ts — the drawing rules
 * @see ./📖️stories/🟦️.tsx — the gallery for judging rigs and behaviour by eye (never part of a release)
 */

import "./🎨️.css";

//#region 🔁️Reexports
export { MENAGERIE_SCHEMA, PET_MODES, TICKS_PER_SECOND } from "@semio-tech/pets";
export type { ActorFrame, EyeFrame, Frame, Menagerie, PetMode, Point, Species } from "@semio-tech/pets";
export { depict, depictionMarkup, paint } from "./🔨️modules/🖌️depiction/🟦️.ts";
export type { Depiction } from "./🔨️modules/🖌️depiction/🟦️.ts";
export { FLOOR, FOCUS_MARGIN, KEEPOUT_MARGIN, PET_CONTROLS, PET_KEEPOUTS, PET_SURFACES, PET_TEXTS, SURFACE_WIDTH, SURVEY_ATTRIBUTES, stageBox, surfaceId, survey, watchPointer, watchSurvey } from "./🔨️modules/📡️survey/🟦️.ts";
export type { PointerOptions, SurveyHost, SurveyOptions, SurveyWatchOptions } from "./🔨️modules/📡️survey/🟦️.ts";
export { FRAME_TICKS, TICK_MILLISECONDS, createPacer, documentHidden, watchVisibility } from "./🔨️modules/⏲️pacing/🟦️.ts";
export type { Pace, Pacer, PacerSeams, PacingHost } from "./🔨️modules/⏲️pacing/🟦️.ts";
export { PET_EDGE_INSET, PET_HOME_SCENE, PET_LAYER_Z_INDEX, PET_MEDIUM_WIDTH, PET_NARROW_WIDTH, PET_ROTATION_SECONDS, PetLayer, petCapacity, petCast, petScale } from "./🔨️modules/🫧️layer/🟦️.tsx";
export type { PetLayerProps } from "./🔨️modules/🫧️layer/🟦️.tsx";
//#endregion 🔁️Reexports
