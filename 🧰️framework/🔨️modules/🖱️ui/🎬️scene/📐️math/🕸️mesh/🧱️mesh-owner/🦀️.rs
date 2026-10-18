//! 🧱️ Mesh owner: the mesh layout and page, the component length and demand helpers, the `Mesh3dOwner` with its reservations and drop. See [`super`].

use super::*;

//#region Mesh
pub(super) fn mesh3d_component_number(label: &str) -> Option<u64> {
    if label.len() > 20 || !label.as_bytes().first().is_some_and(|byte| matches!(byte, b'1'..=b'9')) || !label.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
    label.parse().ok()
}

#[derive(Clone, Copy)]
pub(super) struct Mesh3dLayout {
    pub(super) offsets: [usize; 14],
    pub(super) total_bytes: usize,
    pub(super) page_count: u16,
}

pub(super) struct Mesh3dPage {
    pub(super) bytes: Box<[u8; MESH3D_PAGE_BYTES]>,
}

pub(super) fn component_lengths(table:&ComponentReferenceTable)->Result<[usize;3],Mesh3dFault> {
    if table.len()>3 {return Err(Mesh3dFault::Schema);}
    let mut lengths=[0;3];
    let mut domains=0u8;
    for(kind,labels)in table {let domain=match kind.as_str(){"face"=>0,"edge"=>1,"vertex"=>2,_=>return Err(Mesh3dFault::Schema)};if labels.len()>600_000||domains&(1<<domain)!=0{return Err(Mesh3dFault::Schema);}domains|=1<<domain;lengths[domain]=labels.len();}
    Ok(lengths)
}

pub(super) fn controlled_demands<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(|_|Mesh3dFault::Closing)?,capacity_bytes:owner.next_capacity_byte_demand(work).map_err(|_|Mesh3dFault::Closing)?,release_bytes:owner.next_release_byte_demand().map_err(|_|Mesh3dFault::Closing)?,depth:owner.next_depth_demand().map_err(|_|Mesh3dFault::Closing)?})
}

pub(super) struct Mesh3dOwner {
    pub(super) generation: u64,
    pub(super) revision: u64,
    pub(super) schema: Mesh3dSchema,
    pub(super) layout: Mesh3dLayout,
    pub(super) pages: Option<Box<[Option<Mesh3dPage>; MESH3D_OWNER_PAGE_CAPACITY]>>,
    pub(super) allocated_pages: u16,
    pub(super) written: [u32; 14],
    pub(super) aabb_min: [f32; 3],
    pub(super) aabb_max: [f32; 3],
    pub(super) closing: bool,
    close_page: u16,
    pub(super) component_references: Option<ComponentReferenceTable>,
    component_retirement: Option<ControlledRetirement<ComponentReferenceTable>>,
    scratch_retirement: Option<ControlledRetirement<(NumericSeen,IneligibleGroups)>>,
    pub(super) component_domain: u8,
    pub(super) component_group: usize,
    pub(super) component_complete: bool,
    pub(super) component_credit: usize,
    pub(super) component_fault: Option<Mesh3dFault>,
    pub(super) component_seen: NumericSeen,
    pub(super) component_ineligible: IneligibleGroups,
    component_probe: Option<(u64,usize)>,
    pub(super) has_topological_vertex: bool,
    pub(super) resource_claimed: bool,
    pub(super) resource_closing: bool,
    pub(super) resource_loans: u32,
}

impl Mesh3dOwner {
    pub(super) fn new(generation: u64, revision: u64, schema: Mesh3dSchema, layout: Mesh3dLayout) -> Self {
        Self { generation, revision, schema, layout, pages: Some(Box::new(std::array::from_fn(|_| None))), allocated_pages: 0, written: [0; 14], aabb_min: [f32::INFINITY; 3], aabb_max: [f32::NEG_INFINITY; 3], closing: false, close_page: 0, component_references: None, component_retirement: None,scratch_retirement:None, component_domain: 0, component_group: 0, component_complete: true, component_credit: 0, component_fault: None, component_seen: NumericSeen::default(), component_ineligible: IneligibleGroups::default(),component_probe:None, has_topological_vertex: false, resource_claimed: false, resource_closing: false, resource_loans: 0 }
    }

