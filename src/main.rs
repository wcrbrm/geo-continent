extern crate geo;

mod africa;
mod antarctica;
mod asia;
mod australia_oceania;
mod central_america;
mod europe;
mod north_america;
mod south_america;

fn main() {
    println!("Africa {:?}", africa::get_polygon());
    println!("Antarctica {:?}", antarctica::get_polygon());
    println!("Asia {:?}", asia::get_polygon());
    println!("Australia/Ocenia {:?}", australia_oceania::get_polygon());
    println!("Central America {:?}", central_america::get_polygon());
    println!("Europe {:?}", europe::get_polygon());
    println!("North America {:?}", north_america::get_polygon());
    println!("South America {:?}", south_america::get_polygon());
}
