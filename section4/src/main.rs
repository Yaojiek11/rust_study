fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");  //push_str()関数は、リテラルをStringに付け加える
        println!("{}", s);
        println!("{}", s);    
    }
}