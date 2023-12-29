fn main() {

    let calibration_document = [
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"
    ];

    let result: u32 = calibration_document
        .map(|s| parse_numbers(s))
        .into_iter()
        .sum();

    println!("{}", result);
}

fn parse_numbers(string: &str) -> u32 {
  return first_digit(string.chars().into_iter()) * 10 +
  first_digit(string.chars().rev().into_iter());
}

fn first_digit<I>(chars: I) -> u32
where
    I: Iterator<Item = char>,
{
    for i in chars {
        match i.to_digit(10)  {
            Some(decimal) => if decimal < 10 {
                return decimal;
            }
            None => ()
        }    
    }
    return 0;
}
