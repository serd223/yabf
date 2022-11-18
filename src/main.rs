mod bf;
use bf::*;

mod hello_world;
use hello_world::HELLO_WORLD;

fn main() {
    let mut parser = Parser::new();
    let mut bf: BfInstance<256> = BfInstance::from(parser.parse(HELLO_WORLD));
    bf.run();
}
