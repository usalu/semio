//! ⌨️ A local keymap draft; only a validated deliberate save becomes a preference event.
pub use dashboard_preferences::{Change,Preferences};
pub use dashboard_preferences::keymap::{Keymap,KeySpec,Scope};
pub use dashboard_labels::DashboardLabels;
use std::collections::BTreeMap;

pub const MAXIMUM_INPUT_BYTES:usize=4096;
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Target {Prefix,Alias(String,usize)}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Action {Edit(Target),Add(String),Remove(String,usize),Reset(String),ResetAll,Save,Cancel}
pub struct Editor {
    original_prefix:String,
    original:BTreeMap<String,Vec<String>>,
    pub prefix:String,
    pub bindings:BTreeMap<String,Vec<String>>,
    pub editing:Option<Target>,
    pub input:String,
    pub problem:Option<String>,
}
impl Editor {
    pub fn new(preferences:&Preferences)->Self {
        let (effective,_)=preferences.keymap();
        let bindings=Scope::ALL.into_iter().flat_map(|scope|effective.bindings(scope)).map(|binding|(binding.id(),binding.keys.iter().map(ToString::to_string).collect())).collect::<BTreeMap<_,_>>();
        let prefix=effective.prefix().to_string();
        Self{original_prefix:prefix.clone(),original:bindings.clone(),prefix,bindings,editing:None,input:String::new(),problem:None}
    }
    pub fn begin(&mut self,target:Target) {
        self.input=match &target{Target::Prefix=>self.prefix.clone(),Target::Alias(id,index)=>self.bindings.get(id).and_then(|keys|keys.get(*index)).cloned().unwrap_or_default()};
        self.editing=Some(target);self.problem=None;
    }
    pub fn push(&mut self,text:&str) {for c in text.chars().filter(|c|!c.is_control()){if self.input.len()+c.len_utf8()<=MAXIMUM_INPUT_BYTES{self.input.push(c);}else{self.problem=Some(format!("maximumInputBytes={MAXIMUM_INPUT_BYTES}"));break;}}}
    pub fn commit(&mut self)->Result<(),String> {
        let Some(target)=self.editing.clone() else{return Ok(())};
        let key=match &target{Target::Prefix=>Keymap::check_prefix(&self.input)?,Target::Alias(..)=>KeySpec::parse(&self.input)?}.to_string();
        match target{Target::Prefix=>self.prefix=key,Target::Alias(id,index)=>{let keys=self.bindings.get_mut(&id).ok_or_else(||id.clone())?;if index==keys.len(){keys.push(key);}else if let Some(value)=keys.get_mut(index){*value=key;}}}
        self.editing=None;self.problem=None;Ok(())
    }
    pub fn reset(&mut self,id:Option<&str>) {
        let defaults=Keymap::defaults();
        if id.is_none(){self.prefix=defaults.prefix().to_string();}
        for scope in Scope::ALL{for binding in defaults.bindings(scope){if id.is_none_or(|id|id==binding.id()){self.bindings.insert(binding.id(),binding.keys.iter().map(ToString::to_string).collect());}}}
        self.problem=None;
    }
    pub fn change(&self)->Result<Option<Change>,String> {
        if self.editing.is_some(){return Err(self.input.clone());}
        let (_,problems)=Keymap::build(&self.prefix,&self.bindings);
        if !problems.is_empty(){return Err(problems.join("\n"));}
        let changed=self.bindings.iter().filter(|(id,keys)|self.original.get(*id)!=Some(*keys)).map(|(id,keys)|(id.clone(),keys.clone())).collect::<BTreeMap<_,_>>();
        let prefix=(self.prefix!=self.original_prefix).then(||self.prefix.clone());
        if changed.is_empty()&&prefix.is_none(){return Ok(None);}
        Ok(Some(Change{prefix,bindings:(!changed.is_empty()).then_some(changed),..Default::default()}))
    }
    pub fn rows(&self,text:&DashboardLabels)->Vec<(String,Action)> {
        let mut rows=vec![(format!("{}: {}",text.setting_prefix.as_str(),self.prefix),Action::Edit(Target::Prefix))];
        if self.editing.is_some(){rows.insert(0,(format!("{}: {} · {}",text.keymap_editing.as_str(),self.input,text.keymap_edit_hint.as_str()),Action::Edit(self.editing.clone().unwrap())));}
        if let Some(problem)=&self.problem{rows.insert(0,(text.keymap_invalid.fill(&[("detail",problem)]).into_string(),Action::Edit(Target::Prefix)));}
        for (id,keys) in &self.bindings{
            let action=id.split_once('.').map_or(id.as_str(),|(_,action)|action);
            for (index,key) in keys.iter().enumerate(){
                rows.push((format!("{id} · {}: {key}",text.action(action)),Action::Edit(Target::Alias(id.clone(),index))));
                if keys.len()>1{rows.push((format!("  {} {key}",text.keymap_remove.as_str()),Action::Remove(id.clone(),index)));}
            }
            rows.push((format!("  {} · {id}",text.keymap_add.as_str()),Action::Add(id.clone())));
            rows.push((format!("  {} · {id}",text.keymap_reset.as_str()),Action::Reset(id.clone())));
        }
        rows.extend([(text.keymap_reset_all.as_str().into(),Action::ResetAll),(text.keymap_save.as_str().into(),Action::Save),(text.keymap_cancel.as_str().into(),Action::Cancel)]);
        rows
    }
}
