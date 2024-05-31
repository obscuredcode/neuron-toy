mod neuron;

use std::cell::RefCell;
use std::rc::Rc;
use plotters::chart::DualCoordChartContext;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use crate::neuron::engine::{NeuronEngine, GaussianSG, DCSG, SingleSpike};
use crate::neuron::izhikevich::*;

use plotters::prelude::*;
use typed_arena::Arena;
use crate::neuron::network::{Listeners, Node, Synapse};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rand = Box::new(GaussianSG::new(0.05));
    let dc = Box::new(DCSG::new(10.0e0));
    let single = Box::new(SingleSpike::new(14.0e0,100.0,10.0));
    let sg = dc;
    let params = IzhikevichParams::spike_freq_adapt;
    let params_name = format!("Izhikevich {:?}", params);
    //let model: &mut Izhikevich = &mut Izhikevich::new(sg, params);
    let arena : Arena<Node<Izhikevich>> = Arena::new();
    let mut n1 = Rc::new(RefCell::new(Node::new(Izhikevich::new(sg,
                                           IzhikevichParams::phasic_spiking), &arena)));
    let mut n2 = Rc::new(RefCell::new(Node::new(Izhikevich::new(Box::new(DCSG::new(0.0)),
                                           IzhikevichParams::intrinsically_bursting), &arena)));
    let mut n3 = Rc::new(RefCell::new(Node::new(Izhikevich::new(Box::new(DCSG::new(0.0)),
                                                                IzhikevichParams::tonic_spiking), &arena)));


    n1.borrow_mut().listeners.borrow_mut().add(Box::new(|| {
        println!("fired n1");
    }));

    //n1.borrow_mut().add_downstream(n2.clone());
    let syn = Synapse::new(n2.clone(), 30.0, 3.0);
    n1.borrow_mut().add_downstream(syn);
    //n2.borrow_mut().add_downstream(n3.clone());

    n2.borrow_mut().listeners.borrow_mut().add(Box::new(|| {
        println!("fired n2");
    }));


    //let n1 = Node::new(model, &arena);



    /*&mut IntegrateFire {
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

    };*/

    let len = 4000;
    let time_step : f64 = 0.1;

    let root = BitMapBackend::new("0.png", (2*640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((2,2));
    let (left, right,misc1,misc2) =  (&areas[0], &areas[1], &areas[2], &areas[3]);


    let mut left_chart = ChartBuilder::on(&left)
        .caption("n1", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .right_y_label_area_size(30)
        .build_cartesian_2d(0..len, -110f32..50f32)?
        .set_secondary_coord(0..len, -1.0e2f32..1.0e2f32);


    let mut right_chart = ChartBuilder::on(&right)
        .caption("n2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .right_y_label_area_size(30)
        .build_cartesian_2d(0..len, -110f32..50f32)?
        .set_secondary_coord(0..len, -1.0e2f32..1.0e2f32);

    let mut misc_chart = ChartBuilder::on(&misc1)
        .caption("n2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .right_y_label_area_size(30)
        .build_cartesian_2d(-1.0e2f32..1.0e2f32, -1.0e1f32..1.0e1f32)?;

    misc_chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("v")
        .y_desc("u").draw()?;



    config_chart(&mut left_chart, time_step);
    config_chart(&mut right_chart, time_step);

    let mut n1_data: Vec<(f64,f64)> = Vec::new();
    let mut n2_data: Vec<(f64,f64)> = Vec::new();
    let mut phase_n1: Vec<(f64, f64)> = Vec::new();

    for n in 0..=len {
        // build listeners

        //let i = model.step(Rc::new(RefCell::new(listeners)),time_step);

        //println!("potential {}", model.v);
        let i = n1.borrow_mut().step(time_step);
        n1_data.push((n1.borrow().get_potential(), i));
        phase_n1.push((n1.borrow().get_potential(), n1.borrow().engine.borrow().u));
        let i = n2.borrow_mut().step(time_step);
        n2_data.push((n2.borrow().get_potential(), i));
    }


    plot_data(&mut left_chart, &n1_data, len)?;
    plot_data(&mut right_chart, &n2_data, len)?;

    misc_chart
        .draw_series(LineSeries::new(
            (0..=len).map(|x| {
                //println!("potential {}", model.membrane_potential);
                ((phase_n1[x as usize].0 *1.0e3) as f32, (phase_n1[x as usize].1 *1.0e0) as f32)
            }),
            &RED,
        ))?
        .label("v, u phase")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    left_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    right_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    misc_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;





    Ok(())
}


fn config_chart(chart: &mut DualCoordChartContext<BitMapBackend,
                                Cartesian2d<RangedCoordi32, RangedCoordf32>,
                                Cartesian2d<RangedCoordi32, RangedCoordf32>>,
                time_step: f64
) -> Result<(), Box<dyn std::error::Error>> {


    chart.configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|x| format!("{}", ((*x as f64) * time_step).floor()))
        .x_desc("ms")
        .y_desc("potential")
        .draw()?;

    Ok(())
}

fn plot_data(chart: &mut DualCoordChartContext<BitMapBackend, Cartesian2d<RangedCoordi32, RangedCoordf32>, Cartesian2d<RangedCoordi32, RangedCoordf32>>,
            data: &Vec<(f64, f64)>,  len: i32) -> Result<(), Box<dyn std::error::Error>> {

    chart
        .draw_series(LineSeries::new(
            (0..=len).map(|x| {
                //println!("potential {}", model.membrane_potential);
                (x, (data[x as usize].0 * 1e3) as f32)
            }),
            &RED,
        ))?
        .label("membrane_potential")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_secondary_series(LineSeries::new(
            (0..=len).map(|x| {
                //println!("potential {}", model.membrane_potential);
                (x, (data[x as usize].1) as f32)
            }),
            &BLUE,
        ))?
        .label("incoming current")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)],&BLUE));

    Ok(())
}