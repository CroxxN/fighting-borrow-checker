#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
       if n>1.0{
           Q7(127)
       } else if n< -1.0{
          Q7(-128)
        } else{
           Q7((n*128.0) as i8)
       }
    
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 7.0 
    }
}

fn main() {
    let number = 0.5;
   println!("{:?}", Q7::from(number)); 
   println!("{:?}", f64::from(Q7(2)));
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn out_of_bounds(){
        assert_eq!(Q7::from(10.0), Q7::from(1.0));
    }
}