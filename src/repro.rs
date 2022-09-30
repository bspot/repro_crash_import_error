use rquickjs::{Context, FileResolver, Runtime, ScriptLoader};

fn testit() -> rquickjs::Result<()>{
    let runtime = Runtime::new().unwrap();
    runtime.set_loader(FileResolver::default(), ScriptLoader::default());

    let ctx = Context::full(&runtime).unwrap();

    ctx.with(|ctx| {
        ctx
            .compile(
                "test",
                "import './js/error.js'"
            )
            .map(|_| ())
    })
}

fn main() {
    for i in 1..1000 {
        println!("Attempt {}", i);

        if let Err(e) = testit() {
            println!("{}", e)
        }
    }
}
