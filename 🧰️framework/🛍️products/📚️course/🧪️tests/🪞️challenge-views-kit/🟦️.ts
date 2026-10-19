/** 🪞️ Shared fixtures of the challenge-view suites: the quantities and items of the sample tasks, the hint questions the second hint audit quotes, and the sheet and text helpers. */

import { screen } from "@testing-library/react";
import { type Challenge, type Hint, type Sheet, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask, type SheetTask } from "@semio-tech/course";
import { courseText, type CourseLocale } from "@semio-tech/course-react";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";

export const text = (en: string, de: string) => ({ en, de });
export const en = courseText("en");
export const NBSP = " ";

/** 🃏️ The shared sheet `icons-1` at `challenge` (its medium form has no suffix). */
export function sheetAt(challenge: Challenge): Sheet {
  const id = challenge === "medium" ? "icons-1" : `icons-1-${challenge}`;
  return sheets.sheets.find((vector) => vector.id === id)!.sheet as unknown as Sheet;
}

export const MASSES: SheetSortingTask = {
  kind: "sorting",
  id: "masses",
  title: text("Masses", "Massen"),
  prompt: text("Sort by mass.", "Nach Masse sortieren."),
  quantity: { label: text("Mass", "Masse"), unit: "g", scale: "logarithmic", prefixed: true, additive: true },
  keys: [20, 4000, 500_000],
  items: [
    { id: "horse", label: text("Horse", "Pferd") },
    { id: "mouse", label: text("Mouse", "Maus") },
    { id: "cat", label: text("Cat", "Katze") },
  ],
};

export const TEMPERATURES: SheetSortingTask = {
  kind: "sorting",
  id: "temperatures",
  title: text("Temperatures", "Temperaturen"),
  prompt: text("Sort by temperature.", "Nach Temperatur sortieren."),
  quantity: { label: text("Temperature", "Temperatur"), unit: "K", scale: "linear", prefixed: false, additive: false },
  keys: [250, 300, 350],
  items: [
    { id: "ice", label: text("Ice", "Eis") },
    { id: "tea", label: text("Tea", "Tee") },
    { id: "room", label: text("Room", "Raum") },
  ],
};

export const LAMPS: SheetMatchingTask = {
  kind: "matching",
  id: "lamps",
  title: text("Lamps", "Lampen"),
  prompt: text("Match each lamp its power.", "Ordne jeder Lampe ihre Leistung zu."),
  dimensions: [{ id: "power", quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true }, cards: [50, 8, 2000] }],
  items: [
    { id: "led", label: text("LED bulb", "LED-Lampe") },
    { id: "halogen", label: text("Halogen spot", "Halogenstrahler") },
    { id: "floodlight", label: text("Floodlight", "Flutlicht") },
  ],
};

export const PLACES: SheetClassificationTask = {
  kind: "classification",
  id: "places",
  title: text("Places", "Orte"),
  prompt: text("Assign each place.", "Ordne jeden Ort zu."),
  axes: [
    { id: "rain", label: text("Rain", "Regen") },
    { id: "sun", label: text("Sun", "Sonne") },
    { id: "heat", label: text("Heat", "Wärme") },
  ],
  categories: [
    { id: "dry", label: text("Dry", "Trocken"), profile: { rain: 0.05, sun: 0.9, heat: 0.8 } },
    { id: "wet", label: text("Wet", "Nass"), profile: { rain: 0.8, sun: 0.3, heat: 0.4 } },
  ],
  items: [
    { id: "desert", label: text("Desert", "Wüste") },
    { id: "fjord", label: text("Fjord", "Fjord") },
  ],
};

/** 🌧️ A classification whose keys show: axes with units and ranges (one below zero), a described and an undescribed
 * category; short forms on the categories, on two axes and on one item, whose labels are long. */
