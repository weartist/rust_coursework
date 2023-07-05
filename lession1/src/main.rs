mod first {
    pub mod second {
        pub fn log() {
            println!("this is second layer");
            for s in 'A'..='z' {
                println!("{}", s);
            }
        }
    }

    pub fn log() {
        println!("this is first layer");
        for s in ('Z'..='a').rev() {
            println!("{}", s);
        }
    }
    
}


fn main() {
    first::log();
    first::second::log();
}
