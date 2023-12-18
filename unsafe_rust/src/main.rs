use std::slice;
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
}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid < len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}