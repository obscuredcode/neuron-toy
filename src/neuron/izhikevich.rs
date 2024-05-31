use std::cell::RefCell;
use std::rc::Rc;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Serialize)]
pub struct Izhikevich {
    pub v: f64,
    pub u: f64,
    threshold: f64,
    a: f64,
    b: f64,
    reset_potential: f64,
    d: f64,
    #[serde(skip_serializing)]
    sg: Box<dyn SpikeGenerator>,
    input_current: f64,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumString)]
pub enum IzhikevichParams {
    tonic_spiking,
    phasic_spiking,
    tonic_bursting,
    phasic_bursting,
    mixed_mode,
    spike_freq_adapt,
    Class_1_excit,
    Class_2_excit,
    spike_latency,
    subthreshold_osc,
    resonator,
    integrator,
    rebound_spike,
    rebound_burst,
    thresh_variability,
    bistability,
    DAP,
    accomodation,
    inh_induced_sp,
    inh_induced_brst,
    intrinsically_bursting,
    fast_spiking,
}
use IzhikevichParams::*;
use crate::neuron::engine::{NeuronEngine, SpikeGenerator};
use crate::neuron::network::Listeners;

impl Izhikevich {
    pub fn new(sg: Box<dyn SpikeGenerator>, params: IzhikevichParams) -> Self {
        let mut a = 0.02;
        let mut b = 0.2;
        let mut c = -80.0;
        #[allow(unused_assignments)]
        let mut d = -8.0;
        let mut v0 = -70.0;
        let mut u_override = 0.0;
        match params {
            tonic_spiking => {c=-65.0;d=6.0;},
            phasic_spiking => {b=0.25;c=-65.0;d=6.0;v0=-64.0;},
            tonic_bursting => {c=-50.0;d=2.0},
            phasic_bursting => {b=0.25;c=-55.0;d=0.05},
            mixed_mode => {c=-55.0;d=4.0;},
            spike_freq_adapt => {a=0.01;d=8.0; v0=-65.0},
            Class_1_excit => {b=-0.1; c=-55.0;d=6.0; v0=-60.0},
            Class_2_excit => {a=0.2;b=-0.26; c=-65.0;d=0.0; v0=-64.0},
            spike_latency => {c=-65.0; d=6.0;},
            subthreshold_osc => {a=0.05;b=0.26;c=-60.0;d=0.0; v0=-62.0},
            resonator => {a=0.1;b=0.26; c=-60.0;d=-1.0; v0=-62.0},
            integrator => {a=0.02;b=-0.1; c=-55.0;d=6.0; v0=-60.0},
            rebound_spike => {a=0.03;b=0.25;c=-60.0; d=4.0;v0=-64.0},
            rebound_burst => {a=0.03;b=0.25;c=-52.0;d=0.0; v0=-64.0},
            thresh_variability => {a=0.03;b=0.25; c=-60.0;d=4.0; v0=-64.0},
            bistability => {a=0.1;b=0.26; c=-60.0;d=0.0; v0=-61.0},
            DAP => {a=1.0;b=0.2;c=-60.0;d=-21.0;},
            accomodation => {a=0.02;b=1.0;c=-55.0;d=4.0; v0=-65.0; u_override=-16.0},
            inh_induced_sp => {a=0.02;b=-1.0; c=-60.0;d=8.0; v0=-63.8},
            inh_induced_brst => {a=0.026;b=-1.0; c=-45.0;d=-2.0; v0=-63.8},
            intrinsically_bursting => {a=0.1;c=-55.0;d=4.0;},
            fast_spiking => {a=0.1;b=0.3;c=-65.0; d=2.0; v0 = -65.0}
        }

        let u = if u_override == 0.0 {b * v0} else {u_override};

        Izhikevich {
            v: v0,
            u: u,
            threshold: 30.0,
            a: a, // recovery-timescale
            b: b, // senstivity of U to V
            reset_potential: c,
            d: d,
            sg: sg,
            input_current: 0.0
        }
    }
}

impl NeuronEngine for Izhikevich {
    fn step(&mut self, listeners: Rc<RefCell<Listeners>>, dt: f64) -> (bool, f64) {
        let i = self.sg.step(dt) + self.input_current;// * 8.0e12; // pA
        self.input_current = 0.0;
        let v2 = self.v * self.v;
        let dv = (0.04 * v2) + (5.0 * self.v) + 140.0 - self.u + i;
        let du = self.a * ((self.b * self.v) - self.u);
        //println!("I {}, dv: {}, v2: {}, 5v: {}, du: {}", i, dv, v2, 5.0*self.v, du);
        self.v += dv * dt;
        self.u += du * dt;
        let mut fired = false;
        if self.v >= self.threshold {
            // spike
            //println!("spike {} u {}", self.v, self.u);
            fired = true;
            listeners.borrow().inform();
            self.v = self.reset_potential;
            self.u += self.d;
        }

        (fired, i)
    }

    fn reset(&mut self) {
        todo!()
    }

    fn receive(&mut self, curr: f64) {
        self.input_current += curr;
    }

    fn get_membrane_potential(&self) -> f64 {
        self.v * 1.0e-3
    }
}