//! 🚪️ Snapshot ingress of the board fill: the node, handle, kind and rule publication byte by byte and the ingress close protocol. See [`super`].

use super::*;

// #region board_host
//#region 🪣️Fill
impl BoardFillSnapshotIngress {
    pub fn new(suggestion_offset: f64, naming: &'static BoardNaming) -> Self {
        Self {
            snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset, naming }),
            node: None,
            handle: None,
            kind: None,
            template: None,
            rule: None,
            closing: false,
        }
    }

    pub fn begin_node(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.node.is_some() {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        self.node = Some(BoardFillNodeSnapshot { id: BoardFillText::empty(), bounds: [0.0; 4] });
        Ok(())
    }

    pub fn push_node_id_byte(&mut self, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        self.node.as_mut().ok_or(BoardFillCaptureFault::StaleNode)?.id.try_push_byte(byte)
    }

    pub fn set_node_bound(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !value.is_finite() {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        let bound = self.node.as_mut().and_then(|node| node.bounds.get_mut(index)).ok_or(BoardFillCaptureFault::StaleNode)?;
        *bound = value;
        Ok(())
    }

    pub fn publish_node(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleNode);
        }
        let node = self.node.take().ok_or(BoardFillCaptureFault::NodeCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.node = Some(node);
            return Err(BoardFillCaptureFault::NodeCapacity);
        };
        if let Err(node) = snapshot.nodes.try_push_owned(node) {
            self.node = Some(node);
            return Err(BoardFillCaptureFault::NodeCapacity);
        }
        Ok(())
    }

    pub fn begin_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.handle.is_some() {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle = Some(BoardFillHandleSnapshot {
            id: BoardFillText::empty(),
            node_kind: BoardFillText::empty(),
            handle_kind: BoardFillText::empty(),
            wire_kind: BoardFillText::empty(),
            edge_kind: BoardFillText::empty(),
            slot: [0.0; 2],
            visible: false,
            connected: false,
            weight: 0.0,
        });
        Ok(())
    }

    pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let handle = self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle)?;
        match field {
            BoardFillIngressHandleText::Id => handle.id.try_push_byte(byte),
            BoardFillIngressHandleText::NodeKind => handle.node_kind.try_push_byte(byte),
            BoardFillIngressHandleText::HandleKind => handle.handle_kind.try_push_byte(byte),
            BoardFillIngressHandleText::WireKind => handle.wire_kind.try_push_byte(byte),
            BoardFillIngressHandleText::EdgeKind => handle.edge_kind.try_push_byte(byte),
        }
    }

    pub fn set_handle_slot(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !value.is_finite() {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let slot = self.handle.as_mut().and_then(|handle| handle.slot.get_mut(index)).ok_or(BoardFillCaptureFault::StaleHandle)?;
        *slot = value;
        Ok(())
    }

    pub fn set_handle_visible(&mut self, visible: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.visible = visible)
    }

    pub fn set_handle_connected(&mut self, connected: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.connected = connected)
    }

    pub fn set_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.weight = weight)
    }

    pub fn publish_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleHandle);
        }
        let handle = self.handle.take().ok_or(BoardFillCaptureFault::HandleCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.handle = Some(handle);
            return Err(BoardFillCaptureFault::HandleCapacity);
        };
        if let Err(handle) = snapshot.handles.try_push_owned(handle) {
            self.handle = Some(handle);
            return Err(BoardFillCaptureFault::HandleCapacity);
        }
        Ok(())
    }

    pub fn begin_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.kind.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind = Some(BoardFillKindSnapshot {
            id: BoardFillText::empty(),
            shape: BoardFillShape::Circle,
            radius: 0.0,
            width: 0.0,
            height: 0.0,
            icon: None,
            weight: 0.0,
            handles: BoardFillFixedPages::try_new().map_err(|_| BoardFillCaptureFault::KindHandleCapacity)?,
        });
        Ok(())
    }

    pub fn begin_kind_icon(&mut self) -> Result<(), BoardFillCaptureFault> {
        let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        if self.closing || kind.icon.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        kind.icon = Some(BoardFillText::empty());
        Ok(())
    }

    pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        match field {
            BoardFillIngressKindText::Id => kind.id.try_push_byte(byte),
            BoardFillIngressKindText::Icon => kind.icon.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?.try_push_byte(byte),
        }
    }

    pub fn set_kind_rectangle(&mut self, rectangle: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.shape = if rectangle { BoardFillShape::Rectangle } else { BoardFillShape::Circle })
    }

    pub fn set_kind_radius(&mut self, radius: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !radius.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.radius = radius)
    }

    pub fn set_kind_width(&mut self, width: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !width.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.width = width)
    }

    pub fn set_kind_height(&mut self, height: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !height.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.height = height)
    }

    pub fn set_kind_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.weight = weight)
    }

    pub fn begin_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.kind.is_none() || self.template.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template = Some(BoardFillTemplateSnapshot { handle_kind: BoardFillText::empty(), wire_kind: BoardFillText::empty(), edge_kind: BoardFillText::empty(), angle: 0.0, radius: None, weight: 0.0 });
        Ok(())
    }

    pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let template = self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
        match field {
            BoardFillIngressTemplateText::HandleKind => template.handle_kind.try_push_byte(byte),
            BoardFillIngressTemplateText::WireKind => template.wire_kind.try_push_byte(byte),
            BoardFillIngressTemplateText::EdgeKind => template.edge_kind.try_push_byte(byte),
        }
    }

    pub fn set_kind_handle_angle(&mut self, angle: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !angle.is_finite() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.angle = angle)
    }

    pub fn set_kind_handle_radius(&mut self, radius: Option<f64>) -> Result<(), BoardFillCaptureFault> {
        if self.closing || radius.is_some_and(|radius| !radius.is_finite()) {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.radius = radius)
    }

    pub fn set_kind_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
        if self.closing || !weight.is_finite() || weight <= 0.0 {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.weight = weight)
    }

    pub fn publish_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let template = self.template.take().ok_or(BoardFillCaptureFault::KindHandleCapacity)?;
        let Some(kind) = self.kind.as_mut() else {
            self.template = Some(template);
            return Err(BoardFillCaptureFault::StaleKind);
        };
        if let Err(template) = kind.handles.try_push_owned(template) {
            self.template = Some(template);
            return Err(BoardFillCaptureFault::KindHandleCapacity);
        }
        Ok(())
    }

    pub fn publish_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.template.is_some() {
            return Err(BoardFillCaptureFault::StaleKind);
        }
        let kind = self.kind.take().ok_or(BoardFillCaptureFault::StaleKind)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.kind = Some(kind);
            return Err(BoardFillCaptureFault::KindCapacity);
        };
        if let Err(kind) = snapshot.kinds.try_push_owned(kind) {
            self.kind = Some(kind);
            return Err(BoardFillCaptureFault::KindCapacity);
        }
        Ok(())
    }

    pub fn begin_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing || self.rule.is_some() {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        self.rule = Some(BoardFillRuleSnapshot { source: BoardFillText::empty(), target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::Handle });
        Ok(())
    }

    pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let rule = self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule)?;
        match field {
            BoardFillIngressRuleText::Source => rule.source.try_push_byte(byte),
            BoardFillIngressRuleText::Target => rule.target.try_push_byte(byte),
        }
    }

    pub fn set_rule_bidirectional(&mut self, bidirectional: bool) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.bidirectional = bidirectional)
    }

    pub fn set_rule_specificity(&mut self, specificity: &str) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let specificity = match specificity {
            "general" => CompatSpecificity::General,
            "node" => CompatSpecificity::Node,
            "edge" => CompatSpecificity::Edge,
            "wire" => CompatSpecificity::Wire,
            "handle" | "vortex" => CompatSpecificity::Handle,
            _ => return Err(BoardFillCaptureFault::StaleRule),
        };
        self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.specificity = specificity)
    }

    pub fn publish_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
        if self.closing {
            return Err(BoardFillCaptureFault::StaleRule);
        }
        let rule = self.rule.take().ok_or(BoardFillCaptureFault::RuleCapacity)?;
        let Some(snapshot) = self.snapshot.as_mut() else {
            self.rule = Some(rule);
            return Err(BoardFillCaptureFault::RuleCapacity);
        };
        if let Err(rule) = snapshot.rules.try_push_owned(rule) {
            self.rule = Some(rule);
            return Err(BoardFillCaptureFault::RuleCapacity);
        }
        Ok(())
    }

    pub fn take_snapshot(&mut self) -> Option<BoardFillSnapshot> {
        if self.closing || self.node.is_some() || self.handle.is_some() || self.kind.is_some() || self.template.is_some() || self.rule.is_some() {
            return None;
        }
        self.closing = true;
        self.snapshot.take()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn next_close_release_byte_demand(&self)->usize {
        if self.node.is_some()||self.handle.is_some()||self.template.is_some()||self.rule.is_some(){return 0;}
        if let Some(kind)=self.kind.as_ref(){return kind.handles.next_release_bytes();}
        let Some(snapshot)=self.snapshot.as_ref()else{return 0};
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get(index)){if !kind.handles.terminal_is_empty(){return kind.handles.next_release_bytes();}}
        if !snapshot.kinds.is_empty()||!snapshot.rules.is_empty()||!snapshot.handles.is_empty()||!snapshot.nodes.is_empty(){return 0;}
        for bytes in [snapshot.kinds.next_release_bytes(),snapshot.rules.next_release_bytes(),snapshot.handles.next_release_bytes(),snapshot.nodes.next_release_bytes()]{if bytes!=0{return bytes;}}
        0
    }

    pub fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let maximum_items=grant.maximum_items;
        let maximum_bytes=grant.maximum_release_bytes;
        if grant.maximum_depth==0 {return semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()};}
        self.begin_close();
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if self.node.take().is_some() || self.handle.take().is_some() || self.template.take().is_some() || self.rule.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        if let Some(kind) = self.kind.as_mut() {
            if kind.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
            if let Some(step)=kind.handles.retire_descriptor(grant){return step;}
            self.kind = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        let Some(snapshot) = self.snapshot.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()} };
        if let Some(kind) = snapshot.kinds.len.checked_sub(1).and_then(|index| snapshot.kinds.get_mut(index)) {
            if kind.handles.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
            }
            if let Some(bytes) = kind.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                }
                let Some(released_bytes) = kind.handles.retire_empty_page() else {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                };
                return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
            }
        }
        if let Some(kind)=snapshot.kinds.len.checked_sub(1).and_then(|index|snapshot.kinds.get_mut(index)){if let Some(step)=kind.handles.retire_descriptor(grant){return step;}}
        if snapshot.kinds.pop().is_some() || snapshot.rules.pop().is_some() || snapshot.handles.pop().is_some() || snapshot.nodes.pop().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,..Default::default()} };
        }
        macro_rules! retire_ingress_page {
            ($owners:expr) => {
                if let Some(bytes) = $owners.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    }
                    let Some(released_bytes) = $owners.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::RetainedCloneProgress {copied_items:1,released_bytes,..Default::default()} };
                }
                if let Some(step)=$owners.retire_descriptor(grant){return step;}
            };
        }
        retire_ingress_page!(snapshot.kinds);
        retire_ingress_page!(snapshot.rules);
        retire_ingress_page!(snapshot.handles);
        retire_ingress_page!(snapshot.nodes);
        self.snapshot = None;
        semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}}
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.snapshot.is_none() && self.node.is_none() && self.handle.is_none() && self.kind.is_none() && self.template.is_none() && self.rule.is_none()
    }
}

impl Drop for BoardFillSnapshotIngress {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Board fill ingress must reach exact terminal-empty before Drop");
    }
}
//#endregion 🧵️FillJob
// #endregion board_host
