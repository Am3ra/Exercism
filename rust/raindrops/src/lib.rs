pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let mut changed: bool = false;
    if n % 3 == 0 {
        s.push_str("Pling");
        changed = true;
    }
    if n % 5 == 0 {
        s.push_str("Plang");
        changed = true;
    }
    if n % 7 == 0 {
        s.push_str("Plong");
        changed = true;
    }

    if !changed {
        s = n.to_string();
    }

    s
}
