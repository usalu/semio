//! 👁️ Borrowed accepted graph facts without allocated publication projections.
use super::DagChannelDirection;
#[derive(Clone,Copy)]
pub struct DagChannelView<'a>{pub widget_id:&'a str,pub port:&'a str,pub direction:DagChannelDirection}
#[derive(Clone,Copy)]
pub struct DagRefusalView<'a>{pub source:&'a str,pub source_types:&'a[String],pub target:&'a str,pub target_types:&'a[String]}
pub trait DagOutputSource{
    fn selected_node_count(&self)->usize;
    fn selected_node_id(&self,index:usize)->Option<&str>;
    fn selected_edge_count(&self)->usize;
    fn selected_edge_id(&self,index:usize)->Option<&str>;
    fn selected_channel_count(&self)->usize;
    fn selected_channel(&self,index:usize)->Option<DagChannelView<'_>>;
    fn selected_handle_identity(&self,index:usize)->Option<&str>;
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>;
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>;
}
pub trait DagOutputCandidates{
    fn node_candidate_count(&self)->usize;
    fn node_candidate_id(&self,index:usize)->Option<&str>;
    fn edge_candidate_count(&self)->usize;
    fn edge_candidate_id(&self,index:usize)->Option<&str>;
    fn channel_candidate_count(&self)->usize;
    fn channel_candidate(&self,index:usize)->Option<DagChannelView<'_>>;
    fn handle_candidate_identity(&self,index:usize)->Option<&str>;
    fn hovered_channel_view(&self)->Option<DagChannelView<'_>>;
    fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>;
}
