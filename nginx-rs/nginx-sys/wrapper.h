// #include <nginx.h>
// #include <ngx_conf_file.h>
#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>
#include <ngx_mail.h>
#include <ngx_stream.h>

// Define as constants since bindgen can't parse these values
const size_t NGX_RS_HTTP_LOC_CONF_OFFSET = NGX_HTTP_LOC_CONF_OFFSET;
const size_t NGX_RS_HTTP_MAIN_CONF_OFFSET = NGX_HTTP_MAIN_CONF_OFFSET;
const size_t NGX_RS_HTTP_SRV_CONF_OFFSET = NGX_HTTP_SRV_CONF_OFFSET;
const size_t NGX_RS_MAIL_MAIN_CONF_OFFSET = NGX_MAIL_MAIN_CONF_OFFSET;
const size_t NGX_RS_MAIL_SRV_CONF_OFFSET = NGX_MAIL_SRV_CONF_OFFSET;
const size_t NGX_RS_STREAM_MAIN_CONF_OFFSET = NGX_STREAM_MAIN_CONF_OFFSET;
const size_t NGX_RS_STREAM_SRV_CONF_OFFSET = NGX_STREAM_SRV_CONF_OFFSET;

const char* NGX_RS_MODULE_SIGNATURE = NGX_MODULE_SIGNATURE;

typedef struct {
    ngx_str_t                      key_start;
    ngx_str_t                      schema;
    ngx_str_t                      host_header;
    ngx_str_t                      port;
    ngx_str_t                      uri;
} ngx_http_proxy_vars_t;

typedef struct {
    ngx_array_t                   *flushes;
    ngx_array_t                   *lengths;
    ngx_array_t                   *values;
    ngx_hash_t                     hash;
} ngx_http_proxy_headers_t;

// extern ngx_module_t  ngx_http_proxy_module;
// #if (NGX_HTTP_PROXY)
// #endif
typedef struct {
    ngx_http_upstream_conf_t       upstream;

    ngx_array_t                   *body_flushes;
    ngx_array_t                   *body_lengths;
    ngx_array_t                   *body_values;
    ngx_str_t                      body_source;

    ngx_http_proxy_headers_t       headers;
#if (NGX_HTTP_CACHE)
    ngx_http_proxy_headers_t       headers_cache;
#endif
    ngx_array_t                   *headers_source;

    ngx_array_t                   *proxy_lengths;
    ngx_array_t                   *proxy_values;

    ngx_array_t                   *redirects;
    ngx_array_t                   *cookie_domains;
    ngx_array_t                   *cookie_paths;
    ngx_array_t                   *cookie_flags;

    ngx_http_complex_value_t      *method;
    ngx_str_t                      location;
    ngx_str_t                      url;

#if (NGX_HTTP_CACHE)
    ngx_http_complex_value_t       cache_key;
#endif

    ngx_http_proxy_vars_t          vars;

    ngx_flag_t                     redirect;

    ngx_uint_t                     http_version;

    ngx_uint_t                     headers_hash_max_size;
    ngx_uint_t                     headers_hash_bucket_size;

#if (NGX_HTTP_SSL)
    ngx_uint_t                     ssl;
    ngx_uint_t                     ssl_protocols;
    ngx_str_t                      ssl_ciphers;
    ngx_uint_t                     ssl_verify_depth;
    ngx_str_t                      ssl_trusted_certificate;
    ngx_str_t                      ssl_crl;
    ngx_array_t                   *ssl_conf_commands;
#endif
} ngx_http_proxy_loc_conf_t;

typedef struct {
    ngx_array_t                    caches;  /* ngx_http_file_cache_t * */
} ngx_http_proxy_main_conf_t;