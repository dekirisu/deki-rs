use crate::*;

// Lerp by Steps \\

    pub trait Stepable {
        fn sterp(&mut self,to:Self,step:Self) -> bool;
    }

    buns::sandwich!{
        impl Stepable for ^0 {
            fn sterp(&mut self,to:Self,step:Self) -> bool {
                let delta = to - *self;
                if delta.abs() <= step {
                    *self = to;
                    true
                } else {
                    *self += delta.signum() * step;
                    false
                }
            }    
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

    // Cyclic Lerp by Steps \\

    pub trait CycleStapable {
        fn sterp_qucy(&mut self,to:Self,step:Self,min:Self,max:Self) -> bool;
    }
    
    buns::sandwich!{
        impl CycleStapable for ^0 { 
            fn sterp_qucy(&mut self,to:Self,step:Self,min:Self,max:Self) -> bool {
                let delta = self.delta_qucy(to,min,max);
                if delta.abs() <= step {
                    *self = to;
                    true
                } else {
                    *self += delta.signum() * step;
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// EOF \\
