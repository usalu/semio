//! 🎮️ Guided workspace commands share the native file worker and ordinary customizable actions.
use super::*;
use semio_framework_ui_tui_widgets::widget::{InputState,ListState};



impl<H:SourceWindowPort> View<'_,H> {
    pub fn begin_management(&mut self,tui:&mut Tui,index:usize,kind:EntryAction) {
        if self.source_job.is_some(){self.effects.push(Effect::Notice(Some(self.text().source_busy.as_str().into())));return;}
        if self.dirty_close(index){return;}
        let Body::Files(files)=self.port.window(index).body else{self.effects.push(Effect::Notice(Some(self.text().source_choose_entry.as_str().into())));return;};
        if files.history{self.effects.push(Effect::Notice(Some(self.text().source_read_only.as_str().into())));return;}
        if matches!(kind,EntryAction::CreateFile|EntryAction::CreateDirectory){self.show_management_form(tui,index,kind,String::new(),None);return;}
        let selected=match &tui.scene.node(self.port.window(index).list).content{semio_framework_ui_tui_frame::scene::NodeContent::Widget(WidgetState::List(list))=>list.selected,_=>return};
        let parent_visible=!files.directory.is_empty()&&files.query.is_empty();if parent_visible&&selected==0{return;}
        let Some(entry)=files.visible.get(selected.saturating_sub(usize::from(parent_visible))).and_then(|position|files.entries.get(*position))else{return;};let path=entry.path.clone();
        if self.management_dirty(tui,&path,None){return;}
        self.request_source(index,Request::Inspect{path,kind});
    }

    pub(super) fn show_management_form(&mut self,tui:&mut Tui,index:usize,kind:EntryAction,path:String,revision:Option<EntryRevision>) {
        let placeholder=self.text().source_name.as_str().to_string();let mut input=InputState::new(placeholder);
        if kind==EntryAction::Rename{input.set_value(path.rsplit('/').next().unwrap_or(&path));}
        if let Body::Files(files)=self.port.window_mut(index).body{files.form=Some(EntryForm{kind,path,revision,busy:false});}else{return;}
        if let Some(widget)=tui.scene.node_mut(self.port.window(index).list).widget(){*widget=WidgetState::Input(input);}tui.set_focus(Some(self.port.window(index).list));self.effects.push(Effect::Notice(None));
    }

    pub(super) fn management_key(&mut self,tui:&mut Tui,index:usize,key:&KeyEvent)->bool {
        let Some(form)=(match self.port.window(index).body{Body::Files(files)=>files.form.as_ref(),_=>None})else{return false;};
        if key.key==Key::Esc{let busy=form.busy;self.cancel_source();if !busy{self.finish_management_form(tui,index);}return true;}
        if form.busy{return true;}
        if key.key==Key::Enter{self.submit_management(tui,index);return true;}
        self.source_form_key(tui,index,key);true
    }

    pub fn management_paste(&mut self,tui:&mut Tui,index:usize,text:&str)->bool {
        if self.history_paste(tui,index,text){return true;}
        let Some(form)=(match self.port.window(index).body{Body::Files(files)=>files.form.as_ref(),_=>None})else{return false;};if form.busy{return true;}
        self.source_form_paste(tui,index,text);true
    }

    pub(super) fn source_form_key(&mut self,tui:&mut Tui,index:usize,key:&KeyEvent){if let Some(WidgetState::Input(input))=tui.scene.node_mut(self.port.window(index).list).widget(){if matches!(key.key,Key::Char(c)if !c.is_control()&&key.mods&(mods::CTRL|mods::ALT)==0&&input.value.encode_utf16().count()+c.len_utf16()>256){self.effects.push(Effect::Notice(Some(self.text().source_path_error.as_str().into())));return;}let _=semio_framework_ui_tui_widgets::input::input_on_key(input,key);}}
    pub(super) fn source_form_paste(&mut self,tui:&mut Tui,index:usize,text:&str){if let Some(WidgetState::Input(input))=tui.scene.node_mut(self.port.window(index).list).widget(){if text.len()<=1024&&input.value.encode_utf16().count()+text.encode_utf16().count()<=256{let _=semio_framework_ui_tui_widgets::input::input_on_paste(input,text);}else{self.effects.push(Effect::Notice(Some(self.text().source_path_error.as_str().into())));}}}

    fn submit_management(&mut self,tui:&mut Tui,index:usize) {
        let value=match &tui.scene.node(self.port.window(index).list).content{semio_framework_ui_tui_frame::scene::NodeContent::Widget(WidgetState::Input(input))=>input.value.clone(),_=>return};
        let Body::Files(files)=self.port.window(index).body else{return;};let request=match files.prepare_entry(&value){Ok(request)=>request,Err(error)=>{self.effects.push(Effect::Notice(Some(self.text().source_browser_error(error,files.form.as_ref().map(|form|form.path.as_str())))));return;}};
        let paths=request.management_paths();if self.management_dirty(tui,paths[0].unwrap_or(""),paths[1]){return;}
        if let Body::Files(files)=self.port.window_mut(index).body{if let Some(form)=&mut files.form{form.busy=true;}}
        self.request_source(index,request);
    }

    fn management_dirty(&mut self,tui:&mut Tui,path:&str,destination:Option<&str>)->bool {
        let dirty=self.port.windows().position(|window|matches!(&window.body,Body::Document(Some(document))if document.dirty&&(commands::path_contains(path,&document.document.path)||destination.is_some_and(|path|commands::path_contains(path,&document.document.path)))));
        if let Some(index)=dirty{self.focus_window(tui,index);self.dirty_close(index);return true;}false
    }

    pub fn management_blocks_edit(&self,index:usize)->bool {
        let Body::Document(Some(document))=self.port.window(index).body else{return false;};blocked_path(self.source_job.as_ref(),self.source_pending.as_ref(),&document.document.path)
    }

    pub(super) fn unlock_management(&mut self,index:usize){if let Body::Files(files)=self.port.window_mut(index).body{files.unlock_forms();}}

    pub(super) fn finish_management_form(&mut self,tui:&mut Tui,index:usize){if let Body::Files(files)=self.port.window_mut(index).body{files.form=None;}if let Some(widget)=tui.scene.node_mut(self.port.window(index).list).widget(){*widget=WidgetState::List(ListState::new(Vec::new()));}self.paint_files(tui,index);}

    pub(super) fn management_completed(&mut self,tui:&mut Tui,index:usize,path:&str,recovery:Option<String>){self.finish_management_form(tui,index);if let Body::Files(files)=self.port.window_mut(index).body{files.recovery=recovery;}self.request_source(index,Request::List(parent(path)));}

    pub(super) fn adopt_created_document(&mut self,tui:&mut Tui,index:usize,document:Document){self.replace_body(tui,index,Kind::Document);if let Some(widget)=tui.scene.node_mut(self.port.window(index).list).widget(){*widget=WidgetState::Editor(EditorState::new(document.text.clone(),1_048_576));}self.port.set_model(index,OwnedBody::Document(Some(Box::new(DocumentBuffer::new(document,false)))));self.effects.push(Effect::Notice(Some(self.text().source_saved.as_str().into())));}

    pub(super) fn reconcile_renamed_documents(&mut self,tui:&mut Tui,previous:&str,path:&str,document:Option<Document>){for mut window in self.port.windows_mut(){if let Body::Document(Some(current))=&mut window.body{if current.remap(previous,path,document.as_ref()){if let Some(WidgetState::Editor(editor))=tui.scene.node_mut(window.list).widget(){if !current.read_only{*editor=EditorState::new(current.document.text.clone(),1_048_576);}editor.read_only=current.read_only;}}}}}

    pub(super) fn mark_uncertain_documents(&mut self,tui:&mut Tui,paths:&[String],recovery:Option<&str>){for mut window in self.port.windows_mut(){if let Body::Document(Some(current))=&mut window.body{if current.mark_uncertain(paths,recovery){if let Some(WidgetState::Editor(editor))=tui.scene.node_mut(window.list).widget(){editor.read_only=true;}}}}}

    pub(super) fn reconcile_retained_documents(&mut self,tui:&mut Tui,path:&str){let indices=self.port.windows().enumerate().filter_map(|(index,window)|matches!(&window.body,Body::Document(Some(current))if commands::path_contains(path,&current.document.path)).then_some(index)).collect::<Vec<_>>();for index in indices{self.replace_body(tui,index,Kind::Files);}}

}
