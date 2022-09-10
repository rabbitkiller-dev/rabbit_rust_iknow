
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
mod csdn;
mod str_map;




struct EventManager<T> {
    vec: Vec<Box<dyn Fn(T)>>
}




#[test]
fn run() {
    let result = add();
    match result {
        None => {
        },
        Some(value) => {

        }
    }
}

fn add() -> Option<String> {
    Option::Some("num".to_string())
}

// impl<T> EventManager<T> {
//     fn on(&mut self, call: fn(T)) {
//         self.vec.push(call);
//     }

//     fn send(&mut self, value: T) {
//         self.vec.iter().for_each(|f| {
//             f(value);
//         });
//         let mut x: Vec<fn()> = Vec::new();
//         x.iter().for_each(|f| {
//             f();
//         });
//     }
// }


// fn test() {
//     Rc::downgrade(this)
//     // RC::downgrade();
// }

// struct EventManagerContext<T>  {
//     event_map: HashMap<&'static str, Vec<(impl Fn(Event))>>
// }

// impl<T: EventManager> EventManagerContext<T> {
// }


// trait EventManager {
//     type Event;

//     fn on(&self, call: impl Fn(Self::Event), v: Self::Event) {
        
//     }
// }

// #[derive(Debug)]
// enum InputEvent {
//     Change(u32)
// }

// struct InputEventManager;

// impl EventManager for InputEventManager {
//     type Event = InputEvent;
// }

// #[test]
// fn event_map_main() {
//     let em = InputEventManager{};

//     em.on(|e| {

//         match e {
//             InputEvent::Change(val) => {
//                 println!("{}", val);
//             }
//             _ => {}
//         }

//     }
//     , InputEvent::Change(32));
// }