export const CLIMATES: SheetClassificationTask = {
  ...PLACES,
  id: "climates",
  axes: [
    { id: "rain", label: text("Annual rainfall", "Jährlicher Niederschlag"), short: text("Rainfall", "Niederschlag"), unit: "mm", min: 0, max: 3000 },
    { id: "sun", label: text("Sunshine hours per year", "Sonnenstunden pro Jahr"), short: text("sunshine", "Sonnenschein"), unit: "h", min: 0, max: 4000 },
    { id: "frost", label: text("Coldest month mean", "Januarmittel"), unit: "°C", min: -30, max: 30 },
  ],
  categories: [
    { id: "dry", label: text("Dry climate zone", "Trockene Klimazone"), short: text("Dry", "Trocken"), profile: { rain: 50, sun: 3500, frost: 12 } },
    { id: "wet", label: text("Wet climate zone", "Feuchte Klimazone"), short: text("Wet", "Nass"), description: text("Rain on most days.", "Regen an den meisten Tagen."), profile: { rain: 1500, sun: 1200, frost: -2.5 } },
  ],
  items: [{ id: "desert", label: text("Hot desert of the Sahara", "Heiße Wüste der Sahara"), short: text("Desert", "Wüste") }, PLACES.items[1]!, { id: "steppe", label: text("Steppe", "Steppe") }],
};

/** ➗️ The masses as a quantity whose amounts do not add up, so a hint asks about a ratio. */
const RATIOS: SheetSortingTask = { ...MASSES, id: "ratios", quantity: { ...MASSES.quantity, additive: false } };

