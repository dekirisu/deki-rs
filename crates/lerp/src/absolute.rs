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
                    *self =  self.add_qucy(delta.signum()*step,min,max);
                    false
                }
            }
        }
        #f32 #f64 #i8 #i16 #i32 #i64 #i128 #isize
    }

// Tests \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stapable_i32 () {
        let mut num = 1i32;
        assert!{!num.sterp(4,2)}
        assert!{num.sterp(4,5)}
        assert_eq!{num,4}
        assert!(!num.sterp(-4,6));
        assert!(num.sterp(-4,1337));
        assert_eq!(num,-4);
    }

    #[test]
    fn stapable_f32 () {
        let mut num = 1.0f32;
        assert!{!num.sterp(2.,0.6)}
        assert!{num.sterp(2.,0.6)}
        assert_eq!{num,2.}
        assert!(!num.sterp(-2.,3.2));
        assert!(num.sterp(-2.,100.));
        assert_eq!(num,-2.);
    }

    #[test]
    fn stapable_auto_left_overflow(){
        let mut num = 2.9f32;
        let (min,max) = (1.5,8.);
        assert!(!num.sterp_qucy(7.,1.,min,max));
        assert!(num < 2.);
        assert!(!num.sterp_qucy(7.,1.,min,max));
        assert!(num > 7.);
        assert!(num.sterp_qucy(7.,1.,min,max));
        assert_eq!(num,7.);
    }

    #[test]
    fn stapable_auto_right_overflow(){
        let mut num = 6.9f32;
        let (min,max) = (1.5,8.);
        assert!(!num.sterp_qucy(3.,1.,min,max));
        assert!(num > 7.);
        assert!(!num.sterp_qucy(3.,1.,min,max));
        assert!(num < 2.5);
        assert!(num.sterp_qucy(3.,1.,min,max));
        assert_eq!(num,3.);
    }

}

// EOF \\