    pub(super) fn component_header_credit(&self) -> Result<usize, Mesh3dFault> {
        let Some(table) = self.component_references.as_ref() else { return Ok(0); };
        if table.len() > 3 { return Err(Mesh3dFault::Schema); }
        let credit = table.directory_capacity_bytes().ok_or(Mesh3dFault::ByteCapacity)?;
        for (kind, labels) in table {
            if !matches!(kind.as_str(), "face" | "edge" | "vertex") || labels.len() > 600_000 { return Err(Mesh3dFault::Schema); }
        }
        let lengths=component_lengths(table)?;
        credit.checked_add(NumericSeen::capacity_bytes(*lengths.iter().max().unwrap()).ok_or(Mesh3dFault::ByteCapacity)?).and_then(|credit|credit.checked_add(lengths.iter().sum::<usize>())).ok_or(Mesh3dFault::ByteCapacity)
    }

    fn component_current_label(&self) -> Option<&String> {
        self.component_references.as_ref()?.get(*["face", "edge", "vertex"].get(self.component_domain as usize)?)?.get(self.component_group)
    }

    fn component_key(&self, group: u32) -> u64 { (u64::from(self.component_domain) << 32) | u64::from(group) }

    pub(super) fn component_turn_byte_demand(&self) -> usize {
        if self.component_probe.is_some() {return 12;}
        if let Some(label) = self.component_current_label() { return 24 + if label.len() <= 20 { label.len() } else { 0 }; }
        if self.component_seen.is_empty() { 1 } else { 12 }
    }

    pub(super) fn component_turn_credit(&self) -> Result<(usize, Option<u64>), Mesh3dFault> {
        if self.component_probe.is_some() {return Ok((self.component_credit,None));}
        if let Some(label) = self.component_current_label() {
            let credit = self.component_credit.checked_add(label.capacity()).ok_or(Mesh3dFault::ByteCapacity)?;
            let number = mesh3d_component_number(label);
            return Ok((credit, number));
        }
        Ok((self.component_credit, None))
    }

    pub(super) fn component_turn(&mut self, number: Option<u64>) -> Mesh3dComponentAdmissionStep {
        if let Some((number,probe))=self.component_probe {
            self.component_probe_turn(number,probe);
            return Mesh3dComponentAdmissionStep{processed_items:1,processed_bytes:12,..Default::default()};
        }
        if let Some(label) = self.component_current_label() {
            let bytes = 24 + if label.len() <= 20 { label.len() } else { 0 };
            let group = self.component_group as u32;
            match number {
                Some(number) => self.component_probe_turn(number,self.component_seen.first_probe(number)),
                None => { self.component_ineligible.insert(self.component_key(group));self.component_group+=1; }
            }
            return Mesh3dComponentAdmissionStep { processed_items: 1, processed_bytes: bytes, ..Default::default() };
        }
        if self.component_seen.pop_first().is_some() { return Mesh3dComponentAdmissionStep { retired_items: 1, retired_bytes: 12, ..Default::default() }; }
        self.component_domain += 1;
        self.component_group = 0;
        self.component_complete = self.component_domain == 3;
        Mesh3dComponentAdmissionStep { complete: self.component_complete, processed_items: 1, processed_bytes: 1, ..Default::default() }
    }

    fn component_probe_turn(&mut self,number:u64,probe:usize) {
        let group=self.component_group as u32;
        match self.component_seen.probe(probe) {
            None=>{self.component_seen.insert_at(probe,number,group);self.component_probe=None;self.component_group+=1;},
            Some((previous,first))if previous==number=>{self.component_ineligible.insert(self.component_key(first));self.component_ineligible.insert(self.component_key(group));self.component_probe=None;self.component_group+=1;},
            Some(_)=>self.component_probe=Some((number,self.component_seen.next_probe(probe))),
        }
    }

