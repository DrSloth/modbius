Modbius aims to support all common modbus codecs. TCP, RTU and even the legacy ASCII codec.
Codecs provide two main ways to de- and encode data. One is a direct way going through the codec
and one reversed trait based approach that passes the codec to the data object which then implements 
how it has to be encoded on codec implementors (somewhat like a reversed std::io::Write or like serde::Deserialize)
