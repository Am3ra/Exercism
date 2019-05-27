pub fn nth(n: u32) -> u32 {
    let mut primes : Vec<u32> = Vec::new();
    primes.push(2);
    let mut current = 3;
    'hello :while primes.len() -1 < n as usize{
        for i in &primes {
            if current % i == 0{
                current += 1;
                continue 'hello;
            }
        }
        primes.push(current);
    }

    primes[n as usize]
}


