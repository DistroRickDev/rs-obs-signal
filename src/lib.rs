//!This crate implements part of the [observer pattern] <https://refactoring.guru/design-patterns/observer> 
//!with variadic template arguments
//!
//!It uses rust macros and generics in order to create a `Signal` object, 
//!that subscribes to a given function or method and is able to notify on demand
//!

#![warn(missing_docs)]
/// This struct holds a subscription with X arguments of Y types
pub struct Signal<T>{    
    /// Contained subscription callable object 
    pub subscription : Option<T>
}

impl <T> Signal<T>{
    /// Creates a new Signal with support for a single argument object with empty subscription
    pub fn new() -> Self{
        Self{
            subscription: None
        }
    }
    /// Metohd responsible to store function/method into the contained subscription
    pub fn subscribe(&mut self, subscription: T){
        self.subscription = Some(subscription)
    }

}

#[macro_export]
/// Macro helper to create variadic template argument signal
macro_rules! signal{
    ($( $argument_type:ty),*) => {
        Signal<fn($( $argument_type), *) -> ()>
    };  
}

#[macro_export]
/// Macro helper to call inner multi argument subscription 
/// with unfolding variadic arguments
macro_rules! notify{
    ($signal: expr, $( $args:expr),*) => {
        assert_ne!($signal.subscription.is_none(), true);
        $signal.subscription.unwrap()($( $args),*);
    }
}
