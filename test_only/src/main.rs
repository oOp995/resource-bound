use resource_bound::ResourceBound;
use resource_bound::StackOnly;


fn main() {
    println!("{}",std::mem::size_of::<Test>());

    
}

#[derive(ResourceBound)]
#[size_limit=524]
#[allow_heap=true]
struct TacomaHygin<'a>{
    field1:i32,
    field2:String,
    feld3:&'a str,
    //fiedl4:&'b i32 
}


struct Test<'a,'b>{
    a:&'a i32,
    b:&'b i32,
}