    pub(super) fn allocate_step(&mut self,grant:RetainedCloneGrant) -> Mesh3dOwnershipStep {
        if self.allocated_pages == self.layout.page_count {
            return Mesh3dOwnershipStep{complete:true,..Default::default()};
        }
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<MESH3D_PAGE_BYTES{return Mesh3dOwnershipStep::default();}
        let slot = usize::from(self.allocated_pages);
        self.pages.as_mut().unwrap()[slot] = Some(Mesh3dPage { bytes: Box::new([0; MESH3D_PAGE_BYTES]) });
        self.allocated_pages += 1;
        Mesh3dOwnershipStep{complete:self.allocated_pages==self.layout.page_count,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:MESH3D_PAGE_BYTES,..Default::default()}}
    }

    pub(super) fn write(&mut self, field: Mesh3dField, bytes: &[u8]) -> Result<u32, Mesh3dFault> {
        if self.closing || self.allocated_pages != self.layout.page_count {
            return Err(if self.closing { Mesh3dFault::Closing } else { Mesh3dFault::Incomplete });
        }
        let field_index = field.index();
        let item = self.written[field_index];
        if item >= self.schema.field_items(field) || bytes.len() != Mesh3dSchema::field_item_bytes(field) {
            return Err(Mesh3dFault::Order);
        }
        let absolute = self.layout.offsets[field_index].checked_add(item as usize * bytes.len()).ok_or(Mesh3dFault::ByteCapacity)?;
        self.write_at(absolute, bytes)?;
        self.written[field_index] += 1;
        if field == Mesh3dField::VertexIds && bytes != u32::MAX.to_le_bytes() { self.has_topological_vertex = true; }
        Ok(item)
    }

    pub(super) fn write_at(&mut self, absolute: usize, bytes: &[u8]) -> Result<(), Mesh3dFault> {
        let end = absolute.checked_add(bytes.len()).ok_or(Mesh3dFault::ByteCapacity)?;
        if end > self.layout.total_bytes {
            return Err(Mesh3dFault::ByteCapacity);
        }
        for (delta, byte) in bytes.iter().enumerate() {
            let at = absolute + delta;
            let page = self.pages.as_mut().ok_or(Mesh3dFault::Incomplete)?[at / MESH3D_PAGE_BYTES].as_mut().ok_or(Mesh3dFault::Incomplete)?;
            page.bytes[at % MESH3D_PAGE_BYTES] = *byte;
        }
        Ok(())
    }

    pub(super) fn read<const N: usize>(&self, absolute: usize) -> Result<[u8; N], Mesh3dFault> {
        let end = absolute.checked_add(N).ok_or(Mesh3dFault::ByteCapacity)?;
        if self.closing || end > self.layout.total_bytes {
            return Err(if self.closing { Mesh3dFault::Closing } else { Mesh3dFault::ByteCapacity });
        }
        let mut result = [0; N];
        for (delta, output) in result.iter_mut().enumerate() {
            let at = absolute + delta;
            let page = self.pages.as_ref().ok_or(Mesh3dFault::Incomplete)?[at / MESH3D_PAGE_BYTES].as_ref().ok_or(Mesh3dFault::Incomplete)?;
            *output = page.bytes[at % MESH3D_PAGE_BYTES];
        }
        Ok(result)
    }

    pub(super) fn item_bytes<const N: usize>(&self, field: Mesh3dField, item: u32) -> Result<[u8; N], Mesh3dFault> {
        if N != Mesh3dSchema::field_item_bytes(field) || item >= self.schema.field_items(field) {
            return Err(Mesh3dFault::Schema);
        }
        let absolute = self.layout.offsets[field.index()].checked_add(item as usize * N).ok_or(Mesh3dFault::ByteCapacity)?;
        self.read(absolute)
    }

    pub(super) fn terminal_is_complete(&self) -> bool {
        self.component_complete && self.component_fault.is_none() && self.allocated_pages == self.layout.page_count
            && self.written.iter().enumerate().all(|(index, count)| {
                *count
                    == self.schema.field_items([Mesh3dField::Positions, Mesh3dField::Normals, Mesh3dField::Indices, Mesh3dField::FaceIds, Mesh3dField::VertexIds, Mesh3dField::Edges, Mesh3dField::EdgeIds, Mesh3dField::Uvs, Mesh3dField::Colors,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive,Mesh3dField::Tangents][index])
            })
    }

    pub(super) fn close_demands(&self,work:usize)->Result<RetirementDemand,Mesh3dFault> {
        if self.close_page<self.allocated_pages{return Ok(RetirementDemand{release_bytes:MESH3D_PAGE_BYTES,depth:1,..Default::default()});}
        if let Some(owner)=&self.component_retirement{return controlled_demands(owner,work);}
        if self.component_references.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=&self.scratch_retirement{return controlled_demands(owner,work);}
        if self.component_seen.owns_backing()||self.component_ineligible.owns_backing(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        Ok(RetirementDemand{release_bytes:if self.pages.is_some(){mesh3d_begin_capacity_byte_demand()}else{0},depth:1,..Default::default()})
    }

    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<Mesh3dOwnershipStep,Mesh3dFault> {
        self.closing=true;
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(Mesh3dOwnershipStep::default());}
        if self.close_page<self.allocated_pages {
            if grant.maximum_release_bytes<MESH3D_PAGE_BYTES{return Ok(Mesh3dOwnershipStep::default());}
            self.pages.as_mut().unwrap()[usize::from(self.close_page)]=None;self.close_page+=1;
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,released_bytes:MESH3D_PAGE_BYTES,..Default::default()},..Default::default()});
        }
        if let Some(owner)=self.component_retirement.as_mut() {
            let step=owner.step(grant).map_err(|_|Mesh3dFault::Closing)?;let progress=step.progress();
            if owner.terminal_is_empty(){self.component_retirement=None;}
            return Ok(Mesh3dOwnershipStep{progress,..Default::default()});
        }
        if let Some(table)=self.component_references.take() {
            match ControlledRetirement::new(table){Ok(owner)=>self.component_retirement=Some(owner),Err((_,table))=>{self.component_references=Some(table);return Err(Mesh3dFault::Closing);}}
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});
        }
        if let Some(owner)=self.scratch_retirement.as_mut() {
            let step=owner.step(grant).map_err(|_|Mesh3dFault::Closing)?;let progress=step.progress();
            if owner.terminal_is_empty(){self.scratch_retirement=None;}
            return Ok(Mesh3dOwnershipStep{progress,..Default::default()});
        }
        if self.component_seen.owns_backing()||self.component_ineligible.owns_backing() {
            let original=(std::mem::take(&mut self.component_seen),std::mem::take(&mut self.component_ineligible));
            self.scratch_retirement=Some(ControlledRetirement::new(original).map_err(|_|Mesh3dFault::Closing)?);
            self.component_probe=None;
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});
        }
        let release=if self.pages.is_some(){mesh3d_begin_capacity_byte_demand()}else{0};
        if grant.maximum_release_bytes<release{return Ok(Mesh3dOwnershipStep::default());}
        self.pages=None;self.layout.total_bytes=0;self.layout.page_count=0;self.allocated_pages=0;self.written=[0;14];
        Ok(Mesh3dOwnershipStep{complete:true,progress:RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}})
    }

}

pub(super) enum Mesh3dSlotState {
    Writing(Mesh3dOwner),
    Ready(Mesh3dOwner),
    Closing(Mesh3dOwner),
    Transition,
}

impl Drop for Mesh3dOwner {
    fn drop(&mut self) {assert!(std::thread::panicking()||(self.pages.is_none()&&self.component_references.is_none()&&self.component_retirement.is_none()&&self.scratch_retirement.is_none()&&!self.component_seen.owns_backing()&&!self.component_ineligible.owns_backing()),"mesh owner dropped before admitted terminal retirement");}
}
//#endregion Mesh
