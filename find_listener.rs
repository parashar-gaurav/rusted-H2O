//finding a listener is a function that takes socket
// address and socket length as input and returns
 // listener configuration corresponding to that
  // address and socket length.
fn find_listener(mut conf: *mut config_t,
                                   mut addr: *mut sockaddr,
                                   mut addrlen: socklen_t)
 -> *mut listener_config_t {
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i != (*conf).num_listeners {
        let mut listener: *mut listener_config_t =
            *(*conf).listeners.offset(i as isize);
        if (*listener).addrlen == addrlen &&
               h2o_socket_compare_address(&mut (*listener).addr as
                                              *mut sockaddr_storage as
                                              *mut libc::c_void as
                                              *mut sockaddr, addr) == 0i32 {
            return listener
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut listener_config_t;
}

/*
static struct listener_config_t *find_listener(struct config_t *conf, struct sockaddr *addr, socklen_t addrlen)
{
    size_t i;

    for (i = 0; i != conf->num_listeners; ++i) {
        struct listener_config_t *listener = conf->listeners[i];
        if (listener->addrlen == addrlen
            && h2o_socket_compare_address((void*)&listener->addr, addr) == 0)
            return listener;
    }

    return NULL;
}
*/