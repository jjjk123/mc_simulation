use crate::atom::Atom;
use crate::atoms::energy;

use rand::Rng;

pub fn simulation(atoms: &mut Vec<Atom>, length: f32, temp: f32, d_min: f32, d_max: f32) -> bool {
    let n: usize = atoms.len();

    let mut rng = rand::thread_rng();
    let i_moved: usize = rng.gen_range(0..n);
    println!("i_moved {}", i_moved);

    let energy_before = energy(atoms, length, i_moved, 1.0, 1.0);
    println!("energy before: {}", energy_before);

    let dx = rng.gen_range(-1.0..1.0);
    let dy = rng.gen_range(-1.0..1.0);
    
    let prev_x = atoms[i_moved].x;
    let prev_y = atoms[i_moved].y;

    let tmp_x = prev_x + dx;
    let tmp_y = prev_y + dy;

    if tmp_x < 0.0 {
        atoms[i_moved].x = length - tmp_x;
    } else {
        atoms[i_moved].x = if tmp_x < length { tmp_x } else { tmp_x - length };
    }

    if tmp_y < 0.0 {
        atoms[i_moved].y = length - tmp_y;
    } else {
        atoms[i_moved].y = if tmp_y < length { tmp_y } else { tmp_y - length };
    }

    let energy_after = energy(atoms, length, i_moved, 1.0, 1.0);

    println!("{:?}, {:?}", prev_x, prev_y);
    println!("{:?}, {:?}", tmp_x, tmp_y);

    println!("energy after: {}", energy_after);

    let d_energy = energy_after - energy_before;
    println!("d_energy: {}", d_energy);

    if d_energy < 0.0 {
        println!("move accepted");
        true
    } else {
        let r = rng.gen_range(0.0..1.0);
        if (-d_energy / 1.0).exp() > r {
            println!("move accepted");
            true
        } else {
            atoms[i_moved].x = prev_x;
            atoms[i_moved].y = prev_y;
            println!("move not accepted");
            false
        }
    }
}