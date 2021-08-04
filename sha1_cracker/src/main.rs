use sha1::Digest;

use std:: {
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;


fn main() -> Result<(), Box<dyn Error>> {
   let args: Vec<String> = env::args().collect();

   if args.len() !=3 {
       //println!("input error");
       println!("Usage:\n      sha1_cracker: <wordlist.txt> <sha1_hash>");

       return Ok(());
   }

   let hash_to_crack = args[2].trim();

   if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
       return  Err("sha1 hash is not valid".into());
   }

   let worldlist_file = File::open(&args[1])?;
   let reader = BufReader::new(&worldlist_file);

   for line in reader.lines() {
       let password = line?.trim().to_string();
       let gen_password = hex::encode(sha1::Sha1::digest(password.as_bytes()));
       //println!("{}", gen_password);
       
       if hash_to_crack == gen_password {
           println!("password found: {}", &password);
           return Ok(());
       }
   }

   println!("password not found in wordlist!");

   Ok(())
}
