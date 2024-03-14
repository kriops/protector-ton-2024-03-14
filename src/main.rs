#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use std::collections::BTreeSet;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/prime-check/<prime_candidate>")]
fn prime_check(prime_candidate: usize) -> status::Custom<String> {
    if is_prime(prime_candidate) {
        status::Custom(Status::Ok, format!("{} is prime", prime_candidate))
    } else {
        status::Custom(Status::Ok, format!("{} is not prime", prime_candidate))
    }
}

#[get("/factorize/<number>")]
fn factorize(number: usize) -> status::Custom<String> {
    status::Custom(Status::Ok, format!("{:?}", _factorize(number)))
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let max_divisor = (n as f64).sqrt() as usize;
    for i in 2..=max_divisor {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn _factorize(n: usize) -> Vec<usize> {
    let max_prime = (n as f64).sqrt() as usize;
    let mut primes: BTreeSet<usize> = BTreeSet::from([2]);
    let mut not_primes: BTreeSet<usize> = BTreeSet::new();
    for i in (3..=max_prime).step_by(2) {
        if not_primes.contains(&i) {
            continue;
        } else {
            primes.insert(i);
            for j in (2 * i..=max_prime).step_by(i) {
                not_primes.insert(j);
            }
        }
    }

    let mut temp = n;
    let mut result = vec![];
    for i in primes {
        if i > temp {
            break;
        }
        while temp % i == 0 {
            result.push(i);
            temp = temp / i;
        }
    }
    if temp != 1 {
        result.push(temp);
    }
    return result;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, prime_check, factorize])
}
