/*on config listen enterOn_config_listen_enter is
 a function that takes h2o_configurator and its 
 context including file and node of type yoml and
  returns an integer. Yoml is a DOM like interface to YAML
*/
 fn on_config_listen_enter(mut _configurator:
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
    (*configurator).num_host_listeners = 0i32 as size_t;
    return 0i32;
}

/*
static int on_config_listen_enter(h2o_configurator_t *_configurator, h2o_configurator_context_t *ctx, const char *file, yoml_t *node)
{
    struct listener_configurator_t *configurator = (void*)_configurator;

    /* bail-out unless at host-level */
    if (ctx->hostconf == NULL || ctx->pathconf != NULL)
        return 0;

    configurator->num_host_listeners = 0;
    return 0;
}

*/