import {createIdmlAdmissionOracle} from '../../🧪️tests/oracle.mjs';
/** 🔮 Physical ZIP/XML admission plus the independent RFC6902 implementation. */
export function createIdmlChangeOracle(fflate,SaxesParser,jsonPatch){const admission=createIdmlAdmissionOracle(fflate,SaxesParser);return{valid:snapshot=>admission.valid(snapshot),apply(before,after){return jsonPatch.applyPatch(structuredClone(before),jsonPatch.compare(before,after),true).newDocument;}};}
