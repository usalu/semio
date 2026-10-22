//! 🌐️ Requires an explicit language event before rendering an unresolved dashboard.
use semio_framework_ui_tui_core::event::{mods,Event,Key,MouseButton,MouseKind};
use semio_framework_ui_tui_core::geometry::Size;
use semio_framework_ui_tui_core::theme::Theme;
use semio_framework_ui_tui_frame::layout::{Constraint,Dimension};
use semio_framework_ui_tui_frame::scene::{Node,NodeContent};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_terminal::backend::{TerminalBackend};
use ui_locale::Locale;
use std::path::Path;
#[derive(Default)]
struct Choice { selected:Option<usize> }
#[derive(Debug,PartialEq,Eq)]
enum Outcome { Pending,Accepted(Locale),Cancelled }
impl Choice {
    fn action(&mut self, action:&str)->Outcome {
        let choices=dashboard_preferences::language::declared().choices();
        match action {
            "escape"=>return Outcome::Cancelled,
            "enter"=>if let Some(index)=self.selected { return Outcome::Accepted(Locale::parse(&choices[index].language).expect("validated choice")); },
            "1"=>self.selected=Some(0),
            "2" if choices.len()>1=>self.selected=Some(1),
            "up"=>self.selected=Some(self.selected.map_or(choices.len()-1,|index|(index+choices.len()-1)%choices.len())),
            "down"|"tab"=>self.selected=Some(self.selected.map_or(0,|index|(index+1)%choices.len())),
            _=>{}
        }
        Outcome::Pending
    }
    fn text(&self)->String {
        let declaration=dashboard_preferences::language::declared();
        let mut lines=vec![declaration.prompt()[0].clone(),String::new()];
        lines.extend(declaration.choices().iter().enumerate().map(|(index,choice)|format!("{} {} {}",if self.selected==Some(index){">"}else{" "},index+1,choice.name)));
        lines.push(String::new());lines.push(declaration.prompt()[2].clone());lines.join("\n")
    }
}
fn action(event:&Event)->&'static str {
    match event {
        Event::Key(key) if key.mods&mods::CTRL!=0 && key.key==Key::Char('c')=>"escape",
        Event::Key(key) if key.key==Key::BackTab || key.key==Key::Tab&&key.mods==mods::SHIFT=>"up",
        Event::Key(key) if key.mods==0=>match key.key.text_equivalent(){Key::Char('1')=>"1",Key::Char('2')=>"2",Key::Enter=>"enter",Key::Esc=>"escape",Key::Up=>"up",Key::Down=>"down",Key::Tab=>"tab",_=>""},
        Event::Mouse(mouse) if mouse.kind==MouseKind::Down(MouseButton::Left)=>match mouse.pos.y {2=>"1",3=>"2",_=>""},
        _=>""
    }
}
/// 🎛️ Keeps the host responsive while an explicit selection becomes a durable preference event.
pub(super) fn run<B:TerminalBackend>(term:&mut B,path:&Path,theme:Theme)->Result<Option<Locale>,dashboard_feedback::Failure> {
    let mut choice=Choice::default();let mut tui=Tui::new(term.size().unwrap_or(Size{width:80,height:24}),theme);tui.set_width_mode(term.width_mode());
    let root=tui.scene.root();let node=tui.scene.add(root,Node::new(NodeContent::Text(String::new())));
    tui.scene.node_mut(node).set_constraint(Constraint{width:Dimension::Weight(1),height:Dimension::Weight(1),..Default::default()});
    let mut pending:Option<(Locale,std::sync::Arc<std::sync::atomic::AtomicBool>,std::sync::mpsc::Receiver<std::io::Result<u64>>)>=None;let mut problem=String::new();
    loop {
        if let Some((locale,_,receiver))=&pending {match receiver.try_recv(){Ok(Ok(_))=>return Ok(Some(*locale)),Ok(Err(error))=>{problem=if error.kind()==std::io::ErrorKind::Interrupted{dashboard_preferences::language::declared().message("cancelled").into()}else{format!("{}: {error}",dashboard_preferences::language::declared().message("saveFailed"))};pending=None;},Err(std::sync::mpsc::TryRecvError::Disconnected)=>return Err(dashboard_feedback::Failure::io(dashboard_feedback::Operation::Preferences,std::io::Error::other(dashboard_preferences::language::declared().message("disconnected")))),Err(std::sync::mpsc::TryRecvError::Empty)=>{}}}
        let mut text=choice.text();if pending.is_some(){text.push('\n');text.push_str(dashboard_preferences::language::declared().message("saving"));}if !problem.is_empty(){text.push('\n');text.push_str(&problem);}
        if !matches!(&tui.scene.node(node).content,NodeContent::Text(current) if current==&text){tui.scene.node_mut(node).set_text(text);}
        let patch=tui.render();term.present(&patch,tui.cursor()).map_err(|error|dashboard_feedback::Failure::io(dashboard_feedback::Operation::Present,error))?;
        for event in term.wait(Some(std::time::Instant::now()+std::time::Duration::from_millis(80))).map_err(|error|dashboard_feedback::Failure::io(dashboard_feedback::Operation::Wait,error))? {
            if let Event::Resize(size)=event {tui.resize(size);continue;}
            if let Some((_,cancel,_))=&pending {if action(&event)=="escape"{cancel.store(true,std::sync::atomic::Ordering::Release);}continue;}
            match choice.action(action(&event)) {
                Outcome::Cancelled=>return Ok(None),Outcome::Pending=>{},Outcome::Accepted(locale)=>{let path=path.to_path_buf();let cancel=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));let worker_cancel=cancel.clone();let (sender,receiver)=std::sync::mpsc::channel();std::thread::spawn(move||{let result=dashboard_preferences::append_cancellable(&path,&dashboard_preferences::Change{language:Some(locale.as_str().into()),..Default::default()},&worker_cancel);let _=sender.send(result);});pending=Some((locale,cancel,receiver));problem.clear();}
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unresolved_language_requires_explicit_selection_and_cancel_publishes_nothing() {
        let vectors:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/🌐️language/🔣️.json")).unwrap();
        for row in vectors["selection"].as_array().unwrap(){let mut choice=Choice::default();let mut outcome=Outcome::Pending;for action in row["actions"].as_array().unwrap(){outcome=choice.action(action.as_str().unwrap());}let selected=choice.selected.map(|index|dashboard_preferences::language::declared().choices()[index].language.clone());assert_eq!(serde_json::to_value(selected).unwrap(),row["selected"]);assert_eq!(match outcome {Outcome::Pending=>"pending",Outcome::Accepted(_)=>"accepted",Outcome::Cancelled=>"cancelled"},row["outcome"].as_str().unwrap());}
        assert_eq!(Choice::default().action("enter"),Outcome::Pending);
    }
}
