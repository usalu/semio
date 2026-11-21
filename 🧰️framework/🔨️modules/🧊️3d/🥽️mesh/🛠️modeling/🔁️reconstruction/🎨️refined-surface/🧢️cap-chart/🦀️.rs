//! 🧢️ Scans a cap boundary to fit a winding-aligned planar UV chart.

pub(crate) struct CapChart {
    start:u32,
    next:u32,
    origin:[f64;3],
    normal:[f64;3],
    minimum:[f64;3],
    maximum:[f64;3],
    axes:Option<[usize;2]>,
}

impl CapChart {
    pub(super) fn new(start:u32,origin:[f32;3])->Self {
        let origin=origin.map(f64::from);
        Self{start,next:start,origin,normal:[0.0;3],minimum:origin,maximum:origin,axes:None}
    }

    pub(super) fn ready(&self)->bool {self.axes.is_some()}
    pub(super) fn next(&self)->u32 {self.next}

    pub(super) fn observe(&mut self,point:[f32;3],end:[f32;3],next:u32) {
        let point=point.map(f64::from);
        let end=end.map(f64::from);
        let a=[0,1,2].map(|axis|point[axis]-self.origin[axis]);
        let b=[0,1,2].map(|axis|end[axis]-self.origin[axis]);
        for axis in 0..3 {
            let left=(axis+1)%3;let right=(axis+2)%3;
            self.normal[axis]+=a[left]*b[right]-a[right]*b[left];
            self.minimum[axis]=self.minimum[axis].min(point[axis]);
            self.maximum[axis]=self.maximum[axis].max(point[axis]);
        }
        self.next=next;
        if next==self.start {
            let axis=(0..3).max_by(|&a,&b|self.normal[a].abs().total_cmp(&self.normal[b].abs())).unwrap();
            let mut axes=[(axis+1)%3,(axis+2)%3];
            if self.normal[axis]<0.0 {axes.swap(0,1);}
            self.axes=Some(axes);
        }
    }

    pub(super) fn uv(&self,point:[f32;3])->[f32;2] {
        self.axes.unwrap().map(|axis|{
            let width=self.maximum[axis]-self.minimum[axis];
            if width==0.0 {0.0}else {((f64::from(point[axis])-self.minimum[axis])/width) as f32}
        })
    }
}
