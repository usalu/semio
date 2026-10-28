/** 🧾️ Public entry `@semio-tech/value-ts/dictionary`: the explicit surface that importers outside the module use. */
export { default as dictionarySchema } from "../../../🧾️dictionary/🧬️schema/🔣️.json" with { type: "json" };
export { parseQuestionDictionary } from "../../../🧾️dictionary/🟦️.ts";
export type { QuestionDictionary } from "../../../🧾️dictionary/🟦️.ts";

export {BorrowedQuestionEquality,QuestionDictionaryEqualityCursor,QuestionValueEqualityCursor} from '../../../🧾️dictionary/🟰️equal/🟦️.ts';
export type {QuestionEqualitySource,QuestionEqualityGrant,QuestionEqualityDemand,QuestionEqualityProgress,QuestionEqualityStep} from '../../../🧾️dictionary/🟰️equal/🟦️.ts';
