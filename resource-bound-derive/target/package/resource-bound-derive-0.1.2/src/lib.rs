
mod outer_attr;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{DeriveInput, Fields, spanned::Spanned};

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

    //lifetimes
    let lifetimes=input.generics.lifetimes();
    let liftimes_use=input.generics.lifetimes();
    
    // 2️⃣ Parse outer attributes
    let mut outer_attrs = OuterAttributes::new();
    for attr in &input.attrs {
        outer_attr::parse_struct_attrs(&mut outer_attrs, attr)?;
    }


    //extract data
    let size_limit=match outer_attrs.get_max_size() {
        Some(s)=>s,
        None=>0    
    };
    let heap_permission=outer_attrs.get_heap_allo();//false by default if not explicitly defined
    if !heap_permission{
        map_fields(&fields,&mut outer_attrs)?;
    }
    let mut field_ctr=0;
    let field_types=outer_attrs.get_types();
    let field_assert=field_types.iter().map(move |ty|{
        field_ctr+=1;
        let ctr_name=String::from("FIELD")+&field_ctr.to_string()+&name.to_string()[..];
        let ctr_name=syn::Ident::new(&ctr_name,ty.span());
        let assert_fn_name=String::from("assert_fn")+&field_ctr.to_string()[..]+&name.to_string()[..];
        let assert_fn_name=syn::Ident::new(&assert_fn_name[..], ty.span());
        quote! {
            
                const #ctr_name : () ={
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
    let expanded_limit=quote! {
    
        #heap_assert_code
        impl<#(#lifetimes)*> #name <#(#liftimes_use)*> {
            #[doc(hidden)]
            const __RESOURCE_BOUND_ASSERTION_FAILED_BECAUSE_THE_STRUCT_SIZE_IS_MORE_THAN_THE_EXPLICITLY_DEFINED_VALUE: [(); 0] =
                [(); (core::mem::size_of::<Self>() <= #size_limit) as usize - 1];
        }

    };
    // 3️⃣ Generate code
    Ok(expanded_limit
    .into())
}


fn map_fields(fields:&Fields,mutable:&mut OuterAttributes)->syn::Result<()>{
    
    for field in fields{
        let ty=&field.ty;
        mutable.push_type(ty.clone());
    }


    Ok(())
}