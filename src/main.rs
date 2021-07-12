mod lib;
use crate::lib::*;
use std::io::Write;

fn main() {
    // Read files
    let texts_raw = std::fs::read_to_string("texts.txt").expect("Can't find file 'texts.txt'");
    let mids_raw = std::fs::read_to_string("coords.txt").expect("Can't find file 'coords.txt'");

    // Make args
    let texts: Vec<_> = texts_raw.split("\n\n").collect();
    let mids: Vec<_> = mids_raw
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let ints: Vec<_> = s
                .split(' ')
                .map(|w| w.parse::<u32>().expect("Not a valid i32"))
                .collect();
            (ints[0], ints[1])
        })
        .collect();

    // Generate image
    std::fs::File::create("out.png")
        .expect("Can't create file out.png")
        .write_all(to_png(image_text(texts, (50., 50.), mids.as_slice())).as_slice())
        .expect("Can't write to file.");
}
