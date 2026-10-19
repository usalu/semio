/** 🧠️ The run state every harness module shares: the CLI-derived configuration, the clock, the current locale and step, and the browser handles. Reads are live bindings; every write goes through the setter beside the variable. */
import type { Browser, BrowserContext, Page } from "playwright";
import type { Locale, Renderer, TimeTravelScenarioV1 } from "../🧩️scenario/🟦️.ts";
export type { Locale, Renderer };

//#region 🔖️Arguments

export type Point = { x: number; y: number };

export let renderer: Renderer = "react";

/** ✏️ Replaces `renderer`; the state module is the only place the binding is assigned. */
export const setRenderer = (value: typeof renderer): void => {
  renderer = value;
};

export let serveOrigin = "";

/** ✏️ Replaces `serveOrigin`; the state module is the only place the binding is assigned. */
export const setServeOrigin = (value: typeof serveOrigin): void => {
  serveOrigin = value;
};

export let only: Set<number> | null = null;

/** ✏️ Replaces `only`; the state module is the only place the binding is assigned. */
export const setOnly = (value: typeof only): void => {
  only = value;
};

export let reloadSelected = false;

/** ✏️ Replaces `reloadSelected`; the state module is the only place the binding is assigned. */
export const setReloadSelected = (value: typeof reloadSelected): void => {
  reloadSelected = value;
};

export let locales: Locale[] = ["en", "de"];

/** ✏️ Replaces `locales`; the state module is the only place the binding is assigned. */
export const setLocales = (value: typeof locales): void => {
  locales = value;
};

export let chordLocales = new Set<string>(["en", "de"]);

/** ✏️ Replaces `chordLocales`; the state module is the only place the binding is assigned. */
export const setChordLocales = (value: typeof chordLocales): void => {
  chordLocales = value;
};

export let explore = false;

/** ✏️ Replaces `explore`; the state module is the only place the binding is assigned. */
export const setExplore = (value: typeof explore): void => {
  explore = value;
};

export let folderAt = 1;

/** ✏️ Replaces `folderAt`; the state module is the only place the binding is assigned. */
export const setFolderAt = (value: typeof folderAt): void => {
  folderAt = value;
};

export let longHistory = 200;

/** ✏️ Replaces `longHistory`; the state module is the only place the binding is assigned. */
export const setLongHistory = (value: typeof longHistory): void => {
  longHistory = value;
};

export let OUT = "";

/** ✏️ Replaces `OUT`; the state module is the only place the binding is assigned. */
export const setOUT = (value: typeof OUT): void => {
  OUT = value;
};

export let stamp = "";

/** ✏️ Replaces `stamp`; the state module is the only place the binding is assigned. */
export const setStamp = (value: typeof stamp): void => {
  stamp = value;
};

export let base = "";

/** ✏️ Replaces `base`; the state module is the only place the binding is assigned. */
export const setBase = (value: typeof base): void => {
  base = value;
};

export let ndjsonPath = "";

/** ✏️ Replaces `ndjsonPath`; the state module is the only place the binding is assigned. */
export const setNdjsonPath = (value: typeof ndjsonPath): void => {
  ndjsonPath = value;
};

export let t0 = Date.now();

/** ✏️ Replaces `t0`; the state module is the only place the binding is assigned. */
export const setT0 = (value: typeof t0): void => {
  t0 = value;
};

export let cancelled: AbortSignal = new AbortController().signal;

/** ✏️ Replaces `cancelled`; the state module is the only place the binding is assigned. */
export const setCancelled = (value: typeof cancelled): void => {
  cancelled = value;
};
//#endregion 🔖️Arguments
//#region 🔖️Report

export let currentLocale: Locale = "en";

/** ✏️ Replaces `currentLocale`; the state module is the only place the binding is assigned. */
export const setCurrentLocale = (value: typeof currentLocale): void => {
  currentLocale = value;
};

export let currentStep = 0;

/** ✏️ Replaces `currentStep`; the state module is the only place the binding is assigned. */
export const setCurrentStep = (value: typeof currentStep): void => {
  currentStep = value;
};
//#endregion 🔖️Report
//#region 🔖️Page

export let browser: Browser;

/** ✏️ Replaces `browser`; the state module is the only place the binding is assigned. */
export const setBrowser = (value: typeof browser): void => {
  browser = value;
};

export let context: BrowserContext;

/** ✏️ Replaces `context`; the state module is the only place the binding is assigned. */
export const setContext = (value: typeof context): void => {
  context = value;
};

export let page: Page;

/** ✏️ Replaces `page`; the state module is the only place the binding is assigned. */
export const setPage = (value: typeof page): void => {
  page = value;
};

export let mod = "Meta";

/** ✏️ Replaces `mod`; the state module is the only place the binding is assigned. */
export const setMod = (value: typeof mod): void => {
  mod = value;
};

export let navigations = 0;

/** ✏️ Replaces `navigations`; the state module is the only place the binding is assigned. */
export const setNavigations = (value: typeof navigations): void => {
  navigations = value;
};

export let expectedReloads = 0;

/** ✏️ Replaces `expectedReloads`; the state module is the only place the binding is assigned. */
export const setExpectedReloads = (value: typeof expectedReloads): void => {
  expectedReloads = value;
};

export let probeReloadAt = 0;

/** ✏️ Replaces `probeReloadAt`; the state module is the only place the binding is assigned. */
export const setProbeReloadAt = (value: typeof probeReloadAt): void => {
  probeReloadAt = value;
};

export let invalidatedBy: string | null = null;

/** ✏️ Replaces `invalidatedBy`; the state module is the only place the binding is assigned. */
export const setInvalidatedBy = (value: typeof invalidatedBy): void => {
  invalidatedBy = value;
};

export let wgpuSeedTerminology = false;

/** ✏️ Replaces `wgpuSeedTerminology`; the state module is the only place the binding is assigned. */
export const setWgpuSeedTerminology = (value: typeof wgpuSeedTerminology): void => {
  wgpuSeedTerminology = value;
};

export let universalRoute: string | null = null;

/** ✏️ Replaces `universalRoute`; the state module is the only place the binding is assigned. */
export const setUniversalRoute = (value: typeof universalRoute): void => {
  universalRoute = value;
};

export let universalEditor: string | null = null;

/** ✏️ Replaces `universalEditor`; the state module is the only place the binding is assigned. */
export const setUniversalEditor = (value: typeof universalEditor): void => {
  universalEditor = value;
};

export let lastBootFault = "";

/** ✏️ Replaces `lastBootFault`; the state module is the only place the binding is assigned. */
export const setLastBootFault = (value: typeof lastBootFault): void => {
  lastBootFault = value;
};

export type Device = { key: string; viewport: { width: number; height: number }; isMobile: boolean; hasTouch: boolean; windows: number };

export type Peer = { context: BrowserContext; page: Page };

/** 🧩️ The scenario of the running pass; absent for a universal run. */
export let scenario: TimeTravelScenarioV1<object> | undefined = undefined;

/** ✏️ Replaces `scenario`. */
export const setScenario = (value: TimeTravelScenarioV1<object> | undefined): void => {
  scenario = value;
};
//#endregion 🔖️Page
