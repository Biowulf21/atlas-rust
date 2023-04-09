use atlas_v2::helpers::api::api_helper;

fn main() {
    println!("Hello, world!");
    let number_of_students: u8 = api_helper::get_students();
    print!("{}", number_of_students.to_string());
}
