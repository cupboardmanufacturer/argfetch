# argfetch

> **command line argument parser for rust.**  

## about

argfetch returns a string that has all the args separated by spaces  
if the flag doesnt exist, or there is no value it will return an empty string

## usage

run:
```bash
$ cargo add argfetch
```

to use argfetch you have to pass flag you want to get the args from, and the vector of arguments

## examples

in this example, we will be fetching the d flag:

```rust
use argfetch::fetch;

fn main() {
  let args: Vec<String> = env::args().collect(); // get the arguments
  let args_value = fetch(String::from("-d"), &args); // the flag (-a in this example) has to be a string, and the args have to be passed as a reference

  if args_value.is_empty() { // check if its empty, which means the flag is missing, or has no value
    println!("the flag is missing, or is empty");
    std::process::exit(1); // exit
  }

  println!("{}", args_value);
}
```

running this code would output:

```bash
$ argfetch-example -d "hi" -a test -e some args
hi
$
```


instead if you wanted to fetch the a flag, you would do: 

```rust
use argfetch::fetch;

fn main() {
  let args: Vec<String> = env::args().collect();
  let args_value = fetch(String::from("-a"), &args);

  if args_value.is_empty() {
    println!("the flag is missing, or is empty");
    std::process::exit(1);
  }
  
  println!("{}", args_value);
}
```

then the output would be:

```bash
$ argfetch-example -d "hi" -a test -e some args
test
$
```

and for getting the e flag:

```rust
use argfetch::fetch;

fn main() {
  let args: Vec<String> = env::args().collect();
  let a_args_value = fetch(String::from("-e"), &args);

  if a_args_value.is_empty() {
    println!("the flag is missing, or is empty");
    std::process::exit(1);
  }

  println!("{}", args_value);
}
```

then run: 

```bash
$ argfetch-example -d "hi" -a test -e some args
some args
$
```

however, running it with no e flag, will output:

```bash
$ argfetch-example -d "hi" -a test
the flag is missing, or is empty
$
```
