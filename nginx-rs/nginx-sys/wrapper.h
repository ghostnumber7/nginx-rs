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




