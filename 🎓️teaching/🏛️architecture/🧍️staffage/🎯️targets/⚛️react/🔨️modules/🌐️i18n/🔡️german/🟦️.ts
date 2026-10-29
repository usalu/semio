/** 🔡️ The German bundle: every part of the app's own texts, merged; the compiler checks it against English key for key. @see ../🟦️.ts — the translator */
import type { ENGLISH } from "../🔠️english/🟦️.ts";
import { COLLECTIONS_DE } from "./⭐️collections.ts";
import { COMPARE_DE } from "./⚖️compare.ts";
import { CORE_DE } from "./🧱️core.ts";
import { DETAIL_DE } from "./📖️detail.ts";
import { PLAYGROUND_DE } from "./🧪️playground.ts";
import { VIEWER_DE } from "./🎛️viewer.ts";

export const GERMAN: Record<keyof typeof ENGLISH, string> = { ...CORE_DE, ...COLLECTIONS_DE, ...COMPARE_DE, ...DETAIL_DE, ...PLAYGROUND_DE, ...VIEWER_DE };
