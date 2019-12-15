use day03::panel::input;

fn main() {
    let panel = input::panel_with_input_paths();

   match panel.min_distance() {
       Some(dist) => println!("min distance to intersection: {}", dist),
       None => println!("no intersection found!")
   }
}