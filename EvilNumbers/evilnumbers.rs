fn main(){for i in 0..50{if format!("{:b}",i).matches("1").count()%2==0{println!("{i}")}}}
