use std::io;

pub fn looping()  {
    let mut counter = 0;

    loop {
         println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lecture");
        counter +=1;

            if input.trim().to_lowercase() == "the letter e" {
                println!("Congrats ! number of trials: {}.", counter);
                break;
            }


    }

    
}



