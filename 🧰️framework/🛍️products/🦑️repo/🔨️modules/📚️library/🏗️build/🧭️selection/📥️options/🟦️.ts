import admissionSchema from "../../🧬️schema/🚪️admission.json" with {type:"json"};

/** 🧺️ Retains repeatable option occurrences in order without reading literal task arguments. */
export function buildOptionsV1(args:readonly string[],names:readonly string[]):string[] {
  const selected:string[]=[];
  for(let index=0;index<args.length&&args[index]!=="--";index++){
    const argument=args[index]!;
    if(names.some(name=>argument===`--${name}`||argument===`-${name}`)){const value=args[index+1];if(value!==undefined&&!value.startsWith("-")){selected.push(value);index++;}}
    else if(names.some(name=>argument.startsWith(`--${name}=`)||argument.startsWith(`-${name}=`)))selected.push(argument.slice(argument.indexOf("=")+1));
  }
  return selected;
}

/** 🎯️ Recognizes bounded project identities without consulting the workspace roster. */
export function literalBuildProjectSelectorsV1(projects:readonly string[],exclude:readonly string[]):boolean{
 const policy=admissionSchema.$defs.execOwnerSelection.const,values=[...projects,...exclude];return projects.length>0&&projects.length<=policy.maximumSelectors&&exclude.length<=policy.maximumSelectors&&values.every(value=>value.length>0&&value.length<=policy.maximumSelectorCharacters&&!policy.rosterPrefixes.some(prefix=>value.startsWith(prefix))&&!policy.rosterMetacharacters.some(character=>value.includes(character)));
}

/** 🪪️ Resolves exact identity or root selectors only against the captured owner declarations. */
export function selectLiteralBuildProjectsV1(owners:readonly {name:string;root:string}[],projects:readonly string[],exclude:readonly string[]):string[]{
 const matches=(owner:{name:string;root:string},value:string)=>value.startsWith("name:")?owner.name===value.slice(5):value.startsWith("directory:")?owner.root===value.slice(10):owner.name===value||owner.root===value;return owners.filter(owner=>projects.some(value=>matches(owner,value))&&!exclude.some(value=>matches(owner,value))).map(owner=>owner.name).sort();
}

/** 🧵️ Matches one selector pattern against one value with literal text and portable `*` wildcards. */
export function buildSelectorPatternV1(pattern:string,value:string):boolean{if(/[?\[\]{}()\\]/u.test(pattern))throw Error("Unsupported build selector pattern: "+pattern);return new RegExp("^"+pattern.split("*").map(part=>part.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join(".*")+"$").test(value);}

/** 🗂️ Whether the selectors resolve from declared owner names and roots alone: bounded wildcard, `name:`, `directory:`, comma-listed and later-negated patterns do; `tag:` selectors need inferred tags and a leading negation selects the whole roster, so both keep the complete read.
 * @see ../../🧬️schema/🚪️admission.json */
export function declaredBuildProjectSelectorsV1(projects:readonly string[],exclude:readonly string[]):boolean{
 const policy=admissionSchema.$defs.execOwnerSelection.const,values=[...projects,...exclude];
 return projects.length>0&&projects.length<=policy.maximumSelectors&&exclude.length<=policy.maximumSelectors&&!projects[0]!.startsWith("!")&&!projects.some(value=>value==="*"||value==="name:*")&&values.every(value=>value.length>0&&value.length<=policy.maximumSelectorCharacters&&!value.replace(/^!/u,"").startsWith("tag:")&&!/[?\[\]{}()\\]/u.test(value));
}

/** 🌐️ The bilingual progress line naming why a selection keeps the complete workspace read, or undefined when declared names suffice. */
export function wholeBuildReadReasonV1(projects:readonly string[],exclude:readonly string[]):string|undefined{
 if(!projects.length||declaredBuildProjectSelectorsV1(projects,exclude))return undefined;
 return"Project selector needs inferred tags or selects everything: reading the whole workspace\nProjektauswahl braucht abgeleitete Tags oder wählt alles: der gesamte Arbeitsbereich wird gelesen";
}

/** 🎯️ Selects names by identity, root, `name:`, `directory:` or `tag:` patterns in order, then removes the excluded; the one rule shared by the owner registry and the loaded graph so both select the same set. */
export function selectBuildNamesV1(entries:readonly {name:string;root:string;tags?:readonly string[]}[],patterns:readonly string[]=[],exclude:readonly string[]=[]):string[]{
 const match=(entry:{name:string;root:string;tags?:readonly string[]},pattern:string)=>pattern.startsWith("tag:")?(entry.tags??[]).some(tag=>buildSelectorPatternV1(pattern.slice(4),tag)):pattern.startsWith("name:")?buildSelectorPatternV1(pattern.slice(5),entry.name):pattern.startsWith("directory:")?buildSelectorPatternV1(pattern.slice(10),entry.root):buildSelectorPatternV1(pattern,entry.name)||buildSelectorPatternV1(pattern,entry.root);
 const selected=new Set<string>(),ordered=patterns.length?patterns:["*"];if(ordered[0]!.startsWith("!"))for(const entry of entries)selected.add(entry.name);
 for(const value of ordered){const negative=value.startsWith("!"),pattern=negative?value.slice(1):value;buildSelectorPatternV1(pattern,"");for(const entry of entries)if(match(entry,pattern)){if(negative)selected.delete(entry.name);else selected.add(entry.name);}}
 const byName=new Map(entries.map(entry=>[entry.name,entry]));
 for(const pattern of exclude){buildSelectorPatternV1(pattern,"");for(const name of selected)if(match(byName.get(name)!,pattern))selected.delete(name);}return[...selected].sort();
}
