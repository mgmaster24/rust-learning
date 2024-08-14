pub fn exercise() {
    println!("\tThe five Unsafe Superpowers\n");
    println!("\t---- 1. Dereferencing a Raw Pointer ----");
    // immutable and mutable raw pointers
    let mut num = 5;
    // raw pointers can be created in safe code but not dereferenced
    let im_ptr = &num as *const i32;
    let m_ptr = &mut num as *mut i32;

    unsafe {
        let val = *im_ptr;
        println!("\t\tDereferencing immutable raw pointer: {val}");

        *m_ptr = 42;
        let val = *im_ptr;
        println!("\t\tUpdate pointer value via mutable raw pointer: {val}")
    }

    let address = 0x012345usize;
    let adr_ptr = address as *const i32;

    println!("\t---- End Dereferencing a Raw Pointer ----\n");

    println!("\t---- 2. Calling unsafe functions/methods ----");

    println!("\t---- End Calling unsafe functions/methods ----\n");

    println!("\t---- 3. Access/Modify mutable static variables ----");

    println!("\t---- End Access/Modify mutable static variables ----\n");

    println!("\t---- 4. Implement an unsafe trait ----");

    println!("\t---- End Implement an unsafe trait ----\n");

    println!("\t---- 5. Access fields of a union ----");

    println!("\t---- End Access fields of a union ----");
}
