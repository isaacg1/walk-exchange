#![feature(isqrt)]
use image::{ImageBuffer, RgbImage};
use rand::prelude::*;

type Color = [u8; 3];
type ColorBase = [u8; 3];

fn color_base_to_color(cb: ColorBase, color_size: u64) -> Color {
    cb.map(|cbc| (cbc as u64 * 255 / (color_size - 1)) as u8)
}

fn color_dist(c1: ColorBase, c2: ColorBase) -> i64 {
    c1.iter()
        .zip(c2)
        .map(|(c1e, c2e)| (*c1e as i64 - c2e as i64).abs().isqrt())
        .sum()
}
type Location = [usize; 2];

#[derive(Clone, Copy, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}
impl Dir {
    fn apply(self, loc: Location, size: usize, dist: usize) -> Location {
        match self {
            Dir::Up => [(loc[0] + size - dist) % size, loc[1]],
            Dir::Down => [(loc[0] + dist) % size, loc[1]],
            Dir::Left => [loc[0], (loc[1] + size - dist) % size],
            Dir::Right => [loc[0], (loc[1] + dist) % size],
        }
    }
    fn non_back(self) -> [Self; 3] {
        match self {
            Dir::Up => [Dir::Up, Dir::Left, Dir::Right],
            Dir::Down => [Dir::Down, Dir::Left, Dir::Right],
            Dir::Left => [Dir::Down, Dir::Left, Dir::Up],
            Dir::Right => [Dir::Down, Dir::Right, Dir::Up],
        }
    }
}

fn make_image(scale: u64, steps: u64, ratio: f64, seed: u64) -> RgbImage {
    let mut rng = StdRng::seed_from_u64(seed);
    let size = scale.pow(3) as usize;
    let color_size = scale.pow(2);
    let mut color_bases: Vec<ColorBase> = (0..scale.pow(6))
        .map(|n| {
            let r_base = n % color_size;
            let g_base = (n / color_size) % color_size;
            let b_base = n / color_size.pow(2);
            [r_base as u8, g_base as u8, b_base as u8]
        })
        .collect();
    color_bases.shuffle(&mut rng);
    let mut grid: Vec<Vec<ColorBase>> = color_bases
        .chunks(size)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut loc: Location = [rng.gen_range(0..size), rng.gen_range(0..size)];
    let mut cur_dir: Dir = Dir::Up;
    let max2 = ((size as f64).log2() * ratio) as usize;
    let dists: Vec<usize> = (0..max2).map(|i| 1 << i).collect();
    for _ in 0..steps {
        // Compare and swap
        let dist = dists[rng.gen_range(0..dists.len())];
        let next_loc = cur_dir.apply(loc, size, dist);
        let nn_loc = cur_dir.apply(next_loc, size, dist);
        let loc_color = grid[loc[0]][loc[1]];
        let next_color = grid[next_loc[0]][next_loc[1]];
        let nn_color = grid[nn_loc[0]][nn_loc[1]];
        let dist1 = color_dist(loc_color, next_color);
        let dist2 = color_dist(next_color, nn_color);
        let newd = color_dist(loc_color, nn_color);
        if newd < dist2 && dist2 >= dist1 {
            grid[loc[0]][loc[1]] = next_color;
            grid[next_loc[0]][next_loc[1]] = loc_color;
        } else if newd < dist1 {
            grid[next_loc[0]][next_loc[1]] = nn_color;
            grid[nn_loc[0]][nn_loc[1]] = next_color;
        }
        // Move
        loc = next_loc;
        // New dir
        cur_dir = cur_dir.non_back()[rng.gen_range(0..3)];
    }
    let mut img: RgbImage = ImageBuffer::new(size as u32, size as u32);
    for (i, row) in grid.into_iter().enumerate() {
        for (j, color_base) in row.into_iter().enumerate() {
            img.put_pixel(
                i as u32,
                j as u32,
                image::Rgb(color_base_to_color(color_base, color_size)),
            );
        }
    }
    img
}

fn main() {
    let scale = 10;
    let seed = 0;
    let ratio = 0.5;
    let steps = 50000 * scale.pow(6);
    let filename = format!("img-{}-{}-{}-{}.png", scale, steps, ratio, seed,);
    println!("{}", filename);
    let img = make_image(scale, steps, ratio, seed);
    img.save(&filename).expect("Saved successfully");
}
