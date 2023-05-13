use std::ops::Deref;

pub mod reference_cycles;
pub mod tracker;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T> Deref for Rectangle<T> {
    type Target = Rectangle<T>;
    // 等价于下面
    // type Target = T;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}")
}
pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
