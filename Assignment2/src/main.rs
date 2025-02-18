fn is_even(n: i32) -> bool{
    if n%2 == 0 {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let arr = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55];

    for num in arr.iter() {

        let mut byt = false;
        let mut byf = false;

        if num%3 == 0{
            byt = true;
        }

        if num%5 == 0{
            byf = true;
        }

        match (is_even(*num), byt, byf){
        (false, false, false) => println!("{} is Odd", num),
        (false, false, true) => println!("Buzz is Odd"),
        (false, true, false) => println!("Fizz is Odd"),
        (false, true, true) => println!("FizzBuzz is Odd"),
        (true, false, false) => println!("{} is Even", num),
        (true, false, true) => println!("Buzz is Even"),
        (true, true, false) => println!("Fizz is Even"),
        (true, true, true) => println!("FizzBuzz is Even"),
        }
    }

    let mut sum = 0;
    let mut c = 10;
    while c != 0 {
        sum = sum + arr[c-1];
        c -= 1;
    }
    println!("Sum of Array is {}", sum);

    let mut count = 1;
    let mut top = arr[0];
    loop {
        if count > 9 {
            break;
        }

        if arr[count-1] < arr[count]{
            top = arr[count];
        }
        count += 1;
    }

    println!("The largest number is {}", top);
}