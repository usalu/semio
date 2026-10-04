/** 🪜️ The challenges in the client. The shared sheets of one seed at every challenge show each task kind with its keys
 * (easy, medium) or ask for guesses (hard, expert); guesses complete an answer; a key is read with its place; an easy
 * run's hints are questions about two concrete items in every wording branch, in English and German with exact
 * strings — named by their short forms, counts in words up to a power of ten, a reversed pair by its order alone, the
 * quantity named where a matching has several — stand beside the item they doubt with a question mark in a circle, describe its controls and are spoken
 * once when they appear or their question changes — several at once by their count; a timed task runs behind its clock —
 * a timer read in words, it starts on request, speaks at the start, at 30 s, at 10 s and when the time is up (never
 * when shown already late), ticks whatever the device says about motion and without rendering the task again, looks
 * again when the page shows, turns read-only at the end without moving focus, says when an entry was lost, and ends its
 * timers with the screen; every timed task's step shows its clock, and a task whose time runs out while another is
 * shown is told once; a timed run can be submitted at any time naming the tasks still open and their clocks; locked
 * controls say why; during a run that hides the keys the others' answers are not offered; and the results name the
 * challenge and the points, show guesses with their miss mark and what was not answered. The tolerance factor the
 * results name is worked out independently of the core (square root of the spread's ratio, at most 1000) and cut down to two digits
 * by plain arithmetic; accessible descriptions are computed by `dom-accessibility-api`.
 *
 * @see ../../🧫️fixtures/🃏️sheet-assembly/🔣️.json — `icons-1` at every challenge
 * @see ../../🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx — the clock
 */

import { useMemo, useState, type ReactElement } from "react";
import { act, cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { computeAccessibleDescription } from "dom-accessibility-api";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  answerComplete,
  type Answer,
  type Challenge,
  type Hint,
  type MatchingAnswer,
  type RunResult,
  type RunView,
  type Sheet,
  type SheetClassificationTask,
  type SheetMatchingTask,
  type SheetSortingTask,
  type SheetTask,
  type SortingAnswer,
} from "@semio-tech/quiz";
import { ClassificationTaskView, MatchingTaskView, ResultsScreen, RunScreen, SortingTaskView, TaskView, classificationHintText, clockStage, formatCountdown, formatDuration, formatQuantity, formatScore, hintTerm, initialQuizState, quizText, type OthersChoice, type QuizLocale, type QuizSession, type QuizText } from "@semio-tech/quiz-react";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";

const text = (en: string, de: string) => ({ en, de });
const en = quizText("en");
const NBSP = " ";

/** 🃏️ The shared sheet `icons-1` at `challenge` (its medium form has no suffix). */
function sheetAt(challenge: Challenge): Sheet {
  const id = challenge === "medium" ? "icons-1" : `icons-1-${challenge}`;
  return sheets.sheets.find((vector) => vector.id === id)!.sheet as unknown as Sheet;
}

