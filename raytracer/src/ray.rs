pub use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Ray{
    pub ori: Vec3,
    pub dic: Vec3,
   pub tm:f64,
}
impl Ray{
    pub fn new(ori:Vec3,dic:Vec3,tm:f64) -> Self {
        Self { ori, dic,tm }
    }
    pub fn at(&self,t:f64)->Vec3{
       return  self.ori+self.dic*t
    }
}