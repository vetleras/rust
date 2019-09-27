// In the past, the code generated by `format!` produced temporaries in the surrounding scope that
// borrowed the arguments through `&dyn Trait`. These temporaries do not implement `Send`, which
// meant that when `format!` was used in an async block, the resulting generator was not `Send`.
// See https://github.com/rust-lang/rust/issues/64477#issuecomment-534669068 for details
// and https://github.com/rust-lang/rust/issues/64477#issuecomment-531882958 for an example.
async fn foo(_: String) {}

fn bar() -> impl Send {
    async move {
        foo(format!("{}:{}", 1, 2)).await;
    }
}

fn main() {
    let _ = bar();
}
