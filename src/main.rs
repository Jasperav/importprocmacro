use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
use b_query::query;

mod some_other_mod;

fn main() {
    let _ = query!("I work");
}
