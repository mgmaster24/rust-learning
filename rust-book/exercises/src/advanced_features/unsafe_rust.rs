use core::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

unsafe trait Foo {
    fn update_value(&mut self, new_val: i32);
}

unsafe impl Foo for i32 {
    fn update_value(&mut self, new_val: i32) {
        *self += new_val;
    }
}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: u32,
}

pub fn exercise() {
    println!("\tThe five Unsafe Superpowers\n");
    deref_raw_pointer();

    println!("\t---- 2. Calling unsafe functions/methods ----");
    unsafe_fuctions();

    calling_external_code();
    println!("\t---- End Calling unsafe functions/methods ----\n");

    println!("\t---- 3. Access/Modify mutable static variables ----");
    add_to_count(42);

    unsafe {
        println!("\t\tGLOBAL STATIC COUNTER: {COUNTER}");
    }
    println!("\t---- End Access/Modify mutable static variables ----\n");

    println!("\t---- 4. Implement an unsafe trait ----");
    let mut val = 42;
    val.update_value(55);
    println!("\t\tValue from Unsafe Trait {val}");
    println!("\t---- End Implement an unsafe trait ----\n");

    println!("\t---- 5. Access fields of a union ----");
    let u = MyUnion { f1: 42 };
    let f = unsafe { u.f1 };
    println!("Union value = {f}");
    println!("\t---- End Access fields of a union ----");
}

fn deref_raw_pointer() {
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
        println!("\t\tUpdate pointer value via mutable raw pointer: {val}");
    }

    // Just an example that addressing a random memory address is possible in Rust
    let address = 0x012345usize;
    let adr_ptr = address as *const i32;

    println!("\t---- End Dereferencing a Raw Pointer ----\n");
}

fn unsafe_fuctions() {
    // Have to call unsafe functions in an unsafe block
    unsafe { dangerous() }

    // Creating a safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("\t\tVector split a: {:?}", a);
    assert_eq!(a, &mut [1, 2, 3]);
    println!("\t\tVector split b: {:?}", b);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("\t\tMy split at mut");
    let mut v2 = vec![1, 2, 3, 4, 5, 6];
    let s = &mut v2[..];
    let (x, y) = my_split_at_mut(s, 3);
    println!("\t\tVector split x: {:?}", x);
    assert_eq!(x, &mut [1, 2, 3]);
    println!("\t\tVector split y: {:?}", y);
    assert_eq!(y, &mut [4, 5, 6]);
}

unsafe fn dangerous() {
    println!("\t\tDanger is my middle name!");
}

fn calling_external_code() {
    unsafe {
        println!("\t\tAbsolute value of -3 according to C: {}", abs(-3));
    }
}

// The rust way doesn't work here, because we can't borrow from vals more than once.
// fn my_split_at_mut(vals: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = vals.len();
//     assert!(mid <= len);
//     (&mut vals[..mid], &mut vals[mid..])
// }

// The "unsafe" work around
fn my_split_at_mut(vals: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = vals.len();
    assert!(mid <= len);
    let ptr = vals.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
