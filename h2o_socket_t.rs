//h2o_socket_t 
//H2o_socket_t is a structure, which
//Peername is a variable of type h2o_socket_peername_t
// which is another structure and it contains data about
 // socket address and length. It is filled with 0, in case of invalid address

pub struct h2o_socket_t {
    pub data: *mut libc::c_void,
    pub input: *mut h2o_buffer_t,
    pub bytes_read: size_t,
    pub peername: h2o_socket_peername_t,
}

/*correspoding c code : 
/**
 * abstraction layer for sockets 
 */
struct st_h2o_socket_t {
    void *data;
    struct st_h2o_socket_ssl_t *ssl;
    h2o_buffer_t *input;
    size_t bytes_read;
    struct {
        void (*cb)(void *data);
        void *data;
    } on_close;
    struct {
        h2o_socket_cb read;
        h2o_socket_cb write;
    } _cb;
    /* zero-filled in case of invalid address */
    h2o_socket_peername_t peername;
};
*/
