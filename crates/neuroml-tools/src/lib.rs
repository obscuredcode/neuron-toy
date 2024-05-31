#![allow(non_snake_case, non_camel_case_types)]

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use strum::ParseError;
use strum_macros::EnumString;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent;

#[macro_use] extern crate strum_macros;

#[derive(EnumString, Display, Eq, PartialEq)]
enum ElementTags {
    neuroml,
    include,
    morphology,
    ionChannel,
    ionChannelHH,
    ionChannelVShift,
    ionChannelKS,
    biophysicalProperties,
    membraneProperties,
    intracellularProperties,
    extraCellularProprties
}

enum SequenceState {

}

/// [Schema](https://docs.neuroml.org/Userdocs/NeuroMLv2.html)
pub struct NeuroMLDocument {
    pub id: String,
    includes: Option<Vec<IncludeType>>,
    extracellularProperties: Option<Vec<ExtracellularProperties>>,
    intracellularProperties: Option<Vec<IntracellularProperties>>,
    cells: Option<Vec<Cell>>,
    izhikevichCells: Option<Vec<IzhikevichCell>>,
    izhikevich2007Cells: Option<Vec<Izhikevich2007Cell>>,
    morphology: Option<Vec<Morphology>>,
    ionChannel: Option<Vec<IonChannel>>,
    ionChannelHH: Option<Vec<IonChannelHH>>,
    ionChannelVShift: Option<Vec<IonChannel>>,
    ionChannelKS: Option<Vec<IonChannelKS>>,
    network: Option<Vec<IonChannelHH>>,
    ComponentType: Option<Vec<ComponentType>>

}

impl NeuroMLDocument {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            includes: None,
            extracellularProperties: None,
            intracellularProperties: None,
            cells: None,
            izhikevichCells: None,
            izhikevich2007Cells: None,
            morphology: None,
            ionChannel: None,
            ionChannelHH: None,
            ionChannelVShift: None,
            ionChannelKS: None,
            network: None,
            ComponentType: None,
        }
    }
}

pub struct IncludeType {
    segmentGroup: String,
    Path: String
}

pub struct ExtracellularProperties {

}

pub struct IntracellularProperties {

}

pub struct Morphology {

}

pub struct IonChannel {

}

pub struct IonChannelHH {

}

pub struct IonChannelVshift {

}

pub struct IonChannelKS {

}

pub struct IzhikevichCell {
    id: String,
    v0: String,
    thresh: String,
    a: String,
    b: String,
    c: String,
    d: String,

}

pub struct Izhikevich2007Cell {
    id: String,
    v0: String,
    k: String,
    vr: String,
    vt: String,
    vpeak: String,
    a: String,
    b: String,
    c: String,
    d: String,

}

pub struct Cell {
    id: String,
    metaid: Option<String>,
    biophysicalProperties: Option<Vec<BiophysicalProperties>>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            id: "".to_string(),
            metaid: None,
            biophysicalProperties: None,
        }
    }
}

pub struct BiophysicalProperties {
    id: String,
    membraneProperties: MembraneProperties

}

impl BiophysicalProperties {
    fn new() -> Self {
        Self {
            id: "".to_string(),
            membraneProperties: MembraneProperties::default()
        }
    }
}

#[derive(Default)]
struct MembraneProperties {
    id: String,
    channelPopulation: Option<Vec<ChannelPopulation>>,
    channelDensity: Option<Vec<ChannelDensity>>,
    specificCapacitance: Option<Vec<SpecificCapacitance>>
}


struct ChannelPopulation {
    id: String,
    ionChannel: String,
    number: String,
    erev: String,
    segment: Option<String>,
    ion: String,
}

struct ChannelDensity {
    id: String,
    ionChannel: String,
    erev: String,
    segmentGroup: Option<String>,
    ion: String,
}

struct SpecificCapacitance {

}

pub struct Network {

}

pub struct ComponentType {

}

fn unrecognized_tag(str: String) -> Result<ElementTags, ParseError> {
    println!("unrecognized tag");
    Ok(ElementTags::include)
}

fn find_attribute(name: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
    let mut ret: Option<String> = None;
    attrs.iter().map(|a| {
        if a.name.local_name.as_str() == name {
            ret = Some(a.value.clone())
        }
        ()
    }).collect::<Vec<_>>();
    ret
}


