// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u64) -> u64 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let mut cache = Vec::with_capacity((n+1) as usize) as Vec<u64>;

    for i in 0..n+1 {
        if i == 0 {
            cache.push(0);
        }
        if i == 1 {
            cache.push(1);
        }
        if i > 1{
            // get value from cash
            let stepback_1 = cache.get((i-1) as usize);
            let stepback_2 = cache.get((i-2) as usize);
            if stepback_1.is_some() && stepback_2.is_some(){
                cache.push(stepback_1.unwrap()+stepback_2.unwrap());
            }
        }
    }
    return cache[n as usize];

}

pub fn fib_v2(n:u64) -> u64{
    if n == 0{
        0
    }
    else if n == 1{
        1
    }
    else{
        return fib_v2(n-1) + fib_v2(n-2)
    }
}
#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        println!("aaa{}",fibonacci(30));
        assert_eq!(fibonacci(30), 832040);
    }
}
