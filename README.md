# Modbius

A Modbus framework designed to be performant and largely allocation free.

Modbus is a prominent IOT and fieldbus protocol used in many regions like home automation, charging infrastructure, telecontrol and many many more.One main goal of Modbius is to be completely standard compliant but flexible enough to react to non strictly compliant devices on the same bus.Providing the common transportation types TCP, RTU and the lesser prominent ASCII is another goal Modbius tries to achieve.

Optimisation for space as well as speed is important for real time or embedded applications.

The higher level modbius libs try to be async as much as possible, providing sync abstractions is a secondary goal. 

Modbius is split into multiple crates:
- `modbius-core`: The core parsing libs other libs will mainly depend on. It works as a no alloc and no std crate and is mainly meant to be abstracted over.
- `modbius-traits`: Sync (no high goal) and Async Traits that Modbus Clients/Servers may depend upon for integration with other Modbius related projects
- `modbius-types`: Modbus typing crate used to parse, convert and store various data often stored in Modbus applications.
- `modbius-client`: Modbus client implementations based on `modbius-core` implementing traits from `modbius-traits`
- `modbius-server`: A Modbus server implementation based on `modbius-core` implementing traits from `modbius-traits`
- `modbius`: A reexport crate for all other crates 


## Naming
The name of this framework is a wordplay on the scientist Möbius (also spelled Mobius) and Modbus

## Contributing
Contributions are welcome!

Feel free to open an issue to ask questions, suggest improvements (on API or code), point out errors, report bugs or request features. You may also open pull requests for inclusion into any modbius crate, however i suggest opening an issue first.

### License 
Modbius is licensed under the [MIT License](https://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Modbius by you,
should also be licensed under the same terms and conditions.
