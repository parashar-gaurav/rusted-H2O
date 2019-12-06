/*adding a listeneris another function that adds a new listener given
 the file descriptor, socket address and address length
  (IPv4 or IPv6), returns the configuration of the listener.
  */
fn add_listener(mut conf: *mut config_t,
                                  mut fd: libc::c_int,
                                  mut addr: *mut sockaddr,
                                  mut addrlen: socklen_t)
 -> *mut listener_config_t {
    let mut listener: *mut listener_config_t =
        h2o_mem_alloc(::std::mem::size_of::<listener_config_t>() as
                          libc::c_ulong) as *mut listener_config_t;
    memcpy(&mut (*listener).addr as *mut sockaddr_storage as
               *mut libc::c_void, addr as *const libc::c_void,
           addrlen as libc::c_ulong);
    (*listener).fd = fd;
    (*listener).addrlen = addrlen;
    (*conf).listeners =
        h2o_mem_realloc((*conf).listeners as *mut libc::c_void,
                        (::std::mem::size_of::<*mut listener_config_t>() as
                             libc::c_ulong).wrapping_mul((*conf).num_listeners.wrapping_add(1i32
                                                                                                as
                                                                                                libc::c_ulong)))
            as *mut *mut listener_config_t;
    let var1 = (*conf).num_listeners;
    (*conf).num_listeners = (*conf).num_listeners.wrapping_add(1);
    let ref mut var2 = *(*conf).listeners.offset(var1 as isize);
    *var2 = listener;
    return listener;
}

/*
static struct listener_config_t *add_listener(struct config_t *conf, int fd, struct sockaddr *addr, socklen_t addrlen)
{
    struct listener_config_t *listener = h2o_mem_alloc(sizeof(*listener));

    memcpy(&listener->addr, addr, addrlen);
    listener->fd = fd;
    listener->addrlen = addrlen;
    memset(&listener->ssl, 0, sizeof(listener->ssl));
    conf->listeners = h2o_mem_realloc(conf->listeners, sizeof(*conf->listeners) * (conf->num_listeners + 1));
    conf->listeners[conf->num_listeners++] = listener;

    return listener;
}
*/