# Rust Playground

Hey there this is Chapter-2 Branch , there are multiple branches in this git repo. And each branch represents a new chapter . So make sure to check the branches if you want to learn something.

Here I have built a guessing game 

## Features
- enter any no.
- program will generate a random number
- if the number you entered is lower than generated no. then o/p will be "too low"
- if the number you entered is greater than generated no. then o/p will be "too high"
- if the number you entered is equal to generated no. then o/p will be "you win"

## Steps

### 

```bash
cargo new guessing_game
```

```bash
use std::io;
```

```bash
let mut guess = String::new();
```

```bash
io::stdin()
        .read_line(&mut guess)
        .expect("Please enter a value");
```

This .read_line returns enum which is having <Ok, Err>

so if error is thrown then RUST panics, and to avoid that we use .expect keyword to handle the error

Now we need to randomize the value, for that we need another dep known as 

```bash
rand="0.5.5"
```

so we need to add this in Cargo.toml file under dependency 

```bash
use rand::Rnd;

fn main{
	let secret_num = rand::thread_rng().gen_range(1, 101);
}
```

now to compare the secret_num and guessed_num , we need something called as Ordering

It is basically result of comparing two things

```rust
use std::cmp::Ordering;
```

this is how we are going to compare them :

```rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("too low"),
        Ordering::Equal => println!("You won"),
        Ordering::Greater =>println!("too high"),
    }
```

for comparision we need guess and secret_number of same data type so for that we are doing this 

```rust
let guess:u32 = guess.trim().parse().expect("Enter a number");
```

but this is kinda naive so we will do this

```rust
let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {continue},
        };
```

This is kinda intuitive 

btw we declared guess variable as string earlier and now we declared it as u32 , this is called as shadowing

now the game is almost complete but we need to add one more thing : colors