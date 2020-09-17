# How to use

```
git clone https://github.com/Sniadekk/axiv.git
cargo run
```

`cargo run` will run the program with default arguments.  
It's possible to tweak the arguments, but we're gonna need to build the binary with `cargo build`.  
After it's built, we are able to use it with our own arguments `./target/debug/axiv [OPTIONS]`



    -h <hotels>        Path to the file where data about hotels is stored. DataSource will look for data to import there
                       [default: hotels.json]
    -i <input>         Path to the input file containing incomplete data [default: input.csv]
    -o <output>        Path to the file where the outcome of the program will be saved. This file will be created if it
                       doesn't exist [default: output.csv]
    -r <rooms>         Path to the file where data about rooms is stored. DataSource will look for data to import there
                       [default: room_names.csv]

I didn't try it out with large input, so this program would need some tweaking in a real life scenario.
