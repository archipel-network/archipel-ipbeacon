# ANNEX 2 : Service type tags

The following is service type tags and structure of data.

| TAG    | Definition  | Construction                                |
|--------|-------------|---------------------------------------------|
|     0  | CLA-TCPv4   | {Port (fixed16)}                            |
|     1  | CLA-TCPv3   | {Port (fixed16)}                            |
|     2  | CLA-MTCP    | {Port (fixed16)}                            |
|   3-63 | UNASSIGNED (CLA service) |                                |
|     64 | GEO         | {latitude (float32), longitude (float32)}   |
|     69 | ADDRESS     | {Address} (string)}                         |
| 70-125 | UNASSIGNED  |                                             |
|125-255 | PRIVATE USE |                                             |