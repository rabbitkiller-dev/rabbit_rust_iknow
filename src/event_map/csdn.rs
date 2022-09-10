use std::rc::{Rc, Weak};
use std::sync::{Arc};
 
// 监听事件类型
enum EventType {
    Click,
    DoubleClick,
    Touch,
    None,
}
 
// 监听事件
struct Event {
    e_type: EventType,
}
 
// 处理事件
trait HandleEvent {
    fn on_event(&self, event: &Event);
}
 
// 事件处理者
struct EventHandler;
 
// 实现事件触发时的回调函数
impl HandleEvent for EventHandler {
    fn on_event(&self, event: &Event) {
        match event.e_type {
            EventType::Click => {
                println!("clicked");
            },
            EventType::DoubleClick => {
                println!("double clicked");
            }
            EventType::Touch => {
                println!("touched");
            },
            _ => {
                println!("none");
            },
        }
    }
}
 
// 事件处理者本身的其他功能函数
impl EventHandler {
    pub fn other_func(&self) {
        println!("other_func");
    }
}
 
// 事件管理者
struct Dispatcher<'a> {
    events: Vec<Event>,
    handlers: Vec<Arc<Weak<dyn HandleEvent + 'a>>>,
}
 
impl<'a> Dispatcher<'a> {
    // 注册事件监听者
    pub fn register_handler(&mut self, cb: Weak<impl HandleEvent + 'a>) -> usize {
        self.handlers.push(Arc::new(cb));
        let id = self.handlers.len();
        id - 1
    }
 
    // 反注册事件监听者
    pub fn unregister_handler(&mut self, id: usize) {
        self.handlers.remove(id);
    }
 
    // 发布事件，触发事件回调
    pub fn dispatch_event(&self) {
        for h in self.handlers.iter() {
            for event in self.events.iter() {
                h.upgrade().unwrap().on_event(event);
            }
        }
    }
}

 #[test]
fn maintest() {
    let mut dispatcher = Dispatcher {
        events: vec![Event {
            e_type: EventType::Click,
        }, Event {
            e_type: EventType::DoubleClick,
        }, Event {
            e_type: EventType::Touch,
        }, Event {
            e_type: EventType::None,
        }],
        handlers: vec![],
    };
    let handler = Rc::new(EventHandler);
    let handler_id = dispatcher.register_handler(Rc::downgrade(&handler));
    println!("handler id: {}", handler_id);
    handler.other_func();
    let handler1 = Rc::new(EventHandler);
    let handler1_id = dispatcher.register_handler(Rc::downgrade(&handler1));
    println!("handler1 id: {}", handler1_id);
    handler1.other_func();
    dispatcher.dispatch_event();
    dispatcher.unregister_handler(handler1_id);
    dispatcher.dispatch_event();
}