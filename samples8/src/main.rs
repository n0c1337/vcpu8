use vcpu8::{cpu::cpu::CPU, alu::alu::Mode};

fn main() {
    let mut cpu = CPU::new(Mode::Emulated);

    /*
    mov r7, 6
    mov r2, 5
    add r7, r2
    */
    let program: Vec<(u8, u8, u8)> = vec![
        (0, 7, 6),
        (0, 2, 5), 
        (3, 7, 2)
    ];
    /*
     mov r7, 42
     mov r6, 56
     inc r7
     dec r6
    */
    /*
    let program: Vec<(u8, u8, u8)> = vec![
        (0, 7, 42), 
        (0, 6, 56),
        (5, 7, 0),
        (6, 6, 0)
    ];
    */
    let output = cpu.execute(program);
    println!("{}", output)
}
