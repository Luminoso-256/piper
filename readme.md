# Piper

_a learning project/fun experiment in internet protocol_

Version 0.6.0 (SEMVER)

## Goals

- Piper is Simple. A page is a page. There are no secondary requests for more resources.
- Piper is Small. The specification should be minimal, and not painful to implement in a language of choice.
- Piper is Stateless. All requests can exist in a vacuum. The specification does not require any request/response to depend on a prior one.
- Piper is Finite. The specification is not open-ended or extensible. This results in simplicity and ease of implementation.

## Basic Tech Notes

- Piper uses TCP for data transfer, as TCP is reliable and battle-tested.
- Piper's default port is port 60
- Piper is Little Endian. To maintain simplicity, this is spec-defined, and not controllable by the server or the client.

## Piper URLs
the piper URL formart is laid out below. Parts in brackets can be omitted, but note that this omission may change behavior in some cases.
```
[piper://]example.com[:60]/some_file.txt
```

## Piper Requests

A Piper request is simple. Once a TCP connection with the server has been established, the target URI is then sent. The client then waits for a response. Once a response is received, the TCP connection is closed.

The request structure is detailed below:

| Num Bytes | Purpose               |
| --------- | --------------------- |
| 2         | URI Length Specifier  |
| Remaining | URI (encoded in UTF8) |

## Piper Responses

A response entails slightly more complexity than a request, as it contains first a header data structure, and then the contents of the response, if they exist.

the below table lays out the fields of the header:

| Num Bytes | Purpose                |
| --------- | ---------------------- |
| 1         | Content-Type (`u8`)    |
| 8         | Content-Length (`u64`) |
| Remaining | Content                |

This header design meets the criterion of Piper well. Its static nature (and the lack of any sort of header length field) keeps it Finite, whereas its small size helps keep it Simple and Small. There are no fields present that would help one evade Piper's promise to keep things Stateless.

Everything after the header is assumed to be body content. The Content-Length field is present solely to help clients with ease of allocation while reading a response.

The below table lays out the pre-allocated Content-Type field values. Any value not present is not part of the specification and is considered invalid.

### Content Type Table

| Code (Hex) | Purpose                                                   |
| ---------- | --------------------------------------------------------- |
| `0x00`     | Text (UTF8, no formatting)                                |
| `0x01`     | Text (UTF8, gemtext)                                      |
| `0x02`     | Text (ASCII)                                              |
| `0x10`     | File - raw bytes                                          |
| `0x20`     | Redirect to other Piper page. If the `piper://` component of the URL is present, it is an out-of-site redirect. Otherwise the redirect is relative and considered in-site. |
| `0x22`     | Error - resource not found                                |
| `0x23`     | Error - internal server error                             |
| `0x24`     | Specification Version - used to avoid unexpected errors on spec changes |

Note that Content Types are laid out in "ranges":

- `0x0X` is text
- `0x1X` is file transfer
- `0x2X` is Status-code like things. (Piper has no formal concept of status codes, as the required functionality can be reimplemented here.)

Also note that the `0xFX` range is **reserved** for client-side utility purposes.

## Reference Implementations (this and everything below is not part of the spec)
Server:
- [petri](https://github.com/luminoso-256/petri)

Client:
- [pipette](https://github.com/Luminoso-256/pipette)

