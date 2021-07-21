use crate::Vec3;

pub struct Onb{
    axis0:Vec3,
    axis1:Vec3,
    axis2:Vec3,



}
impl Onb{
    pub fn u(&self)->Vec3{
        return self.axis0
    }
    pub fn v(&self)->Vec3{
        return self.axis1
    }
    pub fn w(&self)->Vec3{
        return self.axis2
    }
    pub fn local(&self,a:f64,b:f64,c:f64)->Vec3{
        return self.u()*a+self.v()*b+self.w()*c;
    }
    pub fn build_from(n:&Vec3)->Self{
        let mut a =Vec3::zero();
        let axiss2=n.unit();

        if axiss2.x.abs()>0.9 {
            a=Vec3::new(0.0,1.0,0.0);
        }else {
            a=Vec3::new(1.0,0.0,0.0);
        }
        let axiss1=Vec3::cross(axiss2,a).unit();
        let axiss0=Vec3::cross(axiss2,axiss1);
        Self{
            axis0: axiss0,
            axis1: axiss1,
            axis2: axiss2,
        }

    }
}