//h2o_configurator_create: This function is responsible for adding and registering a configurator.


fn h2o_configurator_create(conf: *mut h2o_globalconf_t, sz: size_t)
     -> *mut h2o_configurator_t;

/*corresponding c code 

h2o_configurator_t *h2o_configurator_create(h2o_globalconf_t *conf, size_t sz)
{
    h2o_configurator_t *c;

    assert(sz >= sizeof(*c));

    c = h2o_mem_alloc(sz);
    memset(c, 0, sz);
    h2o_linklist_insert(&conf->configurators, &c->_link);

    return c;
}

*/