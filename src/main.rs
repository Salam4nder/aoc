// #![deny(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
// #![warn(clippy::cargo)]

use std::error::Error;
pub mod csv;
pub mod day_1_part_1;
pub mod day_1_part_2;
pub mod day_2_part_1;
pub mod day_2_part_2;
pub mod day_3_part_1;
pub mod day_3_part_2;
pub mod day_4_part_1;
pub mod day_4_part_2;
pub mod day_5_part_1;
pub mod day_5_part_2;
pub mod day_6_part_1;
pub mod day_6_part_2;
pub mod file;

fn main() -> Result<(), Box<dyn Error>> {
    day_1_part_1::solution()?;
    day_1_part_2::solution()?;
    day_2_part_1::solution()?;
    day_2_part_2::solution()?;
    day_3_part_1::solution()?;
    day_3_part_2::solution()?;
    day_4_part_1::solution()?;
    day_4_part_2::solution()?;
    day_5_part_1::solution()?;
    day_5_part_2::solution()?;
    day_6_part_1::solution()?;
    day_6_part_2::solution()?;
    Ok(())
}
