fn main() {
  print_lyrics();
}

fn print_lyrics() {
  println!("");
  for num in 1..=12 {
    println!("On the {} day of Christmas, my true love sent to me", ordinal_num(num));
    for i in (1..=num).rev() {
      println!("{}{}", {
        if num != 1 && i == 1 {"And a ".to_string()}
        else if num == 1 && i == 1 {"A ".to_string()}
        else {"".to_string()}
      }, gift(i));
    }
    println!("");
    if num == 12 {println!("And a {}\n", gift(1))}
  }
}

fn ordinal_num(num: u8) -> String {
  match num {
    1 => "first".to_string(),
    2 => "second".to_string(),
    3 => "third".to_string(),
    4 => "fourth".to_string(),
    5 => "fifth".to_string(),
    6 => "sixth".to_string(),
    7 => "seventh".to_string(),
    8 => "eighth".to_string(),
    9 => "ninth".to_string(),
    10 => "tenth".to_string(),
    11 => "eleventh".to_string(),
    12 => "twelfth".to_string(),
    _ => "unknown".to_string(),
  }
}

fn gift(num: u8) -> String {
  match num {
    1 => "Partridge in a Pear Tree.".to_string(),
    2 => "Two Turtle Doves,".to_string(),
    3 => "Three French Hens,".to_string(),
    4 => "Four Calling Birds,".to_string(),
    5 => "Five Gold Rings,".to_string(),
    6 => "Six Geese-a-Laying,".to_string(),
    7 => "Seven Swans-a-Swimming,".to_string(),
    8 => "Eight Maids-a-Milking,".to_string(),
    9 => "Nine Ladies Dancing,".to_string(),
    10 => "Ten Lords-a-Leaping,".to_string(),
    11 => "Eleven Pipers Piping,".to_string(),
    12 => "Twelve Drummers Drumming,".to_string(),
    _ => "unknown".to_string(),
  }
}
