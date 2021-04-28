# Increment Sum
A small test project to practice Rust

## What does it do?
It just sums up some numbers from beginning to end by increment in constant time.
e.g. 10 to 30 by increment of 5 (10+15+20+25+30)

## How does it work?
It expands on [Gauss' Summation](https://letstalkscience.ca/educational-resources/backgrounders/gauss-summation): `Sum(1..n) = n(n+1) / 2`.


Formula: `Sum(x..y,z) = x+y * (y-x / z)+1`

Where:
- `x` is the start number
- `y` is the end number
- `z` is the increment
- `z` must be non-zero
- `z` is positive if `y > x` and negative if `y < x`
- the difference between `y` and `x` must be divisible by `z`

## Usage
`cargo run`
- Start defaults to 1
- End defaults to 10
- Increment defaults to 1

`cargo run -- <end>`
- Start defaults to 1
- Increment defaults to 1

`cargo run -- <start> <end>`
- Increment defaults to 1

`cargo run -- <start> <end> <increment>`

## Example 1 (Default)
### 1 to 10 by 1
Command: `cargo run`

Result:
```
Start: 1
End: 10
Increment: 1
Sum: 55
```
in `0.03s`

## Example 2 (Integer arguments)
### 10 to 100 by 5
Command: `cargo run -- 10 100 5`

Result:
```
Start: 10
End: 100
Increment: 5
Sum: 1045
```
in `0.03s`

## Example 3 (Float arguments)
### 1 to 10 by 0.5
Command: `cargo run -- 1 10 0.5`

Result:
```
Start: 1
End: 10
Increment: 0.5
Sum: 104.5
```
in `0.03s`

## Example 4 (Max Int32)
### 1 to 2147483647 by 1
Command: `cargo run -- 1 2147483647`

Result:
```
Start: 1
End: 2147483647
Increment: 1
Sum: 2305843008139952000
```
in `0.03s`