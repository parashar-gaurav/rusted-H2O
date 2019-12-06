
/*structure for socket address
Structure sockaddr: It is a structure 
sockaddr structure defining the endpoint
 for the connect operation. It contains 
 two fields, in which sa_family represents 
 the address family which can be AF_INET,
  AF_UNIX, AF_NS, AF_IMPLINK. 
The sa_data field is a 14 byte long protocol 
specific address. For internet family, it is 
port number along with IP address.
*/
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

/*
 struct sockaddr_in {
               sa_family_t    sin_family; /* address family: AF_INET */
               in_port_t      sin_port;   /* port in network byte order */
               struct in_addr sin_addr;   /* internet address */
           };
*/