#[cfg(test)]
mod ut_signal {
    use rs_obs_signal::*;
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