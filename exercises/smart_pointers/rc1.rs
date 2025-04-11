// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.



use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    /*
    根据代码中的注释，每个行星都应该拥有同一个Sun的Rc实例，这样它们的引用计数会增加。
    但在当前的代码中，比如Saturn、Uranus、Neptune这三个行星的创建方式有问题。
    比如，Saturn被赋值为Planet::Saturn(Rc::new(Sun {}))，这会导致每个行星都创建一个新的Sun实例，而不是共享原来的那个。这样它们的Rc引用计数就不会增加到主sun变量的计数上，导致后面的断言失败。
    所以，正确的做法应该是让这三个行星也使用Rc::clone(&sun)，而不是新建Sun实例。这样它们的Sun引用都是指向同一个Rc实例，从而正确增加引用计数。
     */
    // TODO
    //let saturn = Planet::Saturn(Rc::new(Sun {}));
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    // TODO
    //let uranus = Planet::Uranus(Rc::new(Sun {}));
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    // TODO
    //let neptune = Planet::Neptune(Rc::new(Sun {}));
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);
/*
接下来看main函数中的各个步骤。初始时，sun的引用计数是1。然后每个行星的创建都是通过Rc::clone(&sun)，所以每创建一个，计数加1。

在后面的drop部分，当释放neptune、uranus、saturn时，主sun的计数应该减少3次，所以原来的计数应该是9，减到6
所有行星变量在main函数作用域内保持存活，直到被显式drop。这保证了它们的Rc引用在整个生命周期内有效。
通过显式drop每个行星变量，逐步减少引用计数，最终回到初始的1。
*/
    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    // TODO
    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    // TODO
   drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    // TODO
    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
