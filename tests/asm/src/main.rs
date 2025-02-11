use std::arch::asm;

// Multiply x by 6 using shifts and adds
fn main()
{
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "lsl {tmp}, {tmp}, #1",
            "lsl {x}, {x}, #2",
            "add {x}, {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);

    println!("4 * 6 = {}", x);
}