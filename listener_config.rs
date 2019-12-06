

//listener_configurator_t is a structure which stores information 
//about number of global listeners and host listeners.

pub struct listener_configurator_t {
    pub super: h2o_configurator_t,
    pub num_global_listeners: size_t,
    pub num_host_listeners: size_t,
}

/* corresponding c structure

struct listener_configurator_t {
    h2o_configurator_t super;
    size_t num_global_listeners;
    size_t num_host_listeners;
};
*/
