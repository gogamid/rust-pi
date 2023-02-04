
# Running rust
`rustc -C opt-level=2 funcPi.rs`

`time ./funcPI`


```bash
feat: initial pi with for loop 
0.02s user 0.00s system 90% cpu 0.024 total

feat: pi calculation with functiononal way 
0.03s user 0.00s system 94% cpu 0.034 total


feat: pi calculation  into par iter with rayon 
0.05s user 0.01s system 477% cpu 0.011 total
```
