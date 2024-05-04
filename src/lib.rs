/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_no_whitespace = code.replace(" ", "");
    dbg!("Invalid code detected:", &code_no_whitespace);

    if code_no_whitespace.chars().any(|x| !x.is_ascii_digit()) || 
        code_no_whitespace.len() <= 1  || 
        code_no_whitespace.chars().all(|x| x == '0') {
        return false;
    }
        
    let code_num:Vec<u32> = code_no_whitespace.chars()
            .filter_map(|x| x.to_digit(10))
            .collect();
    dbg!("Invalid code detected:", &code_num);
    
    
    let sum: u32 = code_num.iter().rev().enumerate().map(|(i, &x)| {
        if i % 2 != 0 {
            let doubled = x * 2;
            if doubled > 9 {
                doubled - 9
            } else {
                doubled
            }
        } else {
            x
        }
    }).sum();
    dbg!("Invalid code detected:", &sum);

    sum % 10 == 0
}
