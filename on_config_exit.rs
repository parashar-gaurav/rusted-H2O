/*on config On_config_listen_exit is another function
 with same parameters as of On_config_listen_enter,
  but returns 0 or -1, depending on the success of 
  the function call execution. If number of host 
  listening on that particular configurator is 0,
 and number of global listener is also 0, then it
 will print “mandatory configuration directive is missing”
  and returns -1
 listen exit*/
fn on_config_listen_exit(mut _configurator:
                                               *mut h2o_configurator_t,
                                           mut ctx:
                                               *mut h2o_configurator_context_t,
                                           mut file: *const libc::c_char,
                                           mut node: *mut yoml_t)
 -> libc::c_int {
    let mut configurator: *mut listener_configurator_t =
        _configurator as *mut libc::c_void as *mut listener_configurator_t;
    /* bail-out unless at host-level */
    if (*ctx).hostconf.is_null() || !(*ctx).pathconf.is_null() { return 0i32 }
    if (*configurator).num_host_listeners == 0i32 as libc::c_ulong &&
           (*configurator).num_global_listeners == 0i32 as libc::c_ulong {
        h2o_configurator_errprintf(0 as *mut h2o_configurator_command_t, file,
                                   node,
                                   b"mandatory configuration directive `listen` is missing\x00"
                                       as *const u8 as *const libc::c_char);
        return -1i32
    }
    return 0i32;
}

/*c code
static int on_config_listen_exit(h2o_configurator_t *_configurator, h2o_configurator_context_t *ctx, const char *file, yoml_t *node)
{
    struct listener_configurator_t *configurator = (void*)_configurator;

    /* bail-out unless at host-level */
    if (ctx->hostconf == NULL || ctx->pathconf != NULL)
        return 0;

    if (configurator->num_host_listeners == 0 && configurator->num_global_listeners == 0) {
        h2o_configurator_errprintf(NULL, file, node, "mandatory configuration directive `listen` is missing");
        return -1;
    }
    return 0;
}

*/