/** 📃️ Console and page-error capture of every page and worker the run opens. */
import type { ConsoleMessage, Page } from "playwright";
import { t0, currentLocale, currentStep } from "../🧠️state/🟦️.ts";
import type { Locale } from "../🧠️state/🟦️.ts";

//#region 🔖️Console

export type ConsoleRow = { t: number; locale: Locale; step: number; source: "page" | "worker"; type: string; text: string };

export const consoleRows: ConsoleRow[] = [];

export const pageErrors: { locale: Locale; step: number; text: string }[] = [];

export const hardFaults: { locale: Locale; step: number; text: string }[] = [];

export const BENIGN_RE = /contributions document sources|staged plugin module|\[stale\]|activate-[a-z0-9]+-react-prototype|transform freshness|Download the React DevTools/i;

const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked/i;

/** 📥️ Keeps one console line (page or worker) in the ring and classifies it as a hard fault. */
const keepConsole = (source: ConsoleRow["source"], message: ConsoleMessage) => {
  const row: ConsoleRow = { t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source, type: message.type(), text: message.text().slice(0, 2000) };
  consoleRows.push(row);
  if (consoleRows.length > 20000) consoleRows.shift();
  if (HARD_FAULT_RE.test(row.text) && !BENIGN_RE.test(row.text)) hardFaults.push({ locale: row.locale, step: row.step, text: row.text.slice(0, 400) });
};

/** 🎧️ Subscribes one page and every worker it spawns: page lines that belong to a worker are left to that worker's
 * own listener, so no line is counted twice. The dev serve's HMR socket is read too: its `full-reload` and `error` frames are kept
 * (type `vite`), so a navigation under a step can be attributed ({@link navigationCause}). */
export const listen = (target: Page) => {
  target.on("console", (message) => {
    if (message.worker?.()) return;
    keepConsole("page", message);
  });
  target.on("worker", (worker) => worker.on("console", (message) => keepConsole("worker", message)));
  target.on("websocket", (socket) =>
    socket.on("framereceived", (frame) => {
      const text = typeof frame.payload === "string" ? frame.payload : "";
      if (!/"type":"(full-reload|error)"/u.test(text)) return;
      consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "vite", text: text.slice(0, 600) });
    }),
  );
  target.on("response", (response) => {
    if (response.url().includes("/semio-backbone")) consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "backbone", text: `HTTP ${response.status()} ${response.request().method()} ${decodeURIComponent(response.url()).slice(0, 400)}` });
    if (response.status() < 400) return;
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "http", text: `HTTP ${response.status()} ${response.request().method()} ${response.url().slice(0, 300)}` });
  });
  target.on("pageerror", (error) => {
    const text = String(error?.stack ?? error).slice(0, 1200);
    pageErrors.push({ locale: currentLocale, step: currentStep, text });
    consoleRows.push({ t: Number(((Date.now() - t0) / 1000).toFixed(1)), locale: currentLocale, step: currentStep, source: "page", type: "pageerror", text });
  });
};

/** 🔓️ `text` with its percent-escapes decoded (stack frames carry `@fs` URLs of emoji paths); unchanged when it is not decodable. */
export const readableUrlText = (text: string) => {
  try {
    return decodeURIComponent(text);
  } catch {
    return text;
  }
};

/** 🛰️ Why the page just navigated: the dev serve told it to (a `full-reload` frame on the HMR socket within the last 3 s — a served
 * file was saved; its `triggeredBy` / `path` names the file) or nothing did, in which case the PRODUCT reloaded its own page. */
export const navigationCause = () => {
  const now = (Date.now() - t0) / 1000;
  const frame = [...consoleRows].reverse().find((row) => row.type === "vite" && row.text.includes("full-reload") && now - row.t <= 3);
  return frame ? `dev serve full-reload ${frame.text.slice(0, 300)}` : "no dev-serve reload frame in the 3 s before it: the page reloaded itself";
};
//#endregion 🔖️Console
