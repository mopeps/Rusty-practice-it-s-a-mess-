//using panic can be expensive so an alternative
//is to abort instead of panicking.
//The way you do this is the following e.g:
/*
(On your cargo.toml file write)
[profile.release]
panic = 'abort'

*/

fn main() {
    //panic!("crash, burn and cry on the corner :)");
    cause_panic_due_toBug();
}
fn cause_panic_due_toBug() {
    let v = vec![1, 2, 3];

    v[99];
}
