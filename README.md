# PWDGEN
## Why?
To generate passwords and easily keep everything in mind instead of using any storage (199% safer)

## How?
Just make an alias:
```
alias pwdgen="$DIR/pwdgen/target/release/pwdgen"
```
There 2 options to use it:
1. Step by step - just call `pwdgen`
2. By one line - `pwdgen gmail 1234`

## How (it works)?
Simple:
It uses name of the service + your secret code (aka master password) to generate md5 hash which then translated in a copy-pastable password

## Build from source
`cargo build --release`
`cargo run -- target secret prefix?`
`cargo run` to run in interactive mode
