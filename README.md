# rs-obs-signal

This crate implements part of the [observer pattern](https://refactoring.guru/design-patterns/observer) with variadic template arguments

It uses rust macros and generics in order to create a `Signal` object, that subscribes to a given function or method and is able to notify on demand 

## Examples
```rust

struct Actor {
    actor_id: i64,
    signal: signal!(&Observer, i64)  
}

impl Actor {
    fn subscribe_to_observer(&mut self){
        self.signal.subscribe(Observer::on_notified)
    }
    fn notify(&self, observer: &Observer){
        notify!(&self.signal, observer, self.actor_id);
    }
}

impl Observer {
    fn on_notified(&self, actor_id: i64){
        println!("Notified by actor with id: {}", actor_id)
    } 
}

fn main() {
    let observer  = Observer{};
    let mut actor = Actor{
        actor_id: 42,
        signal: Signal::new()
    };
    actor.subscribe_to_observer();
    actor.notify(&observer);
}
    
```


## Author

[@DistroRickDev](https://github.com/DistroRickDev)

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

### The MIT License (MIT)
Copyright © 2023 DistroRickDev

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
