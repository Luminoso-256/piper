# Piper
*a learning project/fun experiment in internet protocol*
Version 0.1.0 (SEMBVER)

## Goals
- Piper is Simple. A page is a page. There are no secondary requests for more resources.
- Piper is Small. The specification should be minimal, and not painful to implement in a language of choice.
- Piper is Stateless. All requests can exist in a vacuum. The specification does not require any request/response to depend on a prior one.
- Piper is Finite. The specification is not open-ended or extensible. This results in simplicity and ease of implementation.

## Basic Tech Notes
- Piper uses TCP for data transfer, as TCP is reliable and battletested.
- Piper's default port is port 60
- Piper is Little Endian. To maintain simplicity, this is spec-defined, and not controllable by the server or the client.

## Piper Requests
A Piper request simple. Once a TCP connection with the server has been established, the target URI path and query are sent. The client then waits for a repsonse. Once a response is recieved, the TCP connection is closed. 

## Piper Responses
A response entails slightly more complexity than a request, as it contains first a header datastructure, and then the contents of the response, if they exist.

the below table lays out the fields of the header:

| Num Bytes | Purpose |
|-------|---------|
| 1 | Content Type (`u8`) |
| 8 | Content Length (`u64`) |

this header design meets the criterion of Piper well. It's static nature (and the lack of any sort of header length field) keeps it Finite, whereas it's small size helps keep it Simple and Small. There are no fields present which would help one evade Piper's promise to keep things Stateless.

Everything after the header is assumed to be body content. The Content Length field is present soley to help clients with ease of allocation while reading a response.

the below table lays out the pre-allocated Content Type field values. Any value not present is not part of the specification, and is considered invalid.

### Content Type Table
| Code (Hex) | Purpose |
|------------|---------|
| `0x00` | Text (UTF8, no formatting) |
| `0x01` | Text (UTF8, gemtext) |
| `0x10` | File - data is raw bytes. Should not be treated as having an encoding. |
| `0x20` | Redirect to other Piper URI. URI is supplied in contents. |
| `0x21` | Redirect to non-Piper URI. URI is supplied in contents |
| `0x22` | Error - resource not found |
| `0x23` | Error - internal server error |

Note that Content Types are laid out in "ranges":
- `0x0X` is text
- `0x1X` is file transfer
- `0x2X` is Status-code like things. (Piper has no formal concept of status codes, as the required functionality can be reimplemented here.)