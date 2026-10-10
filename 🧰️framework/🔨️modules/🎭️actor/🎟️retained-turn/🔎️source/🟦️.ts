/** 🔎️ Resolves the sole current borrowed input through its actual declared Rust field graph. */
export function resolveOriginalTurnInputSource(source:string):string{
 const start=source.indexOf("impl TurnGrant {"),end=source.indexOf("pub async fn pack_encode",start),accessor=source.slice(start,end).match(/pub fn original_input\(&self\)\s*->\s*&RetainedTurnInput\s*\{\s*&self((?:\.[A-Za-z_][A-Za-z_0-9]*)+)\s*\}/);
 if(!accessor)throw Error("TurnGrant must borrow one actual original input");
 const fields=accessor[1]!.slice(1).split(".");let type="TurnGrant";
 for(const name of fields){const declaration=source.match(new RegExp("pub struct "+type+"\\s*\\{([\\s\\S]*?)\\n\\}"));if(!declaration)throw Error("Original input owner declaration is absent");const field=declaration[1]!.match(new RegExp("\\bpub\\s+"+name+"\\s*:\\s*([A-Za-z_][A-Za-z_0-9]*)\\b"));if(!field)throw Error("Original input field declaration is absent");type=field[1]!;}
 if(type!=="RetainedTurnInput")throw Error("Original input accessor must resolve to its genuine declared type");
 return "self."+fields.join(".");
}
