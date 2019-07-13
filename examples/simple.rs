extern crate nom_midi as midi;

fn main() {
    let midi = include_bytes!("./test.mid");
    println!("{:?}", &midi[..]);
    println!("{:#?}", midi::parser::parse_smf(midi));
}
