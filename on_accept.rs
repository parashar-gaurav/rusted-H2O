//On_accept is a function that raises an event when a connection is made to the listener socket.

fn on_accept(mut listener: *mut h2o_socket_t,
                               mut status: libc::c_int) {
    let mut ctx: *mut listener_ctx_t =
        (*listener).data as *mut listener_ctx_t;
    let mut conf: *mut config_t =
        ((*(*ctx).ctx).globalconf as *mut libc::c_char).offset(-0) as
            *mut config_t;
    let mut num_accepts: libc::c_int = 16i32;
    if status == -1i32 { return }
    loop  {
        let mut sock: *mut h2o_socket_t = 0 as *mut h2o_socket_t;
        if (*conf).state.num_connections >= (*conf).max_connections {
            break ;
        }
        sock = h2o_evloop_socket_accept(listener);
        if sock.is_null() { break ; }
        let var1 = &mut (*conf).state.num_connections;
        let var2 = 1i32 as libc::c_uint;
        (::std::intrinsics::atomic_xadd(var1, var2)) + var2;
        (*sock).on_close.cb = on_socketclose ;
        (*sock).on_close.data = (*ctx).ctx as *mut libc::c_void;
        
        num_accepts -= 1;
        if !(num_accepts != 0i32) { break ; }
    };
}


/*static void on_accept(h2o_socket_t *listener, int status)
{
    struct listener_ctx_t *ctx = listener->data;
    struct config_t *conf = H2O_STRUCT_FROM_MEMBER(struct config_t, globalconf, ctx->ctx->globalconf);
    int num_accepts = 16;

    if (status == -1) {
        return;
    }

    do {
        h2o_socket_t *sock;
        if (conf->state.num_connections >= conf->max_connections)
            break;
        if ((sock = h2o_evloop_socket_accept(listener)) == NULL) {
            break;
        }
        __sync_add_and_fetch(&conf->state.num_connections, 1);

        sock->on_close.cb = on_socketclose;
        sock->on_close.data = ctx->ctx;

        if (ctx->ssl_ctx != NULL)
            h2o_accept_ssl(ctx->ctx, sock, ctx->ssl_ctx);
        else
            h2o_http1_accept(ctx->ctx, sock);

    } while (--num_accepts != 0);
}
*/