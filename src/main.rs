use std::error::Error;

mod fninfo;

fn p(p:& str) -> Result<(), Box<dyn Error>> {
    let m = fninfo::from(p)?;
    println!("{} -> {}", p, m.to_file_name());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let m = meta::Meta::from_exif("tests/BCDF1203-FD49-4805-B2AE-8E93B67D9076.JPG")?;
    // println!("{}", m.to_name());
    // let m = meta::Meta::from("tests/IMG_0256.HEIC")?;
    // println!("{}, {}", m.to_file_name());
    p("tests/IMG_0256.HEIC")?;
    p("tests/IMG_0257.DNG")?;
    Ok(())
}
