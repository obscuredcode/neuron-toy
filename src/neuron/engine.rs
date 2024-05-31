#![allow(unused_variables, dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

use crate::neuron::network;
use crate::neuron::network::Listeners;

pub trait NeuronEngine {
    fn step(&mut self, listeners: Rc<RefCell<Listeners>>, dt: f64) -> (bool, f64);
    fn reset(&mut self);
    fn receive(&mut self, curr: f64);
    fn get_membrane_potential(&self) -> f64;
}

pub trait SpikeGenerator {
    fn step(&mut self, dt: f64) -> f64;
}

pub struct GaussianSG {
    rate: f64
}

impl GaussianSG {
    pub fn new(rate: f64) -> Self {
        GaussianSG {
            rate
        }
    }
}

impl SpikeGenerator for GaussianSG {
    fn step(&mut self, dt: f64) -> f64 {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.rate) { rng.gen_range(0.0..1.0)*1e-9 } else {0.0}
    }
}

pub struct DCSG {
    mag: f64,
    counter: f64
}
impl DCSG {
    pub fn new(mag: f64) -> Self {
        Self {
            mag: mag,
            counter: 0.0
        }
    }
}
impl SpikeGenerator for DCSG {
    fn step(&mut self, dt: f64) -> f64 {
        let mut i = 0.0;
        if self.counter > 0.01 {
            i = self.mag;
        }
        self.counter += dt;
        i
    }
}

pub struct SingleSpike {
    mag: f64,
    pos: f64,
    width: f64,
    counter: f64
}

impl SingleSpike {
    pub fn new(mag: f64, width: f64, pos: f64) -> Self {
        Self {
            mag: mag,
            pos: pos,
            width: width,
            counter: 0.0
        }
    }
}

impl SpikeGenerator for SingleSpike {
    fn step(&mut self, dt: f64) -> f64 {
        let mut i = 0.0;
        if (self.counter - self.pos).abs() < self.width * dt {
            i = self.mag;
            //print!("single");
        }
        self.counter += dt;
        i
    }
}

struct RampGenerator {
    mag: f64,
    width: f64,
    counter: f64
}

impl RampGenerator {
    pub fn new(mag: f64, width: f64) -> Self {
        Self {
            mag: mag,
            width: width,
            counter: 0.0
        }
    }
}












/*

pub struct IntegrateFire {
    pub threshold: f64,
    pub membrane_potential: f64,
    pub reset_potential: f64,
    pub resting_potential: f64,
    pub membrane_capacitance: f64,
    pub membrane_resistance: f64,
    pub refractory_period: f64,
    pub(crate) refractory_counter: f64,
    pub(crate) fired: bool,
    pub(crate) sg: Box<dyn SpikeGenerator>
}

impl IntegrateFire {
    pub fn new(threshold: f64, resting_potential: f64, reset_potential: f64, membrane_capacitance: f64, membrane_resistance: f64, sg: Box<dyn SpikeGenerator>) -> Self {
        Self {
            threshold: threshold,
            membrane_potential: resting_potential,
            reset_potential: reset_potential,
            resting_potential: resting_potential,
            membrane_capacitance: membrane_capacitance,
            membrane_resistance: membrane_resistance,
            refractory_period: 0.0,
            refractory_counter: 0.0,
            fired: false,
            sg: sg
        }
    }
    pub fn default() {
        let sg = Box::new(GaussianSG::new(0.1));
        let model = &mut Self { //&mut Izhikevich ::new(sg);
            membrane_potential: -70.0e-3,
            reset_potential: -80.0e-3,
            resting_potential: -70.0e-3,
            membrane_capacitance: 0.2e-9,
            membrane_resistance: 100.0e6,
            threshold: -55.0e-3,
            fired: false,
            refractory_period: 2.0e-3,
            refractory_counter: 0.0,
            sg: sg
        };
    }
}

impl NeuronEngine for IntegrateFire {
    #[allow(non_snake_case)]
    fn step(&mut self, dt: f64) -> f64{
        let mut I : f64 = 0.0;
        if self.refractory_counter > 0.0 {
            if self.membrane_potential > self.threshold {
                self.membrane_potential = self.reset_potential;
            } else {
                self.membrane_potential +=  1.0 / self.membrane_resistance * (self.membrane_potential - self.reset_potential);
            }
            self.refractory_counter -= dt;
        } else {
            let incoming = self.sg.step(dt);
            let leak = (- 1.0 / self.membrane_resistance) * (self.membrane_potential - self.resting_potential);
            I =   incoming + leak;
            println!("total current {}, leak current: {}", I, leak);
            let dV = I / self.membrane_capacitance * dt;
            self.membrane_potential += dV;

            if self.membrane_potential > self.threshold {
                self.refractory_counter = self.refractory_period;
                self.membrane_potential = 20e-3;
                self.fired = true;
                //self.membrane_potential = self.reset_potential;
                println!("fire");
            }
        }
        I
    }

    fn reset(&mut self) {
        self.membrane_potential = self.reset_potential;
    }

    fn checK_fire(&mut self, listeners: Listeners) {
        todo!()
    }

    fn get_membrane_potential(&self) -> f64 {
        self.membrane_potential
    }
}


pub struct HHGate {
    alpha: f64,
    beta: f64,

}

pub struct HodgkinsHuxley {
}

impl HodgkinsHuxley {
    pub fn new() -> Self {
        HodgkinsHuxley {

        }
    }
}

impl NeuronEngine for HodgkinsHuxley {
    fn step(&mut self, dt: f64) -> f64 {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn check_fire(&mut self, listeners: Listeners) {
        todo!()
    }

    fn get_membrane_potential(&self) -> f64 {
        todo!()
    }
}

 */