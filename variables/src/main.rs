fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let c: char = '🐝';
    println!("The value of c is: {c}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("First: {}, Second: {}", first, second);
}
