
/*config max connectionOn_config_max_connection
 is a function that takes configurator command 
 as a parameter, along with configuration context,
  configuration file and the yoml node. It then sets 
  the maximum number of connections corresponding to
   the configurator passed as the parameter.
    Configurator has a field, called max_connections
    */
 fn on_config_max_connections(mut cmd:
                                                   *mut h2o_configurator_command_t,
                                               mut ctx:
                                                   *mut h2o_configurator_context_t,
                                               mut config_file:
                                                   *const libc::c_char,
                                               mut config_node: *mut yoml_t)
 -> libc::c_int {
    let mut conf: *mut config_t =
        ((*ctx).globalconf as *mut libc::c_char).offset(-0) as *mut config_t;
    return h2o_configurator_scanf(cmd, config_file, config_node,
                                  b"%u\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut (*conf).max_connections as
                                      *mut libc::c_uint);
}
/*
static int on_config_max_connections(h2o_configurator_command_t *cmd, h2o_configurator_context_t *ctx, const char *config_file, yoml_t *config_node)
{
    struct config_t *conf = H2O_STRUCT_FROM_MEMBER(struct config_t, globalconf, ctx->globalconf);
    return h2o_configurator_scanf(cmd, config_file, config_node, "%u", &conf->max_connections);
}

*/