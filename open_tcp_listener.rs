/*Open_tcp_listener is a function that takes many parameters,
 like configurator command, host name, server name, domain , 
 type, protocol, socket address and addles length(IPv4/IPv6 )
 as input and returns file descriptor or -1, depending on , if 
 it was able to successfully open the tcp_listener, or failed,
  respectively.
*/
fn open_tcp_listener(mut cmd:
                                           *mut h2o_configurator_command_t,
                                       mut config_file: *const libc::c_char,
                                       mut config_node: *mut yoml_t,
                                       mut hostname: *const libc::c_char,
                                       mut servname: *const libc::c_char,
                                       mut domain: libc::c_int,
                                       mut type_0: libc::c_int,
                                       mut protocol: libc::c_int,
                                       mut addr: *mut sockaddr,
                                       mut addrlen: socklen_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    fd = socket(domain, type_0, protocol);
    if !(fd == -1i32) {
        /* set reuseaddr */
        let mut flag: libc::c_int = 1i32;
        if !(setsockopt(fd, 1i32, 2i32,
                        &mut flag as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as socklen_t) != 0i32) {
            /* set TCP_DEFER_ACCEPT */
            let mut flag: libc::c_int = 1i32;
            if !(setsockopt(fd, IPPROTO_TCP as libc::c_int, 9i32,
                            &mut flag as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) != 0i32) {
                /* set IPv6only */
                if domain == 10i32 {
                    let mut flag: libc::c_int = 1i32;
                    if setsockopt(fd, IPPROTO_IPV6 as libc::c_int, IPV6_V6ONLY,
                                  &mut flag as *mut libc::c_int as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<libc::c_int>() as
                                      libc::c_ulong as socklen_t) != 0i32 {
                        close(fd)
                    }
                } 
               
    }
    if fd != -1i32 { close(fd); }
    h2o_configurator_errprintf(0 as *mut h2o_configurator_command_t,
                               config_file, config_node,
                               b"failed to listen to port %s:%s: %s\x00" as
                                   *const u8 as *const libc::c_char,
                               if !hostname.is_null() {
                                   hostname
                               } else {
                                   b"ANY\x00" as *const u8 as
                                       *const libc::c_char
                               }, servname, strerror(*__errno_location()));
    return -1i32;
}

/* C code 
static int open_tcp_listener(h2o_configurator_command_t *cmd, const char *config_file, yoml_t *config_node, const char *hostname, const char *servname, int domain, int type, int protocol, struct sockaddr *addr, socklen_t addrlen)
{
    int fd;

    if ((fd = socket(domain, type, protocol)) == -1)
        goto Error;
    { /* set reuseaddr */
        int flag = 1;
        if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &flag, sizeof(flag)) != 0)
            goto Error;
    }
#ifdef TCP_DEFER_ACCEPT
    { /* set TCP_DEFER_ACCEPT */
        int flag = 1;
        if (setsockopt(fd, IPPROTO_TCP, TCP_DEFER_ACCEPT, &flag, sizeof(flag)) != 0)
            goto Error;
    }
#endif
#ifdef IPV6_V6ONLY
    /* set IPv6only */
    if (domain == AF_INET6) {
        int flag = 1;
        if (setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &flag, sizeof(flag)) != 0)
            goto Error;
    }
#endif
    if (bind(fd, addr, addrlen) != 0)
        goto Error;
    if (listen(fd, SOMAXCONN) != 0)
        goto Error;

    return fd;

Error:
    if (fd != -1)
        close(fd);
    h2o_configurator_errprintf(NULL, config_file, config_node, "failed to listen to port %s:%s: %s", hostname != NULL ? hostname : "ANY", servname, strerror(errno));
    return -1;
}
*/