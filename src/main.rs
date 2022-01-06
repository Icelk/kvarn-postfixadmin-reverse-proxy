use kvarn::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut extensions = Extensions::new();

    kvarn_extensions::php(
        &mut extensions,
        kvarn_extensions::Connection::UnixSocket(Path::new("/run/postfixadmin/postfixadmin.sock")),
    );
    extensions.with_csp(Csp::new().arc());

    let mut host_options = host::Options::new();
    host_options.folder_default = Some("index.php".into());
    host_options.extension_default = Some("php".into());
    let mut host = Host::unsecure(
        "localhost",
        "/usr/share/webapps/postfixadmin/",
        extensions,
        host_options,
    );
    host.disable_server_cache();
    let data = HostCollection::builder().default(host).build();
    let port_descriptor = PortDescriptor::unsecure(8081, data);

    let shutdown_manager = RunConfig::new()
        .bind(port_descriptor)
        .disable_handover()
        .execute()
        .await;
    shutdown_manager.wait().await;
}
