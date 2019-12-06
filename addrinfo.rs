
//address info

//Addinfo is a structure used by getaddrinfo() and contains the following fields :
//ai_family: specifies the desired family for the return addresses which can be for example, IPv4 or IPv6.
// ai_socktype: specifies the preferred socket type for socket address which can be UDP or TCP
// ai_protocol: specifies the protocol for the return socket address.
//ai_flags: specifies additional options, like specifying hints as null is equivalent to setting ai_socktype and ai_protocol to 0. Multiple flags are specified by bitwise OR-ing them together.
// ai_cannonname: it will return the canonical name of the node corresponding to the address info structure value passed back, after successful name lookup.
//ai_addrlen: specifies the size of the socket

pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}


/* Corresponding c code
struct addrinfo {
               int              ai_flags;
               int              ai_family;
               int              ai_socktype;
               int              ai_protocol;
               socklen_t        ai_addrlen;
               struct sockaddr *ai_addr;
               char            *ai_canonname;
               struct addrinfo *ai_next;
           };
*/