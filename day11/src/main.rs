use util::find_all_paths;

use crate::input::build_input;

mod input;
mod part2;

fn main() {
    let g = build_input("input/day11-real.txt");

    println!(
        "part 1: {}",
        find_all_paths(&g, String::from("you"), String::from("out")).len()
    );

    let dac_fft = part2::count_all_paths(&g, String::from("dac"), String::from("fft"));
    let fft_dac = part2::count_all_paths(&g, String::from("fft"), String::from("dac"));
    let svr_dac = part2::count_all_paths(&g, String::from("svr"), String::from("dac"));
    let svr_fft = part2::count_all_paths(&g, String::from("svr"), String::from("fft"));
    let dac_out = part2::count_all_paths(&g, String::from("dac"), String::from("out"));
    let fft_out = part2::count_all_paths(&g, String::from("fft"), String::from("out"));

    let sum = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;
    println!("part 2: {}", sum)
}
