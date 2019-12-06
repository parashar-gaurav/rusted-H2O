//function on_socketclose

fn on_socketclose(mut data: *mut libc::c_void) {
    let mut ctx: *mut h2o_context_t = data as *mut h2o_context_t;
    let mut conf: *mut config_t =
        ((*ctx).globalconf as *mut libc::c_char).offset(-0) as *mut config_t;
    let mut prev_num_connections: libc::c_uint =
        ::std::intrinsics::atomic_xsub(&mut (*conf).state.num_connections,
                                       1i32 as libc::c_uint);
    if (*conf).num_threads != 1i32 as libc::c_uint {
        if prev_num_connections == (*conf).max_connections {
            /* ready to accept new connections.  wake up the threads! */
            let mut self_tid: pthread_t = pthread_self();
            let mut i: libc::c_uint = 0;
            i = 0i32 as libc::c_uint;
            while i != (*conf).num_threads {
                if *(*conf).thread_ids.offset(i as isize) != self_tid {
                    pthread_kill(*(*conf).thread_ids.offset(i as isize),
                                 18i32);
                }
                i = i.wrapping_add(1)
            }
        }
    };
}

/*
static void on_socketclose(void *data)
{
    h2o_context_t *ctx = data;
    struct config_t *conf = H2O_STRUCT_FROM_MEMBER(struct config_t, globalconf, ctx->globalconf);
    unsigned prev_num_connections = __sync_fetch_and_sub(&conf->state.num_connections, 1);

    if (conf->num_threads != 1) {
        if (prev_num_connections == conf->max_connections) {
            /* ready to accept new connections.  wake up the threads! */
            pthread_t self_tid = pthread_self();
            unsigned i;
            for (i = 0; i != conf->num_threads; ++i) {
                if (conf->thread_ids[i] != self_tid)
                    pthread_kill(conf->thread_ids[i], SIGCONT);
            }
        }
    }
}
*/



