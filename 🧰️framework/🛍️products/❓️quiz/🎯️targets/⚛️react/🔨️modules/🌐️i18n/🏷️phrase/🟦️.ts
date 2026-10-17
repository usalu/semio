/** 🏷️ The constructor of one chrome label: the label a learner reads at normal and at beginner level (the same text unless a beginner wording is given). */

import { type UiLabelValue } from "@semio-tech/ui-react/i18n";

//#region 📚️Bundles
export const phrase = (normal: string, beginner: string = normal): UiLabelValue => ({ label: { normal, beginner } });
//#endregion 📚️Bundles
