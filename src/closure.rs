// Understanding Closures. 

/*
- Rust closures are anonymous function. 
- You can either save this inside a variable. 
- Or pass them as a parameter. 
- Function can not capture the value, closures can ( from the scope they are defined ) 


 ** Lets CAPTURE VALUE with an example from rust book. 

 We have a company and people on the mailing list. On a tshirt giveaway campaign people on the mailing list 
 get the Tshirts, and they can optionally set fav colour on their profile. 
 We will deleiver fav color tshirt to someone who have mentioned the color and 
 any one who has not made choice we will give the max stocked color. 


Data structures we are gonna use - 

- Enum for type of shirt color 
- Struct Inventory , that store shirt color in stock.
- Method impl on Inventory - giveaway | get the tshirt color a person is going to get. 
 */

// traits we are gonna need to display and copy the contents of struct. 
#[derive(Debug, PartialEq, Clone, Copy)]

//defined enum
pub enum ShirtColor {
    Red, 
    Black
}

// struct 
pub struct Inventory{
   pub shirts : Vec<ShirtColor>
}

impl Inventory {
    // works simply like the value passed in the parameter will be returned if its available.
    // Color could be some or none. 
    pub fn giveaway(&self, user_preference : Option<ShirtColor>)-> ShirtColor {

        user_preference.unwrap_or_else(|| self.most_stocked())// closure , a anonymous function that has captured a value , here Shirt.
    }
    pub fn most_stocked(&self) -> ShirtColor { 

        // calculating the color count on tshirt. 
        // making them mutable by mut keyword, so that we can update its value. in rust values of immutable by default.
        let mut red_tshirt = 0; 
        let mut black_tshirt = 0; 

        // iterating on shirts which contain vector of shirt color
        for shirt_color in &self.shirts{
            match shirt_color {
                ShirtColor::Red => red_tshirt += 1,
                ShirtColor::Black =>black_tshirt += 1,
            }
        }

        if red_tshirt > black_tshirt { 
            ShirtColor::Red
        }else {
            ShirtColor::Black
        }
    }


}

/*
use this main function in main.rs
fn main() {
    // defined a store with values of different type of shit color. 

    let store = Inventory{
        shirts : vec![ShirtColor::Red, ShirtColor::Black, ShirtColor::Black],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference {:?} get : {:?}",user_pref1, giveaway1);
    
    let user_pref2 = Some(ShirtColor::Red); 
    let giveaway2 = store.giveaway(user_pref2);
    println!("User with preference {:?} get : {:?}",user_pref2, giveaway2);
    
    let user_pref3 = None; 
    let giveaway3 = store.giveaway(user_pref3);
    println!("User with preference {:?} get : {:?}",user_pref3, giveaway3);
    
}
*/