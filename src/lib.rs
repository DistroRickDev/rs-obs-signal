//! This crate implements part of the [observer pattern] <https://refactoring.guru/design-patterns/observer> 
//! with variadic template arguments
//!
//! It uses rust macros and generics in order to create a `Signal` object, 
//! that subscribes to a given function or method and is able to notify on demand
//! 
//! # Example
//! ```rust
//! struct Actor {
//!     actor_id: i64,
//!     signal: signal!(&Observer, i64)  
//! }

//! impl Actor {
//!     fn subscribe_to_observer(&mut self){
//!         self.signal.subscribe(Observer::on_notified)
//!     }
//!     fn notify(&self, observer: &Observer){
//!         notify!(&self.signal, observer, self.actor_id);
//!     }
//! }

//! impl Observer {
//!     fn on_notified(&self, actor_id: i64){
//!         println!("Notified by actor with id: {}", actor_id)
//!     } 
//! }

//! fn main() {
//!     let observer  = Observer{};
//!     let mut actor = Actor{
//!         actor_id: 42,
//!         signal: Signal::new()
//!     };
//!     actor.subscribe_to_observer();
//!     actor.notify(&observer);
//! }    
//! ```
 
#![warn(missing_docs)]
/// This struct holds a subscription with X arguments of Y types
pub struct Signal<T>{
    subscription : Option<T>
}

impl <T> Signal<T>{
    /// Creates a new Signal object with empty subscription
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

#[cfg(test)]
mod ut_signal {
    use super::Signal;
    struct Observer;
    
    struct Actor{
        actor_id: i64,
        signal: signal!(&Observer, i64)  
    }
    
    impl Actor{
        fn subscribe_to_observer(&mut self){
            self.signal.subscribe(Observer::on_notified)
        }
        fn notify(&self, observer: &Observer){
            notify!(&self.signal, observer, self.actor_id);
        }
    }
    
    impl Observer{
        fn on_notified(&self, actor_id: i64){
            println!("Notified by actor with id: {}", actor_id)
        } 
    }
    
    #[test]
    fn test_signal_two_objects(){
        let observer  = Observer{};
        let mut actor = Actor{
            actor_id: 42,
            signal: Signal::new()
        };
        assert_eq!(true, actor.signal.subscription.is_none());
        actor.subscribe_to_observer();
        assert_eq!(true, actor.signal.subscription.is_some());
        actor.notify(&observer);
    }
    
}
