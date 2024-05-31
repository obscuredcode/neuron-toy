

#[cfg(test)]
mod tests {
    use std::env;
    use neuroml_tools::{load_neuroml, NeuroMLDocument};

    #[test]
    fn main() {
        println!("Trying to load file {:?}", env::current_dir().unwrap());
        let mut neuroml = NeuroMLDocument::new();
        load_neuroml("./tests/NML2_FullCell.nml", &mut neuroml).expect("failed to load");
        assert_eq!(neuroml.id, "NML2_FullCell");
        println!("ending")
    }
}