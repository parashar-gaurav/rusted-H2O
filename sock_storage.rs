
/*structure for socket address storage
Structure sockaddr_storage: It is
 a structure that stores socket 
 address information, which is 
 sufficient for storing IPv4 as 
 well as IPv6 address information.
  It is key in promoting protocol
   family and protocol version
    independence.
*/
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}


/*c code
typedef struct sockaddr_storage {
  short   ss_family;
  char    __ss_pad1[_SS_PAD1SIZE];
  __int64 __ss_align;

};

*/