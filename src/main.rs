//My solution to problem 1 from projecteuler.com
//Q: Find the sum of all the multiples of 3 or 5 below 1000.

//Returns a Vec<i32> where each element is a consecutive number from 1 to 999.
fn vector_build() -> Vec<i32> {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }
    v
}

//Filters out all numbers that are not multiples of 3 or 5 and returns
//the numbers that are multiples of 3 or 5 in a Vec<i32>.
fn vector_filter(v: Vec<i32>) -> Vec<i32> {
    let mut y: Vec<i32> = Vec::new();

    for i in v.iter() {

        if i % 3 == 0 {
            y.push(*i)
        }

        else if i % 5 == 0 {
            y.push(*i)
        }
    }
    y
}

//Returns the sum of all the elements in a Vec<i32>
fn vector_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter() {
        sum += i
    }
    sum
}


fn main() {

    let v = vector_build();

    let y = vector_filter(v);

    let sum = vector_sum(y);
    
    println!("The sum is of all the multiples of 3 or 5 below 1000 is: {}.", sum);
    
}
