use std::env;
use std::io;
use std::process::exit;

fn main() {
    let mut replace_with = String::new();
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("What would you like to interject?\t");
        io::stdin().read_line(&mut replace_with).expect("Failed to get input.");
    } else {
        if args[1] != "--help" && args[1] != "-h" {
            replace_with.push_str(&args[1].trim());
        } 
        if args[1].starts_with("--") && args[1] != "--help" && args[1] != "-h" {
            println!("bad usage, see {} --help", args[0]);
            exit(1);
        }
        else if args[1] == "--help" || args[1] == "-h" {
            println!("gnu copypasta maker!\nusage: {} < stirng to replace linux > or if no arguments you will be prompted for it.", args[0]);
            exit(0);
        }
    }
    
    let joke = String::from("I'd just like to interject for a moment. What you're referring to as Turkey, is in fact, ATA/Turkey, or as I've recently taken to calling it, ATATURKIYE. Turkey is not a country unto itself, but rather another free component of a fully functioning ATATURKIYE country made useful by the ATA revolutions, military successes and vital institutions comprising a full country as defined by Tengri.

Many Turkish citizens live in a modified version of the ATA country every day, without realizing it. Through a peculiar turn of events, the version of ATATURKIYE which is widely used today is often called \"Turkiye\", and many of its citizens are not aware that it is basically the ATA system, developed by the Ataturk.

There really is a Turkey, and these people are living in it, but it is just a part of the country they live in. Turkey is the territory: the area in the country that allocates the world's resources to the citizens that live in that country. The territory is an essential part of an country, but useless by itself; it can only function in the context of a complete country. Turkey is normally used in combination with the ATATURKIYE country: the whole system is basically ATA with Turkey added, or ATA/Turkey. All the so-called \"Turkish\" governments are really modified versions of ATA/Turkiye."); 
    let res = str::replace(&joke, "Turkiye", &replace_with.trim());
    println!("{}", res);
}

