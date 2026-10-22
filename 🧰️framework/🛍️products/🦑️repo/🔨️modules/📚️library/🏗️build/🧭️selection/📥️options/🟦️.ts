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
