

fn main(){
  // vec! is a macro that make possible create
  // a vector with some values by default
  let ordered_list:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
  let objective:i32 = 8;
  println!("Result {}",binary_search(ordered_list,objective));
}



// How makes binary search in rust
// Binary search works with organizeted list
// you get the list, and split half
fn binary_search(list:Vec<i32>,objective:i32) -> i32{
     // why -1? 
     // convert into i32 if not can do that,throw error.
     let mut high:i32 = (list.len() - 1).try_into().unwrap();
     let mut low:i32 = 0;
     
     while low <= high {
       let average = (low + high ) / 2;
       let kick:i32 = list[average as usize];
       
       if kick == objective {
          return average;
       }
       
       if kick > objective {
         high = average - 1;
       } else {
         low = average + 1;
       }
       
     }
     
     return 0;
}
