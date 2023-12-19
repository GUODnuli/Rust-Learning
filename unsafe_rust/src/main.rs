use std::slice;

// static variables
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// union
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

// =============================================== //

    let mut num = vec![1, 2, 3, 4, 5, 6];
    {
        let r = &mut num[..];

        let (a, b) = r.split_at_mut(3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    {
        let p = &mut num[..];
        let (c, d) = my_split_at_mut(p, 3);
        assert_eq!(c, &mut [1, 2, 3]);
        assert_eq!(d, &mut [4, 5, 6]);
    }
// =============================================== //

    // really unsafe code!
    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
// =============================================== //

    // use static variables
    println!("name is: {}", HELLO_WORLD);

// =============================================== //

    // change static variables
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

// =============================================== //

    // use union
    let u = MyUnion { f1: 1 };
    let f = unsafe {
        u.f1
    };
    println!("MyUnion.f1: {}", f);
}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid < len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}