# Archipel Neighbour Discovery protocol

Protocol implemented in *archipel-ipbeacon* is described in
this document. This protocol is inspired by [IPND drafts](https://datatracker.ietf.org/doc/html/draft-irtf-dtnrg-ipnd-03) and [dtn7-rs implementation](https://github.com/dtn7/dtn7-rs/tree/master/core/dtn7/src/ipnd). This protocol is modified to reduce beacon size and improve data structure.

This protocol is experimental and should be treated as work in progress.

## Networking

Software MUST emit beacons in UDP.

Software MUST listen and emit on port defined in [A1-network](./A1-network.md).

Software SHOULD be available in 2 modes : *Broadcast* or *Multicast*.

In *Broadcast* mode, software MUST emit beacons only on local broadcast addresses (see [A1-network](./A1-network.md)).

In *Multicast* mode, software MUST emit beacons only on local multicast addresses defined in [A1-network](./A1-network.md).

In any case, software MUST listen and take into account Beacons from Broadcast and Multicast addresses.

Software MUST emit and listen on ipv4.

Software SHOULD emit and listen on ipv6.

## Emission

Beacon SHOULD be emitted according to their period on the network.
Either Boradcast or Multicast depending on mode (See "Networking" part).

## Beacon format

Beacon MUST be serialized as [CBOR](https://www.rfc-editor.org/rfc/rfc8949) data structure.

Beacon MUST be a CBOR array with the following fields.

Beacon MUST start by version number (see below).

MUST be followed by flag (see below).

MAY include a sequence number after flag (see below).

MAY include node EID after sequence number (see below).

MAY include service block after node EID (see below).

MAY include period after service block (see below).

```
86      # {array(6)}
   08     # Version number {unsigned}
   07     # Flag {unsigned}
   00     # Sequence number {unsigned} (Optional)
   60     # Node EID {text} (Optional)
     ...
   80     # Service block {array} (Optional)
   0A     # Period {unsigned} (Optional)
```

### Version number

Version number MUST be an unsigned number equal to `8`.

Software MUST reject Beacons with different version number.

### Flag

Flag gives hints about Beacon interpretation.

Flag MUST be 8 bit long number at `000000000` by default.

If a node EID is present in beacon, flag MUST binary OR `00000001`.

If a service block is present in beacon, flag MUST binary OR `00000010`.

If a Period is present in beacon, flag MUST binary OR `00000100`.

> **Examples**
>
> If a beacon contains Node EID and period, flag is equal to `00000101`
>
> If a beacon contains service block and period, flag is equal to `00000011`

### Sequence number

Software MUST increment sequence number each time a Beacon is emitted.

### Node EID

Node EID field MUST be a Node identifier available on software's current IP address.

### Period

Software MAY include emission period in bundle.

Persiod MUST be expressed in seconds between twi Beacon emission.

## Service block

An array of services available on node defined in "Node EID" field.

A Service is defined as 2 elements array with the following structure.

```
82         # Service definition {array(2)}
    01      # Service type {unsigned}
    ...     # Service parameters {any data structure} (Optional)
```

Service type represents the kind of service available on node.
This service MAY be completed by parameters under any valid CBOR data structure.

Service types are summarized in [A2-service-tags](./A2-services-tags.md).

### 0 - TCP Convergence layer v4

If node have [TCPCLv4 (RFC9174)](https://www.rfc-editor.org/rfc/rfc9174.html) convergence layer available, software SHOULD include a service typed `0`.

Service MUST have a single unsigned number as parameter.
This parameter is port on which convergence layer is listening.

```
82
    00                          # Service type
    19 ...                      # Listening port {unsigned}
```

### 1 - TCP Convergence layer v3

If node have TCPCLv3 convergence layer available, software SHOULD include a service typed `1`.

Service MUST have a single unsigned number as parameter.
This parameter is port on which convergence layer is listening.

```
82
    01                          # Service type
    19 ...                      # Listening port {unsigned}
```

### 2 - Minimal TCP Convergence layer

If node have [MTCPCL](https://datatracker.ietf.org/doc/html/draft-ietf-dtn-mtcpcl-01) convergence layer available, software SHOULD include a service typed `2`.

Service MUST have a single unsigned number as parameter.
This parameter is port on which convergence layer is listening.

```
82
    02                          # Service type
    19 ...                      # Listening port {unsigned}
```

### 64 - Geolocation coordinates

Software MAY include a service describing gographical location of node.
This service type is `64`.

Parameter MUST be an array of two floating point number for latitude and longitude.

```
82
    18 40                       # Service type
    82                          # array
      ...                       # Latitude {float}
      ...                       # Longitude {float}
```

### 65 - Physical Address

Software MAY include a service describing physical address of node.
This service type id `65`.

Parameter MUST be a text string.

```
82
  18 41                       # Service type
  6C                          # Address {text}
      ...
```

## Example 1

```
86                                      # array(6)
   08                                   # unsigned(8)
   07                                   # unsigned(7)
   00                                   # unsigned(0)
   72                                   # text(18)
      64746E3A2F2F657069636B6977692E66722F # "dtn://epickiwi.fr/"
   85                                   # array(5)
      82                                # array(2)
         01                             # unsigned(1)
         19 1080                        # unsigned(4224)
      82                                # array(2)
         00                             # unsigned(0)
         19 147C                        # unsigned(5244)
      82                                # array(2)
         02                             # unsigned(2)
         19 07C4                        # unsigned(1988)
      82                                # array(2)
         18 40                          # unsigned(64)
         82                             # array(2)
            FA 423707FD                 # primitive(1110902781)
            FA 409A9FBE                 # primitive(1083875262)
      82                                # array(2)
         18 41                          # unsigned(65)
         6C                             # text(12)
            4C796F6E2C204672616E6365    # "Lyon, France"
   0A                                   # unsigned(10)
```

This beacon describes a node with id "dtn://epickiwi.fr/" with :

* a TCPCLv3 convergence layer on port 4224
* a TCPCLv4 convergence layer on port 5244
* a MTCP convergence layer on port 1988
* Located at lat : 45.7578, lon: 4.8320
* With an address "Lyon, France"

It's sequence number is 0 and next beacon may be emitted in 10 seconds.

See [Ex-1.cbor](./Ex-1.cbor) for a binary example.

## Example 2

```
84                                      # array(4)
   08                                   # unsigned(8)
   01                                   # unsigned(1)
   00                                   # unsigned(0)
   78 1B                                # text(27)
      64746E3A2F2F617263686970656C2E657069636B6977692E66722F
                                        # "dtn://archipel.epickiwi.fr/"
```

This beacon describes a node with id "dtn://archipel.epickiwi.fr/".
It's not describing any service or period.
It's sequence number is 0.

See [Ex-2.cbor](./Ex-2.cbor) for a binary example.