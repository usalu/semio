import{nativeProgressLanguagesV1}from"../../../🎛️owned-execution/🟦️.ts";
import diagnostics from"../🧬️schema/📣️diagnostics.json" with{type:"json"};

/** 🔑️ Every diagnostic sentence of the native build. */
export type NativeBuildDiagnosticV1=keyof typeof diagnostics.const.labels.en;

/** 📣️ Renders one diagnostic in the language the environment chooses, in English and German when it chooses none; one line per language. */
export function nativeBuildDiagnosticLinesV1(key:NativeBuildDiagnosticV1,values:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>=process.env):readonly string[]{
 return nativeProgressLanguagesV1(environment).map(language=>diagnostics.const.labels[language][key].replace(/\{([A-Za-z]+)\}/gu,(_token,name:string)=>String(values[name]??"")));
}

/** 🪧️ A progress sink that prints the observation rows of the cargo provenance as sentences instead of JSON. */
export function cargoProvenanceConsoleSinkV1(kind:"provenance"|"recipe",environment:Readonly<Record<string,string|undefined>>=process.env):(row:Readonly<{bytes:number;totalBytes:number;work:number}>)=>void{
 return row=>{for(const line of nativeBuildDiagnosticLinesV1(kind==="recipe"?"recipeReading":"provenanceReading",{work:row.work,bytes:row.bytes,totalBytes:row.totalBytes},environment))console.log(line);};
}
