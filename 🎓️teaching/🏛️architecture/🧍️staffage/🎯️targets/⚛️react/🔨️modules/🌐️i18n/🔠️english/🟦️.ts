/** 🔠️ The English bundle: every part of the app's own texts, merged; keys are unique across parts. @see ../🟦️.ts — the translator */
import { COLLECTIONS_EN } from "./⭐️collections.ts";
import { COMPARE_EN } from "./⚖️compare.ts";
import { CORE_EN } from "./🧱️core.ts";
import { DETAIL_EN } from "./📖️detail.ts";
import { PLAYGROUND_EN } from "./🧪️playground.ts";
import { VIEWER_EN } from "./🎛️viewer.ts";

export const ENGLISH = { ...CORE_EN, ...COLLECTIONS_EN, ...COMPARE_EN, ...DETAIL_EN, ...PLAYGROUND_EN, ...VIEWER_EN } as const;
