extern crate bindgen;
extern crate tempfile;

use std::env;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::process::Output;
use std::io::Result;

use tempfile::TempDir;

const NGINX_VERSION: &str = "1.19.10";
const NGINX_DOWNLOAD_PATH: &str = "https://nginx.org/download";

/// Download nginx
fn download_nginx() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let version = env::var("NGINX_VERSION").unwrap_or_else(|_| NGINX_VERSION.to_string());
    let filename = format!("nginx-{}.tar.gz", version);

    let url = format!(
        "{}/{}",
        NGINX_DOWNLOAD_PATH,
        filename
    );

    let out_dir_name = tmp_dir
        .path()
        .join(&filename)
        .display()
        .to_string();

    println!("Downloading nginx from {} into {}", url, &out_dir_name);

    let result = Command::new("curl")
        .args([
            url.as_str(),
            "-o",
            filename.as_str()
        ])
        .current_dir(&tmp_dir.path())
        .status()?;

    if !result.success() {
        panic!("Failed to download nginx to {}", &out_dir_name);
    }

    let output_dir = nginx_dir();

    let result = Command::new("mkdir")
        .args([
            "-p",
            output_dir.as_str()
        ])
        .current_dir(&tmp_dir.path())
        .status()?;

    if !result.success() {
        panic!("Failed to create nginx target folder");
    }

    println!("Inflating nginx from {}", &out_dir_name);

    let result = Command::new("tar")
        .args([
            "-C",
            output_dir.as_str(),
            "-xzf",
            filename.as_str(),
            "--strip-components",
            "1"
        ])
        .current_dir(&tmp_dir.path())
        .status()?;

    if !result.success() {
        panic!("Failed to inflate nginx file");
    }

    tmp_dir.close()?;

    Ok(())
}

// return all nginx features
fn ngix_features() -> Vec<&'static str> {
    let mut features: Vec<&'static str> = Vec::new();
    if cfg!(feature = "with-compat") {
        features.push("--with-compat");
    }
    if cfg!(feature = "with-threads") {
        features.push("--with-threads");
    }
    if cfg!(feature = "with-http_addition_module") {
        features.push("--with-http_addition_module");
    }
    if cfg!(feature = "with-http_auth_request_module") {
        features.push("--with-http_auth_request_module");
    }
    if cfg!(feature = "with-http_gunzip_module") {
        features.push("--with-http_gunzip_module");
    }
    if cfg!(feature = "with-http_auth_request_module") {
        features.push("--with-http_auth_request_module");
    }
    if cfg!(feature = "with-http_gzip_static_module") {
        features.push("--with-http_gzip_static_module");
    }
    if cfg!(feature = "with-http_random_index_module") {
        features.push("--with-http_random_index_module");
    }
    if cfg!(feature = "with-http_realip_module") {
        features.push("--with-http_realip_module");
    }
    if cfg!(feature = "with-http_secure_link_module") {
        features.push("--with-http_secure_link_module");
    }
    if cfg!(feature = "with-http_slice_module") {
        features.push("--with-http_slice_module");
    }
    if cfg!(feature = "with-http_stub_status_module") {
        features.push("--with-http_stub_status_module");
    }
    if cfg!(feature = "with-http_sub_module") {
        features.push("--with-http_sub_module");
    }
    if cfg!(feature = "with-stream") {
        features.push("--with-stream");
    }
    if cfg!(feature = "with-stream_realip_module") {
        features.push("--with-stream_realip_module");
    }
    if cfg!(feature = "with-stream_ssl_preread_module") {
        features.push("--with-stream_ssl_preread_module");
    }

    if cfg!(all(feature = "with-file-aio", target_os="linux")) {
        features.push("--with-file-aio");
    }
    if cfg!(all(feature = "with-file-aio", target_os="linux"))  {
        features.push("--with-http_ssl_module");
    }
    if cfg!(all(feature = "with-file-aio", target_os="linux"))  {
        features.push("--with-stream_ssl_module");
    }

    // TOOD: don't know what this is, if it's needed and why. It did break my build and it worked by commentig it out.
    // if cfg!(target_os="linux")  {
    //     features.push("--with-cc-opt='-g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC'");
    //     features.push("--with-ld-opt='-Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie'");
    // }


    println!("configuring nginx: {:?}",features);

    features
}


