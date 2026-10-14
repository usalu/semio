//! 👁️ Rich graph presentation borrows labels, geometry and exact endpoint components.
/// 🏷️ Borrows rich node geometry and labels independently of the query projection's value families.
#[derive(Clone,Copy)]
pub struct JackContentNodeView<'a>{pub id:&'a str,pub kind:&'a str,pub label:&'a str,pub x:f64,pub y:f64,pub width:f64,pub height:f64,pub port_count:usize}

/// 🔌️ A rich port can participate in both directions without query normalization.
#[derive(Clone,Copy)]
pub struct JackContentPortView<'a>{pub id:&'a str,pub label:&'a str,pub inbound:bool,pub outbound:bool}

/// 🔗️ Borrows exact endpoint components and labels, including literal separator characters.
#[derive(Clone,Copy)]
pub struct JackContentEdgeView<'a>{pub id:&'a str,pub kind:&'a str,pub label:&'a str,pub source:&'a str,pub target:&'a str,pub source_port:Option<&'a str>,pub target_port:Option<&'a str>}

impl JackContentNodeView<'_>{
    pub fn presented_label(&self)->&str{if self.label.is_empty(){self.id}else{self.label}}
    pub fn presented_extent(&self)->(f64,f64){(if self.width>0.0{self.width}else{96.0},if self.height>0.0{self.height}else{48.0})}
}

/// 🪟️ Views retain rich port directions independently of query value projection.
pub trait JackContentVisual{
    fn visual_counts(&self)->(usize,usize);
    fn node_view(&self,index:usize)->Option<JackContentNodeView<'_>>;
    fn port_view(&self,node_index:usize,port_index:usize)->Option<JackContentPortView<'_>>;
    fn edge_view(&self,index:usize)->Option<JackContentEdgeView<'_>>;
}

