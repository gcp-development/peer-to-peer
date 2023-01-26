
# p2p application 

This Rust application is base in the [ping example](https://github.com/libp2p/rust-libp2p/blob/master/examples/ping.rs) for the libp2p.It was developed using the [Intellij Community](https://www.jetbrains.com/idea/download/#section=linux) with the [Rust plugin](https://www.jetbrains.com/rust/).

@startuml
skinparam backgroundColor white

entity Client
entity Server

== /ipfs/ping/1.0.0 ==
loop until Client closes write
    Client -> Server: 32 random bytes
    Client <- Server: Same 32 random bytes
end
@enduml


To compile in release mode.

```bash
cargo build --release
```

To run the application with the argument "a".

```bash
cargo run a
```

To run the application with the argument "b".


```bash
cargo run b
```

Remove al artifacts from the target directory generated in the past.

```bash
cargo clean
```

Where the pods IPs are defined.

![image](https://user-images.githubusercontent.com/76512851/214858437-b54f1b3f-ed59-48cb-b593-594285527c59.png)

Note: to know the pods ips please go [here](https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#how-do-we-know-the-pods-ips)

<hr>

References

[Rust libp2p](https://github.com/libp2p/rust-libp2p)<br>
