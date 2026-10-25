//! 📂️ Binds app windows to the canonical workspace interaction owner.
use super::windows::{Body,Dashboard,DashboardWindow,Kind,Place};
use dashboard_source_view::{Context,SourceWindowPort,WindowRef,WindowMut,Body as ViewBody,OwnedBody,Effect as SourceEffect,Request,EntryAction};
use semio_framework_filesystem_workspace::commands::HistoryAction;
use semio_framework_ui_tui_core::event::KeyEvent;
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_frame::scene::{Node,NodeContent};

pub struct Windows<'a>{windows:&'a mut [DashboardWindow],input_mode:&'a mut bool}
fn read(window:&DashboardWindow)->WindowRef<'_>{WindowRef{id:&window.id,list:window.list,caption:window.caption,body:match &window.body{Body::Files(value)=>ViewBody::Files(value),Body::Document(value)=>ViewBody::Document(value.as_deref()),_=>ViewBody::Other}}}
fn write(window:&mut DashboardWindow)->WindowMut<'_>{WindowMut{id:&window.id,list:window.list,body:match &mut window.body{Body::Files(value)=>ViewBody::Files(value),Body::Document(value)=>ViewBody::Document(value.as_deref_mut()),_=>ViewBody::Other}}}
impl SourceWindowPort for Windows<'_>{
    type Windows<'a>=std::iter::Map<std::slice::Iter<'a,DashboardWindow>,fn(&'a DashboardWindow)->WindowRef<'a>> where Self:'a;
    type WindowsMut<'a>=std::iter::Map<std::slice::IterMut<'a,DashboardWindow>,fn(&'a mut DashboardWindow)->WindowMut<'a>> where Self:'a;
    fn windows(&self)->Self::Windows<'_>{self.windows.iter().map(read)}
    fn windows_mut(&mut self)->Self::WindowsMut<'_>{self.windows.iter_mut().map(write)}
    fn window(&self,index:usize)->WindowRef<'_>{read(&self.windows[index])}
    fn window_mut(&mut self,index:usize)->WindowMut<'_>{write(&mut self.windows[index])}
    fn len(&self)->usize{self.windows.len()}
    fn set_model(&mut self,index:usize,body:OwnedBody){self.windows[index].body=match body{OwnedBody::Files(value)=>Body::Files(value),OwnedBody::Document(value)=>Body::Document(value)};}
    fn replace(&mut self,tui:&mut Tui,index:usize,kind:dashboard_source_view::Kind){
        let window=&mut self.windows[index];
        let mut focus=tui.focus();let mut focused=false;
        while let Some(node)=focus{if node==window.chrome{focused=true;break;}focus=tui.scene.try_node(node).and_then(|node|node.parent());}
        tui.scene.remove(window.root);
        let root=tui.scene.add(window.chrome,Node::new(NodeContent::Box));
        tui.scene.node_mut(root).set_constraint(super::windows::node_constraint());
        let(list,caption)=dashboard_source_view::mount(tui,root,kind);
        window.root=root;window.list=list;window.caption=Some(caption);window.input=None;window.tree=None;window.editor_slot=None;window.pane=Default::default();
        window.body=match kind{dashboard_source_view::Kind::Files=>Body::Files(Default::default()),dashboard_source_view::Kind::Document=>Body::Document(None)};
        *self.input_mode=false;
        if focused||tui.focus().is_none_or(|node|!tui.scene.contains(node)){tui.set_focus(Some(list));}
    }
}
impl Dashboard{
    pub fn with_sources<R>(&mut self,action:impl FnOnce(&mut dashboard_source_view::View<'_,Windows<'_>>)->R)->R{
        let context=Context{root:&self.root,keymap:&self.keymap,locale:self.locale,waker:self.waker.clone()};
        let mut port=Windows{windows:&mut self.windows,input_mode:&mut self.input_mode};
        let(result,effects)=self.source.interact(&mut port,context,action);
        self.source_effects.extend(effects);self.apply_local_source_effects();result
    }
    fn apply_local_source_effects(&mut self){
        while matches!(self.source_effects.first(),Some(SourceEffect::Notice(_)|SourceEffect::Failure(_)|SourceEffect::Request(..))){
            match self.source_effects.remove(0){
                SourceEffect::Notice(value)=>self.notice=value,
                SourceEffect::Failure(value)=>self.record_failure(value),
                SourceEffect::Request(window,request)=>if let Some(index)=self.window_index(&window){self.request_source(index,request);},
                _=>unreachable!()
            }
        }
    }
    pub fn flush_source_effects(&mut self,tui:&mut Tui){
        while !self.source_effects.is_empty(){
            match self.source_effects.remove(0){
                SourceEffect::Notice(value)=>self.notice=value,
                SourceEffect::Failure(value)=>self.record_failure(value),
                SourceEffect::Focus(window)=>if let Some(index)=self.window_index(&window){self.focus_window(tui,index);},
                SourceEffect::Close(window)=>if let Some(index)=self.window_index(&window){self.close_window(tui,index);},
                SourceEffect::Remount=>self.remount(tui),
                SourceEffect::Signals(signals)=>self.on_signals(tui,signals),
                SourceEffect::Request(window,request)=>if let Some(index)=self.window_index(&window){self.request_source(index,request);}
            }
        }
    }
    pub fn open_files(&mut self,tui:&mut Tui){
        let index=self.windows.iter().position(|window|matches!(window.body,Body::Files(_))).unwrap_or_else(||self.create_window(tui,Kind::Files,Place::Preference));
        self.focus_window(tui,index);
        let request=match &self.windows[index].body{Body::Files(files)=>files.browse(files.directory.clone()),_=>Request::List(String::new())};
        self.request_source(index,request);
    }
    pub fn request_source(&mut self,index:usize,request:Request){self.with_sources(|view|view.request_source(index,request));}
    pub fn cancel_source(&mut self){self.with_sources(|view|view.cancel_source());}
    pub fn poll_documents(&mut self,tui:&mut Tui)->bool{let value=self.with_sources(|view|view.poll_documents(tui));self.flush_source_effects(tui);value}
    pub fn paint_files(&mut self,tui:&mut Tui,index:usize){self.with_sources(|view|view.paint_files(tui,index));self.flush_source_effects(tui);}
    pub fn files_key(&mut self,tui:&mut Tui,index:usize,key:&KeyEvent){self.with_sources(|view|view.files_key(tui,index,key));self.flush_source_effects(tui);}
    pub fn activate_file(&mut self,tui:&Tui,index:usize,selected:usize){self.with_sources(|view|view.activate_file(tui,index,selected));}
    pub fn save_document(&mut self,tui:&Tui,index:usize){self.with_sources(|view|view.save_document(tui,index));}
    pub fn dirty_close(&mut self,index:usize)->bool{self.with_sources(|view|view.dirty_close(index))}
    pub fn reload_document(&mut self,index:usize){self.with_sources(|view|view.reload_document(index));}
    pub fn discard_document(&mut self,tui:&mut Tui,index:usize){self.with_sources(|view|view.discard_document(tui,index));self.flush_source_effects(tui);}
    pub fn refresh_documents(&mut self,tui:&mut Tui){self.with_sources(|view|view.refresh_documents(tui));self.flush_source_effects(tui);}
    pub fn begin_management(&mut self,tui:&mut Tui,index:usize,kind:EntryAction){self.with_sources(|view|view.begin_management(tui,index,kind));self.flush_source_effects(tui);}
    pub fn management_paste(&mut self,tui:&mut Tui,index:usize,text:&str)->bool{let value=self.with_sources(|view|view.management_paste(tui,index,text));self.flush_source_effects(tui);value}
    pub fn management_blocks_edit(&self,index:usize)->bool{match &self.windows[index].body{Body::Document(Some(document))=>self.source.blocks_path(&document.document.path),_=>false}}
    pub fn begin_history(&mut self,tui:&mut Tui,index:usize,action:Option<HistoryAction>){
        if self.source.active(){self.notice=Some(self.text().source_busy.as_str().into());return;}
        let index=if matches!(self.windows[index].body,Body::Files(_)){index}else{self.create_window(tui,Kind::Files,Place::Preference)};
        self.focus_window(tui,index);self.request_source(index,Request::InspectHistory(action));
    }
    pub fn next_history_page(&mut self,index:usize){self.with_sources(|view|view.next_history_page(index));}
    pub fn inspect_recovery(&mut self,tui:&mut Tui,index:usize){
        let path=match &self.windows[index].body{Body::Document(Some(document))=>document.recovery.clone(),Body::Files(files)=>Some(files.recovery.clone().unwrap_or_default()),_=>None};
        let Some(path)=path else{return;};let request=if path.is_empty(){Request::HistoryPage(None)}else{Request::Recovery(path)};
        let target=self.create_window(tui,Kind::Files,Place::Preference);self.focus_window(tui,target);self.request_source(target,request);
    }
}
