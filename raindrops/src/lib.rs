pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");
    let divisors = [3, 5, 7];
    let strs = ["Pling", "Plang", "Plong"];

    for (i, v) in divisors.iter().enumerate() {
       if n % v == 0 {
           result.push_str(strs[i]);
       }
    }

    if result.is_empty() {
        return n.to_string();
    }

    result
}

