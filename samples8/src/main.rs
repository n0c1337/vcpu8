use vcpu8::cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    let program: Vec<(u8, u8, u8)> = vec![
        (1, 7, 42),
        (1, 2, 42), 
        (4, 7, 2)
    ];
    
    let output = cpu.execute(program);
    println!("{}", output)
}
