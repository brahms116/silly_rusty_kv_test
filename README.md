# Testing and benchingish tool for the key-value store

This is a test program for my toy key value store [here](https://github.com/brahms116/silly_rusty_kv)


## Tests and benches

It performs primitive tests and benches based on time taken to perform operations.
The following is an example output of the program at the [time](https://github.com/brahms116/silly_rusty_kv/commit/db64d738893b9d93b8e9e5dbf822f1603789ac03) of writing. 
The timed operations were executed with 10_000 records in scale.

```
Testing stdin...
Writing...
Writing took 455.099417ms
Reading...
Reading took 220.559042ms
Updating...
Updating took 449.805334ms
Reading updated...
Reading updated took 241.354458ms
Deleting...
Deleting took 433.986666ms
Reading after delete...
Reading after delete took 232.197292ms
Testing server basic
Writing took 447.367917ms
Reading...
Reading took 288.516458ms
Testing server transaction...
```

## TODO

- [ ] Profile memory usage
- [ ] Test for speed difference in using concurrent reads when implemented in the key value store
