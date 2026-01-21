
mod outer_attr;

use std::collections::HashMap;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{DeriveInput, Fields, Type,spanned::Spanned};

use outer_attr::OuterAttributes;


/// Enforces compile-time resource constraints on structs.
///
/// See the crate README for details and examples.

#[proc_macro_derive(ResourceBound, attributes(size_limit, allow_heap))]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    match expand_resource(&input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_resource(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let fields;
    // 1️⃣ Validate target
    match &input.data {
        syn::Data::Struct(ds) => {fields=&ds.fields}
        syn::Data::Enum(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "ResourceBound cannot be derived for enums",
            ));
        }
        syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "ResourceBound cannot be derived for unions",
            ));
        }
    }

  

   
    // 2️⃣ Parse outer attributes
    let mut outer_attrs = OuterAttributes::new();
    for attr in &input.attrs {
        outer_attr::parse_struct_attrs(&mut outer_attrs, attr)?;
    }


    //extract data
    let mut lifetime_attr=LifetimeAttr{  lt:HashMap::new()   };
    let size_limit=match outer_attrs.get_max_size() {
        Some(s)=>s,
        None=>0    
    };
    let heap_permission=outer_attrs.get_heap_allo();//false by default if not explicitly defined
    if !heap_permission{
        map_fields(&fields,&mut outer_attrs,&mut lifetime_attr)?;
    }
    let mut field_ctr=0;
    let field_types=outer_attrs.get_types();
    let field_assert=field_types.iter().map(move |ty|{
        field_ctr+=1;
        let ctr_name=String::from("FIELD")+&field_ctr.to_string().to_ascii_uppercase()+&name.to_string().to_ascii_uppercase()[..];
        let ctr_name=syn::Ident::new(&ctr_name,ty.span());
        let assert_fn_name=String::from("assert_fn")+&field_ctr.to_string()[..]+&name.to_string().to_ascii_lowercase()[..];
        let assert_fn_name=syn::Ident::new(&assert_fn_name[..], ty.span());
        quote! {
            
                const #ctr_name: () ={
                    const fn #assert_fn_name<T:StackOnly>(){};
                    #assert_fn_name::<#ty>();
                    ()
                };
        }
    });

    let heap_assert_code=if !heap_permission {
        quote! {
           #(#field_assert)* 
        }
    }else{
        quote! {
            
        }
    };
    
    
    

    //Lifetimes
      //lifetimes
    let lifetimes=input.generics.lifetimes();
    let mut lifetimes_names=vec![];
    lifetimes.for_each(|lt|{
        lifetimes_names.push(lt.lifetime.clone());
    });

    let lifetimes_code=quote! {
        <#(#lifetimes_names),*>
    };

    //let c=lifetimes_use.to_string();                //dev only
    //let e=syn::Error::new(c.span(), c);          //dev only
    //return Err(e);                               //dev only
    
    //rules check

    //rule1: &T must be annotated with allow_heap=true
    if lifetime_attr.lt.len() > 0 && !heap_permission{
        let (unsolved_type,unsolved_lt)=lifetime_attr.lt.iter().next().unwrap();//safe unwrap due to prev condition
        let unsolved_lt=unsolved_lt.to_string();
        let error_msg=format!("if you are borrowing '&T'
{} then allocating mechanism is unknown even if `T` implements `StackOnly`
 , so struct {} must be annotated with  `allow_heap=true`.
 
 consider feature :
 #[allow_heap=true]",unsolved_lt,name);
        let e=syn::Error::new(unsolved_type.span(),error_msg);          //dev only
        return Err(e);                               //dev only
        
    }     

    // Expansion - code

    let expanded_limit=quote! {
        
        #heap_assert_code
        impl #lifetimes_code #name #lifetimes_code {
            #[doc(hidden)]
            const __RESOURCE_BOUND_ASSERTION_FAILED_BECAUSE_THE_STRUCT_SIZE_IS_MORE_THAN_THE_EXPLICITLY_DEFINED_VALUE: [(); 0] =
                [(); (core::mem::size_of::<Self>() <= #size_limit) as usize - 1];
        }

    };
    
    // 3️⃣ Generate code
    Ok(expanded_limit
    .into())
}


fn map_fields(fields:&Fields,mutable:&mut OuterAttributes,field_attr:&mut LifetimeAttr)->syn::Result<()>{
    
    for field in fields{
        let ty=&field.ty;
        mutable.push_type(ty.clone());
        match ty{
            Type::Reference(rf)=>{
                match &rf.lifetime{
                    Some(lt)=>{
                        field_attr.lt.insert(ty.clone(), lt.clone());
                    },
                    None=>{

                    },
                }
                
            },
            _=>{}
        }
    }


    Ok(())
}

struct LifetimeAttr{
    lt:HashMap<syn::Type,syn::Lifetime>,
}