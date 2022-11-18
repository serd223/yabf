mod bf;
use bf::*;

mod hello_world;
use hello_world::HELLO_WORLD;

fn main() {
    let program = Program::from(HELLO_WORLD);
    let mut bf: BfInstance<256> = BfInstance::from(program);
    bf.run();
}
