use async_std::task;
use hello::run;
fn main() {
    task::block_on(run());
}
