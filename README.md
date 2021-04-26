# Increment Sum
A small test project to practice Rust

## What does it do?
It just sums up some numbers from beginning to end by increment in constant time.
e.g. 10 to 30 by increment of 5 (10+15+20+25+30)

## How does it work?
It expands on Gauss' method

## Usage
`cargo run`
- Start defaults to 1
- End defaults to 10
- Increment defaults to 1

`cargo run -- <end>`
- Start defaults to 1
- Increment defaults to 1

`cargo run -- <start> end>`
- Increment defaults to 1

`cargo run -- <start> end> <increment>`

## Example 1 (Default)
### 1 to 10 by 1
Command: `cargo run`
Result:
```
Start: 1
End: 10
Increment: 1
Number of Addends: 10
Multiplier: 5
Sum: 55
```

## Example 2 (Integer arguments)
### 10 to 100 by 5
Command: `cargo run -- 10 100 5`
Result:
```
Start: 10
End: 100
Increment: 5
Number of Addends: 19
Multiplier: 9.5
Sum: 1045
```

## Example 3 (Float arguments)
### 1 to 10 by 0.5
Command: `cargo run -- 1 10 0.5`
Result:
```
Start: 1
End: 10
Increment: 0.5
Number of Addends: 19
Multiplier: 9.5
Sum: 104.5
```