use crate::atom::Atom;
use std::fs;

pub fn print_pdb(atoms: &mut Vec<Atom>, model: i32, file_name: &str, en: &f32) -> std::io::Result<()> {
    let mut pdb = String::from("");
    pdb.push_str(format!("MODEL    {}\n", model).as_str());

    for atom in atoms {
        pdb.push_str(format!("{:?}\n", atom).as_str());
    }
    
    fs::write("exmpl.pdb", pdb).expect("Unable to read file");
    
    Ok(())
}