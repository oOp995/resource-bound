use resource_bound::ResourceBound;
use resource_bound::StackOnly;


fn main() {
    

    
}
/* 
#[derive(ResourceBound)]
#[size_limit=524]
#[allow_heap=true]
struct TacomaHygin<'a>{
    field1:i32,
    field2:String,
    feld3:&'a str,
}


*/
#[derive(ResourceBound)]
#[size_limit=28]
#[allow_heap=true]
struct Ref<'a,'b,'c>{
    a:&'a i32,
    b:&'b i32,
    c:&'c i32,
}

impl<'a,'b,'c> Ref<'_,'_,'_> {
    
}





