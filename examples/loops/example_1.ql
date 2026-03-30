# Calculates factorial of `n`
fn factorial(n) {
    let result = 1;

    for i in 1..=n {
        result *= i;
    }

    return result;
}

# Example usage
println(str_of(factorial(10)));
println(str_of(factorial(15)));
