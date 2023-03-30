# sway-config-resolver

This is the config resolver which uses a very small subset of the directives used by sway.
It also contains support for the additional directive proposed in the gsoc take-home-tasks proposed by ccextractor

## How to run

```cargo run <config file 1> <config file 2> ...```

If you are on Unix systems the OS will expand glob patterns automatically so you can also use them as arguments.

alternatively you could also run 

```cargo test``` to run the inbuilt tests. They should work on unix and as long as you navigate to the project directory before running,
As these tests depend on the PWD environment variable to locate the config files.