pub fn load_neuroml(name: &str, neuroml: &mut NeuroMLDocument) -> Result<(), std::io::Error>  {
    let file = File::open(name)?;
    let file = BufReader::new(file);
    println!("load_neuroml");


    let mut parser = EventReader::new(file);
    let mut depth = 0;
    let mut current_tag = ElementTags::neuroml;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes,.. }) => {
                let l = attributes.len();
                let tag = name.local_name.as_str();
                match tag {
                    "neuroml" => {
                        neuroml.id = find_attribute("id", &attributes).unwrap();
                    }
                    "include" => {
                        if neuroml.includes.is_none() {
                            neuroml.includes = Some(Vec::new())
                        }
                        for attr in attributes {
                            neuroml.includes.as_mut().unwrap().push(IncludeType {
                                segmentGroup: attr.name.local_name,
                                Path: attr.value
                            });
                        }

                    }
                    "ionChannelHH" => {

                    }
                    "izhikevich" => {
                        if neuroml.izhikevichCells.is_none() {
                            neuroml.izhikevichCells = Some(Vec::new())
                        }
                        let cell = IzhikevichCell {
                            id: find_attribute("id", &attributes).unwrap(),
                            v0: find_attribute("v0", &attributes).unwrap(),
                            thresh: find_attribute("thresh", &attributes).unwrap(),
                            a: find_attribute("a", &attributes).unwrap(),
                            b: find_attribute("b", &attributes).unwrap(),
                            c: find_attribute("c", &attributes).unwrap(),
                            d: find_attribute("d", &attributes).unwrap(),
                        };
                        neuroml.izhikevichCells.as_mut().unwrap().push(cell);
                    }
                    "izhikevich2007" => {
                        if neuroml.izhikevich2007Cells.is_none() {
                            neuroml.izhikevich2007Cells = Some(Vec::new())
                        }
                        let cell = Izhikevich2007Cell {
                            id: find_attribute("id", &attributes).unwrap(),
                            v0: find_attribute("v0", &attributes).unwrap(),
                            k: find_attribute("k", &attributes).unwrap(),
                            vr: find_attribute("vr", &attributes).unwrap(),
                            vt: find_attribute("vt", &attributes).unwrap(),
                            vpeak: find_attribute("vpeak", &attributes).unwrap(),
                            a: find_attribute("a", &attributes).unwrap(),
                            b: find_attribute("b", &attributes).unwrap(),
                            c: find_attribute("c", &attributes).unwrap(),
                            d: find_attribute("d", &attributes).unwrap(),
                        };
                        neuroml.izhikevich2007Cells.as_mut().unwrap().push(cell);
                    }
                    "cell" => {
                        if neuroml.cells.is_none() {
                            neuroml.cells = Some(Vec::new())
                        }
                        let mut cell = Cell::new();
                        cell.id = find_attribute("id", &attributes).unwrap();
                        cell.metaid = find_attribute("metaid", &attributes);
                        neuroml.cells.as_mut().unwrap().push(cell);
                    }
                    "biophysicalProperties" => {
                        let cell = neuroml.cells.as_mut().unwrap()
                            .last_mut().expect(""); // TODO: parser errors

                        if cell.biophysicalProperties.is_none() {
                            cell.biophysicalProperties = Some(Vec::new());
                        }
                        let mut bpp = BiophysicalProperties::new();
                        bpp.id = find_attribute("id", &attributes).unwrap();
                        cell.biophysicalProperties.as_mut().unwrap().push(bpp);
                        current_tag = ElementTags::biophysicalProperties;
                    }
                    "membraneProperties" => {
                        if current_tag == ElementTags::biophysicalProperties {
                            current_tag = ElementTags::membraneProperties;
                        }
                    }
                    "channelPopulation" => {
                        if current_tag == ElementTags::membraneProperties {
                            let cell = neuroml.cells.as_mut().unwrap()
                                .last_mut().expect("");
                            let bpp = cell.biophysicalProperties.
                                as_mut().unwrap().last_mut().expect("");
                            if bpp.membraneProperties.channelPopulation.is_none() {
                                bpp.membraneProperties.channelPopulation = Some(Vec::new());
                            }
                            let cp = ChannelPopulation {
                                id: find_attribute("id", &attributes).unwrap(),
                                ionChannel: find_attribute("ionChannel", &attributes).unwrap(),
                                number: find_attribute("number", &attributes).unwrap(),
                                erev: find_attribute("erev", &attributes).unwrap(),
                                segment: find_attribute("segment", &attributes),
                                ion: find_attribute("ion", &attributes).unwrap()
                            };
                            bpp.membraneProperties.channelPopulation.as_mut().unwrap().push(cp);
                        }
                    }
                    "channelDensity" => {
                        if current_tag == ElementTags::membraneProperties {
                            let cell = neuroml.cells.as_mut().unwrap()
                                .last_mut().expect("");
                            let bpp = cell.biophysicalProperties.
                                as_mut().unwrap().last_mut().expect("");
                            if bpp.membraneProperties.channelDensity.is_none() {
                                bpp.membraneProperties.channelDensity = Some(Vec::new());
                            }
                            let cd = ChannelDensity {
                                id: find_attribute("id", &attributes).unwrap(),
                                ionChannel: find_attribute("ionChannel", &attributes).unwrap(),
                                erev: find_attribute("erev", &attributes).unwrap(),
                                segmentGroup: Some(find_attribute("segmentGroup", &attributes).
                                    unwrap_or("all".to_string())),
                                ion: find_attribute("ion", &attributes).unwrap()
                            };
                            bpp.membraneProperties.channelDensity.as_mut().unwrap().push(cd);
                        }
                    }

                    _ => {println!("found {tag}");}
                }

                println!("{:spaces$}+{name}, {l}", "", spaces = depth * 2);
                /*let attrs: Vec<_> = attributes.iter().map(|a| {

                    println!("attr {tag}: {:?}: {:?}", a.name.local_name, a.value);
                }).collect();*/
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}