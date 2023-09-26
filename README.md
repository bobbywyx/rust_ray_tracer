# Ray Tracer In Rust

This is a ray tracer written in Rust. It is based on the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## Dependencies

* Rust
* Nothing else
* Maybe a powerful computer

## Usage

```bash
cargo run --release
```

Then you can find the output image in `output.ppm`, you may need a special software to open it. Fortunately, ubuntu image viewer can directly open this file and there is a vscode extension. If you want to change the output image size, you can change the `WIDTH` and `HEIGHT` constants (or anything else) in `src/main.rs`.  

## Output

![output](./output/out.png)