// nginx source directory
fn nginx_dir() -> String  {
    let out_dir = env::var("NGINX_PATH").unwrap_or_else(|_| {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        format!("{}/nginx", out_dir.display())
    });
    format!("{}", PathBuf::from(out_dir).display())
}

fn configure() -> Result<Output> {
    let nginx_path_name = nginx_dir();
    println!("nginx configure at {}", nginx_path_name);
    let mut args: Vec<&str> = vec![
        "configure"
    ];

    let features = ngix_features();

    for feature in features {
        args.push(feature)
    }

    let output = Command::new("bash")
        .args(&args)
        .current_dir(nginx_path_name)
        .output()?;

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        panic!("Failed to configure nginx");
    }

    Ok(output)
}

fn generate_binding() -> Result<()> {
    let nginx_dir = nginx_dir();

    let paths = vec![
        format!("{}/src/core", nginx_dir),
        format!("{}/src/event", nginx_dir),
        format!("{}/src/mail", nginx_dir),
        format!("{}/src/stream", nginx_dir),
        format!("{}/src/event/modules", nginx_dir),
        format!("{}/src/os/unix", nginx_dir),
        format!("{}/objs", nginx_dir),
        format!("{}/src/http", nginx_dir),
        format!("{}/src/http/v2", nginx_dir),
        format!("{}/src/http/modules", nginx_dir)
    ];

    paths.iter().for_each(|path| {
        let exists = Path::new(path).exists();
        if !exists {
            panic!("Failed to locate required directory {}", path)
        }
        // println!("cargo:rustc-link-search=native={}", path);
    });

    // env::var("NGINX_PATH").unwrap_or(String::from("../nginx"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .layout_tests(false)
        .trust_clang_mangling(false)
        .blocklist_item("IPPORT_RESERVED")
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .translate_enum_integer_types(true)
        .allowlist_type("ngx_.*")
        .allowlist_function("ngx_.*")
        .allowlist_var("NGX_.*|ngx_.*|nginx_.*")
        .clang_args(
            paths.iter().map(|path| {
                format!("-I{}", path)
            }).collect::<Vec<_>>()
        )
        // .clang_arg(format!("-I{}/src/core", nginx_dir))
        // .clang_arg(format!("-I{}/src/event", nginx_dir))
        // .clang_arg(format!("-I{}/src/mail", nginx_dir))
        // .clang_arg(format!("-I{}/src/stream", nginx_dir))
        // .clang_arg(format!("-I{}/src/event/modules", nginx_dir))
        // .clang_arg(format!("-I{}/src/os/unix", nginx_dir))
        // // .clang_arg(format!("-I{}/src/os/win32", nginx_dir))
        // .clang_arg(format!("-I{}/objs", nginx_dir))
        // .clang_arg(format!("-I{}/src/http", nginx_dir))
        // .clang_arg(format!("-I{}/src/http/v2", nginx_dir))
        // .clang_arg(format!("-I{}/src/http/modules", nginx_dir))
        // .clang_arg(format!("-v"))
        // .clang_arg(format!("--target=arm64-apple-darwin"))
        // .clang_arg(format!("-arch aarch64"))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
}

fn main() {
    println!("cargo:rerun-if-env-changed=NGINX_VERSION");
    println!("cargo:rerun-if-env-changed=NGINX_PATH");
    println!("cargo:rerun-if-changed=wrapper.h");

    if let Err(_) = env::var("NGINX_PATH") {
        if let Err(err) = download_nginx() {
            panic!("Download error: {:?}", err);
        }

        match configure() {
            Err(err) => {
                panic!("Configure Error: {:?}", err)
            }
            Ok(_data) => {}
        }
    }

    if let Err(err) = generate_binding() {
        panic!("Binding Error: {:?}", err)
    }
}