const MASSES: SheetSortingTask = {
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

const TEMPERATURES: SheetSortingTask = {
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

const LAMPS: SheetMatchingTask = {
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

const PLACES: SheetClassificationTask = {
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
const CLIMATES: SheetClassificationTask = {
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
const POWERS: SheetSortingTask = {
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
const HOUSES: SheetMatchingTask = {
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
const HINT_QUESTIONS: readonly { readonly locale: QuizLocale; readonly task: SheetTask; readonly hint: Hint; readonly question: string }[] = [
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
function factorOf(keys: readonly number[], locale: "en" | "de" = "en"): string {
  const reachOf = Math.min(1000, Math.sqrt(Math.max(...keys) / Math.min(...keys)));
  const step = 10 ** (Math.floor(Math.log10(reachOf)) - 1);
  return (Math.floor(reachOf / step) * step).toLocaleString(locale);
}

/** 📢️ What the polite region of a task's hints last said (its first region speaks its moves). */
function hinted(): string {
  return screen.getAllByRole("status")[1]?.textContent ?? "";
}

/** 🗣️ Everything the polite regions on screen say now. */
function said(): readonly string[] {
  return screen.getAllByRole("status").map((region) => region.textContent ?? "");
}

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe("🪜️ every task kind at every challenge", () => {
  for (const challenge of ["easy", "medium", "hard", "expert"] as const) {
    it(`shows the keys or asks for guesses on ${challenge}`, () => {
      const shown = challenge === "easy" || challenge === "medium";
      for (const task of sheetAt(challenge).tasks) {
        const { unmount } = render(<TaskView task={task} answer={undefined} onAnswer={() => undefined} text={en} locale="en" />);
        if (task.kind === "sorting") {
          expect(task.keys !== undefined, task.id).toBe(shown);
          expect([...document.querySelectorAll(".quiz-sort-key")].map((key) => key.textContent)).toEqual(shown ? task.keys!.map((key) => formatQuantity(key, task.quantity, "en")) : []);
          expect(screen.queryAllByRole("textbox")).toHaveLength(shown ? 0 : task.items.length);
          expect(screen.queryAllByRole("button", { name: /^Move / })).toHaveLength(shown ? 2 * task.items.length : 0);
          if (!shown) expect(screen.getByText(`Guessed: 0 of ${task.items.length}`)).toBeTruthy();
        }
        if (task.kind === "matching") {
          expect(screen.queryAllByRole("list", { name: /^Value cards: / })).toHaveLength(shown ? task.dimensions.length : 0);
          expect(screen.queryAllByRole("combobox")).toHaveLength(shown ? task.dimensions.length * task.items.length : 0);
          expect(screen.queryAllByRole("textbox")).toHaveLength(shown ? 0 : task.dimensions.length * task.items.length);
          expect(document.querySelectorAll('.quiz-match[data-keys="hidden"]')).toHaveLength(shown ? 0 : task.dimensions.length);
        }
        if (task.kind === "classification") expect(screen.getAllByRole("combobox")).toHaveLength(task.items.length);
        unmount();
      }
    });
  }

  it("completes a sorting and a matching whose keys are hidden once every item has a guess", async () => {
    const user = userEvent.setup();
    const hard = sheetAt("hard");
    for (const task of hard.tasks.filter((candidate) => candidate.kind !== "classification")) {
      const answers: Answer[] = [];
      function Harness(): ReactElement {
        const [answer, setAnswer] = useState<Answer | undefined>(undefined);
        return (
          <TaskView
            task={task}
            answer={answer}
            onAnswer={(next) => {
              answers.push(next);
              setAnswer(next);
            }}
            text={en}
            locale="en"
          />
        );
      }
      const { unmount } = render(<Harness />);
      const fields = screen.getAllByRole("textbox");
      for (const [index, field] of fields.entries()) {
        expect(answerComplete(task, answers.at(-1))).toBe(false);
        await user.type(screen.getAllByRole("textbox")[index] ?? field, `${index + 2}{Enter}`);
      }
      expect(answerComplete(task, answers.at(-1)), task.id).toBe(true);
      expect(Object.keys(answers.at(-1)!)).toEqual(["kind", task.kind === "sorting" ? "order" : "guesses", ...(task.kind === "sorting" ? ["guesses"] : [])]);
      unmount();
    }
  });
});

describe("💡️ hints of an easy run", () => {
  for (const { locale, task, hint, question } of HINT_QUESTIONS) {
    it(`asks “${question}” beside the item it doubts, with a question mark in a circle, describing that item's controls`, () => {
      const answer: Answer =
        task.kind === "sorting"
          ? { kind: "sorting", order: task.items.map((item) => item.id) }
          : task.kind === "matching"
            ? { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item, index) => [item.id, index]))])) }
            : { kind: "classification", assignments: { desert: "wet", fjord: "wet", steppe: "dry" } };
      render(<TaskView task={task} answer={answer} hints={[hint]} onAnswer={() => undefined} text={quizText(locale)} locale={locale} />);
      const notes = document.querySelectorAll<HTMLElement>(".quiz-hint");
      expect(notes).toHaveLength(1);
      const note = notes[0]!;
      expect(note.getAttribute("data-hint")).toBe(hint.kind);
      expect(note.querySelector(".quiz-hint-question")?.textContent).toBe(question);
      expect([note.querySelector(".quiz-hint-symbol")?.textContent, note.querySelector(".quiz-hint-symbol")?.getAttribute("aria-hidden")]).toEqual(["?", "true"]);
      expect(note.closest("li")?.getAttribute("data-presence-anchor")).toBe(`item:${hint.item}`);
      const label = (task.items.find((item) => item.id === hint.item)?.label ?? text("", ""))[locale];
      const controls = task.kind === "sorting" ? within(note.closest("li")!).getAllByRole("button") : within(note.closest("li")!).getAllByRole("combobox");
      expect(controls.length).toBeGreaterThan(0);
      for (const control of controls) expect(computeAccessibleDescription(control), label).toBe(question);
      expect(document.body.textContent).not.toMatch(/≪|≫|far too|viel zu|wrong category|falschen Kategorie|as much as|ganze|\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
      expect(question).not.toMatch(/\d{7}|\d[,.]\d{3}[,.]\d{3}|-\d/u);
    });
  }

  it("leaves every other item's controls undescribed", () => {
    render(<SortingTaskView task={MASSES} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"] }} hints={[{ kind: "compare", item: "horse", other: "mouse", factor: 200, verdict: "under" }]} onAnswer={() => undefined} text={en} locale="en" />);
    for (const name of ["Move Mouse up", "Move Cat down"]) expect(screen.getByRole("button", { name }).getAttribute("aria-describedby")).toBeNull();
  });

  it("names things in its question by their short forms while the rows, bins and controls keep their labels", () => {
    const { unmount } = render(<SortingTaskView task={POWERS} answer={{ kind: "sorting", order: ["tealight", "kettle", "sun"] }} hints={[{ kind: "compare", item: "sun", other: "tealight", factor: 1.1e26, verdict: "under" }]} onAnswer={() => undefined} text={en} locale="en" />);
    expect(screen.getByRole("button", { name: "Move Total radiant power of the Sun up" })).toBeTruthy();
    expect(document.querySelector(".quiz-hint-question")?.textContent).toBe(`Are you sure 1.1${NBSP}×${NBSP}10²⁶ × “Tea light” together only add up to the power of 1 × “Sun”?`);
    unmount();
    render(<ClassificationTaskView task={CLIMATES} answer={{ kind: "classification", assignments: { desert: "wet" } }} hints={[{ kind: "category", item: "desert", category: "wet" }]} onAnswer={() => undefined} text={en} locale="en" />);
    expect(screen.getByRole("heading", { name: "Wet climate zone" })).toBeTruthy();
    expect(computeAccessibleDescription(screen.getByRole("combobox", { name: "Category for Hot desert of the Sahara" }))).toBe("Are you sure “Desert” belongs to Wet — Rain on most days?");
  });

  it("names the quantity on a matching of several columns, so one pair asks a different question in each", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { load: { old: 0, passive: 1 }, energy: { old: 0, passive: 1 }, indoor: { old: 0, passive: 1 } } };
    const hints: readonly Hint[] = [
      { kind: "compare", item: "old", other: "passive", dimension: "load", factor: 0.083, verdict: "reversed" },
      { kind: "compare", item: "old", other: "passive", dimension: "energy", factor: 0.0375, verdict: "reversed" },
    ];
    const { rerender } = render(<MatchingTaskView task={HOUSES} answer={answer} hints={[]} onAnswer={() => undefined} text={en} locale="en" />);
    rerender(<MatchingTaskView task={HOUSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />);
    expect(hinted()).toBe("New hints: 2");
    expect(["Specific heating load of Unrenovated 1960s single-family house", "Annual heating energy of Unrenovated 1960s single-family house"].map((name) => computeAccessibleDescription(screen.getByRole("combobox", { name })))).toEqual([
      "Are you sure “Passive house” is higher in heating load than “Old house”?",
      "Are you sure “Passive house” is higher in heating energy than “Old house”?",
    ]);
  });

  it("writes a quantity's or an axis's name lower-case mid-sentence in English unless its second letter is a capital, and as it is in German", () => {
    const names = [
      [text("Heating demand", "Heizwärmebedarf"), "heating demand", "Heizwärmebedarf"],
      [text("U-value", "U-Wert"), "U-value", "U-Wert"],
      [text("CO₂ emissions", "CO₂-Emissionen"), "CO₂ emissions", "CO₂-Emissionen"],
      [text("PV yield", "PV-Ertrag"), "PV yield", "PV-Ertrag"],
      [text("Ärger", "Ärger"), "ärger", "Ärger"],
      [text("Power", "Leistung"), "power", "Leistung"],
      [text("n", "n"), "n", "n"],
    ] as const;
    for (const [label, english, german] of names) expect([hintTerm({ label }, "en"), hintTerm({ label }, "de")], label.en).toEqual([english, german]);
    expect(hintTerm({ label: text("Specific cooling load", "Spezifische Kühllast"), short: text("Cooling load", "Kühllast") }, "en")).toBe("cooling load");
  });

  it("asks the profile questions the second hint audit quotes with the axis lower-case in English", () => {
    const profiles: SheetClassificationTask = {
      kind: "classification",
      id: "profiles",
      title: text("Profiles", "Profile"),
      prompt: text("Assign each standard its profile.", "Ordne jedem Standard sein Profil zu."),
      axes: [{ id: "heating", label: text("Heating demand", "Heizwärmebedarf"), unit: "kWh/(m²·a)", min: 0, max: 400 }],
      categories: [
        { id: "a", label: text("Profile A", "Profil A"), profile: { heating: 15 } },
        { id: "b", label: text("Profile B", "Profil B"), profile: { heating: 303 } },
      ],
      items: [
        { id: "enev", label: text("New build to EnEV 2014", "Neubau nach EnEV 2014") },
        { id: "passive", label: text("Passive house", "Passivhaus") },
      ],
    };
    const asked = (hint: Hint): readonly string[] => (["en", "de"] as const).map((locale) => classificationHintText(profiles, hint, quizText(locale), locale));
    expect(asked({ kind: "profile", item: "enev", category: "b", axis: "heating" })).toEqual([
      `Are you sure “New build to EnEV 2014” fits Profile B, with heating demand at about 303${NBSP}kWh/(m²·a)?`,
      `Bist du sicher, dass „Neubau nach EnEV 2014“ zu Profil B passt, mit Heizwärmebedarf bei rund 303${NBSP}kWh/(m²·a)?`,
    ]);
    expect(asked({ kind: "profile", item: "enev", category: "b", axis: "heating", other: "passive", above: true })).toEqual([
      "Are you sure “New build to EnEV 2014” is higher in heating demand than “Passive house”?",
      "Bist du sicher, dass „Neubau nach EnEV 2014“ in puncto Heizwärmebedarf höher ist als „Passivhaus“?",
    ]);
  });

  it("reads every key with its place, never as the value of the item standing there", () => {
    render(<SortingTaskView task={MASSES} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"] }} onAnswer={() => undefined} text={en} locale="en" />);
    const rows = within(screen.getByRole("list", { name: "Order by Mass" })).getAllByRole("listitem");
    expect(rows.map((row) => row.querySelector(".quiz-sort-place .sr-only")?.textContent)).toEqual([`Place 1: 20${NBSP}g`, `Place 2: 4${NBSP}kg`, `Place 3: 500${NBSP}kg`]);
    for (const row of rows) {
      expect(row.querySelector(".quiz-sort-key")?.getAttribute("aria-hidden")).toBe("true");
      expect(row.querySelector(".quiz-sort-place")?.getAttribute("aria-hidden")).toBeNull();
    }
  });

  it("speaks a hint once when it appears, not those a task already has, and again when it comes back", () => {
    const answer: SortingAnswer = { kind: "sorting", order: ["horse", "mouse", "cat"] };
    const low: readonly Hint[] = [{ kind: "compare", item: "horse", other: "mouse", factor: 200, verdict: "under" }];
    const view = (hints: readonly Hint[]): ReactElement => <SortingTaskView task={MASSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const shown = render(view(low));
    expect(hinted()).toBe("");
    shown.unmount();
    const { rerender } = render(view([]));
    expect(hinted()).toBe("");
    rerender(view(low));
    const spoken = "Are you sure 200 × “Mouse” together only add up to the mass of 1 × “Horse”?";
    expect(hinted()).toBe(spoken);
    const first = screen.getAllByRole("status")[1]!.firstElementChild;
    rerender(view([...low]));
    expect(screen.getAllByRole("status")[1]!.firstElementChild).toBe(first);
    rerender(view([]));
    rerender(view(low));
    expect(hinted()).toBe(spoken);
    expect(screen.getAllByRole("status")[1]!.firstElementChild).not.toBe(first);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("");
  });

  it("speaks several hints that appear at once only by their count, leaving their questions beside their items", () => {
    const answer: SortingAnswer = { kind: "sorting", order: ["horse", "cat", "mouse"] };
    const view = (hints: readonly Hint[]): ReactElement => <SortingTaskView task={MASSES} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const { rerender } = render(view([]));
    const three: readonly Hint[] = [
      { kind: "compare", item: "horse", other: "cat", factor: 0.008, verdict: "over" },
      { kind: "compare", item: "cat", other: "mouse", factor: 25000, verdict: "over" },
      { kind: "compare", item: "mouse", other: "cat", factor: 125, verdict: "over" },
    ];
    rerender(view(three));
    expect(hinted()).toBe("New hints: 3");
    expect(document.querySelectorAll(".quiz-hint")).toHaveLength(3);
    rerender(view(three.slice(0, 1)));
    rerender(view(three.slice(0, 2)));
    expect(hinted()).toBe("Are you sure it takes 25,000 × “Mouse” to add up to the mass of 1 × “Cat”?");
  });

  it("asks again when a hint's question changes — another reference or another claimed factor — and not when it stays", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { power: { led: 2, halogen: 1, floodlight: 0 } } };
    const view = (hints: readonly Hint[]): ReactElement => <MatchingTaskView task={LAMPS} answer={answer} hints={hints} onAnswer={() => undefined} text={en} locale="en" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "compare", item: "led", other: "halogen", dimension: "power", factor: 250, verdict: "over" }]));
    expect(hinted()).toBe("Are you sure it takes 250 × “Halogen spot” to add up to the power of 1 × “LED bulb”?");
    const first = screen.getAllByRole("status")[1]!.firstElementChild;
    rerender(view([{ kind: "compare", item: "led", other: "halogen", dimension: "power", factor: 250, verdict: "over" }]));
    expect(screen.getAllByRole("status")[1]!.firstElementChild).toBe(first);
    rerender(view([{ kind: "compare", item: "led", other: "floodlight", dimension: "power", factor: 40, verdict: "over" }]));
    expect(hinted()).toBe("Are you sure it takes 40 × “Floodlight” to add up to the power of 1 × “LED bulb”?");
    expect(screen.getByRole("combobox", { name: "Power of Floodlight" }).getAttribute("aria-describedby")).toBeNull();
  });

  it("draws the profiles of a classification whose keys are hidden as shapes, with no descriptions and no numbers", () => {
    render(<ClassificationTaskView task={PLACES} answer={undefined} onAnswer={() => undefined} text={en} locale="en" />);
    const image = screen.getByRole("img", { name: "Spider diagram of Dry" });
    const percent = (share: number): string => formatScore(share, "en");
    expect(document.getElementById(image.getAttribute("aria-describedby") ?? "")?.textContent).toBe(`Rain: ${percent(0.05)}; Sun: ${percent(0.9)}; Heat: ${percent(0.8)}. The values follow as a table.`);
    expect(screen.queryByText("Minimum")).toBeNull();
  });
});

