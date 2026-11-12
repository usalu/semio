/** 🚫️ The named, bilingual refusals of the 3D model pipeline: a request that makes no sense (a clip as a static GLB, a pose as an FBX, a viewpoint for a model, a view for a file that holds all of them) is refused by name before any work starts, never repaired.
 *
 * @see ../🟦️.ts — `modelOf`
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";

/** 🏷️ The stable codes of {@link ModelRequestError}. */
export const MODEL_REFUSALS = {
  "glb-needs-pose": { en: "A GLB is one static pose: name --pose, a clip is exported as FBX.", de: "Eine GLB ist eine statische Pose: --pose angeben, ein Clip wird als FBX exportiert." },
  "fbx-needs-clip": { en: "An FBX is one clip: name --clip, a static pose is exported as GLB.", de: "Eine FBX ist ein Clip: --clip angeben, eine statische Pose wird als GLB exportiert." },
  "bundle-takes-neither": { en: "A model bundle holds every pose and every clip: give neither --pose nor --clip.", de: "Ein Modellbündel enthält jede Pose und jeden Clip: weder --pose noch --clip angeben." },
  "model-needs-one": { en: "A model document is one pose or one clip: give exactly one of --pose and --clip.", de: "Ein Modelldokument ist eine Pose oder ein Clip: genau eines von --pose und --clip angeben." },
  "model-no-viewpoint": { en: "A 3D model has no viewpoint: --viewpoint and --projection do not apply.", de: "Ein 3D-Modell hat keine Ansicht: --viewpoint und --projection gelten nicht." },
  "model-no-time": { en: "A 3D model holds the whole clip: --time and --fps do not apply (the clip fps is authored).", de: "Ein 3D-Modell enthält den ganzen Clip: --time und --fps gelten nicht (die Clip-fps sind festgelegt)." },
  "model-all-views": { en: "This file holds all seven views as variants: --view does not apply.", de: "Diese Datei enthält alle sieben Ansichten als Varianten: --view gilt nicht." },
  "model-environment-view": { en: "--environment applies to the thermal view only: use --view thermal.", de: "--environment gilt nur für die thermische Ansicht: --view thermal verwenden." },
  "model-unknown-pose": { en: "The asset has no such pose.", de: "Das Asset hat keine solche Pose." },
  "model-unknown-clip": { en: "The asset has no such clip.", de: "Das Asset hat keinen solchen Clip." },
  "model-no-volume": { en: "The asset has no authored volume, so it has no 3D model.", de: "Das Asset hat kein erstelltes Volumen und damit kein 3D-Modell." },
  "model-no-clips": { en: "The asset has no clip, so it has no FBX.", de: "Das Asset hat keinen Clip und damit keine FBX." },
  "model-unknown-bone": { en: "A volume solid names a bone that the rig does not have.", de: "Ein Volumenkörper nennt einen Knochen, den das Gerüst nicht hat." },
  "model-batch-path": { en: "An output path is longer than 256 characters.", de: "Ein Ausgabepfad ist länger als 256 Zeichen." },
  "camera-azimuth": { en: "The azimuth of an axonometric camera lies between -180 and 180 degrees.", de: "Der Azimut einer axonometrischen Kamera liegt zwischen -180 und 180 Grad." },
  "camera-elevation": { en: "The elevation of an axonometric camera lies between 0 and 75 degrees.", de: "Die Elevation einer axonometrischen Kamera liegt zwischen 0 und 75 Grad." },
  "camera-format": { en: "--camera must be isometric, dimetric or <azimuth>,<elevation> in degrees.", de: "--camera muss isometric, dimetric oder <Azimut>,<Elevation> in Grad sein." },
  "views-bad": { en: "--views must name elevation, front and plan, each once, and may add axonometric.", de: "--views muss elevation, front und plan je einmal nennen und darf axonometric hinzufügen." },
  "model-bad-option": { en: "A model option is out of range.", de: "Eine Modelloption liegt ausserhalb des Bereichs." },
} as const satisfies Record<string, Text>;

/** 🔖️ One of the codes of {@link MODEL_REFUSALS}. */
export type ModelRefusal = keyof typeof MODEL_REFUSALS;

/** 🚨️ Thrown when a model request is refused; `code` is stable, `reason` holds both languages, the message names the code and carries English then German. */
export class ModelRequestError extends Error {
  readonly reason: Text;

  constructor(readonly code: ModelRefusal, readonly detail?: Slug | string) {
    const reason = MODEL_REFUSALS[code];
    const suffix = detail === undefined ? "" : ` (${detail})`;
    super(`${code}: ${reason.en}${suffix} / ${reason.de}${suffix}`);
    this.name = "ModelRequestError";
    this.reason = reason;
  }
}
