# RDTR Binary Encoding Specification v0.1

## Message Header (6 bytes)

0       1       2 3 4 5
+-------+-------+-------+
|ver u8 |type u8|len u32|
+-------+-------+-------+

Big-endian integers.

## Message Types

0x01 INIT_LAYOUT
0x02 INIT_WASM
0x03 PATCH
0x04 EVENT
0x05 RESIZE
0x06 HEARTBEAT
0x07 ERROR

## Primitive Encoding

u8      -> 1 byte
u16     -> 2 bytes
u32     -> 4 bytes
bool    -> 1 byte (0 or 1)
string  -> u32 length + UTF-8 bytes
vec<T>  -> u32 count + T items

## Node Encoding

u32 id
u8 kind
u16 property_count
properties...
u32 child_count
children...

## Mutation Encoding

u8 mutation_type
payload...
