# v 0.1.3 changelog
* Fixed github repository link.
* Fixed multiple lifetime syntax. 
    
    ```rust
    use resource_bound::ResourceBound;
    use resource_bound::StackOnly;

    #[derive(ResourceBound)]
    #[size_limit=28]
    #[allow_heap=true]
    struct RefStruct<'a,'b,'c>{
        a:&'a i32,
        b:&'b i32,
        c:&'c i32,
    }

    ```