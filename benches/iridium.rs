#![feature(test)]

extern crate iridium;
extern crate test;
use iridium::vm::VM;
use test::Bencher;

#[bench]
fn bench_fill_all_registers(b: &mut Bencher) {
    b.iter(|| {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 50;
        test_vm.registers[1] = 50;
        test_vm.registers[2] = 50;
    });
}
