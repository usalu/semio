//! 🧾️ Original validation findings with a funded append and direct indexed access.

use super::ValidationIssue;
use semio_framework_value::{RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};

const PAGE_ROWS:usize=32;
const MAXIMUM_PAGES:usize=512;

#[derive(Default)]
pub(super) struct ValidationFindings {
    first:Option<ValidationIssue>,
    pages:Vec<Vec<ValidationIssue>>,
    tail_len:usize,
}
semio_framework_value::artifact_retire_struct!(ValidationFindings{first,pages,tail_len});

impl ValidationFindings {
    pub(super) fn len(&self)->usize{usize::from(self.first.is_some())+self.tail_len}

    pub(super) fn is_empty(&self)->bool{self.first.is_none()}

    pub(super) fn get(&self,index:usize)->Option<&ValidationIssue>{
        match index {
            0=>self.first.as_ref(),
            index=>self.pages.get((index-1)/PAGE_ROWS)?.get((index-1)%PAGE_ROWS),
        }
    }

    pub(super) fn get_mut(&mut self,index:usize)->Option<&mut ValidationIssue>{
        match index {
            0=>self.first.as_mut(),
            index=>self.pages.get_mut((index-1)/PAGE_ROWS)?.get_mut((index-1)%PAGE_ROWS),
        }
    }

    pub(super) fn append_demand(&self)->Result<RetirementDemand,ValueError>{
        let capacity_bytes=if self.first.is_none()||self.pages.last().is_some_and(|page|page.len()<PAGE_ROWS){0}
        else if self.pages.capacity()==0{MAXIMUM_PAGES*size_of::<Vec<ValidationIssue>>()}
        else if self.pages.len()<MAXIMUM_PAGES{PAGE_ROWS*size_of::<ValidationIssue>()}
        else{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"validation exceeds the retained finding limit"));};
        Ok(RetirementDemand{capacity_bytes,depth:2,..Default::default()})
    }

    pub(super) fn reserve_append(&mut self)->Result<RetainedCloneProgress,ValueError>{
        let allocation=||ValueError::literal(ValueRefusalKind::AllocationFailed,"validation finding storage allocation failed");
        let retained_capacity_bytes=if self.pages.capacity()==0{
            self.pages.try_reserve_exact(MAXIMUM_PAGES).map_err(|_|allocation())?;
            self.pages.capacity()*size_of::<Vec<ValidationIssue>>()
        }else{
            let mut page=Vec::new();
            page.try_reserve_exact(PAGE_ROWS).map_err(|_|allocation())?;
            let capacity=page.capacity()*size_of::<ValidationIssue>();
            self.pages.push(page);
            capacity
        };
        Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes,..Default::default()})
    }

    pub(super) fn push_reserved(&mut self,issue:ValidationIssue)->Result<(),ValidationIssue>{
        if self.first.is_none(){self.first=Some(issue);return Ok(());}
        let Some(page)=self.pages.last_mut().filter(|page|page.len()<PAGE_ROWS)else{return Err(issue);};
        page.push(issue);
        self.tail_len+=1;
        Ok(())
    }

    pub(super) fn push_cold(&mut self,issue:ValidationIssue){
        if let Err(issue)=self.push_reserved(issue){
            self.pages.push(Vec::with_capacity(PAGE_ROWS));
            self.pages.last_mut().expect("validation page inserted").push(issue);
            self.tail_len+=1;
        }
    }

    #[cfg(test)]
    pub(super) fn iter(&self)->impl Iterator<Item=&ValidationIssue>{
        self.first.iter().chain(self.pages.iter().flatten())
    }

    pub(super) fn into_iter(self)->impl Iterator<Item=ValidationIssue>{
        self.first.into_iter().chain(self.pages.into_iter().flatten())
    }
}
