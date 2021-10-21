The [modbius](https://github.com/DrSloth/modbius) core parsing library implementing parsing logic meant to be abstracted over.

Parsing is realised here for the modbius specific client/server implementations.
Other parsing implementations can be integrated into the modbius framework by depending on this crate and using its traits.
In the future there will be a feature gate to not depend on the parsing logic.

modbius-core is a no_std and no_alloc crate. It aims to be completely allocation free and usable for embedded/no_std crates.

It is very important that this library is highly optimised for space and speed such that parsing will not be any kind of bottleneck.

Still a WIP and not usable. However the name is reserved by this version 0.0.1.