describe("🇩🇪 hints and guesses in German", () => {
  const de = quizText("de");
  const raw = /\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u;

  it("speaks a classification question in German once it appears, and several new ones by their count, each beside its item", () => {
    const answer = { kind: "classification", assignments: { desert: "wet", fjord: "wet", steppe: "wet" } } as const;
    const view = (hints: readonly Hint[]): ReactElement => <ClassificationTaskView task={CLIMATES} answer={answer} hints={hints} onAnswer={() => undefined} text={de} locale="de" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun" }]));
    expect(hinted()).toBe(`Bist du sicher, dass „Wüste“ zu Nass passt, mit Sonnenschein bei rund 1.200${NBSP}h?`);
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun", other: "fjord", above: false }]));
    expect(hinted()).toBe("Bist du sicher, dass „Wüste“ in puncto Sonnenschein niedriger ist als „Fjord“?");
    rerender(view([{ kind: "profile", item: "desert", category: "wet", axis: "sun", other: "fjord", above: false }, { kind: "group", item: "fjord", other: "steppe", together: true }, { kind: "category", item: "steppe", category: "wet" }]));
    expect(hinted()).toBe("Neue Hinweise: 2");
    expect([...document.querySelectorAll(".quiz-hint")].map((note) => [note.closest("li")?.getAttribute("data-quiz-item"), note.querySelector(".quiz-hint-question")?.textContent])).toEqual([
      ["desert", "Bist du sicher, dass „Wüste“ in puncto Sonnenschein niedriger ist als „Fjord“?"],
      ["fjord", "Bist du sicher, dass „Fjord“ und „Steppe“ in dieselbe Kategorie gehören?"],
      ["steppe", "Bist du sicher, dass „Steppe“ zu Nass gehört — Regen an den meisten Tagen?"],
    ]);
    expect(document.body.textContent).not.toMatch(raw);
  });

  it("speaks a reversed pair of a matching of several columns in German by its order alone, naming the quantity, with the question mark beside it", () => {
    const answer: MatchingAnswer = { kind: "matching", assignments: { load: { old: 0, passive: 1 }, energy: { old: 1, passive: 0 }, indoor: { old: 0, passive: 1 } } };
    const view = (hints: readonly Hint[]): ReactElement => <MatchingTaskView task={HOUSES} answer={answer} hints={hints} onAnswer={() => undefined} text={de} locale="de" />;
    const { rerender } = render(view([]));
    rerender(view([{ kind: "compare", item: "passive", other: "old", dimension: "load", factor: 12, verdict: "reversed" }]));
    const question = "Bist du sicher, dass „Passivhaus“ in puncto Heizlast höher ist als „Altbau“?";
    expect(hinted()).toBe(question);
    const note = document.querySelector(".quiz-hint")!;
    expect([note.getAttribute("data-hint"), note.querySelector(".quiz-hint-symbol")?.textContent, note.querySelector(".quiz-hint-question")?.textContent]).toEqual(["compare", "?", question]);
    expect(computeAccessibleDescription(screen.getByRole("combobox", { name: "Spezifische Heizlast von Passivhaus" }))).toBe(question);
    expect(document.body.textContent).not.toMatch(raw);
  });

  it("names every guess field in German, with its example in the unit, and refuses an unreadable guess with a German message", async () => {
    const user = userEvent.setup();
    const { keys: _masses, ...masses } = MASSES;
    const { keys: _temperatures, ...temperatures } = TEMPERATURES;
    const lamps: SheetMatchingTask = { ...LAMPS, dimensions: LAMPS.dimensions.map(({ cards: _cards, ...dimension }) => dimension) };
    const cases = [
      { view: <SortingTaskView task={masses} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Pferd", example: `2${NBSP}kg`, invalid: `Gib eine positive Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}kg.`, guide: "Gib für jedes Element deinen Schätzwert ein, zum Beispiel 2 kg. Die Elemente ordnen sich nach deinen Schätzwerten." },
      { view: <SortingTaskView task={temperatures} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Eis", example: `2${NBSP}K`, invalid: `Gib eine Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}K.`, guide: "Gib für jedes Element deinen Schätzwert ein, zum Beispiel 2 K. Die Elemente ordnen sich nach deinen Schätzwerten." },
      { view: <MatchingTaskView task={lamps} answer={undefined} onAnswer={() => undefined} text={de} locale="de" />, field: "Schätzwert für Leistung von LED-Lampe", example: `2${NBSP}kW`, invalid: `Gib eine positive Zahl ein, optional mit SI-Vorsatz und Einheit, zum Beispiel 2${NBSP}kW.`, guide: "Gib für jedes Element und jede Größe deinen Schätzwert ein; jedes Feld zeigt ein Beispiel in seiner Einheit." },
    ];
    for (const { view, field, example, invalid, guide } of cases) {
      const shown = render(view);
      expect(screen.getByText(guide)).toBeTruthy();
      expect(screen.getByText("Geschätzt: 0 von 3")).toBeTruthy();
      const input = screen.getByRole("textbox", { name: field });
      expect(input.getAttribute("placeholder"), field).toBe(`z. B. ${example}`);
      await user.type(input, "viel{Enter}");
      expect(input.getAttribute("aria-invalid"), field).toBe("true");
      expect(screen.getByRole("alert").textContent, field).toBe(invalid);
      expect(computeAccessibleDescription(input), field).toBe(invalid);
      await user.clear(input);
      await user.type(input, "-3");
      expect(input.getAttribute("aria-invalid"), field).toBe("false");
      expect(document.body.textContent, field).not.toMatch(raw);
      shown.unmount();
    }
  });
});

describe("🔒️ a task whose time is up", () => {
  it("keeps its guesses read-only and takes none", async () => {
    const answers: Answer[] = [];
    const { keys: _keys, ...hidden } = MASSES;
    render(<SortingTaskView task={hidden} answer={{ kind: "sorting", order: ["horse", "mouse", "cat"], guesses: { horse: 500_000 } }} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    const horse = screen.getByRole("textbox", { name: "Guess for Horse" }) as HTMLInputElement;
    expect(horse.readOnly).toBe(true);
    const user = userEvent.setup();
    await user.type(horse, "9{Enter}");
    await user.type(screen.getByRole("textbox", { name: "Guess for Cat" }), "4 kg{Enter}");
    expect(horse.value).toBe(`500${NBSP}kg`);
    expect(answers).toEqual([]);
  });

  it("refuses to move by its buttons, saying why, and keeps focus on the button", async () => {
    const answers: Answer[] = [];
    render(<SortingTaskView task={MASSES} answer={undefined} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    expect(screen.queryByRole("button", { name: "Keep this order" })).toBeNull();
    const down = screen.getByRole("button", { name: "Move Horse down" });
    expect(down.getAttribute("aria-disabled")).toBe("true");
    down.focus();
    await userEvent.setup().keyboard("{Enter}");
    expect(answers).toEqual([]);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("Time is up: this task takes no more answers.");
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Move Horse down" }));
    expect(computeAccessibleDescription(screen.getByRole("button", { name: "Move Horse down" }))).toBe("Time is up: this task takes no more answers.");
  });

  it("describes its locked selects by why, and says it again when the arrow keys try to change one", () => {
    vi.useFakeTimers();
    const answers: Answer[] = [];
    render(<ClassificationTaskView task={PLACES} answer={{ kind: "classification", assignments: { desert: "dry" } }} locked onAnswer={(answer) => answers.push(answer)} text={en} locale="en" />);
    const select = screen.getByRole("combobox", { name: "Category for Desert" }) as HTMLSelectElement;
    expect(select.getAttribute("aria-disabled")).toBe("true");
    expect(computeAccessibleDescription(select)).toBe("Time is up: this task takes no more answers.");
    act(() => {
      fireEvent.change(select, { target: { value: "wet" } });
      vi.advanceTimersByTime(400);
    });
    expect(answers).toEqual([]);
    expect(select.value).toBe("dry");
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("Time is up: this task takes no more answers.");
  });

  it("hands on a guess typed but not committed when the lock comes, and loses one that does not read as a guess", async () => {
    const { keys: _keys, ...hidden } = MASSES;
    const answers: Answer[] = [];
    let dropped = 0;
    const view = (locked: boolean): ReactElement => <SortingTaskView task={hidden} answer={undefined} locked={locked} onAnswer={(answer) => answers.push(answer)} onDropped={() => (dropped += 1)} text={en} locale="en" />;
    const shown = render(view(false));
    const user = userEvent.setup();
    await user.type(screen.getByRole("textbox", { name: "Guess for Horse" }), "500 kg");
    shown.rerender(view(true));
    expect(answers).toEqual([{ kind: "sorting", order: ["horse", "mouse", "cat"], guesses: { horse: 500_000 } }]);
    expect(dropped).toBe(0);
    expect(screen.getAllByRole("status")[0]!.textContent).toBe("");
    shown.unmount();
    const again = render(view(false));
    await user.type(screen.getByRole("textbox", { name: "Guess for Cat" }), "heavy");
    again.rerender(view(true));
    expect(answers).toHaveLength(1);
    expect(dropped).toBe(1);
    expect((screen.getByRole("textbox", { name: "Guess for Cat" }) as HTMLInputElement).value).toBe("");
  });
});

describe("⏱️ the clock of a timed task", () => {
  it("is closed until opened, runs, warns in its last 30 and 10 seconds and is up at zero", () => {
    expect([undefined, 90_000, 30_001, 30_000, 10_001, 10_000, 1, 0].map(clockStage)).toEqual(["closed", "running", "running", "thirty", "thirty", "ten", "ten", "up"]);
  });

  /** 🎭️ A run screen over a session double whose clock the test turns: opening a task stamps it with the clock. */
  function Run(props: { readonly view: RunView; readonly clock: { now: number }; readonly calls: string[]; readonly others?: OthersChoice; readonly text?: QuizText; readonly locale?: "en" | "de" }): ReactElement {
    const { clock, calls } = props;
    const [view, setView] = useState(props.view);
    const session = useMemo(
      () =>
        ({
          now: () => clock.now,
          openTask: (run: string, task: string, signal: AbortSignal) => {
            calls.push(`open ${task} ${signal.aborted}`);
            setView((present) => ({ ...present, opened: { ...present.opened, [task]: clock.now } }));
            return Promise.resolve(undefined);
          },
          answer: (run: string, task: string, answer: Answer) => {
            calls.push(`answer ${task}`);
            setView((present) => ({ ...present, answers: { ...present.answers, [task]: answer } }));
          },
          askCrowd: () => undefined,
          unaskCrowd: () => undefined,
          submit: () => Promise.resolve(undefined),
        }) as unknown as QuizSession,
      [clock, calls],
    );
    return <RunScreen session={session} state={initialQuizState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others={props.others} text={props.text ?? en} locale={props.locale ?? "en"} />;
  }

  const runOf = (sheet: Sheet, opened: Readonly<Record<string, number>> = {}): RunView => ({ run: "run-1", learner: "learner-1", quiz: sheet.quiz, status: "open", sheet, answers: {}, startedAt: 1_000_000, ...(sheet.challenge === "expert" ? { opened } : {}) });

  /** 🔔️ What the task's polite status beside its clock last said. */
  const clockSaid = (): string => document.querySelector(".quiz-clock")?.parentElement?.querySelector('[role="status"]')?.textContent ?? "";

  /** 👣️ What the step of the task at `index` says of its clock: its stage, what is shown and what is read. */
  const stepClock = (index: number): readonly (string | null | undefined)[] => {
    const state = document.querySelectorAll(".quiz-step")[index]?.querySelector(".quiz-clock-state");
    return [state?.getAttribute("data-clock-state"), state?.querySelector(":scope > [aria-hidden='true']:not(:first-child)")?.textContent ?? state?.textContent, state?.querySelector(".sr-only")?.textContent];
  };

  /** ⏩️ Turns the session clock and the fake timers on by whole seconds, one act per second. */
  const ticker =
    (clock: { now: number }) =>
    (seconds: number): void => {
      for (let step = 0; step < seconds; step += 1)
        act(() => {
          clock.now += 1000;
          vi.advanceTimersByTime(1000);
        });
    };

  /** ▶️ Starts the clock of the task shown. */
  const start = async (): Promise<void> => {
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Start the clock" }));
    });
  };

  it("starts on request, ticks as text that is no live region, speaks at the start, at 30 s, at 10 s and at time up, then locks without moving focus — on a device that asks for reduced motion too", async () => {
    vi.useFakeTimers();
    vi.stubGlobal("matchMedia", (query: string) => ({ matches: query.includes("reduce"), media: query, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, onchange: null, dispatchEvent: () => false }));
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks as readonly [SheetClassificationTask, SheetMatchingTask];
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    const advance = ticker(clock);
    const started = `The clock runs: ${first.seconds} seconds left`;
    render(<Run view={runOf(sheet)} clock={clock} calls={calls} />);
    expect(screen.getByText("Challenge: Expert")).toBeTruthy();
    expect(screen.getByText(`Time allowed: 0:${first.seconds}`)).toBeTruthy();
    expect(document.querySelector(".quiz-task")).toBeNull();
    expect(screen.queryByText(first.prompt.en)).toBeNull();
    expect(stepClock(0)).toEqual(["closed", "⏸ Clock not started", undefined]);
    await start();
    expect(calls).toEqual([`open ${first.id} false`]);
    expect(clockSaid()).toBe(started);
    expect(document.activeElement?.tagName).toMatch(/^H[1-6]$/u);
    expect(document.activeElement?.textContent).toContain(first.title.en);
    const shown = document.querySelector<HTMLElement>(".quiz-clock")!;
    expect([shown.getAttribute("data-clock"), shown.getAttribute("aria-live"), shown.getAttribute("role")]).toEqual(["running", null, "timer"]);
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Time left: 0:${first.seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Time left: ${first.seconds} seconds`);
    expect(stepClock(0)).toEqual(["running", `Time left: 0:${first.seconds}`, `Time left: ${first.seconds} seconds`]);
    expect(screen.getByText(first.prompt.en)).toBeTruthy();
    advance(1);
    expect(document.querySelector(".quiz-clock")!.textContent).toContain(`Time left: 0:${first.seconds! - 1}`);
    expect(document.querySelector(".quiz-clock .sr-only")!.textContent).toBe(`Time left: ${first.seconds! - 1} seconds`);
    expect(stepClock(0)[1]).toBe(`Time left: 0:${first.seconds! - 1}`);
    expect(clockSaid()).toBe(started);
    advance(first.seconds! - 32);
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Time left: 0:31");
    expect(clockSaid()).toBe(started);
    advance(1);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("thirty");
    expect(clockSaid()).toBe("30 seconds left");
    advance(20);
    expect(clockSaid()).toBe("10 seconds left");
    const select = screen.getAllByRole("combobox")[0]! as HTMLSelectElement;
    select.focus();
    act(() => {
      fireEvent.change(select, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toHaveLength(1);
    const focused = document.activeElement;
    expect(focused?.tagName).toBe("SELECT");
    advance(10);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Time is up");
    expect(document.activeElement).toBe(focused);
    expect(focused?.getAttribute("aria-disabled")).toBe("true");
    act(() => {
      fireEvent.change(screen.getAllByRole("combobox")[1]!, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toHaveLength(1);
    advance(5);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(screen.getByText(`Time allowed: ${Math.floor(second.seconds! / 60)}:${String(second.seconds! % 60).padStart(2, "0")}`)).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Previous task" }));
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(clockSaid()).toBe("");
  });

  it("says every stage of the clock in German: closed, started, running, 30 s, 10 s and up, with the lock that says why", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0] as SheetClassificationTask;
    const seconds = first.seconds!;
    const clock = { now: 2_000_000 };
    const advance = ticker(clock);
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} text={quizText("de")} locale="de" />);
    expect(screen.getByText("Herausforderung: Experte")).toBeTruthy();
    expect(screen.getByText(`Verfügbare Zeit: 0:${seconds}`)).toBeTruthy();
    expect(screen.getByText("Diese Aufgabe läuft gegen die Uhr. Du siehst sie, sobald du die Uhr startest.")).toBeTruthy();
    expect(stepClock(0)).toEqual(["closed", "⏸ Uhr nicht gestartet", undefined]);
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: "Uhr starten" }));
    });
    expect(clockSaid()).toBe(`Die Uhr läuft: noch ${formatDuration(seconds * 1000, "de")}`);
    const shown = document.querySelector<HTMLElement>(".quiz-clock")!;
    expect(shown.querySelector("[aria-hidden='true']:not(:first-child)")?.textContent).toBe(`Verbleibende Zeit: 0:${seconds}`);
    expect(shown.querySelector(".sr-only")?.textContent).toBe(`Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`);
    expect(stepClock(0)).toEqual(["running", `Verbleibende Zeit: 0:${seconds}`, `Verbleibende Zeit: ${formatDuration(seconds * 1000, "de")}`]);
    advance(seconds - 30);
    expect(clockSaid()).toBe("Noch 30 Sekunden");
    advance(20);
    expect(clockSaid()).toBe("Noch 10 Sekunden");
    advance(10);
    expect(clockSaid()).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.querySelector(".quiz-clock")!.textContent).toContain("Die Zeit ist um");
    expect(stepClock(0)[0]).toBe("up");
    expect(computeAccessibleDescription(screen.getAllByRole("combobox")[0]!)).toBe("Die Zeit ist um: Diese Aufgabe nimmt keine Antworten mehr an.");
    expect(document.body.textContent).not.toMatch(/\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
  });

  it("lets a timed run be submitted at any time and names the tasks still open in the confirmation with where their clocks stand", () => {
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet, { [first!.id]: clock.now - first!.seconds! * 1000 - 1, [second!.id]: clock.now - 5_000 })} clock={clock} calls={[]} />);
    const submit = screen.getByRole("button", { name: "Submit quiz" });
    expect(submit.getAttribute("aria-disabled")).toBe("false");
    fireEvent.click(submit);
    const dialog = screen.getByRole("alertdialog");
    expect(within(dialog).getByText("Not answered or incomplete, scored as missed:")).toBeTruthy();
    const left = formatCountdown(second!.seconds! * 1000 - 5_000);
    const words = formatDuration(second!.seconds! * 1000 - 5_000, "en");
    expect(within(dialog).getAllByRole("listitem").map((item) => item.textContent)).toEqual([
      `○ 1. ${first!.title.en} · ⌛ Time is up`,
      `○ 2. ${second!.title.en} · ⏱ Time left: ${left}Time left: ${words}`,
      ...sheet.tasks.slice(2).map((task, index) => `○ ${index + 3}. ${task.title.en} · ⏸ Clock not started`),
    ]);
  });

  it("tells once when the time of a task runs out while another is shown, and shows it in that task's step", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    const advance = ticker(clock);
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} />);
    await start();
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(screen.getByRole("button", { name: "Start the clock" })).toBeTruthy();
    expect(stepClock(1)[0]).toBe("closed");
    advance(first!.seconds! - 1);
    expect(stepClock(0)).toEqual(["ten", "Time left: 0:01", "Time left: 1 second"]);
    expect(said().filter((message) => message !== "")).toEqual([]);
    advance(1);
    expect(stepClock(0)).toEqual(["up", "⌛ Time is up", undefined]);
    expect(said().filter((message) => message !== "")).toEqual(["Task 1: time is up"]);
    const told = screen.getAllByRole("status").find((region) => region.textContent === "Task 1: time is up")!.firstElementChild;
    advance(3);
    expect(screen.getAllByRole("status").find((region) => region.textContent === "Task 1: time is up")!.firstElementChild).toBe(told);
    await start();
    expect(stepClock(1)[0]).toBe("running");
    advance(second!.seconds!);
    expect(said().filter((message) => message !== "")).toEqual(["Task 1: time is up", "Time is up: this task takes no more answers."]);
  });

  it("speaks nothing when it shows a clock already in its last 30 seconds or already up", () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const [first, second] = sheet.tasks;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet, { [first!.id]: clock.now - (first!.seconds! - 20) * 1000, [second!.id]: clock.now - second!.seconds! * 1000 })} clock={clock} calls={[]} />);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("thirty");
    expect(said().filter((message) => message !== "")).toEqual([]);
    ticker(clock)(5);
    expect(said().filter((message) => message !== "")).toEqual([]);
    fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(second!.kind).toBe("matching");
    expect(screen.getAllByRole("textbox").map((field) => (field as HTMLInputElement).readOnly)).not.toContain(false);
    expect(said().filter((message) => message !== "")).toEqual([]);
  });

  it("takes no answer between the deadline and the lock, and says the entry was lost", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0] as SheetClassificationTask;
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    render(<Run view={runOf(sheet)} clock={clock} calls={calls} />);
    await start();
    ticker(clock)(first.seconds! - 1);
    clock.now += 1000;
    act(() => {
      fireEvent.change(screen.getAllByRole("combobox")[0]!, { target: { value: first.categories[0]!.id } });
    });
    expect(calls.filter((call) => call.startsWith("answer"))).toEqual([]);
    expect(screen.getByText("Your last entry was not saved.")).toBeTruthy();
    act(() => {
      vi.advanceTimersByTime(1000);
    });
    expect(clockSaid()).toBe("Time is up: this task takes no more answers. Your last entry was not saved.");
  });

  it("loses a guess typed but not committed when the time runs out, and says so", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const matching = sheet.tasks.findIndex((task) => task.kind === "matching");
    const task = sheet.tasks[matching]!;
    const clock = { now: 2_000_000 };
    const calls: string[] = [];
    render(<Run view={runOf(sheet, { [task.id]: clock.now })} clock={clock} calls={calls} />);
    for (let step = 0; step < matching; step += 1) fireEvent.click(screen.getByRole("button", { name: "Next task" }));
    fireEvent.change(screen.getAllByRole("textbox")[0]!, { target: { value: "12 W" } });
    ticker(clock)(task.seconds!);
    expect(calls).toEqual([]);
    expect(clockSaid()).toBe("Time is up: this task takes no more answers. Your last entry was not saved.");
    expect((screen.getAllByRole("textbox")[0] as HTMLInputElement).value).toBe("");
  });

  it("looks at the clock again when the page shows again, though no timer fired while it was hidden", async () => {
    vi.useFakeTimers();
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0]!;
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} />);
    await start();
    clock.now += first.seconds! * 1000;
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("running");
    act(() => {
      document.dispatchEvent(new Event("visibilitychange"));
    });
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("up");
    expect(stepClock(0)[0]).toBe("up");
    expect(clockSaid()).toBe("Time is up: this task takes no more answers.");
  });

  it("ends every timer and listener of its clocks with the screen, also while a clock runs", async () => {
    vi.useFakeTimers();
    const added = vi.spyOn(document, "addEventListener");
    const removed = vi.spyOn(document, "removeEventListener");
    const visibility = (spy: typeof added): number => spy.mock.calls.filter(([type]) => type === "visibilitychange").length;
    const clock = { now: 2_000_000 };
    const { unmount } = render(<Run view={runOf(sheetAt("expert"))} clock={clock} calls={[]} />);
    await start();
    ticker(clock)(3);
    expect(document.querySelector(".quiz-clock")!.getAttribute("data-clock")).toBe("running");
    expect(visibility(added)).toBeGreaterThanOrEqual(4);
    unmount();
    expect(visibility(removed)).toBe(visibility(added));
    act(() => {
      vi.runOnlyPendingTimers();
    });
    expect(vi.getTimerCount()).toBe(0);
  });

  it("ticks only the clocks: the task renders again once, when its time is up", async () => {
    vi.useFakeTimers();
    let renders = 0;
    const counting: QuizText = (key, values) => {
      if (key === "quiz.classification.categories") renders += 1;
      return en(key, values);
    };
    const sheet = sheetAt("expert");
    const first = sheet.tasks[0]!;
    expect(first.kind).toBe("classification");
    const clock = { now: 2_000_000 };
    render(<Run view={runOf(sheet)} clock={clock} calls={[]} text={counting} />);
    await start();
    const before = renders;
    ticker(clock)(first.seconds! - 1);
    expect(renders).toBe(before);
    ticker(clock)(1);
    expect(renders).toBe(before + 1);
  });

  it("keeps an untimed run as it was: no clock, and no submission before every task is complete", () => {
    render(<Run view={runOf(sheetAt("medium"))} clock={{ now: 2_000_000 }} calls={[]} />);
    expect(screen.getByText("Challenge: Medium")).toBeTruthy();
    expect(document.querySelector(".quiz-clock")).toBeNull();
    expect(screen.queryByRole("button", { name: "Start the clock" })).toBeNull();
    expect(screen.getByRole("button", { name: "Submit quiz" }).getAttribute("aria-disabled")).toBe("true");
  });

  it("offers the others' answers during a run that shows the keys, never during one that hides them", () => {
    const { unmount } = render(<Run view={runOf(sheetAt("medium"))} clock={{ now: 2_000_000 }} calls={[]} others="submitted" />);
    expect(document.querySelector("[data-crowd-gate]")).not.toBeNull();
    unmount();
    for (const challenge of ["hard", "expert"] as const) {
      const shown = render(<Run view={runOf(sheetAt(challenge))} clock={{ now: 2_000_000 }} calls={[]} others="always" />);
      expect(document.querySelector("[data-crowd-gate]"), challenge).toBeNull();
      expect(document.querySelector("[data-crowd]"), challenge).toBeNull();
      shown.unmount();
    }
  });
});

describe("🏁️ results by challenge", () => {
  const submitted = (sheet: Sheet, result: RunResult): RunView => ({ run: "run-1", learner: "learner-1", quiz: sheet.quiz, status: "submitted", sheet, answers: {}, result, startedAt: 1_000_000, submittedAt: 1_100_000 });
  const results = (view: RunView): void => {
    render(<ResultsScreen session={{ open: () => undefined } as unknown as QuizSession} state={initialQuizState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others="never" text={en} locale="en" />);
  };
  const columns = (table: HTMLElement): readonly string[] => within(table).getAllByRole("columnheader").map((head) => head.textContent ?? "");
  const cellsOf = (table: HTMLElement, label: string): readonly string[] => [...table.querySelectorAll(`td[data-label="${label}"]`)].map((cell) => cell.textContent ?? "");
  const verdictsOf = (table: HTMLElement, label: string): readonly string[] =>
    [...table.querySelectorAll(`td[data-label="${label}"]`)].map((cell) => {
      const copy = cell.cloneNode(true) as HTMLElement;
      copy.querySelector("[data-off]")?.remove();
      return copy.textContent ?? "";
    });
  const offsOf = (table: HTMLElement): readonly string[] => [...table.querySelectorAll("[data-off]")].map((off) => off.textContent ?? "");

  it("names the challenge and the points, shows guesses with a miss mark where the keys were hidden and what was left unanswered", () => {
    const sheet = sheetAt("expert");
    const [sources, appliances, masses] = sheet.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
    const category = sources.categories[0]!.id;
    const result: RunResult = {
      quiz: sheet.quiz,
      challenge: "expert",
      score: 0.652,
      points: 260.8,
      tasks: [
        { kind: "classification", task: sources.id, score: 2 / 3, items: sources.items.map((item, index) => (index === 0 ? { item: item.id, correct: category, credit: 0 } : { item: item.id, assigned: category, correct: category, credit: 1 })) },
        {
          kind: "matching",
          task: appliances.id,
          score: 1 / 3,
          dimensions: appliances.dimensions.map((dimension) => ({
            dimension: dimension.id,
            score: 1 / 3,
            items: appliances.items.map((item, index) => (index === 0 ? { item: item.id, assigned: 12, correct: 10, miss: false } : index === 1 ? { item: item.id, assigned: 50_000, correct: 10, miss: true } : { item: item.id, correct: 10, miss: true })),
          })),
        },
        {
          kind: "sorting",
          task: masses.id,
          score: 0,
          items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: index, miss: index !== 0, ...(index === 2 ? {} : { guess: index === 0 ? 1 : 1e6 }) })),
        },
      ],
    };
    results(submitted(sheet, result));
    expect(screen.getByText(`Your score: ${formatScore(0.652, "en")}`)).toBeTruthy();
    expect(screen.getByText("Expert · Points: 260.8 of 400")).toBeTruthy();
    const tables = screen.getAllByRole("table");
    const [classification, ...rest] = tables;
    expect(cellsOf(classification!, "Your answer")[0]).toBe("Not answered");
    const sorting = rest.at(-1)!;
    expect(columns(sorting)).toContain("Your guess");
    expect(verdictsOf(sorting, "Your guess")).toEqual([`${formatQuantity(1, masses.quantity, "en")} ≈ Not far off`, `${formatQuantity(1e6, masses.quantity, "en")} ≉ Far off`, "Not answered"]);
    expect(offsOf(sorting)).toEqual(["×1 from the true value", "×100,000 from the true value"]);
    for (const [index, dimension] of appliances.dimensions.entries()) {
      const table = rest[index]!;
      expect(columns(table)).toEqual(["Item", "Your guess", "Solution", "Explanation"]);
      expect(verdictsOf(table, "Your guess")).toEqual([`${formatQuantity(12, dimension.quantity, "en")} ≈ Not far off`, `${formatQuantity(50_000, dimension.quantity, "en")} ≉ Far off`, "Not answered"]);
      expect(offsOf(table)).toHaveLength(2);
    }
    expect([...document.querySelectorAll(".quiz-miss")].map((mark) => mark.textContent)).toEqual([...appliances.dimensions.map(() => "≉ Far off"), "≉ Far off"]);
    for (const mark of document.querySelectorAll(".quiz-miss [aria-hidden]")) expect(mark.textContent).toBe("≉ ");
  });

  it("says where the keys were hidden how far a guess may lie off — ×N on a logarithmic scale, ±d on a linear one — and how far each guess lay, in English and German", () => {
    const { keys: _masses, ...masses } = MASSES;
    const { keys: _temperatures, ...temperatures } = TEMPERATURES;
    const lamps: SheetMatchingTask = { ...LAMPS, dimensions: LAMPS.dimensions.map(({ cards: _cards, ...dimension }) => dimension) };
    const sheet = { quiz: "guesses", seed: 7, challenge: "hard", title: text("Guesses", "Schätzungen"), description: text("Guess.", "Schätze."), tasks: [masses, temperatures, lamps] } as Sheet;
    const result: RunResult = {
      quiz: "guesses",
      challenge: "hard",
      score: 0.5,
      points: 150,
      tasks: [
        { kind: "sorting", task: "masses", score: 0.5, items: [{ item: "mouse", value: 20, position: 0, rank: 0, guess: 30, miss: false }, { item: "cat", value: 4000, position: 1, rank: 1, guess: 4000, miss: false }, { item: "horse", value: 500_000, position: 2, rank: 2, guess: 5e9, miss: true }] },
        { kind: "sorting", task: "temperatures", score: 0.5, items: [{ item: "ice", value: 270, position: 0, rank: 0, guess: 280, miss: false }, { item: "room", value: 293, position: 1, rank: 1, miss: true }, { item: "tea", value: 350, position: 2, rank: 2, guess: 600, miss: true }] },
        { kind: "matching", task: "lamps", score: 0.5, dimensions: [{ dimension: "power", score: 0.5, items: [{ item: "led", assigned: 10, correct: 8, miss: false }, { item: "halogen", assigned: 5000, correct: 50, miss: true }, { item: "floodlight", correct: 2000, miss: true }] }] },
      ],
    };
    const kelvin = (amount: number, locale: "en" | "de"): string => formatQuantity(amount, TEMPERATURES.quantity, locale);
    const cases = [
      { locale: "en", within: (bound: string) => `A guess counts within ${bound} of the true value.`, off: (amount: string) => `${amount} from the true value`, miss: "≉ Far off", unanswered: "Not answered" },
      { locale: "de", within: (bound: string) => `Ein Schätzwert zählt, wenn er höchstens ${bound} vom wahren Wert abweicht.`, off: (amount: string) => `${amount} vom wahren Wert entfernt`, miss: "≉ Weit daneben", unanswered: "Nicht beantwortet" },
    ] as const;
    for (const { locale, within: line, off, miss, unanswered } of cases) {
      const shown = render(<ResultsScreen session={{ open: () => undefined } as unknown as QuizSession} state={initialQuizState({ introduced: true, runs: { "run-1": submitted(sheet, result) } })} run="run-1" others="never" text={quizText(locale)} locale={locale} />);
      const lines = [`×${factorOf([20, 4000, 500_000], locale)}`, `±${kelvin((350 - 270) / 2, locale)}`, `×${factorOf([8, 50, 2000], locale)}`].map(line);
      expect([...document.querySelectorAll("[data-tolerance]")].map((note) => note.textContent), locale).toEqual(lines);
      expect(screen.getAllByRole("table").map((table) => computeAccessibleDescription(table)), locale).toEqual(lines);
      expect(offsOf(document.body), locale).toEqual([`×${(30 / 20).toLocaleString(locale)}`, "×1", `×${(10_000).toLocaleString(locale)}`, kelvin(280 - 270, locale), kelvin(600 - 350, locale), `×${(1.2).toLocaleString(locale)}`, `×${(5000 / 50).toLocaleString(locale)}`].map(off));
      expect([...document.querySelectorAll(".quiz-miss")].map((mark) => mark.textContent), locale).toEqual([miss, miss, miss]);
      expect(screen.getAllByText(unanswered), locale).toHaveLength(2);
      expect(document.body.textContent, locale).not.toMatch(/\{\{|quiz\.[a-z]+\.[a-zA-Z]+/u);
      shown.unmount();
    }
  });

  it("reads every position of a timed sorting nobody ordered as not answered, with no verdict on it", () => {
    const sheet = sheetAt("expert");
    const masses = sheet.tasks.find((task): task is SheetSortingTask => task.kind === "sorting")!;
    const result: RunResult = { quiz: sheet.quiz, challenge: "expert", score: 0, points: 0, tasks: [{ kind: "sorting", task: masses.id, score: 0, items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: masses.items.length - 1 - index, miss: true })) }] };
    results(submitted(sheet, result));
    const table = screen.getByRole("table");
    expect(cellsOf(table, "Your position")).toEqual(masses.items.map(() => "Not answered"));
    expect(cellsOf(table, "Correct position")).toEqual(masses.items.map((_, index) => String(masses.items.length - index)));
  });

  it("names the credit of an item and the points of a run by different German words", () => {
    const de = quizText("de");
    expect(de("quiz.results.credit")).toBe("Beurteilung");
    expect(de("quiz.challenge.scored", { challenge: "Schwer", points: 1, par: 300 })).toContain("Punkte");
    expect(de("quiz.results.credit")).not.toMatch(/Punkt/u);
  });

  it("shows no guess where the keys were shown", () => {
    const sheet = sheetAt("medium");
    const masses = sheet.tasks.find((task): task is SheetSortingTask => task.kind === "sorting")!;
    const result: RunResult = { quiz: sheet.quiz, challenge: "medium", score: 1, points: 200, tasks: [{ kind: "sorting", task: masses.id, score: 1, items: masses.items.map((item, index) => ({ item: item.id, value: masses.keys![index]!, position: index, rank: index })) }] };
    results(submitted(sheet, result));
    expect(screen.getByText("Medium · Points: 200 of 200")).toBeTruthy();
    expect(columns(screen.getByRole("table"))).not.toContain("Your guess");
    expect(document.querySelector(".quiz-miss")).toBeNull();
  });
});
