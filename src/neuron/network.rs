use std::cell::{RefCell, UnsafeCell};
use std::mem::transmute;
use std::ops::Deref;
use std::rc::Rc;
use crate::neuron::engine::NeuronEngine;
use typed_arena::Arena;

pub struct Node<'a, E: NeuronEngine> {
    pub engine: Rc<RefCell<E>>,
    pub listeners: Rc<RefCell<Listeners>>,
    pub dummy: &'a str,
    //Rc<RefCell<&'a mut Self>>
    pub outgoing: UnsafeCell<Vec<Synapse<'a, E>>>,
    //pub listeners: UnsafeCell<Vec<String>>
}

impl<'a, E: NeuronEngine> Node<'a, E> {
    pub fn new(engine: E, arena: &'a Arena<Node<'a, E>>) -> &'a mut Node<'a, E> {
        arena.alloc(Node {
            engine: Rc::new(RefCell::new(engine)),
            listeners: Rc::new(RefCell::new(Listeners::new())),
            //listeners: UnsafeCell::new(Vec::new()),
            outgoing: UnsafeCell::new(Vec::new()),
            dummy: "",
        })
    }

    pub fn get_potential(&self) -> f64 {
        self.engine.borrow().get_membrane_potential()
    }

    pub fn step(&mut self, dt: f64) -> f64 {
        let mut engine = self.engine.borrow_mut();
        let (fired, i) = engine.step(self.listeners.clone(), dt);
        for n in unsafe {(*self.outgoing.get()).iter_mut()} {
            if fired {
                n.fire();
            }
            let curr = n.step(dt);
            n.target.borrow().engine.borrow_mut().receive(curr);
        }
        i
    }

    // neuron: Rc<RefCell<&'a mut Self>>
    pub fn add_downstream(&mut self, syn: Synapse<'a, E>) {
        unsafe {
            self.outgoing.get_mut().push(syn);
        }
    }
}

pub struct Synapse<'a, E: NeuronEngine> {
    target: Rc<RefCell<&'a mut Node<'a, E>>>,
    max_current: f64,
    time_factor: f64,
    counter: f64,
}

impl<'a, E: NeuronEngine> Synapse<'a, E> {
    pub fn new(target: Rc<RefCell<&'a mut Node<'a, E>>>, max_current: f64, time_factor: f64) -> Self {
        Self {
            target: target,
            max_current: max_current,
            time_factor: time_factor,
            counter: -1.0
        }
    }
    pub fn fire(&mut self) {
        self.counter = 0.0;
    }
    pub fn step(&mut self, dt: f64) -> f64 {
        if (self.counter > dt * 1.0e3) {
            self.counter  =  -1.0
        }
        if !self.counter.is_sign_negative() {
            self.counter += dt;
            return self.max_current * (-(self.counter)/self.time_factor).exp();
        }
        0.0
    }
}

pub trait Listener {

}

pub struct Listeners {
    pub listeners: Vec<Box<dyn Fn()>>
}

impl<> Listeners<> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new()
        }
    }
    pub fn add(&mut self, listener: Box<dyn Fn()>)
    {
        self.listeners.push(listener);
    }

    pub fn inform(&self) {
        for listener in &self.listeners {
            listener();
        }
    }
}

pub struct Axon { //<'a, E: NeuronEngine> {
    //from: UnsafeCell<Node<'a, E>>,
    //recepients: UnsafeCell<Vec<&'a Node<'a, E>>>
}

