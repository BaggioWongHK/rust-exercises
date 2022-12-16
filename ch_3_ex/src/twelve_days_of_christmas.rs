const ORDINAL_NUMS: [&str; 12] = [
  "first", 
  "second", 
  "third", 
  "fourth", 
  "fifth", 
  "sixth", 
  "seventh", 
  "eight", 
  "ninth", 
  "tenth", 
  "eleventh", 
  "twelfth"
];
const CHRISTMAS_GIFTS: [&str; 12] = [
  "Partridge in a Pear Tree",
  "Turtle Doves",
  "French Hens",
  "Calling Birds",
  "Golden Rings",
  "Geese a Laying",
  "Swans a Swimming",
  "Maids a Milking",
  "Ladies Dancing",
  "Lords a Leaping",
  "Pipers Piping",
  "Drummers Drumming"
];

pub fn lyrics() {
  for day in 0..12 {
      let ordinal_num = ORDINAL_NUMS[day];

      println!("On the {ordinal_num} day of Christmas my true love sent to me:");

      for gift_index in (0..day + 1).rev() {
          let current_gift = CHRISTMAS_GIFTS[gift_index];
          let gift_amount = gift_index + 1;

          if gift_index == 0 {
              let conjunction = if day != 0 { "and " } else { "" };
              print!("{conjunction}");
              println!("a {current_gift}");
              break;
          }

          println!("{gift_amount} {current_gift}");
      }

      println!();
  }
}