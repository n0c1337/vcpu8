use vcpu8::{CPU, MOV, R1, R2, ADD, RD};

fn main() {
    let mut cpu = CPU::new();

    let program: Vec<(u8, u8, u8, u8)> = vec![
        (MOV, R1, 42, 0),
        (MOV, R2, 42, 0), 
        (ADD, RD, R1, R2)
    ];
    
    let output = cpu.execute(program);
    println!("{}", output)
}
