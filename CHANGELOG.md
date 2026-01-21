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
       /*..*/
    }

    ```

# Re-Calrified rule :
 if you are borrowing `&T` then allocating mechanism is unknown even if `T` implements `resource_bound_core::StackOnly`, so it is  must be featured with  `allow_heap=true`