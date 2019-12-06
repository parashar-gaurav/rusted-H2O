
/*It is a function which defines the user under which the server should
 handle incoming request.Configurator: *mut h2o_configurator_t is the 
 configurator to which the command belongs .name: *const libc::c_char is
  the name of the command handled by the configurator,example “listen”.
   Defined flags are:
 */




 fn h2o_configurator__define_command(mut configurator:
                                                              *mut h2o_configurator_t,
                                                          mut name:
                                                              *const libc::c_char,
                                                          mut flags:
                                                              libc::c_int,
                                                          mut cb:
                                                              h2o_configurator_command_cb,
                                                          mut desc:
                                                              *mut *const libc::c_char) {
    let mut cmd: *mut h2o_configurator_command_t =
        0 as *mut h2o_configurator_command_t;
    let fresh3 = (*configurator).commands.size;
    (*configurator).commands.size =
        (*configurator).commands.size.wrapping_add(1);
    cmd = (*configurator).commands.entries.offset(fresh3 as isize);
    (*cmd).configurator = configurator;
    (*cmd).flags = flags;
    (*cmd).name = name;
    (*cmd).cb = cb;
    (*cmd).description = desc;
}


/*
void h2o_configurator__define_command(h2o_configurator_t *configurator, const char *name, int flags, h2o_configurator_command_cb cb, const char **desc)
{
    h2o_configurator_command_t *cmd;
    cmd = configurator->commands.entries + configurator->commands.size++;
    cmd->configurator = configurator;
    cmd->flags = flags;
    cmd->name = name;
    cmd->cb = cb;
    cmd->description = desc;
}
*/