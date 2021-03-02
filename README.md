# winvec-rs
Windowed Vector (TTL) Collection for Rust

## Usage 

```rust
fn main() {
  let mut winvec = WinVec::with_duration(Duration::from_secs(5));
  
  winvec.push("Hello!");
  winvec.push("World!");
  
  winvec.iter().for_each(|e| println!("{}", e));
}
