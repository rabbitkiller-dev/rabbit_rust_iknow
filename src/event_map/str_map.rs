
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
enum Event {
    Click(u32),
    DBClick(String),
}

struct EventManager<F>
where F: FnMut(Event) -> () {
    event_map: HashMap<&'static str, Vec<F>>
}

impl<F> EventManager<F>
where F: FnMut(Event) -> () {
    fn new() -> EventManager<F>
    {
        EventManager {
            event_map: HashMap::new()
        }
    }

    fn on(&mut self, key: &'static str, f: F) {
        let regs = match self.event_map.get_mut(key) {
            Some(regs) => {
                regs
            },
            None => {
                self.event_map.insert(key, vec![]);
                self.event_map.get_mut(key).unwrap()
            }
        };
        regs.push(f);
    }

    fn send(&self, key: &'static str, num: Event) {
        let call = self.event_map.get(key);
        match call {
            Some(funVec) => {
                funVec.iter().for_each(move |f| {
                    f(num)
                });
            },
            None => {
                println!("该key没有注册过方法")
            }
            
        }
    }
}

#[test]
fn event_map_main() {
    let mut event_manager = EventManager::new();
    // 事件监听
    event_manager.on("change", |num| {
        println!("测试:{:?}", num);
    });


    // 触发
    event_manager.send("change", Event::Click(212));
}