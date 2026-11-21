//! 🐚️ Certifies each outward closed Mesh shell and the cyclic links at its vertices.

use super::*;

#[derive(Clone, Copy, Default, semio_framework_value::RetireOwned)]
struct Shell {
    vertices: usize,
    edges: usize,
    faces: usize,
    volume: f64,
    opens_on_cutter: bool,
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct VertexLink {
    parent: u32,
    weight: usize,
    first: Option<u32>,
    degree: u32,
    opening_degree: u8,
    edges: usize,
    faces: usize,
    volume: f64,
}

#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct ExteriorLinks {
    vertices: HistoryFoldIndex<u32, VertexLink>,
    successors: HistoryFoldIndex<(u32, u32), u32>,
    shells: HistoryFoldIndex<u32, Shell>,
    cursor: usize,
    walk: Option<(u32, u32, u32, u32)>,
    checking_shells: bool,
}

impl ExteriorLinks {
    pub(super) fn record(&mut self, vertex: u32, previous: u32, next: u32, volume: f64, face_start: bool) -> MeshResult<()> {
        if self.successors.contains_key(&(vertex, previous)) {
            return Err(invalid("partitioned difference repeated an exterior vertex link"));
        }
        self.successors.insert((vertex, previous), next);
        for id in [vertex, next] {
            self.vertices.entry(id).or_insert_with(||VertexLink{
                parent:id,weight:1,first:None,degree:0,opening_degree:0,edges:0,faces:0,volume:0.,
            });
        }
        let link = self.vertices.get_mut(&vertex).unwrap();
        link.first.get_or_insert(previous);
        link.degree += 1;
        link.edges += usize::from(vertex < next);
        link.faces += usize::from(face_start);
        link.volume += volume;
        let (mut a, mut b) = (self.root(vertex), self.root(next));
        if a != b {
            if self.vertices.get(&a).unwrap().weight < self.vertices.get(&b).unwrap().weight {
                std::mem::swap(&mut a, &mut b);
            }
            let child = self.vertices.get_mut(&b).unwrap();
            child.parent = a;
            let weight = child.weight;
            self.vertices.get_mut(&a).unwrap().weight += weight;
        }
        Ok(())
    }

    pub(super) fn opening(&mut self,a:u32,b:u32)->MeshResult<()>{
        for vertex in [a,b]{
            let link=self.vertices.get_mut(&vertex).unwrap();
            if link.opening_degree==2{return Err(invalid("partitioned difference produced a branched opening"));}
            link.opening_degree+=1;
        }
        Ok(())
    }

    fn root(&self, mut id: u32) -> u32 {
        loop {
            let parent = self.vertices.get(&id).unwrap().parent;
            if parent == id { return id; }
            id = parent;
        }
    }

    pub(super) fn advance(&mut self,epsilon:f32)->MeshResult<bool> {
        if self.checking_shells{return self.check_shell(epsilon);}
        if let Some((vertex, first, current, remaining)) = self.walk {
            let next = *self.successors.get(&(vertex, current))
                .ok_or_else(|| invalid("partitioned difference produced an open exterior vertex link"))?;
            self.walk = match (next == first, remaining == 1) {
                (true, true) => None,
                (false, false) => Some((vertex, first, next, remaining - 1)),
                _ => return Err(invalid("partitioned difference produced more than one fan at an exterior vertex")),
            };
            return Ok(false);
        }
        if self.cursor == self.vertices.slot_count() {
            if self.shells.len()==0{return Err(invalid("partitioned difference needs a positive material shell"));}
            self.checking_shells=true;
            self.cursor=0;
            return Ok(false);
        }
        if let Some((&vertex, &link)) = self.vertices.slot_entry(self.cursor) {
            if link.opening_degree==1{return Err(invalid("partitioned difference produced an open cutter rim"));}
            let first = link.first.filter(|_| link.degree >= 2)
                .ok_or_else(|| invalid("partitioned difference needs a closed exterior vertex fan"))?;
            self.walk = Some((vertex, first, first, link.degree));
            let shell=self.shells.entry(self.root(vertex)).or_insert_with(Shell::default);
            shell.vertices+=1;
            shell.edges+=link.edges;
            shell.faces+=link.faces;
            shell.volume+=link.volume;
            shell.opens_on_cutter|=link.opening_degree!=0;
        }
        self.cursor += 1;
        Ok(false)
    }

    fn check_shell(&mut self,epsilon:f32)->MeshResult<bool>{
        if self.cursor==self.shells.slot_count(){return Ok(true);}
        if let Some((_,shell))=self.shells.slot_entry(self.cursor){
            if !shell.opens_on_cutter{return Err(invalid("partitioned difference needs an opening in each material shell"));}
            if !shell.volume.is_finite()||shell.volume<=f64::from(epsilon).powi(3){
                return Err(invalid("partitioned difference produced a shell without outward closed volume"));
            }
            let Some(twice_genus)=(shell.edges+2).checked_sub(shell.vertices+shell.faces)
                else{return Err(invalid("partitioned difference produced invalid shell incidence"));};
            if twice_genus%2!=0{return Err(invalid("partitioned difference produced nonorientable shell incidence"));}
        }
        self.cursor+=1;
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinched_vertex_has_two_closed_fans_and_is_refused() {
        let mut links = ExteriorLinks::default();
        for [a,b,c] in [[0,2,1],[0,1,3],[1,2,3],[2,0,3],[0,5,4],[0,4,6],[4,5,6],[5,0,6]] {
            for (vertex,previous,next) in [(a,c,b),(b,a,c),(c,b,a)] {
                links.record(vertex,previous,next,0.,false).unwrap();
            }
        }
        let error = loop {
            match links.advance(1e-6) {
                Ok(false) => continue,
                Ok(true) => panic!("the shared vertex is pinched"),
                Err(error) => break error,
            }
        };
        assert!(format!("{error:?}").contains("more than one fan"));
    }
}
