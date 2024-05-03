/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|x| x.is_ascii_alphabetic() || !x.is_ascii_alphanumeric()) || code.replace(" ", "").len() == 1 {
        return false;
    }
        
    let code_num:Vec<u32> = code.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .collect();
    

    let sum: u32 = code_num.iter().rev().enumerate().step_by(2).map(|(i, &x)| {
        match i {
            _ if x * 2 > 9 => x * 2 - 9,
            _ => x * 2,
        }
    }).sum();

    sum % 10 == 0
}
