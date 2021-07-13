pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Ray{
    pub ori: Vec3,
    pub dic: Vec3,
}
impl Ray{
    pub fn new(ori:Vec3,dic:Vec3) -> Self {
        Self { ori, dic }
    }
    pub fn at(&self,t:f64)->Vec3{
        self.ori+self.dic*t
    }
}