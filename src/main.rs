mod args;
mod atom;
mod atoms;
mod pdb;
mod metropolis;

use crate::atom::Atom;
use atoms::{
    generate_2D,
    total_energy
};
use pdb::print_pdb;
use metropolis::simulation;

fn main() {
    // let (temp, length) = args::return_args();

    let atom = Atom::new();
    let mut atoms = vec![atom; 16];
    let box_length: f32 = 5.0;
    let temp: f32 = 1.0;

    generate_2D(&mut atoms, box_length);

    let en = total_energy(&mut atoms, box_length, 1.0, 1.0);
    println!("initial energy {}", en);

    // let model = 1;
    // let file_name = "exmpl.pdb";
    // print_pdb(&mut atoms, model, &file_name, &en);

    let outer_cycles: usize = 100;
    let inner_cycles: usize = 10;

// TOSOLVE    
    let mut energies = vec![0.0; 1];

    // std::process::exit(1);

    for j in 1..outer_cycles {
        for i in 1..inner_cycles {
            simulation(&mut atoms, box_length, temp, 1.0, 1.0);
            let en_simulation = total_energy(&mut atoms, box_length, 1.0, 1.0);
            energies.push(en_simulation);
        }
    }

    println!("{:?}", energies);
}