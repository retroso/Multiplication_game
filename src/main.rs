use std::io;
use rand::Rng;
fn rng_number(min: i32, max: i32) -> i32{ rand::thread_rng().gen_range(min..=max)
}

    fn main() {
    /*let mut user_input = String::new();
    let test_number = rand::thread_rng().gen_range(1..=100);
    println!("{test_number}");
    let y = test_number + 10;
    println!("{y}");
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed");
    let number_input: i32 = user_input.trim().parse().unwrap();
    println!("{}",number_input);
    let times_two = number_input * 2;
    println!("{number_input} * 2 = {}", times_two)*/
        //----- quick addition game ----

let mut n:i32 = 0;
while n < 5{
        println!("Take a wild guess! will the next addition of numbers add above 100 or not? 1 if it will, 2 if not!");
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("failed");
        let user_number: i32 = user_guess.trim().parse().unwrap();
        match user_number{
            1=>{
                println!("big");
                println!("large");}
            2=>{println!("small");println!("chisai");}
            _=>{println!("out");println!("dumbass");}
            }




    let numbah1 = rng_number(1,100);
    let numbah2 = rng_number(1,100);
    let hyut = numbah1 + numbah2;
    println!("{numbah1}");println!("{numbah2}");
    println!("{numbah1} + {numbah2} = {}", hyut);
    n = n + 1;
     if hyut >= 100{
         println!("the number is larger than 100");}
    else{
         println!("the number is below 100");}

    if hyut >= 100 && user_number == 1{
        println!("You win");
    }
    else if hyut <= 100 && user_number == 2{
        println!("You win");
    }
    else{
        println!("you lost bozo");
    }
    drop(user_number);
    

}
}
