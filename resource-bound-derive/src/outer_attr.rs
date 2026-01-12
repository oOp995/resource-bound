
use syn::{Attribute,Meta, spanned::Spanned};



pub struct OuterAttributes{
    max_size:Option<usize>,
    allow_heap:bool,
    field_types:Vec<syn::Type>,
}

impl OuterAttributes {
    pub fn new()->Self{
        OuterAttributes { max_size:None, allow_heap: false,field_types:Vec::new() }
    }

    pub fn set_max_size(&mut self,value:usize){
        self.max_size=Some(value);
    }
    pub fn set_heap_allo(&mut self,value:bool){
        self.allow_heap=value;
    }
    pub fn get_max_size(&self)->Option<usize>{
        self.max_size
    }
    pub fn get_heap_allo(&self)->bool{
        self.allow_heap
    }
    pub fn push_type(&mut self,value:syn::Type){
        self.field_types.push(value);
    }
    pub fn get_types(&self)->&Vec<syn::Type>{
        &self.field_types
    }
}

pub fn parse_struct_attrs(struct_attr:&mut OuterAttributes,attr:&Attribute)->syn::Result<()>{
    let path=attr.path();
    if path.is_ident("size_limit"){
        
        match &attr.meta{
            Meta::NameValue(nv)=>{
                if let syn::Expr::Lit(c)=&nv.value{
                    if let syn::Lit::Int(i)=&c.lit{
                       let size_limit=match i.base10_digits().parse::<usize>(){
                        Ok(n)=>n,
                        Err(_e)=>{return Err(syn::Error::new(i.span(),"parse int error"));}
                       };
                       struct_attr.set_max_size(size_limit);
                    }else{
                        let err=syn::Error::new(c.span(),"#[size_limit] only accepts integer literals
");
                        return Err(err);
                    }   
                }else{
                    let err=syn::Error::new(attr.span(),"invalid #[size_limit] attribute\nexpected: #[size_limit = <positive integer in bytes>]\nexample:  #[size_limit = 1024]");
                    return Err(err);    
                }
            },
            _=>{
                let err=syn::Error::new(attr.span(),"invalid #[size_limit] attribute\nexpected: #[size_limit = <positive integer in bytes>]\nexample:  #[size_limit = 1024]");
                return Err(err);
            }
        }
    }else if path.is_ident("allow_heap"){
        match &attr.meta {
            syn::Meta::NameValue(nv)=>{
                if let syn::Expr::Lit(lit)=&nv.value{
                    if let syn::Lit::Bool(permission)=&lit.lit{
                        struct_attr.set_heap_allo(permission.value());
                    }else{
                        let err=syn::Error::new(attr.span(), "invalid #[allow_heap] attribute\nexpected: #[allow_heap = <bool>]\nexample:  #[allow_heap = true]");
                        return Err(err);
                    }
                }else{
                    let err=syn::Error::new(attr.span(), "invalid #[allow_heap] attribute\nexpected: #[allow_heap = <bool>]\nexample:  #[allow_heap = true]");
                    return Err(err);
                }
            },
            _=>{
                let err=syn::Error::new(attr.span(), "invalid #[allow_heap] attribute\nexpected: #[allow_heap = <bool>]\nexample:  #[allow_heap = true]");
                return Err(err);
            }
        }
    }
    Ok(())
}
