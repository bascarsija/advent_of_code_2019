use day03::panel::input;

fn main() {
    let panel = input::panel_with_input_paths();

   match panel.min_combined_path_length() {
       Some(path_len) => println!("min combined path length to intersection: {}", path_len),
       None => println!("no intersection found!")
   }
}