// This test is adapted from https://github.com/rust-lang/miri/issues/1340#issue-600900312.

fn main() {
    let _val = unsafe { std::mem::MaybeUninit::<i32>::uninit().assume_init() };
    //~^ ERROR constructing invalid value at .value: encountered uninitialized bytes, but expected initialized bytes
}