/** ☀️ Powers that add up and span 25 decades, items named in hints by their short forms. */
export const POWERS: SheetSortingTask = {
  kind: "sorting",
  id: "powers",
  title: text("Powers", "Leistungen"),
  prompt: text("Sort by power.", "Nach Leistung sortieren."),
  quantity: { label: text("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true },
  keys: [40, 2000, 3.8e26],
  items: [
    { id: "sun", label: text("Total radiant power of the Sun", "Gesamte Strahlungsleistung der Sonne"), short: text("Sun", "Sonne") },
    { id: "tealight", label: text("Burning tea light (heat)", "Brennendes Teelicht (Wärme)"), short: text("Tea light", "Teelicht") },
    { id: "kettle", label: text("Kettle", "Wasserkocher") },
  ],
};

/** 🏘️ A matching of three dimensions — a ratio, an amount that adds up and a linear scale — whose quantities hints name
 * by their short forms (the temperature by its label, having none); one house is named by its short form. */
export const HOUSES: SheetMatchingTask = {
  kind: "matching",
  id: "houses",
  title: text("Houses", "Häuser"),
  prompt: text("Match each house its values.", "Ordne jedem Haus seine Werte zu."),
  dimensions: [
    { id: "load", quantity: { label: text("Specific heating load", "Spezifische Heizlast"), short: text("heating load", "Heizlast"), unit: "W/m²", scale: "logarithmic", prefixed: false, additive: false }, cards: [10, 120] },
    { id: "energy", quantity: { label: text("Annual heating energy", "Jährliche Heizenergie"), short: text("heating energy", "Heizenergie"), unit: "kWh", scale: "logarithmic", prefixed: true, additive: true }, cards: [1500, 40_000] },
    { id: "indoor", quantity: { label: text("Indoor temperature", "Raumtemperatur"), unit: "°C", scale: "linear", prefixed: false, additive: false }, cards: [20, 23] },
  ],
  items: [
    { id: "old", label: text("Unrenovated 1960s single-family house", "Unsaniertes Einfamilienhaus der 1960er-Jahre"), short: text("Old house", "Altbau") },
    { id: "passive", label: text("Passive house", "Passivhaus") },
  ],
};

/** 🪟️ The U-values of the second hint audit's first regression: an abbreviation that keeps its capital mid-sentence. */
const U_VALUES: SheetSortingTask = {
  kind: "sorting",
  id: "u-values",
  title: text("U-values", "U-Werte"),
  prompt: text("Sort by U-value.", "Nach U-Wert sortieren."),
  quantity: { label: text("U-value", "U-Wert"), unit: "W/(m²·K)", scale: "logarithmic", prefixed: false, additive: false },
  keys: [0.8, 5.8],
  items: [
    { id: "passive", label: text("Passive-house window", "Passivhausfenster") },
    { id: "old", label: text("Old aluminium or steel window", "Altes Aluminium- oder Stahlfenster") },
  ],
};

/** 🌬️ The air change rates of the second hint audit's second regression: items whose physical size is no answer. */
const AIR_CHANGES: SheetSortingTask = {
  kind: "sorting",
  id: "air-change-rates",
  title: text("Air change rates", "Luftwechselraten"),
  prompt: text("Sort by air change rate.", "Nach Luftwechselrate sortieren."),
  quantity: { label: text("Air change rate", "Luftwechselrate"), unit: "1/h", scale: "logarithmic", prefixed: false, additive: false },
  keys: [0.1, 5],
  items: [
    { id: "warehouse", label: text("High-bay warehouse", "Hochregallager") },
    { id: "car-park", label: text("Underground car park", "Tiefgarage") },
  ],
};

/** 🍫️ The energies of the second hint audit's third regression: a count beyond 10¹⁵. */
const ENERGIES: SheetSortingTask = {
  kind: "sorting",
  id: "energies",
  title: text("Energies", "Energien"),
  prompt: text("Sort by energy.", "Nach Energie sortieren."),
  quantity: { label: text("Energy", "Energie"), unit: "Wh", scale: "logarithmic", prefixed: true, additive: true },
  keys: [1e-12, 10_000],
  items: [
    { id: "chocolate", label: text("Food energy of a 100 g chocolate bar", "Brennwert einer 100-g-Tafel Schokolade") },
    { id: "oil", label: text("Litre of heating oil", "Liter Heizöl") },
  ],
};

/** 🏥️ The cooling matching of the second hint audit's fourth regression: two quantities named by capitalised shorts. */
const COOLING: SheetMatchingTask = {
  kind: "matching",
  id: "cooling",
  title: text("Cooling", "Kühlung"),
  prompt: text("Match each building its values.", "Ordne jedem Gebäude seine Werte zu."),
  dimensions: [
    { id: "load", quantity: { label: text("Specific cooling load", "Spezifische Kühllast"), short: text("Cooling load", "Kühllast"), unit: "W/m²", scale: "logarithmic", prefixed: false, additive: false }, cards: [5, 100] },
    { id: "demand", quantity: { label: text("Annual cooling demand", "Jahres-Kühlbedarf"), short: text("Cooling demand", "Kühlbedarf"), unit: "kWh/(m²·a)", scale: "logarithmic", prefixed: false, additive: false }, cards: [2, 60] },
  ],
  items: [
    { id: "hospital", label: text("Hospital with operating rooms", "Krankenhaus mit OP-Sälen") },
    { id: "attic", label: text("Attic flat under an uninsulated roof", "Dachwohnung unter ungedämmtem Dach") },
  ],
};

/** ❔️ Every wording branch of a hint and the question it asks, in English and German: a factor (oriented to read at
 * least 1: below 1 the items swap and it inverts) cut to two significant digits toward the claim — down where the claim
 * understates ("only"), up where it overstates ("really") — with the locale's grouping and decimal marks, a scale word
 * that agrees with its number from a million, and a two-digit mantissa times a power of ten from 10¹⁵; amounts that add
 * up asked about together, others as a ratio, a linear scale by the difference in its unit; a pair the keys have the
 * wrong way round by its order alone; every compare question in the quantity it is about, English writing its name
 * lower-case mid-sentence unless it starts with an abbreviation; a profile by another item on the axis or by the given
 * category's value on it with its unit and a true minus; a pair together or apart; a category with its description (its
 * full stop dropped) or without one. Items, categories, axes and quantities are named by their short forms where they
 * have them. The last eight are the regressions the second hint audit quotes (its worst 1–4). */
export const HINT_QUESTIONS: readonly { readonly locale: CourseLocale; readonly task: SheetTask; readonly hint: Hint; readonly question: string }[] = [
  { locale: "en", task: MASSES, hint: { kind: "compare", item: "horse", other: "mouse", factor: 1234.5, verdict: "under" }, question: "Are you sure 1,200 × “Mouse” together only add up to the mass of 1 × “Horse”?" },
  { locale: "de", task: MASSES, hint: { kind: "compare", item: "horse", other: "mouse", factor: 1234.5, verdict: "under" }, question: "Bist du sicher, dass 1.200 × „Maus“ in puncto Masse zusammen nur 1 × „Pferd“ ergeben?" },
  { locale: "en", task: MASSES, hint: { kind: "compare", item: "mouse", other: "horse", factor: 0.0008, verdict: "over" }, question: "Are you sure it takes 1,300 × “Mouse” to add up to the mass of 1 × “Horse”?" },
  { locale: "de", task: MASSES, hint: { kind: "compare", item: "mouse", other: "horse", factor: 0.0008, verdict: "over" }, question: "Bist du sicher, dass es in puncto Masse 1.300 × „Maus“ braucht, um 1 × „Pferd“ zu ergeben?" },
  { locale: "en", task: MASSES, hint: { kind: "compare", item: "horse", other: "mouse", factor: 0.001, verdict: "reversed" }, question: "Are you sure “Mouse” is higher in mass than “Horse”?" },
  { locale: "de", task: MASSES, hint: { kind: "compare", item: "horse", other: "mouse", factor: 0.001, verdict: "reversed" }, question: "Bist du sicher, dass „Maus“ in puncto Masse höher ist als „Pferd“?" },
  { locale: "en", task: RATIOS, hint: { kind: "compare", item: "cat", other: "mouse", factor: 2.96, verdict: "under" }, question: "Are you sure “Cat” is only 2.9 times as high in mass as “Mouse”?" },
  { locale: "de", task: RATIOS, hint: { kind: "compare", item: "cat", other: "mouse", factor: 2.96, verdict: "under" }, question: "Bist du sicher, dass „Katze“ in puncto Masse nur 2,9-mal so hoch ist wie „Maus“?" },
  { locale: "en", task: RATIOS, hint: { kind: "compare", item: "mouse", other: "cat", factor: 0.35, verdict: "over" }, question: "Are you sure “Cat” is really 2.9 times as high in mass as “Mouse”?" },
  { locale: "de", task: RATIOS, hint: { kind: "compare", item: "mouse", other: "cat", factor: 0.35, verdict: "over" }, question: "Bist du sicher, dass „Katze“ in puncto Masse tatsächlich 2,9-mal so hoch ist wie „Maus“?" },
  { locale: "en", task: RATIOS, hint: { kind: "compare", item: "mouse", other: "cat", factor: 3, verdict: "reversed" }, question: "Are you sure “Mouse” is higher in mass than “Cat”?" },
  { locale: "de", task: RATIOS, hint: { kind: "compare", item: "mouse", other: "cat", factor: 3, verdict: "reversed" }, question: "Bist du sicher, dass „Maus“ in puncto Masse höher ist als „Katze“?" },
  { locale: "en", task: RATIOS, hint: { kind: "compare", item: "cat", other: "mouse", factor: 1.2e6, verdict: "under" }, question: `Are you sure “Cat” is only 1.2${NBSP}million times as high in mass as “Mouse”?` },
  { locale: "de", task: RATIOS, hint: { kind: "compare", item: "cat", other: "mouse", factor: 1.2e6, verdict: "under" }, question: `Bist du sicher, dass „Katze“ in puncto Masse nur 1,2${NBSP}Millionen Mal so hoch ist wie „Maus“?` },
  { locale: "en", task: RATIOS, hint: { kind: "compare", item: "horse", other: "mouse", factor: 3.14e20, verdict: "over" }, question: `Are you sure “Horse” is really 3.2${NBSP}×${NBSP}10²⁰ times as high in mass as “Mouse”?` },
  { locale: "de", task: RATIOS, hint: { kind: "compare", item: "horse", other: "mouse", factor: 3.14e20, verdict: "over" }, question: `Bist du sicher, dass „Pferd“ in puncto Masse tatsächlich 3,2${NBSP}×${NBSP}10²⁰-mal so hoch ist wie „Maus“?` },
  { locale: "en", task: POWERS, hint: { kind: "compare", item: "sun", other: "tealight", factor: 1.1e26, verdict: "under" }, question: `Are you sure 1.1${NBSP}×${NBSP}10²⁶ × “Tea light” together only add up to the power of 1 × “Sun”?` },
  { locale: "de", task: POWERS, hint: { kind: "compare", item: "sun", other: "tealight", factor: 1.1e26, verdict: "under" }, question: `Bist du sicher, dass 1,1${NBSP}×${NBSP}10²⁶ × „Teelicht“ in puncto Leistung zusammen nur 1 × „Sonne“ ergeben?` },
  { locale: "en", task: POWERS, hint: { kind: "compare", item: "sun", other: "tealight", factor: 12_345_678, verdict: "under" }, question: `Are you sure 12${NBSP}million × “Tea light” together only add up to the power of 1 × “Sun”?` },
  { locale: "de", task: POWERS, hint: { kind: "compare", item: "sun", other: "tealight", factor: 12_345_678, verdict: "under" }, question: `Bist du sicher, dass 12${NBSP}Millionen × „Teelicht“ in puncto Leistung zusammen nur 1 × „Sonne“ ergeben?` },
  { locale: "en", task: POWERS, hint: { kind: "compare", item: "kettle", other: "sun", factor: 1e-6, verdict: "over" }, question: `Are you sure it takes 1${NBSP}million × “Kettle” to add up to the power of 1 × “Sun”?` },
  { locale: "de", task: POWERS, hint: { kind: "compare", item: "kettle", other: "sun", factor: 1e-6, verdict: "over" }, question: `Bist du sicher, dass es in puncto Leistung 1${NBSP}Million × „Wasserkocher“ braucht, um 1 × „Sonne“ zu ergeben?` },
  { locale: "en", task: POWERS, hint: { kind: "compare", item: "tealight", other: "sun", factor: 2 ** -32, verdict: "over" }, question: `Are you sure it takes 4.3${NBSP}billion × “Tea light” to add up to the power of 1 × “Sun”?` },
  { locale: "de", task: POWERS, hint: { kind: "compare", item: "tealight", other: "sun", factor: 2 ** -32, verdict: "over" }, question: `Bist du sicher, dass es in puncto Leistung 4,3${NBSP}Milliarden × „Teelicht“ braucht, um 1 × „Sonne“ zu ergeben?` },
  { locale: "en", task: POWERS, hint: { kind: "compare", item: "tealight", other: "sun", factor: 1e-25, verdict: "reversed" }, question: "Are you sure “Sun” is higher in power than “Tea light”?" },
  { locale: "de", task: POWERS, hint: { kind: "compare", item: "tealight", other: "sun", factor: 1e-25, verdict: "reversed" }, question: "Bist du sicher, dass „Sonne“ in puncto Leistung höher ist als „Teelicht“?" },
  { locale: "en", task: TEMPERATURES, hint: { kind: "compare", item: "tea", other: "ice", difference: 50, verdict: "under" }, question: `Are you sure “Tea” is only 50${NBSP}K higher in temperature than “Ice”?` },
  { locale: "de", task: TEMPERATURES, hint: { kind: "compare", item: "tea", other: "ice", difference: 50, verdict: "under" }, question: `Bist du sicher, dass „Tee“ in puncto Temperatur nur 50${NBSP}K höher ist als „Eis“?` },
  { locale: "en", task: TEMPERATURES, hint: { kind: "compare", item: "ice", other: "room", difference: -66.5, verdict: "over" }, question: `Are you sure “Room” is really 66.5${NBSP}K higher in temperature than “Ice”?` },
  { locale: "de", task: TEMPERATURES, hint: { kind: "compare", item: "ice", other: "room", difference: -66.5, verdict: "over" }, question: `Bist du sicher, dass „Raum“ in puncto Temperatur tatsächlich 66,5${NBSP}K höher ist als „Eis“?` },
  { locale: "en", task: TEMPERATURES, hint: { kind: "compare", item: "tea", other: "room", difference: -20, verdict: "reversed" }, question: "Are you sure “Room” is higher in temperature than “Tea”?" },
  { locale: "de", task: TEMPERATURES, hint: { kind: "compare", item: "tea", other: "room", difference: -20, verdict: "reversed" }, question: "Bist du sicher, dass „Raum“ in puncto Temperatur höher ist als „Tee“?" },
  { locale: "en", task: LAMPS, hint: { kind: "compare", item: "led", other: "floodlight", dimension: "power", factor: 0.004, verdict: "over" }, question: "Are you sure it takes 250 × “LED bulb” to add up to the power of 1 × “Floodlight”?" },
  { locale: "de", task: LAMPS, hint: { kind: "compare", item: "led", other: "floodlight", dimension: "power", factor: 0.004, verdict: "over" }, question: "Bist du sicher, dass es in puncto Leistung 250 × „LED-Lampe“ braucht, um 1 × „Flutlicht“ zu ergeben?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "load", factor: 2.96, verdict: "under" }, question: "Are you sure “Old house” is only 2.9 times as high in heating load as “Passive house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "load", factor: 2.96, verdict: "under" }, question: "Bist du sicher, dass „Altbau“ in puncto Heizlast nur 2,9-mal so hoch ist wie „Passivhaus“?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "load", factor: 0.35, verdict: "over" }, question: "Are you sure “Old house” is really 2.9 times as high in heating load as “Passive house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "load", factor: 0.35, verdict: "over" }, question: "Bist du sicher, dass „Altbau“ in puncto Heizlast tatsächlich 2,9-mal so hoch ist wie „Passivhaus“?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "load", factor: 12, verdict: "reversed" }, question: "Are you sure “Passive house” is higher in heating load than “Old house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "load", factor: 12, verdict: "reversed" }, question: "Bist du sicher, dass „Passivhaus“ in puncto Heizlast höher ist als „Altbau“?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "energy", factor: 12, verdict: "under" }, question: "Are you sure 12 × “Passive house” together only add up to the heating energy of 1 × “Old house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "energy", factor: 12, verdict: "under" }, question: "Bist du sicher, dass 12 × „Passivhaus“ in puncto Heizenergie zusammen nur 1 × „Altbau“ ergeben?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "energy", factor: 0.0004, verdict: "over" }, question: "Are you sure it takes 2,500 × “Passive house” to add up to the heating energy of 1 × “Old house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "energy", factor: 0.0004, verdict: "over" }, question: "Bist du sicher, dass es in puncto Heizenergie 2.500 × „Passivhaus“ braucht, um 1 × „Altbau“ zu ergeben?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "energy", factor: 30, verdict: "reversed" }, question: "Are you sure “Passive house” is higher in heating energy than “Old house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "energy", factor: 30, verdict: "reversed" }, question: "Bist du sicher, dass „Passivhaus“ in puncto Heizenergie höher ist als „Altbau“?" },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "indoor", difference: 2, verdict: "under" }, question: `Are you sure “Old house” is only 2${NBSP}°C higher in indoor temperature than “Passive house”?` },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "indoor", difference: 2, verdict: "under" }, question: `Bist du sicher, dass „Altbau“ in puncto Raumtemperatur nur 2${NBSP}°C höher ist als „Passivhaus“?` },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "indoor", difference: -6, verdict: "over" }, question: `Are you sure “Old house” is really 6${NBSP}°C higher in indoor temperature than “Passive house”?` },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "passive", other: "old", dimension: "indoor", difference: -6, verdict: "over" }, question: `Bist du sicher, dass „Altbau“ in puncto Raumtemperatur tatsächlich 6${NBSP}°C höher ist als „Passivhaus“?` },
  { locale: "en", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "indoor", difference: 3, verdict: "reversed" }, question: "Are you sure “Old house” is higher in indoor temperature than “Passive house”?" },
  { locale: "de", task: HOUSES, hint: { kind: "compare", item: "old", other: "passive", dimension: "indoor", difference: 3, verdict: "reversed" }, question: "Bist du sicher, dass „Altbau“ in puncto Raumtemperatur höher ist als „Passivhaus“?" },
  { locale: "en", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "rain" }, question: `Are you sure “Desert” fits Wet, with rainfall at about 1,500${NBSP}mm?` },
  { locale: "de", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "rain" }, question: `Bist du sicher, dass „Wüste“ zu Nass passt, mit Niederschlag bei rund 1.500${NBSP}mm?` },
  { locale: "en", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "frost" }, question: `Are you sure “Desert” fits Wet, with coldest month mean at about −2.5${NBSP}°C?` },
  { locale: "de", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "frost" }, question: `Bist du sicher, dass „Wüste“ zu Nass passt, mit Januarmittel bei rund −2,5${NBSP}°C?` },
  { locale: "en", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "rain", other: "steppe", above: true }, question: "Are you sure “Desert” is higher in rainfall than “Steppe”?" },
  { locale: "de", task: CLIMATES, hint: { kind: "profile", item: "desert", category: "wet", axis: "rain", other: "steppe", above: true }, question: "Bist du sicher, dass „Wüste“ in puncto Niederschlag höher ist als „Steppe“?" },
  { locale: "en", task: CLIMATES, hint: { kind: "profile", item: "fjord", category: "dry", axis: "sun", other: "desert", above: false }, question: "Are you sure “Fjord” is lower in sunshine than “Desert”?" },
  { locale: "de", task: CLIMATES, hint: { kind: "profile", item: "fjord", category: "dry", axis: "sun", other: "desert", above: false }, question: "Bist du sicher, dass „Fjord“ in puncto Sonnenschein niedriger ist als „Wüste“?" },
  { locale: "en", task: CLIMATES, hint: { kind: "group", item: "desert", other: "fjord", together: true }, question: "Are you sure “Desert” and “Fjord” belong to the same category?" },
  { locale: "de", task: CLIMATES, hint: { kind: "group", item: "desert", other: "fjord", together: true }, question: "Bist du sicher, dass „Wüste“ und „Fjord“ in dieselbe Kategorie gehören?" },
  { locale: "en", task: CLIMATES, hint: { kind: "group", item: "steppe", other: "desert", together: false }, question: "Are you sure “Steppe” and “Desert” belong to different categories?" },
  { locale: "de", task: CLIMATES, hint: { kind: "group", item: "steppe", other: "desert", together: false }, question: "Bist du sicher, dass „Steppe“ und „Wüste“ in verschiedene Kategorien gehören?" },
  { locale: "en", task: CLIMATES, hint: { kind: "category", item: "desert", category: "wet" }, question: "Are you sure “Desert” belongs to Wet — Rain on most days?" },
  { locale: "de", task: CLIMATES, hint: { kind: "category", item: "desert", category: "wet" }, question: "Bist du sicher, dass „Wüste“ zu Nass gehört — Regen an den meisten Tagen?" },
  { locale: "en", task: CLIMATES, hint: { kind: "category", item: "steppe", category: "dry" }, question: "Are you sure “Steppe” belongs to Dry?" },
  { locale: "de", task: CLIMATES, hint: { kind: "category", item: "steppe", category: "dry" }, question: "Bist du sicher, dass „Steppe“ zu Trocken gehört?" },
  { locale: "en", task: U_VALUES, hint: { kind: "compare", item: "old", other: "passive", factor: 0.0483, verdict: "reversed" }, question: "Are you sure “Passive-house window” is higher in U-value than “Old aluminium or steel window”?" },
  { locale: "de", task: U_VALUES, hint: { kind: "compare", item: "old", other: "passive", factor: 0.0483, verdict: "reversed" }, question: "Bist du sicher, dass „Passivhausfenster“ in puncto U-Wert höher ist als „Altes Aluminium- oder Stahlfenster“?" },
  { locale: "en", task: AIR_CHANGES, hint: { kind: "compare", item: "warehouse", other: "car-park", factor: 50, verdict: "reversed" }, question: "Are you sure “High-bay warehouse” is higher in air change rate than “Underground car park”?" },
  { locale: "de", task: AIR_CHANGES, hint: { kind: "compare", item: "warehouse", other: "car-park", factor: 50, verdict: "reversed" }, question: "Bist du sicher, dass „Hochregallager“ in puncto Luftwechselrate höher ist als „Tiefgarage“?" },
  { locale: "en", task: ENERGIES, hint: { kind: "compare", item: "oil", other: "chocolate", factor: 1.06e16, verdict: "over" }, question: `Are you sure it takes 1.1${NBSP}×${NBSP}10¹⁶ × “Food energy of a 100 g chocolate bar” to add up to the energy of 1 × “Litre of heating oil”?` },
  { locale: "de", task: ENERGIES, hint: { kind: "compare", item: "oil", other: "chocolate", factor: 1.06e16, verdict: "over" }, question: `Bist du sicher, dass es in puncto Energie 1,1${NBSP}×${NBSP}10¹⁶ × „Brennwert einer 100-g-Tafel Schokolade“ braucht, um 1 × „Liter Heizöl“ zu ergeben?` },
  { locale: "en", task: COOLING, hint: { kind: "compare", item: "hospital", other: "attic", dimension: "load", factor: 20.4, verdict: "over" }, question: "Are you sure “Hospital with operating rooms” is really 21 times as high in cooling load as “Attic flat under an uninsulated roof”?" },
  { locale: "de", task: COOLING, hint: { kind: "compare", item: "hospital", other: "attic", dimension: "load", factor: 20.4, verdict: "over" }, question: "Bist du sicher, dass „Krankenhaus mit OP-Sälen“ in puncto Kühllast tatsächlich 21-mal so hoch ist wie „Dachwohnung unter ungedämmtem Dach“?" },
];

/** 🔢️ The tolerance factor the results name for `keys` on a logarithmic scale, worked out independently of the core and of the
 * client's formatting: the square root of the ratio of the largest to the smallest, at most 1000, cut down (never
 * rounded up) to two significant digits by plain arithmetic. */
export function factorOf(keys: readonly number[], locale: "en" | "de" = "en"): string {
  const reachOf = Math.min(1000, Math.sqrt(Math.max(...keys) / Math.min(...keys)));
  const step = 10 ** (Math.floor(Math.log10(reachOf)) - 1);
  return (Math.floor(reachOf / step) * step).toLocaleString(locale);
}

/** 📢️ What the polite region of a task's hints last said (its first region speaks its moves). */
export function hinted(): string {
  return screen.getAllByRole("status")[1]?.textContent ?? "";
}

/** 🗣️ Everything the polite regions on screen say now. */
export function said(): readonly string[] {
  return screen.getAllByRole("status").map((region) => region.textContent ?? "");
}
