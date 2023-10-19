fn main(){
    let t:f64 = 450_000.00;
    let m:f64 = 1_500_000.00;
    let h:f64 = 750_000.00;
    let d:f64 = 2_850_000.00;
    let a:f64 = 250_000.00;

    // now we find the sum of the sales 

    let s = t + m + h + d + a;
    println!("The sum of the sales is {}", s);

    //now we find the average

    let a = s / 5.0;
    println!("the avearage of the sales is {}", a);


}