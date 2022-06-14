use crate::atom::Atom;

pub fn generate_2D(atoms: &mut Vec<Atom>, length: f32) {
    let n = (atoms.len() as f32).sqrt();
    let l = length/(n+1.0);

    for (i, atom) in atoms.iter_mut().enumerate() {
        let x_i: i32 = (i as i32) % (n as i32);
        let y_i: i32 = (i as i32) / (n as i32);
        let x: f32 = l * (x_i as f32) + l;
        let y: f32 = l * (y_i as f32) + l;
        atom.x = x as f32;
        atom.y = y as f32;
    }
}

pub fn total_energy(atoms: &mut Vec<Atom>, length: f32, d_min: f32, d_max: f32) -> f32 {
    let mut total_en: f32 = 0.0;
    
    for i in 0..atoms.len() {
        total_en += energy(atoms, length, i, d_min, d_max);
    }
    total_en
}

pub fn energy(atoms: &mut Vec<Atom>, length: f32, index: usize, d_min: f32, d_max: f32) -> f32 {
    let mut en: f32 = 0.0;

    for i in 0..atoms.len() {
        if i == index { continue; }
        else {
            let mut dx = (atoms[index].x - atoms[i].x).abs();
            let mut dy = (atoms[index].y - atoms[i].y).abs();
            if dx > length / 2.0 { dx = length - dx };
            if dy > length / 2.0 { dy = length - dy };
            let d = (f32::powf(dx, 2.0) + f32::powf(dy, 2.0)).sqrt();
            if d < d_min {
                en += 100000.0;
            } else if d < d_max {
                en += -1.0;
            }
        }
    }
    en
}