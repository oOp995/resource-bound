//!
//! resource-bound-core
//! is designed as part 
//! of resource-bound
//! crate
//! and it is not designed to be used separately
//! 

pub trait StackOnly {}
macro_rules! impl_stack_only {
    ($($t:ty),*$(,)?) => {
        $(impl StackOnly for $t{})*
    };
}


impl_stack_only!(i8,i16,i32,i64,i128);//mark signed-integer     as    stack only     (non-heap)
impl_stack_only!(isize);              //mark isize              as    stack only     (non-heap)
impl_stack_only!(u8,u16,u32,u64,u128);//mark unsigned-integer   as    stack only     (non-heap)
impl_stack_only!(usize);              //mark usize              as    stack only     (non-heap)
impl_stack_only!(f32,f64);            //mark float              as    stack only     (non-heap)
impl_stack_only!(bool);               //mark bool               as    stack only     (non-heap)
impl_stack_only!(char);               //mark char               as    stack only     (non-heap)
impl_stack_only!(());                 //mark ()                 as    stack only     (non-heap)   