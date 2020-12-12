use std::fs::File;
use std::io::Read;
/*
 * getpassword: a simple password file parser thing
 * Remember to encrypt you password file to keep it safe, and then temporarily
 * unencrypt it for use with this program.
 * Format for the password file is 'name, password, name, password, ' etc.
 * The space is signficant.
 * Best used as part of an alias. For example:
 * 'mcrypt -d ~/main/misc/passwords.nc; ~/main/programs/rustprograms/getpassword
 * ; rm ~/main/misc/passwords; sleep 15; clear;'
 */
fn main() {
  //Set VthisV to the path to your unencrpyted password file
  let password_path = "/home/mehbark/main/misc/passwords";
  let mut file = File::open(password_path).expect("Unable to open");
  let mut contents = String::new();
  let mut input = String::new();
  file.read_to_string(&mut contents);
  contents.pop();

  let passwords: Vec<&str> = contents.split(", ").collect();

  println!("Here are all of the passwords you have stored: ");
  for i in 0..passwords.len() {
    if i % 2 == 0 {
      println!("{}", passwords[i]);
    }
  }

  let lnth = std::io::stdin().read_line(&mut input).unwrap();
  input = input.to_lowercase();

  if input.get(lnth - 1..lnth) == Some("\n") { //Remove the last byte if it's a
    input.pop();                               //newline
  }

  let mut success = 0;
  for i in 0..passwords.len() {
    if passwords[i] == input {
      println!("{}", passwords[i+1]);
      success = 1;
    }
  }

  if success == 0 {
    println!("{1} not found!\nHere's the whole list:\n{0:?}", passwords, input);
  }
}