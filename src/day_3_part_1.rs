use crate::file;
use std::io::BufRead;

pub fn solution() -> Result<(), Box<dyn std::error::Error>> {
    let reader = file::to_reader("inputs/day3.txt")?;

    let mut total: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        let mut largest = 0;
        let mut second_largest = 0;
        for (i, c) in chars.iter().enumerate() {
            let c = c.to_digit(10).ok_or("not a digit")?;
            if c > largest && i != chars.len() - 1 {
                largest = c;
                second_largest = 0;
            } else {
                second_largest = second_largest.max(c);
            }
        }
        total += (largest * 10) + second_largest;
    }

    assert_eq!(17332, total);
    println!("Day 3 part 1 passed");
    Ok(())
}

// pub fn solution(sleep: bool) -> Result<(), Box<dyn std::error::Error>> {
//     if sleep {
//         return Ok(());
//     }
//
//     let reader = file::to_reader("inputs/day3.txt")?;
//
//     let total = reader.lines().try_fold(0u64, |total, line| {
//         let line = line?;
//         let bytes = line.as_bytes();
//
//         let mut largest = 0u32;
//         let mut second_largest = 0u32;
//         let mut largest_idx = None;
//
//         for (i, &b) in bytes.iter().enumerate() {
//             let d = (b as char)
//                 .to_digit(10)
//                 .ok_or_else(|| "not a digit")?;
//
//             match largest_idx {
//                 // First digit or new largest (but not last char)
//                 None | Some(_) if d > largest && i + 1 < bytes.len() => {
//                     largest = d;
//                     largest_idx = Some(i);
//                     second_largest = 0; // reset because largest moved
//                 }
//                 // After largest, try updating second_largest
//                 Some(idx) if i > idx => {
//                     second_largest = second_largest.max(d);
//                 }
//                 _ => {}
//             }
//         }
//
//         Ok(total + (largest as u64) * 10 + second_largest as u64)
//     })?;
//
//     println!("total: {total}");
//     Ok(())
// }
//
