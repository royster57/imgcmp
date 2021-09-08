use std::cmp::Ordering;
use std::path::Path;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;

/// Uses an Average-Hash algorithm to determine whether the images referenced by the `first` and
/// `second` paths are the same.
/// An error is returned if either image file cannot be opened, or if the image could not be
/// decoded into the format indicated by its file name extension
///
/// # Example
///
/// ```
/// // Comparing different images:
///
/// assert!(!are_same("cat.png", "ferrari.jpg").unwrap())
///
/// // Comparing the similar images in different formats:
///
/// assert!(are_same("cat.png", "cat_resized.jpg").unwrap())
///
/// // If an image name does not contain a file extenision, the function returns an error:
/// assert!(are_same("cat.png", "badimagename").is_err())
pub(crate) fn are_same(
    first: impl AsRef<Path>,
    second: impl AsRef<Path>,
) -> Result<bool, anyhow::Error> {
    const THRESHOLD: u64 = 5;
    match hamming::distance(
        &hash(&ImageReader::open(first)?.decode()?),
        &hash(&ImageReader::open(second)?.decode()?),
    )
    .cmp(&THRESHOLD)
    {
        Ordering::Less | Ordering::Equal => Ok(true),
        _ => Ok(false),
    }
}

/// Computes the Average-Hash of an image, returning the hash as a collection of bits.
fn hash(img: &DynamicImage) -> Vec<u8> {
    const THUMBNAIL_SIZE: u32 = 8;
    let grayscale = img
        .resize_exact(THUMBNAIL_SIZE, THUMBNAIL_SIZE, FilterType::Gaussian)
        .grayscale();
    let average = average(grayscale.as_bytes());
    grayscale
        .as_bytes()
        .iter()
        .map(|byte| if *byte > average { 1 } else { 0 })
        .collect::<Vec<u8>>()
}

/// Computes the average value of a slice of bytes.
fn average(bytes: &[u8]) -> u8 {
    let bytes: Vec<u32> = bytes.iter().map(|x| *x as u32).collect();
    let float_avg = bytes.iter().sum::<u32>() as f32 / bytes.len() as f32;
    float_avg.round() as u8
}
