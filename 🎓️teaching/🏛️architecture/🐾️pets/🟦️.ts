/** 🐾️ The architecture menagerie: the thirty-nine pets of the architecture and technology topics and quizzes, each the animated likeness
 * of a thing the quiz items are about, with the bonds between them and the casts of the home screen and of every quiz.
 *
 * Every document is imported statically, so the menagerie reaches a browser as part of the script chunk that imports
 * this module and nothing is fetched at run time. The species are listed in the order of the ensemble's paths.
 * @see ./🔣️.json — the ensemble: species paths, bonds and casts
 * @see ./README.md — the roster, the casts, the bonds in words and how to add a pet
 * @see ../📚️course/🟦️.ts — the site that hands this menagerie to the quiz
 * @see ../../../🧰️framework/🛍️products/🐾️pets/README.md — the pets product: menageries, stages and frames */
import { assembleMenagerie, type Ensemble, type Menagerie, type Species } from "@semio-tech/pets";
import ensemble from "./🔣️.json";
import sunny from "./☀️sunny/🔣️.json";
import cloudy from "./☁️cloudy/🔣️.json";
import housy from "./🏠️housy/🔣️.json";
import solary from "./🔆️solary/🔣️.json";
import radiatory from "./♨️radiatory/🔣️.json";
import pumpy from "./🌀️pumpy/🔣️.json";
import windowy from "./🪟️windowy/🔣️.json";
import waly from "./🧱️waly/🔣️.json";
import battery from "./🔋️battery/🔣️.json";
import windy from "./💨️windy/🔣️.json";
import boily from "./🔥️boily/🔣️.json";
import roofy from "./🛖️roofy/🔣️.json";
import insuly from "./🧶️insuly/🔣️.json";
import shady from "./😎️shady/🔣️.json";
import venty from "./🌬️venty/🔣️.json";
import chilly from "./❄️chilly/🔣️.json";
import kettly from "./🫖️kettly/🔣️.json";
import flamy from "./🕯️flamy/🔣️.json";
import thermy from "./🌡️thermy/🔣️.json";
import servy from "./🖥️servy/🔣️.json";
import concept0 from "./🦏️rhino/🔣️.json";
import concept1 from "./🦗️grasshopper/🔣️.json";
import concept2 from "./🎚️slider/🔣️.json";
import concept3 from "./🎨️colour-wheel/🔣️.json";
import concept4 from "./📚️workbook/🔣️.json";
import concept5 from "./📄️worksheet/🔣️.json";
import concept6 from "./🔲️cell/🔣️.json";
import concept7 from "./ƒ️formula/🔣️.json";
import concept8 from "./〰️double-curved-surface/🔣️.json";
import concept9 from "./∞️continuous-parameter/🔣️.json";
import concept10 from "./🔢️discrete-parameter/🔣️.json";
import concept11 from "./📋️execution-plan/🔣️.json";
import concept12 from "./🧊️model-3d/🔣️.json";
import concept13 from "./📐️plan-2d/🔣️.json";
import concept14 from "./🧮️quantities/🔣️.json";
import concept15 from "./📉️pareto/🔣️.json";
import concept16 from "./🟧️heatmap/🔣️.json";
import concept17 from "./🏞️minimized-surface/🔣️.json";
import concept18 from "./📊️bar-chart/🔣️.json";

/** 🎪️ The architecture menagerie: the owner's nine first, twenty energy companions and nineteen authored concepts,
 * with casts for home, each topic, each quiz and each public question subject. */
export const ARCHITECTURE_MENAGERIE: Menagerie = assembleMenagerie(
  ensemble as unknown as Ensemble,
  [sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy, concept0, concept1, concept2, concept3, concept4, concept5, concept6, concept7, concept8, concept9, concept10, concept11, concept12, concept13, concept14, concept15, concept16, concept17, concept18] as unknown as readonly Species[],
);
