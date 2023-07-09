pub struct Signal<T>{
    subscription : Option<T>
}

impl <T> Signal<T>{
    pub fn new() -> Self{
        Self{
            subscription: None
        }
    }
    pub fn subscribe(&mut self, subscription: T){
        self.subscription = Some(subscription)
    }

}

#[macro_export]
macro_rules! signal{
    // Macro helper to create variadic template argument signal
    ($( $argument_type:ty),*) => {
        Signal<fn($( $argument_type), *) -> ()>
    };  
}

#[macro_export]
macro_rules! notify{
    // Macro helper to call inner multi argument subscription 
    // with unfolding variadic arguments
    ($signal: expr, $( $args:expr),*) => {
        assert_ne!($signal.subscription.is_none(), true);
        $signal.subscription.unwrap()($( $args),*);
    }
}

#[cfg(test)]
mod ut_signal {
    use super::Signal;
    struct Observer{
        observer_id: i64,     
    }
    
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
        let observer  = Observer{
            observer_id : 0
        };
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
