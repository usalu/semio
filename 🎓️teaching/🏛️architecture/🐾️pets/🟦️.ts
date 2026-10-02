/** 🐾️ The architecture menagerie: the twenty pets of the architecture and technology quizzes, each the animated likeness
 * of a thing the quiz items are about, with the bonds between them and the casts of the home screen and of every quiz.
 *
 * Every document is imported statically, so the menagerie reaches a browser as part of the script chunk that imports
 * this module and nothing is fetched at run time. The species are listed in the order of the ensemble's paths.
 * @see ./🔣️.json — the ensemble: species paths, bonds and casts
 * @see ./README.md — the roster, the casts, the bonds in words and how to add a pet
 * @see ../❓️quiz/🟦️.ts — the site that hands this menagerie to the quiz
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

/** 🎪️ The menagerie of the architecture and technology quizzes: the owner's nine first, then the eleven the quiz items
 * added; scene `home` and one scene per quiz of the catalog. */
export const ARCHITECTURE_MENAGERIE: Menagerie = assembleMenagerie(
  ensemble as unknown as Ensemble,
  [sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy] as unknown as readonly Species[],
);
